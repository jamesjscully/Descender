#![allow(clippy::manual_is_multiple_of, clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;

const PROFILE: &str = "validated-q12289-n32-w1-r26-compact-completed-response";
const HASH_DOMAIN: &str = "Descender.Signature.q12289.compact_completed_response.v1";
const Q: u16 = 12_289;
const GROUPS: usize = 16;
const LOCAL_PIVOT: usize = 3;
const LOCAL_FREE: usize = 2;
const VIEW_COORDS: usize = GROUPS * (LOCAL_PIVOT + LOCAL_FREE);
const PIVOT_COORDS: usize = GROUPS * LOCAL_PIVOT;
const PK_SEED_BYTES: usize = 32;
const PK_PAYLOAD_BYTES: usize = 140;
const PK_BYTES: usize = PK_SEED_BYTES + PK_PAYLOAD_BYTES;
const FIELD_BITS: usize = 14;
const REPS: usize = 26;
const CHALLENGE_BITS_PER_REP: usize = 5;
const CHALLENGE_USED_BITS: usize = REPS * CHALLENGE_BITS_PER_REP;
const CHALLENGE_BYTES: usize = CHALLENGE_USED_BITS.div_ceil(8);
const CHALLENGE_LAST_BYTE_MASK: u8 = 0x03;
const RESPONSE_BOUND: i16 = 48;
const COMPLETED_RESPONSE_BITS: usize = 7;
const RESPONSE_BYTES_PER_REP: usize = LOCAL_FREE;
const RESPONSE_BYTES: usize = REPS * RESPONSE_BYTES_PER_REP;
const STRUCTURAL_SIGNATURE_BYTES: usize = CHALLENGE_BYTES + RESPONSE_BYTES;
const COMPLETED_RESPONSE_COORDS_PER_REP: usize = GROUPS * LOCAL_FREE;
const COMPLETED_RESPONSE_COORDS: usize = REPS * COMPLETED_RESPONSE_COORDS_PER_REP;
const COMPLETED_RESPONSE_BYTES: usize =
    (COMPLETED_RESPONSE_COORDS * COMPLETED_RESPONSE_BITS).div_ceil(8);
const COMPLETED_RESPONSE_SIGNATURE_BYTES: usize = CHALLENGE_BYTES + COMPLETED_RESPONSE_BYTES;
const EXPLICIT_COMMITMENT_BYTES: usize = REPS * PK_PAYLOAD_BYTES;
const EXPLICIT_COMMITMENT_SIGNATURE_BYTES: usize =
    STRUCTURAL_SIGNATURE_BYTES + EXPLICIT_COMMITMENT_BYTES;
const RECONSTRUCTED_COMMITMENT_BYTES: usize = EXPLICIT_COMMITMENT_BYTES;
const ACTIVE_COMMITMENT_COORDS_PER_REP: usize = LOCAL_PIVOT + LOCAL_FREE;
const ACTIVE_COMMITMENT_COORDS: usize = REPS * ACTIVE_COMMITMENT_COORDS_PER_REP;
const ACTIVE_COMMITMENT_BYTES: usize = (ACTIVE_COMMITMENT_COORDS * FIELD_BITS).div_ceil(8);

const LOCAL_BLOCK: [[u16; LOCAL_FREE]; LOCAL_PIVOT] =
    [[10_849, 1_171], [1_597, 1_615], [6_102, 11_189]];

#[derive(Clone, Debug)]
struct Config {
    public_key: Vec<u8>,
    message: Vec<u8>,
    signature: Vec<u8>,
    mode: SignatureMode,
    json: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SignatureMode {
    CompletedResponse,
    ExplicitCommitment,
    StructuralOnly,
}

impl SignatureMode {
    fn name(self) -> &'static str {
        match self {
            SignatureMode::CompletedResponse => "completed-response",
            SignatureMode::ExplicitCommitment => "explicit-commitment",
            SignatureMode::StructuralOnly => "structural-only",
        }
    }

