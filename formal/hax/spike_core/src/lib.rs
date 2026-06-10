//! HAX-SPIKE minimal core.
//!
//! Verbatim copy (modulo the `action` abstraction) of the pure arithmetic core
//! of `reviewed_fq12_account_carried_expected_response_syndrome` from
//! `tools/parameter-search/src/production_reference.rs`. Constants inlined.
//!
//! `reviewed_fq12_challenge_action_vector` (the SHA-256 rejection sampler) is
//! REPLACED by an opaque `action_oracle` — exactly the abstraction the spike's
//! OBLIGATION prescribes ("action left abstract, random-oracle").

pub const Q: u16 = 12_289;
pub const FIELD_BITS: usize = 14;
pub const DIGEST_BYTES: usize = 32;
pub const CHALLENGE_BYTES: usize = 32; // ALGEBRAIC_CHALLENGE_BYTES + HARDENING; value irrelevant to core
pub const LINEAR_SIGMA_PROOF_COMMITMENT_BYTES: usize = 32;
pub const REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS: usize = 12;
pub const REVIEWED_FQ12_ACCOUNT_CARRIED_SYNDROME_PACKED_BITS: usize =
    REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS * FIELD_BITS;
pub const REVIEWED_FQ12_ACCOUNT_CARRIED_SYNDROME_SLACK_BITS: usize =
    DIGEST_BYTES * 8 - REVIEWED_FQ12_ACCOUNT_CARRIED_SYNDROME_PACKED_BITS;
