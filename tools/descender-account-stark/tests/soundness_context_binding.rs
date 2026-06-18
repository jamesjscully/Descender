//! Soundness negative tests for the context fields that the AIR does NOT
//! constrain in-circuit (single mode): `account_state_root` and
//! `verifier_version`. Per the 2026-06-18 soundness audit (MED-1), these are
//! bound ONLY by being observed into the Fiat–Shamir transcript via
//! `public_values_vec`. That is a load-bearing single point of failure with the
//! exact shape of the prior batch "verify with empty public values" break, so we
//! pin it with explicit attempted-tamper tests: a verifier that observes the
//! tampered public value MUST reject (the FS challenges diverge from the proof).
//!
//! One honest prove per test (single-instance, memory-light).

use descender_account_stark::{
    prove_account_creation_with_seed, verify_account_creation, AccountCreationSecrets,
};

fn honest_secrets() -> AccountCreationSecrets {
    AccountCreationSecrets {
        pad_seed: [0x5A; 32],
        slot_id: 42,
        account_projection: [1, 22, 333, 4096, 55, 6000],
        lifecycle_anchor: [0x77; 32],
        c1_nonce: [0x11; 16],
        c6_nonce: [0x22; 16],
        account_id: [0x99; 32],
        verifier_version: 0x0000_0007,
        account_state_root: [0x33; 32],
        syndrome: [10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120],
    }
}

#[test]
fn tampered_account_state_root_is_rejected() {
    let s = honest_secrets();
    let (public, proof) = prove_account_creation_with_seed(&s, 0xA11CE).expect("prove");
    // Sanity: honest verifies.
    verify_account_creation(&public, &proof).expect("honest must verify");

    // MED-1: account_state_root is NOT constrained in-AIR; flipping it in the
    // public statement must still be rejected (transcript divergence).
    let mut tampered = public.clone();
    tampered.account_state_root[0] ^= 0x01;
    assert!(
        verify_account_creation(&tampered, &proof).is_err(),
        "tampered account_state_root must be rejected (FS transcript binding)"
    );
}

#[test]
fn tampered_verifier_version_is_rejected() {
    let s = honest_secrets();
    let (public, proof) = prove_account_creation_with_seed(&s, 0xB0B).expect("prove");
    verify_account_creation(&public, &proof).expect("honest must verify");

    let mut tampered = public.clone();
    tampered.verifier_version ^= 0x01;
    assert!(
        verify_account_creation(&tampered, &proof).is_err(),
        "tampered verifier_version must be rejected (FS transcript binding)"
    );
}
