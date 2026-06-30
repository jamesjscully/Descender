//! Phase-2 → Phase-3 bridge: aggregate SPEND proofs into one O(1) `chain_state_root`.
//!
//! This connects the spend subsystem (`descender_account_stark::spend_air`) to the generic O(1)
//! block aggregator ([`crate::block_recursive_air`]). The generic recursion machinery
//! (FS transcript + FRI commit-phase fold + Merkle/FRI-layer openings + DEEP/quotient consumer)
//! verifies ANY inner STARK proof — it is not creation-coupled (the only creation coupling is the
//! optional `deep_c_full_leaf` full-C(zeta) re-derivation, which the creation aggregator itself runs
//! WITHOUT). So a spend proof verifies in-circuit at the SAME depth the creation aggregator runs at,
//! and N spend members aggregate to one root through the same proven path.
//!
//! Here each member's IN-CIRCUIT-verified Merkle leaf can be either:
//!
//! * a digest of that spend's actual `packed_witness_commitment` (derived from a real `SpendWitness`
//!   via `spend_air::spend_derive`), for the generic spend-only aggregate; or
//! * a digest of an authenticated creation/spend binding row. [`SpendBindingBlockAir`] then enforces
//!   inside the aggregate AIR that the creation and spend packed-witness commitments match (MED-2)
//!   and that the spend witness key equals the creation-registered checked-core key.
//!
//! The full constraint-level spend C(zeta) mirror now lives in `spend_constraint_air` and is wired
//! into `recursive_air::deep_c_full_spend`; composing that expensive capstone with this paired binding
//! row is the deeper proof-cost rung.

use crate::recursive_air::RecursiveConfig;
use descender_account_stark::checked_core::CC_DIGEST_BYTES;
use descender_account_stark::spend_air::{
    spend_derive, SpendAir, SpendWitness, SPEND_COMMIT_BYTES,
};
use descender_account_stark::{AccountCreationPublic, SpendPublic, Val, DIGEST_ELEMS, SALT_ELEMS};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;

extern crate alloc;
use alloc::vec::Vec;

use crate::merkle::Digest;

/// Map a spend's 32-byte `packed_witness_commitment` to a recursion leaf digest (`DIGEST_ELEMS`
/// lanes), injectively folding all 32 bytes into the lanes (lane i absorbs bytes i, i+13, i+26 in
/// base-256; max lane < 2^24 < p, so canonical). The aggregated member leaf is this commitment.
pub fn spend_leaf_digest(packed: &[u8; 32]) -> Digest {
    core::array::from_fn(|i| {
        let mut acc = packed[i] as u32;
        if i + DIGEST_ELEMS < 32 {
            acc += (packed[i + DIGEST_ELEMS] as u32) << 8;
        }
        if i + 2 * DIGEST_ELEMS < 32 {
            acc += (packed[i + 2 * DIGEST_ELEMS] as u32) << 16;
        }
        Val::from_u32(acc)
    })
}

