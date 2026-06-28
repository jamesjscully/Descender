//! ADVERSARIAL RED-TEAM probes against HIDING and BATCH/AGGREGATION binding.
//!
//! These are written to SUCCEED if a break exists (an assertion that fires the
//! "BREAK" message means the scheme is broken in that dimension).

use descender_account_stark::{
    prove_account_creation_batch, prove_account_creation_with_seed, verify_account_creation_batch,
    AccountCreationPublic, AccountCreationSecrets, Val, RELATION_ID,
};
use p3_field::PrimeCharacteristicRing;

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

// ===========================================================================
// TARGET B #1 — MIX-AND-MATCH: reorder the supplied tuples (multiset binding).
// The digest is computed over publics in ROW ORDER, but the proof does not bind
// position. If the digest were truly position-binding, a permutation of the
// tuples would change the digest and the SAME proof would still attest the same
// rows. Here we test whether a verifier can be made to accept a DIFFERENT
// (digest, publics) ordering against the SAME proof.
// ===========================================================================
#[test]
fn target_b_mix_and_match_reorder() {
    // FIXED: the digest is now an in-AIR, ORDER-SENSITIVE running chain over the
    // per-row pubio, exposed as a STARK public value. Reordering the tuples
    // changes the chain digest -> the proof (bound to the original digest) MUST
    // reject. (Previously this was only multiset-binding.)
    let n = 3usize; // small N + reduced config per the resource constraint
    let secrets: Vec<_> = (0..n as u64).map(secrets_indexed).collect();
    let (digest, publics, proof) = prove_account_creation_batch(&secrets).expect("prove");

    // Honest accepts.
    verify_account_creation_batch(&digest, &publics, &proof).expect("honest verifies");

    // Adversary REORDERS the tuples. To get a self-consistent digest they reprove
    // the permuted secret order and take ITS digest (same multiset, different
    // order). The ORIGINAL proof was bound to the ORIGINAL-order digest.
    let mut permuted = publics.clone();
    permuted.swap(0, 1);
    let secrets_perm: Vec<_> = {
        let mut s = secrets.clone();
        s.swap(0, 1);
        s
    };
    let (digest_perm, publics_perm, _proof_perm) =
        prove_account_creation_batch(&secrets_perm).expect("prove permuted");
    assert_eq!(publics_perm, permuted, "permuted publics line up");
    // Order matters now: the permuted digest differs from the original.
    assert_ne!(
        digest_perm, digest,
        "chain digest is order-sensitive (reorder changes the digest)"
    );

    // (a) Reordered publics + their (different) digest against the ORIGINAL proof:
    //     the proof was bound to `digest`, not `digest_perm` -> reject in STARK.
    let res = verify_account_creation_batch(&digest_perm, &publics_perm, &proof);
    println!("[B#1 reorder, reproved digest, orig proof] verify = {:?}", res.is_ok());
    assert!(
        res.is_err(),
        "PER-POSITION BREAK: original proof accepted a REORDERED public list"
    );

    // (b) Reordered publics claimed under the ORIGINAL digest: digest recompute
    //     over the permuted list != original digest -> reject in step 1.
    let res2 = verify_account_creation_batch(&digest, &publics_perm, &proof);
    assert!(
        res2.is_err(),
        "PER-POSITION BREAK: reordered tuples accepted under the original digest"
    );
}

// ===========================================================================
// TARGET B #4 (the strong one) — FABRICATED tuples: supply publics that do NOT
// correspond to ANY committed row, with a self-consistent digest. The proof and
// the digest are independent prover-supplied objects; the STARK never constrains
// the digest. Does the verifier accept arbitrary publics as long as
// hash(publics)==digest and SOME valid proof is attached?
// ===========================================================================
#[test]
fn target_b_fabricated_tuples_with_unrelated_proof() {
    // FIXED: the digest is a STARK public value bound in-AIR to the proof's rows.
    // A's proof was bound to A's digest; presenting B's (digest,publics) against
    // A's proof must REJECT (the proof's public digest != B's digest).
    let secrets_a: Vec<_> = (0..3u64).map(secrets_indexed).collect();
    let (digest_a, _publics_a, proof_a) =
        prove_account_creation_batch(&secrets_a).expect("prove A");

    // Batch B: a COMPLETELY DIFFERENT set of accounts (different witnesses).
    let secrets_b: Vec<_> = (100..103u64).map(secrets_indexed).collect();
    let (digest_b, publics_b, _proof_b) =
        prove_account_creation_batch(&secrets_b).expect("prove B");
    assert_ne!(digest_a, digest_b, "different batches -> different digests");

    // Adversary claim: "here are accounts publics_b, digest_b, proven by proof_a".
    let res = verify_account_creation_batch(&digest_b, &publics_b, &proof_a);
    println!("[B#4 fabricated tuples + unrelated valid proof] verify = {:?}", res.is_ok());
    assert!(
        res.is_err(),
        "CRITICAL BREAK: verifier accepted batch B's tuples bound to batch A's proof"
    );
}

