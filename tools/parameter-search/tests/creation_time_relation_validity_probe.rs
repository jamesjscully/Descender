// Relation-VALIDITY probe for the creation-time hidden-source soundness certificate.
//
// CONTEXT. A prior risk-first attack PROVED the account-witness opening map is
// INJECTIVE (binding SOUND, reduces to SHA256-CR). The risk frontier moved from
// BINDING (uniqueness of the opening) to RELATION-VALIDITY (does the unique
// opening satisfy a VALID hidden-source relation?).
//
// THE ATTACK. Construct a creation certificate whose account_projection / witness
// opening is UNIQUE and repacks consistently to the leaf-fixed
// account_witness_commitment (so it PASSES the binding check) but is NOT a valid
// hidden-source relation opening (relation-INVALID), and determine whether
// `verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate`
// ACCEPTS or REJECTS it.
//
//   ACCEPTS a relation-invalid opening  => creation_time_hidden_source_soundness_break is REAL.
//   REJECTS                              => the relation arm is on track.
//
// This file builds the honest accepting fixture (replicating the in-crate
// `reviewed_fq12_packed_witness_fixture`) then mutates it into two relation-invalid
// shapes and runs them against the REAL soundness certificate verifier.

use descender_parameter_search::production_reference as pr;

use pr::{
    DIGEST_BYTES, HIDDEN_SOURCE_FALLBACK_ACCOUNT_PROJECTION_LANES as ACCOUNT_LANES,
    HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES,
    HIDDEN_SOURCE_FALLBACK_PROJECTION_LANES as LANES,
    HiddenSourceFallbackProjectionAccountWitnessOpening, HiddenSourceFallbackProjectionCarrier,
    LOCAL_FREE, PK_BYTES, ProductionAccountCommitment, ProductionCompactResponses,
    ProductionLocalOpenings, ProductionPublicKey, ProductionReviewedFq12AccountCarriedProof,
    ProductionReviewedFq12StateAssistedAccountSlot,
    ProductionReviewedFq12StateAssistedAccountState, ProductionReviewedFq12StateAssistedOpening,
    ProductionSignature, Q, REPS, REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS as ROWS,
    REVIEWED_FQ12_ACCOUNT_CARRIED_RESPONSE_RESIDUES, REVIEWED_FQ12_PAD_SEED_BYTES, challenge_bytes,
    decode_challenge, encode_hidden_source_fallback_account_witness_opening_frame,
    encode_reviewed_fq12_account_carried_masked_response,
    encode_reviewed_fq12_account_carried_signature,
    encode_reviewed_fq12_account_carried_with_fallback_projection,
    hidden_source_fallback_projection_values_from_hidden_material,
    pack_reviewed_fq12_account_carried_syndrome_digest,
    reviewed_fq12_account_witness_projection_opening_digest,
    reviewed_fq12_pack_masked_witness_state18_from_parts, reviewed_fq12_pad_residues_from_seed,
    reviewed_fq12_pad_seed_opening_digest, reviewed_fq12_state_assisted_hidden_source_statement,
    reviewed_fq12_state_assisted_opening_commitment_digest,
    reviewed_fq12_state_assisted_projection_carrier_binding,
    verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate,
};

struct Fixture {
    public_key: ProductionPublicKey,
    message: &'static [u8],
    account: ProductionReviewedFq12StateAssistedAccountState,
    opening: ProductionReviewedFq12StateAssistedOpening,
    statement: pr::ProductionReviewedFq12HiddenSourceStatement,
    pad_seed: [u8; REVIEWED_FQ12_PAD_SEED_BYTES],
    signature: ProductionSignature,
    signature_with_carrier: ProductionSignature,
    opening_commitment: [u8; DIGEST_BYTES],
    account_projection: [u16; ACCOUNT_LANES],
}

