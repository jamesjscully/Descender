//! Gadget 2 — in-AIR Merkle opening verification (reference layer).
//!
//! Verifies a binary Merkle authentication path against a committed root, mirroring
//! `p3-merkle-tree`'s `verify_batch` fold (mmcs/batch.rs:211-233) constant-for-constant:
//! at each level `pos_in_group = index % 2` places the running digest, the sibling fills
//! the other slot, and `compress` folds them — `inputs = [digest, sibling]` when the index
//! bit is 0, else `[sibling, digest]`; then `index /= 2`. The node compression is the
//! leaf STARK's exact `MyCompress = TruncatedPermutation<Perm, 2, 8, 16>` (so node hashing
//! is byte-identical to the deployed Merkle), used here as a BLACK BOX — gadget 2's AIR
//! will constrain the Poseidon2 permutation inside it via the gadget-2/Poseidon2 sub-AIR.
//!
//! Scope of this reference: the binary single-matrix path fold (the dominant case for the
//! recursion's FRI-layer openings). DEFERRED: leaf-row hashing (`MyHash` over opened matrix
//! rows incl. the hiding salt) and the multi-height batch injection (batch.rs:243-259) for
//! mixed-height commitments — documented follow-ons, same pattern.

use descender_account_stark::{Perm, Val, DIGEST_ELEMS, WIDTH};
use p3_koala_bear::default_koalabear_poseidon2_16;
use p3_symmetric::{PseudoCompressionFunction, TruncatedPermutation};

/// A Merkle node digest (8 KoalaBear elems ≈ 248 bits — the accepted 2⁻¹²⁴ creation floor).
pub type Digest = [Val; DIGEST_ELEMS];

/// The leaf STARK's exact 2-to-1 node compression (`MyCompress`). Literal const params
/// `2,8,16` match `config::MyCompress`; [`assert_layout_matches_leaf`] pins them.
pub type Compress = TruncatedPermutation<Perm, 2, DIGEST_ELEMS, WIDTH>;

/// Build the node compressor over the pinned Poseidon2-16 (byte-identical to the leaf).
pub fn compressor() -> Compress {
    TruncatedPermutation::new(default_koalabear_poseidon2_16())
}

/// One fold step (spec / `batch.rs:216-232`): place `digest` at `index_bit`'s slot, the
/// `sibling` at the other, compress.
#[inline]
pub fn fold_step(c: &Compress, digest: Digest, sibling: Digest, index_bit: bool) -> Digest {
    let inputs = if index_bit {
        [sibling, digest] // index % 2 == 1 ⇒ running digest is the right child
    } else {
        [digest, sibling] // index % 2 == 0 ⇒ running digest is the left child
    };
    c.compress(inputs)
}

/// Fold a leaf digest up `siblings` (bottom-to-top) at tree position `index`, returning the
/// reconstructed root. `siblings.len()` = tree depth.
pub fn fold_to_root(c: &Compress, leaf: Digest, mut index: usize, siblings: &[Digest]) -> Digest {
    let mut digest = leaf;
    for &sib in siblings {
        digest = fold_step(c, digest, sib, index & 1 == 1);
        index >>= 1;
    }
    digest
}

/// Verify an opening: the leaf at `index` with `siblings` folds to `root`.
pub fn verify_path(
    c: &Compress,
    leaf: Digest,
    index: usize,
    siblings: &[Digest],
    root: Digest,
) -> bool {
    fold_to_root(c, leaf, index, siblings) == root
}

