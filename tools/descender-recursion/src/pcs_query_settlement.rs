//! Versioned settlement envelope for production PCS queries.
//!
//! A `PcsQueryAir` proof alone exposes leaf digests and a reduced FRI opening,
//! but it does not authenticate those leaves to the three input commitments or
//! prove that the reduced value reaches the transcript-selected final
//! polynomial.  This module is the consensus-facing composition boundary: one
//! query statement is consumed by one query-link proof, three vertical MMCS
//! proofs, and one beta-bound FRI-chain proof.  The all-query envelope fixes the
//! query count and requires every query to share the same transcript and
//! commitment anchors. Its checked materializer evaluates all five generated
//! component AIR traces one query at a time without claiming that host-side
//! constraint evaluation is a recursive proof.
//!
//! Verification deliberately requires caller-supplied expected statements.
//! This prevents a self-contained but transcript-free envelope from choosing
//! favorable query indices.  The remaining production step is to export those
//! exact statements from the recursive transcript/fan-out proof and bind their
//! digest into the block settlement root.

use crate::fri_chain_air::{self, FriChainAir};
use crate::fri_fold_air::{limbs, D};
use crate::inlined_perm;
use crate::leaf_hash::leaf_hash_digest;
use crate::merkle_air::{node_blocks, ABSORB_LEN};
use crate::pcs_query_air::{self, PcsBatchShape, PcsQueryAir, PcsQueryInput};
use crate::settlement::{SettlementConfig, SettlementVerificationError};
use crate::vertical_mmcs_air::{self, VerticalMmcsOpeningAir};
use descender_account_stark::{
    Challenge, Val, DIGEST_ELEMS, PROD_LOG_BLOWUP, PROD_NUM_QUERIES, SALT_ELEMS,
    SINGLE_PROOF_TRACE_ROWS, WIDTH,
};
use p3_air::check_all_constraints;
use p3_field::{BasedVectorSpace, PrimeCharacteristicRing, TwoAdicField};
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{prove, verify, Proof};
use serde::{Deserialize, Serialize};

extern crate alloc;
use alloc::vec::Vec;

pub const PCS_QUERY_SETTLEMENT_VERSION_V1: u16 = 1;
pub const PCS_QUERY_INPUT_BATCHES_V1: usize = 3;
pub const PCS_QUERY_COMPONENT_PROOFS_V1: usize = 5;
pub const PRODUCTION_PCS_LOG_HEIGHT_V1: usize =
    SINGLE_PROOF_TRACE_ROWS.ilog2() as usize + 1 + PROD_LOG_BLOWUP;
pub const PRODUCTION_FRI_ROUNDS_V1: usize = SINGLE_PROOF_TRACE_ROWS.ilog2() as usize + 1;

pub type PcsQueryComponentProof = Proof<SettlementConfig>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PcsQuerySettlementProfileV1 {
    pub version: u16,
    pub num_queries: u32,
    pub log_height: u8,
    pub fri_rounds: u8,
    pub matrix_widths: Vec<Vec<u32>>,
}

impl PcsQuerySettlementProfileV1 {
    pub fn new(
        num_queries: usize,
        log_height: usize,
        fri_rounds: usize,
        matrix_widths: Vec<Vec<usize>>,
    ) -> Result<Self, PcsQuerySettlementInputError> {
        let profile = Self {
            version: PCS_QUERY_SETTLEMENT_VERSION_V1,
            num_queries: u32::try_from(num_queries)
                .map_err(|_| PcsQuerySettlementInputError::ShapeOverflow)?,
            log_height: u8::try_from(log_height)
                .map_err(|_| PcsQuerySettlementInputError::ShapeOverflow)?,
            fri_rounds: u8::try_from(fri_rounds)
                .map_err(|_| PcsQuerySettlementInputError::ShapeOverflow)?,
            matrix_widths: matrix_widths
                .into_iter()
                .map(|batch| {
                    batch
                        .into_iter()
                        .map(|width| {
                            u32::try_from(width)
                                .map_err(|_| PcsQuerySettlementInputError::ShapeOverflow)
                        })
                        .collect()
                })
                .collect::<Result<_, _>>()?,
        };
        profile.validate()?;
        Ok(profile)
    }

    pub fn production() -> Self {
        use crate::pcs_opening_shape::{
            QUOTIENT_MATRIX_WIDTH_V1, RANDOM_MATRIX_WIDTH_V1, TRACE_MATRIX_WIDTH_V1,
        };
        Self::new(
            PROD_NUM_QUERIES,
            PRODUCTION_PCS_LOG_HEIGHT_V1,
            PRODUCTION_FRI_ROUNDS_V1,
            vec![
                vec![RANDOM_MATRIX_WIDTH_V1],
                vec![TRACE_MATRIX_WIDTH_V1],
                vec![
                    QUOTIENT_MATRIX_WIDTH_V1;
                    crate::pcs_opening_shape::PRODUCTION_QUOTIENT_MATRICES_V1
                ],
            ],
        )
        .expect("source-pinned production PCS settlement profile")
    }

    pub fn validate(&self) -> Result<(), PcsQuerySettlementInputError> {
        if self.version != PCS_QUERY_SETTLEMENT_VERSION_V1 {
            return Err(PcsQuerySettlementInputError::UnsupportedVersion(
                self.version,
            ));
        }
        if self.num_queries == 0 {
            return Err(PcsQuerySettlementInputError::NoQueries);
        }
        let log_height = self.log_height as usize;
        if log_height == 0 || log_height > Val::TWO_ADICITY {
            return Err(PcsQuerySettlementInputError::InvalidLogHeight);
        }
        let fri_rounds = self.fri_rounds as usize;
        if fri_rounds == 0 || !fri_rounds.is_power_of_two() || fri_rounds > log_height {
            return Err(PcsQuerySettlementInputError::InvalidFriRounds);
        }
        if self.matrix_widths.len() != PCS_QUERY_INPUT_BATCHES_V1
            || self
                .matrix_widths
                .iter()
                .any(|batch| batch.is_empty() || batch.iter().any(|&width| width == 0))
        {
            return Err(PcsQuerySettlementInputError::InvalidBatchShape);
        }
        Ok(())
    }

    pub fn air(&self) -> PcsQueryAir {
        PcsQueryAir::new(
            self.matrix_widths
                .iter()
                .map(|batch| {
                    PcsBatchShape::new(batch.iter().map(|&width| width as usize).collect())
                })
                .collect(),
            self.log_height as usize,
        )
    }

    pub fn salted_input_len(&self, batch: usize) -> usize {
        self.matrix_widths[batch]
            .iter()
            .map(|&width| width as usize + SALT_ELEMS)
            .sum()
    }