/// Build the `recursion` member window for one spend: a Merkle opening whose opened leaf is the
/// spend's `packed_witness_commitment` digest. Mirrors `block_recursive_air::windows_for_settlement`
/// with the spend-derived leaf.
fn spend_member_window(
    cfg: &RecursiveConfig,
    levels: usize,
    member: u32,
    leaf: Digest,
) -> Vec<Val> {
    use crate::challenger::RATE;
    use crate::merkle::compressor;
    use crate::recursive_air::{pack_window, WindowInputs};
    use descender_account_stark::{Challenge, SALT_ELEMS, WIDTH};
    use p3_field::BasedVectorSpace;
    use p3_symmetric::PseudoCompressionFunction;

    let d = |seed: u32| -> Digest {
        core::array::from_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 + 1))
        })
    };
    let ext = |seed: u32| -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(40503).wrapping_add(i as u32 + 1))
        })
    };
    let c = compressor();
    let leaf_count = 1usize << levels;
    let idx = (member as usize) % leaf_count;
    // Sibling leaves are filler; the opened leaf at `idx` is the spend's packed-witness commitment.
    let leaves: Vec<Digest> = (0..leaf_count as u32)
        .map(|x| {
            if x as usize == idx {
                leaf
            } else {
                d(x + member * 1000 + 1)
            }
        })
        .collect();
    let mut levels_vec = vec![leaves.clone()];
    let mut cur = leaves.clone();
    while cur.len() > 1 {
        cur = cur
            .chunks_exact(2)
            .map(|p| c.compress([p[0], p[1]]))
            .collect();
        levels_vec.push(cur.clone());
    }
    let mut sibs = Vec::new();
    let mut k = idx;
    for level in &levels_vec[..levels_vec.len() - 1] {
        sibs.push(level[k ^ 1]);
        k >>= 1;
    }
    let inp = WindowInputs {
        draw_out: Val::from_u32(idx as u32),
        e0: ext(1),
        e1: ext(2),
        beta: ext(3),
        descent: &[],
        merkle_leaf: leaf,
        merkle_siblings: &sibs,
        deep_zeta: Challenge::ZERO,
        deep_q: Challenge::ZERO,
        deep_alpha: Challenge::ZERO,
        fs_blocks: &[],
        sponge_in: [Val::ZERO; WIDTH],
        prior_state: [Val::ZERO; WIDTH],
        absorbed: [Val::ZERO; RATE],
        prefix_blocks: &[],
        public_io: &[],
        fs_beta_blocks: &[],
        fs_ood_blocks: &[],
        fs_ood_partial_block: [Val::ZERO; RATE],
        fs_final_blocks: &[],
        fs_final_partial_block: [Val::ZERO; RATE],
        inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
        layer_opens: &[],
        final_poly_x: Val::ZERO,
        final_poly_coeffs: &[],
        leaf_row: &[],
        leaf_salt: [Val::ZERO; SALT_ELEMS],
        dr_value_at_zeta: &[],
        dr_value_at_zeta_next: &[],
        dr_x: Val::ZERO,
        lc_value_at_zeta: &[],
        lc_c_committed: Challenge::ZERO,
        sc_value_at_zeta: &[],
        sc_public_values: &[],
        sc_c_committed: Challenge::ZERO,
        quot_commit: [Val::ZERO; DIGEST_ELEMS],
        quot_opened_at_x: &[],
        quot_at_zeta: &[],
        quot_zps: &[],
        quot_salt: [Val::ZERO; SALT_ELEMS],
        quot_x: Val::ZERO,
        quot_siblings: &[],
    };
    pack_window(cfg, &inp)
}

/// Build member windows for a block of spends: each window opens (in-circuit) a Merkle tree whose
/// leaf is that spend's `packed_witness_commitment` digest. `air` supplies the Poseidon2 constants
/// for `spend_derive` (it re-derives the challenge → syndrome → packed witness from each witness).
pub fn windows_for_spend(
    cfg: &RecursiveConfig,
    levels: usize,
    air: &SpendAir,
    spends: &[SpendWitness],
) -> Vec<Vec<Val>> {
    spends
        .iter()
        .enumerate()
        .map(|(m, w)| {
            let packed = spend_derive(air, w).packed;
            spend_member_window(cfg, levels, m as u32, spend_leaf_digest(&packed))
        })
        .collect()
}

/// Creation/spend binding leaf layout:
/// `creation.packed_witness_commitment || spend.packed_witness_commitment ||
///  creation.checked_core_key_commitment || spend.witness_commitment`.
pub const BIND_CREATION_PACKED: usize = 0;
pub const BIND_SPEND_PACKED: usize = BIND_CREATION_PACKED + SPEND_COMMIT_BYTES;
pub const BIND_CREATION_KEY: usize = BIND_SPEND_PACKED + SPEND_COMMIT_BYTES;
pub const BIND_SPEND_KEY: usize = BIND_CREATION_KEY + CC_DIGEST_BYTES;
pub const BINDING_LEAF_ROW_LEN: usize = BIND_SPEND_KEY + CC_DIGEST_BYTES;
const _: () = assert!(SPEND_COMMIT_BYTES == CC_DIGEST_BYTES);

/// Public fields the aggregate AIR must bind across the creation and spend proofs.
#[derive(Clone)]
pub struct CreationSpendBinding {
    pub creation_packed_witness: [Val; SPEND_COMMIT_BYTES],
    pub spend_packed_witness: [Val; SPEND_COMMIT_BYTES],
    pub creation_key_commitment: [Val; CC_DIGEST_BYTES],
    pub spend_witness_commitment: [Val; CC_DIGEST_BYTES],
}

/// Convert settlement-layer public statements into the aggregate binding row.
pub fn binding_from_publics(
    creation: &AccountCreationPublic,
    spend: &SpendPublic,
) -> CreationSpendBinding {
    CreationSpendBinding {
        creation_packed_witness: creation.packed_witness_commitment,
        spend_packed_witness: core::array::from_fn(|i| {
            Val::from_u32(spend.packed_witness_commitment[i] as u32)
        }),
        creation_key_commitment: creation.checked_core_key_commitment,
        spend_witness_commitment: core::array::from_fn(|i| {
            Val::from_u32(spend.witness_commitment[i] as u32)
        }),
    }
}

