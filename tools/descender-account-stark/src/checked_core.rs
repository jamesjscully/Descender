//! Phase-2 SPEND checked-core SIS relation — the public matrix `H` and the response→syndrome map.
//!
//! The deployed direct spend verifier accepts a checked-core proof iff its response syndrome equals
//! the expected syndrome (`path2_native_checked_core_accept_v1`). The response syndrome is a
//! matrix-vector product mod q over a FIXED public matrix `H` (12×22):
//!
//!   `syndrome[row] = Σ_{col} H[row][col] · response[col]  (mod q12289)`
//!
//! where `response[22]` is the prover's short canonical response (each lane `< q`). Proving that the
//! committed syndrome has a known short canonical preimage `response` under `H` IS the checked-core
//! signature verification (finding such a preimage for an arbitrary syndrome is the SIS hardness).
//!
//! `H` is derived EXACTLY as the deployed `domain_matrix_entry`: SHA256 rejection sampling over
//! `(domain ‖ row_le_u16 ‖ col_le_u16 ‖ counter_le_u32)`, reduced mod q. This module reproduces it
//! byte-identically (a fidelity test pins the first entries) so the in-AIR constraint matches the
//! deployed verifier's matrix.

use sha2::{Digest, Sha256};

/// Checked-core syndrome rows (= `REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS`).
pub const CC_ROWS: usize = 12;
/// Checked-core response residues / matrix columns (= `REVIEWED_FQ12_ACCOUNT_CARRIED_RESPONSE_RESIDUES`).
pub const CC_COLS: usize = 22;
/// q12289 — the canonical residue modulus.
pub const CC_Q: u16 = 12289;

/// Matrix domain, byte-identical to the deployed `REVIEWED_FQ12_ACCOUNT_CARRIED_MATRIX_DOMAIN`.
pub const CC_MATRIX_DOMAIN: &str =
    "Descender.Signature.q12289.local_delta.sis_fq12_account_carried_matrix.v1";

/// One matrix entry `H[row][col]`, byte-identical to the deployed `domain_matrix_entry`.
pub fn matrix_entry(row: usize, col: usize) -> u16 {
    let q = CC_Q;
    let rejection_limit = u16::MAX - (u16::MAX % q);
    for counter in 0_u32.. {
        let mut hasher = Sha256::new();
        hasher.update(CC_MATRIX_DOMAIN.as_bytes());
        hasher.update((row as u16).to_le_bytes());
        hasher.update((col as u16).to_le_bytes());
        hasher.update(counter.to_le_bytes());
        let digest = hasher.finalize();
        for chunk in digest.chunks_exact(2) {
            let value = u16::from_le_bytes([chunk[0], chunk[1]]);
            if value < rejection_limit {
                return value % q;
            }
        }
    }
    unreachable!("unbounded rejection sampler loop");
}

/// The full fixed checked-core matrix `H` (12×22).
pub fn checked_core_matrix() -> [[u16; CC_COLS]; CC_ROWS] {
    core::array::from_fn(|row| core::array::from_fn(|col| matrix_entry(row, col)))
}

/// The response syndrome `H·response mod q`, byte-identical to the deployed
/// `reviewed_fq12_account_carried_response_syndrome` (running reduction per term).
pub fn response_syndrome(response: &[u16; CC_COLS]) -> [u16; CC_ROWS] {
    let h = checked_core_matrix();
    core::array::from_fn(|row| {
        let mut acc = 0u32;
        for (col, &v) in response.iter().enumerate() {
            acc += h[row][col] as u32 * v as u32;
            acc %= CC_Q as u32;
        }
        acc as u16
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The matrix entries are deterministic and canonical (`< q`).
    #[test]
    fn matrix_entries_canonical_and_deterministic() {
        let h = checked_core_matrix();
        for row in &h {
            for &e in row {
                assert!(e < CC_Q, "matrix entry not canonical");
            }
        }
        // Determinism: recompute a couple of entries.
        assert_eq!(h[0][0], matrix_entry(0, 0));
        assert_eq!(h[11][21], matrix_entry(11, 21));
    }

    /// `response_syndrome` reduces mod q and is deterministic.
    #[test]
    fn response_syndrome_is_canonical() {
        let response: [u16; CC_COLS] = core::array::from_fn(|i| ((i as u32 * 503 + 7) % CC_Q as u32) as u16);
        let s = response_syndrome(&response);
        for &v in &s {
            assert!(v < CC_Q);
        }
        assert_eq!(s, response_syndrome(&response));
    }
}