    pub fn base_witness_bytes_per_query(&self) -> usize {
        let log_height = self.log_height as usize;
        let query = self.air().base_witness_bytes();
        let mmcs: usize = (0..PCS_QUERY_INPUT_BATCHES_V1)
            .map(|batch| {
                VerticalMmcsOpeningAir::new(self.salted_input_len(batch), log_height)
                    .base_witness_bytes()
            })
            .sum();
        let fri = self.fri_rounds as usize * fri_chain_air::N_COLS * core::mem::size_of::<Val>();
        query + mmcs + fri
    }

    pub fn base_witness_bytes_all_queries(&self) -> usize {
        self.base_witness_bytes_per_query() * self.num_queries as usize
    }

    pub fn component_proof_count(&self) -> usize {
        PCS_QUERY_COMPONENT_PROOFS_V1 * self.num_queries as usize
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PcsQueryStatementV1 {
    pub alpha: Challenge,
    pub zeta: Challenge,
    pub x: Val,
    pub ood_start: [Val; WIDTH],
    pub ood_end: [Val; WIDTH],
    pub input_roots: [[Val; DIGEST_ELEMS]; PCS_QUERY_INPUT_BATCHES_V1],
    pub leaf_digests: [[Val; DIGEST_ELEMS]; PCS_QUERY_INPUT_BATCHES_V1],
    pub index: u32,
    pub reduced_opening: Challenge,
    pub fri_betas: Vec<Challenge>,
    pub final_poly: Challenge,
}

#[derive(Serialize, Deserialize)]
pub struct PcsQuerySettlementProofV1 {
    pub statement: PcsQueryStatementV1,
    pub query_link_proof: PcsQueryComponentProof,
    pub input_mmcs_proofs: Vec<PcsQueryComponentProof>,
    pub fri_chain_proof: PcsQueryComponentProof,
}

#[derive(Serialize, Deserialize)]
pub struct AllPcsQueriesSettlementProofV1 {
    pub profile: PcsQuerySettlementProfileV1,
    /// Canonical root over the fields the recursive Fiat--Shamir/fan-out proof
    /// must export: challenges, commitments, endpoints, final polynomial, and
    /// each ordered query index.
    pub transcript_anchor_root: [Val; DIGEST_ELEMS],
    /// Canonical root over each complete typed query statement, including the
    /// external leaf digests and reduced opening.
    pub statement_root: [Val; DIGEST_ELEMS],
    pub queries: Vec<PcsQuerySettlementProofV1>,
}

/// Compact, proof-independent materialization of the exact ordered statements
/// consumed by a production all-query settlement.  This is the input contract
/// for a future economical aggregate proof: it fixes all 120 genuine query
/// statements and both canonical roots without serializing 600 component
/// proofs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllPcsQueryStatementsV1 {
    pub profile: PcsQuerySettlementProfileV1,
    pub transcript_anchor_root: [Val; DIGEST_ELEMS],
    pub statement_root: [Val; DIGEST_ELEMS],
    pub statements: Vec<PcsQueryStatementV1>,
}

pub struct PcsQuerySettlementWitnessV1 {
    pub query_input: PcsQueryInput,
    pub input_siblings: [Vec<[Val; DIGEST_ELEMS]>; PCS_QUERY_INPUT_BATCHES_V1],
    pub fri_betas: Vec<Challenge>,
    pub fri_siblings: Vec<Challenge>,
    pub final_poly: Challenge,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PcsQuerySettlementInputError {
    UnsupportedVersion(u16),
    NoQueries,
    ShapeOverflow,
    InvalidLogHeight,
    InvalidFriRounds,
    InvalidBatchShape,
    QueryCountMismatch,
    QueryIndexOutOfRange,
    BatchCountMismatch,
    MatrixCountMismatch { batch: usize },
    MatrixWidthMismatch { batch: usize, matrix: usize },
    SaltCountMismatch { batch: usize },
    MmcsLevelMismatch { batch: usize },
    FriRoundCountMismatch,
    FriTerminalMismatch,
    SharedStatementMismatch { query: usize },
}

#[derive(Debug)]
pub enum PcsQuerySettlementVerificationError {
    InvalidProfile(PcsQuerySettlementInputError),
    ProfileMismatch,
    QueryCountMismatch,
    ExpectedStatementCountMismatch,
    TranscriptAnchorRootMismatch,
    ExpectedTranscriptAnchorRootMismatch,
    StatementRootMismatch,
    InvalidStatement {
        query: usize,
        source: PcsQuerySettlementInputError,
    },
    StatementMismatch {
        query: usize,
    },
    SharedStatementMismatch {
        query: usize,
    },
    ComponentCountMismatch {
        query: usize,
    },
    QueryLink {
        query: usize,
        source: SettlementVerificationError,
    },
    InputMmcs {
        query: usize,
        batch: usize,
        source: SettlementVerificationError,
    },
    FriChain {
        query: usize,
        source: SettlementVerificationError,
    },
}

/// The component trace that failed host-side AIR constraint evaluation while
/// preparing the compact all-query contract.  This is diagnostic evidence, not
/// a replacement for a recursive proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcsQueryConstraintComponentV1 {
    QueryLink,
    InputMmcs { batch: usize },
    FriChain,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PcsQueryConstraintCheckErrorV1 {
    Input {
        query: usize,
        source: PcsQuerySettlementInputError,
    },
    ConstraintViolation {
        query: usize,
        component: PcsQueryConstraintComponentV1,
        failures: usize,
    },
}

fn query_public_values(
    profile: &PcsQuerySettlementProfileV1,
    statement: &PcsQueryStatementV1,
) -> Result<Vec<Val>, PcsQuerySettlementInputError> {
    let log_height = profile.log_height as usize;
    let index = statement.index as usize;
    if index >= 1usize << log_height {
        return Err(PcsQuerySettlementInputError::QueryIndexOutOfRange);
    }
    let mut out = Vec::with_capacity(profile.air().public_width());
    out.extend_from_slice(&limbs(statement.alpha));
    out.extend_from_slice(&limbs(statement.zeta));
    out.push(statement.x);
    out.extend_from_slice(&statement.ood_start);
    out.extend_from_slice(&statement.ood_end);
    for digest in &statement.leaf_digests {
        out.extend_from_slice(digest);
    }
    out.extend((0..log_height).map(|k| Val::from_bool((index >> k) & 1 == 1)));
    out.extend_from_slice(&limbs(statement.reduced_opening));
    Ok(out)
}

fn validate_statement(
    profile: &PcsQuerySettlementProfileV1,
    statement: &PcsQueryStatementV1,
) -> Result<(), PcsQuerySettlementInputError> {
    if statement.fri_betas.len() != profile.fri_rounds as usize {
        return Err(PcsQuerySettlementInputError::FriRoundCountMismatch);
    }
    let _ = query_public_values(profile, statement)?;
    Ok(())
}

fn flatten_salted_batch(
    profile: &PcsQuerySettlementProfileV1,
    input: &PcsQueryInput,
    batch: usize,
) -> Result<Vec<Val>, PcsQuerySettlementInputError> {
    if input.batches.len() != PCS_QUERY_INPUT_BATCHES_V1 {
        return Err(PcsQuerySettlementInputError::BatchCountMismatch);
    }
    let got = &input.batches[batch];
    let expected = &profile.matrix_widths[batch];
    if got.opened_values.len() != expected.len() {
        return Err(PcsQuerySettlementInputError::MatrixCountMismatch { batch });
    }
    if got.salts.len() != expected.len() {
        return Err(PcsQuerySettlementInputError::SaltCountMismatch { batch });
    }
    let mut out = Vec::with_capacity(profile.salted_input_len(batch));
    for (matrix, ((opened, salt), &width)) in got
        .opened_values
        .iter()
        .zip(&got.salts)
        .zip(expected)
        .enumerate()
    {
        if opened.len() != width as usize {
            return Err(PcsQuerySettlementInputError::MatrixWidthMismatch { batch, matrix });
        }
        out.extend_from_slice(opened);
        out.extend_from_slice(salt);
    }
    Ok(out)
}

fn shared_statement_eq(a: &PcsQueryStatementV1, b: &PcsQueryStatementV1) -> bool {
    a.alpha == b.alpha
        && a.zeta == b.zeta
        && a.ood_start == b.ood_start
        && a.ood_end == b.ood_end
        && a.input_roots == b.input_roots
        && a.fri_betas == b.fri_betas
        && a.final_poly == b.final_poly
}

pub const TRANSCRIPT_ANCHOR_DOMAIN_V1: u32 = 0x4453_5101;
const COMPLETE_STATEMENT_DOMAIN_V1: u32 = 0x4453_5102;

fn push_challenge(out: &mut Vec<Val>, value: Challenge) {
    out.extend_from_slice(&limbs(value));
}

/// Canonical per-query digest of every value that must be exported by the
/// recursive transcript/fan-out proof. Query ordinal is included independently
/// of the sampled domain index, so reordering equal or duplicate indices changes
/// the block-wide root.
pub fn transcript_anchor_digest_v1(
    query_ordinal: usize,
    statement: &PcsQueryStatementV1,
) -> [Val; DIGEST_ELEMS] {
    leaf_hash_digest(&transcript_anchor_preimage_v1(query_ordinal, statement))
}

/// Canonical field preimage hashed by [`transcript_anchor_digest_v1`].  The
/// recursive fan-out AIR consumes this exact order from its transcript-owned
/// cells, so the native settlement envelope and the in-AIR export cannot drift
/// into two merely similar encodings.
pub fn transcript_anchor_preimage_v1(
    query_ordinal: usize,
    statement: &PcsQueryStatementV1,
) -> Vec<Val> {
    let mut values = Vec::new();
    values.push(Val::from_u32(TRANSCRIPT_ANCHOR_DOMAIN_V1));
    values.push(Val::from_u32(PCS_QUERY_SETTLEMENT_VERSION_V1 as u32));
    values.push(Val::from_usize(query_ordinal));
    values.push(Val::from_u32(statement.index));
    push_challenge(&mut values, statement.alpha);
    push_challenge(&mut values, statement.zeta);
    values.extend_from_slice(&statement.ood_start);
    values.extend_from_slice(&statement.ood_end);
    for root in &statement.input_roots {
        values.extend_from_slice(root);
    }
    values.push(Val::from_usize(statement.fri_betas.len()));
    for beta in &statement.fri_betas {
        push_challenge(&mut values, *beta);
    }
    push_challenge(&mut values, statement.final_poly);
    values
}

/// Canonical per-query digest of the full statement consumed by all five
/// component proofs.
pub fn complete_statement_digest_v1(
    query_ordinal: usize,
    statement: &PcsQueryStatementV1,
) -> [Val; DIGEST_ELEMS] {
    let mut values = Vec::new();
    values.push(Val::from_u32(COMPLETE_STATEMENT_DOMAIN_V1));
    values.push(Val::from_u32(PCS_QUERY_SETTLEMENT_VERSION_V1 as u32));
    values.push(Val::from_usize(query_ordinal));
    values.push(Val::from_u32(statement.index));
    values.push(statement.x);
    push_challenge(&mut values, statement.alpha);
    push_challenge(&mut values, statement.zeta);
    values.extend_from_slice(&statement.ood_start);
    values.extend_from_slice(&statement.ood_end);
    for root in &statement.input_roots {
        values.extend_from_slice(root);
    }
    for digest in &statement.leaf_digests {
        values.extend_from_slice(digest);
    }
    push_challenge(&mut values, statement.reduced_opening);
    values.push(Val::from_usize(statement.fri_betas.len()));
    for beta in &statement.fri_betas {
        push_challenge(&mut values, *beta);
    }
    push_challenge(&mut values, statement.final_poly);
    leaf_hash_digest(&values)
}

fn fold_digest_chain(
    digests: impl IntoIterator<Item = [Val; DIGEST_ELEMS]>,
) -> [Val; DIGEST_ELEMS] {
    let constants = inlined_perm::Constants::pinned();
    let mut chain = [Val::ZERO; DIGEST_ELEMS];
    for digest in digests {
        let mut combined = [Val::ZERO; ABSORB_LEN];
        combined[..DIGEST_ELEMS].copy_from_slice(&chain);
        combined[DIGEST_ELEMS..].copy_from_slice(&digest);
        chain = node_blocks(&combined, &constants).1;
    }
    chain
}

pub fn statement_roots_v1(
    statements: &[PcsQueryStatementV1],
) -> ([Val; DIGEST_ELEMS], [Val; DIGEST_ELEMS]) {
    let transcript_anchor_root = fold_digest_chain(
        statements
            .iter()
            .enumerate()
            .map(|(ordinal, statement)| transcript_anchor_digest_v1(ordinal, statement)),
    );
    let statement_root = fold_digest_chain(
        statements
            .iter()
            .enumerate()
            .map(|(ordinal, statement)| complete_statement_digest_v1(ordinal, statement)),
    );
    (transcript_anchor_root, statement_root)
}

fn validate_shared_statements(
    statements: &[PcsQueryStatementV1],
) -> Result<(), PcsQuerySettlementInputError> {
    if let Some(first) = statements.first() {
        for (query, statement) in statements.iter().enumerate().skip(1) {
            if !shared_statement_eq(first, statement) {
                return Err(PcsQuerySettlementInputError::SharedStatementMismatch { query });
            }
        }
    }
    Ok(())
}

struct MaterializedPcsQueryV1 {
    statement: PcsQueryStatementV1,
    query_trace: RowMajorMatrix<Val>,
    mmcs_traces: Vec<RowMajorMatrix<Val>>,
    fri_flat: Vec<Val>,
}

fn materialize_pcs_query_v1(
    profile: &PcsQuerySettlementProfileV1,
    witness: PcsQuerySettlementWitnessV1,
) -> Result<MaterializedPcsQueryV1, PcsQuerySettlementInputError> {
    profile.validate()?;
    let log_height = profile.log_height as usize;
    if witness.query_input.index >= 1usize << log_height {
        return Err(PcsQuerySettlementInputError::QueryIndexOutOfRange);
    }
    if witness.fri_betas.len() != profile.fri_rounds as usize
        || witness.fri_siblings.len() != witness.fri_betas.len()
    {
        return Err(PcsQuerySettlementInputError::FriRoundCountMismatch);
    }

    let air = profile.air();
    for batch in 0..PCS_QUERY_INPUT_BATCHES_V1 {
        let _ = flatten_salted_batch(profile, &witness.query_input, batch)?;
        if witness.input_siblings[batch].len() != log_height {
            return Err(PcsQuerySettlementInputError::MmcsLevelMismatch { batch });
        }
    }
    let (query_trace, output) = pcs_query_air::build_trace(&air, &witness.query_input);

    let mut input_roots = [[Val::ZERO; DIGEST_ELEMS]; PCS_QUERY_INPUT_BATCHES_V1];
    let mut mmcs_traces = Vec::with_capacity(PCS_QUERY_INPUT_BATCHES_V1);
    for batch in 0..PCS_QUERY_INPUT_BATCHES_V1 {
        let salted = flatten_salted_batch(profile, &witness.query_input, batch)?;
        let (trace, root) = vertical_mmcs_air::build_trace(
            &salted,
            witness.query_input.index,
            &witness.input_siblings[batch],
        );
        input_roots[batch] = root;
        mmcs_traces.push(trace);
    }
    let leaf_digests: [[Val; DIGEST_ELEMS]; PCS_QUERY_INPUT_BATCHES_V1] = output
        .leaf_digests
        .clone()
        .try_into()
        .map_err(|_| PcsQuerySettlementInputError::BatchCountMismatch)?;

    let fri_flat = fri_chain_air::build_query_chain_from_opening(
        output.reduced_opening,
        &witness.fri_siblings,
        &witness.fri_betas,
        log_height,
        witness.query_input.index,
    );
    let terminal_offset =
        (witness.fri_betas.len() - 1) * fri_chain_air::N_COLS + fri_chain_air::C_FOLDED;
    let terminal =
        Challenge::from_basis_coefficients_slice(&fri_flat[terminal_offset..terminal_offset + D])
            .expect("FRI terminal limbs");
    if terminal != witness.final_poly {
        return Err(PcsQuerySettlementInputError::FriTerminalMismatch);
    }

    Ok(MaterializedPcsQueryV1 {
        statement: PcsQueryStatementV1 {
            alpha: output.alpha,
            zeta: witness.query_input.z,
            x: output.x,
            ood_start: witness.query_input.ood_start,
            ood_end: output.ood_end,
            input_roots,
            leaf_digests,
            index: witness.query_input.index as u32,
            reduced_opening: output.reduced_opening,
            fri_betas: witness.fri_betas,
            final_poly: witness.final_poly,
        },
        query_trace,
        mmcs_traces,
        fri_flat,
    })
}

fn check_materialized_pcs_query_constraints_v1(
    profile: &PcsQuerySettlementProfileV1,
    query: usize,
    materialized: &MaterializedPcsQueryV1,
) -> Result<(), PcsQueryConstraintCheckErrorV1> {
    let violation = |component, failures| {
        Err(PcsQueryConstraintCheckErrorV1::ConstraintViolation {
            query,
            component,
            failures,
        })
    };

    let query_public = query_public_values(profile, &materialized.statement)
        .map_err(|source| PcsQueryConstraintCheckErrorV1::Input { query, source })?;
    let query_report = check_all_constraints(
        &profile.air(),
        &materialized.query_trace,
        &query_public,
        None,
    );
    if !query_report.is_ok() {
        return violation(
            PcsQueryConstraintComponentV1::QueryLink,
            query_report.failures.len(),
        );
    }

    let log_height = profile.log_height as usize;
    for (batch, trace) in materialized.mmcs_traces.iter().enumerate() {
        let mmcs_air = VerticalMmcsOpeningAir::new(profile.salted_input_len(batch), log_height);
        let public = vertical_mmcs_air::public_values(
            materialized.statement.input_roots[batch],
            materialized.statement.index as usize,
            log_height,
            materialized.statement.leaf_digests[batch],
        );
        let report = check_all_constraints(&mmcs_air, trace, &public, None);
        if !report.is_ok() {
            return violation(
                PcsQueryConstraintComponentV1::InputMmcs { batch },
                report.failures.len(),
            );
        }
    }

    let fri_air = FriChainAir::new(profile.fri_rounds as usize);
    let fri_public = fri_chain_air::public_values(
        materialized.statement.reduced_opening,
        materialized.statement.index & 1 == 1,
        materialized.statement.final_poly,
        &materialized.statement.fri_betas,
    );
    let fri_trace = RowMajorMatrix::new(materialized.fri_flat.clone(), fri_chain_air::N_COLS);
    let fri_report = check_all_constraints(&fri_air, &fri_trace, &fri_public, None);
    if !fri_report.is_ok() {
        return violation(
            PcsQueryConstraintComponentV1::FriChain,
            fri_report.failures.len(),
        );
    }

    Ok(())
}

/// Materialize and validate the exact statement for one query without proving
/// its five component AIRs.
pub fn materialize_pcs_query_statement_v1(
    profile: &PcsQuerySettlementProfileV1,
    witness: PcsQuerySettlementWitnessV1,
) -> Result<PcsQueryStatementV1, PcsQuerySettlementInputError> {
    Ok(materialize_pcs_query_v1(profile, witness)?.statement)
}

/// Materialize the fixed-size ordered production statement set.  Count,
/// shared transcript fields, MMCS roots, FRI terminal values, and both roots
/// are recomputed from witnesses; the caller cannot supply them directly.
pub fn materialize_all_pcs_query_statements_v1(
    profile: PcsQuerySettlementProfileV1,
    witnesses: Vec<PcsQuerySettlementWitnessV1>,
) -> Result<AllPcsQueryStatementsV1, PcsQuerySettlementInputError> {
    profile.validate()?;
    if witnesses.len() != profile.num_queries as usize {
        return Err(PcsQuerySettlementInputError::QueryCountMismatch);
    }
    let statements = witnesses
        .into_iter()
        .map(|witness| materialize_pcs_query_statement_v1(&profile, witness))
        .collect::<Result<Vec<_>, _>>()?;
    validate_shared_statements(&statements)?;
    let (transcript_anchor_root, statement_root) = statement_roots_v1(&statements);
    Ok(AllPcsQueryStatementsV1 {
        profile,
        transcript_anchor_root,
        statement_root,
        statements,
    })
}

/// Materialize every query statement and evaluate all five component AIRs
/// against their generated traces before returning the compact contract.  The
/// traces are checked and dropped one query at a time, so the production path
/// does not retain the 5.88 GB all-query base witness in memory.
///
/// This is stronger host-side construction evidence than unchecked statement
/// materialization, but it is deliberately not called a proof: an economical
/// recursive all-query proof remains a separate deployment obligation.
pub fn materialize_all_pcs_query_statements_constraint_checked_v1(
    profile: PcsQuerySettlementProfileV1,
    witnesses: Vec<PcsQuerySettlementWitnessV1>,
) -> Result<AllPcsQueryStatementsV1, PcsQueryConstraintCheckErrorV1> {
    profile
        .validate()
        .map_err(|source| PcsQueryConstraintCheckErrorV1::Input { query: 0, source })?;
    if witnesses.len() != profile.num_queries as usize {
        return Err(PcsQueryConstraintCheckErrorV1::Input {
            query: witnesses.len().min(profile.num_queries as usize),
            source: PcsQuerySettlementInputError::QueryCountMismatch,
        });
    }

    let mut statements = Vec::with_capacity(witnesses.len());
    for (query, witness) in witnesses.into_iter().enumerate() {
        let materialized = materialize_pcs_query_v1(&profile, witness)
            .map_err(|source| PcsQueryConstraintCheckErrorV1::Input { query, source })?;
        check_materialized_pcs_query_constraints_v1(&profile, query, &materialized)?;
        statements.push(materialized.statement);
    }
    validate_shared_statements(&statements).map_err(|source| {
        let query = match source {
            PcsQuerySettlementInputError::SharedStatementMismatch { query } => query,
            _ => 0,
        };
        PcsQueryConstraintCheckErrorV1::Input { query, source }
    })?;
    let (transcript_anchor_root, statement_root) = statement_roots_v1(&statements);
    Ok(AllPcsQueryStatementsV1 {
        profile,
        transcript_anchor_root,
        statement_root,
        statements,
    })
}

/// Verify a transported compact all-query statement contract before an
/// aggregate prover or settlement verifier consumes it.
pub fn verify_all_pcs_query_statements_v1(
    expected_profile: &PcsQuerySettlementProfileV1,
    materialized: &AllPcsQueryStatementsV1,
) -> Result<(), PcsQuerySettlementVerificationError> {
    expected_profile
        .validate()
        .map_err(PcsQuerySettlementVerificationError::InvalidProfile)?;
    materialized
        .profile
        .validate()
        .map_err(PcsQuerySettlementVerificationError::InvalidProfile)?;
    if &materialized.profile != expected_profile {
        return Err(PcsQuerySettlementVerificationError::ProfileMismatch);
    }
    if materialized.statements.len() != expected_profile.num_queries as usize {
        return Err(PcsQuerySettlementVerificationError::QueryCountMismatch);
    }
    for (query, statement) in materialized.statements.iter().enumerate() {
        validate_statement(expected_profile, statement).map_err(|source| {
            PcsQuerySettlementVerificationError::InvalidStatement { query, source }
        })?;
    }
    validate_shared_statements(&materialized.statements).map_err(|error| match error {
        PcsQuerySettlementInputError::SharedStatementMismatch { query } => {
            PcsQuerySettlementVerificationError::SharedStatementMismatch { query }
        }
        other => PcsQuerySettlementVerificationError::InvalidProfile(other),
    })?;
    let (transcript_anchor_root, statement_root) = statement_roots_v1(&materialized.statements);
    if materialized.transcript_anchor_root != transcript_anchor_root {
        return Err(PcsQuerySettlementVerificationError::TranscriptAnchorRootMismatch);
    }
    if materialized.statement_root != statement_root {
        return Err(PcsQuerySettlementVerificationError::StatementRootMismatch);
    }
    Ok(())
}

pub fn serialize_all_pcs_query_statements_v1(
    materialized: &AllPcsQueryStatementsV1,
) -> Result<Vec<u8>, bincode::Error> {
    bincode::serialize(materialized)
}

pub fn deserialize_all_pcs_query_statements_v1(
    bytes: &[u8],
) -> Result<AllPcsQueryStatementsV1, bincode::Error> {
    bincode::deserialize(bytes)
}

pub fn prove_pcs_query_settlement_v1(
    config: &SettlementConfig,
    profile: &PcsQuerySettlementProfileV1,
    witness: PcsQuerySettlementWitnessV1,
) -> Result<PcsQuerySettlementProofV1, PcsQuerySettlementInputError> {
    let materialized = materialize_pcs_query_v1(profile, witness)?;
    let log_height = profile.log_height as usize;
    let air = profile.air();
    let MaterializedPcsQueryV1 {
        statement,
        query_trace,
        mmcs_traces,
        fri_flat,
    } = materialized;
    let query_public = query_public_values(profile, &statement)?;
    let query_link_proof = prove(config, &air, query_trace, &query_public);

    let mut input_mmcs_proofs = Vec::with_capacity(PCS_QUERY_INPUT_BATCHES_V1);
    for (batch, trace) in mmcs_traces.into_iter().enumerate() {
        let mmcs_air = VerticalMmcsOpeningAir::new(profile.salted_input_len(batch), log_height);
        let public = vertical_mmcs_air::public_values(
            statement.input_roots[batch],
            statement.index as usize,
            log_height,
            statement.leaf_digests[batch],
        );
        input_mmcs_proofs.push(prove(config, &mmcs_air, trace, &public));
    }

    let fri_air = FriChainAir::new(profile.fri_rounds as usize);
    let fri_public = fri_chain_air::public_values(
        statement.reduced_opening,
        statement.index & 1 == 1,
        statement.final_poly,
        &statement.fri_betas,
    );
    let fri_chain_proof = prove(
        config,
        &fri_air,
        RowMajorMatrix::new(fri_flat, fri_chain_air::N_COLS),
        &fri_public,
    );

    Ok(PcsQuerySettlementProofV1 {
        statement,
        query_link_proof,
        input_mmcs_proofs,
        fri_chain_proof,
    })
}

pub fn prove_all_pcs_queries_settlement_v1(
    config: &SettlementConfig,
    profile: PcsQuerySettlementProfileV1,
    witnesses: Vec<PcsQuerySettlementWitnessV1>,
) -> Result<AllPcsQueriesSettlementProofV1, PcsQuerySettlementInputError> {
    profile.validate()?;
    if witnesses.len() != profile.num_queries as usize {
        return Err(PcsQuerySettlementInputError::QueryCountMismatch);
    }
    let mut queries = Vec::with_capacity(witnesses.len());
    for witness in witnesses {
        queries.push(prove_pcs_query_settlement_v1(config, &profile, witness)?);
    }
    let statements: Vec<_> = queries
        .iter()
        .map(|query| query.statement.clone())
        .collect();
    validate_shared_statements(&statements)?;
    let (transcript_anchor_root, statement_root) = statement_roots_v1(&statements);
    Ok(AllPcsQueriesSettlementProofV1 {
        profile,
        transcript_anchor_root,
        statement_root,
        queries,
    })
}

pub fn verify_pcs_query_settlement_v1(
    config: &SettlementConfig,
    profile: &PcsQuerySettlementProfileV1,
    query_ordinal: usize,
    proof: &PcsQuerySettlementProofV1,
    expected_statement: &PcsQueryStatementV1,
) -> Result<(), PcsQuerySettlementVerificationError> {
    profile
        .validate()
        .map_err(PcsQuerySettlementVerificationError::InvalidProfile)?;
    if &proof.statement != expected_statement {
        return Err(PcsQuerySettlementVerificationError::StatementMismatch {
            query: query_ordinal,
        });
    }
    validate_statement(profile, &proof.statement).map_err(|source| {
        PcsQuerySettlementVerificationError::InvalidStatement {
            query: query_ordinal,
            source,
        }
    })?;
    if proof.input_mmcs_proofs.len() != PCS_QUERY_INPUT_BATCHES_V1 {
        return Err(
            PcsQuerySettlementVerificationError::ComponentCountMismatch {
                query: query_ordinal,
            },
        );
    }
    let query_public = query_public_values(profile, &proof.statement)
        .map_err(PcsQuerySettlementVerificationError::InvalidProfile)?;
    verify(
        config,
        &profile.air(),
        &proof.query_link_proof,
        &query_public,
    )
    .map_err(|source| PcsQuerySettlementVerificationError::QueryLink {
        query: query_ordinal,
        source,
    })?;

    let log_height = profile.log_height as usize;
    for batch in 0..PCS_QUERY_INPUT_BATCHES_V1 {
        let mmcs_air = VerticalMmcsOpeningAir::new(profile.salted_input_len(batch), log_height);
        let public = vertical_mmcs_air::public_values(
            proof.statement.input_roots[batch],
            proof.statement.index as usize,
            log_height,
            proof.statement.leaf_digests[batch],
        );
        verify(config, &mmcs_air, &proof.input_mmcs_proofs[batch], &public).map_err(|source| {
            PcsQuerySettlementVerificationError::InputMmcs {
                query: query_ordinal,
                batch,
                source,
            }
        })?;
    }
    let fri_air = FriChainAir::new(profile.fri_rounds as usize);
    let fri_public = fri_chain_air::public_values(
        proof.statement.reduced_opening,
        proof.statement.index & 1 == 1,
        proof.statement.final_poly,
        &proof.statement.fri_betas,
    );
    verify(config, &fri_air, &proof.fri_chain_proof, &fri_public).map_err(|source| {
        PcsQuerySettlementVerificationError::FriChain {
            query: query_ordinal,
            source,
        }
    })
}

pub fn verify_all_pcs_queries_settlement_v1(
    config: &SettlementConfig,
    expected_profile: &PcsQuerySettlementProfileV1,
    proof: &AllPcsQueriesSettlementProofV1,
    expected_statements: &[PcsQueryStatementV1],
) -> Result<(), PcsQuerySettlementVerificationError> {
    expected_profile
        .validate()
        .map_err(PcsQuerySettlementVerificationError::InvalidProfile)?;
    proof
        .profile
        .validate()
        .map_err(PcsQuerySettlementVerificationError::InvalidProfile)?;
    if &proof.profile != expected_profile {
        return Err(PcsQuerySettlementVerificationError::ProfileMismatch);
    }
    let count = expected_profile.num_queries as usize;
    if proof.queries.len() != count {
        return Err(PcsQuerySettlementVerificationError::QueryCountMismatch);
    }
    if expected_statements.len() != count {
        return Err(PcsQuerySettlementVerificationError::ExpectedStatementCountMismatch);
    }
    let statements: Vec<_> = proof
        .queries
        .iter()
        .map(|query| query.statement.clone())
        .collect();
    validate_shared_statements(&statements).map_err(|error| match error {
        PcsQuerySettlementInputError::SharedStatementMismatch { query } => {
            PcsQuerySettlementVerificationError::SharedStatementMismatch { query }
        }
        other => PcsQuerySettlementVerificationError::InvalidProfile(other),
    })?;
    let (transcript_anchor_root, statement_root) = statement_roots_v1(&statements);
    if proof.transcript_anchor_root != transcript_anchor_root {
        return Err(PcsQuerySettlementVerificationError::TranscriptAnchorRootMismatch);
    }
    if proof.statement_root != statement_root {
        return Err(PcsQuerySettlementVerificationError::StatementRootMismatch);
    }

    for (query_index, (query, expected)) in
        proof.queries.iter().zip(expected_statements).enumerate()
    {
        verify_pcs_query_settlement_v1(config, expected_profile, query_index, query, expected)?;
    }
    Ok(())
}

pub fn verify_production_all_pcs_queries_settlement_v1(
    config: &SettlementConfig,
    proof: &AllPcsQueriesSettlementProofV1,
    expected_statements: &[PcsQueryStatementV1],
) -> Result<(), PcsQuerySettlementVerificationError> {
    verify_all_pcs_queries_settlement_v1(
        config,
        &PcsQuerySettlementProfileV1::production(),
        proof,
        expected_statements,
    )
}

/// Verify the full envelope against the compact root a recursive transcript/
/// fan-out proof is expected to publish.  This is the settlement composition
/// API: component statements remain inside the envelope, while consensus needs
/// only the fixed profile and one transcript-owned digest.
pub fn verify_all_pcs_queries_against_anchor_root_v1(
    config: &SettlementConfig,
    expected_profile: &PcsQuerySettlementProfileV1,
    proof: &AllPcsQueriesSettlementProofV1,
    expected_transcript_anchor_root: &[Val; DIGEST_ELEMS],
) -> Result<(), PcsQuerySettlementVerificationError> {
    if &proof.transcript_anchor_root != expected_transcript_anchor_root {
        return Err(PcsQuerySettlementVerificationError::ExpectedTranscriptAnchorRootMismatch);
    }
    let statements: Vec<_> = proof
        .queries
        .iter()
        .map(|query| query.statement.clone())
        .collect();
    verify_all_pcs_queries_settlement_v1(config, expected_profile, proof, &statements)
}

pub fn verify_production_all_pcs_queries_against_anchor_root_v1(
    config: &SettlementConfig,
    proof: &AllPcsQueriesSettlementProofV1,
    expected_transcript_anchor_root: &[Val; DIGEST_ELEMS],
) -> Result<(), PcsQuerySettlementVerificationError> {
    verify_all_pcs_queries_against_anchor_root_v1(
        config,
        &PcsQuerySettlementProfileV1::production(),
        proof,
        expected_transcript_anchor_root,
    )
}

pub fn serialize_all_pcs_queries_settlement_v1(
    proof: &AllPcsQueriesSettlementProofV1,
) -> Result<Vec<u8>, bincode::Error> {
    bincode::serialize(proof)
}

pub fn deserialize_all_pcs_queries_settlement_v1(
    bytes: &[u8],
) -> Result<AllPcsQueriesSettlementProofV1, bincode::Error> {
    bincode::deserialize(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leaf_hash::leaf_hash_digest;
    use crate::merkle::{compressor, Digest};
    use p3_symmetric::PseudoCompressionFunction;

    fn ext(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(65_537).wrapping_add(i as u32 + 1))
        })
    }

