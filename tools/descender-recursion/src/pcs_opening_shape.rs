//! Canonical production ordering for the leaf PCS opening-reduction terms.
//!
//! Plonky3's `p3_fri::verifier::open_input` visits openings in the exact order
//! `batch -> matrix -> point -> column` and advances one `alpha` power per
//! `(p_at_x, p_at_z)` pair.  The Fiat--Shamir challenger observes the same
//! `p_at_z` values in that order before sampling the FRI batch-combination
//! challenge.  This module is the single index map shared by the vertical OOD
//! transcript and the production opening-reduction consumer; neither side may
//! invent a different flattening.
//!
//! Relation-v3 has one common LDE log height and the following committed batches:
//!
//! 1. the ZK random matrix, one point, width `8 + 4`;
//! 2. the main trace matrix, at `zeta`, width `23_142 + 4`;
//! 3. eight quotient matrices, one point each, width `8 + 4`.
//!
//! Here `4` is the hiding PCS random-codeword width.  The total is 23,254
//! extension-field terms, or 186,032 KoalaBear limbs at extension degree eight.

use crate::fri_fold_air::D;
use descender_account_stark::NUM_RANDOM_CODEWORDS;

/// Production relation-v3 main-trace width.
///
/// This is independently pinned against `descender_account_stark::trace_columns()`
/// in the test below.  A relation/layout change must update the PCS shape and KAT
/// rather than silently reinterpreting the claim stream.
pub const PRODUCTION_TRACE_WIDTH_V1: usize = 23_142;
/// Production quotient matrix count (`num_quotient_chunks`).
pub const PRODUCTION_QUOTIENT_MATRICES_V1: usize = 8;
/// Public width of the randomization matrix opened by the uni-STARK verifier.
pub const PRODUCTION_RANDOM_PUBLIC_WIDTH_V1: usize = D;
/// Public width of each quotient matrix (one extension column flattened by the
/// extension MMCS into `D` base columns before commitment).
pub const PRODUCTION_QUOTIENT_PUBLIC_WIDTH_V1: usize = D;
/// Hiding columns appended to every non-preprocessed matrix.
pub const PRODUCTION_HIDING_WIDTH_V1: usize = NUM_RANDOM_CODEWORDS;

pub const RANDOM_MATRIX_WIDTH_V1: usize =
    PRODUCTION_RANDOM_PUBLIC_WIDTH_V1 + PRODUCTION_HIDING_WIDTH_V1;
pub const TRACE_MATRIX_WIDTH_V1: usize = PRODUCTION_TRACE_WIDTH_V1 + PRODUCTION_HIDING_WIDTH_V1;
pub const QUOTIENT_MATRIX_WIDTH_V1: usize =
    PRODUCTION_QUOTIENT_PUBLIC_WIDTH_V1 + PRODUCTION_HIDING_WIDTH_V1;

pub const RANDOM_TERMS_V1: usize = RANDOM_MATRIX_WIDTH_V1;
pub const TRACE_TERMS_V1: usize = TRACE_MATRIX_WIDTH_V1;
pub const QUOTIENT_TERMS_V1: usize = PRODUCTION_QUOTIENT_MATRICES_V1 * QUOTIENT_MATRIX_WIDTH_V1;
pub const PRODUCTION_OPENING_TERMS_V1: usize = RANDOM_TERMS_V1 + TRACE_TERMS_V1 + QUOTIENT_TERMS_V1;
pub const PRODUCTION_OOD_BASE_ELEMS_V1: usize = PRODUCTION_OPENING_TERMS_V1 * D;

// Compile-time arithmetic pins.  These fail at the definition site if the
// extension degree or hiding width drifts without an explicit shape update.
const _: [(); 12] = [(); RANDOM_MATRIX_WIDTH_V1];
const _: [(); 23_146] = [(); TRACE_MATRIX_WIDTH_V1];
const _: [(); 12] = [(); QUOTIENT_MATRIX_WIDTH_V1];
const _: [(); 23_254] = [(); PRODUCTION_OPENING_TERMS_V1];
const _: [(); 186_032] = [(); PRODUCTION_OOD_BASE_ELEMS_V1];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductionOpeningBatch {
    Random,
    Trace,
    Quotient,
}

/// Address of one extension-field term in Plonky3 `open_input` order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProductionOpeningTerm {
    pub batch: ProductionOpeningBatch,
    pub matrix: usize,
    pub point: usize,
    pub column: usize,
    /// The alpha exponent used by `open_input` for this term.  All production
    /// matrices share one log-height bucket, so this is the global term index.
    pub alpha_ordinal: usize,
}