// Replicates the in-crate honest accepting fixture, but exposes opening_commitment
// and the recomputed projection lanes so we can build relation-invalid carriers.
fn build_fixture() -> Fixture {
    let public_key = ProductionPublicKey::new(vec![0_u8; PK_BYTES]).unwrap();
    let message: &'static [u8] = b"reviewed Fq12 relation-validity probe fixture";
    let slot_id = 0_u32;
    let pad_seed = [0x42_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
    let witness_syndrome = [0_u16; ROWS];
    let account_projection = [1_u16, 2, 3, 4, 5, 6];
    let witness_commitment = reviewed_fq12_pack_masked_witness_state18_from_parts(
        &witness_syndrome,
        &account_projection,
        &pad_seed,
        slot_id as u64,
    )
    .unwrap();
    let randomizer_commitment =
        pack_reviewed_fq12_account_carried_syndrome_digest(&[0_u16; ROWS]).unwrap();
    let challenge = challenge_bytes(
        public_key.as_bytes(),
        message,
        slot_id,
        &witness_commitment,
        &randomizer_commitment,
    );
    let decoded_challenge = decode_challenge(&challenge).unwrap();
    let mut openings = ProductionLocalOpenings {
        active_free: [[0_u16; LOCAL_FREE]; REPS],
    };
    for rep in 0..REPS {
        openings.active_free[rep] =
            reviewed_fq12_pad_residues_from_seed(&pad_seed, decoded_challenge.active_block[rep]);
    }
    let proof = ProductionReviewedFq12AccountCarriedProof {
        proof_commitment: randomizer_commitment,
        masked_quotient_response: encode_reviewed_fq12_account_carried_masked_response(
            &[0_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_RESPONSE_RESIDUES],
        )
        .unwrap(),
    };
    let signature = encode_reviewed_fq12_account_carried_signature(
        slot_id,
        &challenge,
        &ProductionCompactResponses {
            active_free: [[0_i16; LOCAL_FREE]; REPS],
        },
        &openings,
        &proof,
    )
    .unwrap();
    let account_witness_opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
        selected_source_values: account_projection,
    };
    let account_witness_opening_frame =
        encode_hidden_source_fallback_account_witness_opening_frame(&account_witness_opening)
            .unwrap();
    let witness_opening_digest =
        reviewed_fq12_account_witness_projection_opening_digest(&account_witness_opening_frame)
            .unwrap();
    let pad_seed_digest = reviewed_fq12_pad_seed_opening_digest(&pad_seed);
    let mut opening = ProductionReviewedFq12StateAssistedOpening {
        slot_id: slot_id as u64,
        account_witness_commitment: witness_commitment,
        account_randomizer_commitment: randomizer_commitment,
        restore_anchor_digest: [0x33_u8; DIGEST_BYTES],
        witness_opening_digest,
        pad_seed_digest,
        carrier_handle_secret_digest: [0x44_u8; DIGEST_BYTES],
    };
    let opening_commitment = reviewed_fq12_state_assisted_opening_commitment_digest(&opening);
    let slot = ProductionReviewedFq12StateAssistedAccountSlot::new(
        slot_id,
        ProductionAccountCommitment::new(witness_commitment, randomizer_commitment),
        opening_commitment,
        opening.restore_anchor_digest,
        pad_seed_digest,
    );
    let account = ProductionReviewedFq12StateAssistedAccountState::new(vec![slot]).unwrap();
    let statement = reviewed_fq12_state_assisted_hidden_source_statement(
        &public_key,
        message,
        &account,
        &signature,
    )
    .unwrap();
    opening.account_witness_commitment = statement.witness_commitment;

    let projection_lanes = hidden_source_fallback_projection_values_from_hidden_material(
        &account_witness_opening,
        &pad_seed,
    )
    .unwrap();
    let (check_bytes, transcript_binding) =
        reviewed_fq12_state_assisted_projection_carrier_binding(
            &opening_commitment,
            &opening,
            &statement,
            &projection_lanes,
        )
        .unwrap();
    let mut carrier_bytes = [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES];
    for (lane, value) in projection_lanes.iter().enumerate() {
        carrier_bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    carrier_bytes[24..27].copy_from_slice(&check_bytes);
    carrier_bytes[27..56].copy_from_slice(&transcript_binding);
    let carrier = HiddenSourceFallbackProjectionCarrier::from_bytes(&carrier_bytes).unwrap();
    let signature_with_carrier =
        encode_reviewed_fq12_account_carried_with_fallback_projection(&signature, &carrier)
            .unwrap();

    Fixture {
        public_key,
        message,
        account,
        opening,
        statement,
        pad_seed,
        signature,
        signature_with_carrier,
        opening_commitment,
        account_projection,
    }
}

fn run_soundness(
    f: &Fixture,
    signature_with_carrier: &ProductionSignature,
) -> Result<
    pr::ProductionReviewedFq12MaskedProjectionSoundnessCertificate,
    pr::ProductionAccountBindingError,
> {
    verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate(
        &f.public_key,
        f.message,
        &f.account,
        signature_with_carrier,
        &f.opening,
        &f.statement,
        &f.pad_seed,
    )
}

// Sanity: the honest fixture is ACCEPTED by the soundness certificate verifier.
#[test]
fn honest_fixture_soundness_certificate_accepts() {
    let f = build_fixture();
    let cert = run_soundness(&f, &f.signature_with_carrier).expect("honest fixture must accept");
    assert_eq!(cert.slot_id, 0);
    // The account_projection in the certificate is the relation-derived value,
    // determined solely by the leaf-fixed commitment + pad_seed + slot_id.
    assert_eq!(
        cert.packed_witness_opening.account_projection, f.account_projection,
        "certificate account_projection must be the relation-derived value"
    );
}

// RELATION-INVALID construction A: mutate one CLAIMED carrier lane away from the
// relation-derived value, leaving check_bytes/transcript_binding untouched. The
// witness commitment / opening_commitment are UNCHANGED => binding still holds
// (the opening still uniquely opens the leaf). But carrier.lane != recomputed.
#[test]
fn relation_invalid_claimed_lane_mutation_is_rejected() {
    let f = build_fixture();
    let mut bytes = f.signature_with_carrier.as_bytes().to_vec();
    // carrier starts right after the 258-byte checked-core transaction signature.
    let carrier_start = bytes.len() - HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES;
    let lane0 = u16::from_le_bytes([bytes[carrier_start], bytes[carrier_start + 1]]);
    bytes[carrier_start..carrier_start + 2].copy_from_slice(&((lane0 + 1) % Q).to_le_bytes());
    let mutated = ProductionSignature::new(bytes);

    let result = run_soundness(&f, &mutated);
    assert!(
        result.is_err(),
        "RELATION BREAK: soundness certificate ACCEPTED a claimed-lane != recomputed projection"
    );
}

// RELATION-INVALID construction B (the decisive one). Build a carrier that is
// INTERNALLY consistent (check_bytes/transcript_binding recomputed for the mutated
// lanes, binding them to the SAME fixed opening_commitment) but whose claimed
// projection lanes differ from the relation-derived ones. This passes:
//   - binding   (witness commitment / opening_commitment unchanged, unique opening)
//   - the carrier-binding (check_bytes match the claimed lanes + opening_commitment)
// yet is relation-INVALID (claimed projection != value derived from the leaf).
// If the soundness certificate ACCEPTS this, the relation is not enforced.
#[test]
fn relation_invalid_internally_consistent_carrier_is_rejected() {
    let f = build_fixture();

    // The relation-derived lanes (what the verifier recomputes from the leaf).
    let derived = hidden_source_fallback_projection_values_from_hidden_material(
        &HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: f.account_projection,
        },
        &f.pad_seed,
    )
    .unwrap();

    // Forge claimed lanes that differ from derived on the first account lane.
    let mut forged_lanes = derived;
    forged_lanes[0] = (forged_lanes[0] + 1) % Q;
    assert_ne!(
        forged_lanes, derived,
        "forged lanes must differ from derived"
    );

    // Recompute a SELF-CONSISTENT carrier binding for the forged lanes, bound to
    // the unchanged opening_commitment + opening + statement. This makes the
    // carrier internally well-formed (defeats the check-bytes guard) while the
    // claimed projection is relation-INVALID.
    let (check_bytes, transcript_binding) =
        reviewed_fq12_state_assisted_projection_carrier_binding(
            &f.opening_commitment,
            &f.opening,
            &f.statement,
            &forged_lanes,
        )
        .unwrap();

    let mut carrier_bytes = [0_u8; HIDDEN_SOURCE_FALLBACK_PROJECTION_CARRIER_BYTES];
    for (lane, value) in forged_lanes.iter().enumerate() {
        carrier_bytes[lane * 2..lane * 2 + 2].copy_from_slice(&value.to_le_bytes());
    }
    carrier_bytes[24..27].copy_from_slice(&check_bytes);
    carrier_bytes[27..56].copy_from_slice(&transcript_binding);
    let carrier = HiddenSourceFallbackProjectionCarrier::from_bytes(&carrier_bytes).unwrap();
    let signature_with_carrier =
        encode_reviewed_fq12_account_carried_with_fallback_projection(&f.signature, &carrier)
            .unwrap();

    let result = run_soundness(&f, &signature_with_carrier);
    match &result {
        Ok(cert) => panic!(
            "RELATION BREAK: soundness certificate ACCEPTED an internally-consistent but \
             relation-INVALID carrier. slot_id={}, derived={:?}, forged_claimed={:?}",
            cert.slot_id, derived, forged_lanes
        ),
        Err(err) => {
            // Expected: the lane mismatch (claimed != recomputed) drives the
            // hidden-material predicate to Export, which the carrier-tuple wrapper
            // turns into a REJECT. Print the reason for the verdict record.
            eprintln!(
                "REJECTED (expected). reason={:?} slot_id={:?}",
                err.reason(),
                err.slot_id()
            );
        }
    }
    assert!(
        result.is_err(),
        "RELATION BREAK: relation-invalid internally-consistent carrier was accepted"
    );
}

