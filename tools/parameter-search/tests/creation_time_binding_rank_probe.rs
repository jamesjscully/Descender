// Structural / rank probe for the creation-time account-witness binding.
//
// Question: does the masked_state18 repack-to-fixed-commitment binding make the
// account-witness map injective (closing the spend-only two-witness survivor),
// or does a residual nullity / survivor direction remain?

use descender_parameter_search::production_reference as pr;

use pr::{
    pack_reviewed_fq12_packed_witness_state18_from_parts,
    reviewed_fq12_masked_state18_packed_witness_opening,
    reviewed_fq12_pack_masked_witness_state18_from_parts,
    reviewed_fq12_packed_witness_state18_projection_masks,
    reviewed_fq12_state_assisted_opening_commitment_digest, unpack_reviewed_fq12_packed_witness_state18,
    HIDDEN_SOURCE_FALLBACK_ACCOUNT_PROJECTION_LANES as LANES, Q,
    REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS as ROWS, REVIEWED_FQ12_PACKED_WITNESS_STATE18_ROWS,
    REVIEWED_FQ12_PAD_SEED_BYTES, DIGEST_BYTES,
};

// Probe 1: the 18-residue packing is a bijection (invertible bit-packing, not a
// hiding/binding hash). So the witness commitment fully determines all 18
// residues, and any change to any residue changes the commitment.
#[test]
fn probe1_state18_packing_is_a_bijection() {
    let mut syndrome = [0u16; ROWS];
    let mut proj = [0u16; LANES];
    for i in 0..ROWS {
        syndrome[i] = ((i * 911 + 7) % Q as usize) as u16;
    }
    for i in 0..LANES {
        proj[i] = ((i * 4099 + 100) % Q as usize) as u16;
    }
    let commit = pack_reviewed_fq12_packed_witness_state18_from_parts(&syndrome, &proj).unwrap();
    let residues = unpack_reviewed_fq12_packed_witness_state18(&commit).unwrap();
    for i in 0..ROWS {
        assert_eq!(residues[i], syndrome[i]);
    }
    for i in 0..LANES {
        assert_eq!(residues[ROWS + i], proj[i]);
    }
    // Flip one residue -> commitment must change (injectivity of pack).
    let mut proj2 = proj;
    proj2[0] = (proj2[0] + 1) % Q;
    let commit2 = pack_reviewed_fq12_packed_witness_state18_from_parts(&syndrome, &proj2).unwrap();
    assert_ne!(commit, commit2);
    let mut syn2 = syndrome;
    syn2[3] = (syn2[3] + 1) % Q;
    let commit3 = pack_reviewed_fq12_packed_witness_state18_from_parts(&syn2, &proj).unwrap();
    assert_ne!(commit, commit3);
}

// Probe 2: masking is an affine bijection on each of the 6 projection lanes for
// fixed (pad_seed, slot_id). Given the same commitment, the recovered
// account_projection is unique. Different account_projection -> different
// commitment (after masking).
#[test]
fn probe2_masking_is_affine_bijection_per_lane() {
    let pad_seed = [0x5au8; REVIEWED_FQ12_PAD_SEED_BYTES];
    let slot_id = 7u64;
    let masks = reviewed_fq12_packed_witness_state18_projection_masks(&pad_seed, slot_id);
    let syndrome = [1u16; ROWS];
    let proj_a = [123u16, 456, 789, 1011, 1213, 1415];
    let mut proj_b = proj_a;
    proj_b[2] = (proj_b[2] + 1) % Q; // move one account-witness coordinate

    let commit_a =
        reviewed_fq12_pack_masked_witness_state18_from_parts(&syndrome, &proj_a, &pad_seed, slot_id)
            .unwrap();
    let commit_b =
        reviewed_fq12_pack_masked_witness_state18_from_parts(&syndrome, &proj_b, &pad_seed, slot_id)
            .unwrap();
    // distinct account projections -> distinct commitments (mask is additive bijection)
    assert_ne!(commit_a, commit_b);

    // Recovery is exact and unique.
    let open_a =
        reviewed_fq12_masked_state18_packed_witness_opening(&commit_a, &pad_seed, slot_id).unwrap();
    assert_eq!(open_a.account_projection, proj_a);
    assert_eq!(open_a.witness_syndrome, syndrome);
    assert_eq!(&open_a.repacked_witness_commitment, &commit_a);
    // mask cancels: masked = account + mask (mod q)
    for i in 0..LANES {
        assert_eq!(
            open_a.masked_projection[i],
            ((proj_a[i] as u32 + masks[i] as u32) % Q as u32) as u16
        );
    }
}