impl ProductionOpeningTerm {
    pub const fn public_width(self) -> usize {
        match self.batch {
            ProductionOpeningBatch::Random => PRODUCTION_RANDOM_PUBLIC_WIDTH_V1,
            ProductionOpeningBatch::Trace => PRODUCTION_TRACE_WIDTH_V1,
            ProductionOpeningBatch::Quotient => PRODUCTION_QUOTIENT_PUBLIC_WIDTH_V1,
        }
    }

    pub const fn is_hiding_column(self) -> bool {
        self.column >= self.public_width()
    }
}

/// Map an extension-term ordinal to its exact batch/matrix/point/column address.
pub const fn production_opening_term(index: usize) -> Option<ProductionOpeningTerm> {
    if index >= PRODUCTION_OPENING_TERMS_V1 {
        return None;
    }
    if index < RANDOM_TERMS_V1 {
        return Some(ProductionOpeningTerm {
            batch: ProductionOpeningBatch::Random,
            matrix: 0,
            point: 0,
            column: index,
            alpha_ordinal: index,
        });
    }

    let trace_index = index - RANDOM_TERMS_V1;
    if trace_index < TRACE_TERMS_V1 {
        return Some(ProductionOpeningTerm {
            batch: ProductionOpeningBatch::Trace,
            matrix: 0,
            point: 0,
            column: trace_index,
            alpha_ordinal: index,
        });
    }

    let quotient_index = trace_index - TRACE_TERMS_V1;
    Some(ProductionOpeningTerm {
        batch: ProductionOpeningBatch::Quotient,
        matrix: quotient_index / QUOTIENT_MATRIX_WIDTH_V1,
        point: 0,
        column: quotient_index % QUOTIENT_MATRIX_WIDTH_V1,
        alpha_ordinal: index,
    })
}

/// Map one flattened base-field OOD element to its extension term and limb.
pub const fn production_ood_limb(base_index: usize) -> Option<(ProductionOpeningTerm, usize)> {
    if base_index >= PRODUCTION_OOD_BASE_ELEMS_V1 {
        return None;
    }
    let term = production_opening_term(base_index / D);
    match term {
        Some(term) => Some((term, base_index % D)),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_shape_matches_relation_and_vertical_stream() {
        assert_eq!(
            descender_account_stark::trace_columns(),
            PRODUCTION_TRACE_WIDTH_V1
        );
        assert_eq!(
            PRODUCTION_OOD_BASE_ELEMS_V1,
            crate::block_ood_binding_air::PRODUCTION_OOD_STREAM_ELEMS_V1
        );
        assert_eq!(PRODUCTION_OPENING_TERMS_V1, 23_254);
    }

    #[test]
    fn boundary_addresses_follow_p3_batch_matrix_point_column_order() {
        let at = |i| production_opening_term(i).expect("in range");
        assert_eq!(
            at(0),
            ProductionOpeningTerm {
                batch: ProductionOpeningBatch::Random,
                matrix: 0,
                point: 0,
                column: 0,
                alpha_ordinal: 0,
            }
        );
        assert!(at(RANDOM_TERMS_V1 - 1).is_hiding_column());
        assert_eq!(at(RANDOM_TERMS_V1).batch, ProductionOpeningBatch::Trace);
        assert_eq!(at(RANDOM_TERMS_V1).point, 0);
        assert_eq!(at(RANDOM_TERMS_V1).column, 0);
        assert_eq!(at(RANDOM_TERMS_V1 + TRACE_TERMS_V1 - 1).point, 0);
        assert_eq!(
            at(RANDOM_TERMS_V1 + TRACE_TERMS_V1 - 1).column,
            TRACE_MATRIX_WIDTH_V1 - 1
        );
        assert_eq!(
            at(RANDOM_TERMS_V1 + TRACE_TERMS_V1).batch,
            ProductionOpeningBatch::Quotient
        );
        let last = at(PRODUCTION_OPENING_TERMS_V1 - 1);
        assert_eq!(last.matrix, PRODUCTION_QUOTIENT_MATRICES_V1 - 1);
        assert_eq!(last.column, QUOTIENT_MATRIX_WIDTH_V1 - 1);
        assert!(last.is_hiding_column());
        assert_eq!(last.alpha_ordinal, PRODUCTION_OPENING_TERMS_V1 - 1);
        assert!(production_opening_term(PRODUCTION_OPENING_TERMS_V1).is_none());
    }

    #[test]
    fn base_limb_map_is_extension_element_major() {
        for i in [0, D - 1, D, PRODUCTION_OOD_BASE_ELEMS_V1 - 1] {
            let (term, limb) = production_ood_limb(i).expect("in range");
            assert_eq!(term.alpha_ordinal, i / D);
            assert_eq!(limb, i % D);
        }
        assert!(production_ood_limb(PRODUCTION_OOD_BASE_ELEMS_V1).is_none());
    }
}