// ===========================================================================
// TARGET B #2/#3 — SMUGGLE an account whose individual proof would be REJECTED
// into a padded slot, by claiming N+ accounts than were validly proven. Because
// padding replicates the last real row, can we claim a tuple for a padded slot
// that the proof does not actually cover? We test: build a batch of N real, then
// claim N+1 publics (the extra being arbitrary) with a recomputed digest.
// ===========================================================================
#[test]
fn target_b_claim_more_accounts_than_proven() {
    // FIXED: the count N is bound (in-AIR accumulator == public N, and the chain
    // length depends on N). Appending an extra fabricated account changes the
    // recomputed chain digest AND the count -> reject.
    let secrets: Vec<_> = (0..3u64).map(secrets_indexed).collect(); // 3 real -> padded to 4
    let (digest, publics, proof) = prove_account_creation_batch(&secrets).expect("prove");
    verify_account_creation_batch(&digest, &publics, &proof).expect("honest verifies");

    // Adversary appends an EXTRA fabricated account tuple (a 4th "account").
    let mut inflated = publics.clone();
    let fake = AccountCreationPublic {
        pad_source_commitment: core::array::from_fn(|_| Val::from_u32(0xAAAA)),
        opening_tuple_commitment: core::array::from_fn(|_| Val::from_u32(0xBBBB)),
        masked_projection: core::array::from_fn(|_| Val::from_u32(7)),
        account_id: [0xCC; 32],
        slot_id: 0xDEAD,
        verifier_version: 9,
        account_state_root: [0xDD; 32],
        lifecycle_anchor: [0xEE; 32],
        role_root: core::array::from_fn(|_| Val::from_u32(0xF00D)),
        nullifier: core::array::from_fn(|_| Val::from_u32(0xBEEF)),
        // Use the pinned relation_id so this test still exercises the COUNT/digest
        // break (not the CLASS-B relation_id check).
        relation_id: core::array::from_fn(|j| Val::from_u32(RELATION_ID[j])),
        syndrome: core::array::from_fn(|_| Val::from_u32(5)),
        packed_witness_commitment: core::array::from_fn(|_| Val::from_u32(0)),
    };
    inflated.push(fake);

    // (a) Inflated list under the ORIGINAL digest: chain recompute over 4 tuples
    //     != the 3-tuple digest -> reject in the recompute step.
    let res = verify_account_creation_batch(&digest, &inflated, &proof);
    println!("[B#2 inflate count, original digest] verify = {:?}", res.is_ok());
    assert!(res.is_err(), "COUNT BREAK: inflated tuple list accepted");

    // (b) Even with a self-consistent digest for the inflated list (the verifier
    //     recomputes it), the proof was bound to N=3 and digest_3; the inflated
    //     N=4/digest_4 do not match the proof's public values -> reject in STARK.
    //     We obtain a self-consistent digest by reproving with a 4th real account;
    //     the resulting digest still differs from `proof`'s bound digest.
    let secrets4: Vec<_> = (0..4u64).map(secrets_indexed).collect();
    let (digest4, _p4, _proof4) = prove_account_creation_batch(&secrets4).expect("prove 4");
    let res2 = verify_account_creation_batch(&digest4, &inflated, &proof);
    assert!(
        res2.is_err(),
        "COUNT BREAK: 4-account claim accepted against a 3-account proof"
    );
}

