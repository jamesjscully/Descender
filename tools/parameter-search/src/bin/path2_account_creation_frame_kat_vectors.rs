use descender_parameter_search::production_reference::{
    DIGEST_BYTES, PATH2_ACCOUNT_CREATION_FRAME_KAT_BOUNDARY_ROWS,
    PATH2_EVIDENCE_KIND_DIRECT_REPLAYABLE_RELATION_CERTIFICATE, PATH2_SLOT_PREVALIDATE_BATCH_DOMAIN,
    Path2AccountCreationEvidenceV1, Path2PrevalidatedSlotLeafV1, PROFILE,
    path2_account_creation_frame_kat_boundary_report_v1,
    path2_account_creation_frame_public_rejection_class,
    path2_account_creation_frame_kat_fixture_v1, path2_prevalidated_slot_leaf_digest,
    path2_slot_authorization_digest, verify_path2_prevalidated_slot_creation_frame_bindings,
    verify_path2_prevalidated_slot_creation_payload_bindings,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn flip_first_digest_byte(digest: &mut [u8; DIGEST_BYTES]) {
    digest[0] ^= 1;
}

fn refresh_leaf_dependent_evidence(
    leaf: &Path2PrevalidatedSlotLeafV1,
    evidence: &mut Path2AccountCreationEvidenceV1,
) -> Result<(), String> {
    evidence.slot_leaf_digest = path2_prevalidated_slot_leaf_digest(leaf)?;
    evidence.slot_authorization_digest = path2_slot_authorization_digest(leaf)?;
    Ok(())
}

fn mutated_case(case_id: &str) -> Result<(String, &'static str), String> {
    let fixture = path2_account_creation_frame_kat_fixture_v1()?;
    let mut leaf = fixture.leaf;
    let mut evidence = fixture.evidence;
    match case_id {
        "leaf_frame_version_mutation" => {
            leaf.frame_version ^= 1;
        }
        "leaf_live_status_marker_mutation" => {
            leaf.live_status_marker = 0;
        }
        "evidence_domain_mutation" => {
            evidence.evidence_domain = PATH2_SLOT_PREVALIDATE_BATCH_DOMAIN;
        }
        "evidence_version_mutation" => {
            evidence.evidence_version += 1;
        }
        "evidence_kind_mutation" => {
            evidence.evidence_kind = PATH2_EVIDENCE_KIND_DIRECT_REPLAYABLE_RELATION_CERTIFICATE + 1;
        }
        "slot_leaf_digest_mutation" => {
            flip_first_digest_byte(&mut evidence.slot_leaf_digest);
        }
        "slot_authorization_digest_mutation" => {
            flip_first_digest_byte(&mut evidence.slot_authorization_digest);
        }
        "creation_verifier_version_digest_mutation" => {
            flip_first_digest_byte(&mut evidence.creation_verifier_version_digest);
        }
        "hidden_source_statement_digest_mutation" => {
            flip_first_digest_byte(&mut evidence.hidden_source_statement_digest);
        }
        "opening_commitment_mutation" => {
            flip_first_digest_byte(&mut evidence.opening_commitment);
        }
        "pad_seed_digest_mutation" => {
            flip_first_digest_byte(&mut evidence.pad_seed_digest);
        }
        "account_identity_digest_mutation" => {
            flip_first_digest_byte(&mut evidence.account_identity_digest);
        }
        "public_key_identity_mutation" => {
            flip_first_digest_byte(&mut evidence.public_key_identity);
        }
        "slot_id_mutation" => {
            evidence.slot_id += 1;
        }
        "state_root_before_mutation" => {
            flip_first_digest_byte(&mut evidence.state_root_before);
        }
        "lifecycle_anchor_mutation" => {
            flip_first_digest_byte(&mut evidence.lifecycle_anchor);
        }
        "prevalidation_transcript_digest_mutation" => {
            flip_first_digest_byte(&mut leaf.prevalidation_transcript_digest);
            refresh_leaf_dependent_evidence(&leaf, &mut evidence)?;
        }
        other => return Err(format!("unknown account-creation frame KAT case {other}")),
    }
    let observed = verify_path2_prevalidated_slot_creation_frame_bindings(&leaf, &evidence)
        .expect_err("mutated frame KAT case must reject");
    let public_class = path2_account_creation_frame_public_rejection_class(&observed);
    Ok((observed, public_class))
}

fn manifest_body() -> Result<Value, String> {
    let report = path2_account_creation_frame_kat_boundary_report_v1();
    let fixture = path2_account_creation_frame_kat_fixture_v1()?;
    verify_path2_prevalidated_slot_creation_payload_bindings(
        &fixture.leaf,
        &fixture.evidence,
        Some(&fixture.certificate),
    )?;
    let mut cases = vec![json!({
        "case_id": "honest_frame_accepts",
        "kind": "accept",
        "leaf_digest": hex_encode(&fixture.leaf_digest),
        "slot_authorization_digest": hex_encode(&fixture.slot_authorization_digest),
        "evidence_transcript_digest": hex_encode(&fixture.evidence_transcript_digest),
        "certificate_digest": hex_encode(&fixture.certificate_digest)
    })];
    for row in PATH2_ACCOUNT_CREATION_FRAME_KAT_BOUNDARY_ROWS {
        let (observed_error, public_rejection_class) = mutated_case(row.case_id)?;
        if !observed_error.contains(row.internal_error_match) {
            return Err(format!(
                "frame KAT case {} observed error {observed_error:?} missing marker {:?}",
                row.case_id, row.internal_error_match
            ));
        }
        if public_rejection_class != row.public_rejection_class {
            return Err(format!(
                "frame KAT case {} public class {public_rejection_class} != {}",
                row.case_id, row.public_rejection_class
            ));
        }
        cases.push(json!({
            "case_id": row.case_id,
            "kind": "reject",
            "requirement": row.requirement,
            "verifier_endpoint": row.verifier_endpoint,
            "mutated_surface": row.mutated_surface,
            "observed_error": observed_error,
            "internal_error_match": row.internal_error_match,
            "public_rejection_class": public_rejection_class
        }));
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountCreationFrameKatVectors.v1",
        "corpus_id": "Descender.Path2.AccountCreationFrameKatBoundaryVectors.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "status": "digest_pinned_boundary_vectors",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_creation_frame_kat_vectors",
            "source": "tools/parameter-search/src/bin/path2_account_creation_frame_kat_vectors.rs"
        },
        "fixture": {
            "fixture_id": fixture.fixture_id,
            "selected_leaf_type": report.selected_leaf_type,
            "selected_evidence_type": report.selected_evidence_type,
            "selected_certificate_type": report.selected_certificate_type
        },
        "boundary_report": {
            "domain": report.domain,
            "digest": hex_encode(&report.digest),
            "row_count": report.row_count,
            "every_current_row_has_generated_vector": true,
            "deployment_ready": report.deployment_ready,
            "blocker": report.blocker
        },
        "case_count": cases.len(),
        "rejection_case_count": cases.len() - 1,
        "cases": cases
    }))
}

fn main() -> Result<(), String> {
    let corpus = manifest_body()?;
    let corpus_json = serde_json::to_string_pretty(&corpus)
        .map_err(|err| format!("failed to serialize frame KAT corpus body: {err}"))?;
    let out = json!({
        "corpus_digest": hex_encode(&Sha256::digest(corpus_json.as_bytes())),
        "corpus": corpus
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&out)
            .map_err(|err| format!("failed to serialize frame KAT corpus: {err}"))?
    );
    Ok(())
}
