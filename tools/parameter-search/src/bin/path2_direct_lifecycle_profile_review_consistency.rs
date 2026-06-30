use descender_parameter_search::production_reference::{
    PATH2_DIRECT_LIFECYCLE_PROFILE_REVIEW_ARTIFACT, PATH2_DIRECT_LIFECYCLE_PROFILE_REVIEW_ID,
    path2_direct_lifecycle_boundary_report_v1, path2_direct_lifecycle_boundary_report_v1_digest,
    validate_path2_direct_lifecycle_boundary_report_v1,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
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

fn string_vec_at(value: &Value, pointer: &str) -> Result<Vec<String>, String> {
    at(value, pointer)?
        .as_array()
        .ok_or_else(|| format!("JSON pointer {pointer} is not an array"))?
        .iter()
        .enumerate()
        .map(|(index, item)| {
            item.as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| format!("{pointer}[{index}] is not a string"))
        })
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
    let review_file = load_json(&root, PATH2_DIRECT_LIFECYCLE_PROFILE_REVIEW_ARTIFACT)?;
    let review = at(&review_file, "/review")?;
    let review_json = serde_json::to_string_pretty(review)
        .map_err(|err| format!("serialize lifecycle profile review body: {err}"))?;
    let recomputed_review_digest = hex_encode(&Sha256::digest(review_json.as_bytes()));
    if str_at(&review_file, "/review_digest")? != recomputed_review_digest {
        return Err("lifecycle profile review digest mismatch".to_string());
    }

    let report = path2_direct_lifecycle_boundary_report_v1();
    validate_path2_direct_lifecycle_boundary_report_v1(&report)?;
    let lifecycle_digest = path2_direct_lifecycle_boundary_report_v1_digest(&report);
    if lifecycle_digest != report.digest {
        return Err("direct lifecycle boundary report digest mismatch".to_string());
    }

    if str_at(review, "/schema")? != "Descender.Path2.DirectLifecycleProfileReview.v1" {
        return Err("lifecycle profile review schema mismatch".to_string());
    }
    if str_at(review, "/status")?
        != "digest_pinned_target_chain_lifecycle_profile_review_not_deployment_ready"
    {
        return Err("lifecycle profile review status mismatch".to_string());
    }
    if str_at(review, "/review/review_id")? != PATH2_DIRECT_LIFECYCLE_PROFILE_REVIEW_ID
        || str_at(review, "/review/artifact_path")?
            != PATH2_DIRECT_LIFECYCLE_PROFILE_REVIEW_ARTIFACT
        || str_at(review, "/review/lifecycle_boundary_report_digest")? != hex_encode(&report.digest)
        || str_at(
            review,
            "/review/lifecycle_boundary_report_recomputed_digest",
        )? != hex_encode(&lifecycle_digest)
        || str_at(review, "/review/target_chain_finality_profile_id")?
            != report.target_chain_finality_profile_id
        || str_at(review, "/review/target_chain_finality_profile_authority")?
            != report.target_chain_finality_profile_authority
        || str_at(review, "/review/target_chain_finality_profile_rule")?
            != report.target_chain_finality_profile_rule
        || str_at(review, "/review/blocker")? != report.blocker
    {
        return Err("lifecycle profile review is not bound to lifecycle report".to_string());
    }
    if !bool_at(review, "/review/target_chain_finality_profile_instantiated")?
        || !bool_at(
            review,
            "/review/target_chain_lifecycle_profile_review_pinned",
        )?
        || !bool_at(review, "/review/lifecycle_transition_profile_verified")?
        || bool_at(review, "/review/deployment_ready")?
    {
        return Err("lifecycle profile review has unexpected readiness flags".to_string());
    }

    let expected_requirement_ids = report
        .target_chain_lifecycle_profile_review_requirement_ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>();
    if string_vec_at(review, "/audit_summary/requirement_ids")? != expected_requirement_ids {
        return Err("lifecycle profile review requirement ids drifted".to_string());
    }
    if u64_at(review, "/requirement_count")? != expected_requirement_ids.len() as u64
        || u64_at(review, "/audit_summary/requirement_count")?
            != expected_requirement_ids.len() as u64
    {
        return Err("lifecycle profile review requirement count drifted".to_string());
    }
    for pointer in [
        "/audit_summary/all_requirements_have_authority",
        "/audit_summary/all_requirements_reviewed",
        "/audit_summary/all_reviewed_requirements_have_evidence_surface",
        "/audit_summary/all_relaxations_require_new_profile",
        "/audit_summary/d2_safe_default_profile_reviewed",
        "/audit_summary/target_chain_finality_profile_instantiated",
        "/audit_summary/target_chain_lifecycle_profile_review_pinned",
        "/audit_summary/lifecycle_transition_profile_verified",
    ] {
        if !bool_at(review, pointer)? {
            return Err(format!(
                "lifecycle profile review lost expected true flag {pointer}"
            ));
        }
    }
    for pointer in [
        "/audit_summary/adoption_evidence_ready",
        "/audit_summary/deployment_ready",
    ] {
        if bool_at(review, pointer)? {
            return Err(format!(
                "lifecycle profile review unexpectedly closes {pointer}"
            ));
        }
    }

    let report_tests = report
        .executable_tests
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let requirements = at(review, "/requirements")?
        .as_array()
        .ok_or_else(|| "lifecycle profile review requirements are not an array".to_string())?;
    if requirements.len() != expected_requirement_ids.len() {
        return Err("lifecycle profile review requirements array drifted".to_string());
    }
    for (index, (row, expected)) in requirements
        .iter()
        .zip(expected_requirement_ids.iter())
        .enumerate()
    {
        let requirement_id = str_at(row, "/requirement_id")?;
        if requirement_id != expected {
            return Err(format!(
                "lifecycle profile review requirement {index} id {requirement_id:?} != {expected:?}"
            ));
        }
        if !bool_at(row, "/reviewed")? {
            return Err(format!(
                "lifecycle profile review requirement {requirement_id} lost reviewed flag"
            ));
        }
        if bool_at(row, "/adoption_evidence_ready")? {
            return Err(format!(
                "lifecycle profile review requirement {requirement_id} unexpectedly claims adoption evidence"
            ));
        }
        if str_at(row, "/normative_source")?.is_empty()
            || str_at(row, "/profile_decision")?.is_empty()
            || str_at(row, "/evidence_surface")?.is_empty()
            || str_at(row, "/residual")?.is_empty()
        {
            return Err(format!(
                "lifecycle profile review requirement {requirement_id} has empty review fields"
            ));
        }
        for support in string_vec_at(row, "/support_tests")? {
            if !report_tests.contains(support.as_str()) {
                return Err(format!(
                    "lifecycle profile review requirement {requirement_id} references unpinned support test {support}"
                ));
            }
        }
    }

    let lifecycle_model = load_text(&root, "spec/lifecycle_model.md")?;
    let deployment_profiles = load_text(&root, "spec/deployment_profiles.md")?;
    let lifecycle_theorem = load_text(&root, "research/path2_lifecycle_finality_theorem_v1.md")?;
    let adr0010 = load_text(
        &root,
        "decisions/ADR-0010-registry-lifecycle-consensus-wiring-and-finality.md",
    )?;
    for (needle, label) in [
        (
            "finality depth or finality source",
            "lifecycle profile finality requirement",
        ),
        (
            "what happens when a spend is dropped before finality",
            "dropped-spend policy",
        ),
        (
            "what happens when a finalized or presumed-final spend is reverted",
            "finalized reorg policy",
        ),
        (
            "which data a wallet or HSM must persist before returning a spend payload",
            "persist-before-release policy",
        ),
    ] {
        require_contains(&lifecycle_model, needle, label)?;
    }
    for (needle, label) in [
        (
            "A profile becomes adoption evidence only when the corresponding",
            "deployment profile readiness rule",
        ),
        ("lifecycle_profile", "deployment profile lifecycle field"),
        (
            "Before a deployment profile can be marked `verified`",
            "deployment verification rule",
        ),
    ] {
        require_contains(&deployment_profiles, needle, label)?;
    }
    for (needle, label) in [
        (
            "Safe default (baseline `A_finality`)",
            "safe default profile theorem",
        ),
        (
            "D2 safe-default target-chain lifecycle profile verification",
            "profile verification evidence",
        ),
        (
            "lifecycle_transition_profile_verified",
            "lifecycle verification flag",
        ),
    ] {
        require_contains(&lifecycle_theorem, needle, label)?;
    }
    for (needle, label) in [
        ("FINALIZED head", "D2 finalized-head profile"),
        ("[finalized, head]", "D2 reorg-freeable window"),
        ("reorg_to(new_head)", "D2 reorg rule"),
    ] {
        require_contains(&adr0010, needle, label)?;
    }

    println!("Path 2 direct lifecycle profile review consistency check passed.");
    Ok(())
}
