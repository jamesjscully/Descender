//! Binary Merkle vector commitment over the Poseidon2 hash (q = 12289).
//!
//! This is the vector commitment the upcoming FRI proof layer builds on:
//! commit to a vector of leaves (each leaf is itself a vector of Fq=12289 field
//! values — e.g. one row of codeword / trace columns), produce a single root
//! digest, and later open any position with a logarithmic authentication path
//! that any verifier can check against the root.
//!
//! CORRECTNESS and clean authentication-path verification are the priorities.
//! The cryptographic hash is the EXISTING, externally-validated Poseidon2 sponge
//! in `poseidon2_q12289` — we do NOT reimplement the permutation here.
//!
//! ----------------------------------------------------------------------------
//! DESIGN
//! ----------------------------------------------------------------------------
//!  * DIGEST: `[u16; 20]` — 20 Fq=12289 lanes. A Merkle digest is a BINDING
//!    (collision) object, so its security is (output bits)/2: 20 lanes ≈ 272
//!    output bits, collision-bounded by the sponge capacity (19 lanes ≈ 258
//!    bits) ⇒ ~129-bit collision resistance, adequate for the 128-bit target.
//!    (A 10-lane / ~136-bit digest gave only ~68-bit collision resistance — too
//!    short.) Matches the project's 20-lane commitment-output convention (cf.
//!    the pad-source / opening-tuple commitments in `poseidon2_q12289`).
//!
//!  * LEAF HASH: `leaf_digest(leaf) = poseidon2_sponge(LEAF_DOMAIN, leaf, 20)`.
//!
//!  * NODE COMPRESSION (2-to-1): `compress(left, right) =`
//!    `poseidon2_sponge(NODE_DOMAIN, left ++ right /* 40 lanes */, 20)`.
//!    NODE_DOMAIN is DISTINCT from LEAF_DOMAIN: this domain separation prevents a
//!    leaf whose contents happen to equal a node's 20-lane two-child
//!    concatenation from colliding with that node's digest.
//!
//!  * TREE: a complete binary Merkle tree over `N` leaves where `N` is a power of
//!    two. `commit` PADS the leaf set up to the next power of two with a fixed
//!    padding leaf (see `PADDING_LEAF` / limitation note below) and records the
//!    padded length, so callers may pass any non-zero count.
//!
//!  * INDEX-BIT FOLDING CONVENTION (LSB-first): at tree level `d` (d = 0 is the
//!    leaf level), bit `d` of `index` (the d-th least-significant bit) tells us
//!    where the CURRENT node sits relative to its sibling:
//!      - bit == 0  =>  current node is the LEFT child  =>  parent = compress(cur, sibling)
//!      - bit == 1  =>  current node is the RIGHT child =>  parent = compress(sibling, cur)
//!    The path lists siblings bottom-up (leaf level first, just below root last).
//!
//!  * VERIFY recomputes the LEAF DIGEST from the supplied raw leaf — it does NOT
//!    trust a caller-supplied leaf digest — then folds it with the path siblings
//!    using the index bits and compares the result to `root`.

use crate::poseidon2_q12289::poseidon2_sponge;

/// Digest width in Fq=12289 lanes (~129-bit collision resistance: a Merkle
/// digest is a binding/collision object, so security is output/2, capacity-
/// limited to ~258/2 bits).
pub const DIGEST_LANES: usize = 20;

/// A Merkle digest: 20 canonical Fq=12289 lanes.
pub type Digest = [u16; DIGEST_LANES];

/// Domain-separation tag for LEAF hashing. DISTINCT from NODE_DOMAIN.
pub const LEAF_DOMAIN: &[u8] = b"Descender.Merkle.q12289.poseidon2.leaf.v1";

/// Domain-separation tag for internal NODE (2-to-1) compression. DISTINCT from
/// LEAF_DOMAIN so a leaf can never collide with an internal node digest.
pub const NODE_DOMAIN: &[u8] = b"Descender.Merkle.q12289.poseidon2.node.v1";

/// Fixed padding leaf used to round a non-power-of-two leaf count up to the next
/// power of two. It is a single zero field element. Because `commit` records the
/// padded length and the padding is a FIXED, publicly-known value, padding does
/// not affect binding of the real leaves (positions `0..n_real`).
pub const PADDING_LEAF: &[u16] = &[0u16];

/// Hash one leaf (a vector of Fq=12289 field values) to a digest.
///
/// `poseidon2_sponge` reduces each input lane mod Q and emits DIGEST_LANES (20)
/// canonical lanes, so the returned digest lanes are all `< Q`.
pub fn leaf_digest(leaf: &[u16]) -> Digest {
    let out = poseidon2_sponge(LEAF_DOMAIN, leaf, DIGEST_LANES);
    let mut d = [0u16; DIGEST_LANES];
    d.copy_from_slice(&out);
    d
}

