use serde::Serialize;
use sha2::{Digest, Sha256};
use std::env;

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

#[derive(Clone, Debug, Serialize)]
struct VerifyReport {
    accepted: bool,
    profile: &'static str,
    hash_domain: &'static str,
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

#[derive(Clone, Debug, Serialize)]
struct ConformancePositiveVector {
    name: &'static str,
    public_key_hex: String,
    message_hex: String,
    message_text: &'static str,
    signature_hex: String,
    challenge_hex: String,
    response_hex: String,
    reconstructed_commitments_hex: String,
    hash_frame_hex: String,
    hash_frame_bytes: usize,
    hash_frame_sha256: String,
    report: VerifyReport,
}

#[derive(Clone, Debug, Serialize)]
struct ConformanceNegativeVector {
    name: &'static str,
    public_key_hex: String,
    message_hex: String,
    signature_hex: String,
    expected_reason: String,
    report: VerifyReport,
}

#[derive(Clone, Debug, Serialize)]
struct ConformanceVectorBundle {
    profile: &'static str,
    hash_domain: &'static str,
    modulus: u16,
    public_key_bytes: usize,
    signature_bytes: usize,
    challenge_bytes: usize,
    challenge_used_bits: usize,
    response_bytes_per_repetition: usize,
    repetitions: usize,
    response_bound: i16,
    lean_challenge_codec_endpoint: &'static str,
    lean_hash_frame_endpoint: &'static str,
    lean_hash_frame_length_endpoint: &'static str,
    positive: ConformancePositiveVector,
    negative: Vec<ConformanceNegativeVector>,
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
        profile: PROFILE,
        hash_domain: HASH_DOMAIN,
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
        profile: PROFILE,
        hash_domain: HASH_DOMAIN,
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
        name: "zero-public-key-valid-signature",
        public_key_hex: hex_encode(&public_key),
        message_hex: hex_encode(message),
        message_text,
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
        name,
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
    let mut short_public_key = public_key.clone();
    short_public_key.pop();
    let mut long_public_key = public_key.clone();
    long_public_key.push(0);

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
    let mut long_signature = signature.clone();
    long_signature.push(0);

    let negative = vec![
        negative_vector(
            "public-key-too-short",
            short_public_key,
            &message,
            signature.clone(),
        ),
        negative_vector(
            "public-key-too-long",
            long_public_key,
            &message,
            signature.clone(),
        ),
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
        negative_vector("signature-too-short", public_key, &message, short_signature),
        negative_vector(
            "signature-too-long",
            hex_decode(&positive.public_key_hex).unwrap(),
            &message,
            long_signature,
        ),
    ];

    ConformanceVectorBundle {
        profile: PROFILE,
        hash_domain: HASH_DOMAIN,
        modulus: Q,
        public_key_bytes: PK_BYTES,
        signature_bytes: SIGNATURE_BYTES,
        challenge_bytes: CHALLENGE_BYTES,
        challenge_used_bits: CHALLENGE_USED_BITS,
        response_bytes_per_repetition: RESPONSE_BYTES_PER_REP,
        repetitions: REPS,
        response_bound: RESPONSE_BOUND,
        lean_challenge_codec_endpoint: "N32CanonicalChallengeByteCodec.productionBitLayout17",
        lean_hash_frame_endpoint: "N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame",
        lean_hash_frame_length_endpoint: "N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame_length",
        positive,
        negative,
    }
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
           --json"
    );
}

fn parse_args() -> Result<Config, String> {
    let mut public_key = None;
    let mut signature = None;
    let mut message = None;
    let mut json = false;
    let mut emit_conformance_vector = false;
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
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    Ok(Config {
        public_key: if emit_conformance_vector {
            public_key.unwrap_or_default()
        } else {
            public_key.ok_or_else(|| "missing --public-key-hex".to_string())?
        },
        message: message.unwrap_or_default(),
        signature: if emit_conformance_vector {
            signature.unwrap_or_default()
        } else {
            signature.ok_or_else(|| "missing --signature-hex".to_string())?
        },
        json,
        emit_conformance_vector,
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
        println!(
            "{}",
            serde_json::to_string_pretty(&conformance_vector_bundle()).unwrap()
        );
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
    fn canonical_length_checks_reject_short_and_long_inputs() {
        let positive = zero_public_key_positive_vector();
        let public_key = hex_decode(&positive.public_key_hex).unwrap();
        let signature = hex_decode(&positive.signature_hex).unwrap();
        let message = hex_decode(&positive.message_hex).unwrap();

        let report = verify_signature(&public_key[..public_key.len() - 1], &message, &signature);
        assert_eq!(
            report.reason,
            format!("public key has {} bytes, expected {PK_BYTES}", PK_BYTES - 1)
        );

        let mut long_public_key = public_key.clone();
        long_public_key.push(0);
        let report = verify_signature(&long_public_key, &message, &signature);
        assert_eq!(
            report.reason,
            format!("public key has {} bytes, expected {PK_BYTES}", PK_BYTES + 1)
        );

        let report = verify_signature(&public_key, &message, &signature[..signature.len() - 1]);
        assert_eq!(
            report.reason,
            format!(
                "signature has {} bytes, expected {SIGNATURE_BYTES}",
                SIGNATURE_BYTES - 1
            )
        );

        let mut long_signature = signature;
        long_signature.push(0);
        let report = verify_signature(&public_key, &message, &long_signature);
        assert_eq!(
            report.reason,
            format!(
                "signature has {} bytes, expected {SIGNATURE_BYTES}",
                SIGNATURE_BYTES + 1
            )
        );
    }

    #[test]
    fn every_single_byte_mutation_of_positive_fixture_rejects() {
        let positive = zero_public_key_positive_vector();
        let public_key = hex_decode(&positive.public_key_hex).unwrap();
        let signature = hex_decode(&positive.signature_hex).unwrap();
        let message = hex_decode(&positive.message_hex).unwrap();

        for idx in 0..public_key.len() {
            let mut mutated = public_key.clone();
            mutated[idx] ^= 0x01;
            let report = verify_signature(&mutated, &message, &signature);
            assert!(
                !report.accepted,
                "public key byte mutation at index {idx} unexpectedly accepted"
            );
        }

        for idx in 0..signature.len() {
            let mut mutated = signature.clone();
            mutated[idx] ^= 0x01;
            let report = verify_signature(&public_key, &message, &mutated);
            assert!(
                !report.accepted,
                "signature byte mutation at index {idx} unexpectedly accepted"
            );
        }
    }

    #[test]
    fn conformance_vector_bundle_has_expected_shape() {
        let bundle = conformance_vector_bundle();
        assert_eq!(bundle.positive.hash_frame_bytes, 1_540);
        assert_eq!(
            bundle.positive.hash_frame_sha256,
            "500b7e675f1970875c294ccb0d00eee386910b1a37c8ecf5b78d97e0af51993c"
        );
        assert_eq!(bundle.negative.len(), 9);
        assert!(bundle.negative.iter().all(|case| !case.report.accepted));
    }
}
