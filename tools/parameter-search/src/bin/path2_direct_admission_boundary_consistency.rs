use descender_parameter_search::production_reference::{
    path2_direct_admission_boundary_report_v1, path2_direct_admission_boundary_report_v1_digest,
    path2_direct_theorem_boundary_report_v1, validate_path2_direct_admission_boundary_report_v1,
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
    fs::read_to_string(root.join(relative_path))
        .map_err(|err| format!("read {relative_path}: {err}"))
}

fn load_json(root: &Path, relative_path: &str) -> Result<Value, String> {
    serde_json::from_str(&load_text(root, relative_path)?)
        .map_err(|err| format!("parse {relative_path}: {err}"))
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

fn require_contains(haystack: &str, needle: &str, label: &str) -> Result<(), String> {
    if haystack.contains(needle) {
        Ok(())
    } else {
        Err(format!("{label} is missing {needle:?}"))
    }
}

fn main() -> Result<(), String> {
    let root = repo_root();
    let manifest = load_json(&root, "formal/kat/path2_direct_admission_boundary_v1.json")?;
    let rust_source = load_text(&root, "tools/parameter-search/src/production_reference.rs")?;
    let lean_source = load_text(&root, "Descender/Path2FullSpendConcrete.lean")?;
    let deployment_spec = load_text(&root, "spec/deployment_profiles.md")?;
    let adr0009 = load_text(
        &root,
        "decisions/ADR-0009-native-sovereign-pq-rollup-settlement.md",
    )?;
    let adr0010 = load_text(
        &root,
        "decisions/ADR-0010-registry-lifecycle-consensus-wiring-and-finality.md",
    )?;
    let report = path2_direct_admission_boundary_report_v1();
    let direct_report = path2_direct_theorem_boundary_report_v1();
    validate_path2_direct_admission_boundary_report_v1(&report)?;

    if str_at(&manifest, "/manifest/schema")?
        != "Descender.Path2.DirectAdmissionBoundaryManifest.v1"
        || str_at(&manifest, "/manifest/frozen_status")?
            != "closed_theorem_scoped_admission_boundary_not_deployment_ready"
        || str_at(
            &manifest,
            "/manifest/digests/direct_admission_boundary_report_digest",
        )? != hex_encode(&report.digest)
        || str_at(
            &manifest,
            "/manifest/digests/direct_admission_boundary_report_recomputed_digest",
        )? != hex_encode(&path2_direct_admission_boundary_report_v1_digest(&report))
        || str_at(
            &manifest,
            "/manifest/digests/direct_theorem_report_admission_digest",
        )? != hex_encode(&direct_report.direct_admission_boundary_report_digest)
    {
        return Err("direct admission boundary manifest digest or status drifted".to_string());
    }
    if direct_report.direct_admission_boundary_report_digest != report.digest {
        return Err("direct theorem report is not bound to admission boundary".to_string());
    }

    for pointer in [
        "/manifest/consensus_state_contract/exact_namespace_tuple_key_pinned",
        "/manifest/consensus_state_contract/canonical_leaf_bytes_stored_directly",
        "/manifest/consensus_state_contract/leaf_and_evidence_digests_rechecked",
        "/manifest/consensus_state_contract/spend_reads_canonical_consensus_record",
        "/manifest/consensus_state_contract/missing_record_rejected",
        "/manifest/consensus_state_contract/corrupt_record_fails_closed",
        "/manifest/consensus_state_contract/duplicate_admission_rejected",
        "/manifest/consensus_state_contract/d2_finalized_head_reorg_semantics",
        "/manifest/consensus_state_contract/verified_creation_nullifier_anchor_bound",
        "/manifest/consensus_state_contract/sovereign_nullifier_registry_consensus_wired",
        "/manifest/consensus_state_contract/sovereign_nullifier_block_atomic",
        "/manifest/consensus_state_contract/sovereign_nullifier_d2_reorg_index_rebuild_verified",
        "/manifest/consensus_state_contract/pad_seed_reorg_safe_uniqueness_selected_state_reduction_imported",
        "/manifest/consensus_state_contract/snapshot_restores_nullifier_registry",
        "/manifest/consensus_state_contract/snapshot_restore_anchor_bound",
        "/manifest/consensus_state_contract/target_chain_data_availability_instantiated",
        "/manifest/consensus_state_contract/admission_closure_lean_imported",
        "/manifest/consensus_state_contract/direct_admission_namespace_closed_by_sovereign_state",
    ] {
        if !bool_at(&manifest, pointer)? {
            return Err(format!(
                "direct admission boundary lost closure flag {pointer}"
            ));
        }
    }
    for pointer in [
        "/manifest/consensus_state_contract/external_storage_review_complete",
        "/manifest/consensus_state_contract/deployment_ready",
    ] {
        if bool_at(&manifest, pointer)? {
            return Err(format!(
                "direct admission boundary unexpectedly enabled {pointer}"
            ));
        }
    }

    for marker in [
        "Path2SovereignDirectAdmissionStateV1",
        "Path2SovereignDirectAdmissionRecordV1",
        "Path2VerifiedSovereignCreationAdmissionV1",
        "Path2DirectAdmissionNamespaceKeyV1",
        "verify_path2_direct_creation_account_stark_admission_candidate_v1",
        "verify_and_admit_path2_direct_creation_account_stark_block_v1",
        "pub fn admit_verified_creation_block(",
        "candidate_head <= self.finalized_head",
        "let mut next_state = self.clone()",
        "next_state.head = candidate_head",
        "*self = next_state",
        "Path2SovereignDirectAdmissionRestoreAnchorV1",
        "pub fn restore_anchor(&self)",
        "Path 2 sovereign admission restore anchor mismatch",
        "pub fn require_available_record(",
        "record.canonical_leaf_bytes != canonical_leaf_bytes",
        "record.evidence_transcript_digest != evidence_transcript_digest",
        "record.pad_seed_nullifier",
        "verify_path2_direct_single_slot_spend_authorization_from_sovereign_state_v1",
        "admission_state.require_available_record",
        "verify_path2_direct_single_slot_spend_authorization_with_tuple_evidence_from_sovereign_state_v1",
        "admission_state.admit_verified_creation_block(",
        "new_head < self.finalized_head",
    ] {
        require_contains(&rust_source, marker, "production reference")?;
    }
    for test in report.executable_tests {
        require_contains(&rust_source, test, "production reference tests")?;
    }
    if rust_source.contains("pub fn admit_verified_creation(")
        || rust_source.contains("pub fn verify_and_admit_path2_direct_creation_account_stark_v1(")
    {
        return Err(
            "single-candidate admission must not remain a public consensus API".to_string(),
        );
    }
    for marker in [
        "SelectedRustDirectAdmissionNamespaceAndDataAvailabilityClosure",
        "selectedRustDirectAdmissionNamespaceAndDataAvailabilityClosure_of_concreteSpendAccepts",
        "ReplayableCertificateDataAvailable selectedRustConcreteGameConfig spend",
        "admissionNamespaceAndDataAvailabilityClosed",
        "SelectedRustSovereignVerifiedNullifierRegistryContract",
        "selectedRustSovereignPadSeedReorgSafeUniqueness_of_verifiedNullifierRegistry_v1",
    ] {
        require_contains(&lean_source, marker, "Lean admission closure")?;
    }
    for marker in [
        "exact `slot_authorization_digest` lookup key and storage scope",
        "state storage and data availability for slot records",
    ] {
        require_contains(&deployment_spec, marker, "deployment profile spec")?;
    }
    require_contains(
        &adr0009,
        "project owns consensus, data availability, and",
        "ADR-0009",
    )?;
    require_contains(&adr0010, "canonical (permanent) boundary", "ADR-0010")?;

    let state_start = rust_source
        .find("\nimpl Path2SovereignDirectAdmissionStateV1")
        .ok_or_else(|| {
            "production reference is missing sovereign admission state impl".to_string()
        })?;
    let state_end = rust_source[state_start..]
        .find("\n#[cfg(test)]\nfn path2_test_verified_sovereign_creation_admission_v1")
        .ok_or_else(|| {
            "production reference is missing sovereign admission state impl end".to_string()
        })?;
    let state_window = &rust_source[state_start..state_start + state_end];
    require_contains(
        state_window,
        "candidate.pad_seed_nullifier()",
        "sovereign admission state",
    )?;
    if state_window.contains("leaf.pad_seed_digest") {
        return Err(
            "sovereign nullifier registry must not key on caller-controlled leaf.pad_seed_digest"
                .to_string(),
        );
    }

    let body = at(&manifest, "/manifest")?;
    let body_json = serde_json::to_string_pretty(body)
        .map_err(|err| format!("serialize manifest body: {err}"))?;
    if str_at(&manifest, "/manifest_digest")? != hex_encode(&Sha256::digest(body_json.as_bytes())) {
        return Err("direct admission boundary manifest digest mismatch".to_string());
    }
    println!("Path 2 direct admission boundary consistency check passed.");
    Ok(())
}