    struct Tree {
        levels: Vec<Vec<Digest>>,
    }

    impl Tree {
        fn with_leaf(levels: usize, index: usize, leaf: Digest, seed: u32) -> Self {
            let mut leaves: Vec<Digest> = (0..1usize << levels)
                .map(|n| {
                    core::array::from_fn(|i| {
                        Val::from_u32(seed.wrapping_add((n * 31 + i + 1) as u32))
                    })
                })
                .collect();
            leaves[index] = leaf;
            let compress = compressor();
            let mut all = vec![leaves.clone()];
            while leaves.len() > 1 {
                leaves = leaves
                    .chunks_exact(2)
                    .map(|pair| compress.compress([pair[0], pair[1]]))
                    .collect();
                all.push(leaves.clone());
            }
            Self { levels: all }
        }

        fn path(&self, mut index: usize) -> Vec<Digest> {
            let mut path = Vec::with_capacity(self.levels.len() - 1);
            for level in &self.levels[..self.levels.len() - 1] {
                path.push(level[index ^ 1]);
                index >>= 1;
            }
            path
        }
    }

    fn fixture(profile: &PcsQuerySettlementProfileV1) -> PcsQuerySettlementWitnessV1 {
        let mut next = 100u32;
        let batches = profile
            .matrix_widths
            .iter()
            .map(|widths| {
                let opened_values = widths
                    .iter()
                    .map(|&width| {
                        (0..width)
                            .map(|_| {
                                next += 1;
                                Val::from_u32(next)
                            })
                            .collect()
                    })
                    .collect();
                let salts = widths
                    .iter()
                    .map(|_| {
                        core::array::from_fn(|_| {
                            next += 1;
                            Val::from_u32(next)
                        })
                    })
                    .collect();
                pcs_query_air::PcsQueryBatchInput {
                    opened_values,
                    salts,
                }
            })
            .collect();
        let air = profile.air();
        let query_input = PcsQueryInput {
            batches,
            p_at_z: (0..air.terms()).map(|i| ext(1_000 + i as u32)).collect(),
            z: ext(90_000),
            index: 9,
            ood_start: core::array::from_fn(|i| Val::from_u32(70_000 + i as u32)),
        };
        let (_, output) = pcs_query_air::build_trace(&air, &query_input);
        let input_siblings = core::array::from_fn(|batch| {
            let salted = flatten_salted_batch(profile, &query_input, batch).unwrap();
            Tree::with_leaf(
                profile.log_height as usize,
                query_input.index,
                leaf_hash_digest(&salted),
                20_000 + batch as u32 * 1_000,
            )
            .path(query_input.index)
        });
        let fri_betas = vec![ext(80_000), ext(80_001)];
        let fri_siblings = vec![ext(81_000), ext(81_001)];
        let fri = fri_chain_air::build_query_chain_from_opening(
            output.reduced_opening,
            &fri_siblings,
            &fri_betas,
            profile.log_height as usize,
            query_input.index,
        );
        let off = (fri_betas.len() - 1) * fri_chain_air::N_COLS + fri_chain_air::C_FOLDED;
        let final_poly = Challenge::from_basis_coefficients_slice(&fri[off..off + D]).unwrap();
        PcsQuerySettlementWitnessV1 {
            query_input,
            input_siblings,
            fri_betas,
            fri_siblings,
            final_poly,
        }
    }

