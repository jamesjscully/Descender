// RED-TEAM Vector 3: state-machine / lifecycle attacks on the production reference.
// Each test attempts a concrete double-spend / replay / rollback. A test that
// asserts the attack SUCCEEDS (i.e. a second consume is accepted) is a BREAK.

use descender_parameter_search::production_reference::*;

fn d(label: &str) -> [u8; DIGEST_BYTES] {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(b"redteam-state-machine");
    h.update(label.as_bytes());
    h.finalize().into()
}

/// Build a single-slot state-assisted account whose slot 0 has a valid opening
/// frame, returning (account, opening_frame, pad_seed_digest).
fn build_account() -> (
    ProductionReviewedFq12StateAssistedAccountState,
    Vec<u8>,
    [u8; DIGEST_BYTES],
) {
    let pad_seed_digest = d("pad-seed");
    let opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
        selected_source_values: [3_u16; HIDDEN_SOURCE_FALLBACK_ACCOUNT_PROJECTION_LANES],
    };
    let frame = encode_hidden_source_fallback_account_witness_opening_frame(&opening).unwrap();
    let witness_commitment = pack_reviewed_fq12_account_carried_syndrome_digest(
        &[1_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
    )
    .unwrap();
    let randomizer_commitment = pack_reviewed_fq12_account_carried_syndrome_digest(
        &[7_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
    )
    .unwrap();
    let account = hidden_source_fallback_projection_account_state_from_slot_frames([
        HiddenSourceFallbackProjectionAccountSlotFrameSeed {
            witness_commitment,
            randomizer_commitment,
            account_witness_opening_frame: frame.clone(),
            restore_anchor_digest: d("restore-anchor"),
            pad_seed_digest,
            unused: true,
        },
    ])
    .unwrap();
    (account, frame, pad_seed_digest)
}

/// ATTACK F (END-TO-END): full DB-adapter double-spend. This is the closest model
/// of a real deployment: a trusted chain DB holds (snapshot, ledger, anchor) per
/// public key, and the adapter loads them itself — the attacker only supplies the
/// spend transaction (pk, slot_id, opening frame). Spend slot 0 twice through
/// `apply_opening_frame_and_commit`; the SECOND must be rejected. Acceptance is a
/// CRITICAL production double-spend.
#[test]
fn attack_f_db_adapter_double_spend() {
    let (account, frame, _pad) = build_account();
    let manager = ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(account).unwrap();
    let snapshot = manager.snapshot().unwrap();
    let pk = ProductionPublicKey::new(vec![0_u8; PK_BYTES]).unwrap();

    let mut db = ProductionInMemorySelectedProjectionTargetChainDb::new();
    let anchor = ProductionTrustedRestoreAnchorRecord::new(
        &pk,
        snapshot.snapshot_sequence,
        PRODUCTION_TRUSTED_RESTORE_ANCHOR_CHAIN_OBSERVED_PLACEMENT,
    )
    .unwrap();
    db.set_lifecycle_snapshot(&pk, snapshot);
    db.update_trusted_anchor(anchor).unwrap();

    let mut adapter = ProductionSelectedProjectionTargetChainDbAdapter::new(db);

    let first = adapter
        .apply_opening_frame_and_commit(&pk, 0, &frame)
        .expect("first spend should commit");
    assert_eq!(first.slot_id, 0);

    let second = adapter.apply_opening_frame_and_commit(&pk, 0, &frame);
    match second {
        Ok(_) => panic!(
            "CRITICAL BREAK: end-to-end DB-adapter double-spend of slot 0 ACCEPTED across \
             two commits."
        ),
        Err(e) => println!("attack_f second commit rejected by: {}", e.reason()),
    }
}

/// ATTACK E: ledger-record tampering. The consensus path carries forward a
/// caller-supplied `Vec<ProductionSpentSlotRecord>`. If an attacker can strip the
/// `pad_seed_digest` field (set it to None) AND change the slot_id in the carried
/// record's key, the pad-seed global guard is silenced and the (pk, slot_id) key
/// no longer matches the target slot. Probe whether re-consuming slot 0 succeeds
/// when the carried spent record is rewritten to key (pk, slot 5) with pad None.
#[test]
fn attack_e_ledger_record_tamper() {
    let (account, frame, pad) = build_account();
    let manager = ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(account).unwrap();
    let snapshot = manager.snapshot().unwrap();
    let pk = ProductionPublicKey::new(vec![0_u8; PK_BYTES]).unwrap();
    let anchor = ProductionLifecycleRestoreAnchor {
        minimum_snapshot_sequence: 0,
    };

    let t1 = apply_selected_projection_consensus_opening_frame(
        snapshot.clone(),
        Vec::new(),
        anchor.clone(),
        &pk,
        0,
        &frame,
    )
    .expect("first spend");
    let _ = pad;

    // Attacker rewrites the spent record: move its key to a different slot id and
    // drop the pad_seed_digest, so neither (pk, slot 0) nor the pad guard matches.
    let mut tampered = t1.next_spent_slot_records.clone();
    for rec in &mut tampered {
        rec.key = ProductionSpentSlotLedger::key(&pk, 5);
        rec.pad_seed_digest = None;
    }

    // Replay on the ORIGINAL snapshot (slot 0 unused) with the tampered ledger.
    let replay = apply_selected_projection_consensus_opening_frame(
        snapshot, tampered, anchor, &pk, 0, &frame,
    );
    match replay {
        Ok(_) => println!(
            "WEAKNESS (trusted-input): tampered spent-ledger record (key moved to slot 5, \
             pad stripped) allowed re-consume of slot 0. The bare transition fn \
             apply_selected_projection_consensus_opening_frame TRUSTS its spent_slot_records \
             argument verbatim. Production DB adapters load these from an authenticated store \
             (commit_selected_projection_transition re-derives them), so this does NOT lift to \
             a production double-spend, but any caller passing attacker-influenced records loses \
             consume-once."
        ),
        Err(e) => println!("attack_e rejected by: {}", e.reason()),
    }
}

/// ATTACK A: cross-public-key DOUBLE-SPEND on the consensus path.
/// The opening-frame verification does not bind the public key; the spent ledger
/// is keyed on (public_key_sha256, slot_id). If the SAME lifecycle snapshot /
/// account / opening frame can be presented under two DIFFERENT public keys, the
/// same economic slot is consumed twice and the ledger never sees a collision.
#[test]
fn attack_a_cross_public_key_double_spend() {
    let (account, frame, _pad) = build_account();
    let manager = ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(account).unwrap();
    let snapshot = manager.snapshot().unwrap();
    let anchor = ProductionLifecycleRestoreAnchor {
        minimum_snapshot_sequence: 0,
    };

    let pk_a = ProductionPublicKey::new(vec![0_u8; PK_BYTES]).unwrap();
    // A second, DIFFERENT public key. Same account/snapshot/opening frame.
    let mut pk_b_bytes = vec![0_u8; PK_BYTES];
    pk_b_bytes[0] = 1;
    let pk_b = ProductionPublicKey::new(pk_b_bytes).unwrap();

    // First spend under pk_a.
    let t1 = apply_selected_projection_consensus_opening_frame(
        snapshot.clone(),
        Vec::new(),
        anchor.clone(),
        &pk_a,
        0,
        &frame,
    )
    .expect("first spend should succeed");

    // Second spend under pk_b, carrying forward pk_a's spent ledger, but on the
    // ORIGINAL (pre-spend) snapshot. The opening frame is unchanged.
    let t2 = apply_selected_projection_consensus_opening_frame(
        snapshot.clone(),
        t1.next_spent_slot_records.clone(),
        anchor.clone(),
        &pk_b,
        0,
        &frame,
    );

    match t2 {
        Ok(t2) => {
            panic!(
                "BREAK: cross-public-key double-spend ACCEPTED. \
                 same snapshot+opening frame consumed under pk_a and pk_b. \
                 t1 slot={} pad={:02x?} ; t2 slot={} pad={:02x?}",
                t1.spend_record.slot_id,
                t1.spend_record.pad_seed_digest,
                t2.spend_record.slot_id,
                t2.spend_record.pad_seed_digest
            );
        }
        Err(e) => {
            // Not a break on the consensus path; record what guard fired.
            println!("attack_a rejected by: {}", e.reason());
        }
    }
}

/// ATTACK B: ROLLBACK via anchor-free snapshot restore on the non-ledger lifecycle
/// manager. `verify_and_consume` consults NO spent ledger; it trusts only the
/// in-memory slot flag. If an attacker can feed an older snapshot (via the
/// no-anchor `from_snapshot`/`load_manager`), a consumed slot becomes unused
/// again and is re-spendable.
#[test]
fn attack_b_rollback_no_anchor_restore() {
    let (account, frame, _pad) = build_account();
    let mut manager =
        ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(account).unwrap();

    // Snapshot BEFORE the spend (slot 0 unused).
    let pre_spend = manager.snapshot().unwrap();

    // Consume slot 0.
    manager
        .verify_expanded_opening_frame_and_consume(0, &frame)
        .expect("consume should succeed");
    assert!(
        !manager.account().get_slot(0).unwrap().is_unused(),
        "slot 0 should be consumed after spend"
    );

    // ROLLBACK: restore from the pre-spend snapshot with NO restore anchor.
    let restored =
        ProductionReviewedFq12StateAssistedAccountLifecycleManager::from_snapshot(pre_spend)
            .expect("no-anchor restore accepts any snapshot");

    if restored.account().get_slot(0).unwrap().is_unused() {
        // The slot is unused again. Try to re-consume it.
        let mut restored = restored;
        let second = restored.verify_expanded_opening_frame_and_consume(0, &frame);
        match second {
            Ok(_) => println!(
                "WEAKNESS (non-production path): rollback via no-anchor from_snapshot \
                 restored a consumed slot to UNUSED and a SECOND consume was accepted on \
                 the LEDGER-FREE building-block API verify_expanded_opening_frame_and_consume. \
                 No production store flow uses this without a spent ledger, so this is NOT a \
                 production double-spend; it shows the snapshot alone is not a replay guard."
            ),
            Err(e) => println!("attack_b second consume rejected: {}", e.reason()),
        }
    } else {
        println!("attack_b: restored slot was not unused");
    }
}

/// ATTACK C (REALISTIC): rollback the LIFECYCLE SNAPSHOT only, with the persistent
/// spent ledger carried forward (the real production scenario: the snapshot and the
/// ledger are two SEPARATE files; an attacker who can revert the snapshot file
/// cannot necessarily revert the ledger). The production consensus path
/// (`apply_selected_projection_consensus_opening_frame`) must still reject because
/// the ledger remembers the spend. This test CONFIRMS the ledger is the load-bearing
/// guard; if it ever passed it would be a BREAK.
#[test]
fn attack_c_consensus_snapshot_rollback_ledger_persists() {
    let (account, frame, _pad) = build_account();
    let manager = ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(account).unwrap();
    let snapshot = manager.snapshot().unwrap();
    let pk = ProductionPublicKey::new(vec![0_u8; PK_BYTES]).unwrap();

    let t1 = apply_selected_projection_consensus_opening_frame(
        snapshot.clone(),
        Vec::new(),
        ProductionLifecycleRestoreAnchor {
            minimum_snapshot_sequence: 0,
        },
        &pk,
        0,
        &frame,
    )
    .expect("first spend");

    // Rollback the SNAPSHOT to pre-spend, but carry forward the REAL spent ledger.
    // Anchor permissive (0) so the snapshot-sequence guard does NOT catch it; the
    // ledger must be what rejects.
    let rollback = apply_selected_projection_consensus_opening_frame(
        snapshot,
        t1.next_spent_slot_records.clone(),
        ProductionLifecycleRestoreAnchor {
            minimum_snapshot_sequence: 0,
        },
        &pk,
        0,
        &frame,
    );
    match rollback {
        Ok(_) => {
            panic!("BREAK: snapshot rollback (with persistent ledger) ACCEPTED a second consume.")
        }
        Err(e) => println!("attack_c rejected by: {}", e.reason()),
    }
}

/// ATTACK D: per-public-key slot-id ledger keying. The spent ledger is keyed on
/// (sha256(public_key), slot_id). The PAD-SEED-DIGEST global check is the only thing
/// preventing a cross-public-key replay (attack A). Probe whether a DIFFERENT slot_id
/// with the SAME (still-unused) pad-seed could double-spend. Build an account with
/// the same opening frame at slot 0 and slot 1 (same pad seed) and try to consume
/// both — the pad-seed global guard should stop the second.
#[test]
fn attack_d_same_pad_seed_two_slots() {
    let pad_seed_digest = d("pad-seed-shared");
    let opening = HiddenSourceFallbackProjectionAccountWitnessOpening {
        selected_source_values: [3_u16; HIDDEN_SOURCE_FALLBACK_ACCOUNT_PROJECTION_LANES],
    };
    let frame = encode_hidden_source_fallback_account_witness_opening_frame(&opening).unwrap();
    let wc = pack_reviewed_fq12_account_carried_syndrome_digest(
        &[1_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
    )
    .unwrap();
    let rc = pack_reviewed_fq12_account_carried_syndrome_digest(
        &[7_u16; REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS],
    )
    .unwrap();
    let seed = |unused| HiddenSourceFallbackProjectionAccountSlotFrameSeed {
        witness_commitment: wc,
        randomizer_commitment: rc,
        account_witness_opening_frame: frame.clone(),
        restore_anchor_digest: d("anchor"),
        pad_seed_digest,
        unused,
    };
    // Two slots, SAME pad seed digest.
    let account =
        hidden_source_fallback_projection_account_state_from_slot_frames([seed(true), seed(true)]);
    // The account constructor itself may reject duplicate pad-seed; record either way.
    let account = match account {
        Ok(a) => a,
        Err(e) => {
            println!("attack_d: account ctor rejected duplicate pad seed: {e}");
            return;
        }
    };
    let manager = ProductionReviewedFq12StateAssistedAccountLifecycleManager::new(account).unwrap();
    let snapshot = manager.snapshot().unwrap();
    let pk = ProductionPublicKey::new(vec![0_u8; PK_BYTES]).unwrap();
    let anchor = ProductionLifecycleRestoreAnchor {
        minimum_snapshot_sequence: 0,
    };

    let t1 = apply_selected_projection_consensus_opening_frame(
        snapshot.clone(),
        Vec::new(),
        anchor.clone(),
        &pk,
        0,
        &frame,
    );
    let t1 = match t1 {
        Ok(t) => t,
        Err(e) => {
            println!("attack_d: first spend rejected: {}", e.reason());
            return;
        }
    };
    // Spend slot 1 (different ledger key) reusing pad seed — global pad guard should stop it.
    let t2 = apply_selected_projection_consensus_opening_frame(
        t1.next_lifecycle_snapshot.clone(),
        t1.next_spent_slot_records.clone(),
        anchor,
        &pk,
        1,
        &frame,
    );
    match t2 {
        Ok(_) => panic!("BREAK: two slots sharing a pad seed both consumed (slot 0 then slot 1)."),
        Err(e) => println!("attack_d second slot rejected by: {}", e.reason()),
    }
}
