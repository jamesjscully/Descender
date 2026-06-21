//! Integration tests for the Path 2 account-creation ZK-STARK library.
//!
//! Covered (all run in --release; the prover is too slow in debug):
//!   * single-instance prove -> verify round-trip (completeness)
//!   * single-instance soundness: tampered proof / tampered public -> Err
//!   * ZK hiding by ATTEMPTED RECOVERY (not substring scan): the opened pad_seed
//!     column never equals the witness byte and decorrelates across mask seeds
//!   * batch prove -> verify round-trip + digest binding
//!   * batch soundness: tampered public tuple -> Err
//!   * invalid witness (non-canonical projection) -> Err

use descender_account_stark::{
    padseed_witness_column, proof_size_bytes, prove_account_creation,
    prove_account_creation_batch, prove_account_creation_batch_with_seed,
    prove_account_creation_with_seed, trace_columns, verify_account_creation,
    verify_account_creation_batch, AccountCreationSecrets, Challenge, Val, N_LANES,
    PROD_SOUNDNESS_BITS,
};
use p3_field::PrimeCharacteristicRing;

fn secrets_a() -> AccountCreationSecrets {
    AccountCreationSecrets {
        pad_seed: core::array::from_fn(|i| (i as u8).wrapping_mul(7).wrapping_add(1)),
        slot_id: 0xDEAD_BEEF_0000_0042,
        account_projection: [11, 2222, 333, 4444, 555, 12288],
        lifecycle_anchor: core::array::from_fn(|i| (i as u8).wrapping_mul(5).wrapping_add(2)),
        c1_nonce: core::array::from_fn(|i| (i as u8).wrapping_mul(3).wrapping_add(5)),
        c6_nonce: core::array::from_fn(|i| (i as u8).wrapping_mul(9).wrapping_add(1)),
        account_id: core::array::from_fn(|i| (i as u8).wrapping_mul(13).wrapping_add(3)),
        verifier_version: 0x0102_0304,
        account_state_root: core::array::from_fn(|i| (i as u8).wrapping_mul(17).wrapping_add(7)),
        syndrome: core::array::from_fn(|i| ((i as u32 * 1009 + 17) % 12289) as u16),
    }
}

fn secrets_indexed(seed: u64) -> AccountCreationSecrets {
    let b = |salt: u64| -> u8 { (seed.wrapping_mul(31).wrapping_add(salt) & 0xff) as u8 };
    AccountCreationSecrets {
        pad_seed: core::array::from_fn(|i| b(i as u64)),
        slot_id: seed.wrapping_mul(0x9E37),
        account_projection: core::array::from_fn(|i| {
            (seed.wrapping_mul(7).wrapping_add(i as u64) % 12289) as u16
        }),
        lifecycle_anchor: core::array::from_fn(|i| b(100 + i as u64)),
        c1_nonce: core::array::from_fn(|i| b(50 + i as u64)),
        c6_nonce: core::array::from_fn(|i| b(70 + i as u64)),
        account_id: core::array::from_fn(|i| b(150 + i as u64)),
        verifier_version: seed.wrapping_mul(0x1234_5) as u32,
        account_state_root: core::array::from_fn(|i| b(200 + i as u64)),
        syndrome: core::array::from_fn(|i| {
            (seed.wrapping_mul(13).wrapping_add(i as u64) % 12289) as u16
        }),
    }
}

#[test]
fn single_roundtrip_and_size() {
    let s = secrets_a();
    let (public, proof) = prove_account_creation(&s).expect("prove");
    verify_account_creation(&public, &proof).expect("verify Ok");

    let bytes = proof_size_bytes(&proof);
    println!(
        "[single] trace_cols={} proof={} B (conservative Johnson-bound FRI soundness {} bits)",
        trace_columns(),
        bytes,
        PROD_SOUNDNESS_BITS
    );
    assert!(bytes > 0);
    assert_eq!(PROD_SOUNDNESS_BITS, 128);
}

#[test]
fn single_soundness_tampered_public_rejected() {
    let s = secrets_a();
    let (mut public, proof) = prove_account_creation(&s).expect("prove");
    // Flip one masked_projection lane in the claimed public statement.
    public.masked_projection[0] += Val::ONE;
    assert!(
        verify_account_creation(&public, &proof).is_err(),
        "verify accepted a tampered public statement (soundness)"
    );
}

#[test]
fn single_soundness_tampered_proof_rejected() {
    let s = secrets_a();
    let (public, mut proof) = prove_account_creation(&s).expect("prove");
    // Tamper an opened trace value inside the proof (Proof is not Clone, so we
    // mutate the only copy; this test does not need the original afterwards).
    proof.opened_values.trace_local[0] += Challenge::ONE;
    let bad = proof;
    assert!(
        verify_account_creation(&public, &bad).is_err(),
        "verify accepted a tampered proof (soundness)"
    );
}

#[test]
fn single_wrong_public_for_other_instance_rejected() {
    // A proof for instance A must not verify against instance B's public.
    let (pub_a, proof_a) = prove_account_creation(&secrets_a()).expect("prove a");
    let (pub_b, _proof_b) = prove_account_creation(&secrets_indexed(99)).expect("prove b");
    assert_ne!(pub_a, pub_b);
    assert!(
        verify_account_creation(&pub_b, &proof_a).is_err(),
        "verify accepted a proof against the wrong public statement"
    );
}