pub const REVIEWED_FQ12_ACCOUNT_CARRIED_ACTION_POLYNOMIAL_COEFFICIENTS_LOW_TO_HIGH: [u16; 13] =
    [25, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

// ---- modular ops (verbatim) ----
fn add_mod(lhs: u16, rhs: u16) -> u16 {
    ((lhs as u32 + rhs as u32) % Q as u32) as u16
}

fn sub_mod(lhs: u16, rhs: u16) -> u16 {
    (Q as u32 + lhs as u32 - rhs as u32).rem_euclid(Q as u32) as u16
}

// ---- bit unpacking (verbatim) ----
fn read_bits_le(bytes: &[u8], bit_offset: usize, bit_len: usize) -> u32 {
    let mut out = 0_u32;
    for i in 0..bit_len {
        let bit = bit_offset + i;
        let byte = bytes[bit / 8];
        let value = (byte >> (bit % 8)) & 1;
        out |= (value as u32) << i;
    }
    out
}

fn any_bits_set_le(bytes: &[u8], bit_offset: usize, bit_len: usize) -> bool {
    // hax fixup #3: `.any(..)` emits Core_models...Iterator.f_any, which is
    // ABSENT from this hax 0.3.7 proof-lib build (only f_all is bundled).
    // Equivalent explicit accumulator loop (extracts to fold_range).
    let mut found = false;
    for i in 0..bit_len {
        let bit = bit_offset + i;
        if ((bytes[bit / 8] >> (bit % 8)) & 1) != 0 {
            found = true;
        }
    }
    found
}

// ---- poly arithmetic (verbatim, Vec-based) ----
fn poly_trim(poly: &mut Vec<u16>) {
    // hax fixup #4: `.last()` emits Core_models.Slice.impl__last, ABSENT from this
    // hax 0.3.7 proof-lib build. Rewritten via len()/index (both have lib models).
    while poly.len() > 0 && poly[poly.len() - 1] == 0 {
        poly.pop();
    }
}

fn poly_mul_mod(lhs: &[u16], rhs: &[u16], modulus: &[u16]) -> Vec<u16> {
    if lhs.is_empty() || rhs.is_empty() {
        return Vec::new();
    }
    // hax fixup #4 (cont.): `.last().copied()` -> len()/index to avoid impl__last.
    assert!(
        modulus.len() > 0 && modulus[modulus.len() - 1] == 1,
        "challenge action modulus must be monic"
    );
    let mut product = vec![0_u16; lhs.len() + rhs.len() - 1];
    for (lhs_degree, &x) in lhs.iter().enumerate() {
        for (rhs_degree, &y) in rhs.iter().enumerate() {
            let degree = lhs_degree + rhs_degree;
            product[degree] = add_mod(product[degree], ((x as u32 * y as u32) % Q as u32) as u16);
        }
    }
    while product.len() >= modulus.len() {
        if product.len() == 0 {
            break;
        }
        let coefficient = product[product.len() - 1];
        let degree_delta = product.len() - modulus.len();
        product.pop();
        if coefficient == 0 {
            poly_trim(&mut product);
            continue;
        }
        for (idx, &modulus_coefficient) in modulus[..modulus.len() - 1].iter().enumerate() {
            let degree = degree_delta + idx;
            product[degree] = sub_mod(
                product[degree],
                ((coefficient as u32 * modulus_coefficient as u32) % Q as u32) as u16,
            );
        }
        poly_trim(&mut product);
    }
    product
}

fn fq12_syndrome_normalize(poly: Vec<u16>) -> [u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS] {
    assert!(poly.len() <= REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS);
    let mut out = [0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS];
    // hax fixup #2: `poly.into_iter().enumerate()` pulls in Alloc.Vec.Into_iter,
    // whose shipped .fsti is internally inconsistent in this hax 0.3.7 build
    // (t_IntoIter allocator param typed `unit` vs `t_Global`). Index loop avoids it.
    for idx in 0..poly.len() {
        out[idx] = poly[idx];
    }
    out
}

fn fq12_syndrome_add(
    lhs: &[u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
    rhs: &[u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
) -> [u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS] {
    let mut out = [0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS];
    for idx in 0..REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS {
        out[idx] = add_mod(lhs[idx], rhs[idx]);
    }
    out
}

fn fq12_syndrome_mul(
    lhs: &[u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
    rhs: &[u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
) -> [u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS] {
    fq12_syndrome_normalize(poly_mul_mod(
        lhs,
        rhs,
        &REVIEWED_FQ12_ACCOUNT_CARRIED_ACTION_POLYNOMIAL_COEFFICIENTS_LOW_TO_HIGH,
    ))
}

// ---- syndrome digest unpacking (verbatim modulo error message) ----
fn unpack_reviewed_fq12_account_carried_syndrome_digest(
    digest: &[u8; DIGEST_BYTES],
) -> Result<[u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS], String> {
    if any_bits_set_le(
        digest,
        REVIEWED_FQ12_ACCOUNT_CARRIED_SYNDROME_PACKED_BITS,
        REVIEWED_FQ12_ACCOUNT_CARRIED_SYNDROME_SLACK_BITS,
    ) {
        return Err("syndrome digest is not canonical: high bits are set".to_string());
    }
    let mut out = [0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS];
    // hax fixup #1: `out.iter_mut().enumerate()` triggers HAX0003 (issue #420);
    // rewritten as an index loop (semantically identical).
    for idx in 0..REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS {
        let decoded = read_bits_le(digest, idx * FIELD_BITS, FIELD_BITS);
        if decoded >= Q as u32 {
            return Err(format!("syndrome residue {idx} is non-canonical: {decoded}"));
        }
        out[idx] = decoded as u16;
    }
    Ok(out)
}

/// Opaque random-oracle stand-in for `reviewed_fq12_challenge_action_vector`.
/// In production this is a SHA-256 rejection sampler; per the spike OBLIGATION
/// it is abstracted (random oracle, not proven algebraic).
fn action_oracle(
    challenge: &[u8; CHALLENGE_BYTES],
) -> [u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS] {
    // Deterministic, total, hax-friendly placeholder: maps each lane through a
    // mod-Q reduction of the challenge bytes. Algebraically irrelevant — the
    // obligation only needs `action : challenge -> Fq12` to be SOME total map.
    let mut out = [0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS];
    for idx in 0..REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS {
        out[idx] = (challenge[idx] as u16) % Q;
    }
    out
}

/// THE TARGET (verbatim modulo `reviewed_fq12_challenge_action_vector` ->
/// `action_oracle`). RHS of `z = a + action(c)*w` over Fq12.
pub fn reviewed_fq12_account_carried_expected_response_syndrome(
    challenge: &[u8; CHALLENGE_BYTES],
    witness_commitment: &[u8; DIGEST_BYTES],
    proof_commitment: &[u8; LINEAR_SIGMA_PROOF_COMMITMENT_BYTES],
) -> Result<[u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS], String> {
    let witness = unpack_reviewed_fq12_account_carried_syndrome_digest(witness_commitment)?;
    let randomizer = unpack_reviewed_fq12_account_carried_syndrome_digest(proof_commitment)?;
    Ok(fq12_syndrome_add(
        &randomizer,
        &fq12_syndrome_mul(&action_oracle(challenge), &witness),
    ))
}