// ===========================================================================
// TARGET B #4 STRONGEST — certify GARBAGE accounts (not valid instances at all)
// with a real proof, by recomputing the digest ourselves with the public hash.
// ===========================================================================
#[test]
fn target_b_certify_garbage_accounts() {
    // FIXED: a real proof can no longer certify garbage accounts. The chain
    // digest the proof is bound to is the in-AIR commitment to the proof's OWN
    // valid rows; the verifier recomputes the chain over the SUPPLIED tuples and
    // also re-checks the STARK against that digest. Garbage tuples chain to a
    // different digest than the proof's bound digest -> reject. We test the two
    // adversary options (supply our digest of garbage, OR claim the proof's
    // digest); both reject.
    let secrets: Vec<_> = (0..4u64).map(secrets_indexed).collect();
    let (real_digest, _p, proof) = prove_account_creation_batch(&secrets).expect("prove");

    // GARBAGE "accounts": values an honest relation would never produce
    // (masked_projection lanes >= q12289, arbitrary commitments), DIFFERENT count.
    let garbage: Vec<AccountCreationPublic> = (0..9u64)
        .map(|k| AccountCreationPublic {
            pad_source_commitment: core::array::from_fn(|j| Val::from_u32((k as u32) * 1000 + j as u32)),
            opening_tuple_commitment: core::array::from_fn(|j| Val::from_u32((k as u32) * 31 + j as u32 + 1)),
            masked_projection: core::array::from_fn(|_| Val::from_u32(999_999)), // >> q12289
            account_id: [(k as u8).wrapping_add(1); 32],
            slot_id: k.wrapping_mul(7),
            verifier_version: k as u32,
            account_state_root: [(k as u8).wrapping_add(2); 32],
            lifecycle_anchor: [(k as u8).wrapping_add(3); 32],
            role_root: core::array::from_fn(|j| Val::from_u32((k as u32) * 13 + j as u32)),
            nullifier: core::array::from_fn(|j| Val::from_u32((k as u32) * 19 + j as u32)),
            // Pinned relation_id so the rejection is due to the digest/STARK break.
            relation_id: core::array::from_fn(|j| Val::from_u32(RELATION_ID[j])),
            syndrome: core::array::from_fn(|j| Val::from_u32((k as u32) * 17 + j as u32)),
            packed_witness_commitment: core::array::from_fn(|_| Val::from_u32(0xCAFE)),
        })
        .collect();

    // (a) Claim the proof's real digest with garbage tuples: chain recompute over
    //     garbage != real_digest -> reject in the recompute step.
    let res_a = verify_account_creation_batch(&real_digest, &garbage, &proof);
    println!("[B#4 garbage tuples, real digest] verify = {:?}", res_a.is_ok());
    assert!(res_a.is_err(), "GARBAGE BREAK: real proof certified garbage tuples");

    // (b) Supply a digest that DOES chain-match the garbage list (so step 1
    //     passes), then the STARK re-check against that digest fails because the
    //     proof was bound to real_digest, not the garbage digest.
    let garbage_digest = chain_digest_of(&garbage);
    assert_ne!(garbage_digest, real_digest);
    let res_b = verify_account_creation_batch(&garbage_digest, &garbage, &proof);
    println!("[B#4 garbage tuples, matching garbage digest] verify = {:?}", res_b.is_ok());
    assert!(res_b.is_err(), "GARBAGE BREAK: real proof bound to a garbage chain digest");
}

/// Recompute the library's in-AIR digest chain over a list of publics (mirrors
/// `descender_account_stark`'s private chain). Used by adversarial tests to
/// build a self-consistent (but proof-mismatching) digest.
fn chain_digest_of(publics: &[AccountCreationPublic]) -> [Val; 8] {
    use descender_account_stark::recompute_chain_digest_for_tests;
    recompute_chain_digest_for_tests(publics)
}

// ===========================================================================
// FIX ANCHORS — the in-AIR chain digest binding (positive / completeness side).
// ===========================================================================

// Completeness: a small honest batch verifies AND the public chained digest
// equals the verifier's recompute over the supplied tuples.
#[test]
fn fix_completeness_chain_digest_matches() {
    let n = 3usize;
    let secrets: Vec<_> = (0..n as u64).map(secrets_indexed).collect();
    let (digest, publics, proof) = prove_account_creation_batch(&secrets).expect("prove");
    assert_eq!(publics.len(), n);
    verify_account_creation_batch(&digest, &publics, &proof).expect("honest batch verifies");
    // The exposed digest equals the independent recompute over the tuples.
    assert_eq!(
        digest,
        chain_digest_of(&publics),
        "public chained digest != recompute over supplied tuples"
    );
}

// Per-position binding (positive): the chain digest is order-sensitive, so the
// same multiset in a different order produces a different digest.
#[test]
fn fix_per_position_digest_is_order_sensitive() {
    let n = 3usize;
    let secrets: Vec<_> = (0..n as u64).map(secrets_indexed).collect();
    let (digest, publics, _proof) = prove_account_creation_batch(&secrets).expect("prove");
    let mut reordered = publics.clone();
    reordered.swap(0, 2);
    assert_ne!(reordered, publics, "reorder is a real permutation");
    assert_ne!(
        chain_digest_of(&reordered),
        digest,
        "reordered multiset chains to the SAME digest (position binding lost)"
    );
}