// Probe 3: THE SURVIVOR TEST. Replicate the spend-only negative control's move:
// two DIFFERENT account-witness commitments. In the spend-only path both
// accepted against the SAME opening frame + opening_commitment because the
// witness commitment was free. Here we check whether the masked_state18 opening
// + opening_commitment fixes the witness commitment so that the two witnesses
// CANNOT share an opening_commitment (the leaf-fixed binding).
//
// We compute the opening_commitment digest that the leaf fixes. The opening
// frame embeds account_witness_commitment directly. So two distinct witness
// commitments -> two distinct opening_commitments -> two distinct leaves.
#[test]
fn probe3_two_witnesses_cannot_share_opening_commitment() {
    // We cannot easily construct the full ProductionReviewedFq12StateAssistedOpening
    // here without many fields, so we exercise the algebra that the leaf binds:
    // opening_commitment = SHA256(frame containing account_witness_commitment).
    //
    // Instead, drive it through the documented frame: build two commitments and
    // confirm the opening-commitment digest differs. We use a thin opening built
    // from the public constructor if available; otherwise we assert at the
    // commitment level which is what opening_commitment hashes.
    let pad_seed = [0x5au8; REVIEWED_FQ12_PAD_SEED_BYTES];
    let slot_id = 0u64;

    // Spend-only survivor used syndrome_a = 0, syndrome_b[0]=1, projection equal.
    let syndrome_a = [0u16; ROWS];
    let mut syndrome_b = [0u16; ROWS];
    syndrome_b[0] = 1;
    let proj = [3u16, 5, 8, 13, 21, 34]; // the exact selected_source_values from the 2205 test

    let commit_a =
        reviewed_fq12_pack_masked_witness_state18_from_parts(&syndrome_a, &proj, &pad_seed, slot_id)
            .unwrap();
    let commit_b =
        reviewed_fq12_pack_masked_witness_state18_from_parts(&syndrome_b, &proj, &pad_seed, slot_id)
            .unwrap();
    assert_ne!(
        commit_a, commit_b,
        "distinct syndromes must yield distinct witness commitments"
    );

    // Now: the opening_commitment frame embeds account_witness_commitment. So
    // distinct commitments => distinct opening_commitment digests, even when the
    // 6 projection lanes / witness_opening_digest are identical.
    // Build two openings differing only in account_witness_commitment.
    let oc_a = opening_commitment_for(commit_a);
    let oc_b = opening_commitment_for(commit_b);
    assert_ne!(
        oc_a, oc_b,
        "SURVIVOR CLOSED: distinct witness commitments give distinct opening_commitments"
    );
}

// Helper: compute opening_commitment digest with everything fixed except the
// account_witness_commitment, mirroring the leaf-fixed binding.
fn opening_commitment_for(witness_commitment: [u8; DIGEST_BYTES]) -> [u8; DIGEST_BYTES] {
    let opening = pr::ProductionReviewedFq12StateAssistedOpening {
        slot_id: 0,
        account_witness_commitment: witness_commitment,
        account_randomizer_commitment: [1u8; DIGEST_BYTES],
        restore_anchor_digest: [2u8; DIGEST_BYTES],
        witness_opening_digest: [3u8; DIGEST_BYTES],
        pad_seed_digest: [4u8; DIGEST_BYTES],
        carrier_handle_secret_digest: [5u8; DIGEST_BYTES],
    };
    reviewed_fq12_state_assisted_opening_commitment_digest(&opening)
}

