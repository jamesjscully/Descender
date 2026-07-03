use descender_parameter_search::{
    manifest_vector_evidence::{
        path2_account_creation_proof_obligation_vector_corpus_evidence_v1,
        path2_direct_accepted_composition_evidence_artifact_v1,
    },
    production_reference::{
        path2_account_creation_no_export_boundary_report_v1,
        path2_account_creation_no_export_boundary_report_v1_digest,
        path2_account_creation_proof_boundary_report_v1,
        path2_account_creation_proof_boundary_report_v1_digest,
        path2_account_stark_fri_qrom_residual_report_v1,
        path2_account_stark_fri_qrom_residual_report_v1_digest,
        path2_account_stark_payload_kat_boundary_report_v1,
        path2_account_stark_payload_kat_boundary_report_v1_digest,
        path2_direct_theorem_boundary_report_v1, path2_direct_theorem_boundary_report_v1_digest,
        validate_path2_account_creation_no_export_boundary_report_v1,
        validate_path2_account_creation_proof_boundary_report_v1,
        validate_path2_account_stark_fri_qrom_residual_report_v1,
        validate_path2_account_stark_payload_kat_boundary_report_v1,
        validate_path2_direct_theorem_boundary_report_v1,
    },
};
use serde_json::Value;
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

fn load_text(root: &Path, relative_path: &str) -> Result<String, String> {
    let path = root.join(relative_path);
    fs::read_to_string(&path).map_err(|err| format!("read {}: {err}", path.display()))
}

fn load_json(root: &Path, relative_path: &str) -> Result<Value, String> {
    let text = load_text(root, relative_path)?;
    serde_json::from_str(&text).map_err(|err| format!("parse {relative_path}: {err}"))
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

fn require_str(value: &Value, pointer: &str, expected: &str) -> Result<(), String> {
    let observed = str_at(value, pointer)?;
    if observed != expected {
        return Err(format!(
            "proof-obligation consistency {pointer} mismatch: {observed:?} != {expected:?}"
        ));
    }
    Ok(())
}

fn require_hex(value: &Value, pointer: &str, expected: &[u8]) -> Result<(), String> {
    let observed = str_at(value, pointer)?;
    let expected_hex = hex_encode(expected);
    if observed != expected_hex {
        return Err(format!(
            "proof-obligation consistency {pointer} mismatch: {observed} != {expected_hex}"
        ));
    }
    Ok(())
}

fn require_bool(value: &Value, pointer: &str, expected: bool) -> Result<(), String> {
    let observed = bool_at(value, pointer)?;
    if observed != expected {
        return Err(format!(
            "proof-obligation consistency {pointer} mismatch: {observed} != {expected}"
        ));
    }
    Ok(())
}

fn body_digest(value: &Value, label: &str) -> Result<String, String> {
    let json =
        serde_json::to_string_pretty(value).map_err(|err| format!("serialize {label}: {err}"))?;
    Ok(hex_encode(&Sha256::digest(json.as_bytes())))
}

fn require_manifest_body<'a>(file: &'a Value, label: &str) -> Result<&'a Value, String> {
    let body = at(file, "/manifest")?;
    let recomputed = body_digest(body, label)?;
    if str_at(file, "/manifest_digest")? != recomputed {
        return Err(format!("{label} manifest digest mismatch"));
    }
    Ok(body)
}

