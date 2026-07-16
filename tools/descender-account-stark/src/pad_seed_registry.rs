//! F2 — global, persistent, REORG-SAFE pad-seed admission registry.
//!
//! The mask-PRF hiding seam is opened if the same `(pad_seed, slot_id)` is admitted twice and the
//! two `account_projection` values are differenced. The per-account in-memory guard does NOT prevent
//! this across a chain reorg/restore: an admission that is rolled back and then re-applied (or a
//! competing fork) could repeat a pad-seed. The stress test
//! (`research/cryptanalysis/assumption_boundary_stress_test_20260627.md`, F2) flagged this; the Lean
//! model (`Descender/Path2FullSpendConcrete.lean`, `GlobalPadSeedRegistry` +
//! `padSeed_reuse_excluded_of_globalUnique`) proves the INJECTIVITY half — a globally-injective
//! `admittedFor` map excludes the reuse precondition AT ADMISSION. This module is the deployable
//! counterpart that also supplies the REORG-FREEING half the Lean defers: pad-seeds committed on the
//! canonical chain stay blocked; pad-seeds in orphaned blocks are freed (reusable) on reorg.
//!
//! Invariant (F2): on the canonical chain `admittedFor` is injective — each pad-seed maps to at most
//! one admitted leaf — and an admission is durable until its block leaves the canonical chain.

use std::collections::BTreeMap;

/// A pad-seed digest (the global single-use key). Fixed 32-byte digest of the mask-PRF pad seed.
pub type PadSeed = [u8; 32];
/// The admitted slot-leaf identifier a pad-seed is bound to.
pub type LeafId = u64;
/// Block height at which an admission was committed (canonical-chain position).
pub type BlockHeight = u64;

/// Why an admission was rejected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmitError {
    /// The pad-seed is already admitted on the canonical chain (to some leaf). Reuse is the exact
    /// mask-PRF hiding-reuse precondition F2 excludes.
    PadSeedReused {
        existing_leaf: LeafId,
        existing_height: BlockHeight,
    },
    /// The admission height is not strictly above the current canonical head (admissions are append-
    /// only in height; a non-monotone height would corrupt the reorg-freeing model).
    NonMonotoneHeight { head: BlockHeight, got: BlockHeight },
}

/// Global reorg-safe pad-seed registry (the deployable `GlobalPadSeedRegistry`).
#[derive(Clone, Debug, Default)]
pub struct PadSeedRegistry {
    /// `admittedFor`: pad-seed → (leaf, committed height). Injective on the canonical chain.
    admitted: BTreeMap<PadSeed, (LeafId, BlockHeight)>,
    /// Canonical head height (admissions live at heights `1..=head`; height 0 is genesis/empty).
    head: BlockHeight,
}

impl PadSeedRegistry {
    pub fn new() -> Self {
        Self {
            admitted: BTreeMap::new(),
            head: 0,
        }
    }

    /// Current canonical head height.
    pub fn head(&self) -> BlockHeight {
        self.head
    }

    /// `admittedFor(pad_seed)` — the leaf this pad-seed is admitted for on the canonical chain, if any.
    pub fn admitted_for(&self, pad_seed: &PadSeed) -> Option<LeafId> {
        self.admitted.get(pad_seed).map(|&(leaf, _)| leaf)
    }

    /// Whether a pad-seed is admitted on the canonical chain.
    pub fn contains(&self, pad_seed: &PadSeed) -> bool {
        self.admitted.contains_key(pad_seed)
    }

    /// Number of admitted pad-seeds.
    pub fn len(&self) -> usize {
        self.admitted.len()
    }
    pub fn is_empty(&self) -> bool {
        self.admitted.is_empty()
    }

    /// Admit `pad_seed` for `leaf` at canonical block `height` (must be `> head`). Fails if the
    /// pad-seed is already admitted on the canonical chain (F2 injectivity) — this is the global,
    /// persistent guard the per-account in-memory set lacks. On success `head` advances to `height`.
    pub fn admit(
        &mut self,
        pad_seed: PadSeed,
        leaf: LeafId,
        height: BlockHeight,
    ) -> Result<(), AdmitError> {
        if height <= self.head {
            return Err(AdmitError::NonMonotoneHeight {
                head: self.head,
                got: height,
            });
        }
        if let Some(&(existing_leaf, existing_height)) = self.admitted.get(&pad_seed) {
            return Err(AdmitError::PadSeedReused {
                existing_leaf,
                existing_height,
            });
        }
        self.admitted.insert(pad_seed, (leaf, height));
        self.head = height;
        Ok(())
    }