    #[test]
    fn versioned_query_envelope_binds_every_component_at_q128_knobs() {
        let profile =
            PcsQuerySettlementProfileV1::new(1, 4, 2, vec![vec![12], vec![18], vec![12, 12]])
                .unwrap();
        let config = crate::settlement::prod_config();
        let all =
            prove_all_pcs_queries_settlement_v1(&config, profile.clone(), vec![fixture(&profile)])
                .expect("prove composed query envelope");
        let expected = vec![all.queries[0].statement.clone()];
        verify_all_pcs_queries_settlement_v1(&config, &profile, &all, &expected)
            .expect("all five component proofs share one statement");
        verify_all_pcs_queries_against_anchor_root_v1(
            &config,
            &profile,
            &all,
            &all.transcript_anchor_root,
        )
        .expect("compact transcript anchor root composes the envelope");
        let mut wrong_expected_root = all.transcript_anchor_root;
        wrong_expected_root[0] += Val::ONE;
        assert!(matches!(
            verify_all_pcs_queries_against_anchor_root_v1(
                &config,
                &profile,
                &all,
                &wrong_expected_root,
            ),
            Err(PcsQuerySettlementVerificationError::ExpectedTranscriptAnchorRootMismatch)
        ));

        let bytes = serialize_all_pcs_queries_settlement_v1(&all).expect("serialize");
        let received = deserialize_all_pcs_queries_settlement_v1(&bytes).expect("deserialize");
        verify_all_pcs_queries_settlement_v1(&config, &profile, &received, &expected)
            .expect("transport preserves the exact statement and proofs");

        for mutate in 0..5 {
            let mut wrong = expected.clone();
            match mutate {
                0 => wrong[0].input_roots[0][0] += Val::ONE,
                1 => wrong[0].leaf_digests[1][0] += Val::ONE,
                2 => wrong[0].reduced_opening += Challenge::ONE,
                3 => wrong[0].fri_betas[1] += Challenge::ONE,
                _ => wrong[0].final_poly += Challenge::ONE,
            }
            assert!(matches!(
                verify_all_pcs_queries_settlement_v1(&config, &profile, &received, &wrong),
                Err(PcsQuerySettlementVerificationError::StatementMismatch { query: 0 })
            ));
        }

        let mut malformed =
            deserialize_all_pcs_queries_settlement_v1(&bytes).expect("deserialize malformed case");
        malformed.queries[0].statement.fri_betas.pop();
        let malformed_expected = vec![malformed.queries[0].statement.clone()];
        (malformed.transcript_anchor_root, malformed.statement_root) =
            statement_roots_v1(&malformed_expected);
        assert!(matches!(
            verify_all_pcs_queries_settlement_v1(
                &config,
                &profile,
                &malformed,
                &malformed_expected,
            ),
            Err(PcsQuerySettlementVerificationError::InvalidStatement { query: 0, .. })
        ));

        let mut wrong_root =
            deserialize_all_pcs_queries_settlement_v1(&bytes).expect("deserialize root case");
        wrong_root.transcript_anchor_root[0] += Val::ONE;
        assert!(matches!(
            verify_all_pcs_queries_settlement_v1(&config, &profile, &wrong_root, &expected),
            Err(PcsQuerySettlementVerificationError::TranscriptAnchorRootMismatch)
        ));
    }

