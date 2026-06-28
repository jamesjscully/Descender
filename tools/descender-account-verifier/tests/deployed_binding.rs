//! End-to-end: prove a real account-creation instance, transport it through the
//! opaque bundle, build the deployed anchors via the export bridge, and confirm
//! the deployed verifier binds correctly — and REJECTS every tampered anchor and
//! a corrupted proof.
//!
//! This runs ONE real single-instance STARK prove (memory-light; the heavy jobs
//! are batch N≥1024 / 24k-sample tests, deliberately not exercised here).

use descender_account_stark::{prove_account_creation_with_seed, AccountCreationSecrets, N_LANES, SYNDROME_LANES};
use descender_account_verifier::{
    pack_bundle, role_root_export_digest, verify_deployed_account_creation, DeployedCreationAnchors,
    VerifyError,
};
use p3_field::PrimeField32;

fn sample_secrets() -> AccountCreationSecrets {
    AccountCreationSecrets {
        pad_seed: [0xA5; 32],
        slot_id: 7,
        account_projection: [11, 2222, 333, 4444, 555, 6666],
        lifecycle_anchor: [0x11; 32],
        c1_nonce: [0x22; 16],
        c6_nonce: [0x33; 16],
        account_id: [0x44; 32],
        verifier_version: 0x0001_0002,
        account_state_root: [0x55; 32],
        syndrome: [9, 8, 7, 6, 5, 4, 3, 2, 1, 100, 200, 300],
        checked_core_key_commitment: [0x2bu8; 32],
    }
}

/// Build the deployed anchors a correct PRODUCER would emit for `public`.
fn anchors_for(public: &descender_account_verifier::AccountCreationPublic) -> DeployedCreationAnchors {
    let mut masked = [0u16; 6];
    assert_eq!(N_LANES, 6);
    for (j, v) in public.masked_projection.iter().enumerate() {
        masked[j] = v.as_canonical_u32() as u16;
    }
    let mut packed = [0u8; descender_account_verifier::PACKED_WITNESS_COMMITMENT_BYTES];
    for (k, v) in public.packed_witness_commitment.iter().enumerate() {
        packed[k] = v.as_canonical_u32() as u8;
    }
    DeployedCreationAnchors {
        account_id: public.account_id,
        slot_id: public.slot_id,
        account_state_root: public.account_state_root,
        lifecycle_anchor: public.lifecycle_anchor,
        verifier_version: public.verifier_version,
        masked_projection: masked,
        native_role_root_commitment: role_root_export_digest(&public.role_root),
        packed_witness_commitment: packed,
    }
}

#[test]
fn deployed_verify_accepts_honest_and_binds_anchors() {
    assert_eq!(SYNDROME_LANES, 12);
    let secrets = sample_secrets();
    let (public, proof) = prove_account_creation_with_seed(&secrets, 0xDEAD_BEEF).expect("prove");
    let bundle = pack_bundle(&public, &proof).expect("pack");
    let anchors = anchors_for(&public);

    // Honest: accepts.
    verify_deployed_account_creation(&bundle, &anchors).expect("honest bundle must verify");

    // Each tampered anchor must be rejected with an AnchorMismatch (proof is fine).
    let mut a = anchors.clone();
    a.account_id[0] ^= 1;
    assert!(matches!(
        verify_deployed_account_creation(&bundle, &a),
        Err(VerifyError::AnchorMismatch(_))
    ));

    let mut a = anchors.clone();
    a.slot_id ^= 1;
    assert!(matches!(
        verify_deployed_account_creation(&bundle, &a),
        Err(VerifyError::AnchorMismatch(_))
    ));

    let mut a = anchors.clone();
    a.account_state_root[5] ^= 1;
    assert!(matches!(
        verify_deployed_account_creation(&bundle, &a),
        Err(VerifyError::AnchorMismatch(_))
    ));

    let mut a = anchors.clone();
    a.lifecycle_anchor[7] ^= 1;
    assert!(matches!(
        verify_deployed_account_creation(&bundle, &a),
        Err(VerifyError::AnchorMismatch(_))
    ));

    let mut a = anchors.clone();
    a.verifier_version ^= 1;
    assert!(matches!(
        verify_deployed_account_creation(&bundle, &a),
        Err(VerifyError::AnchorMismatch(_))
    ));

    let mut a = anchors.clone();
    a.masked_projection[2] ^= 1;
    assert!(matches!(
        verify_deployed_account_creation(&bundle, &a),
        Err(VerifyError::AnchorMismatch(_))
    ));

    // Tampered role-root export commitment (the named bridge): rejected.
    let mut a = anchors.clone();
    a.native_role_root_commitment[0] ^= 1;
    assert!(matches!(
        verify_deployed_account_creation(&bundle, &a),
        Err(VerifyError::AnchorMismatch(_))
    ));

    // MED-2: a packed_witness_commitment that disagrees with the proven one (i.e.
    // the deployed binding commits a DIFFERENT witness than the creation proof
    // attests) must be rejected — this is the syndrome/masked↔spend binding.
    let mut a = anchors.clone();
    a.packed_witness_commitment[0] ^= 1;
    assert!(matches!(
        verify_deployed_account_creation(&bundle, &a),
        Err(VerifyError::AnchorMismatch(_))
    ));
}

#[test]
fn deployed_verify_rejects_corrupted_proof() {
    let secrets = sample_secrets();
    let (public, proof) = prove_account_creation_with_seed(&secrets, 0x0123_4567).expect("prove");
    let mut bundle = pack_bundle(&public, &proof).expect("pack");
    let anchors = anchors_for(&public);

    // Flip a byte deep in the bundle (hits the embedded proof region) and confirm
    // it no longer verifies (either decode or STARK failure — never Ok).
    let mid = bundle.len() * 3 / 4;
    bundle[mid] ^= 0xFF;
    assert!(
        verify_deployed_account_creation(&bundle, &anchors).is_err(),
        "corrupted bundle must not verify"
    );
}
