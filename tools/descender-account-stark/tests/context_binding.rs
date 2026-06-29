//! CONTEXT-BINDING tests — the fix for the context-free universal-admission-token.
//!
//! Before this fix, the public statement was only (pad_source_commitment,
//! opening_tuple_commitment, masked_projection) — bound to NO account / slot /
//! state / verifier / lifecycle, so a single valid proof was a universal
//! admission token usable for ANY context. This suite proves the token is now
//! bound to exactly one (account_id, slot_id, verifier_version,
//! account_state_root, lifecycle_anchor):
//!
//!   * completeness: honest secrets+context -> verifies AND public context matches
//!   * CRITICAL SOUNDNESS ANCHOR: a proof for context A is REJECTED against a
//!     DIFFERENT context B, tested for EACH context field changed independently
//!   * in-circuit soundness: tampering the C2-absorbed slot / C6-absorbed
//!     lifecycle / C6-absorbed account_id away from the public value -> reject
//!   * carrier mod-q: masked_projection lanes are canonical [0,q12289)
//!
//! All run in --release (the prover is too slow in debug).

use descender_account_stark::{
    prove_account_creation, verify_account_creation, AccountCreationPublic, AccountCreationSecrets,
    Val, RELATION_ID,
};
use p3_field::{PrimeCharacteristicRing, PrimeField32};

fn base_secrets() -> AccountCreationSecrets {
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
        checked_core_key_commitment: [0x2bu8; 32],
    }
}

const Q12289: u32 = 12289;

#[test]
fn completeness_context_matches() {
    let s = base_secrets();
    let (public, proof) = prove_account_creation(&s).expect("prove");
    verify_account_creation(&public, &proof).expect("honest proof verifies");

    // The public statement must carry exactly the context the prover used.
    assert_eq!(public.account_id, s.account_id);
    assert_eq!(public.slot_id, s.slot_id);
    assert_eq!(public.verifier_version, s.verifier_version);
    assert_eq!(public.account_state_root, s.account_state_root);
    assert_eq!(public.lifecycle_anchor, s.lifecycle_anchor);

    // Carrier mod-q: every masked lane is a canonical q12289 residue in [0,q).
    for (j, m) in public.masked_projection.iter().enumerate() {
        assert!(
            m.as_canonical_u32() < Q12289,
            "masked_projection[{j}] = {} is not a canonical q12289 residue",
            m.as_canonical_u32()
        );
    }
}

// ===========================================================================
// CRITICAL SOUNDNESS ANCHOR — the universal-admission-token kill.
// A proof generated for context A must be REJECTED when the verifier is handed
// a DIFFERENT public context B. We test EACH context field changed alone.
// ===========================================================================
#[test]
fn wrong_context_rejected_per_field() {
    let s = base_secrets();
    let (public_a, proof) = prove_account_creation(&s).expect("prove A");
    verify_account_creation(&public_a, &proof).expect("A verifies against its own context");

    // Helper: clone the honest public, mutate ONE context field, expect reject.
    let expect_reject = |label: &str, mutate: &dyn Fn(&mut AccountCreationPublic)| {
        let mut pb = public_a.clone();
        mutate(&mut pb);
        assert_ne!(pb, public_a, "[{label}] mutation was a no-op");
        let r = verify_account_creation(&pb, &proof);
        assert!(
            r.is_err(),
            "[{label}] UNIVERSAL-TOKEN: proof for context A accepted against changed context B"
        );
    };

    expect_reject("account_id", &|p| p.account_id[0] ^= 0xFF);
    expect_reject("slot_id", &|p| p.slot_id ^= 1);
    expect_reject("verifier_version", &|p| p.verifier_version ^= 1);
    expect_reject("account_state_root", &|p| p.account_state_root[0] ^= 0xFF);
    expect_reject("lifecycle_anchor", &|p| p.lifecycle_anchor[0] ^= 0xFF);
}

