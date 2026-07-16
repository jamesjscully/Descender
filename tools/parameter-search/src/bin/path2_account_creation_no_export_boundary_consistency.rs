use descender_parameter_search::production_reference::{
    path2_account_creation_no_export_boundary_report_v1,
    path2_account_creation_no_export_boundary_report_v1_digest,
    path2_account_creation_proof_boundary_report_v1,
    validate_path2_account_creation_no_export_boundary_report_v1,
};
use serde_json::Value;
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

fn require_contains(haystack: &str, needle: &str, label: &str) -> Result<(), String> {
    if haystack.contains(needle) {
        Ok(())
    } else {
        Err(format!("{label} is missing {needle:?}"))
    }
}

fn main() -> Result<(), String> {
    let root = repo_root();
    let manifest = load_json(
        &root,
        "formal/kat/path2_account_creation_no_export_boundary_v1.json",
    )?;
    let source = load_text(&root, "tools/parameter-search/src/production_reference.rs")?;
    let lean_source = load_text(&root, "Descender/Path2NoExportConcrete.lean")?;
    let full_spend_source = load_text(&root, "Descender/Path2FullSpendConcrete.lean")?;
    let account_stark_config = load_text(&root, "tools/descender-account-stark/src/config.rs")?;
    let account_stark_source = load_text(&root, "tools/descender-account-stark/src/lib.rs")?;
    let account_stark_air = load_text(&root, "tools/descender-account-stark/src/air.rs")?;
    let account_stark_reduce =
        load_text(&root, "tools/descender-account-stark/src/reduce_gadget.rs")?;
    let mask_prf_source = load_text(&root, "tools/descender-mask-prf/src/mask.rs")?;
    let account_stark_transport = load_text(&root, "tools/descender-account-verifier/src/lib.rs")?;

    let report = path2_account_creation_no_export_boundary_report_v1();
    let proof_report = path2_account_creation_proof_boundary_report_v1();
    validate_path2_account_creation_no_export_boundary_report_v1(&report)?;
    let report_digest = hex_encode(&report.digest);
    let recomputed_digest = hex_encode(
        &path2_account_creation_no_export_boundary_report_v1_digest(&report),
    );

    if str_at(&manifest, "/manifest/schema")?
        != "Descender.Path2.AccountCreationNoExportBoundaryManifest.v1"
    {
        return Err("account-creation no-export manifest schema mismatch".to_string());
    }
    if str_at(&manifest, "/manifest/frozen_status")? != "draft_no_export_boundary_manifest" {
        return Err("account-creation no-export manifest frozen_status drifted".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/account_creation_no_export_boundary_report_digest",
    )? != report_digest
    {
        return Err("no-export manifest digest is not bound to report".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/account_creation_no_export_boundary_report_recomputed_digest",
    )? != recomputed_digest
    {
        return Err("no-export manifest recomputed digest mismatch".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/digests/proof_boundary_report_no_export_digest",
    )? != hex_encode(&proof_report.no_export_boundary_report_digest)
    {
        return Err("proof-boundary no-export digest is not reflected in manifest".to_string());
    }
    if proof_report.no_export_boundary_report_digest != report.digest {
        return Err("proof-boundary report is not bound to no-export report".to_string());
    }

    for (pointer, expected) in [
        (
            "/manifest/authority_snapshot/support_file",
            report.support_file,
        ),
        (
            "/manifest/authority_snapshot/selected_payload_type",
            report.selected_payload_type,
        ),
        (
            "/manifest/authority_snapshot/verifier_boundary_endpoint",
            report.verifier_boundary_endpoint,
        ),
        (
            "/manifest/authority_snapshot/production_boundary_endpoint",
            report.production_boundary_endpoint,
        ),
        (
            "/manifest/authority_snapshot/no_export_requirement",
            report.no_export_requirement,
        ),
        (
            "/manifest/authority_snapshot/simulator_theorem_target",
            report.simulator_theorem_target,
        ),
        (
            "/manifest/authority_snapshot/no_export_theorem_target",
            report.no_export_theorem_target,
        ),
        (
            "/manifest/authority_snapshot/public_bundle_no_export_lean_theorem",
            report.public_bundle_no_export_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/simulator_input_allowlist_lean_theorem",
            report.simulator_input_allowlist_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/accepted_public_anchor_package_lean_theorem",
            report.accepted_public_anchor_package_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/accepted_bundle_public_anchor_simulation_boundary_lean_theorem",
            report.accepted_bundle_public_anchor_simulation_boundary_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/accepted_bundle_public_bundle_from_public_anchors_lean_theorem",
            report.accepted_bundle_public_bundle_from_public_anchors_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/accepted_bundle_simulator_input_from_public_anchors_lean_theorem",
            report.accepted_bundle_simulator_input_from_public_anchors_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/accepted_bundle_simulator_no_export_reduction_lean_theorem",
            report.accepted_bundle_simulator_no_export_reduction_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/accepted_boundary_simulator_no_export_reduction_lean_theorem",
            report.accepted_boundary_simulator_no_export_reduction_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/mask_translation_bijective_lean_theorem",
            report.mask_translation_bijective_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/mask_prf_domain_separation_lean_theorem",
            report.mask_prf_domain_separation_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/mask_prf_squeeze_index_injective_lean_theorem",
            report.mask_prf_squeeze_index_injective_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/mask_prf_wide_reduction_q128_lean_theorem",
            report.mask_prf_wide_reduction_q128_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/mask_prf_freshness_lean_theorem",
            report.mask_prf_freshness_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/commitment_domain_separation_lean_theorem",
            report.commitment_domain_separation_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/commitment_frame_dimensions_lean_theorem",
            report.commitment_frame_dimensions_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/commitment_qrom_search_q128_lean_theorem",
            report.commitment_qrom_search_q128_lean_theorem,
        ),
        (
            "/manifest/authority_snapshot/named_break",
            report.named_break,
        ),
    ] {
        if str_at(&manifest, pointer)? != expected {
            return Err(format!(
                "account-creation no-export manifest drifted at {pointer}"
            ));
        }
    }
    for (pointer, expected) in [
        (
            "/manifest/no_export_boundary/coverage/payload_public_field_count",
            report.payload_public_field_count,
        ),
        (
            "/manifest/no_export_boundary/coverage/public_payload_field_mutation_count",
            report.public_payload_field_mutation_count,
        ),
        (
            "/manifest/no_export_boundary/coverage/no_export_vector_case_count",
            report.no_export_vector_case_count,
        ),
        (
            "/manifest/no_export_boundary/coverage/hiding_rng_seed_bits",
            report.hiding_rng_seed_bits,
        ),
        (
            "/manifest/no_export_boundary/coverage/mask_prf_domain_lane",
            report.mask_prf_domain_lane as usize,
        ),
        (
            "/manifest/no_export_boundary/coverage/mask_prf_input_lanes",
            report.mask_prf_input_lanes,
        ),
        (
            "/manifest/no_export_boundary/coverage/mask_prf_reduction_digits",
            report.mask_prf_reduction_digits,
        ),
        (
            "/manifest/no_export_boundary/coverage/mask_prf_squeezed_lanes",
            report.mask_prf_squeezed_lanes,
        ),
        (
            "/manifest/no_export_boundary/coverage/mask_prf_target_horizon_log2",
            report.mask_prf_target_horizon_log2,
        ),
        (
            "/manifest/no_export_boundary/coverage/pad_commitment_domain_lane",
            report.pad_commitment_domain_lane as usize,
        ),
        (
            "/manifest/no_export_boundary/coverage/opening_commitment_domain_lane",
            report.opening_commitment_domain_lane as usize,
        ),
        (
            "/manifest/no_export_boundary/coverage/commitment_nonce_bytes",
            report.commitment_nonce_bytes,
        ),
        (
            "/manifest/no_export_boundary/coverage/commitment_nonce_bits",
            report.commitment_nonce_bits,
        ),
        (
            "/manifest/no_export_boundary/coverage/pad_commitment_input_lanes",
            report.pad_commitment_input_lanes,
        ),
        (
            "/manifest/no_export_boundary/coverage/opening_commitment_input_lanes",
            report.opening_commitment_input_lanes,
        ),
        (
            "/manifest/no_export_boundary/coverage/hiding_commitments_per_account",
            report.hiding_commitments_per_account,
        ),
        (
            "/manifest/no_export_boundary/coverage/commitment_target_horizon_log2",
            report.commitment_target_horizon_log2,
        ),
        (
            "/manifest/no_export_boundary/coverage/commitment_qrom_query_log2",
            report.commitment_qrom_query_log2,
        ),
    ] {
        if u64_at(&manifest, pointer)? != expected as u64 {
            return Err(format!(
                "account-creation no-export manifest drifted at {pointer}"
            ));
        }
    }
    for pointer in [
        "/manifest/no_export_boundary/coverage/no_export_field_denylist_pinned",
        "/manifest/no_export_boundary/coverage/simulator_input_allowlist_pinned",
        "/manifest/no_export_boundary/coverage/nested_public_statement_audit_pinned",
        "/manifest/no_export_boundary/coverage/public_syndrome_classified_as_allowed",
        "/manifest/no_export_boundary/coverage/byte_model_public_payload_only",
        "/manifest/no_export_boundary/coverage/boundary_mapping_complete",
        "/manifest/no_export_boundary/coverage/roundtrip_regression_present",
        "/manifest/no_export_boundary/coverage/production_gate_fail_closed",
        "/manifest/no_export_boundary/coverage/no_export_vectors_present",
        "/manifest/no_export_boundary/coverage/public_bundle_no_export_predicate_instantiated",
        "/manifest/no_export_boundary/coverage/simulator_input_allowlist_predicate_instantiated",
        "/manifest/no_export_boundary/coverage/accepted_public_anchor_package_bound",
        "/manifest/no_export_boundary/coverage/accepted_bundle_public_anchor_simulation_boundary_bound",
        "/manifest/no_export_boundary/coverage/accepted_bundle_public_bundle_from_public_anchors_bound",
        "/manifest/no_export_boundary/coverage/accepted_bundle_simulator_input_from_public_anchors_bound",
        "/manifest/no_export_boundary/coverage/accepted_bundle_simulator_no_export_reduction_bound",
        "/manifest/no_export_boundary/coverage/accepted_boundary_simulator_no_export_reduction_bound",
        "/manifest/no_export_boundary/coverage/simulator_hybrid_reduction_bound",
        "/manifest/no_export_boundary/coverage/mask_translation_bijective_bound",
        "/manifest/no_export_boundary/coverage/mask_prf_domain_separation_bound",
        "/manifest/no_export_boundary/coverage/mask_prf_squeeze_index_injective_bound",
        "/manifest/no_export_boundary/coverage/mask_prf_wide_reduction_q128_bound",
        "/manifest/no_export_boundary/coverage/mask_prf_freshness_bound",
        "/manifest/no_export_boundary/coverage/commitment_domain_separation_bound",
        "/manifest/no_export_boundary/coverage/commitment_frame_dimensions_bound",
        "/manifest/no_export_boundary/coverage/commitment_qrom_search_q128_bound",
        "/manifest/no_export_boundary/coverage/deployment_commitment_nonce_csprng_bound",
        "/manifest/no_export_boundary/coverage/hiding_rng_csprng_bound",
        "/manifest/no_export_boundary/coverage/hiding_rng_substreams_domain_separated",
    ] {
        if !bool_at(&manifest, pointer)? {
            return Err(format!(
                "account-creation no-export manifest lost required flag {pointer}"
            ));
        }
    }

    let vector_cases = at(
        &manifest,
        "/manifest/no_export_boundary/no_export_vector_case_ids",
    )?
    .as_array()
    .ok_or_else(|| "no-export manifest vector case ids are not an array".to_string())?;
    if vector_cases.len() != report.no_export_vector_case_ids.len() {
        return Err("no-export manifest vector case count drifted".to_string());
    }
    for (index, expected) in report.no_export_vector_case_ids.iter().enumerate() {
        if vector_cases[index]
            .as_str()
            .ok_or_else(|| format!("no-export vector case {index} is not a string"))?
            != *expected
        {
            return Err(format!("no-export manifest vector case {index} drifted"));
        }
    }
    for pointer in [
        "/manifest/no_export_boundary/coverage/public_payload_exports_private_material",
        "/manifest/no_export_boundary/coverage/public_statement_exports_private_material",
        "/manifest/no_export_boundary/coverage/simulator_theorem_instantiated",
        "/manifest/no_export_boundary/coverage/no_export_theorem_instantiated",
        "/manifest/no_export_boundary/coverage/accepted_bundle_simulatable_from_public_anchors",
        "/manifest/no_export_boundary/coverage/poseidon_mask_prf_theorem_instantiated",
        "/manifest/no_export_boundary/coverage/poseidon_commitment_hiding_theorem_instantiated",
        "/manifest/no_export_boundary/coverage/external_review_complete",
        "/manifest/no_export_boundary/coverage/deployment_ready",
    ] {
        if bool_at(&manifest, pointer)? {
            return Err(format!(
                "account-creation no-export manifest unexpectedly closes {pointer}"
            ));
        }
    }
    if !bool_at(
        &manifest,
        "/manifest/no_export_boundary/coverage/frozen_no_export_vectors_complete",
    )? {
        return Err("account-creation no-export manifest lost frozen vector closure".to_string());
    }
    let statement_fields = at(
        &manifest,
        "/manifest/no_export_boundary/account_stark_public_statement_fields",
    )?
    .as_array()
    .ok_or_else(|| "account-STARK public statement fields are not an array".to_string())?;
    if statement_fields.len() != report.account_stark_public_statement_fields.len()
        || statement_fields
            .iter()
            .zip(report.account_stark_public_statement_fields.iter())
            .any(|(actual, expected)| actual.as_str() != Some(*expected))
    {
        return Err("account-STARK nested public statement field audit drifted".to_string());
    }
    if !report
        .account_stark_public_statement_fields
        .contains(&"syndrome")
        || report
            .no_export_private_material_fields
            .contains(&"syndrome")
    {
        return Err("public checked-core syndrome classification drifted".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/no_export_boundary/public_bundle_no_export_lean_theorem",
    )? != report.public_bundle_no_export_lean_theorem
    {
        return Err("account-creation no-export manifest lost Lean hook".to_string());
    }
    if str_at(
        &manifest,
        "/manifest/no_export_boundary/simulator_input_allowlist_lean_theorem",
    )? != report.simulator_input_allowlist_lean_theorem
    {
        return Err(
            "account-creation no-export manifest lost simulator-input Lean hook".to_string(),
        );
    }
    if str_at(
        &manifest,
        "/manifest/no_export_boundary/accepted_public_anchor_package_lean_theorem",
    )? != report.accepted_public_anchor_package_lean_theorem
    {
        return Err(
            "account-creation no-export manifest lost accepted public-anchor Lean hook".to_string(),
        );
    }
    if str_at(
        &manifest,
        "/manifest/no_export_boundary/accepted_bundle_public_anchor_simulation_boundary_lean_theorem",
    )? != report.accepted_bundle_public_anchor_simulation_boundary_lean_theorem
    {
        return Err(
            "account-creation no-export manifest lost accepted-bundle simulation-boundary Lean hook"
                .to_string(),
        );
    }
    if str_at(
        &manifest,
        "/manifest/no_export_boundary/accepted_bundle_public_bundle_from_public_anchors_lean_theorem",
    )? != report.accepted_bundle_public_bundle_from_public_anchors_lean_theorem
    {
        return Err(
            "account-creation no-export manifest lost accepted-bundle public-bundle projection Lean hook"
                .to_string(),
        );
    }
    if str_at(
        &manifest,
        "/manifest/no_export_boundary/accepted_bundle_simulator_input_from_public_anchors_lean_theorem",
    )? != report.accepted_bundle_simulator_input_from_public_anchors_lean_theorem
    {
        return Err(
            "account-creation no-export manifest lost accepted-bundle simulator-input projection Lean hook"
                .to_string(),
        );
    }
    if str_at(
        &manifest,
        "/manifest/no_export_boundary/accepted_bundle_simulator_no_export_reduction_lean_theorem",
    )? != report.accepted_bundle_simulator_no_export_reduction_lean_theorem
    {
        return Err(
            "account-creation no-export manifest lost accepted-bundle simulator/no-export reduction Lean hook"
                .to_string(),
        );
    }
    if str_at(
        &manifest,
        "/manifest/no_export_boundary/accepted_boundary_simulator_no_export_reduction_lean_theorem",
    )? != report.accepted_boundary_simulator_no_export_reduction_lean_theorem
    {
        return Err(
            "account-creation no-export manifest lost accepted-boundary simulator/no-export reduction Lean hook"
                .to_string(),
        );
    }
    if str_at(
        &manifest,
        "/manifest/no_export_boundary/simulator_hybrid_lean_theorem",
    )? != report.simulator_hybrid_lean_theorem
        || str_at(
            &manifest,
            "/manifest/no_export_boundary/mask_translation_bijective_lean_theorem",
        )? != report.mask_translation_bijective_lean_theorem
        || str_at(
            &manifest,
            "/manifest/no_export_boundary/mask_prf_domain_separation_lean_theorem",
        )? != report.mask_prf_domain_separation_lean_theorem
        || str_at(
            &manifest,
            "/manifest/no_export_boundary/mask_prf_squeeze_index_injective_lean_theorem",
        )? != report.mask_prf_squeeze_index_injective_lean_theorem
        || str_at(
            &manifest,
            "/manifest/no_export_boundary/mask_prf_wide_reduction_q128_lean_theorem",
        )? != report.mask_prf_wide_reduction_q128_lean_theorem
        || str_at(
            &manifest,
            "/manifest/no_export_boundary/mask_prf_freshness_lean_theorem",
        )? != report.mask_prf_freshness_lean_theorem
        || str_at(
            &manifest,
            "/manifest/no_export_boundary/commitment_domain_separation_lean_theorem",
        )? != report.commitment_domain_separation_lean_theorem
        || str_at(
            &manifest,
            "/manifest/no_export_boundary/commitment_frame_dimensions_lean_theorem",
        )? != report.commitment_frame_dimensions_lean_theorem
        || str_at(
            &manifest,
            "/manifest/no_export_boundary/commitment_qrom_search_q128_lean_theorem",
        )? != report.commitment_qrom_search_q128_lean_theorem
        || str_at(
            &manifest,
            "/manifest/no_export_boundary/coverage/hiding_rng_type",
        )? != report.hiding_rng_type
    {
        return Err(
            "account-creation no-export manifest lost simulator hybrid/RNG binding".to_string(),
        );
    }

    for needle in [
        "pub struct ZkCryptoRng(ChaCha20Rng)",
        "impl TryCryptoRng for ZkCryptoRng",
        "ZkCryptoRng",
    ] {
        require_contains(&account_stark_config, needle, "account-STARK config source")?;
    }
    if account_stark_config.contains("use rand::rngs::SmallRng")
        || account_stark_config.contains("HidingChallengeMmcs, SmallRng")
        || account_stark_config.contains("DIGEST_ELEMS, SALT_ELEMS, SmallRng")
    {
        return Err("account-STARK hiding config still selects SmallRng".to_string());
    }
    for needle in [
        "fresh_mask_seed() -> [u8; 32]",
        "b\"input-mmcs\"",
        "b\"fri-mmcs\"",
        "b\"hiding-fri-pcs\"",
        "rand::rng().fill_bytes(&mut seed)",
    ] {
        require_contains(&account_stark_source, needle, "account-STARK prover source")?;
    }
    for needle in [
        "struct PublicWire",
        "syndrome: Vec<u32>",
        "packed_witness_commitment: Vec<u32>",
        "checked_core_key_commitment: Vec<u32>",
    ] {
        require_contains(
            &account_stark_transport,
            needle,
            "account-STARK public statement transport",
        )?;
    }
    for field in report.account_stark_public_statement_fields {
        require_contains(
            &account_stark_transport,
            &format!("{field}:"),
            "account-STARK nested public statement field audit",
        )?;
    }

    let support_doc = load_text(&root, report.support_file)?;
    for needle in [
        "Path2AccountCreationNoExportBoundaryReportV1",
        "zero_knowledge_no_export_semantics",
        "selected_account_stark_public_bundle_simulator_v1",
        "selected_account_stark_no_export_v1",
        "account_stark_simulator_or_no_export_break",
        "Forbidden Exports",
        "Error Path Rule",
        "raw pad seed",
        "private_trace_material",
        "private trace material",
    ] {
        require_contains(&support_doc, needle, report.support_file)?;
    }
    for field in report.no_export_private_material_fields {
        require_contains(&support_doc, field, report.support_file)?;
    }
    for field in report.simulator_public_input_fields {
        require_contains(&support_doc, field, report.support_file)?;
    }

    for needle in [
        "Path2AccountCreationNoExportBoundaryReportV1",
        "path2_account_creation_no_export_boundary_report_v1",
        "PATH2_ACCOUNT_CREATION_SIMULATOR_THEOREM_TARGET",
        "PATH2_ACCOUNT_CREATION_NO_EXPORT_THEOREM_TARGET",
        "PATH2_ACCOUNT_CREATION_PUBLIC_BUNDLE_NO_EXPORT_LEAN_THEOREM",
        "PATH2_ACCOUNT_CREATION_SIMULATOR_INPUT_ALLOWLIST_LEAN_THEOREM",
        "PATH2_ACCOUNT_CREATION_ACCEPTED_PUBLIC_ANCHOR_PACKAGE_LEAN_THEOREM",
        "PATH2_ACCOUNT_CREATION_ACCEPTED_BUNDLE_PUBLIC_ANCHOR_SIMULATION_BOUNDARY_LEAN_THEOREM",
        "PATH2_ACCOUNT_CREATION_ACCEPTED_BUNDLE_PUBLIC_BUNDLE_FROM_PUBLIC_ANCHORS_LEAN_THEOREM",
        "PATH2_ACCOUNT_CREATION_ACCEPTED_BUNDLE_SIMULATOR_INPUT_FROM_PUBLIC_ANCHORS_LEAN_THEOREM",
        "PATH2_ACCOUNT_CREATION_ACCEPTED_BUNDLE_SIMULATOR_NO_EXPORT_REDUCTION_LEAN_THEOREM",
        "PATH2_ACCOUNT_CREATION_ACCEPTED_BOUNDARY_SIMULATOR_NO_EXPORT_REDUCTION_LEAN_THEOREM",
        "PATH2_ACCOUNT_CREATION_SIMULATOR_PUBLIC_INPUT_FIELDS",
        "PATH2_ACCOUNT_CREATION_PROOF_BOUNDARY_NO_EXPORT_PRIVATE_MATERIAL_FIELDS",
        "account_stark_public_statement_bytes_or_digest",
        "pad_seed_nullifier",
        "verify_path2_direct_creation_account_stark_payload_boundary",
        "path2_account_stark_payload_byte_model_report_v1",
        "path2_account_stark_payload_kat_boundary_report_v1",
    ] {
        require_contains(&source, needle, "production reference source")?;
    }
    for needle in [
        report.public_bundle_no_export_lean_theorem,
        report.simulator_input_allowlist_lean_theorem,
        report.accepted_public_anchor_package_lean_theorem,
        report.accepted_bundle_public_anchor_simulation_boundary_lean_theorem,
        report.accepted_bundle_public_bundle_from_public_anchors_lean_theorem,
        report.accepted_bundle_simulator_input_from_public_anchors_lean_theorem,
        report.accepted_bundle_simulator_no_export_reduction_lean_theorem,
        "AccountCreationAcceptedPublicAnchorPackage",
        "PublicBundleInputs",
        "publicBundleFromPublicAnchors",
        "SimulatorPublicInputs",
        "simulatorInputFromPublicAnchors",
        "acceptedBundleSimulatorNoExportWith",
        "selectedRustAcceptedPublicAnchorPackage_surfaces_v1",
        "publicAnchorSimulationBoundary",
        "selectedRustAcceptedBundlePublicBundleFromPublicAnchors_v1",
        "selectedRustAcceptedBundleSimulatorInputFromPublicAnchors_v1",
        "selectedRustAcceptedBundleSimulatorNoExportReduction_v1",
        "selectedRustAcceptedBundlePublicAnchorSimulationBoundary_v1",
        "selectedRustSimulatorInputIndependentOfProofOutput_v1",
        "selectedRustSimulatorInputOutputIndependentOfProofOutput_v1",
        "SelectedAccountStarkSimulatorHybridBounds",
        "advantage_le_totalError",
        report.simulator_hybrid_lean_theorem,
        "SelectedAccountProjectionMaskVector",
        "selectedAccountProjectionMaskTranslation",
        report.mask_translation_bijective_lean_theorem,
        "selectedRustUniformMaskTranslationBijective_v1",
        "selectedRustUniformMaskTranslationCollisionFree_v1",
        "selectedAccountMaskPrfDomainsSeparated_v1",
        "selectedAccountMaskPrfSqueezeIndexInjective_v1",
        "selectedAccountMaskWideReductionError_le_q128_v1",
        "SelectedAccountMaskPrfHybridBounds",
        "advantage_le_poseidonPrfError_add_q128",
        "selectedAccountCommitmentDomainsSeparated_v1",
        "selectedAccountCommitmentFrameDimensions_v1",
        "selectedAccountCommitmentQromSearchError_le_q128_v1",
        "SelectedAccountCommitmentHidingBounds",
        "advantage_le_poseidonCommitmentError_add_q128",
    ] {
        require_contains(&lean_source, needle, "Path2NoExportConcrete Lean source")?;
    }
    for needle in [
        report.accepted_boundary_simulator_no_export_reduction_lean_theorem,
        "SelectedRustAcceptedBoundarySimulatorNoExportWith",
        "selectedRustAcceptedBoundarySimulatorNoExportReduction_v1",
        report.mask_prf_freshness_lean_theorem,
        "SelectedAccountMaskPrfAdmissionFramesFresh",
        "selectedRustAccountMaskPrfFramesFresh_of_verifiedNullifierRegistry_v1",
    ] {
        require_contains(
            &full_spend_source,
            needle,
            "Path2FullSpendConcrete Lean source",
        )?;
    }

    for needle in [
        "pub const MASK_PRF_DOMAIN: u32 = 0x4D41_534B",
        "pub const C2_INPUT_LANES: usize = 1 + 32 + 8",
        "ce::<AB>(MASK_PRF_DOMAIN as u64)",
        "local[lay.c2_off + lay.c2.lanes + 1 + b]",
    ] {
        require_contains(&account_stark_air, needle, "account-STARK AIR source")?;
    }
    for needle in [
        "pub const COMMITMENT_NONCE_BYTES: usize = 40",
        "pub const PAD_COMMITMENT_DOMAIN: u32 = 0x5041_4453",
        "pub const OPENING_COMMITMENT_DOMAIN: u32 = 0x4F50_454E",
        "ce::<AB>(PAD_COMMITMENT_DOMAIN as u64)",
        "ce::<AB>(OPENING_COMMITMENT_DOMAIN as u64)",
    ] {
        require_contains(&account_stark_air, needle, "account-STARK AIR source")?;
    }
    for needle in [
        "fn with_fresh_commitment_nonces",
        "rng.fill_bytes(&mut randomized.c1_nonce)",
        "rng.fill_bytes(&mut randomized.c6_nonce)",
        "let randomized = with_fresh_commitment_nonces(secrets)",
    ] {
        require_contains(
            &account_stark_source,
            needle,
            "account-STARK deployment prover source",
        )?;
    }
    for needle in [
        "pub const LANES_PER_MASK: usize = 6",
        "pub const NUM_SQUEEZE: usize = NUM_OUTPUTS * LANES_PER_MASK",
    ] {
        require_contains(
            &account_stark_reduce,
            needle,
            "account-STARK reduction source",
        )?;
        require_contains(&mask_prf_source, needle, "standalone mask-PRF source")?;
    }

    println!("Path 2 account-creation no-export boundary consistency check passed.");
    Ok(())
}