/// 2-to-1 compression of two child digests into a parent digest.
///
/// Concatenates `left ++ right` (20 lanes) and hashes under NODE_DOMAIN.
pub fn compress(left: &Digest, right: &Digest) -> Digest {
    let mut both = [0u16; 2 * DIGEST_LANES];
    both[..DIGEST_LANES].copy_from_slice(left);
    both[DIGEST_LANES..].copy_from_slice(right);
    let out = poseidon2_sponge(NODE_DOMAIN, &both, DIGEST_LANES);
    let mut d = [0u16; DIGEST_LANES];
    d.copy_from_slice(&out);
    d
}

/// An authentication path: the sibling digests from the leaf level up to (but
/// not including) the root, ordered bottom-up. Length == log2(padded N).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerklePath {
    /// Sibling digests, level 0 (leaf level) first.
    pub siblings: Vec<Digest>,
}

/// A committed Merkle tree over a power-of-two number of leaf digests.
///
/// `layers[0]` is the leaf-digest level (length = padded N, a power of two);
/// each subsequent layer is half the length of the one below; the final layer
/// has a single element — the root.
#[derive(Clone, Debug)]
pub struct MerkleTree {
    /// Number of REAL (non-padding) leaves committed.
    pub n_real: usize,
    /// Padded leaf count (a power of two, == layers[0].len()).
    pub n_padded: usize,
    /// All tree levels, bottom (leaves) to top (root).
    layers: Vec<Vec<Digest>>,
}

/// Round `n` up to the next power of two (n >= 1). Returns `n` if already a
/// power of two.
fn next_pow2(n: usize) -> usize {
    debug_assert!(n >= 1);
    if n.is_power_of_two() {
        n
    } else {
        n.next_power_of_two()
    }
}

impl MerkleTree {
    /// Commit to a vector of leaves. Each leaf is a vector of Fq=12289 values.
    ///
    /// If the leaf count is not a power of two it is padded up to the next power
    /// of two with `PADDING_LEAF`. Panics on an empty leaf set.
    pub fn commit(leaves: &[Vec<u16>]) -> MerkleTree {
        assert!(!leaves.is_empty(), "cannot commit to an empty leaf set");
        let n_real = leaves.len();
        let n_padded = next_pow2(n_real);

        // Bottom layer: leaf digests, padded to a power of two.
        let mut bottom: Vec<Digest> = Vec::with_capacity(n_padded);
        for leaf in leaves {
            bottom.push(leaf_digest(leaf));
        }
        let pad_digest = leaf_digest(PADDING_LEAF);
        while bottom.len() < n_padded {
            bottom.push(pad_digest);
        }

        // Build up the layers by 2-to-1 compression.
        let mut layers: Vec<Vec<Digest>> = Vec::new();
        layers.push(bottom);
        while layers.last().unwrap().len() > 1 {
            let cur = layers.last().unwrap();
            let mut up: Vec<Digest> = Vec::with_capacity(cur.len() / 2);
            for pair in cur.chunks(2) {
                up.push(compress(&pair[0], &pair[1]));
            }
            layers.push(up);
        }

        MerkleTree {
            n_real,
            n_padded,
            layers,
        }
    }

    /// The Merkle root (top digest).
    pub fn root(&self) -> Digest {
        self.layers.last().unwrap()[0]
    }

    /// Tree depth = number of compression levels = log2(padded N) = path length.
    pub fn depth(&self) -> usize {
        self.layers.len() - 1
    }

    /// Open position `index` (0-based, into the PADDED leaf range): the sibling
    /// digests from the leaf level up to the root, bottom-up.
    ///
    /// Panics if `index >= n_padded`.
    pub fn open(&self, index: usize) -> MerklePath {
        assert!(index < self.n_padded, "open index {index} out of range (n_padded = {})", self.n_padded);
        let mut siblings = Vec::with_capacity(self.depth());
        let mut pos = index;
        // Walk every non-root layer, recording the sibling of `pos`.
        for level in 0..self.depth() {
            let sib = pos ^ 1; // flip LSB: the sibling within the pair
            siblings.push(self.layers[level][sib]);
            pos >>= 1; // parent position at the next level up
        }
        MerklePath { siblings }
    }
}