    /// Reorg the canonical chain back to `new_head`: every pad-seed admitted ABOVE `new_head` was in
    /// a now-orphaned block, so it is FREED (removed) and may be re-admitted on the new fork.
    /// Pad-seeds admitted at heights `<= new_head` survive on the canonical chain and stay blocked.
    /// Returns the number of freed pad-seeds. This is the reorg-safe half: an orphaned admission
    /// cannot keep a pad-seed blocked, and a surviving admission cannot be repeated.
    pub fn reorg_to(&mut self, new_head: BlockHeight) -> usize {
        let before = self.admitted.len();
        self.admitted.retain(|_, &mut (_, h)| h <= new_head);
        self.head = new_head;
        before - self.admitted.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed(b: u8) -> PadSeed {
        let mut s = [0u8; 32];
        s[0] = b;
        s
    }

    /// F2 INJECTIVITY: a pad-seed cannot be admitted twice on the canonical chain (the mask-PRF
    /// hiding-reuse precondition is excluded), even for a different leaf.
    #[test]
    fn rejects_pad_seed_reuse_on_canonical_chain() {
        let mut r = PadSeedRegistry::new();
        r.admit(seed(1), 100, 1).unwrap();
        assert_eq!(r.admitted_for(&seed(1)), Some(100));
        // same pad-seed, different leaf, later block ⇒ rejected.
        let e = r.admit(seed(1), 200, 2).unwrap_err();
        assert_eq!(
            e,
            AdmitError::PadSeedReused {
                existing_leaf: 100,
                existing_height: 1
            }
        );
        // distinct pad-seeds are fine.
        r.admit(seed(2), 200, 2).unwrap();
        assert_eq!(r.len(), 2);
    }

    /// Heights must be strictly monotone (append-only canonical chain).
    #[test]
    fn rejects_non_monotone_height() {
        let mut r = PadSeedRegistry::new();
        r.admit(seed(1), 1, 5).unwrap();
        assert_eq!(
            r.admit(seed(2), 2, 5).unwrap_err(),
            AdmitError::NonMonotoneHeight { head: 5, got: 5 }
        );
        assert_eq!(
            r.admit(seed(3), 3, 4).unwrap_err(),
            AdmitError::NonMonotoneHeight { head: 5, got: 4 }
        );
        r.admit(seed(2), 2, 6).unwrap(); // strictly above head ⇒ ok
    }

    /// REORG-FREEING: pad-seeds in orphaned blocks (above the new head) are freed and re-admittable;
    /// pad-seeds that survive on the canonical chain stay blocked.
    #[test]
    fn reorg_frees_orphaned_but_keeps_canonical() {
        let mut r = PadSeedRegistry::new();
        r.admit(seed(1), 10, 1).unwrap(); // canonical (survives)
        r.admit(seed(2), 20, 2).unwrap(); // orphaned by the reorg below
        r.admit(seed(3), 30, 3).unwrap(); // orphaned by the reorg below
        assert_eq!(r.len(), 3);

        let freed = r.reorg_to(1); // roll back to height 1
        assert_eq!(freed, 2, "seeds at heights 2,3 must be freed");
        assert_eq!(r.head(), 1);

        // survivor stays blocked.
        assert!(r.contains(&seed(1)));
        assert_eq!(
            r.admit(seed(1), 10, 2).unwrap_err(),
            AdmitError::PadSeedReused {
                existing_leaf: 10,
                existing_height: 1
            }
        );
        // orphaned seeds are freed — re-admittable on the new fork (even to a different leaf).
        assert!(!r.contains(&seed(2)));
        r.admit(seed(2), 999, 2).unwrap();
        assert_eq!(r.admitted_for(&seed(2)), Some(999));
    }

    /// A pad-seed orphaned by a reorg and NOT re-admitted on the new fork is reusable later — the
    /// reorg cannot permanently burn a pad-seed that never made the canonical chain.
    #[test]
    fn deep_reorg_then_fresh_admissions() {
        let mut r = PadSeedRegistry::new();
        for i in 1..=10u64 {
            r.admit(seed(i as u8), i, i).unwrap();
        }
        assert_eq!(r.len(), 10);
        let freed = r.reorg_to(3);
        assert_eq!(freed, 7);
        assert_eq!(r.head(), 3);
        // heights 1..=3 survive; 4..=10 freed.
        for i in 1..=3u64 {
            assert!(r.contains(&seed(i as u8)));
        }
        for i in 4..=10u64 {
            assert!(!r.contains(&seed(i as u8)));
        }
        // fresh fork re-admits some freed seeds at new heights.
        r.admit(seed(4), 4444, 4).unwrap();
        r.admit(seed(7), 7777, 5).unwrap();
        assert_eq!(r.admitted_for(&seed(4)), Some(4444));
        assert_eq!(r.admitted_for(&seed(7)), Some(7777));
    }

    /// F2 end-to-end property: across an admit → reorg → re-admit cycle, the canonical chain never
    /// holds the same pad-seed twice (the global injective invariant), so the mask-PRF hiding-reuse
    /// precondition never holds on the canonical chain.
    #[test]
    fn canonical_chain_is_always_pad_seed_injective() {
        let mut r = PadSeedRegistry::new();
        r.admit(seed(1), 1, 1).unwrap();
        r.admit(seed(2), 2, 2).unwrap();
        r.reorg_to(1);
        r.admit(seed(2), 22, 2).unwrap(); // re-admit the freed seed on the new fork
        r.admit(seed(3), 3, 3).unwrap();
        // every pad-seed appears at most once with a single (leaf,height).
        let mut keys: std::vec::Vec<PadSeed> = r.admitted.keys().copied().collect();
        keys.sort();
        keys.dedup();
        assert_eq!(
            keys.len(),
            r.len(),
            "canonical admittedFor must be injective"
        );
    }
}