    #[test]
    fn compact_statement_materialization_recomputes_ordered_roots() {
        let profile =
            PcsQuerySettlementProfileV1::new(2, 4, 2, vec![vec![12], vec![18], vec![12, 12]])
                .unwrap();
        let materialized = materialize_all_pcs_query_statements_v1(
            profile.clone(),
            vec![fixture(&profile), fixture(&profile)],
        )
        .expect("materialize two complete query statements");
        assert_eq!(materialized.profile, profile);
        assert_eq!(materialized.statements.len(), 2);
        assert_eq!(
            statement_roots_v1(&materialized.statements),
            (
                materialized.transcript_anchor_root,
                materialized.statement_root
            )
        );
        assert_ne!(
            transcript_anchor_digest_v1(0, &materialized.statements[0]),
            transcript_anchor_digest_v1(1, &materialized.statements[1]),
            "query ordinal must domain-separate otherwise equal statements"
        );
        verify_all_pcs_query_statements_v1(&profile, &materialized)
            .expect("compact statement contract verifies");

        let bytes = serialize_all_pcs_query_statements_v1(&materialized).expect("serialize");
        let received = deserialize_all_pcs_query_statements_v1(&bytes).expect("deserialize");
        verify_all_pcs_query_statements_v1(&profile, &received)
            .expect("transport preserves compact statement contract");

        let mut wrong_root = received.clone();
        wrong_root.transcript_anchor_root[0] += Val::ONE;
        assert!(matches!(
            verify_all_pcs_query_statements_v1(&profile, &wrong_root),
            Err(PcsQuerySettlementVerificationError::TranscriptAnchorRootMismatch)
        ));

        let mut unshared = received.clone();
        unshared.statements[1].zeta += Challenge::ONE;
        (unshared.transcript_anchor_root, unshared.statement_root) =
            statement_roots_v1(&unshared.statements);
        assert!(matches!(
            verify_all_pcs_query_statements_v1(&profile, &unshared),
            Err(PcsQuerySettlementVerificationError::SharedStatementMismatch { query: 1 })
        ));

        let mut truncated = received;
        truncated.statements.pop();
        assert!(matches!(
            verify_all_pcs_query_statements_v1(&profile, &truncated),
            Err(PcsQuerySettlementVerificationError::QueryCountMismatch)
        ));

        assert!(matches!(
            materialize_all_pcs_query_statements_v1(profile.clone(), vec![fixture(&profile)]),
            Err(PcsQuerySettlementInputError::QueryCountMismatch)
        ));
    }