/// Verify an authentication path: recompute the LEAF DIGEST from `leaf` (the
/// raw leaf — a supplied leaf digest is NOT trusted), fold it upward with the
/// path siblings using the LSB-first index-bit convention, and compare to
/// `root`.
///
/// Index-bit convention (LSB-first): at level `d`, bit `d` of `index` selects
/// the current node's position — bit 0 => current is LEFT child, bit 1 =>
/// current is RIGHT child.
///
/// Returns `false` (rather than panicking) on any mismatch, including an index
/// that is out of range for the given path length.
pub fn verify(root: &Digest, index: usize, leaf: &[u16], path: &MerklePath) -> bool {
    let depth = path.siblings.len();
    // `index` must fit within a tree of this depth (2^depth leaves).
    if depth >= usize::BITS as usize {
        // pathological depth; reject rather than overflow the shift.
        return false;
    }
    if index >= (1usize << depth) {
        return false;
    }

    // Recompute the leaf digest from the raw leaf — do NOT trust any supplied
    // digest.
    let mut acc = leaf_digest(leaf);
    let mut pos = index;
    for sib in &path.siblings {
        acc = if pos & 1 == 0 {
            // current node is the LEFT child
            compress(&acc, sib)
        } else {
            // current node is the RIGHT child
            compress(sib, &acc)
        };
        pos >>= 1;
    }
    &acc == root
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poseidon2_q12289::Q;

    /// Deterministic, reproducible leaf generator: leaf `i` is a short vector of
    /// Fq values derived from `i` (and `width`).
    fn make_leaf(i: usize, width: usize) -> Vec<u16> {
        (0..width)
            .map(|k| (((i as u32) * 1009 + (k as u32) * 31 + 7) % Q) as u16)
            .collect()
    }

    fn make_leaves(n: usize, width: usize) -> Vec<Vec<u16>> {
        (0..n).map(|i| make_leaf(i, width)).collect()
    }

    /// Determinism: same leaves -> same root.
    #[test]
    fn merkle_poseidon2_determinism() {
        let leaves = make_leaves(16, 4);
        let t1 = MerkleTree::commit(&leaves);
        let t2 = MerkleTree::commit(&leaves);
        assert_eq!(t1.root(), t2.root(), "same leaves must give same root");
        // every digest lane is canonical (< Q)
        for &x in t1.root().iter() {
            assert!((x as u32) < Q, "root lane {x} not reduced mod Q");
        }
    }

    /// Completeness: for trees of 16 and 1024 leaves, EVERY leaf's opened path
    /// verifies against the root.
    #[test]
    fn merkle_poseidon2_completeness_all_leaves() {
        for &n in &[16usize, 1024usize] {
            let leaves = make_leaves(n, 5);
            let tree = MerkleTree::commit(&leaves);
            let root = tree.root();
            assert_eq!(tree.depth(), (n as f64).log2() as usize);
            for (i, leaf) in leaves.iter().enumerate() {
                let path = tree.open(i);
                assert_eq!(path.siblings.len(), tree.depth(), "path length must equal depth");
                assert!(
                    verify(&root, i, leaf, &path),
                    "leaf {i} of {n} failed to verify against root"
                );
            }
        }
    }

    /// Soundness / tamper: flipped leaf value, flipped sibling, wrong index, and
    /// flipped root must each fail verification.
    #[test]
    fn merkle_poseidon2_tamper_detection() {
        let leaves = make_leaves(64, 4);
        let tree = MerkleTree::commit(&leaves);
        let root = tree.root();
        let idx = 37usize;
        let leaf = &leaves[idx];
        let path = tree.open(idx);

        // baseline: honest opening verifies
        assert!(verify(&root, idx, leaf, &path), "honest opening must verify");

        // (a) flipped leaf value
        let mut bad_leaf = leaf.clone();
        bad_leaf[0] = ((bad_leaf[0] as u32 + 1) % Q) as u16;
        assert!(
            !verify(&root, idx, &bad_leaf, &path),
            "a flipped leaf value must fail verify"
        );

        // (b) flipped path sibling
        let mut bad_path = path.clone();
        bad_path.siblings[0][0] = ((bad_path.siblings[0][0] as u32 + 1) % Q) as u16;
        assert!(
            !verify(&root, idx, leaf, &bad_path),
            "a flipped path sibling must fail verify"
        );

        // (c) wrong index (honest leaf+path but claimed at a different position)
        let wrong_idx = idx ^ 1; // sibling slot
        assert!(
            !verify(&root, wrong_idx, leaf, &path),
            "a wrong index must fail verify"
        );

        // (d) flipped root
        let mut bad_root = root;
        bad_root[0] = ((bad_root[0] as u32 + 1) % Q) as u16;
        assert!(
            !verify(&bad_root, idx, leaf, &path),
            "a flipped root must fail verify"
        );
    }

    /// Domain separation: a leaf whose raw lanes equal a node's two-child (20
    /// lane) concatenation must NOT hash to the same digest as that node. This
    /// guards against leaf/node second-preimage collisions.
    #[test]
    fn merkle_poseidon2_domain_separation_leaf_vs_node() {
        // Two arbitrary child digests.
        let left = leaf_digest(&make_leaf(1, 4));
        let right = leaf_digest(&make_leaf(2, 4));

        // Node digest = compress(left, right) under NODE_DOMAIN.
        let node = compress(&left, &right);

        // A leaf whose raw field values are EXACTLY left ++ right (20 lanes).
        let mut concat: Vec<u16> = Vec::with_capacity(2 * DIGEST_LANES);
        concat.extend_from_slice(&left);
        concat.extend_from_slice(&right);
        let leaf_over_concat = leaf_digest(&concat);

        assert_ne!(
            node, leaf_over_concat,
            "leaf-domain hash of (left++right) must differ from node-domain compress (domain separation)"
        );
    }

    /// Distinct leaves -> distinct roots on small cases (collision sanity).
    #[test]
    fn merkle_poseidon2_distinct_leaves_distinct_roots() {
        let base = make_leaves(8, 3);
        let r0 = MerkleTree::commit(&base).root();

        // Change exactly one value in one leaf.
        for change_pos in [0usize, 3, 7] {
            let mut variant = base.clone();
            variant[change_pos][0] = ((variant[change_pos][0] as u32 + 1) % Q) as u16;
            let r = MerkleTree::commit(&variant).root();
            assert_ne!(r0, r, "changing leaf {change_pos} must change the root");
        }

        // Two entirely different leaf sets.
        let other = make_leaves(8, 3).iter().map(|l| {
            l.iter().map(|&v| ((v as u32 + 5) % Q) as u16).collect()
        }).collect::<Vec<_>>();
        assert_ne!(r0, MerkleTree::commit(&other).root(), "distinct leaf sets must give distinct roots");
    }

    /// Index-bit ordering correctness: for several indices across the tree, the
    /// folding order implied by the LSB-first index bits matches `open`, and a
    /// MANUALLY-FOLDED root (using the documented bit convention) equals the
    /// tree root.
    #[test]
    fn merkle_poseidon2_index_bit_ordering() {
        let n = 32usize;
        let leaves = make_leaves(n, 4);
        let tree = MerkleTree::commit(&leaves);
        let root = tree.root();

        for &idx in &[0usize, 1, 2, 7, 15, 16, 23, 31] {
            let path = tree.open(idx);
            // Manual fold using the documented LSB-first convention.
            let mut acc = leaf_digest(&leaves[idx]);
            let mut pos = idx;
            for sib in &path.siblings {
                acc = if pos & 1 == 0 {
                    compress(&acc, sib) // current is LEFT child
                } else {
                    compress(sib, &acc) // current is RIGHT child
                };
                pos >>= 1;
            }
            assert_eq!(acc, root, "manual LSB-first fold for index {idx} != root");
            // And the public verify agrees.
            assert!(verify(&root, idx, &leaves[idx], &path));
        }
    }

    /// Non-power-of-two leaf counts are padded to the next power of two; all real
    /// leaves still open and verify, and the padded length / depth are correct.
    #[test]
    fn merkle_poseidon2_padding_non_pow2() {
        let n = 13usize; // not a power of two -> pads to 16
        let leaves = make_leaves(n, 4);
        let tree = MerkleTree::commit(&leaves);
        assert_eq!(tree.n_real, 13);
        assert_eq!(tree.n_padded, 16);
        assert_eq!(tree.depth(), 4);
        let root = tree.root();
        for (i, leaf) in leaves.iter().enumerate() {
            let path = tree.open(i);
            assert!(verify(&root, i, leaf, &path), "real leaf {i} must verify in a padded tree");
        }
    }

    /// A single-leaf tree (N = 1) has depth 0, an empty path, and verifies.
    #[test]
    fn merkle_poseidon2_single_leaf() {
        let leaves = make_leaves(1, 6);
        let tree = MerkleTree::commit(&leaves);
        assert_eq!(tree.depth(), 0);
        let root = tree.root();
        assert_eq!(root, leaf_digest(&leaves[0]), "single-leaf root must equal the leaf digest");
        let path = tree.open(0);
        assert!(path.siblings.is_empty(), "single-leaf path must be empty");
        assert!(verify(&root, 0, &leaves[0], &path));
    }
}