#[test]
fn single_zk_hiding_by_attempted_recovery() {
    let s = secrets_a();
    // Two proofs of the SAME witness+statement under DIFFERENT mask seeds.
    let (pub_a, proof_a) = prove_account_creation_with_seed(&s, 0x1111_2222).expect("prove a");
    let (pub_b, proof_b) = prove_account_creation_with_seed(&s, 0x9999_8888).expect("prove b");
    assert_eq!(pub_a, pub_b, "same witness -> same public statement");
    verify_account_creation(&pub_a, &proof_a).expect("a verifies");
    verify_account_creation(&pub_b, &proof_b).expect("b verifies");

    // Attempted recovery: the opened pad_seed witness column must NOT equal the
    // real witness byte under either seed (salted-leaf hiding), and must differ
    // across seeds (randomized opening). This is recovery-based, not substring.
    let k = 7usize;
    let col = padseed_witness_column(k);
    let target = Challenge::from(Val::from_u32(s.pad_seed[k] as u32));
    let ra = proof_a.opened_values.trace_local[col];
    let rb = proof_b.opened_values.trace_local[col];
    assert_ne!(ra, target, "HIDING BROKEN: recovered pad_seed byte (seed a)");
    assert_ne!(rb, target, "HIDING BROKEN: recovered pad_seed byte (seed b)");
    assert_ne!(ra, rb, "opening not randomized across mask seeds");

    // The proofs themselves must differ (ZK randomness present).
    assert!(
        proof_a.opened_values.trace_local != proof_b.opened_values.trace_local
            || proof_a.commitments.trace != proof_b.commitments.trace,
        "proofs identical across mask seeds (ZK randomness missing)"
    );
}

#[test]
fn single_invalid_witness_rejected() {
    let mut s = secrets_a();
    s.account_projection[0] = 12289; // == q12289, not canonical
    assert!(
        prove_account_creation(&s).is_err(),
        "prove accepted a non-canonical account_projection lane"
    );
}

#[test]
fn batch_roundtrip_and_binding() {
    // Small N + reduced footprint (host RAM is limited; the in-AIR digest chain
    // widens the trace, so we keep the batch small).
    let n = 4usize;
    let secrets: Vec<_> = (0..n as u64).map(secrets_indexed).collect();
    let (digest, publics, proof) = prove_account_creation_batch(&secrets).expect("batch prove");
    assert_eq!(publics.len(), n);
    verify_account_creation_batch(&digest, &publics, &proof).expect("batch verify Ok");

    let bytes = proof_size_bytes(&proof);
    let tuples_bytes = n * (2 * 10 + N_LANES) * 4;
    println!(
        "[batch N={n}] proof={} B + tuples={} B = total {} B ({:.3} KB/acct)",
        bytes,
        tuples_bytes,
        bytes + tuples_bytes,
        (bytes + tuples_bytes) as f64 / 1024.0 / n as f64
    );
}

#[test]
fn batch_soundness_tampered_public_rejected() {
    let n = 4usize;
    let secrets: Vec<_> = (0..n as u64).map(secrets_indexed).collect();
    let (digest, mut publics, proof) = prove_account_creation_batch(&secrets).expect("batch prove");

    // Adversary flips one instance's claimed masked output in the supplied tuples.
    publics[0].masked_projection[0] += Val::ONE;
    assert!(
        verify_account_creation_batch(&digest, &publics, &proof).is_err(),
        "batch verify accepted a tampered public tuple (digest binding)"
    );
}

#[test]
fn batch_soundness_tampered_proof_rejected() {
    let n = 4usize;
    let secrets: Vec<_> = (0..n as u64).map(secrets_indexed).collect();
    let (digest, publics, mut proof) =
        prove_account_creation_batch(&secrets).expect("batch prove");

    proof.opened_values.trace_local[0] += Challenge::ONE;
    let bad = proof;
    assert!(
        verify_account_creation_batch(&digest, &publics, &bad).is_err(),
        "batch verify accepted a tampered proof (soundness)"
    );
}

#[test]
fn batch_zk_hiding_by_attempted_recovery() {
    let n = 4usize;
    let secrets: Vec<_> = (0..n as u64).map(secrets_indexed).collect();
    let (d_a, _pa, proof_a) =
        prove_account_creation_batch_with_seed(&secrets, 0x1111_2222).expect("a");
    let (d_b, _pb, proof_b) =
        prove_account_creation_batch_with_seed(&secrets, 0x9999_8888).expect("b");
    assert_eq!(d_a, d_b, "same witnesses -> same digest");

    let k = 7usize;
    let col = padseed_witness_column(k);
    let target = Challenge::from(Val::from_u32(secrets[0].pad_seed[k] as u32));
    let ra = proof_a.opened_values.trace_local[col];
    let rb = proof_b.opened_values.trace_local[col];
    assert_ne!(ra, target, "batch HIDING BROKEN (seed a)");
    assert_ne!(rb, target, "batch HIDING BROKEN (seed b)");
    assert!(
        proof_a.opened_values.trace_local != proof_b.opened_values.trace_local
            || proof_a.commitments.trace != proof_b.commitments.trace,
        "batch proofs identical across mask seeds (ZK missing)"
    );
}