pub fn binding_leaf_row(binding: &CreationSpendBinding) -> [Val; BINDING_LEAF_ROW_LEN] {
    let mut row = [Val::ZERO; BINDING_LEAF_ROW_LEN];
    row[BIND_CREATION_PACKED..BIND_SPEND_PACKED].copy_from_slice(&binding.creation_packed_witness);
    row[BIND_SPEND_PACKED..BIND_CREATION_KEY].copy_from_slice(&binding.spend_packed_witness);
    row[BIND_CREATION_KEY..BIND_SPEND_KEY].copy_from_slice(&binding.creation_key_commitment);
    row[BIND_SPEND_KEY..BINDING_LEAF_ROW_LEN].copy_from_slice(&binding.spend_witness_commitment);
    row
}

fn binding_leaf_salt() -> [Val; SALT_ELEMS] {
    [Val::ZERO; SALT_ELEMS]
}

pub fn binding_leaf_digest(binding: &CreationSpendBinding) -> Digest {
    let mut input = binding_leaf_row(binding).to_vec();
    input.extend_from_slice(&binding_leaf_salt());
    crate::leaf_hash::leaf_hash_digest(&input)
}

/// Member config for aggregate creation/spend binding rows. The recursive window authenticates the
/// binding row through `merkle_leaf == MyHash(row || zero_salt)` and a Merkle path; the aggregate AIR
/// then interprets the authenticated row and enforces MED-2 + key equality.
pub fn cfg_creation_spend_binding_settlement(levels: usize) -> RecursiveConfig {
    RecursiveConfig {
        merkle_levels: levels,
        leaf_row_len: BINDING_LEAF_ROW_LEN,
        ..RecursiveConfig::single_window(levels)
    }
}

/// Member config for the full spend capstone plus authenticated creation/spend binding rows.
/// The recursive window authenticates the same binding row as
/// [`cfg_creation_spend_binding_settlement`], and also carries the production spend `C(zeta)` mirror
/// through `RecursiveConfig::deep_c_full_spend`.
pub fn cfg_creation_spend_binding_full_spend_settlement(
    levels: usize,
    deep_log_degree: usize,
) -> RecursiveConfig {
    let mut cfg = cfg_creation_spend_binding_settlement(levels);
    cfg.deep_log_degree = Some(deep_log_degree);
    cfg.deep_c_full_spend = true;
    cfg
}

fn binding_member_window(
    cfg: &RecursiveConfig,
    levels: usize,
    member: u32,
    binding: &CreationSpendBinding,
) -> Vec<Val> {
    use crate::challenger::RATE;
    use crate::merkle::compressor;
    use crate::recursive_air::{pack_window, WindowInputs};
    use descender_account_stark::{Challenge, WIDTH};
    use p3_field::BasedVectorSpace;
    use p3_symmetric::PseudoCompressionFunction;

    assert_eq!(
        cfg.leaf_row_len, BINDING_LEAF_ROW_LEN,
        "binding member cfg must carry the binding leaf-row width"
    );
    let d = |seed: u32| -> Digest {
        core::array::from_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 + 1))
        })
    };
    let ext = |seed: u32| -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(40503).wrapping_add(i as u32 + 1))
        })
    };
    let leaf = binding_leaf_digest(binding);
    let leaf_row = binding_leaf_row(binding);
    let leaf_salt = binding_leaf_salt();
    let c = compressor();
    let leaf_count = 1usize << levels;
    let idx = (member as usize) % leaf_count;
    let leaves: Vec<Digest> = (0..leaf_count as u32)
        .map(|x| {
            if x as usize == idx {
                leaf
            } else {
                d(x + member * 1000 + 1)
            }
        })
        .collect();
    let mut levels_vec = vec![leaves.clone()];
    let mut cur = leaves.clone();
    while cur.len() > 1 {
        cur = cur
            .chunks_exact(2)
            .map(|p| c.compress([p[0], p[1]]))
            .collect();
        levels_vec.push(cur.clone());
    }
    let mut sibs = Vec::new();
    let mut k = idx;
    for level in &levels_vec[..levels_vec.len() - 1] {
        sibs.push(level[k ^ 1]);
        k >>= 1;
    }
    let inp = WindowInputs {
        draw_out: Val::from_u32(idx as u32),
        e0: ext(1),
        e1: ext(2),
        beta: ext(3),
        descent: &[],
        merkle_leaf: leaf,
        merkle_siblings: &sibs,
        deep_zeta: Challenge::ZERO,
        deep_q: Challenge::ZERO,
        deep_alpha: Challenge::ZERO,
        fs_blocks: &[],
        sponge_in: [Val::ZERO; WIDTH],
        prior_state: [Val::ZERO; WIDTH],
        absorbed: [Val::ZERO; RATE],
        prefix_blocks: &[],
        public_io: &[],
        fs_beta_blocks: &[],
        fs_ood_blocks: &[],
        fs_ood_partial_block: [Val::ZERO; RATE],
        fs_final_blocks: &[],
        fs_final_partial_block: [Val::ZERO; RATE],
        inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
        layer_opens: &[],
        final_poly_x: Val::ZERO,
        final_poly_coeffs: &[],
        leaf_row: &leaf_row,
        leaf_salt,
        dr_value_at_zeta: &[],
        dr_value_at_zeta_next: &[],
        dr_x: Val::ZERO,
        lc_value_at_zeta: &[],
        lc_c_committed: descender_account_stark::Challenge::ZERO,
        sc_value_at_zeta: &[],
        sc_public_values: &[],
        sc_c_committed: descender_account_stark::Challenge::ZERO,
        quot_commit: [Val::ZERO; DIGEST_ELEMS],
        quot_opened_at_x: &[],
        quot_at_zeta: &[],
        quot_zps: &[],
        quot_salt: [Val::ZERO; SALT_ELEMS],
        quot_x: Val::ZERO,
        quot_siblings: &[],
    };
    pack_window(cfg, &inp)
}