    #[test]
    fn constraint_checked_materialization_rejects_component_trace_tampering() {
        let profile =
            PcsQuerySettlementProfileV1::new(1, 4, 2, vec![vec![12], vec![18], vec![12, 12]])
                .unwrap();
        let checked = materialize_all_pcs_query_statements_constraint_checked_v1(
            profile.clone(),
            vec![fixture(&profile)],
        )
        .expect("all five honestly generated component traces satisfy their AIRs");
        assert_eq!(checked.statements.len(), 1);

        let mut query_link = materialize_pcs_query_v1(&profile, fixture(&profile)).unwrap();
        query_link.query_trace.values[0] += Val::ONE;
        assert!(matches!(
            check_materialized_pcs_query_constraints_v1(&profile, 7, &query_link),
            Err(PcsQueryConstraintCheckErrorV1::ConstraintViolation {
                query: 7,
                component: PcsQueryConstraintComponentV1::QueryLink,
                ..
            })
        ));

        let mut mmcs = materialize_pcs_query_v1(&profile, fixture(&profile)).unwrap();
        mmcs.mmcs_traces[1].values[0] += Val::ONE;
        assert!(matches!(
            check_materialized_pcs_query_constraints_v1(&profile, 8, &mmcs),
            Err(PcsQueryConstraintCheckErrorV1::ConstraintViolation {
                query: 8,
                component: PcsQueryConstraintComponentV1::InputMmcs { batch: 1 },
                ..
            })
        ));

        let mut fri = materialize_pcs_query_v1(&profile, fixture(&profile)).unwrap();
        fri.fri_flat[fri_chain_air::C_FOLDED] += Val::ONE;
        assert!(matches!(
            check_materialized_pcs_query_constraints_v1(&profile, 9, &fri),
            Err(PcsQueryConstraintCheckErrorV1::ConstraintViolation {
                query: 9,
                component: PcsQueryConstraintComponentV1::FriChain,
                ..
            })
        ));
    }

