use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const PROFILE: &str = "validated-q12289-n48-m1-32-single-carrier";
const HASH_DOMAIN: &str = "Descender.Signature.q12289.implicit_commitment.v1";
const Q: u16 = 12_289;
const GROUPS: usize = 16;
const LOCAL_PIVOT: usize = 3;
const LOCAL_FREE: usize = 2;
const VIEW_COORDS: usize = GROUPS * (LOCAL_PIVOT + LOCAL_FREE);
const PIVOT_COORDS: usize = GROUPS * LOCAL_PIVOT;
const FREE_COORDS: usize = GROUPS * LOCAL_FREE;
const REPS: usize = 7;
const CHALLENGE_BYTES: usize = 17;
const CHALLENGE_BITS_PER_REP: usize = 19;
const CHALLENGE_USED_BITS: usize = REPS * CHALLENGE_BITS_PER_REP;
const PK_SEED_BYTES: usize = 32;
const PK_PAYLOAD_BYTES: usize = 140;
const PK_BYTES: usize = PK_SEED_BYTES + PK_PAYLOAD_BYTES;
const FIELD_BITS: usize = 14;
const RESPONSE_BITS: usize = 9;
const RESPONSE_BYTES_PER_REP: usize = 36;
const RESPONSE_BOUND: i16 = 224;
const SIGNATURE_BYTES: usize = CHALLENGE_BYTES + REPS * RESPONSE_BYTES_PER_REP;
const RECONSTRUCTED_COMMITMENT_BYTES: usize = REPS * PK_PAYLOAD_BYTES;

const LOCAL_BLOCK: [[u16; LOCAL_FREE]; LOCAL_PIVOT] =
    [[10_849, 1_171], [1_597, 1_615], [6_102, 11_189]];

