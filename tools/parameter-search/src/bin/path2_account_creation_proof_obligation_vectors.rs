use descender_parameter_search::production_reference::{
    path2_account_creation_proof_boundary_report_v1,
    path2_account_creation_proof_boundary_report_v1_digest,
    path2_account_stark_payload_kat_boundary_report_v1,
    path2_account_stark_payload_kat_boundary_report_v1_digest,
    path2_direct_theorem_boundary_report_v1, path2_direct_theorem_boundary_report_v1_digest,
    validate_path2_account_creation_proof_boundary_report_v1,
    validate_path2_account_stark_payload_kat_boundary_report_v1,
    validate_path2_direct_theorem_boundary_report_v1, PROFILE,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("parameter-search crate lives under tools/")
        .to_path_buf()
}

fn load_json(relative_path: &str) -> Result<Value, String> {
    let path = repo_root().join(relative_path);
    let text =
        fs::read_to_string(&path).map_err(|err| format!("read {}: {err}", path.display()))?;
    serde_json::from_str(&text).map_err(|err| format!("parse {}: {err}", path.display()))
}

fn at<'a>(value: &'a Value, pointer: &str) -> Result<&'a Value, String> {
    value
        .pointer(pointer)
        .ok_or_else(|| format!("missing JSON pointer {pointer}"))
}

fn str_at<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, String> {
    at(value, pointer)?
        .as_str()
        .ok_or_else(|| format!("JSON pointer {pointer} is not a string"))
}

fn bool_at(value: &Value, pointer: &str) -> Result<bool, String> {
    at(value, pointer)?
        .as_bool()
        .ok_or_else(|| format!("JSON pointer {pointer} is not a bool"))
}

fn u64_at(value: &Value, pointer: &str) -> Result<u64, String> {
    at(value, pointer)?
        .as_u64()
        .ok_or_else(|| format!("JSON pointer {pointer} is not an unsigned integer"))
}

