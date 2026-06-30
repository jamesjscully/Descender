use descender_parameter_search::{
    manifest_vector_evidence::path2_direct_lifecycle_profile_review_evidence_v1,
    production_reference::{
        PATH2_DIRECT_LIFECYCLE_PROFILE_VERIFICATION_ARTIFACT,
        PATH2_DIRECT_LIFECYCLE_PROFILE_VERIFICATION_ID, path2_direct_lifecycle_boundary_report_v1,
        path2_direct_lifecycle_boundary_report_v1_digest,
        validate_path2_direct_lifecycle_boundary_report_v1,
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
        .ok_or_else(|| format!("JSON pointer {pointer} is not a u64"))
}

fn field_values_at<'a>(
    value: &'a Value,
    pointer: &str,
    field: &str,
) -> Result<Vec<&'a str>, String> {
    let field_pointer = format!("/{field}");
    at(value, pointer)?
        .as_array()
        .ok_or_else(|| format!("JSON pointer {pointer} is not an array"))?
        .iter()
        .map(|row| str_at(row, &field_pointer))
        .collect()
}

fn require_contains(haystack: &str, needle: &str, label: &str) -> Result<(), String> {
    if haystack.contains(needle) {
        Ok(())
    } else {
        Err(format!("{label} is missing {needle:?}"))
    }
}

