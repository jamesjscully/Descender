//! Shape constants for the C2 WIDE-THEN-REDUCE mask gadget + the C4 carrier mod-q reduction,
//! MIRRORED from [`descender_account_stark::reduce_gadget`] (the deployed leaf gadget) and from the
//! leaf carrier block (`air.rs:856-906`).
//!
//! These are PINned against the leaf by [`tests::reduce_shape_matches_leaf`] (REDUCE_CELLS equal)
//! and exercised end-to-end by the leaf-constraint mirror's reduce fidelity tests. The mirror in
//! [`crate::leaf_constraint_air`] turns every `eval_reduce_gadget` / carrier `assert_eq` /
//! `assert_bool` into an octic residual; the per-cell OFFSETS within one Horner step are reproduced
//! here so the mirror reads exactly the cells the leaf gadget writes.

/// q12289 modulus for the projection-mask lanes. Mirrors `reduce_gadget::Q12289`.
pub const Q12289: u32 = 12289;
/// Number of projection-mask lanes. Mirrors `reduce_gadget::NUM_OUTPUTS`.
pub const NUM_OUTPUTS: usize = 6;
/// Squeezed field lanes per mask (base-`p` digits of `W`). Mirrors `reduce_gadget::LANES_PER_MASK`.
pub const LANES_PER_MASK: usize = 6;
/// Total squeezed field lanes = 6 masks * 6 lanes each. Mirrors `reduce_gadget::NUM_SQUEEZE`.
pub const NUM_SQUEEZE: usize = NUM_OUTPUTS * LANES_PER_MASK; // 36
/// `mask < q12289` (~14-bit). Mirrors `reduce_gadget::MASK_BITS`.
pub const MASK_BITS: usize = 14;

// --- wide-then-reduce limb/carry gadget constants (base 2^16) — mirror reduce_gadget.rs ---
pub const LIMB_BASE_BITS: usize = 16;
pub const LO_BITS: usize = 16;
pub const HI_BITS: usize = 15;
pub const S_LO_BITS: usize = 16;
pub const S_TOP_BITS: usize = 14;
pub const HORNER_STEPS: usize = LANES_PER_MASK; // 6

/// Field cells per Horner step (scalar values, NOT counting bit cells). Mirrors
/// `reduce_gadget::STEP_VALS`.
pub const STEP_VALS: usize = 11;
/// Bit cells per Horner step. Mirrors `reduce_gadget::STEP_BITS`.
pub const STEP_BITS: usize = LO_BITS
    + HI_BITS
    + LO_BITS
    + HI_BITS
    + S_LO_BITS
    + S_LO_BITS
    + S_TOP_BITS
    + S_TOP_BITS // c0
    + S_TOP_BITS // c1
    + S_TOP_BITS // d0
    + S_TOP_BITS; // d1
/// `r_t` value + bits + slack value + bits. Mirrors `reduce_gadget::STEP_R`.
pub const STEP_R: usize = 1 + MASK_BITS + 1 + MASK_BITS;
/// Total field cells per Horner step. Mirrors `reduce_gadget::STEP_CELLS`.
pub const STEP_CELLS: usize = STEP_VALS + STEP_BITS + STEP_R;
/// Per mask lane: 6 squeezed `lane` value copies + 6 Horner steps. Mirrors
/// `reduce_gadget::PER_MASK_CELLS`.
pub const PER_MASK_CELLS: usize = LANES_PER_MASK + HORNER_STEPS * STEP_CELLS;
/// All 6 mask lanes' reduce-gadget columns. Mirrors `reduce_gadget::REDUCE_CELLS`.
pub const REDUCE_CELLS: usize = NUM_OUTPUTS * PER_MASK_CELLS;