fn manifest_body() -> Result<Value, String> {
    let proof_report = path2_account_creation_proof_boundary_report_v1();
    validate_path2_account_creation_proof_boundary_report_v1(&proof_report)?;
    let proof_digest = path2_account_creation_proof_boundary_report_v1_digest(&proof_report);
    if proof_digest != proof_report.digest {
        return Err("account-creation proof-boundary report digest mismatch".to_string());
    }

    let direct_report = path2_direct_theorem_boundary_report_v1();
    validate_path2_direct_theorem_boundary_report_v1(&direct_report)?;
    let direct_digest = path2_direct_theorem_boundary_report_v1_digest(&direct_report);
    if direct_digest != direct_report.digest {
        return Err("direct theorem boundary report digest mismatch".to_string());
    }
    if direct_report.account_creation_proof_boundary_report_digest != proof_report.digest {
        return Err("direct theorem report is not bound to proof-boundary report".to_string());
    }

    let payload_kat_report = path2_account_stark_payload_kat_boundary_report_v1();
    validate_path2_account_stark_payload_kat_boundary_report_v1(&payload_kat_report)?;
    let payload_kat_digest =
        path2_account_stark_payload_kat_boundary_report_v1_digest(&payload_kat_report);
    if payload_kat_digest != payload_kat_report.digest {
        return Err("payload KAT boundary report digest mismatch".to_string());
    }
    if payload_kat_report.deployment_ready || payload_kat_report.frozen_corpus_complete {
        return Err(
            "payload KAT boundary unexpectedly claims frozen/deployment readiness".to_string(),
        );
    }
    let payload_vectors_file =
        load_json("formal/kat/path2_account_stark_payload_kat_vectors_v1.json")?;
    let payload_vectors = at(&payload_vectors_file, "/corpus")?;
    let payload_vectors_json = serde_json::to_string_pretty(payload_vectors)
        .map_err(|err| format!("serialize payload KAT vector corpus body: {err}"))?;
    let payload_vectors_digest = hex_encode(&Sha256::digest(payload_vectors_json.as_bytes()));
    if str_at(&payload_vectors_file, "/vector_corpus_digest")? != payload_vectors_digest {
        return Err("payload KAT vector corpus digest mismatch".to_string());
    }
    if str_at(payload_vectors, "/schema")? != "Descender.Path2.AccountStarkPayloadKatVectors.v1"
        || str_at(payload_vectors, "/status")? != "digest_pinned_boundary_vectors"
        || str_at(payload_vectors, "/boundary_report/digest")?
            != hex_encode(&payload_kat_report.digest)
        || u64_at(payload_vectors, "/boundary_report/row_count")?
            != payload_kat_report.rows.len() as u64
        || u64_at(payload_vectors, "/rejection_case_count")? != payload_kat_report.rows.len() as u64
        || u64_at(payload_vectors, "/case_count")? != payload_kat_report.rows.len() as u64 + 1
        || !bool_at(
            payload_vectors,
            "/boundary_report/every_current_row_has_generated_vector",
        )?
    {
        return Err(
            "payload KAT vector corpus is not bound to the current boundary rows".to_string(),
        );
    }

    let cases: Vec<Value> = proof_report
        .proof_obligation_rows
        .iter()
        .map(|row| {
            json!({
                "case_id": row.obligation_id,
                "kind": "audit",
                "closure_target": row.closure_target,
                "supporting_surface": row.supporting_surface,
                "current_evidence": row.current_evidence,
                "required_closure": row.required_closure,
                "named_break": row.named_break,
                "blocker": row.blocker,
                "open": row.open
            })
        })
        .collect();

    if cases.len() != proof_report.proof_obligation_audit_vector_case_ids.len() {
        return Err("proof-obligation audit vector generator case count drifted".to_string());
    }
    for (case, expected) in cases
        .iter()
        .zip(proof_report.proof_obligation_audit_vector_case_ids.iter())
    {
        if case["case_id"] != *expected {
            return Err(format!(
                "proof-obligation audit vector case id drift: expected {expected}"
            ));
        }
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountCreationProofObligationVectors.v1",
        "corpus_id": "Descender.Path2.AccountCreationProofObligationAuditVectors.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "status": "digest_pinned_proof_obligation_audit_vectors",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_creation_proof_obligation_vectors",
            "source": "tools/parameter-search/src/bin/path2_account_creation_proof_obligation_vectors.rs"
        },
        "proof_boundary_report": {
            "domain": proof_report.domain,
            "digest": hex_encode(&proof_report.digest),
            "recomputed_digest": hex_encode(&proof_digest),
            "proof_boundary_target": proof_report.proof_boundary_target,
            "proof_boundary_conclusion": proof_report.proof_boundary_conclusion,
            "accepted_boundary_evidence_schema":
                proof_report.accepted_boundary_evidence_schema,
            "accepted_boundary_evidence_endpoint":
                proof_report.accepted_boundary_evidence_endpoint,
            "accepted_boundary_evidence_present":
                proof_report.accepted_boundary_evidence_present,
            "accepted_boundary_binding_evidence_present":
                proof_report.accepted_boundary_binding_evidence_present,
            "replay_payload_digest_binding_evidence_present":
                proof_report.replay_payload_digest_binding_evidence_present,
            "deployed_anchor_binding_evidence_present":
                proof_report.deployed_anchor_binding_evidence_present,
            "proof_obligation_rows_pinned": proof_report.proof_obligation_rows_pinned,
            "every_proof_obligation_has_named_break":
                proof_report.every_proof_obligation_has_named_break,
            "every_open_proof_obligation_blocks_deployment":
                proof_report.every_open_proof_obligation_blocks_deployment,
            "deployment_ready": proof_report.deployment_ready,
            "blocker": proof_report.blocker
        },
        "payload_kat_vector_evidence": {
            "corpus_path": "formal/kat/path2_account_stark_payload_kat_vectors_v1.json",
            "corpus_schema": str_at(payload_vectors, "/schema")?,
            "corpus_status": str_at(payload_vectors, "/status")?,
            "corpus_digest": str_at(&payload_vectors_file, "/vector_corpus_digest")?,
            "corpus_recomputed_digest": payload_vectors_digest,
            "boundary_report_digest": str_at(payload_vectors, "/boundary_report/digest")?,
            "boundary_report_recomputed_digest": hex_encode(&payload_kat_digest),
            "boundary_report_row_count": payload_kat_report.rows.len(),
            "every_current_row_has_generated_vector": bool_at(
                payload_vectors,
                "/boundary_report/every_current_row_has_generated_vector"
            )?,
            "case_count": u64_at(payload_vectors, "/case_count")?,
            "rejection_case_count": u64_at(payload_vectors, "/rejection_case_count")?,
            "honest_payload_digest": str_at(payload_vectors, "/honest_payload/payload_digest")?,
            "frozen_corpus_complete": payload_kat_report.frozen_corpus_complete,
            "deployment_ready": payload_kat_report.deployment_ready,
            "blocker": payload_kat_report.blocker
        },
        "direct_theorem_report": {
            "domain": direct_report.domain,
            "digest": hex_encode(&direct_report.digest),
            "recomputed_digest": hex_encode(&direct_digest),
            "account_creation_proof_boundary_report_digest":
                hex_encode(&direct_report.account_creation_proof_boundary_report_digest),
            "accepted_tuple_evidence_schema":
                direct_report.accepted_tuple_evidence_schema,
            "accepted_tuple_evidence_endpoint":
                direct_report.accepted_tuple_evidence_endpoint,
            "accepted_tuple_evidence_present":
                direct_report.accepted_tuple_evidence_present,
            "accepted_composition_evidence_schema":
                direct_report.accepted_composition_evidence_schema,
            "accepted_composition_evidence_endpoint":
                direct_report.accepted_composition_evidence_endpoint,
            "accepted_composition_evidence_present":
                direct_report.accepted_composition_evidence_present,
            "composition_theorem_instantiated":
                direct_report.composition_theorem_instantiated,
            "account_creation_proof_boundary_verified":
                direct_report.account_creation_proof_boundary_verified,
            "deployment_ready": direct_report.deployment_ready,
            "blocker": direct_report.blocker
        },
        "audit_summary": {
            "proof_obligation_count": proof_report.proof_obligation_count,
            "proof_obligation_audit_vector_case_count":
                proof_report.proof_obligation_audit_vector_case_count,
            "proof_obligation_named_break_classes":
                proof_report.proof_obligation_named_break_classes,
            "all_cases_open": proof_report.proof_obligation_rows.iter().all(|row| row.open),
            "all_cases_have_named_break":
                proof_report.every_proof_obligation_has_named_break,
            "all_open_cases_block_deployment":
                proof_report.every_open_proof_obligation_blocks_deployment,
            "simulator_theorem_instantiated": proof_report.simulator_theorem_instantiated,
            "fri_aok_extractor_theorem_instantiated":
                proof_report.fri_aok_extractor_theorem_instantiated,
            "fri_qrom_residual_priced": proof_report.fri_qrom_residual_priced,
            "frozen_kat_corpus_complete": proof_report.frozen_kat_corpus_complete,
            "current_payload_kat_vectors_cover_every_boundary_row": bool_at(
                payload_vectors,
                "/boundary_report/every_current_row_has_generated_vector"
            )?,
            "external_review_complete": proof_report.external_review_complete,
            "production_gate_open": proof_report.production_gate_open,
            "deployment_ready": proof_report.deployment_ready
        },
        "case_count": cases.len(),
        "cases": cases
    }))
}

fn main() -> Result<(), String> {
    let corpus = manifest_body()?;
    let corpus_json = serde_json::to_string_pretty(&corpus)
        .map_err(|err| format!("failed to serialize proof-obligation corpus body: {err}"))?;
    let out = json!({
        "corpus_digest": hex_encode(&Sha256::digest(corpus_json.as_bytes())),
        "corpus": corpus
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&out)
            .map_err(|err| format!("failed to serialize proof-obligation corpus: {err}"))?
    );
    Ok(())
}