// ===========================================================================
// Cross-context replay: a full proof+statement for account A cannot be replayed
// to admit account B (different account identity entirely).
// ===========================================================================
#[test]
fn cross_account_replay_rejected() {
    let s_a = base_secrets();
    let (pub_a, proof_a) = prove_account_creation(&s_a).expect("prove A");

    // Different account context (a DIFFERENT account_id + slot + state + lifecycle).
    let mut s_b = base_secrets();
    s_b.account_id = core::array::from_fn(|i| (i as u8).wrapping_mul(29).wrapping_add(11));
    s_b.slot_id = 0x0123_4567_89AB_CDEF;
    s_b.lifecycle_anchor = core::array::from_fn(|i| (i as u8).wrapping_mul(23).wrapping_add(4));
    s_b.account_state_root = core::array::from_fn(|i| (i as u8).wrapping_mul(19).wrapping_add(9));
    let (pub_b, _proof_b) = prove_account_creation(&s_b).expect("prove B");

    assert_ne!(pub_a, pub_b);
    // Replay A's proof but claim B's account context -> must reject.
    assert!(
        verify_account_creation(&pub_b, &proof_a).is_err(),
        "UNIVERSAL-TOKEN: account A's proof admitted account B's context"
    );
}

// ===========================================================================
// IN-CIRCUIT soundness: the slot absorbed into the C2 PRF cannot differ from the
// public slot_id. Because slot is bound BOTH in the FS transcript AND by an AIR
// equality, a verifier handed a public slot != the proof's C2 slot rejects.
// (Covered field-wise by wrong_context_rejected_per_field; this isolates slot.)
// ===========================================================================
#[test]
fn c2_slot_must_equal_public_slot() {
    let s = base_secrets();
    let (public, proof) = prove_account_creation(&s).expect("prove");
    let mut tampered = public.clone();
    tampered.slot_id = tampered.slot_id.wrapping_add(0x100); // change a slot byte
    assert!(
        verify_account_creation(&tampered, &proof).is_err(),
        "C2-absorbed slot != public slot was accepted"
    );
}

// ===========================================================================
// IN-CIRCUIT soundness: lifecycle/account absorbed into the C6 opening tuple
// cannot differ from the public values.
// ===========================================================================
#[test]
fn c6_lifecycle_and_account_must_equal_public() {
    let s = base_secrets();
    let (public, proof) = prove_account_creation(&s).expect("prove");

    let mut t_life = public.clone();
    t_life.lifecycle_anchor[5] ^= 0xAA;
    assert!(
        verify_account_creation(&t_life, &proof).is_err(),
        "C6-absorbed lifecycle != public lifecycle was accepted"
    );

    let mut t_acct = public.clone();
    t_acct.account_id[5] ^= 0xAA;
    assert!(
        verify_account_creation(&t_acct, &proof).is_err(),
        "C6-absorbed account_id != public account_id was accepted"
    );
}

// ===========================================================================
// CLASS A (role-restriction) + CLASS B (proof-system-bookkeeping).
// One honest proof is reused for all assertions (keeps the prover footprint to a
// single proof under the small N=2 single-instance trace):
//   * completeness: honest proof verifies; relation_id == pinned RELATION_ID.
//   * CLASS A soundness: a tampered/forged role_root in the public statement
//     rejects (the verifier's public-value vector then disagrees with the
//     in-AIR role_out == public role_root equality / FS transcript).
//   * CLASS B soundness: a wrong relation_id in the public statement rejects
//     (verifier-side pinned-id check + in-AIR pin + FS transcript).
// ===========================================================================
#[test]
fn role_root_and_relation_id_bound() {
    let s = base_secrets();
    let (public, proof) = prove_account_creation(&s).expect("prove");

    // Completeness: honest proof verifies, and relation_id is the pinned constant.
    verify_account_creation(&public, &proof).expect("honest proof verifies");
    for j in 0..RELATION_ID.len() {
        assert_eq!(
            public.relation_id[j],
            Val::from_u32(RELATION_ID[j]),
            "relation_id[{j}] is not the pinned RELATION_ID"
        );
    }
    // role_root is a non-trivial (non-zero) digest exposed in the statement.
    assert!(
        public.role_root.iter().any(|&e| e != Val::ZERO),
        "role_root is all-zero (not bound to the commitments)"
    );

    // CLASS A soundness: forge one role_root lane -> reject.
    let mut t_role = public.clone();
    t_role.role_root[0] += Val::ONE;
    assert!(
        verify_account_creation(&t_role, &proof).is_err(),
        "ROLE-RESTRICTION: a tampered role_root was accepted"
    );

    // CLASS B soundness: claim a wrong relation_id -> reject.
    let mut t_rel = public.clone();
    t_rel.relation_id[0] += Val::ONE;
    assert!(
        verify_account_creation(&t_rel, &proof).is_err(),
        "BOOKKEEPING: a wrong relation_id was accepted"
    );
}