fn main() -> Result<(), String> {
    let root = repo_root();
    let corpus_path = "formal/kat/path2_account_creation_proof_obligation_vectors_v1.json";
    let corpus_file = load_json(&root, corpus_path)?;
    let corpus = at(&corpus_file, "/corpus")?;
    let corpus_json = serde_json::to_string_pretty(corpus)
        .map_err(|err| format!("serialize proof-obligation corpus body: {err}"))?;
    let recomputed_digest = hex_encode(&Sha256::digest(corpus_json.as_bytes()));
    if str_at(&corpus_file, "/corpus_digest")? != recomputed_digest {
        return Err("proof-obligation vector corpus digest mismatch".to_string());
    }
    let accepted_composition_file = load_json(
        &root,
        "formal/kat/path2_direct_accepted_composition_evidence_v1.json",
    )?;
    let accepted_composition = at(&accepted_composition_file, "/manifest")?;
    let accepted_composition_json = serde_json::to_string_pretty(accepted_composition)
        .map_err(|err| format!("serialize direct accepted composition manifest body: {err}"))?;
    let accepted_composition_recomputed_digest =
        hex_encode(&Sha256::digest(accepted_composition_json.as_bytes()));
    if str_at(&accepted_composition_file, "/manifest_digest")?
        != accepted_composition_recomputed_digest
    {
        return Err("direct accepted composition manifest digest mismatch".to_string());
    }
    require_str(
        accepted_composition,
        "/schema",
        "Descender.Path2.DirectAcceptedCompositionDeterministicKatEvidenceManifest.v1",
    )?;
    require_str(
        accepted_composition,
        "/status",
        "digest_pinned_deterministic_kat_accepted_direct_composition_evidence",
    )?;
    require_bool(
        accepted_composition,
        "/deterministic_proof_byte_mode/evidence_digest_recomputed",
        true,
    )?;
    if u64_at(
        accepted_composition,
        "/deterministic_proof_byte_mode/rayon_threads",
    )? != 1
    {
        return Err(
            "direct accepted composition manifest is not in one-thread KAT mode".to_string(),
        );
    }
    let accepted_boundary_file = load_json(
        &root,
        "formal/kat/path2_account_creation_accepted_proof_boundary_evidence_v1.json",
    )?;
    let accepted_boundary = at(&accepted_boundary_file, "/manifest")?;
    let accepted_boundary_json =
        serde_json::to_string_pretty(accepted_boundary).map_err(|err| {
            format!("serialize account-creation accepted proof-boundary manifest body: {err}")
        })?;
    let accepted_boundary_recomputed_digest =
        hex_encode(&Sha256::digest(accepted_boundary_json.as_bytes()));
    if str_at(&accepted_boundary_file, "/manifest_digest")? != accepted_boundary_recomputed_digest {
        return Err(
            "account-creation accepted proof-boundary manifest digest mismatch".to_string(),
        );
    }
    require_str(
        accepted_boundary,
        "/schema",
        "Descender.Path2.AccountCreationAcceptedProofBoundaryDeterministicKatEvidenceManifest.v1",
    )?;
    require_str(
        accepted_boundary,
        "/status",
        "digest_pinned_deterministic_kat_accepted_account_creation_proof_boundary_evidence",
    )?;
    require_bool(
        accepted_boundary,
        "/deterministic_proof_byte_mode/evidence_digest_recomputed",
        true,
    )?;
    if u64_at(
        accepted_boundary,
        "/deterministic_proof_byte_mode/rayon_threads",
    )? != 1
    {
        return Err(
            "account-creation accepted proof-boundary manifest is not in one-thread KAT mode"
                .to_string(),
        );
    }
    let accepted_boundary_artifact = at(corpus, "/accepted_proof_boundary_evidence_artifact")?;
    if str_at(accepted_boundary_artifact, "/artifact_manifest_digest")?
        != str_at(&accepted_boundary_file, "/manifest_digest")?
        || str_at(
            accepted_boundary_artifact,
            "/artifact_manifest_recomputed_digest",
        )? != accepted_boundary_recomputed_digest
        || str_at(accepted_boundary_artifact, "/evidence_digest")?
            != str_at(accepted_boundary, "/evidence/digest")?
        || str_at(accepted_boundary_artifact, "/evidence_recomputed_digest")?
            != str_at(accepted_boundary, "/evidence/recomputed_digest")?
    {
        return Err(
            "proof-obligation corpus lost accepted proof-boundary artifact manifest binding"
                .to_string(),
        );
    }

    let proof_report = path2_account_creation_proof_boundary_report_v1();
    validate_path2_account_creation_proof_boundary_report_v1(&proof_report)?;
    let proof_digest = path2_account_creation_proof_boundary_report_v1_digest(&proof_report);
    let direct_report = path2_direct_theorem_boundary_report_v1();
    validate_path2_direct_theorem_boundary_report_v1(&direct_report)?;
    let direct_digest = path2_direct_theorem_boundary_report_v1_digest(&direct_report);
    let no_export_report = path2_account_creation_no_export_boundary_report_v1();
    validate_path2_account_creation_no_export_boundary_report_v1(&no_export_report)?;
    let no_export_digest =
        path2_account_creation_no_export_boundary_report_v1_digest(&no_export_report);
    if no_export_digest != no_export_report.digest {
        return Err("no-export boundary report digest mismatch".to_string());
    }
    let no_export_vectors_file = load_json(
        &root,
        "formal/kat/path2_account_creation_no_export_vectors_v1.json",
    )?;
    let no_export_vectors = at(&no_export_vectors_file, "/corpus")?;
    let no_export_vectors_json = serde_json::to_string_pretty(no_export_vectors)
        .map_err(|err| format!("serialize no-export vector corpus body: {err}"))?;
    let no_export_vectors_recomputed_digest =
        hex_encode(&Sha256::digest(no_export_vectors_json.as_bytes()));
    if str_at(&no_export_vectors_file, "/corpus_digest")? != no_export_vectors_recomputed_digest {
        return Err("no-export vector corpus digest mismatch".to_string());
    }
    let fri_qrom_report = path2_account_stark_fri_qrom_residual_report_v1();
    validate_path2_account_stark_fri_qrom_residual_report_v1(&fri_qrom_report)?;
    let fri_qrom_digest = path2_account_stark_fri_qrom_residual_report_v1_digest(&fri_qrom_report);
    if fri_qrom_digest != fri_qrom_report.digest {
        return Err("FRI/QROM residual report digest mismatch".to_string());
    }
    let fri_qrom_vectors_file = load_json(
        &root,
        "formal/kat/path2_account_stark_fri_qrom_domain_vectors_v1.json",
    )?;
    let fri_qrom_vectors = at(&fri_qrom_vectors_file, "/corpus")?;
    let fri_qrom_vectors_json = serde_json::to_string_pretty(fri_qrom_vectors)
        .map_err(|err| format!("serialize FRI/QROM domain vector corpus body: {err}"))?;
    let fri_qrom_vectors_recomputed_digest =
        hex_encode(&Sha256::digest(fri_qrom_vectors_json.as_bytes()));
    if str_at(&fri_qrom_vectors_file, "/corpus_digest")? != fri_qrom_vectors_recomputed_digest {
        return Err("FRI/QROM domain vector corpus digest mismatch".to_string());
    }
    let payload_kat_report = path2_account_stark_payload_kat_boundary_report_v1();
    validate_path2_account_stark_payload_kat_boundary_report_v1(&payload_kat_report)?;
    let payload_kat_digest =
        path2_account_stark_payload_kat_boundary_report_v1_digest(&payload_kat_report);
    if payload_kat_digest != payload_kat_report.digest {
        return Err("payload KAT boundary report digest mismatch".to_string());
    }
    let payload_vectors_file = load_json(
        &root,
        "formal/kat/path2_account_stark_payload_kat_vectors_v1.json",
    )?;
    let payload_vectors = at(&payload_vectors_file, "/corpus")?;
    let payload_vectors_json = serde_json::to_string_pretty(payload_vectors)
        .map_err(|err| format!("serialize payload KAT vector corpus body: {err}"))?;
    let payload_vectors_recomputed_digest =
        hex_encode(&Sha256::digest(payload_vectors_json.as_bytes()));
    if str_at(&payload_vectors_file, "/vector_corpus_digest")? != payload_vectors_recomputed_digest
    {
        return Err("payload KAT vector corpus digest mismatch".to_string());
    }

    if str_at(corpus, "/schema")? != "Descender.Path2.AccountCreationProofObligationVectors.v1" {
        return Err("proof-obligation vector corpus schema mismatch".to_string());
    }
    if str_at(corpus, "/status")? != "digest_pinned_proof_obligation_audit_vectors" {
        return Err("proof-obligation vector corpus status mismatch".to_string());
    }
    if str_at(corpus, "/proof_boundary_report/digest")? != hex_encode(&proof_report.digest)
        || str_at(corpus, "/proof_boundary_report/recomputed_digest")? != hex_encode(&proof_digest)
    {
        return Err("proof-obligation vector corpus is not bound to proof report".to_string());
    }
    if str_at(corpus, "/direct_theorem_report/digest")? != hex_encode(&direct_report.digest)
        || str_at(corpus, "/direct_theorem_report/recomputed_digest")? != hex_encode(&direct_digest)
        || str_at(
            corpus,
            "/direct_theorem_report/account_creation_proof_boundary_report_digest",
        )? != hex_encode(&proof_report.digest)
    {
        return Err("proof-obligation vector corpus is not bound to direct report".to_string());
    }
    if str_at(
        corpus,
        "/direct_theorem_report/accepted_tuple_evidence_schema",
    )? != direct_report.accepted_tuple_evidence_schema
        || str_at(
            corpus,
            "/direct_theorem_report/accepted_tuple_evidence_endpoint",
        )? != direct_report.accepted_tuple_evidence_endpoint
        || bool_at(
            corpus,
            "/direct_theorem_report/accepted_tuple_evidence_present",
        )? != direct_report.accepted_tuple_evidence_present
        || str_at(
            corpus,
            "/direct_theorem_report/accepted_composition_evidence_schema",
        )? != direct_report.accepted_composition_evidence_schema
        || str_at(
            corpus,
            "/direct_theorem_report/accepted_composition_evidence_endpoint",
        )? != direct_report.accepted_composition_evidence_endpoint
        || bool_at(
            corpus,
            "/direct_theorem_report/accepted_composition_evidence_present",
        )? != direct_report.accepted_composition_evidence_present
    {
        return Err(
            "proof-obligation vector corpus lost direct accepted evidence hooks".to_string(),
        );
    }
    if str_at(
        corpus,
        "/proof_boundary_report/accepted_boundary_evidence_schema",
    )? != proof_report.accepted_boundary_evidence_schema
        || str_at(
            corpus,
            "/proof_boundary_report/accepted_boundary_evidence_endpoint",
        )? != proof_report.accepted_boundary_evidence_endpoint
        || bool_at(
            corpus,
            "/proof_boundary_report/accepted_boundary_evidence_present",
        )? != proof_report.accepted_boundary_evidence_present
        || bool_at(
            corpus,
            "/proof_boundary_report/accepted_boundary_binding_evidence_present",
        )? != proof_report.accepted_boundary_binding_evidence_present
        || bool_at(
            corpus,
            "/proof_boundary_report/replay_payload_digest_binding_evidence_present",
        )? != proof_report.replay_payload_digest_binding_evidence_present
        || bool_at(
            corpus,
            "/proof_boundary_report/deployed_anchor_binding_evidence_present",
        )? != proof_report.deployed_anchor_binding_evidence_present
    {
        return Err(
            "proof-obligation vector corpus lost accepted proof-boundary evidence hook".to_string(),
        );
    }
    if str_at(
        accepted_boundary_artifact,
        "/account_creation_proof_boundary_report_digest",
    )? != hex_encode(&proof_report.digest)
        || str_at(
            accepted_boundary_artifact,
            "/account_creation_proof_boundary_report_recomputed_digest",
        )? != hex_encode(&proof_digest)
        || str_at(
            accepted_boundary_artifact,
            "/deterministic_proof_byte_mode/accepted_boundary_endpoint",
        )? != proof_report.accepted_boundary_evidence_endpoint
        || str_at(accepted_boundary_artifact, "/evidence_schema")?
            != proof_report.accepted_boundary_evidence_schema
        || !bool_at(
            accepted_boundary_artifact,
            "/deterministic_proof_byte_mode/evidence_digest_recomputed",
        )?
        || u64_at(
            accepted_boundary_artifact,
            "/deterministic_proof_byte_mode/rayon_threads",
        )? != 1
    {
        return Err(
            "proof-obligation corpus accepted proof-boundary artifact drifted from proof report"
                .to_string(),
        );
    }

    let accepted_composition_artifact = path2_direct_accepted_composition_evidence_artifact_v1()?;
    let corpus_composition_artifact = at(corpus, "/direct_accepted_composition_evidence_artifact")?;
    if corpus_composition_artifact != &accepted_composition_artifact {
        return Err(
            "proof-obligation corpus lost direct accepted-composition artifact binding".to_string(),
        );
    }
    if str_at(corpus_composition_artifact, "/artifact_manifest_digest")?
        != str_at(&accepted_composition_file, "/manifest_digest")?
        || str_at(
            corpus_composition_artifact,
            "/artifact_manifest_recomputed_digest",
        )? != accepted_composition_recomputed_digest
        || str_at(corpus_composition_artifact, "/evidence_digest")?
            != str_at(accepted_composition, "/evidence/digest")?
        || str_at(corpus_composition_artifact, "/evidence_recomputed_digest")?
            != str_at(accepted_composition, "/evidence/recomputed_digest")?
        || str_at(
            corpus_composition_artifact,
            "/direct_theorem_boundary_report_digest",
        )? != hex_encode(&direct_report.digest)
        || str_at(
            corpus_composition_artifact,
            "/direct_theorem_boundary_report_recomputed_digest",
        )? != hex_encode(&direct_digest)
        || str_at(
            corpus_composition_artifact,
            "/account_creation_proof_boundary_report_digest",
        )? != hex_encode(&proof_report.digest)
        || str_at(
            corpus_composition_artifact,
            "/account_creation_proof_boundary_report_recomputed_digest",
        )? != hex_encode(&proof_digest)
        || str_at(
            corpus_composition_artifact,
            "/account_creation_accepted_proof_boundary_evidence_digest",
        )? != str_at(accepted_boundary_artifact, "/evidence_digest")?
        || at(
            corpus_composition_artifact,
            "/account_creation_accepted_proof_boundary_evidence_artifact",
        )? != accepted_boundary_artifact
    {
        return Err(
            "proof-obligation corpus direct accepted-composition artifact drifted".to_string(),
        );
    }
    require_str(
        corpus,
        "/no_export_vector_evidence/corpus_schema",
        "Descender.Path2.AccountCreationNoExportVectors.v1",
    )?;
    require_str(
        corpus,
        "/no_export_vector_evidence/corpus_status",
        "digest_pinned_no_export_audit_vectors",
    )?;
    require_str(
        corpus,
        "/no_export_vector_evidence/corpus_digest",
        str_at(&no_export_vectors_file, "/corpus_digest")?,
    )?;
    require_str(
        corpus,
        "/no_export_vector_evidence/corpus_recomputed_digest",
        &no_export_vectors_recomputed_digest,
    )?;
    require_hex(
        corpus,
        "/no_export_vector_evidence/boundary_report_digest",
        &no_export_report.digest,
    )?;
    require_hex(
        corpus,
        "/no_export_vector_evidence/boundary_report_recomputed_digest",
        &no_export_digest,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/all_private_material_absent",
        true,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/payload_denylist_absent",
        true,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/simulator_denylist_absent",
        true,
    )?;
    require_bool(
        corpus,
        "/audit_summary/current_no_export_vectors_cover_boundary_cases",
        true,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/public_payload_exports_private_material",
        false,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/simulator_theorem_instantiated",
        false,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/no_export_theorem_instantiated",
        false,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/frozen_no_export_vectors_complete",
        true,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/public_bundle_no_export_predicate_instantiated",
        true,
    )?;
    require_str(
        corpus,
        "/no_export_vector_evidence/public_bundle_no_export_lean_theorem",
        no_export_report.public_bundle_no_export_lean_theorem,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/simulator_input_allowlist_predicate_instantiated",
        true,
    )?;
    require_str(
        corpus,
        "/no_export_vector_evidence/simulator_input_allowlist_lean_theorem",
        no_export_report.simulator_input_allowlist_lean_theorem,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/accepted_public_anchor_package_bound",
        true,
    )?;
    require_str(
        corpus,
        "/no_export_vector_evidence/accepted_public_anchor_package_lean_theorem",
        no_export_report.accepted_public_anchor_package_lean_theorem,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/accepted_bundle_public_anchor_simulation_boundary_bound",
        true,
    )?;
    require_str(
        corpus,
        "/no_export_vector_evidence/accepted_bundle_public_anchor_simulation_boundary_lean_theorem",
        no_export_report.accepted_bundle_public_anchor_simulation_boundary_lean_theorem,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/accepted_bundle_public_bundle_from_public_anchors_bound",
        true,
    )?;
    require_str(
        corpus,
        "/no_export_vector_evidence/accepted_bundle_public_bundle_from_public_anchors_lean_theorem",
        no_export_report.accepted_bundle_public_bundle_from_public_anchors_lean_theorem,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/accepted_bundle_simulator_input_from_public_anchors_bound",
        true,
    )?;
    require_str(
        corpus,
        "/no_export_vector_evidence/accepted_bundle_simulator_input_from_public_anchors_lean_theorem",
        no_export_report.accepted_bundle_simulator_input_from_public_anchors_lean_theorem,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/accepted_bundle_simulator_no_export_reduction_bound",
        true,
    )?;
    require_str(
        corpus,
        "/no_export_vector_evidence/accepted_bundle_simulator_no_export_reduction_lean_theorem",
        no_export_report.accepted_bundle_simulator_no_export_reduction_lean_theorem,
    )?;
    require_bool(
        corpus,
        "/no_export_vector_evidence/accepted_bundle_simulatable_from_public_anchors",
        false,
    )?;
    require_bool(corpus, "/no_export_vector_evidence/deployment_ready", false)?;
    require_str(
        corpus,
        "/no_export_vector_evidence/blocker",
        no_export_report.blocker,
    )?;
    require_str(
        no_export_vectors,
        "/schema",
        "Descender.Path2.AccountCreationNoExportVectors.v1",
    )?;
    require_str(
        no_export_vectors,
        "/status",
        "digest_pinned_no_export_audit_vectors",
    )?;
    require_hex(
        no_export_vectors,
        "/boundary_report/digest",
        &no_export_report.digest,
    )?;
    if u64_at(corpus, "/no_export_vector_evidence/case_count")?
        != no_export_report.no_export_vector_case_ids.len() as u64
        || u64_at(no_export_vectors, "/case_count")?
            != no_export_report.no_export_vector_case_ids.len() as u64
        || !bool_at(
            no_export_vectors,
            "/audit_summary/all_private_material_absent",
        )?
        || !bool_at(no_export_vectors, "/audit_summary/payload_denylist_absent")?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/simulator_denylist_absent",
        )?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/simulator_input_allowlist_predicate_instantiated",
        )?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/accepted_public_anchor_package_bound",
        )?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/accepted_bundle_public_anchor_simulation_boundary_bound",
        )?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/accepted_bundle_public_bundle_from_public_anchors_bound",
        )?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/accepted_bundle_simulator_input_from_public_anchors_bound",
        )?
        || !bool_at(
            no_export_vectors,
            "/audit_summary/accepted_bundle_simulator_no_export_reduction_bound",
        )?
        || bool_at(
            no_export_vectors,
            "/audit_summary/accepted_bundle_simulatable_from_public_anchors",
        )?
        || bool_at(
            no_export_vectors,
            "/audit_summary/public_payload_exports_private_material",
        )?
        || bool_at(
            no_export_vectors,
            "/audit_summary/simulator_theorem_instantiated",
        )?
        || bool_at(
            no_export_vectors,
            "/audit_summary/no_export_theorem_instantiated",
        )?
        || bool_at(no_export_vectors, "/audit_summary/deployment_ready")?
    {
        return Err("proof-obligation no-export vector evidence count/status mismatch".to_string());
    }

    require_str(
        corpus,
        "/fri_qrom_domain_vector_evidence/corpus_schema",
        "Descender.Path2.AccountStarkFriQromDomainVectors.v1",
    )?;
    require_str(
        corpus,
        "/fri_qrom_domain_vector_evidence/corpus_status",
        "digest_pinned_fri_qrom_domain_audit_vectors",
    )?;
    require_str(
        corpus,
        "/fri_qrom_domain_vector_evidence/corpus_digest",
        str_at(&fri_qrom_vectors_file, "/corpus_digest")?,
    )?;
    require_str(
        corpus,
        "/fri_qrom_domain_vector_evidence/corpus_recomputed_digest",
        &fri_qrom_vectors_recomputed_digest,
    )?;
    require_hex(
        corpus,
        "/fri_qrom_domain_vector_evidence/boundary_report_digest",
        &fri_qrom_report.digest,
    )?;
    require_hex(
        corpus,
        "/fri_qrom_domain_vector_evidence/boundary_report_recomputed_digest",
        &fri_qrom_digest,
    )?;
    require_bool(
        corpus,
        "/fri_qrom_domain_vector_evidence/fri_schedule_pinned",
        true,
    )?;
    require_bool(
        corpus,
        "/fri_qrom_domain_vector_evidence/challenge_field_pinned",
        true,
    )?;
    require_bool(
        corpus,
        "/fri_qrom_domain_vector_evidence/checked_core_qrom_budget_pinned",
        true,
    )?;
    require_str(
        corpus,
        "/fri_qrom_domain_vector_evidence/account_stark_fri_qrom_transcript_reduction_lean_theorem",
        fri_qrom_report.account_stark_fri_qrom_transcript_reduction_lean_theorem,
    )?;
    require_bool(
        corpus,
        "/fri_qrom_domain_vector_evidence/account_stark_fri_qrom_transcript_reduction_bound",
        true,
    )?;
    require_bool(
        corpus,
        "/audit_summary/current_fri_qrom_domain_vectors_cover_boundary_cases",
        true,
    )?;
    for pointer in [
        "/fri_qrom_domain_vector_evidence/account_stark_transcript_qrom_theorem_instantiated",
        "/fri_qrom_domain_vector_evidence/recursion_pricing_complete",
        "/fri_qrom_domain_vector_evidence/residual_pricing_complete",
        "/fri_qrom_domain_vector_evidence/deployment_ready",
    ] {
        require_bool(corpus, pointer, false)?;
    }
    require_str(
        corpus,
        "/fri_qrom_domain_vector_evidence/blocker",
        fri_qrom_report.blocker,
    )?;
    require_str(
        fri_qrom_vectors,
        "/schema",
        "Descender.Path2.AccountStarkFriQromDomainVectors.v1",
    )?;
    require_str(
        fri_qrom_vectors,
        "/status",
        "digest_pinned_fri_qrom_domain_audit_vectors",
    )?;
    require_hex(
        fri_qrom_vectors,
        "/boundary_report/digest",
        &fri_qrom_report.digest,
    )?;
    if u64_at(corpus, "/fri_qrom_domain_vector_evidence/case_count")?
        != fri_qrom_report.qrom_domain_vector_case_count as u64
        || u64_at(fri_qrom_vectors, "/case_count")?
            != fri_qrom_report.qrom_domain_vector_case_count as u64
        || u64_at(
            corpus,
            "/fri_qrom_domain_vector_evidence/poseidon2_sponge_floor_bits",
        )? != fri_qrom_report.poseidon2_sponge_floor_bits as u64
        || u64_at(
            corpus,
            "/fri_qrom_domain_vector_evidence/checked_core_qrom_selected_transcript_term_bits",
        )? != fri_qrom_report.checked_core_qrom_selected_transcript_term_bits as u64
        || !bool_at(fri_qrom_vectors, "/audit_summary/fri_schedule_pinned")?
        || !bool_at(fri_qrom_vectors, "/audit_summary/challenge_field_pinned")?
        || !bool_at(
            fri_qrom_vectors,
            "/audit_summary/checked_core_qrom_budget_pinned",
        )?
        || !bool_at(
            fri_qrom_vectors,
            "/audit_summary/account_stark_fri_qrom_transcript_reduction_bound",
        )?
        || bool_at(
            fri_qrom_vectors,
            "/audit_summary/account_stark_transcript_qrom_theorem_instantiated",
        )?
        || bool_at(
            fri_qrom_vectors,
            "/audit_summary/recursion_pricing_complete",
        )?
        || bool_at(fri_qrom_vectors, "/audit_summary/residual_pricing_complete")?
        || bool_at(fri_qrom_vectors, "/audit_summary/deployment_ready")?
    {
        return Err(
            "proof-obligation FRI/QROM domain vector evidence count/status mismatch".to_string(),
        );
    }

    require_str(
        corpus,
        "/payload_kat_vector_evidence/corpus_schema",
        "Descender.Path2.AccountStarkPayloadKatVectors.v1",
    )?;
    require_str(
        corpus,
        "/payload_kat_vector_evidence/corpus_status",
        "digest_pinned_boundary_vectors",
    )?;
    require_str(
        corpus,
        "/payload_kat_vector_evidence/corpus_digest",
        str_at(&payload_vectors_file, "/vector_corpus_digest")?,
    )?;
    require_str(
        corpus,
        "/payload_kat_vector_evidence/corpus_recomputed_digest",
        &payload_vectors_recomputed_digest,
    )?;
    require_hex(
        corpus,
        "/payload_kat_vector_evidence/boundary_report_digest",
        &payload_kat_report.digest,
    )?;
    require_hex(
        corpus,
        "/payload_kat_vector_evidence/boundary_report_recomputed_digest",
        &payload_kat_digest,
    )?;
    require_bool(
        corpus,
        "/payload_kat_vector_evidence/every_current_row_has_generated_vector",
        true,
    )?;
    require_bool(
        corpus,
        "/audit_summary/current_payload_kat_vectors_cover_every_boundary_row",
        true,
    )?;
    require_bool(
        corpus,
        "/payload_kat_vector_evidence/frozen_corpus_complete",
        true,
    )?;
    require_bool(
        corpus,
        "/payload_kat_vector_evidence/deployment_ready",
        false,
    )?;
    require_str(
        corpus,
        "/payload_kat_vector_evidence/blocker",
        payload_kat_report.blocker,
    )?;
    if u64_at(
        corpus,
        "/payload_kat_vector_evidence/boundary_report_row_count",
    )? != payload_kat_report.rows.len() as u64
        || u64_at(corpus, "/payload_kat_vector_evidence/rejection_case_count")?
            != payload_kat_report.rows.len() as u64
        || u64_at(corpus, "/payload_kat_vector_evidence/case_count")?
            != payload_kat_report.rows.len() as u64 + 1
    {
        return Err("proof-obligation payload KAT vector evidence count mismatch".to_string());
    }
    require_str(
        corpus,
        "/payload_kat_vector_evidence/honest_payload_digest",
        str_at(payload_vectors, "/honest_payload/payload_digest")?,
    )?;
    require_str(
        payload_vectors,
        "/schema",
        "Descender.Path2.AccountStarkPayloadKatVectors.v1",
    )?;
    require_str(payload_vectors, "/status", "digest_pinned_boundary_vectors")?;
    require_hex(
        payload_vectors,
        "/boundary_report/digest",
        &payload_kat_report.digest,
    )?;
    if u64_at(payload_vectors, "/boundary_report/row_count")?
        != payload_kat_report.rows.len() as u64
        || u64_at(payload_vectors, "/rejection_case_count")? != payload_kat_report.rows.len() as u64
        || u64_at(payload_vectors, "/case_count")? != payload_kat_report.rows.len() as u64 + 1
        || !bool_at(
            payload_vectors,
            "/boundary_report/every_current_row_has_generated_vector",
        )?
    {
        return Err(
            "payload KAT vector corpus is not bound to every current boundary row".to_string(),
        );
    }
    if direct_report.account_creation_proof_boundary_report_digest != proof_report.digest {
        return Err("direct theorem report lost proof-boundary binding".to_string());
    }
    require_str(
        accepted_boundary,
        "/deterministic_proof_byte_mode/accepted_boundary_endpoint",
        proof_report.accepted_boundary_evidence_endpoint,
    )?;
    require_str(
        accepted_boundary,
        "/evidence/schema",
        proof_report.accepted_boundary_evidence_schema,
    )?;
    require_str(
        accepted_boundary,
        "/evidence/proof_boundary_target",
        proof_report.proof_boundary_target,
    )?;
    require_str(
        accepted_boundary,
        "/evidence/proof_boundary_conclusion",
        proof_report.proof_boundary_conclusion,
    )?;
    require_hex(
        accepted_boundary,
        "/evidence/account_creation_proof_boundary_report_digest",
        &proof_report.digest,
    )?;
    require_str(
        accepted_composition,
        "/evidence/account_creation_accepted_proof_boundary_evidence_digest",
        str_at(accepted_boundary, "/evidence/digest")?,
    )?;
    if str_at(
        accepted_composition,
        "/evidence/account_creation_accepted_proof_boundary_evidence_digest",
    )? != str_at(accepted_boundary_artifact, "/evidence_digest")?
    {
        return Err(
            "proof-obligation corpus accepted composition drifted from accepted proof-boundary artifact"
                .to_string(),
        );
    }
    for pointer in [
        "/evidence/verifier_boundary_accepts",
        "/evidence/production_gate_fail_closed",
        "/evidence/proof_boundary_report_validated",
        "/evidence/payload_digest_bound_to_certificate",
        "/evidence/evidence_digest_bound_to_certificate",
        "/evidence/leaf_transcript_bound_to_evidence",
        "/evidence/leaf_digest_bound_to_evidence",
        "/evidence/leaf_context_bound",
        "/evidence/binding_digest_bound",
        "/evidence/payload_binding_fields_match_report",
        "/evidence/decoded_public_anchors_match_deployed_anchors",
        "/evidence/deployed_anchor_account_id_bound",
        "/evidence/deployed_anchor_slot_id_bound",
        "/evidence/deployed_anchor_state_root_bound",
        "/evidence/deployed_anchor_lifecycle_anchor_bound",
        "/evidence/deployed_anchor_verifier_version_bound",
        "/evidence/deployed_anchor_masked_projection_bound",
        "/evidence/deployed_anchor_role_root_export_bridge_bound",
        "/evidence/deployed_anchor_packed_witness_commitment_bound",
        "/evidence/deployed_anchor_fields_match_report",
        "/evidence/account_stark_bundle_nonempty",
        "/evidence/proof_family_obligations_still_open",
    ] {
        require_bool(accepted_boundary, pointer, true)?;
    }
    for pointer in [
        "/evidence/public_payload_exports_private_material",
        "/evidence/deployment_ready",
    ] {
        require_bool(accepted_boundary, pointer, false)?;
    }
    for pointer in [
        "/audit_summary/verifier_boundary_accepts",
        "/audit_summary/production_gate_fail_closed",
        "/audit_summary/proof_boundary_report_validated",
        "/audit_summary/proof_family_obligations_still_open",
    ] {
        require_bool(accepted_boundary_artifact, pointer, true)?;
    }
    for pointer in [
        "/audit_summary/public_payload_exports_private_material",
        "/audit_summary/deployment_ready",
    ] {
        require_bool(accepted_boundary_artifact, pointer, false)?;
    }
    require_str(
        accepted_composition,
        "/deterministic_proof_byte_mode/accepted_composition_endpoint",
        direct_report.accepted_composition_evidence_endpoint,
    )?;
    require_str(
        accepted_composition,
        "/evidence/schema",
        direct_report.accepted_composition_evidence_schema,
    )?;
    require_str(
        accepted_composition,
        "/evidence/theorem_target",
        direct_report.theorem_target,
    )?;
    require_str(
        accepted_composition,
        "/evidence/theorem_conclusion",
        direct_report.theorem_conclusion,
    )?;
    require_hex(
        accepted_composition,
        "/evidence/direct_theorem_boundary_report_digest",
        &direct_report.digest,
    )?;
    require_hex(
        accepted_composition,
        "/evidence/account_creation_proof_boundary_report_digest",
        &proof_report.digest,
    )?;
    require_bool(
        accepted_composition,
        "/evidence/proof_boundary_digest_matches_direct_theorem",
        true,
    )?;
    require_bool(
        accepted_composition,
        "/evidence/creation_evidence_matches_direct_tuple",
        true,
    )?;
    require_bool(
        accepted_composition,
        "/evidence/verifier_boundary_accepts_creation",
        true,
    )?;
    require_bool(
        accepted_composition,
        "/evidence/proof_family_obligations_still_open",
        true,
    )?;
    require_bool(
        accepted_composition,
        "/evidence/direct_theorem_obligations_still_open",
        true,
    )?;
    require_bool(
        accepted_composition,
        "/evidence/account_creation_proof_boundary_verified",
        false,
    )?;
    require_bool(
        accepted_composition,
        "/evidence/composition_theorem_instantiated",
        false,
    )?;
    require_bool(accepted_composition, "/evidence/deployment_ready", false)?;
    require_str(
        accepted_composition,
        "/evidence/selected_account_creation_payload_digest",
        str_at(payload_vectors, "/honest_payload/payload_digest")?,
    )?;
    require_str(
        accepted_boundary,
        "/evidence/selected_account_creation_payload_digest",
        str_at(payload_vectors, "/honest_payload/payload_digest")?,
    )?;

    let open_proof_obligation_count = proof_report
        .proof_obligation_rows
        .iter()
        .filter(|row| row.open)
        .count() as u64;
    let closed_proof_obligation_count = proof_report
        .proof_obligation_rows
        .iter()
        .filter(|row| !row.open)
        .count() as u64;
    if u64_at(corpus, "/case_count")? != proof_report.proof_obligation_count as u64
        || u64_at(corpus, "/audit_summary/proof_obligation_count")?
            != proof_report.proof_obligation_count as u64
        || u64_at(corpus, "/audit_summary/open_proof_obligation_count")?
            != open_proof_obligation_count
        || u64_at(corpus, "/audit_summary/closed_proof_obligation_count")?
            != closed_proof_obligation_count
        || u64_at(
            corpus,
            "/audit_summary/proof_obligation_audit_vector_case_count",
        )? != proof_report.proof_obligation_audit_vector_case_count as u64
    {
        return Err("proof-obligation vector corpus case count drifted".to_string());
    }
    let expected_closure_dependencies = serde_json::to_value(proof_report.closure_dependency_rows)
        .map_err(|err| format!("serialize proof-boundary closure dependency rows: {err}"))?;
    if u64_at(corpus, "/proof_boundary_report/closure_dependency_count")?
        != proof_report.closure_dependency_count as u64
        || at(corpus, "/proof_boundary_report/closure_dependency_rows")?
            != &expected_closure_dependencies
        || !bool_at(
            corpus,
            "/proof_boundary_report/every_closure_dependency_has_open_obligation",
        )?
        || !bool_at(
            corpus,
            "/proof_boundary_report/every_closure_dependency_has_open_support_surface",
        )?
        || !bool_at(
            corpus,
            "/proof_boundary_report/every_open_closure_dependency_blocks_deployment",
        )?
    {
        return Err(
            "proof-obligation vector corpus lost proof-boundary closure dependency rows"
                .to_string(),
        );
    }
    if !bool_at(
        corpus,
        "/proof_boundary_report/proof_obligation_rows_pinned",
    )? || !bool_at(
        corpus,
        "/proof_boundary_report/every_proof_obligation_has_named_break",
    )? || !bool_at(
        corpus,
        "/proof_boundary_report/every_open_proof_obligation_blocks_deployment",
    )? || bool_at(corpus, "/audit_summary/all_cases_open")?
        || !bool_at(corpus, "/audit_summary/all_cases_have_named_break")?
        || !bool_at(corpus, "/audit_summary/all_open_cases_block_deployment")?
        || open_proof_obligation_count != 4
        || closed_proof_obligation_count != 4
    {
        return Err("proof-obligation vector corpus lost partial-close row invariants".to_string());
    }
    for pointer in [
        "/proof_boundary_report/deployment_ready",
        "/direct_theorem_report/composition_theorem_instantiated",
        "/direct_theorem_report/account_creation_proof_boundary_verified",
        "/direct_theorem_report/deployment_ready",
        "/audit_summary/simulator_theorem_instantiated",
        "/audit_summary/fri_aok_extractor_theorem_instantiated",
        "/audit_summary/fri_qrom_residual_priced",
        "/audit_summary/external_review_complete",
        "/audit_summary/production_gate_open",
        "/audit_summary/deployment_ready",
    ] {
        if bool_at(corpus, pointer)? {
            return Err(format!(
                "proof-obligation vector corpus unexpectedly closes {pointer}"
            ));
        }
    }
    if !bool_at(corpus, "/audit_summary/frozen_kat_corpus_complete")? {
        return Err("proof-obligation vector corpus lost frozen payload KAT evidence".to_string());
    }

    let proof_obligation_corpus_evidence =
        path2_account_creation_proof_obligation_vector_corpus_evidence_v1()?;
    let proof_manifest_file = load_json(
        &root,
        "formal/kat/path2_account_creation_proof_boundary_v1.json",
    )?;
    let proof_manifest = require_manifest_body(
        &proof_manifest_file,
        "account-creation proof-boundary manifest",
    )?;
    let direct_manifest_file =
        load_json(&root, "formal/kat/path2_direct_theorem_boundary_v1.json")?;
    let direct_manifest =
        require_manifest_body(&direct_manifest_file, "direct theorem boundary manifest")?;
    let proof_boundary = at(proof_manifest, "/account_creation_proof_boundary")?;
    if proof_boundary != at(direct_manifest, "/account_creation_proof_boundary")? {
        return Err(
            "proof-obligation corpus lost shared proof-boundary manifest binding".to_string(),
        );
    }
    for (observed, label) in [
        (
            at(proof_boundary, "/proof_obligation_vector_corpus_evidence")?,
            "proof manifest proof-obligation corpus evidence",
        ),
        (
            at(
                direct_manifest,
                "/account_creation_proof_boundary/proof_obligation_vector_corpus_evidence",
            )?,
            "direct manifest proof-obligation corpus evidence",
        ),
    ] {
        if observed != &proof_obligation_corpus_evidence {
            return Err(format!(
                "{label} drifted from standalone proof-obligation corpus evidence"
            ));
        }
    }
    require_str(
        proof_manifest,
        "/digests/account_creation_proof_boundary_report_digest",
        str_at(
            &proof_obligation_corpus_evidence,
            "/proof_boundary_report_digest",
        )?,
    )?;
    require_str(
        direct_manifest,
        "/digests/account_creation_proof_boundary_report_digest",
        str_at(
            &proof_obligation_corpus_evidence,
            "/proof_boundary_report_digest",
        )?,
    )?;

    let cases = at(corpus, "/cases")?
        .as_array()
        .ok_or_else(|| "proof-obligation corpus cases are not an array".to_string())?;
    if cases.len() != proof_report.proof_obligation_rows.len() {
        return Err("proof-obligation corpus cases array drifted".to_string());
    }
    for (index, ((case, row), expected_case_id)) in cases
        .iter()
        .zip(proof_report.proof_obligation_rows.iter())
        .zip(proof_report.proof_obligation_audit_vector_case_ids.iter())
        .enumerate()
    {
        let case_id = case
            .get("case_id")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("proof-obligation case {index} has no case_id"))?;
        if case_id != row.obligation_id || case_id != *expected_case_id {
            return Err(format!(
                "proof-obligation case {index} id {case_id:?} drifted"
            ));
        }
        if case.get("kind").and_then(Value::as_str) != Some("audit") {
            return Err(format!(
                "proof-obligation case {case_id} is not an audit case"
            ));
        }
        for (field, expected) in [
            ("closure_target", row.closure_target),
            ("supporting_surface", row.supporting_surface),
            ("current_evidence", row.current_evidence),
            ("required_closure", row.required_closure),
            ("named_break", row.named_break),
            ("blocker", row.blocker),
        ] {
            if case.get(field).and_then(Value::as_str) != Some(expected) {
                return Err(format!(
                    "proof-obligation case {case_id} field {field} drifted"
                ));
            }
        }
        if case.get("open").and_then(Value::as_bool) != Some(row.open) {
            return Err(format!("proof-obligation case {case_id} open flag drifted"));
        }
        let expected_closed = matches!(
            case_id,
            "creation_proof_obligation_honest_sampler"
                | "creation_proof_obligation_authorization_class_binding"
                | "creation_proof_obligation_no_postchallenge_timing"
                | "creation_proof_obligation_frozen_payload_kats"
        );
        if !row.open && !expected_closed {
            return Err(format!(
                "proof-obligation case {case_id} closed without imported evidence"
            ));
        }
    }

    println!("Path 2 account-creation proof-obligation vector consistency check passed.");
    Ok(())
}