#[derive(Clone, Debug)]
struct Config {
    public_key: Vec<u8>,
    message: Vec<u8>,
    signature: Vec<u8>,
    json: bool,
    emit_conformance_vector: bool,
    verify_conformance_vector: Option<PathBuf>,
    write_conformance_vector: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Challenge {
    omitted: [usize; REPS],
    coeffs: [[i8; GROUPS]; REPS],
}

#[derive(Clone, Debug)]
struct PublicView {
    pivot: [[u16; LOCAL_PIVOT]; GROUPS],
    free: [[u16; LOCAL_FREE]; GROUPS],
}

#[derive(Clone, Debug)]
struct Responses {
    free: [[[i16; LOCAL_FREE]; GROUPS]; REPS],
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct VerifyReport {
    accepted: bool,
    profile: String,
    hash_domain: String,
    public_key_bytes: usize,
    expected_public_key_bytes: usize,
    message_bytes: usize,
    signature_bytes: usize,
    expected_signature_bytes: usize,
    response_bound: i16,
    claimed_challenge_hex: Option<String>,
    recomputed_challenge_hex: Option<String>,
    reconstructed_commitment_bytes: Option<usize>,
    hash_frame_bytes: Option<usize>,
    hash_frame_sha256: Option<String>,
    reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct ConformancePositiveVector {
    name: String,
    public_key_hex: String,
    message_hex: String,
    message_text: String,
    signature_hex: String,
    challenge_hex: String,
    response_hex: String,
    reconstructed_commitments_hex: String,
    hash_frame_hex: String,
    hash_frame_bytes: usize,
    hash_frame_sha256: String,
    report: VerifyReport,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct ConformanceNegativeVector {
    name: String,
    public_key_hex: String,
    message_hex: String,
    signature_hex: String,
    expected_reason: String,
    report: VerifyReport,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct ConformanceFrameNegativeVector {
    name: String,
    hash_frame_hex: String,
    expected_reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct ConformanceVectorBundle {
    profile: String,
    hash_domain: String,
    modulus: u16,
    public_key_bytes: usize,
    signature_bytes: usize,
    challenge_bytes: usize,
    challenge_used_bits: usize,
    response_bytes_per_repetition: usize,
    repetitions: usize,
    response_bound: i16,
    lean_challenge_codec_endpoint: String,
    lean_hash_frame_endpoint: String,
    lean_hash_frame_length_endpoint: String,
    positive: ConformancePositiveVector,
    negative: Vec<ConformanceNegativeVector>,
    frame_negative: Vec<ConformanceFrameNegativeVector>,
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

fn decode_challenge(bytes: &[u8]) -> Result<Challenge, String> {
    if bytes.len() != CHALLENGE_BYTES {
        return Err(format!(
            "challenge has {} bytes, expected {CHALLENGE_BYTES}",
            bytes.len()
        ));
    }
    if (bytes[CHALLENGE_BYTES - 1] & 0xe0) != 0 {
        return Err("challenge is not canonical: high bits are set".to_string());
    }

    let mut omitted = [0_usize; REPS];
    let mut coeffs = [[0_i8; GROUPS]; REPS];
    let mut bit = 0;
    for rep in 0..REPS {
        let omitted_block = read_bits_le(bytes, bit, 4) as usize;
        bit += 4;
        omitted[rep] = omitted_block;
        for block in 0..GROUPS {
            if block == omitted_block {
                coeffs[rep][block] = 0;
            } else {
                let sign = read_bits_le(bytes, bit, 1);
                bit += 1;
                coeffs[rep][block] = if sign == 0 { 1 } else { -1 };
            }
        }
    }
    debug_assert_eq!(bit, CHALLENGE_USED_BITS);
    Ok(Challenge { omitted, coeffs })
}

#[cfg(test)]
fn encode_challenge(challenge: &Challenge) -> [u8; CHALLENGE_BYTES] {
    let mut bytes = [0_u8; CHALLENGE_BYTES];
    let mut bit = 0;
    for rep in 0..REPS {
        write_bits_le(&mut bytes, bit, 4, challenge.omitted[rep] as u32);
        bit += 4;
        for block in 0..GROUPS {
            if block == challenge.omitted[rep] {
                continue;
            }
            let sign_bit = match challenge.coeffs[rep][block] {
                1 => 0,
                -1 => 1,
                other => panic!("invalid non-omitted challenge coefficient {other}"),
            };
            write_bits_le(&mut bytes, bit, 1, sign_bit);
            bit += 1;
        }
    }
    debug_assert_eq!(bit, CHALLENGE_USED_BITS);
    bytes
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

fn pack_q_elements(coords: &[u16]) -> Vec<u8> {
    let mut out = vec![0_u8; coords.len() * FIELD_BITS / 8];
    for (idx, &value) in coords.iter().enumerate() {
        assert!(value < Q);
        write_bits_le(&mut out, idx * FIELD_BITS, FIELD_BITS, value as u32);
    }
    out
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

fn decode_response_round(bytes: &[u8], rep: usize) -> Result<[[i16; LOCAL_FREE]; GROUPS], String> {
    if bytes.len() != RESPONSE_BYTES_PER_REP {
        return Err(format!(
            "response round {rep} has {} bytes, expected {RESPONSE_BYTES_PER_REP}",
            bytes.len()
        ));
    }
    let mut free = [[0_i16; LOCAL_FREE]; GROUPS];
    for idx in 0..FREE_COORDS {
        let raw = read_bits_le(bytes, idx * RESPONSE_BITS, RESPONSE_BITS) as i16;
        let value = if raw >= 256 { raw - 512 } else { raw };
        if (value as i32).abs() > RESPONSE_BOUND as i32 {
            return Err(format!(
                "response coordinate {idx} in round {rep} has |{value}| > {RESPONSE_BOUND}"
            ));
        }
        free[idx / LOCAL_FREE][idx % LOCAL_FREE] = value;
    }
    Ok(free)
}

fn decode_responses(bytes: &[u8]) -> Result<Responses, String> {
    if bytes.len() != REPS * RESPONSE_BYTES_PER_REP {
        return Err(format!(
            "response payload has {} bytes, expected {}",
            bytes.len(),
            REPS * RESPONSE_BYTES_PER_REP
        ));
    }
    let mut free = [[[0_i16; LOCAL_FREE]; GROUPS]; REPS];
    for rep in 0..REPS {
        let start = rep * RESPONSE_BYTES_PER_REP;
        free[rep] = decode_response_round(&bytes[start..start + RESPONSE_BYTES_PER_REP], rep)?;
    }
    Ok(Responses { free })
}

fn pack_response_round(free: &[[i16; LOCAL_FREE]; GROUPS]) -> [u8; RESPONSE_BYTES_PER_REP] {
    let mut out = [0_u8; RESPONSE_BYTES_PER_REP];
    for block in 0..GROUPS {
        for coord in 0..LOCAL_FREE {
            let value = free[block][coord];
            assert!((-256..=255).contains(&value));
            let raw = if value < 0 {
                (512 + value as i32) as u32
            } else {
                value as u32
            };
            write_bits_le(
                &mut out,
                (block * LOCAL_FREE + coord) * RESPONSE_BITS,
                RESPONSE_BITS,
                raw,
            );
        }
    }
    out
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
        other => panic!("invalid challenge coefficient {other}"),
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
        for coord in 0..LOCAL_FREE {
            acc += LOCAL_BLOCK[row][coord] as i64 * free_mod[coord] as i64;
        }
        pivot[row] = mod_q(-acc);
    }
    pivot
}

fn reconstructed_commitment_coords(
    public_key: &PublicView,
    challenge: &Challenge,
    responses: &Responses,
) -> Vec<u16> {
    let mut coords = Vec::with_capacity(REPS * VIEW_COORDS);
    for rep in 0..REPS {
        for block in 0..GROUPS {
            let az_pivot = systematic_pivot_from_free(responses.free[rep][block]);
            let coeff = challenge.coeffs[rep][block];
            for (coord, &value) in az_pivot.iter().enumerate() {
                coords.push(sub_mod(
                    value,
                    challenge_mul(coeff, public_key.pivot[block][coord]),
                ));
            }
        }
        for block in 0..GROUPS {
            let coeff = challenge.coeffs[rep][block];
            for coord in 0..LOCAL_FREE {
                let value = mod_q(responses.free[rep][block][coord] as i64);
                coords.push(sub_mod(
                    value,
                    challenge_mul(coeff, public_key.free[block][coord]),
                ));
            }
        }
    }
    coords
}

fn reconstructed_commitment_bytes(
    public_key: &PublicView,
    challenge: &Challenge,
    responses: &Responses,
) -> Vec<u8> {
    pack_q_elements(&reconstructed_commitment_coords(
        public_key, challenge, responses,
    ))
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
    reconstructed_commitments: &[u8],
) -> Vec<u8> {
    let mut frame = Vec::new();
    append_frame(&mut frame, b"domain", HASH_DOMAIN.as_bytes());
    append_frame(&mut frame, b"profile", PROFILE.as_bytes());
    append_u64(&mut frame, b"q", Q as u64);
    append_u64(&mut frame, b"groups", GROUPS as u64);
    append_u64(&mut frame, b"local_pivot", LOCAL_PIVOT as u64);
    append_u64(&mut frame, b"local_free", LOCAL_FREE as u64);
    append_u64(&mut frame, b"repetitions", REPS as u64);
    append_u64(&mut frame, b"response_bound", RESPONSE_BOUND as u64);
    append_frame(&mut frame, b"public_key", public_key);
    append_frame(&mut frame, b"message", message);
    append_frame(
        &mut frame,
        b"reconstructed_commitments",
        reconstructed_commitments,
    );
    frame
}

fn hash_frame_to_challenge_bytes(frame: &[u8]) -> [u8; CHALLENGE_BYTES] {
    let mut hasher = Sha256::new();
    hasher.update(frame);
    let digest = hasher.finalize();
    let mut out = [0_u8; CHALLENGE_BYTES];
    out.copy_from_slice(&digest[..CHALLENGE_BYTES]);
    out[CHALLENGE_BYTES - 1] &= 0x1f;
    out
}

#[cfg(test)]
fn hash_to_challenge_bytes(
    public_key: &[u8],
    message: &[u8],
    reconstructed_commitments: &[u8],
) -> [u8; CHALLENGE_BYTES] {
    hash_frame_to_challenge_bytes(&hash_frame_bytes(
        public_key,
        message,
        reconstructed_commitments,
    ))
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

fn reject(
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
    claimed: Option<String>,
    recomputed: Option<String>,
    reconstructed_bytes: Option<usize>,
    hash_frame_bytes: Option<usize>,
    hash_frame_sha256: Option<String>,
    reason: String,
) -> VerifyReport {
    VerifyReport {
        accepted: false,
        profile: PROFILE.to_string(),
        hash_domain: HASH_DOMAIN.to_string(),
        public_key_bytes: public_key.len(),
        expected_public_key_bytes: PK_BYTES,
        message_bytes: message.len(),
        signature_bytes: signature.len(),
        expected_signature_bytes: SIGNATURE_BYTES,
        response_bound: RESPONSE_BOUND,
        claimed_challenge_hex: claimed,
        recomputed_challenge_hex: recomputed,
        reconstructed_commitment_bytes: reconstructed_bytes,
        hash_frame_bytes,
        hash_frame_sha256,
        reason,
    }
}

fn verify_signature(public_key: &[u8], message: &[u8], signature: &[u8]) -> VerifyReport {
    let claimed =
        (signature.len() >= CHALLENGE_BYTES).then(|| hex_encode(&signature[..CHALLENGE_BYTES]));

    if public_key.len() != PK_BYTES {
        return reject(
            public_key,
            message,
            signature,
            claimed,
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
    if signature.len() != SIGNATURE_BYTES {
        return reject(
            public_key,
            message,
            signature,
            claimed,
            None,
            None,
            None,
            None,
            format!(
                "signature has {} bytes, expected {SIGNATURE_BYTES}",
                signature.len()
            ),
        );
    }

    let challenge = match decode_challenge(&signature[..CHALLENGE_BYTES]) {
        Ok(challenge) => challenge,
        Err(err) => {
            return reject(
                public_key, message, signature, claimed, None, None, None, None, err,
            );
        }
    };
    let public_view = match decode_public_key(public_key) {
        Ok(public_view) => public_view,
        Err(err) => {
            return reject(
                public_key, message, signature, claimed, None, None, None, None, err,
            );
        }
    };
    let responses = match decode_responses(&signature[CHALLENGE_BYTES..]) {
        Ok(responses) => responses,
        Err(err) => {
            return reject(
                public_key, message, signature, claimed, None, None, None, None, err,
            );
        }
    };
    let reconstructed = reconstructed_commitment_bytes(&public_view, &challenge, &responses);
    debug_assert_eq!(reconstructed.len(), RECONSTRUCTED_COMMITMENT_BYTES);
    let hash_frame = hash_frame_bytes(public_key, message, &reconstructed);
    let hash_frame_sha256 = sha256_hex(&hash_frame);
    let recomputed = hash_frame_to_challenge_bytes(&hash_frame);
    let recomputed_hex = hex_encode(&recomputed);
    let claimed_bytes = &signature[..CHALLENGE_BYTES];
    if claimed_bytes != recomputed {
        return reject(
            public_key,
            message,
            signature,
            claimed,
            Some(recomputed_hex),
            Some(reconstructed.len()),
            Some(hash_frame.len()),
            Some(hash_frame_sha256),
            "challenge mismatch".to_string(),
        );
    }

    VerifyReport {
        accepted: true,
        profile: PROFILE.to_string(),
        hash_domain: HASH_DOMAIN.to_string(),
        public_key_bytes: public_key.len(),
        expected_public_key_bytes: PK_BYTES,
        message_bytes: message.len(),
        signature_bytes: signature.len(),
        expected_signature_bytes: SIGNATURE_BYTES,
        response_bound: RESPONSE_BOUND,
        claimed_challenge_hex: claimed,
        recomputed_challenge_hex: Some(recomputed_hex),
        reconstructed_commitment_bytes: Some(reconstructed.len()),
        hash_frame_bytes: Some(hash_frame.len()),
        hash_frame_sha256: Some(hash_frame_sha256),
        reason: "ok".to_string(),
    }
}

#[cfg(test)]
fn deterministic_challenge() -> Challenge {
    let mut omitted = [0_usize; REPS];
    let mut coeffs = [[0_i8; GROUPS]; REPS];
    for rep in 0..REPS {
        omitted[rep] = (3 * rep + 1) % GROUPS;
        for block in 0..GROUPS {
            coeffs[rep][block] = if block == omitted[rep] {
                0
            } else if (rep + block) % 2 == 0 {
                1
            } else {
                -1
            };
        }
    }
    Challenge { omitted, coeffs }
}

fn deterministic_response_round(rep: usize) -> [[i16; LOCAL_FREE]; GROUPS] {
    let mut free = [[0_i16; LOCAL_FREE]; GROUPS];
    for block in 0..GROUPS {
        for coord in 0..LOCAL_FREE {
            free[block][coord] = ((17 * rep + 5 * block + 3 * coord) % 31) as i16 - 15;
        }
    }
    free
}

fn deterministic_responses() -> Responses {
    let mut free = [[[0_i16; LOCAL_FREE]; GROUPS]; REPS];
    for rep in 0..REPS {
        free[rep] = deterministic_response_round(rep);
    }
    Responses { free }
}

fn encode_responses(responses: &Responses) -> Vec<u8> {
    let mut out = Vec::with_capacity(REPS * RESPONSE_BYTES_PER_REP);
    for rep in 0..REPS {
        out.extend_from_slice(&pack_response_round(&responses.free[rep]));
    }
    out
}

#[cfg(test)]
fn deterministic_public_key() -> Vec<u8> {
    let mut coords = Vec::with_capacity(VIEW_COORDS);
    for idx in 0..VIEW_COORDS {
        coords.push(((31 * idx + 7) as u16) % Q);
    }
    let mut public_key = vec![0_u8; PK_SEED_BYTES];
    public_key.extend_from_slice(&pack_q_elements(&coords));
    public_key
}

fn zero_public_key_positive_vector() -> ConformancePositiveVector {
    let public_key = vec![0_u8; PK_BYTES];
    let public_view = decode_public_key(&public_key).unwrap();
    let message_text = "descender implicit verifier test";
    let message = message_text.as_bytes();
    let responses = deterministic_responses();
    let seed_challenge = decode_challenge(&[0_u8; CHALLENGE_BYTES]).unwrap();
    let reconstructed = reconstructed_commitment_bytes(&public_view, &seed_challenge, &responses);
    let hash_frame = hash_frame_bytes(&public_key, message, &reconstructed);
    let challenge_bytes = hash_frame_to_challenge_bytes(&hash_frame);
    let response_bytes = encode_responses(&responses);
    let mut signature = Vec::with_capacity(SIGNATURE_BYTES);
    signature.extend_from_slice(&challenge_bytes);
    signature.extend_from_slice(&response_bytes);
    let report = verify_signature(&public_key, message, &signature);
    assert!(report.accepted, "{report:?}");

    ConformancePositiveVector {
        name: "zero-public-key-valid-signature".to_string(),
        public_key_hex: hex_encode(&public_key),
        message_hex: hex_encode(message),
        message_text: message_text.to_string(),
        signature_hex: hex_encode(&signature),
        challenge_hex: hex_encode(&challenge_bytes),
        response_hex: hex_encode(&response_bytes),
        reconstructed_commitments_hex: hex_encode(&reconstructed),
        hash_frame_hex: hex_encode(&hash_frame),
        hash_frame_bytes: hash_frame.len(),
        hash_frame_sha256: sha256_hex(&hash_frame),
        report,
    }
}

fn negative_vector(
    name: &'static str,
    public_key: Vec<u8>,
    message: &[u8],
    signature: Vec<u8>,
) -> ConformanceNegativeVector {
    let report = verify_signature(&public_key, message, &signature);
    assert!(!report.accepted, "{name}: {report:?}");
    ConformanceNegativeVector {
        name: name.to_string(),
        public_key_hex: hex_encode(&public_key),
        message_hex: hex_encode(message),
        signature_hex: hex_encode(&signature),
        expected_reason: report.reason.clone(),
        report,
    }
}

fn conformance_vector_bundle() -> ConformanceVectorBundle {
    let positive = zero_public_key_positive_vector();
    let public_key = hex_decode(&positive.public_key_hex).unwrap();
    let message = hex_decode(&positive.message_hex).unwrap();
    let signature = hex_decode(&positive.signature_hex).unwrap();

    let mut bad_challenge_high_bits = signature.clone();
    bad_challenge_high_bits[CHALLENGE_BYTES - 1] |= 0x80;

    let mut bad_challenge_mismatch = signature.clone();
    bad_challenge_mismatch[0] ^= 0x01;

    let mut bad_response = signature.clone();
    bad_response[CHALLENGE_BYTES] ^= 0x01;

    let mut bad_response_out_of_range = signature.clone();
    let mut responses = deterministic_responses();
    responses.free[0][0][0] = RESPONSE_BOUND + 1;
    let bad_response_bytes = encode_responses(&responses);
    bad_response_out_of_range[CHALLENGE_BYTES..].copy_from_slice(&bad_response_bytes);

    let mut bad_public_key_q = public_key.clone();
    write_bits_le(
        &mut bad_public_key_q[PK_SEED_BYTES..],
        0,
        FIELD_BITS,
        Q as u32,
    );

    let mut short_signature = signature.clone();
    short_signature.pop();

    let mut short_public_key = public_key.clone();
    short_public_key.pop();

    let mut long_signature = signature.clone();
    long_signature.push(0);

    let mut negative_responses = deterministic_responses();
    negative_responses.free[0][0][0] = -(RESPONSE_BOUND + 1);
    let mut bad_negative_response_out_of_range = signature.clone();
    let bad_negative_response_bytes = encode_responses(&negative_responses);
    bad_negative_response_out_of_range[CHALLENGE_BYTES..]
        .copy_from_slice(&bad_negative_response_bytes);

    let mut modified_message = message.clone();
    modified_message[0] ^= 0x01;

    let mut modified_public_key = public_key.clone();
    modified_public_key[0] ^= 0x01;

    let negative = vec![
        negative_vector(
            "challenge-high-bits-set",
            public_key.clone(),
            &message,
            bad_challenge_high_bits,
        ),
        negative_vector(
            "challenge-mismatch",
            public_key.clone(),
            &message,
            bad_challenge_mismatch,
        ),
        negative_vector(
            "response-mutation-challenge-mismatch",
            public_key.clone(),
            &message,
            bad_response,
        ),
        negative_vector(
            "response-coordinate-out-of-range",
            public_key.clone(),
            &message,
            bad_response_out_of_range,
        ),
        negative_vector(
            "public-key-noncanonical-q-coordinate",
            bad_public_key_q,
            &message,
            signature.clone(),
        ),
        negative_vector(
            "public-key-too-short",
            short_public_key,
            &message,
            signature.clone(),
        ),
        negative_vector(
            "response-coordinate-negative-out-of-range",
            public_key.clone(),
            &message,
            bad_negative_response_out_of_range,
        ),
        negative_vector(
            "modified-message-unchanged-signature",
            public_key.clone(),
            &modified_message,
            signature.clone(),
        ),
        negative_vector(
            "modified-public-key-unchanged-signature",
            modified_public_key,
            &message,
            signature.clone(),
        ),
        negative_vector(
            "signature-too-short",
            public_key.clone(),
            &message,
            short_signature,
        ),
        negative_vector(
            "response-payload-length-mismatch",
            public_key.clone(),
            &message,
            long_signature,
        ),
    ];

    let positive_hash_frame = hex_decode(&positive.hash_frame_hex).unwrap();
    let positive_reconstructed = hex_decode(&positive.reconstructed_commitments_hex).unwrap();
    let reconstructed_header = frame_header_bytes(
        b"reconstructed_commitments",
        positive_reconstructed.len() as u64,
    );
    let reconstructed_offset = positive_hash_frame
        .windows(reconstructed_header.len())
        .position(|window| window == reconstructed_header)
        .map(|idx| idx + reconstructed_header.len())
        .expect("reconstructed_commitments field present");
    let reconstructed_len_offset = reconstructed_offset - 8;

    let mut short_reconstructed = positive_reconstructed.clone();
    short_reconstructed.pop();
    let mut short_frame = positive_hash_frame.clone();
    short_frame.truncate(reconstructed_offset);
    short_frame[reconstructed_len_offset..reconstructed_offset]
        .copy_from_slice(&(short_reconstructed.len() as u64).to_le_bytes());
    short_frame.extend_from_slice(&short_reconstructed);

    let mut noncanonical_reconstructed = positive_reconstructed.clone();
    noncanonical_reconstructed[0] = 0;
    noncanonical_reconstructed[1] &= 0xc0;
    write_bits_le(&mut noncanonical_reconstructed, 0, FIELD_BITS, Q as u32);
    let mut noncanonical_frame = positive_hash_frame.clone();
    noncanonical_frame.truncate(reconstructed_offset);
    noncanonical_frame.extend_from_slice(&noncanonical_reconstructed);

    let frame_negative = vec![
        ConformanceFrameNegativeVector {
            name: "reconstructed-commitments-length-short".to_string(),
            hash_frame_hex: hex_encode(&short_frame),
            expected_reason: format!(
                "reconstructed commitments field has {} bytes, expected {}",
                RECONSTRUCTED_COMMITMENT_BYTES - 1,
                RECONSTRUCTED_COMMITMENT_BYTES
            ),
        },
        ConformanceFrameNegativeVector {
            name: "reconstructed-commitment-noncanonical-q-coordinate".to_string(),
            hash_frame_hex: hex_encode(&noncanonical_frame),
            expected_reason: format!("non-canonical q-coordinate at index 0: {Q}"),
        },
    ];

    ConformanceVectorBundle {
        profile: PROFILE.to_string(),
        hash_domain: HASH_DOMAIN.to_string(),
        modulus: Q,
        public_key_bytes: PK_BYTES,
        signature_bytes: SIGNATURE_BYTES,
        challenge_bytes: CHALLENGE_BYTES,
        challenge_used_bits: CHALLENGE_USED_BITS,
        response_bytes_per_repetition: RESPONSE_BYTES_PER_REP,
        repetitions: REPS,
        response_bound: RESPONSE_BOUND,
        lean_challenge_codec_endpoint: "N32CanonicalChallengeByteCodec.productionBitLayout17"
            .to_string(),
        lean_hash_frame_endpoint:
            "N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame".to_string(),
        lean_hash_frame_length_endpoint:
            "N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame_length".to_string(),
        positive,
        negative,
        frame_negative,
    }
}

fn frame_header_bytes(label: &[u8], data_len: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(2 + label.len() + 8);
    out.extend_from_slice(&(label.len() as u16).to_le_bytes());
    out.extend_from_slice(label);
    out.extend_from_slice(&data_len.to_le_bytes());
    out
}

fn conformance_vector_json() -> String {
    serde_json::to_string_pretty(&conformance_vector_bundle()).unwrap()
}

fn write_conformance_bundle_file(path: &Path) -> Result<(), String> {
    let mut json = conformance_vector_json();
    json.push('\n');
    fs::write(path, json).map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn verify_conformance_bundle_file(path: &Path) -> Result<(), String> {
    let expected_bundle = conformance_vector_bundle();
    let actual_text =
        fs::read_to_string(path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let actual_bundle: ConformanceVectorBundle = serde_json::from_str(&actual_text)
        .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
    if actual_bundle != expected_bundle {
        return Err(format!(
            "conformance bundle drift detected in {}",
            path.display()
        ));
    }
    let expected_text = {
        let mut text = conformance_vector_json();
        text.push('\n');
        text
    };
    if actual_text != expected_text {
        return Err(format!(
            "conformance bundle formatting drift detected in {}",
            path.display()
        ));
    }
    independent_verify_bundle(&actual_bundle)
}

fn independent_read_fixed_le(bytes: &[u8], bit_width: usize, count: usize) -> Result<Vec<u16>, String> {
    let expected_len = count * bit_width / 8;
    if count * bit_width % 8 != 0 {
        return Err("independent fixed-width decode requires byte alignment".to_string());
    }
    if bytes.len() != expected_len {
        return Err(format!(
            "field payload has {} bytes, expected {expected_len}",
            bytes.len()
        ));
    }
    let mask = (1_u32 << bit_width) - 1;
    let mut out = Vec::with_capacity(count);
    let mut acc = 0_u32;
    let mut acc_bits = 0_usize;
    let mut cursor = 0;
    for _ in 0..count {
        while acc_bits < bit_width {
            let byte = bytes
                .get(cursor)
                .ok_or_else(|| "unexpected end of packed field payload".to_string())?;
            acc |= (*byte as u32) << acc_bits;
            acc_bits += 8;
            cursor += 1;
        }
        out.push((acc & mask) as u16);
        acc >>= bit_width;
        acc_bits -= bit_width;
    }
    if cursor != bytes.len() || acc_bits != 0 || acc != 0 {
        return Err("packed field payload has unexpected trailing bits".to_string());
    }
    Ok(out)
}

fn independent_pack_fixed_le(values: &[u16], bit_width: usize) -> Vec<u8> {
    let total_bits = values.len() * bit_width;
    let mut out = Vec::with_capacity(total_bits / 8);
    let mut acc = 0_u32;
    let mut acc_bits = 0_usize;
    for &value in values {
        acc |= (value as u32) << acc_bits;
        acc_bits += bit_width;
        while acc_bits >= 8 {
            out.push((acc & 0xff) as u8);
            acc >>= 8;
            acc_bits -= 8;
        }
    }
    if acc_bits != 0 {
        out.push(acc as u8);
    }
    out
}

fn independent_decode_challenge_bytes(bytes: &[u8]) -> Result<Challenge, String> {
    if bytes.len() != CHALLENGE_BYTES {
        return Err(format!(
            "challenge has {} bytes, expected {CHALLENGE_BYTES}",
            bytes.len()
        ));
    }
    if bytes[CHALLENGE_BYTES - 1] >> 5 != 0 {
        return Err("challenge is not canonical: high bits are set".to_string());
    }
    let mut bits = Vec::with_capacity(CHALLENGE_USED_BITS);
    for &byte in bytes {
        for bit in 0..8 {
            bits.push((byte >> bit) & 1);
        }
    }
    let mut offset = 0;
    let mut omitted = [0_usize; REPS];
    let mut coeffs = [[0_i8; GROUPS]; REPS];
    for rep in 0..REPS {
        let mut omitted_block = 0_usize;
        for shift in 0..4 {
            omitted_block |= (bits[offset + shift] as usize) << shift;
        }
        offset += 4;
        omitted[rep] = omitted_block;
        for block in 0..GROUPS {
            if block == omitted_block {
                coeffs[rep][block] = 0;
            } else {
                coeffs[rep][block] = if bits[offset] == 0 { 1 } else { -1 };
                offset += 1;
            }
        }
    }
    Ok(Challenge { omitted, coeffs })
}

fn independent_decode_public_view(bytes: &[u8]) -> Result<PublicView, String> {
    if bytes.len() != PK_BYTES {
        return Err(format!(
            "public key has {} bytes, expected {PK_BYTES}",
            bytes.len()
        ));
    }
    let coords = independent_read_fixed_le(&bytes[PK_SEED_BYTES..], FIELD_BITS, VIEW_COORDS)?;
    let mut pivot = [[0_u16; LOCAL_PIVOT]; GROUPS];
    let mut free = [[0_u16; LOCAL_FREE]; GROUPS];
    for (idx, &value) in coords.iter().enumerate() {
        if value >= Q {
            return Err(format!("non-canonical q-coordinate at index {idx}: {value}"));
        }
        if idx < PIVOT_COORDS {
            pivot[idx / LOCAL_PIVOT][idx % LOCAL_PIVOT] = value;
        } else {
            let free_idx = idx - PIVOT_COORDS;
            free[free_idx / LOCAL_FREE][free_idx % LOCAL_FREE] = value;
        }
    }
    Ok(PublicView { pivot, free })
}

fn independent_decode_responses(bytes: &[u8]) -> Result<Responses, String> {
    if bytes.len() != REPS * RESPONSE_BYTES_PER_REP {
        return Err(format!(
            "response payload has {} bytes, expected {}",
            bytes.len(),
            REPS * RESPONSE_BYTES_PER_REP
        ));
    }
    let values = independent_read_fixed_le(bytes, RESPONSE_BITS, REPS * FREE_COORDS)?;
    let mut free = [[[0_i16; LOCAL_FREE]; GROUPS]; REPS];
    for (idx, raw) in values.into_iter().enumerate() {
        let value = if raw >= 256 {
            raw as i16 - 512
        } else {
            raw as i16
        };
        if (value as i32).abs() > RESPONSE_BOUND as i32 {
            let round = idx / FREE_COORDS;
            let coord = idx % FREE_COORDS;
            return Err(format!(
                "response coordinate {coord} in round {round} has |{value}| > {RESPONSE_BOUND}"
            ));
        }
        let rep = idx / FREE_COORDS;
        let local = idx % FREE_COORDS;
        free[rep][local / LOCAL_FREE][local % LOCAL_FREE] = value;
    }
    Ok(Responses { free })
}

fn independent_reconstructed_commitment_bytes(
    public_view: &PublicView,
    challenge: &Challenge,
    responses: &Responses,
) -> Vec<u8> {
    let mut coords = Vec::with_capacity(REPS * VIEW_COORDS);
    for rep in 0..REPS {
        for block in 0..GROUPS {
            let free = responses.free[rep][block];
            for row in 0..LOCAL_PIVOT {
                let mut acc = 0_i64;
                for col in 0..LOCAL_FREE {
                    acc += LOCAL_BLOCK[row][col] as i64 * mod_q(free[col] as i64) as i64;
                }
                let az = mod_q(-acc);
                let pk = public_view.pivot[block][row];
                let challenged = match challenge.coeffs[rep][block] {
                    -1 => negate_mod(pk),
                    0 => 0,
                    1 => pk,
                    other => panic!("invalid challenge coefficient {other}"),
                };
                coords.push(mod_q(az as i64 - challenged as i64));
            }
        }
        for block in 0..GROUPS {
            for coord in 0..LOCAL_FREE {
                let z = mod_q(responses.free[rep][block][coord] as i64);
                let pk = public_view.free[block][coord];
                let challenged = match challenge.coeffs[rep][block] {
                    -1 => negate_mod(pk),
                    0 => 0,
                    1 => pk,
                    other => panic!("invalid challenge coefficient {other}"),
                };
                coords.push(mod_q(z as i64 - challenged as i64));
            }
        }
    }
    independent_pack_fixed_le(&coords, FIELD_BITS)
}

fn parse_frame_fields(frame: &[u8]) -> Result<Vec<(String, Vec<u8>)>, String> {
    let mut cursor = 0_usize;
    let mut out = Vec::new();
    while cursor < frame.len() {
        if cursor + 2 > frame.len() {
            return Err("truncated frame label length".to_string());
        }
        let label_len = u16::from_le_bytes([frame[cursor], frame[cursor + 1]]) as usize;
        cursor += 2;
        if cursor + label_len > frame.len() {
            return Err("truncated frame label".to_string());
        }
        let label = std::str::from_utf8(&frame[cursor..cursor + label_len])
            .map_err(|err| format!("frame label is not utf-8: {err}"))?
            .to_string();
        cursor += label_len;
        if cursor + 8 > frame.len() {
            return Err(format!("truncated frame length for field {label}"));
        }
        let data_len = u64::from_le_bytes(
            frame[cursor..cursor + 8]
                .try_into()
                .expect("slice has fixed length"),
        ) as usize;
        cursor += 8;
        if cursor + data_len > frame.len() {
            return Err(format!("truncated frame data for field {label}"));
        }
        out.push((label, frame[cursor..cursor + data_len].to_vec()));
        cursor += data_len;
    }
    Ok(out)
}

fn independent_verify_positive_vector(bundle: &ConformanceVectorBundle) -> Result<(), String> {
    let positive = &bundle.positive;
    let public_key = hex_decode(&positive.public_key_hex)?;
    let message = hex_decode(&positive.message_hex)?;
    let signature = hex_decode(&positive.signature_hex)?;
    let challenge_bytes = hex_decode(&positive.challenge_hex)?;
    let response_bytes = hex_decode(&positive.response_hex)?;
    let reconstructed_bytes = hex_decode(&positive.reconstructed_commitments_hex)?;
    let frame_bytes = hex_decode(&positive.hash_frame_hex)?;

    let challenge = independent_decode_challenge_bytes(&challenge_bytes)?;
    let public_view = independent_decode_public_view(&public_key)?;
    let responses = independent_decode_responses(&response_bytes)?;
    let recomputed_reconstructed =
        independent_reconstructed_commitment_bytes(&public_view, &challenge, &responses);
    if recomputed_reconstructed != reconstructed_bytes {
        return Err("independent reconstructed commitments mismatch".to_string());
    }
    if signature[..CHALLENGE_BYTES] != challenge_bytes
        || signature[CHALLENGE_BYTES..] != response_bytes
    {
        return Err("positive vector signature split does not match challenge/response fields".to_string());
    }

    let fields = parse_frame_fields(&frame_bytes)?;
    let expected_labels = [
        "domain",
        "profile",
        "q",
        "groups",
        "local_pivot",
        "local_free",
        "repetitions",
        "response_bound",
        "public_key",
        "message",
        "reconstructed_commitments",
    ];
    let actual_labels: Vec<&str> = fields.iter().map(|(label, _)| label.as_str()).collect();
    if actual_labels != expected_labels {
        return Err(format!("hash frame field order drifted: {actual_labels:?}"));
    }
    let field = |name: &str| -> &[u8] {
        fields
            .iter()
            .find(|(label, _)| label == name)
            .map(|(_, data)| data.as_slice())
            .expect("required frame field present")
    };
    if field("domain") != HASH_DOMAIN.as_bytes() {
        return Err("hash frame domain drift detected".to_string());
    }
    if field("profile") != PROFILE.as_bytes() {
        return Err("hash frame profile drift detected".to_string());
    }
    for (name, expected) in [
        ("q", Q as u64),
        ("groups", GROUPS as u64),
        ("local_pivot", LOCAL_PIVOT as u64),
        ("local_free", LOCAL_FREE as u64),
        ("repetitions", REPS as u64),
        ("response_bound", RESPONSE_BOUND as u64),
    ] {
        let data = field(name);
        if data.len() != 8 {
            return Err(format!("frame scalar field {name} has {} bytes, expected 8", data.len()));
        }
        let actual = u64::from_le_bytes(data.try_into().expect("u64 field"));
        if actual != expected {
            return Err(format!("frame scalar field {name} drifted: {actual} != {expected}"));
        }
    }
    if field("public_key") != public_key.as_slice() {
        return Err("hash frame public key field drift detected".to_string());
    }
    if field("message") != message.as_slice() {
        return Err("hash frame message field drift detected".to_string());
    }
    if field("reconstructed_commitments") != reconstructed_bytes.as_slice() {
        return Err("hash frame reconstructed commitments field drift detected".to_string());
    }
    if reconstructed_bytes.len() != RECONSTRUCTED_COMMITMENT_BYTES {
        return Err(format!(
            "reconstructed commitments field has {} bytes, expected {}",
            reconstructed_bytes.len(),
            RECONSTRUCTED_COMMITMENT_BYTES
        ));
    }
    independent_decode_public_view(field("public_key"))?;
    for (idx, value) in independent_read_fixed_le(
        field("reconstructed_commitments"),
        FIELD_BITS,
        REPS * VIEW_COORDS,
    )?
    .into_iter()
    .enumerate()
    {
        if value >= Q {
            return Err(format!("non-canonical q-coordinate at index {idx}: {value}"));
        }
    }

    if frame_bytes.len() != positive.hash_frame_bytes {
        return Err(format!(
            "hash frame has {} bytes, expected {}",
            frame_bytes.len(),
            positive.hash_frame_bytes
        ));
    }
    let digest = sha256_hex(&frame_bytes);
    if digest != positive.hash_frame_sha256 {
        return Err("hash frame digest drift detected".to_string());
    }
    let recomputed_challenge = hex_encode(&hash_frame_to_challenge_bytes(&frame_bytes));
    if recomputed_challenge != positive.challenge_hex {
        return Err("challenge drift detected".to_string());
    }
    Ok(())
}

fn independent_verify_frame_negative_vector(
    vector: &ConformanceFrameNegativeVector,
) -> Result<(), String> {
    let frame = hex_decode(&vector.hash_frame_hex)?;
    let fields = parse_frame_fields(&frame)?;
    let reconstructed = fields
        .iter()
        .find(|(label, _)| label == "reconstructed_commitments")
        .map(|(_, data)| data.as_slice())
        .ok_or_else(|| "missing reconstructed_commitments field".to_string())?;
    let actual_reason = if reconstructed.len() != RECONSTRUCTED_COMMITMENT_BYTES {
        format!(
            "reconstructed commitments field has {} bytes, expected {}",
            reconstructed.len(),
            RECONSTRUCTED_COMMITMENT_BYTES
        )
    } else {
        let coords = independent_read_fixed_le(reconstructed, FIELD_BITS, REPS * VIEW_COORDS)?;
        match coords.into_iter().enumerate().find(|(_, value)| *value >= Q) {
            Some((idx, value)) => format!("non-canonical q-coordinate at index {idx}: {value}"),
            None => return Err(format!("frame negative vector {} unexpectedly accepted", vector.name)),
        }
    };
    if actual_reason != vector.expected_reason {
        return Err(format!(
            "frame negative vector {} reason drifted: {} != {}",
            vector.name, actual_reason, vector.expected_reason
        ));
    }
    Ok(())
}

fn independent_verify_bundle(bundle: &ConformanceVectorBundle) -> Result<(), String> {
    independent_verify_positive_vector(bundle)?;
    for negative in &bundle.negative {
        let report = verify_signature(
            &hex_decode(&negative.public_key_hex)?,
            &hex_decode(&negative.message_hex)?,
            &hex_decode(&negative.signature_hex)?,
        );
        if report.accepted {
            return Err(format!("negative vector {} unexpectedly accepted", negative.name));
        }
        if report.reason != negative.expected_reason {
            return Err(format!(
                "negative vector {} reason drifted: {} != {}",
                negative.name, report.reason, negative.expected_reason
            ));
        }
    }
    for negative in &bundle.frame_negative {
        independent_verify_frame_negative_vector(negative)?;
    }
    Ok(())
}

fn print_help() {
    println!(
        "Verify the q12289 implicit-commitment signature byte relation\n\
         \n\
         Options:\n\
           --public-key-hex HEX\n\
           --signature-hex HEX\n\
           --message-hex HEX\n\
           --message-text TEXT\n\
           --emit-conformance-vector\n\
           --verify-conformance-vector PATH\n\
           --write-conformance-vector PATH\n\
           --json"
    );
}

fn parse_args() -> Result<Config, String> {
    let mut public_key = None;
    let mut signature = None;
    let mut message = None;
    let mut json = false;
    let mut emit_conformance_vector = false;
    let mut verify_conformance_vector = None;
    let mut write_conformance_vector = None;
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
            "--json" => json = true,
            "--emit-conformance-vector" => emit_conformance_vector = true,
            "--verify-conformance-vector" => {
                verify_conformance_vector = Some(PathBuf::from(value(&arg)?));
            }
            "--write-conformance-vector" => {
                write_conformance_vector = Some(PathBuf::from(value(&arg)?));
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    if emit_conformance_vector
        && (verify_conformance_vector.is_some() || write_conformance_vector.is_some())
    {
        return Err(
            "--emit-conformance-vector cannot be combined with conformance file options"
                .to_string(),
        );
    }
    Ok(Config {
        public_key: if emit_conformance_vector
            || verify_conformance_vector.is_some()
            || write_conformance_vector.is_some()
        {
            public_key.unwrap_or_default()
        } else {
            public_key.ok_or_else(|| "missing --public-key-hex".to_string())?
        },
        message: message.unwrap_or_default(),
        signature: if emit_conformance_vector
            || verify_conformance_vector.is_some()
            || write_conformance_vector.is_some()
        {
            signature.unwrap_or_default()
        } else {
            signature.ok_or_else(|| "missing --signature-hex".to_string())?
        },
        json,
        emit_conformance_vector,
        verify_conformance_vector,
        write_conformance_vector,
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
    if cfg.emit_conformance_vector {
        println!("{}", conformance_vector_json());
        return;
    }
    if let Some(path) = cfg.write_conformance_vector.as_deref() {
        if let Err(err) = write_conformance_bundle_file(path) {
            eprintln!("{err}");
            std::process::exit(1);
        }
        println!("wrote {}", path.display());
        return;
    }
    if let Some(path) = cfg.verify_conformance_vector.as_deref() {
        if let Err(err) = verify_conformance_bundle_file(path) {
            eprintln!("{err}");
            std::process::exit(1);
        }
        println!("conformance bundle verified: {}", path.display());
        return;
    }
    let report = verify_signature(&cfg.public_key, &cfg.message, &cfg.signature);
    if cfg.json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else if report.accepted {
        println!("accepted");
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

    fn bundle_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../research/cryptanalysis/review_readiness_20260519/q12289_conformance_vectors.json")
    }

    #[test]
    fn challenge_codec_round_trips_and_rejects_high_bits() {
        let challenge = deterministic_challenge();
        let bytes = encode_challenge(&challenge);
        assert_eq!(decode_challenge(&bytes).unwrap(), challenge);

        let mut bad = bytes;
        bad[CHALLENGE_BYTES - 1] |= 0x80;
        assert!(decode_challenge(&bad).is_err());
    }

    #[test]
    fn response_codec_round_trips_and_enforces_bound() {
        let round = deterministic_response_round(2);
        let bytes = pack_response_round(&round);
        assert_eq!(decode_response_round(&bytes, 2).unwrap(), round);

        let mut out_of_range = round;
        out_of_range[0][0] = RESPONSE_BOUND + 1;
        let bad = pack_response_round(&out_of_range);
        assert!(decode_response_round(&bad, 0).is_err());

        let mut negative_out_of_range = round;
        negative_out_of_range[0][0] = -(RESPONSE_BOUND + 1);
        let bad = pack_response_round(&negative_out_of_range);
        assert!(decode_response_round(&bad, 0).is_err());
    }

    #[test]
    fn q_codec_round_trips_and_rejects_noncanonical_coordinate() {
        let coords: Vec<u16> = (0..VIEW_COORDS)
            .map(|idx| ((31 * idx + 7) as u16) % Q)
            .collect();
        let bytes = pack_q_elements(&coords);
        assert_eq!(unpack_q_elements(&bytes, VIEW_COORDS).unwrap(), coords);

        let mut bad = vec![0_u8; PK_PAYLOAD_BYTES];
        write_bits_le(&mut bad, 0, FIELD_BITS, Q as u32);
        assert!(unpack_q_elements(&bad, VIEW_COORDS).is_err());
    }

    #[test]
    fn reconstructed_commitment_bytes_decode_to_coords() {
        let public_key = deterministic_public_key();
        let public_view = decode_public_key(&public_key).unwrap();
        let challenge = deterministic_challenge();
        let responses = deterministic_responses();
        let coords = reconstructed_commitment_coords(&public_view, &challenge, &responses);
        let bytes = reconstructed_commitment_bytes(&public_view, &challenge, &responses);

        assert_eq!(bytes.len(), RECONSTRUCTED_COMMITMENT_BYTES);
        assert_eq!(
            unpack_q_elements(&bytes, REPS * VIEW_COORDS).unwrap(),
            coords
        );
    }

    #[test]
    fn hash_frame_is_pinned() {
        let public_key = vec![0_u8; PK_BYTES];
        let public_view = decode_public_key(&public_key).unwrap();
        let message = b"descender implicit verifier test";
        let responses = deterministic_responses();
        let seed_challenge = decode_challenge(&[0_u8; CHALLENGE_BYTES]).unwrap();
        let reconstructed =
            reconstructed_commitment_bytes(&public_view, &seed_challenge, &responses);
        let frame = hash_frame_bytes(&public_key, message, &reconstructed);

        assert_eq!(frame.len(), 1_540);
        assert_eq!(
            sha256_hex(&frame),
            "500b7e675f1970875c294ccb0d00eee386910b1a37c8ecf5b78d97e0af51993c"
        );
        assert_eq!(
            hex_encode(&hash_frame_to_challenge_bytes(&frame)),
            "500b7e675f1970875c294ccb0d00eee306"
        );
    }

    #[test]
    fn zero_public_key_valid_signature_accepts() {
        let public_key = vec![0_u8; PK_BYTES];
        let public_view = decode_public_key(&public_key).unwrap();
        let message = b"descender implicit verifier test";
        let responses = deterministic_responses();

        let seed_challenge = decode_challenge(&[0_u8; CHALLENGE_BYTES]).unwrap();
        let reconstructed =
            reconstructed_commitment_bytes(&public_view, &seed_challenge, &responses);
        let challenge_bytes = hash_to_challenge_bytes(&public_key, message, &reconstructed);
        assert_eq!(
            hex_encode(&challenge_bytes),
            "500b7e675f1970875c294ccb0d00eee306"
        );

        let mut signature = Vec::with_capacity(SIGNATURE_BYTES);
        signature.extend_from_slice(&challenge_bytes);
        signature.extend_from_slice(&encode_responses(&responses));

        let report = verify_signature(&public_key, message, &signature);
        assert!(report.accepted, "{report:?}");
        assert_eq!(
            report.reconstructed_commitment_bytes,
            Some(RECONSTRUCTED_COMMITMENT_BYTES)
        );
        assert_eq!(report.hash_frame_bytes, Some(1_540));
        assert_eq!(
            report.hash_frame_sha256.as_deref(),
            Some("500b7e675f1970875c294ccb0d00eee386910b1a37c8ecf5b78d97e0af51993c")
        );
    }

    #[test]
    fn challenge_or_response_mutation_rejects() {
        let public_key = vec![0_u8; PK_BYTES];
        let public_view = decode_public_key(&public_key).unwrap();
        let message = b"descender implicit verifier test";
        let responses = deterministic_responses();
        let seed_challenge = decode_challenge(&[0_u8; CHALLENGE_BYTES]).unwrap();
        let reconstructed =
            reconstructed_commitment_bytes(&public_view, &seed_challenge, &responses);
        let challenge_bytes = hash_to_challenge_bytes(&public_key, message, &reconstructed);

        let mut signature = Vec::with_capacity(SIGNATURE_BYTES);
        signature.extend_from_slice(&challenge_bytes);
        signature.extend_from_slice(&encode_responses(&responses));

        let mut bad_challenge = signature.clone();
        bad_challenge[0] ^= 0x01;
        let report = verify_signature(&public_key, message, &bad_challenge);
        assert!(!report.accepted);

        let mut bad_response = signature;
        bad_response[CHALLENGE_BYTES] ^= 0x01;
        let report = verify_signature(&public_key, message, &bad_response);
        assert!(!report.accepted);
    }

    #[test]
    fn conformance_vector_bundle_has_expected_shape() {
        let bundle = conformance_vector_bundle();
        assert_eq!(bundle.positive.hash_frame_bytes, 1_540);
        assert_eq!(
            bundle.positive.hash_frame_sha256,
            "500b7e675f1970875c294ccb0d00eee386910b1a37c8ecf5b78d97e0af51993c"
        );
        assert_eq!(bundle.negative.len(), 11);
        assert_eq!(bundle.frame_negative.len(), 2);
        assert!(bundle.negative.iter().all(|case| !case.report.accepted));
    }

    #[test]
    fn independent_cross_check_accepts_generated_bundle() {
        let bundle = conformance_vector_bundle();
        independent_verify_bundle(&bundle).unwrap();
    }

    #[test]
    fn checked_in_conformance_bundle_matches_generator() {
        verify_conformance_bundle_file(&bundle_path()).unwrap();
    }
}