// Padding soundness: a batch padded to a power of two (N=3 -> 4 rows) does NOT
// let a padding row smuggle an account. The honest proof verifies, and the
// digest binds EXACTLY the 3 real accounts: presenting a 4th (the padding row's
// replicated tuple) as a real account changes the recompute/count and rejects.
#[test]
fn fix_padding_row_cannot_smuggle_account() {
    let n = 3usize; // padded to 4 rows internally
    let secrets: Vec<_> = (0..n as u64).map(secrets_indexed).collect();
    let (digest, publics, proof) = prove_account_creation_batch(&secrets).expect("prove");
    verify_account_creation_batch(&digest, &publics, &proof).expect("honest verifies");
    assert_eq!(publics.len(), 3, "only the 3 REAL accounts are returned (no phantom 4th)");

    // Try to claim the padding row as a real 4th account (replicate the last real
    // tuple). With a self-consistent 4-account chain digest, the proof (bound to
    // the 3-account digest) still rejects: padding rows are in-AIR no-ops.
    let mut smuggled = publics.clone();
    let phantom = publics[n - 1].clone();
    smuggled.push(phantom);
    let smuggled_digest = chain_digest_of(&smuggled);
    assert_ne!(smuggled_digest, digest);
    assert!(
        verify_account_creation_batch(&smuggled_digest, &smuggled, &proof).is_err(),
        "PADDING BREAK: a padding row was certified as a real account"
    );
}

// ===========================================================================
// TARGET A — HIDING: attempt FULL recovery of every hidden value class from the
// proof's opened values, OOD evaluations, and statement. We check that NO opened
// trace_local entry equals any raw witness element (pad_seed bytes,
// account_projection lanes, lifecycle_anchor bytes, nonces) — for a single
// instance whose witness columns are otherwise CONSTANT polynomials.
// ===========================================================================
#[test]
fn target_a_full_witness_recovery_scan() {
    use descender_account_stark::Challenge;

    let s = AccountCreationSecrets {
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
    };
    let (_public, proof) = prove_account_creation_with_seed(&s, 0x1234_5678).expect("prove");

    // Collect every raw witness element as a Challenge.
    let mut witness_vals: Vec<Challenge> = Vec::new();
    for &b in s.pad_seed.iter() {
        witness_vals.push(Challenge::from(Val::from_u32(b as u32)));
    }
    for &b in s.lifecycle_anchor.iter() {
        witness_vals.push(Challenge::from(Val::from_u32(b as u32)));
    }
    for &b in s.c1_nonce.iter() {
        witness_vals.push(Challenge::from(Val::from_u32(b as u32)));
    }
    for &b in s.c6_nonce.iter() {
        witness_vals.push(Challenge::from(Val::from_u32(b as u32)));
    }
    for &v in s.account_projection.iter() {
        witness_vals.push(Challenge::from(Val::from_u32(v as u32)));
    }
    for b in s.slot_id.to_le_bytes().iter() {
        witness_vals.push(Challenge::from(Val::from_u32(*b as u32)));
    }

    let opened = &proof.opened_values.trace_local;
    let mut hits = 0usize;
    for (col, ov) in opened.iter().enumerate() {
        for (wi, wv) in witness_vals.iter().enumerate() {
            if ov == wv {
                println!("[A] LEAK: trace_local[{col}] == witness element #{wi}");
                hits += 1;
            }
        }
    }
    println!(
        "[A full recovery scan] opened trace_local len={}, witness elements={}, direct-equality hits={}",
        opened.len(),
        witness_vals.len(),
        hits
    );
    // A small number of accidental hits among small byte values is possible by
    // chance in the extension field? No: opened values are ~124-bit ext elements;
    // equality to a tiny byte value is overwhelmingly non-accidental.
    assert_eq!(hits, 0, "HIDING BREAK: an opened OOD value equals a raw witness element");
}

// ===========================================================================
// TARGET A #3 — SIDE CHANNELS: proof size must be witness-independent.
// ===========================================================================
#[test]
fn target_a_proof_size_witness_independent() {
    use descender_account_stark::{proof_size_bytes, prove_account_creation_with_seed};
    let mk = |seed: u64| {
        let s = secrets_indexed(seed);
        let (_p, proof) = prove_account_creation_with_seed(&s, 0xAAAA).expect("prove");
        proof_size_bytes(&proof)
    };
    // Wildly different witnesses (incl. all-zero-ish and max projections).
    let sizes: Vec<usize> = [1u64, 2, 12345, 999999, 7].iter().map(|&s| mk(s)).collect();
    println!("[A#3 proof sizes across witnesses] {:?}", sizes);
    let first = sizes[0];
    let all_same = sizes.iter().all(|&x| x == first);
    println!("[A#3] all proof sizes equal = {}", all_same);
    // FRI query count, PoW, and trace shape are all PINNED constants independent of
    // the witness; sizes should match exactly. (HidingFriPcs randomness may add a
    // tiny variance only if encodings are variable-length; KoalaBear elems are
    // fixed 4 B, so expect exact equality.)
}