// Probe 5: end-to-end run of the actual masked_state18 packed witness opening
// verifier. Confirm (a) it ACCEPTS when statement.witness_commitment ==
// opening.account_witness_commitment and the commitment repacks; (b) it REJECTS
// when the statement commitment is swapped to a second witness (the spend-only
// survivor move). This is the binding the spend-only carrier path lacked.
#[test]
fn probe5_masked_state18_verifier_rejects_witness_swap() {
    let pad_seed = [0x5au8; REVIEWED_FQ12_PAD_SEED_BYTES];
    let slot_id: u64 = 0;
    let observed_blocks = [false; pr::GROUPS];
    let pad_opening_statement = pr::reviewed_fq12_pad_opening_statement_from_seed(&pad_seed, &observed_blocks);

    let syndrome_a = [0u16; ROWS];
    let mut syndrome_b = [0u16; ROWS];
    syndrome_b[0] = 1;
    let proj = [3u16, 5, 8, 13, 21, 34];

    let commit_a =
        reviewed_fq12_pack_masked_witness_state18_from_parts(&syndrome_a, &proj, &pad_seed, slot_id)
            .unwrap();
    let commit_b =
        reviewed_fq12_pack_masked_witness_state18_from_parts(&syndrome_b, &proj, &pad_seed, slot_id)
            .unwrap();
    assert_ne!(commit_a, commit_b);

    // Build the matching opening + statement for witness A.
    let build = |witness_commitment: [u8; DIGEST_BYTES]| {
        // witness_opening_digest must equal digest of the recovered account
        // projection frame for THIS commitment.
        let open = reviewed_fq12_masked_state18_packed_witness_opening(
            &witness_commitment,
            &pad_seed,
            slot_id,
        )
        .unwrap();
        let proj_open = pr::HiddenSourceFallbackProjectionAccountWitnessOpening {
            selected_source_values: open.account_projection,
        };
        let frame =
            pr::encode_hidden_source_fallback_account_witness_opening_frame(&proj_open).unwrap();
        let wod = pr::reviewed_fq12_account_witness_projection_opening_digest(&frame).unwrap();
        let opening = pr::ProductionReviewedFq12StateAssistedOpening {
            slot_id,
            account_witness_commitment: witness_commitment,
            account_randomizer_commitment: [1u8; DIGEST_BYTES],
            restore_anchor_digest: [2u8; DIGEST_BYTES],
            witness_opening_digest: wod,
            pad_seed_digest: pr::reviewed_fq12_pad_seed_opening_digest(&pad_seed),
            carrier_handle_secret_digest: [5u8; DIGEST_BYTES],
        };
        let mut statement = pr::ProductionReviewedFq12HiddenSourceStatement {
            digest: [0u8; DIGEST_BYTES],
            slot_id: slot_id as u32,
            witness_commitment,
            randomizer_commitment: [1u8; DIGEST_BYTES],
            public_key_digest: [7u8; DIGEST_BYTES],
            message_digest: [8u8; DIGEST_BYTES],
            signature_digest: [9u8; DIGEST_BYTES],
            pad_opening_statement,
        };
        statement.digest = pr::reviewed_fq12_hidden_source_statement_digest(&statement);
        (opening, statement)
    };

    let (opening_a, statement_a) = build(commit_a);
    // (a) honest accept for A.
    let ok = pr::verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening(
        &opening_a,
        &statement_a,
        &pad_seed,
    );
    assert!(ok.is_ok(), "honest masked_state18 opening must accept: {ok:?}");

    // (b) SURVIVOR MOVE: keep opening_a (bound to commit_a) but swap the
    // statement's witness commitment to commit_b. Spend-only carrier ignored
    // this; masked_state18 must reject.
    let mut statement_swapped = statement_a;
    statement_swapped.witness_commitment = commit_b;
    statement_swapped.digest = pr::reviewed_fq12_hidden_source_statement_digest(&statement_swapped);
    let swapped =
        pr::verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening(
            &opening_a,
            &statement_swapped,
            &pad_seed,
        );
    assert!(
        swapped.is_err(),
        "SURVIVOR: masked_state18 accepted a witness-commitment swap"
    );

    // (c) Also: swap opening.account_witness_commitment to commit_b while leaving
    // witness_opening_digest bound to A's projection -> repack/digest mismatch.
    let mut opening_swapped = opening_a;
    opening_swapped.account_witness_commitment = commit_b;
    let mut statement_b_match = statement_a;
    statement_b_match.witness_commitment = commit_b;
    statement_b_match.digest = pr::reviewed_fq12_hidden_source_statement_digest(&statement_b_match);
    let swapped2 =
        pr::verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening(
            &opening_swapped,
            &statement_b_match,
            &pad_seed,
        );
    // commit_a and commit_b share the same projection lanes (proj), so the
    // recovered account_projection is identical -> witness_opening_digest still
    // matches. Report whether the syndrome difference is caught.
    eprintln!("opening+statement both swapped to commit_b: {swapped2:?}");
}

// Probe 4: report the dimensions and the constraint count concretely.
#[test]
fn probe4_report_dimensions() {
    eprintln!("=== creation-time binding dimensions ===");
    eprintln!("q = {}", Q);
    eprintln!("checked-core syndrome rows (ROWS) = {}", ROWS);
    eprintln!("account-projection lanes (LANES)  = {}", LANES);
    eprintln!(
        "state18 total residues            = {}",
        REVIEWED_FQ12_PACKED_WITNESS_STATE18_ROWS
    );
    eprintln!(
        "witness-commitment packed bits    = {} (18*14)",
        REVIEWED_FQ12_PACKED_WITNESS_STATE18_ROWS * 14
    );
    eprintln!("digest bits                       = {}", DIGEST_BYTES * 8);
    assert_eq!(REVIEWED_FQ12_PACKED_WITNESS_STATE18_ROWS, ROWS + LANES);
    assert_eq!(REVIEWED_FQ12_PACKED_WITNESS_STATE18_ROWS, 18);
}