pub fn windows_for_creation_spend_bindings(
    cfg: &RecursiveConfig,
    levels: usize,
    bindings: &[CreationSpendBinding],
) -> Vec<Vec<Val>> {
    bindings
        .iter()
        .enumerate()
        .map(|(m, binding)| binding_member_window(cfg, levels, m as u32, binding))
        .collect()
}

/// O(1) block aggregator with creation/spend cross-binding over each authenticated member row.
pub struct SpendBindingBlockAir {
    pub cfg: RecursiveConfig,
}

impl<F: Sync> BaseAir<F> for SpendBindingBlockAir {
    fn width(&self) -> usize {
        crate::block_recursive_air::n_cols(&self.cfg)
    }
    fn num_public_values(&self) -> usize {
        crate::block_recursive_air::N_PUBLIC
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for SpendBindingBlockAir {
    fn eval(&self, builder: &mut AB) {
        assert_eq!(
            self.cfg.leaf_row_len, BINDING_LEAF_ROW_LEN,
            "SpendBindingBlockAir requires cfg_creation_spend_binding_settlement"
        );
        crate::block_recursive_air::BlockRecursiveAir {
            cfg: self.cfg.clone(),
        }
        .eval(builder);

        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let row = crate::recursive_air::window_leaf_row_col(&self.cfg);
        for i in 0..SPEND_COMMIT_BYTES {
            let creation: AB::Expr = local[row + BIND_CREATION_PACKED + i].into();
            let spend: AB::Expr = local[row + BIND_SPEND_PACKED + i].into();
            builder.assert_zero(creation - spend);
        }
        for i in 0..CC_DIGEST_BYTES {
            let creation: AB::Expr = local[row + BIND_CREATION_KEY + i].into();
            let spend: AB::Expr = local[row + BIND_SPEND_KEY + i].into();
            builder.assert_zero(creation - spend);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_recursive_air::{
        build_trace, cfg_merkle_settlement, n_cols, trace_root, BlockRecursiveAir,
    };
    use descender_account_stark::spend_air::{generate_spend_trace, SpendWitness};
    use descender_account_stark::Challenge;
    use p3_air::check_all_constraints;
    use p3_koala_bear::{
        KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL, KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
        KOALABEAR_POSEIDON2_RC_16_INTERNAL,
    };
    use p3_matrix::dense::RowMajorMatrix;

    fn spend_air() -> SpendAir {
        SpendAir::from_arrays(
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_INITIAL,
            KOALABEAR_POSEIDON2_RC_16_INTERNAL,
            KOALABEAR_POSEIDON2_RC_16_EXTERNAL_FINAL,
        )
    }

    fn spend_witness(seed: u32) -> SpendWitness {
        SpendWitness {
            sigma_w: core::array::from_fn(|i| {
                ((seed.wrapping_mul(149).wrapping_add(i as u32) + 29) % 12289) as u16
            }),
            sigma_t: core::array::from_fn(|i| {
                ((seed.wrapping_mul(71).wrapping_add(i as u32) + 5) % 12289) as u16
            }),
            projection: core::array::from_fn(|i| {
                ((seed.wrapping_mul(211).wrapping_add(i as u32) + 5) % 12289) as u16
            }),
            account_id: core::array::from_fn(|i| {
                (i as u8).wrapping_mul(13).wrapping_add(seed as u8)
            }),
            slot_id: 0x42 + seed as u64,
            account_state_root: core::array::from_fn(|i| {
                (i as u8).wrapping_mul(17).wrapping_add(7)
            }),
            message_digest: core::array::from_fn(|i| {
                (i as u8).wrapping_mul(7).wrapping_add(seed as u8)
            }),
        }
    }

    fn ext(seed: u32) -> Challenge {
        use p3_field::BasedVectorSpace;
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(40503).wrapping_add(i as u32 + 1))
        })
    }

    fn lifted_spend_opening(
        air: &SpendAir,
        witness: &SpendWitness,
    ) -> (Vec<Challenge>, Vec<Challenge>) {
        let (trace, public_values, _frame_digest) = generate_spend_trace(air, witness);
        let width = air.trace_width();
        assert_eq!(
            width,
            crate::spend_constraint_air::production_trace_width(),
            "production spend width drift"
        );
        assert_eq!(
            public_values.len(),
            crate::spend_constraint_air::production_public_count(),
            "production spend public width drift"
        );
        let cells: Vec<Challenge> = trace.values[..width]
            .iter()
            .map(|&v| Challenge::from(v))
            .collect();
        let pubvals: Vec<Challenge> = public_values.iter().map(|&v| Challenge::from(v)).collect();
        (cells, pubvals)
    }

    fn binding_full_spend_member_window(
        cfg: &RecursiveConfig,
        levels: usize,
        member: u32,
        binding: &CreationSpendBinding,
        cells: &[Challenge],
        pubvals: &[Challenge],
        alpha: Challenge,
        zeta: Challenge,
        c_committed: Challenge,
    ) -> Vec<Val> {
        use crate::challenger::RATE;
        use crate::merkle::compressor;
        use crate::recursive_air::{pack_window, WindowInputs};
        use descender_account_stark::WIDTH;
        use p3_symmetric::PseudoCompressionFunction;

        assert_eq!(
            cfg.leaf_row_len, BINDING_LEAF_ROW_LEN,
            "full-spend binding cfg must carry the binding leaf-row width"
        );
        assert!(
            cfg.deep_c_full_spend,
            "full-spend binding cfg must enable deep_c_full_spend"
        );
        let d = |seed: u32| -> Digest {
            core::array::from_fn(|i| {
                Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 + 1))
            })
        };
        let leaf = binding_leaf_digest(binding);
        let leaf_row = binding_leaf_row(binding);
        let leaf_salt = binding_leaf_salt();
        let c = compressor();
        let leaf_count = 1usize << levels;
        let idx = (member as usize) % leaf_count;
        let leaves: Vec<Digest> = (0..leaf_count as u32)
            .map(|x| {
                if x as usize == idx {
                    leaf
                } else {
                    d(x + member * 1000 + 1)
                }
            })
            .collect();
        let mut levels_vec = vec![leaves.clone()];
        let mut cur = leaves;
        while cur.len() > 1 {
            cur = cur
                .chunks_exact(2)
                .map(|p| c.compress([p[0], p[1]]))
                .collect();
            levels_vec.push(cur.clone());
        }
        let mut sibs = Vec::new();
        let mut k = idx;
        for level in &levels_vec[..levels_vec.len() - 1] {
            sibs.push(level[k ^ 1]);
            k >>= 1;
        }
        let inp = WindowInputs {
            draw_out: Val::from_u32(idx as u32),
            e0: ext(1),
            e1: ext(2),
            beta: ext(3),
            descent: &[],
            merkle_leaf: leaf,
            merkle_siblings: &sibs,
            deep_zeta: zeta,
            deep_q: Challenge::ZERO,
            deep_alpha: alpha,
            fs_blocks: &[],
            sponge_in: [Val::ZERO; WIDTH],
            prior_state: [Val::ZERO; WIDTH],
            absorbed: [Val::ZERO; RATE],
            prefix_blocks: &[],
            public_io: &[],
            fs_beta_blocks: &[],
            fs_ood_blocks: &[],
            fs_ood_partial_block: [Val::ZERO; RATE],
            fs_final_blocks: &[],
            fs_final_partial_block: [Val::ZERO; RATE],
            inner_main_commit: [Val::ZERO; DIGEST_ELEMS],
            layer_opens: &[],
            final_poly_x: Val::ZERO,
            final_poly_coeffs: &[],
            leaf_row: &leaf_row,
            leaf_salt,
            dr_value_at_zeta: &[],
            dr_value_at_zeta_next: &[],
            dr_x: Val::ZERO,
            lc_value_at_zeta: &[],
            lc_c_committed: Challenge::ZERO,
            sc_value_at_zeta: cells,
            sc_public_values: pubvals,
            sc_c_committed: c_committed,
            quot_commit: [Val::ZERO; DIGEST_ELEMS],
            quot_opened_at_x: &[],
            quot_at_zeta: &[],
            quot_zps: &[],
            quot_salt: [Val::ZERO; SALT_ELEMS],
            quot_x: Val::ZERO,
            quot_siblings: &[],
        };
        pack_window(cfg, &inp)
    }

    fn full_spend_binding_window_for_seed(
        cfg: &RecursiveConfig,
        levels: usize,
        member: u32,
        seed: u32,
        alpha: Challenge,
        zeta: Challenge,
        c_committed: Challenge,
    ) -> (
        Vec<Val>,
        CreationSpendBinding,
        Vec<Challenge>,
        Vec<Challenge>,
    ) {
        let a = spend_air();
        let witness = spend_witness(seed);
        let binding = binding_for_seed(&a, seed);
        let (cells, pubvals) = lifted_spend_opening(&a, &witness);
        let row = binding_full_spend_member_window(
            cfg,
            levels,
            member,
            &binding,
            &cells,
            &pubvals,
            alpha,
            zeta,
            c_committed,
        );
        (row, binding, cells, pubvals)
    }

    /// `spend_leaf_digest` is injective over distinct packed witnesses and canonical.
    #[test]
    fn spend_leaf_digest_is_canonical_and_distinguishes() {
        let a = spend_air();
        let p0 = spend_derive(&a, &spend_witness(1)).packed;
        let p1 = spend_derive(&a, &spend_witness(2)).packed;
        assert_ne!(p0, p1, "distinct spends should pack differently");
        assert_ne!(
            spend_leaf_digest(&p0),
            spend_leaf_digest(&p1),
            "leaf digests must differ"
        );
    }

    fn binding_for_seed(air: &SpendAir, seed: u32) -> CreationSpendBinding {
        let d = spend_derive(air, &spend_witness(seed));
        let packed = core::array::from_fn(|i| Val::from_u32(d.packed[i] as u32));
        let key = core::array::from_fn(|i| Val::from_u32(d.witness_commitment[i] as u32));
        CreationSpendBinding {
            creation_packed_witness: packed,
            spend_packed_witness: packed,
            creation_key_commitment: key,
            spend_witness_commitment: key,
        }
    }

    #[test]
    fn creation_spend_binding_block_accepts_honest_rows() {
        let levels = 2usize;
        let cfg = cfg_creation_spend_binding_settlement(levels);
        let a = spend_air();
        let bindings: Vec<CreationSpendBinding> = (0..4u32)
            .map(|seed| binding_for_seed(&a, seed + 1))
            .collect();
        let ws = windows_for_creation_spend_bindings(&cfg, levels, &bindings);
        let flat = build_trace(&cfg, &ws);
        let root = trace_root(&cfg, ws.len(), &flat);
        let m = RowMajorMatrix::new(flat, n_cols(&cfg));
        let report = check_all_constraints(&SpendBindingBlockAir { cfg }, &m, &root.to_vec(), None);
        assert!(
            report.is_ok(),
            "honest creation/spend binding block rejected: {} failures",
            report.failures.len()
        );
    }

    #[test]
    fn creation_spend_binding_block_rejects_wrong_med2_or_key() {
        let levels = 1usize;
        let cfg = cfg_creation_spend_binding_settlement(levels);
        let a = spend_air();
        let good = binding_for_seed(&a, 7);

        let mut wrong_packed = good.clone();
        wrong_packed.spend_packed_witness[0] += Val::ONE;
        let mut wrong_key = good;
        wrong_key.spend_witness_commitment[0] += Val::ONE;

        for (label, binding) in [("MED-2 packed witness", wrong_packed), ("key", wrong_key)] {
            let ws = windows_for_creation_spend_bindings(&cfg, levels, &[binding]);
            let flat = build_trace(&cfg, &ws);
            let root = trace_root(&cfg, ws.len(), &flat);
            let m = RowMajorMatrix::new(flat.clone(), n_cols(&cfg));

            let generic = check_all_constraints(
                &BlockRecursiveAir { cfg: cfg.clone() },
                &m,
                &root.to_vec(),
                None,
            );
            assert!(
                generic.is_ok(),
                "control failed: generic block aggregator should accept the semantically unbound {label} row"
            );

            let bound = check_all_constraints(
                &SpendBindingBlockAir { cfg: cfg.clone() },
                &RowMajorMatrix::new(flat, n_cols(&cfg)),
                &root.to_vec(),
                None,
            );
            assert!(
                !bound.is_ok(),
                "binding block accepted a mismatched {label} row"
            );
        }
    }

    /// FAST PACKING CAPSTONE: one recursive member window simultaneously authenticates the
    /// creation/spend binding row and carries a real production spend opening for the full
    /// `deep_c_full_spend` mirror. The full recursive evaluator/prover checks are intentionally kept
    /// in ignored tests because replaying the production spend mirror inside `RecursiveAir` is slow.
    #[test]
    fn creation_spend_binding_full_spend_window_packs_real_opening() {
        let levels = 1usize;
        let cfg = cfg_creation_spend_binding_full_spend_settlement(levels, 1);
        let alpha = ext(221);
        let zeta = ext(231);
        let c_committed = ext(19);
        let (row, binding, cells, pubvals) =
            full_spend_binding_window_for_seed(&cfg, levels, 0, 17, alpha, zeta, c_committed);

        let row_base = crate::recursive_air::window_leaf_row_col(&cfg);
        assert_eq!(
            &row[row_base..row_base + BINDING_LEAF_ROW_LEN],
            &binding_leaf_row(&binding),
            "authenticated binding leaf row not packed into recursive window"
        );
        assert_eq!(
            crate::spend_constraint_air::production_c_partial(&cells, &pubvals, alpha),
            Challenge::ZERO,
            "honest production spend opening must have zero full C_partial"
        );
    }

    /// SLOW CAPSTONE: a production-q128 aggregate proof whose member windows combine the authenticated
    /// creation/spend binding row (MED-2 + checked-core key equality) with the production
    /// `deep_c_full_spend` mirror. This is the spend-side analogue of the full-C(zeta) aggregate rung:
    /// one `chain_state_root` proof attests the paired binding rows and the full spend relation mirror
    /// for each member. Ignored by default because proving the full spend mirror inside the aggregate
    /// is minute-scale or worse in debug/local CI.
    #[test]
    #[ignore = "slow: production q128 aggregate proof with full spend C(zeta) mirror"]
    fn proves_creation_spend_binding_full_spend_block_q128() {
        use descender_account_stark::{
            MyCompress, MyHash, Perm, PROD_LOG_BLOWUP, PROD_NUM_QUERIES, PROD_QUERY_POW_BITS,
        };
        use p3_challenger::DuplexChallenger;
        use p3_commit::ExtensionMmcs;
        use p3_dft::Radix2DitParallel;
        use p3_field::Field;
        use p3_fri::{FriParameters, TwoAdicFriPcs};
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_uni_stark::{prove, verify, StarkConfig};

        type ValMmcs = MerkleTreeMmcs<
            <Val as Field>::Packing,
            <Val as Field>::Packing,
            MyHash,
            MyCompress,
            2,
            DIGEST_ELEMS,
        >;
        type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 8>;
        type OuterConfig = StarkConfig<Pcs, Challenge, MyChallenger>;

        let perm: Perm = default_koalabear_poseidon2_16();
        let val_mmcs = ValMmcs::new(
            MyHash::new(perm.clone()),
            MyCompress::new(MyHash::new(perm.clone())),
            0,
        );
        let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
        let fri_params = FriParameters {
            log_blowup: PROD_LOG_BLOWUP,
            log_final_poly_len: 0,
            max_log_arity: 1,
            num_queries: PROD_NUM_QUERIES,
            commit_proof_of_work_bits: 0,
            query_proof_of_work_bits: PROD_QUERY_POW_BITS,
            mmcs: challenge_mmcs,
        };
        let pcs = Pcs::new(DftT::default(), val_mmcs, fri_params);
        let config = OuterConfig::new(pcs, MyChallenger::new(perm.clone()));

        let levels = 1usize;
        let cfg = cfg_creation_spend_binding_full_spend_settlement(levels, 1);
        let alpha = ext(311);
        let zeta = ext(331);
        let c_committed = ext(37);
        let ws: Vec<Vec<Val>> = (0..2u32)
            .map(|member| {
                full_spend_binding_window_for_seed(
                    &cfg,
                    levels,
                    member,
                    member + 41,
                    alpha,
                    zeta,
                    c_committed,
                )
                .0
            })
            .collect();
        let n = ws.len();
        let flat = build_trace(&cfg, &ws);
        let root = trace_root(&cfg, n, &flat);
        let trace = RowMajorMatrix::new(flat, n_cols(&cfg));
        let proof = prove(
            &config,
            &SpendBindingBlockAir { cfg: cfg.clone() },
            trace,
            &root.to_vec(),
        );
        verify(
            &config,
            &SpendBindingBlockAir { cfg },
            &proof,
            &root.to_vec(),
        )
        .expect(
            "creation/spend binding full-spend block must prove+verify at production q128 knobs",
        );
    }

    /// O(1) SPEND BLOCK: a block of real spend witnesses aggregates to ONE chain_state_root, with
    /// each member's packed-witness commitment verified in-circuit (Merkle opening) by the generic
    /// recursion — proven + verified at PRODUCTION q128 knobs. The spend analogue of
    /// `block_recursive_air::proves_at_production_q128_knobs`.
    #[test]
    fn proves_spend_block_q128() {
        use descender_account_stark::{
            MyCompress, MyHash, Perm, PROD_LOG_BLOWUP, PROD_NUM_QUERIES, PROD_QUERY_POW_BITS,
        };
        use p3_challenger::DuplexChallenger;
        use p3_commit::ExtensionMmcs;
        use p3_dft::Radix2DitParallel;
        use p3_field::Field;
        use p3_fri::{FriParameters, TwoAdicFriPcs};
        use p3_koala_bear::default_koalabear_poseidon2_16;
        use p3_merkle_tree::MerkleTreeMmcs;
        use p3_uni_stark::{prove, verify, StarkConfig};

        type ValMmcs = MerkleTreeMmcs<
            <Val as Field>::Packing,
            <Val as Field>::Packing,
            MyHash,
            MyCompress,
            2,
            DIGEST_ELEMS,
        >;
        type ChallengeMmcs = ExtensionMmcs<Val, descender_account_stark::Challenge, ValMmcs>;
        type DftT = Radix2DitParallel<Val>;
        type Pcs = TwoAdicFriPcs<Val, DftT, ValMmcs, ChallengeMmcs>;
        type MyChallenger = DuplexChallenger<Val, Perm, 16, 8>;
        type OuterConfig = StarkConfig<Pcs, descender_account_stark::Challenge, MyChallenger>;

        let perm: Perm = default_koalabear_poseidon2_16();
        let val_mmcs = ValMmcs::new(
            MyHash::new(perm.clone()),
            MyCompress::new(MyHash::new(perm.clone())),
            0,
        );
        let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
        let fri_params = FriParameters {
            log_blowup: PROD_LOG_BLOWUP,
            log_final_poly_len: 0,
            max_log_arity: 1,
            num_queries: PROD_NUM_QUERIES,
            commit_proof_of_work_bits: 0,
            query_proof_of_work_bits: PROD_QUERY_POW_BITS,
            mmcs: challenge_mmcs,
        };
        let pcs = Pcs::new(DftT::default(), val_mmcs, fri_params);
        let config = OuterConfig::new(pcs, MyChallenger::new(perm.clone()));

        let levels = 3usize;
        let cfg = cfg_merkle_settlement(levels);
        let sair = spend_air();
        let spends: Vec<SpendWitness> = (0..4u32).map(spend_witness).collect();
        let ws = windows_for_spend(&cfg, levels, &sair, &spends);
        let n = ws.len();
        let flat = build_trace(&cfg, &ws);
        let root = trace_root(&cfg, n, &flat);
        let trace = RowMajorMatrix::new(flat, n_cols(&cfg));
        let proof = prove(
            &config,
            &BlockRecursiveAir { cfg: cfg.clone() },
            trace,
            &root.to_vec(),
        );
        verify(&config, &BlockRecursiveAir { cfg }, &proof, &root.to_vec())
            .expect("spend block must prove+verify at production q128 knobs");
    }
}