    #[test]
    fn production_profile_pins_all_120_queries_and_relation_shape() {
        let profile = PcsQuerySettlementProfileV1::production();
        assert_eq!(profile.num_queries as usize, PROD_NUM_QUERIES);
        assert_eq!(profile.log_height as usize, 6);
        assert_eq!(profile.fri_rounds as usize, 2);
        assert_eq!(profile.salted_input_len(0), 16);
        assert_eq!(profile.salted_input_len(1), 23_150);
        assert_eq!(profile.salted_input_len(2), 128);
        assert_eq!(profile.air().terms(), 23_254);
        assert_eq!(profile.air().base_witness_bytes(), 37_109_760);
        assert_eq!(
            VerticalMmcsOpeningAir::new(16, 6).base_witness_bytes(),
            45_888
        );
        assert_eq!(
            VerticalMmcsOpeningAir::new(23_150, 6).base_witness_bytes(),
            11_747_328
        );
        assert_eq!(
            VerticalMmcsOpeningAir::new(128, 6).base_witness_bytes(),
            91_776
        );
        assert_eq!(profile.base_witness_bytes_per_query(), 48_995_024);
        assert_eq!(profile.base_witness_bytes_all_queries(), 5_879_402_880);
        assert_eq!(profile.component_proof_count(), 600);
    }
}