    fn is_structural_only(self) -> bool {
        self == SignatureMode::StructuralOnly
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CompactChallenge {
    active_block: [usize; REPS],
    sign: [i8; REPS],
}

#[derive(Clone, Debug)]
struct PublicView {
    pivot: [[u16; LOCAL_PIVOT]; GROUPS],
    free: [[u16; LOCAL_FREE]; GROUPS],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CompactResponses {
    active_free: [[i16; LOCAL_FREE]; REPS],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CompletedResponses {
    free: [[[i16; LOCAL_FREE]; GROUPS]; REPS],
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct CompactVerifyReport {
    accepted: bool,
    profile: String,
    hash_domain: String,
    signature_mode: String,
    structural_only: bool,
    public_key_bytes: usize,
    expected_public_key_bytes: usize,
    message_bytes: usize,
    signature_bytes: usize,
    expected_signature_bytes: usize,
    challenge_bytes: usize,
    response_bytes: usize,
    response_bound: i16,
    completed_response_bytes: Option<usize>,
    active_commitment_bytes: Option<usize>,
    reconstructed_commitment_bytes: Option<usize>,
    explicit_commitment_bytes: Option<usize>,
    hash_frame_bytes: Option<usize>,
    hash_frame_sha256: Option<String>,
    claimed_challenge_hex: Option<String>,
    recomputed_challenge_hex: Option<String>,
    reason: String,
}

fn read_bits_le(bytes: &[u8], bit_offset: usize, bit_len: usize) -> u32 {
    let mut out = 0_u32;
    for i in 0..bit_len {
        let bit = (bytes[(bit_offset + i) / 8] >> ((bit_offset + i) % 8)) & 1;
        out |= (bit as u32) << i;
    }
    out
}

fn write_bits_le(bytes: &mut [u8], bit_offset: usize, bit_len: usize, value: u32) {
    for i in 0..bit_len {
        if ((value >> i) & 1) != 0 {
            bytes[(bit_offset + i) / 8] |= 1 << ((bit_offset + i) % 8);
        }
    }
}

fn decode_challenge(bytes: &[u8]) -> Result<CompactChallenge, String> {
    if bytes.len() != CHALLENGE_BYTES {
        return Err(format!(
            "challenge has {} bytes, expected {CHALLENGE_BYTES}",
            bytes.len()
        ));
    }
    if (bytes[CHALLENGE_BYTES - 1] & !CHALLENGE_LAST_BYTE_MASK) != 0 {
        return Err("challenge is not canonical: high bits are set".to_string());
    }
    let mut active_block = [0_usize; REPS];
    let mut sign = [0_i8; REPS];
    for rep in 0..REPS {
        let bit = rep * CHALLENGE_BITS_PER_REP;
        active_block[rep] = read_bits_le(bytes, bit, 4) as usize;
        sign[rep] = if read_bits_le(bytes, bit + 4, 1) == 0 {
            1
        } else {
            -1
        };
    }
    Ok(CompactChallenge { active_block, sign })
}

#[cfg(test)]
fn encode_challenge(challenge: &CompactChallenge) -> [u8; CHALLENGE_BYTES] {
    let mut out = [0_u8; CHALLENGE_BYTES];
    for rep in 0..REPS {
        let bit = rep * CHALLENGE_BITS_PER_REP;
        write_bits_le(&mut out, bit, 4, challenge.active_block[rep] as u32);
        write_bits_le(
            &mut out,
            bit + 4,
            1,
            if challenge.sign[rep] == 1 { 0 } else { 1 },
        );
    }
    out
}

fn decode_responses(bytes: &[u8]) -> Result<CompactResponses, String> {
    if bytes.len() != RESPONSE_BYTES {
        return Err(format!(
            "response payload has {} bytes, expected {RESPONSE_BYTES}",
            bytes.len()
        ));
    }
    let mut active_free = [[0_i16; LOCAL_FREE]; REPS];
    for rep in 0..REPS {
        for coord in 0..LOCAL_FREE {
            let raw = bytes[rep * RESPONSE_BYTES_PER_REP + coord];
            if raw > (2 * RESPONSE_BOUND) as u8 {
                return Err(format!(
                    "active response coordinate {coord} in round {rep} is non-canonical: {raw}"
                ));
            }
            active_free[rep][coord] = raw as i16 - RESPONSE_BOUND;
        }
    }
    Ok(CompactResponses { active_free })
}

fn decode_completed_responses(bytes: &[u8]) -> Result<CompletedResponses, String> {
    if bytes.len() != COMPLETED_RESPONSE_BYTES {
        return Err(format!(
            "completed response payload has {} bytes, expected {COMPLETED_RESPONSE_BYTES}",
            bytes.len()
        ));
    }
    let mut free = [[[0_i16; LOCAL_FREE]; GROUPS]; REPS];
    for idx in 0..COMPLETED_RESPONSE_COORDS {
        let raw = read_bits_le(
            bytes,
            idx * COMPLETED_RESPONSE_BITS,
            COMPLETED_RESPONSE_BITS,
        ) as u8;
        if raw > (2 * RESPONSE_BOUND) as u8 {
            let rep = idx / COMPLETED_RESPONSE_COORDS_PER_REP;
            let local = idx % COMPLETED_RESPONSE_COORDS_PER_REP;
            let block = local / LOCAL_FREE;
            let coord = local % LOCAL_FREE;
            return Err(format!(
                "completed response coordinate {coord} in block {block}, round {rep} is non-canonical: {raw}"
            ));
        }
        let rep = idx / COMPLETED_RESPONSE_COORDS_PER_REP;
        let local = idx % COMPLETED_RESPONSE_COORDS_PER_REP;
        free[rep][local / LOCAL_FREE][local % LOCAL_FREE] = raw as i16 - RESPONSE_BOUND;
    }
    Ok(CompletedResponses { free })
}

#[cfg(test)]
fn encode_responses(responses: &CompactResponses) -> Vec<u8> {
    let mut out = Vec::with_capacity(RESPONSE_BYTES);
    for rep in 0..REPS {
        for coord in 0..LOCAL_FREE {
            let value = responses.active_free[rep][coord];
            assert!((-RESPONSE_BOUND..=RESPONSE_BOUND).contains(&value));
            out.push((value + RESPONSE_BOUND) as u8);
        }
    }
    out
}

#[cfg(test)]
fn encode_completed_responses(responses: &CompletedResponses) -> Vec<u8> {
    let mut out = vec![0_u8; COMPLETED_RESPONSE_BYTES];
    for rep in 0..REPS {
        for block in 0..GROUPS {
            for coord in 0..LOCAL_FREE {
                let idx = rep * COMPLETED_RESPONSE_COORDS_PER_REP + block * LOCAL_FREE + coord;
                let value = responses.free[rep][block][coord];
                assert!((-RESPONSE_BOUND..=RESPONSE_BOUND).contains(&value));
                write_bits_le(
                    &mut out,
                    idx * COMPLETED_RESPONSE_BITS,
                    COMPLETED_RESPONSE_BITS,
                    (value + RESPONSE_BOUND) as u32,
                );
            }
        }
    }
    out
}

fn unpack_q_elements(bytes: &[u8], count: usize) -> Result<Vec<u16>, String> {
    let expected = count * FIELD_BITS / 8;
    if count * FIELD_BITS % 8 != 0 {
        return Err("field packing count is not byte aligned".to_string());
    }
    if bytes.len() != expected {
        return Err(format!(
            "field payload has {} bytes, expected {expected}",
            bytes.len()
        ));
    }
    let mut coords = Vec::with_capacity(count);
    for idx in 0..count {
        let value = read_bits_le(bytes, idx * FIELD_BITS, FIELD_BITS) as u16;
        if value >= Q {
            return Err(format!(
                "non-canonical q-coordinate at index {idx}: {value}"
            ));
        }
        coords.push(value);
    }
    Ok(coords)
}

fn pack_q_elements_padded(coords: &[u16]) -> Vec<u8> {
    let mut out = vec![0_u8; (coords.len() * FIELD_BITS).div_ceil(8)];
    for (idx, &value) in coords.iter().enumerate() {
        assert!(value < Q);
        write_bits_le(&mut out, idx * FIELD_BITS, FIELD_BITS, value as u32);
    }
    out
}

fn decode_explicit_commitments(bytes: &[u8]) -> Result<Vec<u16>, String> {
    if bytes.len() != EXPLICIT_COMMITMENT_BYTES {
        return Err(format!(
            "explicit commitment payload has {} bytes, expected {EXPLICIT_COMMITMENT_BYTES}",
            bytes.len()
        ));
    }
    unpack_q_elements(bytes, REPS * VIEW_COORDS)
}

fn decode_public_key(public_key: &[u8]) -> Result<PublicView, String> {
    if public_key.len() != PK_BYTES {
        return Err(format!(
            "public key has {} bytes, expected {PK_BYTES}",
            public_key.len()
        ));
    }
    let coords = unpack_q_elements(&public_key[PK_SEED_BYTES..], VIEW_COORDS)?;
    let mut pivot = [[0_u16; LOCAL_PIVOT]; GROUPS];
    let mut free = [[0_u16; LOCAL_FREE]; GROUPS];
    for (idx, &value) in coords[..PIVOT_COORDS].iter().enumerate() {
        pivot[idx / LOCAL_PIVOT][idx % LOCAL_PIVOT] = value;
    }
    for (idx, &value) in coords[PIVOT_COORDS..].iter().enumerate() {
        free[idx / LOCAL_FREE][idx % LOCAL_FREE] = value;
    }
    Ok(PublicView { pivot, free })
}

fn mod_q(x: i64) -> u16 {
    x.rem_euclid(Q as i64) as u16
}

fn negate_mod(value: u16) -> u16 {
    if value == 0 { 0 } else { Q - value }
}

fn challenge_mul(coeff: i8, value: u16) -> u16 {
    match coeff {
        -1 => negate_mod(value),
        0 => 0,
        1 => value,
        other => panic!("invalid compact challenge coefficient {other}"),
    }
}

fn sub_mod(lhs: u16, rhs: u16) -> u16 {
    mod_q(lhs as i64 - rhs as i64)
}

fn systematic_pivot_from_free(free: [i16; LOCAL_FREE]) -> [u16; LOCAL_PIVOT] {
    let free_mod = [mod_q(free[0] as i64), mod_q(free[1] as i64)];
    let mut pivot = [0_u16; LOCAL_PIVOT];
    for row in 0..LOCAL_PIVOT {
        let mut acc = 0_i64;
        for (coord, free_value) in free_mod.iter().enumerate() {
            acc += LOCAL_BLOCK[row][coord] as i64 * *free_value as i64;
        }
        pivot[row] = mod_q(-acc);
    }
    pivot
}

fn reconstructed_active_commitment_coords(
    public_key: &PublicView,
    challenge: &CompactChallenge,
    responses: &CompactResponses,
) -> Vec<u16> {
    let mut coords = Vec::with_capacity(ACTIVE_COMMITMENT_COORDS);
    for rep in 0..REPS {
        let block = challenge.active_block[rep];
        let coeff = challenge.sign[rep];
        let active_free = responses.active_free[rep];
        let active_pivot = systematic_pivot_from_free(active_free);
        for (coord, &value) in active_pivot.iter().enumerate() {
            coords.push(sub_mod(
                value,
                challenge_mul(coeff, public_key.pivot[block][coord]),
            ));
        }
        for (coord, &value) in active_free.iter().enumerate() {
            coords.push(sub_mod(
                mod_q(value as i64),
                challenge_mul(coeff, public_key.free[block][coord]),
            ));
        }
    }
    coords
}

fn active_responses_from_completed(
    challenge: &CompactChallenge,
    responses: &CompletedResponses,
) -> CompactResponses {
    let mut active_free = [[0_i16; LOCAL_FREE]; REPS];
    for (rep, active_free_rep) in active_free.iter_mut().enumerate() {
        *active_free_rep = responses.free[rep][challenge.active_block[rep]];
    }
    CompactResponses { active_free }
}

fn reconstructed_commitment_coords_from_completed(
    public_key: &PublicView,
    challenge: &CompactChallenge,
    responses: &CompletedResponses,
) -> Vec<u16> {
    let mut coords = Vec::with_capacity(REPS * VIEW_COORDS);
    for rep in 0..REPS {
        for block in 0..GROUPS {
            let coeff = if block == challenge.active_block[rep] {
                challenge.sign[rep]
            } else {
                0
            };
            let pivot = systematic_pivot_from_free(responses.free[rep][block]);
            for (coord, &value) in pivot.iter().enumerate() {
                coords.push(sub_mod(
                    value,
                    challenge_mul(coeff, public_key.pivot[block][coord]),
                ));
            }
        }
        for block in 0..GROUPS {
            let coeff = if block == challenge.active_block[rep] {
                challenge.sign[rep]
            } else {
                0
            };
            for coord in 0..LOCAL_FREE {
                coords.push(sub_mod(
                    mod_q(responses.free[rep][block][coord] as i64),
                    challenge_mul(coeff, public_key.free[block][coord]),
                ));
            }
        }
    }
    coords
}

fn reconstructed_commitment_bytes_from_completed(
    public_key: &PublicView,
    challenge: &CompactChallenge,
    responses: &CompletedResponses,
) -> Vec<u8> {
    pack_q_elements_padded(&reconstructed_commitment_coords_from_completed(
        public_key, challenge, responses,
    ))
}

fn explicit_active_commitment_coords(
    commitment_coords: &[u16],
    challenge: &CompactChallenge,
) -> Vec<u16> {
    debug_assert_eq!(commitment_coords.len(), REPS * VIEW_COORDS);
    let mut coords = Vec::with_capacity(ACTIVE_COMMITMENT_COORDS);
    for rep in 0..REPS {
        let block = challenge.active_block[rep];
        let base = rep * VIEW_COORDS;
        for coord in 0..LOCAL_PIVOT {
            coords.push(commitment_coords[base + block * LOCAL_PIVOT + coord]);
        }
        let free_base = base + PIVOT_COORDS;
        for coord in 0..LOCAL_FREE {
            coords.push(commitment_coords[free_base + block * LOCAL_FREE + coord]);
        }
    }
    coords
}

fn append_frame(out: &mut Vec<u8>, label: &[u8], data: &[u8]) {
    out.extend_from_slice(&(label.len() as u16).to_le_bytes());
    out.extend_from_slice(label);
    out.extend_from_slice(&(data.len() as u64).to_le_bytes());
    out.extend_from_slice(data);
}

fn append_u64(out: &mut Vec<u8>, label: &[u8], value: u64) {
    append_frame(out, label, &value.to_le_bytes());
}

fn hash_frame_bytes(
    public_key: &[u8],
    message: &[u8],
    commitment_label: &[u8],
    commitments: &[u8],
) -> Vec<u8> {
    let mut frame = Vec::new();
    append_frame(&mut frame, b"domain", HASH_DOMAIN.as_bytes());
    append_frame(&mut frame, b"profile", PROFILE.as_bytes());
    append_u64(&mut frame, b"q", Q as u64);
    append_u64(&mut frame, b"groups", GROUPS as u64);
    append_u64(&mut frame, b"local_pivot", LOCAL_PIVOT as u64);
    append_u64(&mut frame, b"local_free", LOCAL_FREE as u64);
    append_u64(&mut frame, b"repetitions", REPS as u64);
    append_u64(&mut frame, b"block_weight", 1);
    append_u64(&mut frame, b"response_bound", RESPONSE_BOUND as u64);
    append_frame(&mut frame, b"public_key", public_key);
    append_frame(&mut frame, b"message", message);
    append_frame(&mut frame, commitment_label, commitments);
    frame
}

// LEGACY SHA-256 Fiat–Shamir experiment, NOT the deployed signer / NOT the KAT path.
// This is an experimental compact-format probe binary; it is out of scope for the
// 2026-06-18 spend-FS Poseidon2 re-host (which covers the deployed stateful signer and
// the prodref challenge/authorization derivations). Left on SHA-256 deliberately.
fn hash_frame_to_challenge_bytes(frame: &[u8]) -> [u8; CHALLENGE_BYTES] {
    let digest = Sha256::digest(frame);
    let mut out = [0_u8; CHALLENGE_BYTES];
    out.copy_from_slice(&digest[..CHALLENGE_BYTES]);
    out[CHALLENGE_BYTES - 1] &= CHALLENGE_LAST_BYTE_MASK;
    out
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex_encode(&Sha256::digest(bytes))
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn hex_decode(input: &str) -> Result<Vec<u8>, String> {
    let s = input
        .strip_prefix("0x")
        .or_else(|| input.strip_prefix("0X"))
        .unwrap_or(input);
    if s.len() % 2 != 0 {
        return Err("hex string has odd length".to_string());
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    for pair in s.as_bytes().chunks_exact(2) {
        let hi = hex_value(pair[0]).ok_or_else(|| "invalid hex digit".to_string())?;
        let lo = hex_value(pair[1]).ok_or_else(|| "invalid hex digit".to_string())?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

fn expected_signature_bytes(mode: SignatureMode) -> usize {
    match mode {
        SignatureMode::CompletedResponse => COMPLETED_RESPONSE_SIGNATURE_BYTES,
        SignatureMode::ExplicitCommitment => EXPLICIT_COMMITMENT_SIGNATURE_BYTES,
        SignatureMode::StructuralOnly => STRUCTURAL_SIGNATURE_BYTES,
    }
}

fn reject(
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
    mode: SignatureMode,
    claimed: Option<String>,
    recomputed: Option<String>,
    completed_response_bytes: Option<usize>,
    active_commitment_bytes: Option<usize>,
    reconstructed_commitment_bytes: Option<usize>,
    explicit_commitment_bytes: Option<usize>,
    hash_frame_bytes: Option<usize>,
    hash_frame_sha256: Option<String>,
    reason: String,
) -> CompactVerifyReport {
    CompactVerifyReport {
        accepted: false,
        profile: PROFILE.to_string(),
        hash_domain: HASH_DOMAIN.to_string(),
        signature_mode: mode.name().to_string(),
        structural_only: mode.is_structural_only(),
        public_key_bytes: public_key.len(),
        expected_public_key_bytes: PK_BYTES,
        message_bytes: message.len(),
        signature_bytes: signature.len(),
        expected_signature_bytes: expected_signature_bytes(mode),
        challenge_bytes: CHALLENGE_BYTES,
        response_bytes: RESPONSE_BYTES,
        response_bound: RESPONSE_BOUND,
        completed_response_bytes,
        active_commitment_bytes,
        reconstructed_commitment_bytes,
        explicit_commitment_bytes,
        hash_frame_bytes,
        hash_frame_sha256,
        claimed_challenge_hex: claimed,
        recomputed_challenge_hex: recomputed,
        reason,
    }
}

fn verify_compact_signature(
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
    mode: SignatureMode,
) -> CompactVerifyReport {
    let claimed =
        (signature.len() >= CHALLENGE_BYTES).then(|| hex_encode(&signature[..CHALLENGE_BYTES]));
    let expected_signature_bytes = expected_signature_bytes(mode);
    if public_key.len() != PK_BYTES {
        return reject(
            public_key,
            message,
            signature,
            mode,
            claimed,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            format!(
                "public key has {} bytes, expected {PK_BYTES}",
                public_key.len()
            ),
        );
    }
    if signature.len() != expected_signature_bytes {
        return reject(
            public_key,
            message,
            signature,
            mode,
            claimed,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            format!(
                "signature has {} bytes, expected {expected_signature_bytes}",
                signature.len()
            ),
        );
    }
    let challenge = match decode_challenge(&signature[..CHALLENGE_BYTES]) {
        Ok(challenge) => challenge,
        Err(err) => {
            return reject(
                public_key, message, signature, mode, claimed, None, None, None, None, None, None,
                None, err,
            );
        }
    };
    let public_view = match decode_public_key(public_key) {
        Ok(public_view) => public_view,
        Err(err) => {
            return reject(
                public_key, message, signature, mode, claimed, None, None, None, None, None, None,
                None, err,
            );
        }
    };

    if mode == SignatureMode::CompletedResponse {
        let response_payload = &signature[CHALLENGE_BYTES..];
        let completed_responses = match decode_completed_responses(response_payload) {
            Ok(responses) => responses,
            Err(err) => {
                return reject(
                    public_key, message, signature, mode, claimed, None, None, None, None, None,
                    None, None, err,
                );
            }
        };
        let active_responses = active_responses_from_completed(&challenge, &completed_responses);
        let active_commitment_coords =
            reconstructed_active_commitment_coords(&public_view, &challenge, &active_responses);
        let active_commitments = pack_q_elements_padded(&active_commitment_coords);
        debug_assert_eq!(active_commitments.len(), ACTIVE_COMMITMENT_BYTES);
        let reconstructed_commitments = reconstructed_commitment_bytes_from_completed(
            &public_view,
            &challenge,
            &completed_responses,
        );
        debug_assert_eq!(
            reconstructed_commitments.len(),
            RECONSTRUCTED_COMMITMENT_BYTES
        );
        let frame = hash_frame_bytes(
            public_key,
            message,
            b"reconstructed_commitments",
            &reconstructed_commitments,
        );
        let frame_hash = sha256_hex(&frame);
        let recomputed = hash_frame_to_challenge_bytes(&frame);
        let recomputed_hex = hex_encode(&recomputed);
        if signature[..CHALLENGE_BYTES] != recomputed {
            return reject(
                public_key,
                message,
                signature,
                mode,
                claimed,
                Some(recomputed_hex),
                Some(response_payload.len()),
                Some(active_commitments.len()),
                Some(reconstructed_commitments.len()),
                None,
                Some(frame.len()),
                Some(frame_hash),
                "challenge mismatch".to_string(),
            );
        }
        return CompactVerifyReport {
            accepted: true,
            profile: PROFILE.to_string(),
            hash_domain: HASH_DOMAIN.to_string(),
            signature_mode: mode.name().to_string(),
            structural_only: false,
            public_key_bytes: public_key.len(),
            expected_public_key_bytes: PK_BYTES,
            message_bytes: message.len(),
            signature_bytes: signature.len(),
            expected_signature_bytes,
            challenge_bytes: CHALLENGE_BYTES,
            response_bytes: RESPONSE_BYTES,
            response_bound: RESPONSE_BOUND,
            completed_response_bytes: Some(response_payload.len()),
            active_commitment_bytes: Some(active_commitments.len()),
            reconstructed_commitment_bytes: Some(reconstructed_commitments.len()),
            explicit_commitment_bytes: None,
            hash_frame_bytes: Some(frame.len()),
            hash_frame_sha256: Some(frame_hash),
            claimed_challenge_hex: claimed,
            recomputed_challenge_hex: Some(recomputed_hex),
            reason: "ok-completed-response-implicit-commitment".to_string(),
        };
    }

    let responses = match decode_responses(&signature[CHALLENGE_BYTES..STRUCTURAL_SIGNATURE_BYTES])
    {
        Ok(responses) => responses,
        Err(err) => {
            return reject(
                public_key, message, signature, mode, claimed, None, None, None, None, None, None,
                None, err,
            );
        }
    };
    let active_commitment_coords =
        reconstructed_active_commitment_coords(&public_view, &challenge, &responses);
    let active_commitments = pack_q_elements_padded(&active_commitment_coords);
    debug_assert_eq!(active_commitments.len(), ACTIVE_COMMITMENT_BYTES);

    if mode == SignatureMode::StructuralOnly {
        let frame = hash_frame_bytes(
            public_key,
            message,
            b"reconstructed_active_commitments",
            &active_commitments,
        );
        let frame_hash = sha256_hex(&frame);
        let recomputed = hash_frame_to_challenge_bytes(&frame);
        return CompactVerifyReport {
            accepted: true,
            profile: PROFILE.to_string(),
            hash_domain: HASH_DOMAIN.to_string(),
            signature_mode: mode.name().to_string(),
            structural_only: true,
            public_key_bytes: public_key.len(),
            expected_public_key_bytes: PK_BYTES,
            message_bytes: message.len(),
            signature_bytes: signature.len(),
            expected_signature_bytes,
            challenge_bytes: CHALLENGE_BYTES,
            response_bytes: RESPONSE_BYTES,
            response_bound: RESPONSE_BOUND,
            completed_response_bytes: None,
            active_commitment_bytes: Some(active_commitments.len()),
            reconstructed_commitment_bytes: None,
            explicit_commitment_bytes: None,
            hash_frame_bytes: Some(frame.len()),
            hash_frame_sha256: Some(frame_hash),
            claimed_challenge_hex: claimed,
            recomputed_challenge_hex: Some(hex_encode(&recomputed)),
            reason: "ok-structural-compact-active-relation".to_string(),
        };
    }

    let explicit_commitment_bytes = &signature[STRUCTURAL_SIGNATURE_BYTES..];
    let explicit_commitment_coords = match decode_explicit_commitments(explicit_commitment_bytes) {
        Ok(coords) => coords,
        Err(err) => {
            return reject(
                public_key,
                message,
                signature,
                mode,
                claimed,
                None,
                None,
                Some(active_commitments.len()),
                None,
                Some(explicit_commitment_bytes.len()),
                None,
                None,
                err,
            );
        }
    };
    let explicit_active_commitments =
        explicit_active_commitment_coords(&explicit_commitment_coords, &challenge);
    if explicit_active_commitments != active_commitment_coords {
        return reject(
            public_key,
            message,
            signature,
            mode,
            claimed,
            None,
            None,
            Some(active_commitments.len()),
            None,
            Some(explicit_commitment_bytes.len()),
            None,
            None,
            "active commitment relation mismatch".to_string(),
        );
    }

    let frame = hash_frame_bytes(
        public_key,
        message,
        b"explicit_commitments",
        explicit_commitment_bytes,
    );
    let frame_hash = sha256_hex(&frame);
    let recomputed = hash_frame_to_challenge_bytes(&frame);
    let recomputed_hex = hex_encode(&recomputed);
    if signature[..CHALLENGE_BYTES] != recomputed {
        return reject(
            public_key,
            message,
            signature,
            mode,
            claimed,
            Some(recomputed_hex),
            None,
            Some(active_commitments.len()),
            None,
            Some(explicit_commitment_bytes.len()),
            Some(frame.len()),
            Some(frame_hash),
            "challenge mismatch".to_string(),
        );
    }
    CompactVerifyReport {
        accepted: true,
        profile: PROFILE.to_string(),
        hash_domain: HASH_DOMAIN.to_string(),
        signature_mode: mode.name().to_string(),
        structural_only: false,
        public_key_bytes: public_key.len(),
        expected_public_key_bytes: PK_BYTES,
        message_bytes: message.len(),
        signature_bytes: signature.len(),
        expected_signature_bytes,
        challenge_bytes: CHALLENGE_BYTES,
        response_bytes: RESPONSE_BYTES,
        response_bound: RESPONSE_BOUND,
        completed_response_bytes: None,
        active_commitment_bytes: Some(active_commitments.len()),
        reconstructed_commitment_bytes: None,
        explicit_commitment_bytes: Some(explicit_commitment_bytes.len()),
        hash_frame_bytes: Some(frame.len()),
        hash_frame_sha256: Some(frame_hash),
        claimed_challenge_hex: claimed,
        recomputed_challenge_hex: Some(recomputed_hex),
        reason: "ok-explicit-commitment".to_string(),
    }
}

fn print_help() {
    println!(
        "Check the q12289 compact signature byte format\n\
         \n\
         Options:\n\
           --public-key-hex HEX\n\
           --signature-hex HEX\n\
           --message-hex HEX\n\
           --message-text TEXT\n\
           --completed-response       (default)\n\
           --explicit-commitment\n\
           --structural-only\n\
           --json\n\
         \n\
         The default check validates the compact completed-response implicit\n\
         commitment signature:\n\
         17 B challenge || 728 B completed responses = 745 B.\n\
         --explicit-commitment keeps the older 3709 B signable baseline.\n\
         --structural-only validates only the 69 B active payload."
    );
}

fn parse_args() -> Result<Config, String> {
    let mut public_key = None;
    let mut signature = None;
    let mut message = None;
    let mut mode = SignatureMode::CompletedResponse;
    let mut json = false;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = |flag: &str| {
            args.next()
                .ok_or_else(|| format!("{flag} requires a value"))
        };
        match arg.as_str() {
            "--public-key-hex" | "--pk-hex" => public_key = Some(hex_decode(&value(&arg)?)?),
            "--signature-hex" | "--sig-hex" => signature = Some(hex_decode(&value(&arg)?)?),
            "--message-hex" | "--msg-hex" => {
                if message.is_some() {
                    return Err("provide only one message input".to_string());
                }
                message = Some(hex_decode(&value(&arg)?)?);
            }
            "--message-text" | "--msg-text" => {
                if message.is_some() {
                    return Err("provide only one message input".to_string());
                }
                message = Some(value(&arg)?.into_bytes());
            }
            "--completed-response" => mode = SignatureMode::CompletedResponse,
            "--explicit-commitment" => mode = SignatureMode::ExplicitCommitment,
            "--structural-only" => mode = SignatureMode::StructuralOnly,
            "--json" => json = true,
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    Ok(Config {
        public_key: public_key.ok_or_else(|| "missing --public-key-hex".to_string())?,
        message: message.unwrap_or_default(),
        signature: signature.ok_or_else(|| "missing --signature-hex".to_string())?,
        mode,
        json,
    })
}

fn main() {
    let cfg = match parse_args() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("{err}");
            eprintln!("use --help for usage");
            std::process::exit(2);
        }
    };
    let report = verify_compact_signature(&cfg.public_key, &cfg.message, &cfg.signature, cfg.mode);
    if cfg.json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else if report.accepted {
        println!("accepted: {}", report.reason);
    } else {
        println!("rejected: {}", report.reason);
    }
    if !report.accepted {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deterministic_public_key() -> Vec<u8> {
        let mut coords = Vec::with_capacity(VIEW_COORDS);
        for idx in 0..VIEW_COORDS {
            coords.push(((31 * idx + 7) as u16) % Q);
        }
        let mut public_key = vec![0_u8; PK_SEED_BYTES];
        public_key.extend_from_slice(&pack_q_elements_padded(&coords));
        public_key
    }

    fn zero_public_key() -> Vec<u8> {
        vec![0_u8; PK_BYTES]
    }

    fn deterministic_secret_free() -> [[i16; LOCAL_FREE]; GROUPS] {
        let mut secret = [[0_i16; LOCAL_FREE]; GROUPS];
        for (block, block_secret) in secret.iter_mut().enumerate() {
            for (coord, value) in block_secret.iter_mut().enumerate() {
                *value = ((7 * block + 3 * coord) % 7) as i16 - 3;
            }
        }
        secret
    }

    fn public_key_from_secret_free(secret: &[[i16; LOCAL_FREE]; GROUPS]) -> Vec<u8> {
        let mut coords = Vec::with_capacity(VIEW_COORDS);
        for block_secret in secret.iter() {
            coords.extend_from_slice(&systematic_pivot_from_free(*block_secret));
        }
        for block_secret in secret.iter() {
            for &value in block_secret.iter() {
                coords.push(mod_q(value as i64));
            }
        }
        let mut public_key = vec![0_u8; PK_SEED_BYTES];
        public_key.extend_from_slice(&pack_q_elements_padded(&coords));
        public_key
    }

    fn deterministic_challenge() -> CompactChallenge {
        let mut active_block = [0_usize; REPS];
        let mut sign = [1_i8; REPS];
        for rep in 0..REPS {
            active_block[rep] = (7 * rep + 3) % GROUPS;
            sign[rep] = if rep % 2 == 0 { 1 } else { -1 };
        }
        CompactChallenge { active_block, sign }
    }

    fn deterministic_responses() -> CompactResponses {
        let mut active_free = [[0_i16; LOCAL_FREE]; REPS];
        for rep in 0..REPS {
            for coord in 0..LOCAL_FREE {
                active_free[rep][coord] = ((5 * rep + 3 * coord) % 31) as i16 - 15;
            }
        }
        CompactResponses { active_free }
    }

    fn deterministic_structural_signature() -> Vec<u8> {
        let mut signature = Vec::with_capacity(STRUCTURAL_SIGNATURE_BYTES);
        signature.extend_from_slice(&encode_challenge(&deterministic_challenge()));
        signature.extend_from_slice(&encode_responses(&deterministic_responses()));
        signature
    }

    fn deterministic_masks() -> [[[i16; LOCAL_FREE]; GROUPS]; REPS] {
        let mut masks = [[[0_i16; LOCAL_FREE]; GROUPS]; REPS];
        for (rep, rep_masks) in masks.iter_mut().enumerate() {
            for (block, block_mask) in rep_masks.iter_mut().enumerate() {
                for (coord, value) in block_mask.iter_mut().enumerate() {
                    *value = ((11 * rep + 5 * block + 3 * coord) % 31) as i16 - 15;
                }
            }
        }
        masks
    }

    fn commitment_bytes_from_free_masks(masks: &[[[i16; LOCAL_FREE]; GROUPS]; REPS]) -> Vec<u8> {
        let mut coords = Vec::with_capacity(REPS * VIEW_COORDS);
        for rep_masks in masks.iter() {
            for block_mask in rep_masks.iter() {
                coords.extend_from_slice(&systematic_pivot_from_free(*block_mask));
            }
            for block_mask in rep_masks.iter() {
                for &value in block_mask.iter() {
                    coords.push(mod_q(value as i64));
                }
            }
        }
        let bytes = pack_q_elements_padded(&coords);
        assert_eq!(bytes.len(), EXPLICIT_COMMITMENT_BYTES);
        bytes
    }

    fn deterministic_explicit_signature(message: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let public_key = zero_public_key();
        let masks = deterministic_masks();
        let commitments = commitment_bytes_from_free_masks(&masks);
        let frame = hash_frame_bytes(&public_key, message, b"explicit_commitments", &commitments);
        let challenge_bytes = hash_frame_to_challenge_bytes(&frame);
        let challenge = decode_challenge(&challenge_bytes).unwrap();
        let mut responses = CompactResponses {
            active_free: [[0_i16; LOCAL_FREE]; REPS],
        };
        for rep in 0..REPS {
            responses.active_free[rep] = masks[rep][challenge.active_block[rep]];
        }
        let response_bytes = encode_responses(&responses);
        let mut signature = Vec::with_capacity(EXPLICIT_COMMITMENT_SIGNATURE_BYTES);
        signature.extend_from_slice(&challenge_bytes);
        signature.extend_from_slice(&response_bytes);
        signature.extend_from_slice(&commitments);
        assert_eq!(signature.len(), EXPLICIT_COMMITMENT_SIGNATURE_BYTES);
        (public_key, signature)
    }

    fn completed_responses_from_masks(
        masks: &[[[i16; LOCAL_FREE]; GROUPS]; REPS],
    ) -> CompletedResponses {
        CompletedResponses { free: *masks }
    }

    fn deterministic_completed_signature(message: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let secret = deterministic_secret_free();
        let public_key = public_key_from_secret_free(&secret);
        let masks = deterministic_masks();
        let commitments = commitment_bytes_from_free_masks(&masks);
        let frame = hash_frame_bytes(
            &public_key,
            message,
            b"reconstructed_commitments",
            &commitments,
        );
        let challenge_bytes = hash_frame_to_challenge_bytes(&frame);
        let challenge = decode_challenge(&challenge_bytes).unwrap();
        let mut completed_responses = completed_responses_from_masks(&masks);
        for rep in 0..REPS {
            let block = challenge.active_block[rep];
            for coord in 0..LOCAL_FREE {
                completed_responses.free[rep][block][coord] +=
                    challenge.sign[rep] as i16 * secret[block][coord];
                assert!(
                    (-RESPONSE_BOUND..=RESPONSE_BOUND)
                        .contains(&completed_responses.free[rep][block][coord])
                );
            }
        }
        let response_bytes = encode_completed_responses(&completed_responses);
        let mut signature = Vec::with_capacity(COMPLETED_RESPONSE_SIGNATURE_BYTES);
        signature.extend_from_slice(&challenge_bytes);
        signature.extend_from_slice(&response_bytes);
        assert_eq!(signature.len(), COMPLETED_RESPONSE_SIGNATURE_BYTES);
        (public_key, signature)
    }

    #[test]
    fn compact_sizes_match_small_target_payload() {
        assert_eq!(CHALLENGE_BYTES, 17);
        assert_eq!(RESPONSE_BYTES, 52);
        assert_eq!(STRUCTURAL_SIGNATURE_BYTES, 69);
        assert_eq!(COMPLETED_RESPONSE_BYTES, 728);
        assert_eq!(COMPLETED_RESPONSE_SIGNATURE_BYTES, 745);
        assert_eq!(ACTIVE_COMMITMENT_BYTES, 228);
        assert_eq!(EXPLICIT_COMMITMENT_BYTES, 3_640);
        assert_eq!(EXPLICIT_COMMITMENT_SIGNATURE_BYTES, 3_709);
    }

    #[test]
    fn compact_challenge_codec_round_trips_and_rejects_high_bits() {
        let challenge = deterministic_challenge();
        let bytes = encode_challenge(&challenge);
        assert_eq!(decode_challenge(&bytes).unwrap(), challenge);
        let mut bad = bytes;
        bad[CHALLENGE_BYTES - 1] |= 0xfc;
        assert!(decode_challenge(&bad).unwrap_err().contains("high bits"));
    }

    #[test]
    fn compact_response_codec_round_trips_and_rejects_noncanonical_bytes() {
        let responses = deterministic_responses();
        let bytes = encode_responses(&responses);
        assert_eq!(decode_responses(&bytes).unwrap(), responses);
        let mut bad = bytes;
        bad[13] = 97;
        assert!(
            decode_responses(&bad)
                .unwrap_err()
                .contains("non-canonical")
        );
    }

    #[test]
    fn compact_completed_response_codec_round_trips_and_rejects_noncanonical_values() {
        let responses = completed_responses_from_masks(&deterministic_masks());
        let bytes = encode_completed_responses(&responses);
        assert_eq!(decode_completed_responses(&bytes).unwrap(), responses);
        let mut bad_responses = responses;
        bad_responses.free[3][5][1] = RESPONSE_BOUND + 1;
        let mut bad = vec![0_u8; COMPLETED_RESPONSE_BYTES];
        let idx = 3 * COMPLETED_RESPONSE_COORDS_PER_REP + 5 * LOCAL_FREE + 1;
        write_bits_le(
            &mut bad,
            idx * COMPLETED_RESPONSE_BITS,
            COMPLETED_RESPONSE_BITS,
            (bad_responses.free[3][5][1] + RESPONSE_BOUND) as u32,
        );
        assert!(
            decode_completed_responses(&bad)
                .unwrap_err()
                .contains("non-canonical")
        );
    }

    #[test]
    fn compact_structural_check_accepts_deterministic_payload() {
        let public_key = deterministic_public_key();
        let signature = deterministic_structural_signature();
        let report = verify_compact_signature(
            &public_key,
            b"compact structural test",
            &signature,
            SignatureMode::StructuralOnly,
        );
        assert!(report.accepted, "{report:?}");
        assert_eq!(report.signature_bytes, 69);
        assert_eq!(report.active_commitment_bytes, Some(228));
        assert_eq!(report.explicit_commitment_bytes, None);
    }

    #[test]
    fn compact_default_check_rejects_structural_payload_without_commitments() {
        let public_key = deterministic_public_key();
        let signature = deterministic_structural_signature();
        let report = verify_compact_signature(
            &public_key,
            b"compact structural test",
            &signature,
            SignatureMode::CompletedResponse,
        );
        assert!(!report.accepted);
        assert_eq!(
            report.expected_signature_bytes,
            COMPLETED_RESPONSE_SIGNATURE_BYTES
        );
        assert!(report.reason.contains("expected 745"));
    }

    #[test]
    fn compact_completed_response_signature_accepts_by_default() {
        let message = b"compact completed response test";
        let (public_key, signature) = deterministic_completed_signature(message);
        let report = verify_compact_signature(
            &public_key,
            message,
            &signature,
            SignatureMode::CompletedResponse,
        );
        assert!(report.accepted, "{report:?}");
        assert_eq!(report.signature_bytes, COMPLETED_RESPONSE_SIGNATURE_BYTES);
        assert_eq!(
            report.completed_response_bytes,
            Some(COMPLETED_RESPONSE_BYTES)
        );
        assert_eq!(
            report.reconstructed_commitment_bytes,
            Some(RECONSTRUCTED_COMMITMENT_BYTES)
        );
        assert_eq!(report.explicit_commitment_bytes, None);
        assert_eq!(report.reason, "ok-completed-response-implicit-commitment");
    }

    #[test]
    fn compact_completed_response_signature_rejects_response_mutation() {
        let message = b"compact completed response test";
        let (public_key, mut signature) = deterministic_completed_signature(message);
        let mut responses = decode_completed_responses(&signature[CHALLENGE_BYTES..]).unwrap();
        responses.free[0][0][0] += 1;
        let response_bytes = encode_completed_responses(&responses);
        signature[CHALLENGE_BYTES..].copy_from_slice(&response_bytes);
        let report = verify_compact_signature(
            &public_key,
            message,
            &signature,
            SignatureMode::CompletedResponse,
        );
        assert!(!report.accepted);
        assert!(report.reason.contains("challenge mismatch"), "{report:?}");
    }

    #[test]
    fn compact_explicit_commitment_signature_accepts() {
        let message = b"compact explicit commitment test";
        let (public_key, signature) = deterministic_explicit_signature(message);
        let report = verify_compact_signature(
            &public_key,
            message,
            &signature,
            SignatureMode::ExplicitCommitment,
        );
        assert!(report.accepted, "{report:?}");
        assert_eq!(report.signature_bytes, EXPLICIT_COMMITMENT_SIGNATURE_BYTES);
        assert_eq!(
            report.active_commitment_bytes,
            Some(ACTIVE_COMMITMENT_BYTES)
        );
        assert_eq!(
            report.explicit_commitment_bytes,
            Some(EXPLICIT_COMMITMENT_BYTES)
        );
        assert_eq!(report.reason, "ok-explicit-commitment");
    }

    #[test]
    fn compact_explicit_commitment_signature_rejects_response_mutation() {
        let message = b"compact explicit commitment test";
        let (public_key, mut signature) = deterministic_explicit_signature(message);
        let byte = &mut signature[CHALLENGE_BYTES];
        *byte = if *byte == 0 { 1 } else { *byte - 1 };
        let report = verify_compact_signature(
            &public_key,
            message,
            &signature,
            SignatureMode::ExplicitCommitment,
        );
        assert!(!report.accepted);
        assert!(
            report
                .reason
                .contains("active commitment relation mismatch")
        );
    }

    #[test]
    fn compact_explicit_commitment_signature_rejects_inactive_commitment_mutation() {
        let message = b"compact explicit commitment test";
        let (public_key, mut signature) = deterministic_explicit_signature(message);
        let challenge = decode_challenge(&signature[..CHALLENGE_BYTES]).unwrap();
        let mut coords =
            decode_explicit_commitments(&signature[STRUCTURAL_SIGNATURE_BYTES..]).unwrap();
        let inactive_block = (challenge.active_block[0] + 1) % GROUPS;
        let coord_idx = inactive_block * LOCAL_PIVOT;
        coords[coord_idx] = (coords[coord_idx] + 1) % Q;
        let commitments = pack_q_elements_padded(&coords);
        signature[STRUCTURAL_SIGNATURE_BYTES..].copy_from_slice(&commitments);
        let report = verify_compact_signature(
            &public_key,
            message,
            &signature,
            SignatureMode::ExplicitCommitment,
        );
        assert!(!report.accepted);
        assert!(report.reason.contains("challenge mismatch"), "{report:?}");
    }

    #[test]
    fn compact_mutation_rejects_structurally_when_response_noncanonical() {
        let public_key = deterministic_public_key();
        let mut signature = deterministic_structural_signature();
        signature[CHALLENGE_BYTES + 4] = 255;
        let report = verify_compact_signature(
            &public_key,
            b"compact structural test",
            &signature,
            SignatureMode::StructuralOnly,
        );
        assert!(!report.accepted);
        assert!(report.reason.contains("non-canonical"));
    }
}