// MANDATORY binding regression (from the binding verdict): opening_commitment must
// CHANGE under a SYNDROME-ONLY witness mutation. If a refactor drops
// account_witness_commitment from the opening_commitment frame, the 12-dim
// syndrome-lane nullity survivor reopens. This asserts (not just prints) the guard.
#[test]
fn opening_commitment_changes_under_syndrome_only_mutation() {
    let pad_seed = [0x5a_u8; REVIEWED_FQ12_PAD_SEED_BYTES];
    let slot_id = 0_u64;
    // Same projection lanes; mutate ONLY a syndrome coordinate.
    let proj = [3_u16, 5, 8, 13, 21, 34];
    let syndrome_a = [0_u16; ROWS];
    let mut syndrome_b = [0_u16; ROWS];
    syndrome_b[0] = 1;

    let commit_a = reviewed_fq12_pack_masked_witness_state18_from_parts(
        &syndrome_a,
        &proj,
        &pad_seed,
        slot_id,
    )
    .unwrap();
    let commit_b = reviewed_fq12_pack_masked_witness_state18_from_parts(
        &syndrome_b,
        &proj,
        &pad_seed,
        slot_id,
    )
    .unwrap();
    assert_ne!(
        commit_a, commit_b,
        "syndrome-only mutation must change the witness commitment"
    );

    let oc = |witness_commitment: [u8; DIGEST_BYTES]| {
        let opening = ProductionReviewedFq12StateAssistedOpening {
            slot_id,
            account_witness_commitment: witness_commitment,
            account_randomizer_commitment: [1_u8; DIGEST_BYTES],
            restore_anchor_digest: [2_u8; DIGEST_BYTES],
            witness_opening_digest: [3_u8; DIGEST_BYTES],
            pad_seed_digest: [4_u8; DIGEST_BYTES],
            carrier_handle_secret_digest: [5_u8; DIGEST_BYTES],
        };
        reviewed_fq12_state_assisted_opening_commitment_digest(&opening)
    };

    assert_ne!(
        oc(commit_a),
        oc(commit_b),
        "REGRESSION: opening_commitment did NOT change under a syndrome-only mutation \
         (account_witness_commitment dropped from the opening frame => survivor reopened)"
    );
    let _ = LANES;
}