fn main() -> Result<(), String> {
    let root = repo_root();
    let verification_file = load_json(&root, PATH2_DIRECT_LIFECYCLE_PROFILE_VERIFICATION_ARTIFACT)?;
    let verification = at(&verification_file, "/verification")?;
    let verification_json = serde_json::to_string_pretty(verification)
        .map_err(|err| format!("serialize lifecycle profile verification: {err}"))?;
    let verification_digest = hex_encode(&Sha256::digest(verification_json.as_bytes()));
    if str_at(&verification_file, "/verification_digest")? != verification_digest {
        return Err("direct lifecycle profile verification digest mismatch".to_string());
    }

    let source = load_text(&root, "tools/parameter-search/src/production_reference.rs")?;
    let lifecycle_model = load_text(&root, "spec/lifecycle_model.md")?;
    let deployment_profiles = load_text(&root, "spec/deployment_profiles.md")?;
    let signer_state = load_text(&root, "spec/signer_state_profile.md")?;
    let adr0010 = load_text(
        &root,
        "decisions/ADR-0010-registry-lifecycle-consensus-wiring-and-finality.md",
    )?;
    let theorem_note = load_text(&root, "research/path2_lifecycle_finality_theorem_v1.md")?;

    let report = path2_direct_lifecycle_boundary_report_v1();
    validate_path2_direct_lifecycle_boundary_report_v1(&report)?;
    let lifecycle_digest = path2_direct_lifecycle_boundary_report_v1_digest(&report);
    let review_evidence = path2_direct_lifecycle_profile_review_evidence_v1()?;

    if str_at(verification, "/schema")? != "Descender.Path2.DirectLifecycleProfileVerification.v1"
        || str_at(verification, "/status")?
            != "digest_pinned_target_chain_lifecycle_profile_verified_not_deployment_ready"
        || str_at(verification, "/verification/verification_id")?
            != PATH2_DIRECT_LIFECYCLE_PROFILE_VERIFICATION_ID
        || str_at(verification, "/verification/artifact_path")?
            != PATH2_DIRECT_LIFECYCLE_PROFILE_VERIFICATION_ARTIFACT
        || str_at(
            verification,
            "/verification/lifecycle_boundary_report_digest",
        )? != hex_encode(&report.digest)
        || str_at(
            verification,
            "/verification/lifecycle_boundary_report_recomputed_digest",
        )? != hex_encode(&lifecycle_digest)
        || str_at(
            verification,
            "/verification/target_chain_finality_profile_id",
        )? != report.target_chain_finality_profile_id
        || str_at(
            verification,
            "/verification/target_chain_finality_profile_authority",
        )? != report.target_chain_finality_profile_authority
        || str_at(
            verification,
            "/verification/target_chain_finality_profile_rule",
        )? != report.target_chain_finality_profile_rule
        || at(verification, "/verification/review_evidence")? != &review_evidence
        || !bool_at(
            verification,
            "/verification/target_chain_finality_profile_instantiated",
        )?
        || !bool_at(
            verification,
            "/verification/target_chain_lifecycle_profile_review_pinned",
        )?
        || !bool_at(
            verification,
            "/verification/target_chain_lifecycle_profile_verification_pinned",
        )?
        || !bool_at(
            verification,
            "/verification/lifecycle_transition_profile_verified",
        )?
        || bool_at(verification, "/verification/adoption_evidence_ready")?
        || bool_at(verification, "/verification/deployment_ready")?
    {
        return Err(
            "direct lifecycle profile verification is not bound to current lifecycle report"
                .to_string(),
        );
    }

    let requirement_ids = field_values_at(verification, "/requirements", "requirement_id")?;
    if requirement_ids != report.target_chain_lifecycle_profile_verification_requirement_ids {
        return Err(
            "direct lifecycle profile verification requirement ids drifted from lifecycle report"
                .to_string(),
        );
    }
    if u64_at(verification, "/requirement_count")?
        != report
            .target_chain_lifecycle_profile_verification_requirement_ids
            .len() as u64
        || u64_at(verification, "/audit_summary/requirement_count")?
            != report
                .target_chain_lifecycle_profile_verification_requirement_ids
                .len() as u64
    {
        return Err("direct lifecycle profile verification count drifted".to_string());
    }

    for pointer in [
        "/audit_summary/all_requirements_reviewed",
        "/audit_summary/all_review_requirements_verified",
        "/audit_summary/all_requirements_have_executable_evidence",
        "/audit_summary/all_support_tests_present",
        "/audit_summary/all_residuals_non_deployment_claims",
        "/audit_summary/d2_safe_default_profile_verified",
        "/audit_summary/target_chain_finality_profile_instantiated",
        "/audit_summary/target_chain_lifecycle_profile_review_pinned",
        "/audit_summary/target_chain_lifecycle_profile_verification_pinned",
        "/audit_summary/lifecycle_transition_profile_verified",
    ] {
        if !bool_at(verification, pointer)? {
            return Err(format!(
                "direct lifecycle profile verification lost {pointer}"
            ));
        }
    }
    for pointer in [
        "/audit_summary/adoption_evidence_ready",
        "/audit_summary/deployment_ready",
    ] {
        if bool_at(verification, pointer)? {
            return Err(format!(
                "direct lifecycle profile verification unexpectedly closes {pointer}"
            ));
        }
    }

    for (index, row) in at(verification, "/requirements")?
        .as_array()
        .ok_or_else(|| "requirements is not an array".to_string())?
        .iter()
        .enumerate()
    {
        for pointer in [
            "/review_requirement_verified",
            "/executable_evidence_verified",
            "/deployment_profile_slice_verified",
        ] {
            if !bool_at(row, pointer)? {
                return Err(format!(
                    "direct lifecycle profile verification row {index} lost {pointer}"
                ));
            }
        }
        if bool_at(row, "/deployment_ready")? {
            return Err(format!(
                "direct lifecycle profile verification row {index} unexpectedly deployable"
            ));
        }
        let residual = str_at(row, "/residual")?;
        if !residual.contains("remain") {
            return Err(format!(
                "direct lifecycle profile verification row {index} residual is not explicit"
            ));
        }
        for support_test in at(row, "/support_tests")?
            .as_array()
            .ok_or_else(|| "support_tests is not an array".to_string())?
        {
            let test_name = support_test
                .as_str()
                .ok_or_else(|| "support test is not a string".to_string())?;
            require_contains(&source, &format!("fn {test_name}"), "support test")?;
        }
    }

    for needle in [
        "finality depth or finality source",
        "mempool-visible or locally signed spends",
        "what happens when a spend is dropped before finality",
        "what happens when a finalized or presumed-final spend is reverted",
        "which data a wallet or HSM must persist before returning a spend payload",
    ] {
        require_contains(&lifecycle_model, needle, "lifecycle profile requirements")?;
    }
    for needle in [
        "A profile becomes adoption evidence only when the corresponding",
        "gas/resource measurements or deterministic resource proofs",
        "data-availability tests or proofs",
        "Until those artifacts exist, deployment surfaces remain below `ready`",
    ] {
        require_contains(&deployment_profiles, needle, "deployment profile non-claim")?;
    }
    for needle in ["restore-anchor", "crash", "host retry", "state/fault"] {
        require_contains(&signer_state, needle, "signer state profile")?;
    }
    for needle in [
        "FINALIZED head",
        "[finalized, head]",
        "reorg_to(new_head)",
        "permanence boundary equals finalized head",
    ] {
        require_contains(&adr0010, needle, "ADR-0010 D2 profile")?;
    }
    for needle in [
        "D2 safe-default target-chain lifecycle profile verification",
        "deployment readiness remains open",
        "lifecycle_transition_profile_verified",
    ] {
        require_contains(&theorem_note, needle, "lifecycle finality theorem note")?;
    }

    for needle in [
        "Path2D2FinalityProfileState",
        "D2 finality profile rejects reorg below finalized head",
        "ProductionSpentSlotLedger",
        "ProductionAccountLifecycleManager",
        "ProductionChainStateFileStores",
        "verify_consume_record_and_save",
        "slot is already in spent ledger",
        "snapshot sequence is older than restore anchor",
    ] {
        require_contains(&source, needle, "lifecycle verification source")?;
    }

    println!("Path 2 direct lifecycle profile verification consistency check passed.");
    Ok(())
}