/// Drift guard: the const layout must match the leaf STARK's `MyCompress`/digest.
pub fn assert_layout_matches_leaf() {
    assert_eq!(DIGEST_ELEMS, 8, "digest width drifted from leaf config");
    assert_eq!(WIDTH, 16, "Poseidon2 width drifted from leaf config");
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_field::PrimeCharacteristicRing;

    fn digest(seed: u32) -> Digest {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32)))
    }

    /// A reference binary Merkle tree built with the REAL compression — the authority the
    /// path verifier is checked against (analogous to gadget-1's native challenger).
    struct RefTree {
        c: Compress,
        levels: Vec<Vec<Digest>>, // levels[0] = leaves, last = [root]
    }

    impl RefTree {
        fn new(mut leaves: Vec<Digest>) -> Self {
            assert!(leaves.len().is_power_of_two() && !leaves.is_empty());
            let c = compressor();
            let mut levels = vec![leaves.clone()];
            while leaves.len() > 1 {
                let next: Vec<Digest> = leaves
                    .chunks_exact(2)
                    .map(|pair| c.compress([pair[0], pair[1]]))
                    .collect();
                levels.push(next.clone());
                leaves = next;
            }
            Self { c, levels }
        }

        fn root(&self) -> Digest {
            self.levels.last().unwrap()[0]
        }

        /// Sibling path for `index`, bottom-to-top.
        fn auth_path(&self, index: usize) -> Vec<Digest> {
            let mut sibs = Vec::new();
            let mut idx = index;
            for level in &self.levels[..self.levels.len() - 1] {
                sibs.push(level[idx ^ 1]);
                idx >>= 1;
            }
            sibs
        }
    }

    /// GOLDEN (positive): every leaf opening verifies against the root reconstructed by the
    /// same real compression. Anchors the path-fold + index-bit convention to a real tree.
    #[test]
    fn honest_openings_verify_for_every_leaf() {
        let n = 16usize;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        for i in 0..n {
            let sibs = tree.auth_path(i);
            assert!(
                verify_path(&tree.c, leaves[i], i, &sibs, root),
                "honest opening for leaf {i} failed to verify"
            );
        }
    }

    /// Forgery: a tampered sibling must break the fold (spec — the path binds the leaf to
    /// the root). Tested by attempted acceptance being REJECTED.
    #[test]
    fn tampered_sibling_is_rejected() {
        let leaves: Vec<Digest> = (0..8u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let mut sibs = tree.auth_path(3);
        sibs[0][0] += Val::ONE; // flip one element of the first sibling
        assert!(!verify_path(&tree.c, leaves[3], 3, &sibs, tree.root()));
    }

    /// Forgery: a tampered leaf is rejected.
    #[test]
    fn tampered_leaf_is_rejected() {
        let leaves: Vec<Digest> = (0..8u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let sibs = tree.auth_path(5);
        let mut leaf = leaves[5];
        leaf[7] += Val::ONE;
        assert!(!verify_path(&tree.c, leaf, 5, &sibs, tree.root()));
    }

    /// Forgery: claiming the right index (wrong bit ordering) is rejected — the index-bit
    /// convention is load-bearing (left/right child placement). A sibling path opened at
    /// index i must NOT verify when replayed at a sibling index whose path differs.
    #[test]
    fn wrong_index_is_rejected() {
        let leaves: Vec<Digest> = (0..16u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let sibs = tree.auth_path(6);
        // leaf 6's own path verifies at 6 but not at 7 (different bit at level 0 ⇒ wrong
        // child placement) nor at a far index.
        assert!(verify_path(&tree.c, leaves[6], 6, &sibs, tree.root()));
        assert!(!verify_path(&tree.c, leaves[6], 7, &sibs, tree.root()));
        assert!(!verify_path(&tree.c, leaves[6], 2, &sibs, tree.root()));
    }

    /// Forgery: a different root is rejected.
    #[test]
    fn wrong_root_is_rejected() {
        let leaves: Vec<Digest> = (0..8u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let sibs = tree.auth_path(1);
        let mut bad_root = tree.root();
        bad_root[0] += Val::ONE;
        assert!(!verify_path(&tree.c, leaves[1], 1, &sibs, bad_root));
    }

    #[test]
    fn layout_matches_leaf_config() {
        assert_layout_matches_leaf();
    }
}
