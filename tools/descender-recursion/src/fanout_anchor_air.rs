//! Power-of-two proving wrapper for the recursive multi-query fan-out.
//!
//! The production inner proof has 120 FRI queries, while a Plonky3 STARK trace
//! has power-of-two height.  This AIR verifies exactly 120 active
//! [`crate::recursive_air::RecursiveAir`] rows, constrains the remaining eight
//! rows of a 128-row trace inactive, hashes the transcript-owned fields of each
//! active row with the canonical query-settlement encoding, and exports the
//! ordered transcript-anchor root as its only public value.
//!
//! Padding rows are not fabricated FRI queries: every recursive constraint is
//! filtered by the boolean activity column.  The active prefix is nevertheless
//! exact (first row active, monotone `1 -> 0`, last active ordinal fixed to
//! `N-1`).  The first padding window carries the shared cross-row cells needed
//! by the final active recursive transition, but none of its local query cells
//! are accepted as another query.

use crate::inlined_perm::{
    self, eval_perm, ConstantsExpr, PermCells, C_INPUTS, C_OUTPUT, PERM_COLS,
};
use crate::leaf_hash::{leaf_hash_pre_states, lh_perms, LEAF_HASH_RATE};
use crate::merkle_air::{eval_node, node_blocks, ABSORB_LEN, N_PERM};
use crate::pcs_query_settlement::{PCS_QUERY_SETTLEMENT_VERSION_V1, TRANSCRIPT_ANCHOR_DOMAIN_V1};
use crate::recursive_air::{self, n_cols as recursive_n_cols, RecursiveAir, RecursiveConfig};
use descender_account_stark::{Val, DIGEST_ELEMS, PROD_NUM_QUERIES, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{prove, verify, Proof};

extern crate alloc;
use alloc::vec::Vec;

const D: usize = crate::fri_fold_air::D;
const INPUT_ROOTS: usize = 3;
pub const TRANSCRIPT_ANCHOR_PREIMAGE_ELEMS_V1: usize =
    4 + 2 * D + 2 * WIDTH + INPUT_ROOTS * DIGEST_ELEMS + 1 + 2 * D + D;
const ANCHOR_PERMS: usize = lh_perms(TRANSCRIPT_ANCHOR_PREIMAGE_ELEMS_V1);

#[derive(Clone, Debug)]
enum AnchorSource {
    Constant(Val),
    Column(usize),
}

#[derive(Clone, Debug)]
struct QueryAnchorLayoutV1 {
    sources: Vec<AnchorSource>,
    /// Equality seams from transcript-observed cells to independently consumed
    /// recursive-verifier cells (trace root, quotient root, final polynomial).
    equalities: Vec<(usize, usize)>,
}

impl QueryAnchorLayoutV1 {
    fn value(&self, row: &[Val], source: &AnchorSource) -> Val {
        match source {
            AnchorSource::Constant(value) => *value,
            AnchorSource::Column(column) => row[*column],
        }
    }

    fn values(&self, row: &[Val]) -> Vec<Val> {
        self.sources
            .iter()
            .map(|source| self.value(row, source))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FanOutAnchorProfileError {
    WrongQueryCount,
    WrongLogHeight,
    MissingFiatShamirTranscript,
    MissingBatchCombinationAlpha,
    MissingExternalOodEndpoint,
    MissingMainCommitmentConsumer,
    MissingQuotientCommitmentConsumer,
    WrongFriRoundCount,
    MissingFriCommitmentConsumer,
    WrongFriLayerDepths,
    WrongFinalPolynomialShape,
    PreAlphaStreamTooShort,
    QuotientStreamTooShort,
    FinalStreamTooShort,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FanOutAnchorInputError {
    Profile(FanOutAnchorProfileError),
    QueryCountMismatch,
    WindowWidthMismatch { query: usize },
}

pub type FanOutAnchorProofV1 = Proof<crate::settlement::SettlementConfig>;
pub type FanOutAnchorVerificationError =
    p3_uni_stark::VerificationError<p3_uni_stark::PcsError<crate::settlement::SettlementConfig>>;

/// Recursive fan-out plus canonical transcript-anchor export.
#[derive(Clone, Debug)]
pub struct FanOutAnchorAirV1 {
    pub cfg: RecursiveConfig,
    active_queries: usize,
    layout: QueryAnchorLayoutV1,
}

fn c_active(cfg: &RecursiveConfig) -> usize {
    recursive_n_cols(cfg)
}

fn c_chain_in(cfg: &RecursiveConfig) -> usize {
    c_active(cfg) + 1
}

fn c_anchor_perm(cfg: &RecursiveConfig) -> usize {
    c_chain_in(cfg) + DIGEST_ELEMS
}

fn anchor_digest_col(cfg: &RecursiveConfig) -> usize {
    c_anchor_perm(cfg) + (ANCHOR_PERMS - 1) * PERM_COLS + C_OUTPUT
}

fn c_chain_perm(cfg: &RecursiveConfig) -> usize {
    c_anchor_perm(cfg) + ANCHOR_PERMS * PERM_COLS
}

fn chain_out_col(cfg: &RecursiveConfig) -> usize {
    c_chain_perm(cfg) + (N_PERM - 1) * PERM_COLS + C_OUTPUT
}

pub fn n_cols(cfg: &RecursiveConfig) -> usize {
    c_chain_perm(cfg) + N_PERM * PERM_COLS
}

impl FanOutAnchorAirV1 {
    /// The exact relation-v3 production profile: 120 active query rows, two
    /// FRI rounds, log-height six, and the transcript/root/final-polynomial
    /// consumers needed by the five-component query settlement statement.
    pub fn production(cfg: RecursiveConfig) -> Result<Self, FanOutAnchorProfileError> {
        let profile = crate::pcs_query_settlement::PcsQuerySettlementProfileV1::production();
        if cfg.fan_out_queries != PROD_NUM_QUERIES {
            return Err(FanOutAnchorProfileError::WrongQueryCount);
        }
        if cfg.log_height != profile.log_height as usize {
            return Err(FanOutAnchorProfileError::WrongLogHeight);
        }
        if !cfg.fs_alpha_zeta {
            return Err(FanOutAnchorProfileError::MissingFiatShamirTranscript);
        }
        if !cfg.fs_batch_combo_alpha {
            return Err(FanOutAnchorProfileError::MissingBatchCombinationAlpha);
        }
        if !cfg.fs_ood_external {
            return Err(FanOutAnchorProfileError::MissingExternalOodEndpoint);
        }
        if !cfg.bind_root_to_main_commit {
            return Err(FanOutAnchorProfileError::MissingMainCommitmentConsumer);
        }
        if cfg.quot_chunks == 0 || cfg.quot_merkle_levels == 0 {
            return Err(FanOutAnchorProfileError::MissingQuotientCommitmentConsumer);
        }
        if cfg.fs_beta_rounds != profile.fri_rounds as usize {
            return Err(FanOutAnchorProfileError::WrongFriRoundCount);
        }
        if cfg.layer_open_rounds != cfg.fs_beta_rounds {
            return Err(FanOutAnchorProfileError::MissingFriCommitmentConsumer);
        }
        if (0..cfg.layer_open_rounds).any(|round| {
            recursive_air::layer_open_levels_for_round(&cfg, round) != cfg.log_height - round - 1
        }) {
            return Err(FanOutAnchorProfileError::WrongFriLayerDepths);
        }
        if cfg.final_poly_len != 1 {
            return Err(FanOutAnchorProfileError::WrongFinalPolynomialShape);
        }
        let pre_len = cfg.fs_pre_alpha_rounds * crate::challenger::RATE + cfg.fs_pre_alpha_partial;
        if pre_len < 3 + DIGEST_ELEMS {
            return Err(FanOutAnchorProfileError::PreAlphaStreamTooShort);
        }
        let quot_len = cfg.fs_quotient_rounds * crate::challenger::RATE + cfg.fs_quotient_partial;
        if quot_len < 2 * DIGEST_ELEMS {
            return Err(FanOutAnchorProfileError::QuotientStreamTooShort);
        }
        let final_len = cfg.fs_final_blocks * crate::challenger::RATE + cfg.fs_final_partial;
        if final_len < D {
            return Err(FanOutAnchorProfileError::FinalStreamTooShort);
        }

        let mut sources = Vec::with_capacity(TRANSCRIPT_ANCHOR_PREIMAGE_ELEMS_V1);
        sources.push(AnchorSource::Constant(Val::from_u32(
            TRANSCRIPT_ANCHOR_DOMAIN_V1,
        )));
        sources.push(AnchorSource::Constant(Val::from_u32(
            PCS_QUERY_SETTLEMENT_VERSION_V1 as u32,
        )));
        sources.push(AnchorSource::Column(
            recursive_air::window_fan_out_ordinal_col(&cfg),
        ));
        sources.push(AnchorSource::Column(recursive_air::window_index_col()));
        sources.extend(
            (0..D).map(|i| AnchorSource::Column(recursive_air::window_bca_alpha_col(&cfg) + i)),
        );
        sources.extend(
            (0..D).map(|i| AnchorSource::Column(recursive_air::window_fs_zeta_col(&cfg) + i)),
        );
        sources.extend(
            (0..WIDTH).map(|i| {
                AnchorSource::Column(recursive_air::window_external_ood_start_col(&cfg) + i)
            }),
        );
        sources.extend(
            (0..WIDTH).map(|i| {
                AnchorSource::Column(recursive_air::window_external_ood_end_col(&cfg) + i)
            }),
        );

        // PCS input order is random, trace, quotient.  The native transcript
        // observes quotient then random between alpha and zeta; the trace root
        // occurs after the three degree/width tags in the pre-alpha stream.
        sources.extend((0..DIGEST_ELEMS).map(|i| {
            AnchorSource::Column(recursive_air::window_quotient_stream_col(
                &cfg,
                DIGEST_ELEMS + i,
            ))
        }));
        sources.extend((0..DIGEST_ELEMS).map(|i| {
            AnchorSource::Column(recursive_air::window_pre_alpha_stream_col(&cfg, 3 + i))
        }));
        sources.extend(
            (0..DIGEST_ELEMS)
                .map(|i| AnchorSource::Column(recursive_air::window_quotient_stream_col(&cfg, i))),
        );
        sources.push(AnchorSource::Constant(Val::from_usize(cfg.fs_beta_rounds)));
        for round in 0..cfg.fs_beta_rounds {
            sources.extend((0..D).map(|i| {
                AnchorSource::Column(recursive_air::window_fri_beta_col(&cfg, round) + i)
            }));
        }
        sources.extend(
            (0..D).map(|i| AnchorSource::Column(recursive_air::window_final_stream_col(&cfg, i))),
        );
        debug_assert_eq!(sources.len(), TRANSCRIPT_ANCHOR_PREIMAGE_ELEMS_V1);

        let mut equalities = Vec::with_capacity(2 * DIGEST_ELEMS + D);
        for i in 0..DIGEST_ELEMS {
            equalities.push((
                recursive_air::window_pre_alpha_stream_col(&cfg, 3 + i),
                recursive_air::window_main_commit_col(&cfg) + i,
            ));
            equalities.push((
                recursive_air::window_quotient_stream_col(&cfg, i),
                recursive_air::window_quotient_commit_col(&cfg) + i,
            ));
        }
        for i in 0..D {
            equalities.push((
                recursive_air::window_final_stream_col(&cfg, i),
                recursive_air::window_final_poly_coeff_col(&cfg, 0) + i,
            ));
        }

        Ok(Self {
            cfg,
            active_queries: PROD_NUM_QUERIES,
            layout: QueryAnchorLayoutV1 {
                sources,
                equalities,
            },
        })
    }

    pub fn active_queries(&self) -> usize {
        self.active_queries
    }

    pub fn trace_height(&self) -> usize {
        self.active_queries.next_power_of_two().max(2)
    }

    /// Read the exact canonical anchor preimage from one recursive window.
    /// This is also the prover-side audit hook used to compare genuine native
    /// proof statements with the cells selected by the AIR.
    pub fn transcript_anchor_preimage_from_window(
        &self,
        window: &[Val],
    ) -> Result<Vec<Val>, FanOutAnchorInputError> {
        if window.len() != recursive_n_cols(&self.cfg) {
            return Err(FanOutAnchorInputError::WindowWidthMismatch { query: 0 });
        }
        Ok(self.layout.values(window))
    }

    #[cfg(test)]
    fn synthetic_repeated_index(cfg: RecursiveConfig, active_queries: usize) -> Self {
        let ordinal = recursive_air::window_fan_out_ordinal_col(&cfg);
        let index = recursive_air::window_index_col();
        let mut sources = Vec::with_capacity(TRANSCRIPT_ANCHOR_PREIMAGE_ELEMS_V1);
        sources.push(AnchorSource::Constant(Val::from_u32(
            TRANSCRIPT_ANCHOR_DOMAIN_V1,
        )));
        sources.push(AnchorSource::Constant(Val::from_u32(
            PCS_QUERY_SETTLEMENT_VERSION_V1 as u32,
        )));
        sources.push(AnchorSource::Column(ordinal));
        sources.push(AnchorSource::Column(index));
        sources.extend(
            (0..(TRANSCRIPT_ANCHOR_PREIMAGE_ELEMS_V1 - 4)).map(|_| AnchorSource::Column(index)),
        );
        // Overwrite the beta-count slot with the canonical production count.
        let beta_count = 4 + 2 * D + 2 * WIDTH + INPUT_ROOTS * DIGEST_ELEMS;
        sources[beta_count] = AnchorSource::Constant(Val::from_usize(2));
        Self {
            cfg,
            active_queries,
            layout: QueryAnchorLayoutV1 {
                sources,
                equalities: Vec::new(),
            },
        }
    }
}

impl<F: Sync> BaseAir<F> for FanOutAnchorAirV1 {
    fn width(&self) -> usize {
        n_cols(&self.cfg)
    }

    fn num_public_values(&self) -> usize {
        DIGEST_ELEMS
    }
}

struct Cells<AB: AirBuilder<F = Val>> {
    row: Vec<AB::Var>,
}

impl<AB: AirBuilder<F = Val>> PermCells<AB> for Cells<AB> {
    fn at(&self, idx: usize) -> AB::Expr {
        self.row[idx].into()
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for FanOutAnchorAirV1 {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let next: Vec<AB::Var> = main.next_slice().to_vec();
        let pubs: Vec<AB::Expr> = builder
            .public_values()
            .iter()
            .map(|value| (*value).into())
            .collect();
        let l = |i: usize| -> AB::Expr { local[i].into() };
        let n = |i: usize| -> AB::Expr { next[i].into() };
        let active = l(c_active(&self.cfg));
        let next_active = n(c_active(&self.cfg));
        builder.assert_zero(active.clone() * (active.clone() - AB::Expr::ONE));
        builder
            .when_first_row()
            .assert_zero(active.clone() - AB::Expr::ONE);
        if self.trace_height() == self.active_queries {
            builder
                .when_last_row()
                .assert_zero(active.clone() - AB::Expr::ONE);
        } else {
            builder.when_last_row().assert_zero(active.clone());
        }
        builder
            .when_transition()
            .assert_zero(next_active.clone() * (AB::Expr::ONE - active.clone()));

        // Verify the active recursive rows.  The last active row's transition
        // still constrains the first padding row's ordinal/shared cells; the
        // honest builder supplies a copied bridge row for exactly that reason.
        {
            let mut gated = builder.when(active.clone());
            RecursiveAir {
                cfg: self.cfg.clone(),
            }
            .eval(&mut gated);
        }

        let ordinal_col = recursive_air::window_fan_out_ordinal_col(&self.cfg);
        let expected_last = AB::Expr::from(Val::from_usize(self.active_queries - 1));
        if self.trace_height() == self.active_queries {
            builder
                .when_last_row()
                .assert_zero(l(ordinal_col) - expected_last);
        } else {
            let falling_edge = active.clone() - next_active.clone();
            builder
                .when_transition()
                .assert_zero(falling_edge * (l(ordinal_col) - expected_last));
        }

        // Bind transcript-observed roots/final-poly to the cells consumed by
        // the recursive Merkle/FRI checks.
        {
            let mut gated = builder.when(active.clone());
            for &(observed, consumed) in &self.layout.equalities {
                gated.assert_zero(l(observed) - l(consumed));
            }
        }

        // Canonical per-query transcript-anchor hash.
        {
            let mut gated = builder.when(active.clone());
            let cells = Cells::<_> { row: local.clone() };
            let constants = ConstantsExpr::<_>::pinned();
            for k in 0..ANCHOR_PERMS {
                let perm = c_anchor_perm(&self.cfg) + k * PERM_COLS;
                for lane in 0..WIDTH {
                    let position = k * LEAF_HASH_RATE + lane;
                    let input = l(perm + C_INPUTS + lane);
                    let expected = if lane < LEAF_HASH_RATE
                        && position < TRANSCRIPT_ANCHOR_PREIMAGE_ELEMS_V1
                    {
                        match &self.layout.sources[position] {
                            AnchorSource::Constant(value) => AB::Expr::from(*value),
                            AnchorSource::Column(column) => l(*column),
                        }
                    } else if k == 0 {
                        AB::Expr::ZERO
                    } else {
                        l(c_anchor_perm(&self.cfg) + (k - 1) * PERM_COLS + C_OUTPUT + lane)
                    };
                    gated.assert_zero(input - expected);
                }
                eval_perm(&mut gated, &cells, perm, &constants);
            }

            let chain = c_chain_in(&self.cfg);
            let digest = anchor_digest_col(&self.cfg);
            let combined = |position: usize| -> AB::Expr {
                if position < DIGEST_ELEMS {
                    l(chain + position)
                } else {
                    l(digest + position - DIGEST_ELEMS)
                }
            };
            eval_node(
                &mut gated,
                &cells,
                c_chain_perm(&self.cfg),
                combined,
                &constants,
            );
        }

        // Genesis and ordered chain.  Inactive rows carry the chain unchanged.
        {
            let mut first = builder.when_first_row();
            for i in 0..DIGEST_ELEMS {
                first.assert_zero(l(c_chain_in(&self.cfg) + i));
            }
        }
        {
            let mut transition = builder.when_transition();
            for i in 0..DIGEST_ELEMS {
                let carried = active.clone() * l(chain_out_col(&self.cfg) + i)
                    + (AB::Expr::ONE - active.clone()) * l(c_chain_in(&self.cfg) + i);
                transition.assert_zero(n(c_chain_in(&self.cfg) + i) - carried);
            }
        }
        if pubs.len() == DIGEST_ELEMS {
            let mut last = builder.when_last_row();
            for i in 0..DIGEST_ELEMS {
                let terminal = active.clone() * l(chain_out_col(&self.cfg) + i)
                    + (AB::Expr::ONE - active.clone()) * l(c_chain_in(&self.cfg) + i);
                last.assert_zero(terminal - pubs[i].clone());
            }
        }
    }
}

/// Build the padded fan-out trace and return its canonical transcript-anchor
/// root.  `query_windows` contains only real queries; padding is generated here.
pub fn build_trace(
    air: &FanOutAnchorAirV1,
    query_windows: &[Vec<Val>],
) -> Result<(Vec<Val>, [Val; DIGEST_ELEMS]), FanOutAnchorInputError> {
    if query_windows.len() != air.active_queries {
        return Err(FanOutAnchorInputError::QueryCountMismatch);
    }
    let recursive_width = recursive_n_cols(&air.cfg);
    for (query, window) in query_windows.iter().enumerate() {
        if window.len() != recursive_width {
            return Err(FanOutAnchorInputError::WindowWidthMismatch { query });
        }
    }

    let width = n_cols(&air.cfg);
    let height = air.trace_height();
    let constants = inlined_perm::Constants::pinned();
    let mut flat = Vec::with_capacity(height * width);
    let mut chain = [Val::ZERO; DIGEST_ELEMS];

    for window in query_windows {
        let mut row = vec![Val::ZERO; width];
        row[..recursive_width].copy_from_slice(window);
        row[c_active(&air.cfg)] = Val::ONE;
        row[c_chain_in(&air.cfg)..c_chain_in(&air.cfg) + DIGEST_ELEMS].copy_from_slice(&chain);

        let preimage = air.layout.values(window);
        debug_assert_eq!(preimage.len(), TRANSCRIPT_ANCHOR_PREIMAGE_ELEMS_V1);
        let pre_states = leaf_hash_pre_states(&preimage);
        debug_assert_eq!(pre_states.len(), ANCHOR_PERMS);
        for (k, state) in pre_states.into_iter().enumerate() {
            let columns = inlined_perm::generate_perm_cols(state, &constants);
            let base = c_anchor_perm(&air.cfg) + k * PERM_COLS;
            row[base..base + PERM_COLS].copy_from_slice(&columns);
        }
        let digest: [Val; DIGEST_ELEMS] =
            core::array::from_fn(|i| row[anchor_digest_col(&air.cfg) + i]);
        let mut combined = [Val::ZERO; ABSORB_LEN];
        combined[..DIGEST_ELEMS].copy_from_slice(&chain);
        combined[DIGEST_ELEMS..].copy_from_slice(&digest);
        let (node, next_chain) = node_blocks(&combined, &constants);
        row[c_chain_perm(&air.cfg)..c_chain_perm(&air.cfg) + N_PERM * PERM_COLS]
            .copy_from_slice(&node);
        flat.extend_from_slice(&row);
        chain = next_chain;
    }

    if height > air.active_queries {
        let mut bridge = query_windows
            .last()
            .expect("active query count is nonzero")
            .clone();
        bridge[recursive_air::window_fan_out_ordinal_col(&air.cfg)] =
            Val::from_usize(air.active_queries);
        for _ in air.active_queries..height {
            let mut row = vec![Val::ZERO; width];
            row[..recursive_width].copy_from_slice(&bridge);
            row[c_chain_in(&air.cfg)..c_chain_in(&air.cfg) + DIGEST_ELEMS].copy_from_slice(&chain);
            flat.extend_from_slice(&row);
        }
    }
    Ok((flat, chain))
}

/// Prove the power-of-two fan-out and return its public transcript-anchor
/// root.  The consensus-selected [`FanOutAnchorAirV1`] is supplied out of band;
/// a proof cannot select a smaller active-query count.
pub fn prove_fan_out_anchor_v1(
    config: &crate::settlement::SettlementConfig,
    air: &FanOutAnchorAirV1,
    query_windows: &[Vec<Val>],
) -> Result<(FanOutAnchorProofV1, [Val; DIGEST_ELEMS]), FanOutAnchorInputError> {
    let (flat, root) = build_trace(air, query_windows)?;
    let trace = RowMajorMatrix::new(flat, n_cols(&air.cfg));
    let proof = prove(config, air, trace, &root);
    Ok((proof, root))
}

pub fn verify_fan_out_anchor_v1(
    config: &crate::settlement::SettlementConfig,
    air: &FanOutAnchorAirV1,
    proof: &FanOutAnchorProofV1,
    claimed_root: &[Val; DIGEST_ELEMS],
) -> Result<(), FanOutAnchorVerificationError> {
    verify(config, air, proof, claimed_root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenger::RATE;
    use crate::pcs_query_settlement::{
        statement_roots_v1, transcript_anchor_preimage_v1, PcsQueryStatementV1,
    };
    use crate::recursive_air::{pack_window_query, WindowInputs};
    use descender_account_stark::{Challenge, SALT_ELEMS};
    use p3_air::check_all_constraints;
    use p3_field::{BasedVectorSpace, PrimeCharacteristicRing, PrimeField32};
    use p3_matrix::dense::RowMajorMatrix;

    fn ext(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| Val::from_u32(seed + i as u32 + 1))
    }

    fn simple_window(cfg: &RecursiveConfig, ordinal: usize) -> Vec<Val> {
        let input = WindowInputs {
            draw_out: Val::from_usize(ordinal + 1),
            e0: ext(10 + ordinal as u32),
            e1: ext(30 + ordinal as u32),
            beta: ext(50),
            descent: &[],
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
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
        pack_window_query(cfg, &input, ordinal)
    }

    fn repeated_statement(index: Val) -> PcsQueryStatementV1 {
        let challenge = Challenge::from_basis_coefficients_fn(|_| index);
        PcsQueryStatementV1 {
            alpha: challenge,
            zeta: challenge,
            x: Val::ZERO,
            ood_start: [index; WIDTH],
            ood_end: [index; WIDTH],
            input_roots: [[index; DIGEST_ELEMS]; INPUT_ROOTS],
            leaf_digests: [[Val::ZERO; DIGEST_ELEMS]; INPUT_ROOTS],
            index: index.as_canonical_u32(),
            reduced_opening: Challenge::ZERO,
            fri_betas: vec![challenge; 2],
            final_poly: challenge,
        }
    }

    #[test]
    fn three_active_queries_pad_to_four_and_export_canonical_root() {
        let cfg = RecursiveConfig {
            fan_out_queries: 3,
            ..RecursiveConfig::single_window(4)
        };
        let air = FanOutAnchorAirV1::synthetic_repeated_index(cfg, 3);
        let windows: Vec<_> = (0..3).map(|r| simple_window(&air.cfg, r)).collect();
        let (flat, root) = build_trace(&air, &windows).expect("honest padded trace");
        assert_eq!(air.trace_height(), 4);
        let matrix = RowMajorMatrix::new(flat.clone(), n_cols(&air.cfg));
        assert!(check_all_constraints(&air, &matrix, &root, None).is_ok());

        let statements: Vec<_> = windows
            .iter()
            .map(|row| repeated_statement(row[recursive_air::window_index_col()]))
            .collect();
        assert_eq!(statement_roots_v1(&statements).0, root);
        for (ordinal, (row, statement)) in windows.iter().zip(&statements).enumerate() {
            assert_eq!(
                air.layout.values(row),
                transcript_anchor_preimage_v1(ordinal, statement)
            );
        }

        let width = n_cols(&air.cfg);
        let mut bad_root = root;
        bad_root[0] += Val::ONE;
        assert!(!check_all_constraints(&air, &matrix, &bad_root, None).is_ok());

        let mut bad_hash = flat.clone();
        bad_hash[c_anchor_perm(&air.cfg) + C_OUTPUT + 2] += Val::ONE;
        assert!(
            !check_all_constraints(&air, &RowMajorMatrix::new(bad_hash, width), &root, None)
                .is_ok()
        );

        let mut bad_active = flat.clone();
        bad_active[3 * width + c_active(&air.cfg)] = Val::ONE;
        assert!(
            !check_all_constraints(&air, &RowMajorMatrix::new(bad_active, width), &root, None)
                .is_ok()
        );

        let mut bad_query = flat.clone();
        bad_query[1 * width + recursive_air::window_index_col()] += Val::ONE;
        assert!(
            !check_all_constraints(&air, &RowMajorMatrix::new(bad_query, width), &root, None)
                .is_ok()
        );

        // Inactive local query cells are not silently treated as extra FRI
        // queries.  Changing a padding-only cell leaves the proof relation true.
        let mut changed_padding = flat;
        changed_padding[3 * width + recursive_air::window_index_col()] += Val::ONE;
        assert!(check_all_constraints(
            &air,
            &RowMajorMatrix::new(changed_padding, width),
            &root,
            None
        )
        .is_ok());
    }

    #[test]
    fn production_profile_pins_120_active_queries_in_128_rows() {
        let cfg = RecursiveConfig {
            log_height: 6,
            merkle_levels: 6,
            fs_alpha_zeta: true,
            fs_pre_alpha_rounds: 3,
            fs_pre_alpha_partial: 2,
            fs_quotient_rounds: 3,
            fs_quotient_partial: 5,
            fs_batch_combo_alpha: true,
            fs_beta_rounds: 2,
            fs_final_blocks: 1,
            fs_final_partial: 4,
            fs_ood_external: true,
            bind_root_to_main_commit: true,
            layer_open_rounds: 2,
            layer_open_levels: 5,
            layer_open_level_step: 1,
            final_poly_len: 1,
            quot_chunks: 8,
            quot_merkle_levels: 6,
            fan_out_queries: PROD_NUM_QUERIES,
            ..RecursiveConfig::single_window(6)
        };
        let air = FanOutAnchorAirV1::production(cfg).expect("production anchor profile");
        assert_eq!(air.active_queries(), 120);
        assert_eq!(air.trace_height(), 128);
        assert_eq!(
            crate::recursive_air::layer_open_levels_for_round(&air.cfg, 0),
            5
        );
        assert_eq!(
            crate::recursive_air::layer_open_levels_for_round(&air.cfg, 1),
            4
        );
        assert_eq!(TRANSCRIPT_ANCHOR_PREIMAGE_ELEMS_V1, 116);
        assert_eq!(ANCHOR_PERMS, 17);
        assert_eq!(air.layout.sources.len(), 116);
        assert_eq!(air.layout.equalities.len(), 2 * DIGEST_ELEMS + D);

        let mut wrong_depths = air.cfg;
        wrong_depths.layer_open_level_step = 0;
        assert!(matches!(
            FanOutAnchorAirV1::production(wrong_depths),
            Err(FanOutAnchorProfileError::WrongFriLayerDepths)
        ));
    }

    #[test]
    fn padded_anchor_crosses_q128_proof_boundary() {
        let cfg = RecursiveConfig {
            fan_out_queries: 3,
            ..RecursiveConfig::single_window(4)
        };
        let air = FanOutAnchorAirV1::synthetic_repeated_index(cfg, 3);
        let windows: Vec<_> = (0..3).map(|r| simple_window(&air.cfg, r)).collect();
        let config = crate::settlement::prod_config();
        let (proof, root) = prove_fan_out_anchor_v1(&config, &air, &windows).unwrap();
        verify_fan_out_anchor_v1(&config, &air, &proof, &root)
            .expect("q128 fan-out anchor proof verifies");
        let mut wrong = root;
        wrong[0] += Val::ONE;
        assert!(verify_fan_out_anchor_v1(&config, &air, &proof, &wrong).is_err());

        // Compose the anchor proof with the existing q128 block proof under one
        // domain-separated settlement root.  Coherent root substitutions still
        // fail at the corresponding proof boundary.
        let block_cfg = crate::block_recursive_air::cfg_merkle_settlement(2);
        let block_windows = crate::block_recursive_air::windows_for_settlement(&block_cfg, 2, 2);
        let (block_proof, chain_root) =
            crate::settlement::prove_block(&config, &block_cfg, &block_windows);
        let settlement_root =
            crate::settlement::transcript_anchored_block_root_v1(&chain_root, &root);
        crate::settlement::verify_block_with_transcript_anchor_v1(
            &config,
            &block_cfg,
            &block_proof,
            &chain_root,
            &air,
            &proof,
            &root,
            &settlement_root,
        )
        .expect("q128 block and fan-out anchor compose under one settlement root");

        let mut wrong_settlement = settlement_root;
        wrong_settlement[0] += Val::ONE;
        assert!(crate::settlement::verify_block_with_transcript_anchor_v1(
            &config,
            &block_cfg,
            &block_proof,
            &chain_root,
            &air,
            &proof,
            &root,
            &wrong_settlement,
        )
        .is_err());

        let wrong_anchor_settlement =
            crate::settlement::transcript_anchored_block_root_v1(&chain_root, &wrong);
        assert!(crate::settlement::verify_block_with_transcript_anchor_v1(
            &config,
            &block_cfg,
            &block_proof,
            &chain_root,
            &air,
            &proof,
            &wrong,
            &wrong_anchor_settlement,
        )
        .is_err());

        let mut wrong_chain = chain_root;
        wrong_chain[0] += Val::ONE;
        let wrong_chain_settlement =
            crate::settlement::transcript_anchored_block_root_v1(&wrong_chain, &root);
        assert!(crate::settlement::verify_block_with_transcript_anchor_v1(
            &config,
            &block_cfg,
            &block_proof,
            &wrong_chain,
            &air,
            &proof,
            &root,
            &wrong_chain_settlement,
        )
        .is_err());
    }
}