/// Offsets within ONE Horner step block (relative to the step base). MIRRORS
/// `reduce_gadget::StepOff` EXACTLY — a wrong offset reads the wrong cell ⇒ caught by the honest
/// fidelity pin in `leaf_constraint_air`.
pub struct StepOff;
impl StepOff {
    pub const QT_LO: usize = 0;
    pub const QT_HI: usize = 1;
    pub const LANE_LO: usize = 2;
    pub const LANE_HI: usize = 3;
    pub const S0: usize = 4;
    pub const S1: usize = 5;
    pub const S2: usize = 6;
    pub const C0: usize = 7; // LHS carries
    pub const C1: usize = 8;
    pub const D0: usize = 9; // RHS carries
    pub const D1: usize = 10;
    // bit blocks start after the value cells:
    pub const QT_LO_BITS: usize = STEP_VALS;
    pub const QT_HI_BITS: usize = Self::QT_LO_BITS + LO_BITS;
    pub const LANE_LO_BITS: usize = Self::QT_HI_BITS + HI_BITS;
    pub const LANE_HI_BITS: usize = Self::LANE_LO_BITS + LO_BITS;
    pub const S0_BITS: usize = Self::LANE_HI_BITS + HI_BITS;
    pub const S1_BITS: usize = Self::S0_BITS + S_LO_BITS;
    pub const S2_BITS: usize = Self::S1_BITS + S_LO_BITS;
    pub const C0_BITS: usize = Self::S2_BITS + S_TOP_BITS;
    pub const C1_BITS: usize = Self::C0_BITS + S_TOP_BITS;
    pub const D0_BITS: usize = Self::C1_BITS + S_TOP_BITS;
    pub const D1_BITS: usize = Self::D0_BITS + S_TOP_BITS;
    // r block after all bits:
    pub const R: usize = STEP_VALS + STEP_BITS;
    pub const R_BITS: usize = Self::R + 1;
    pub const SLACK: usize = Self::R_BITS + MASK_BITS;
    pub const SLACK_BITS: usize = Self::SLACK + 1;
}

// --- C4 carrier mod-q reduction (masked_projection = (acc + mask) mod q12289), air.rs:206-214 ---
/// Bits to range-check a canonical [0,q) value. Mirrors air.rs `MODQ_BITS`.
pub const MODQ_BITS: usize = 14;
/// Cells per lane in the carrier mod-q reduction block:
/// masked_canonical[1] ‖ masked_bits[14] ‖ slack[1] ‖ slack_bits[14] ‖ wrap[1]. Mirrors
/// air.rs `MODQ_CELLS = 1 + MODQ_BITS + 1 + MODQ_BITS + 1`.
pub const MODQ_CELLS: usize = 1 + MODQ_BITS + 1 + MODQ_BITS + 1; // 31

// --- residual counts (the number of octic residuals the mirror emits) ---

/// Bit cells whose boolean residual `b·(b-1)` is emitted per Horner step (every recompose bit:
/// qt_lo/hi, lane_lo/hi, s0/s1/s2, c0/c1/d0/d1, r_t, slack).
pub const STEP_BIT_BOOLS: usize = LO_BITS // qt_lo
    + HI_BITS // qt_hi
    + LO_BITS // lane_lo
    + HI_BITS // lane_hi
    + S_LO_BITS // s0
    + S_LO_BITS // s1
    + S_TOP_BITS // s2
    + S_TOP_BITS // c0
    + S_TOP_BITS // c1
    + S_TOP_BITS // d0
    + S_TOP_BITS // d1
    + MASK_BITS // r_t
    + MASK_BITS; // slack
/// Value-equality residuals per Horner step: the 11 value recomposes (qt_lo..d1) + r_t-recompose +
/// slack-recompose = 13.
pub const STEP_VAL_EQS: usize = STEP_VALS + 2;
/// Algebraic-equality residuals per Horner step: (r_t+slack==q-1) + (digit==lane_lo+lane_hi·2^16) +
/// 6 limb/carry equalities = 8.
pub const STEP_ALG_EQS: usize = 1 + 1 + 6;
/// Total residuals per Horner step.
pub const N_STEP_RESIDUALS: usize = STEP_BIT_BOOLS + STEP_VAL_EQS + STEP_ALG_EQS;
/// Residuals per mask lane: 6 lane-bind equalities + 6 Horner steps.
pub const N_REDUCE_PER_MASK: usize = LANES_PER_MASK + HORNER_STEPS * N_STEP_RESIDUALS;

/// Residuals per carrier lane: masked-recompose(14 bool + 1 eq) + slack-recompose(14 + 1) +
/// (masked+slack==q-1) + wrap-bool + carrier-eq.
pub const N_CARRIER_PER_LANE: usize = MASK_BITS + 1 + MASK_BITS + 1 + 1 + 1 + 1;

#[cfg(test)]
mod tests {
    use super::*;

    /// FIDELITY PIN (shape) — the mirrored reduce-gadget shape constants MUST equal the deployed
    /// leaf `reduce_gadget`'s. A divergence in any limb/carry width or cell count would silently
    /// read the wrong cells; pinned by attempted equality against the leaf's public constants.
    #[test]
    fn reduce_shape_matches_leaf() {
        use descender_account_stark::reduce_gadget as rg;
        assert_eq!(Q12289, rg::Q12289);
        assert_eq!(NUM_OUTPUTS, rg::NUM_OUTPUTS);
        assert_eq!(LANES_PER_MASK, rg::LANES_PER_MASK);
        assert_eq!(NUM_SQUEEZE, rg::NUM_SQUEEZE);
        assert_eq!(REDUCE_CELLS, rg::REDUCE_CELLS);
    }
}
