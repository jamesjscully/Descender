#![recursion_limit = "256"]

use descender_parameter_search::production_reference::{
    PROFILE, path2_account_creation_no_export_boundary_report_v1,
    path2_account_creation_no_export_boundary_report_v1_digest,
    path2_account_creation_proof_boundary_report_v1,
    validate_path2_account_creation_no_export_boundary_report_v1,
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

fn manifest_body() -> Result<Value, String> {
    let report = path2_account_creation_no_export_boundary_report_v1();
    validate_path2_account_creation_no_export_boundary_report_v1(&report)?;
    let proof_report = path2_account_creation_proof_boundary_report_v1();
    let recomputed_digest = path2_account_creation_no_export_boundary_report_v1_digest(&report);
    if recomputed_digest != report.digest {
        return Err("account-creation no-export boundary report digest mismatch".to_string());
    }
    if proof_report.no_export_boundary_report_digest != report.digest {
        return Err("proof-boundary report is not bound to no-export report".to_string());
    }

    Ok(json!({
        "schema": "Descender.Path2.AccountCreationNoExportBoundaryManifest.v1",
        "corpus_id": "Descender.Path2.AccountCreationNoExportBoundary.v1",
        "corpus_version": 1,
        "profile_id": PROFILE,
        "frozen_status": "draft_no_export_boundary_manifest",
        "generator": {
            "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin path2_account_creation_no_export_boundary_manifest",
            "source": "tools/parameter-search/src/bin/path2_account_creation_no_export_boundary_manifest.rs"
        },
        "authority_snapshot": {
            "no_export_report_domain": report.domain,
            "proof_boundary_report_domain": proof_report.domain,
            "support_file": report.support_file,
            "selected_payload_type": report.selected_payload_type,
            "verifier_boundary_endpoint": report.verifier_boundary_endpoint,
            "production_boundary_endpoint": report.production_boundary_endpoint,
            "no_export_requirement": report.no_export_requirement,
            "simulator_theorem_target": report.simulator_theorem_target,
            "no_export_theorem_target": report.no_export_theorem_target,
            "public_bundle_no_export_lean_theorem":
                report.public_bundle_no_export_lean_theorem,
            "simulator_input_allowlist_lean_theorem":
                report.simulator_input_allowlist_lean_theorem,
            "accepted_public_anchor_package_lean_theorem":
                report.accepted_public_anchor_package_lean_theorem,
            "accepted_bundle_public_anchor_simulation_boundary_lean_theorem":
                report.accepted_bundle_public_anchor_simulation_boundary_lean_theorem,
            "accepted_bundle_public_bundle_from_public_anchors_lean_theorem":
                report.accepted_bundle_public_bundle_from_public_anchors_lean_theorem,
            "accepted_bundle_simulator_input_from_public_anchors_lean_theorem":
                report.accepted_bundle_simulator_input_from_public_anchors_lean_theorem,
            "accepted_bundle_simulator_no_export_reduction_lean_theorem":
                report.accepted_bundle_simulator_no_export_reduction_lean_theorem,
            "accepted_boundary_simulator_no_export_reduction_lean_theorem":
                report.accepted_boundary_simulator_no_export_reduction_lean_theorem,
            "mask_translation_bijective_lean_theorem":
                report.mask_translation_bijective_lean_theorem,
            "mask_prf_domain_separation_lean_theorem":
                report.mask_prf_domain_separation_lean_theorem,
            "mask_prf_squeeze_index_injective_lean_theorem":
                report.mask_prf_squeeze_index_injective_lean_theorem,
            "mask_prf_wide_reduction_q128_lean_theorem":
                report.mask_prf_wide_reduction_q128_lean_theorem,
            "mask_prf_freshness_lean_theorem": report.mask_prf_freshness_lean_theorem,
            "commitment_domain_separation_lean_theorem":
                report.commitment_domain_separation_lean_theorem,
            "commitment_frame_dimensions_lean_theorem":
                report.commitment_frame_dimensions_lean_theorem,
            "commitment_qrom_search_q128_lean_theorem":
                report.commitment_qrom_search_q128_lean_theorem,
            "named_break": report.named_break,
            "blocker": report.blocker
        },
        "digests": {
            "account_creation_no_export_boundary_report_digest": hex_encode(&report.digest),
            "account_creation_no_export_boundary_report_recomputed_digest":
                hex_encode(&recomputed_digest),
            "account_creation_proof_boundary_report_digest": hex_encode(&proof_report.digest),
            "proof_boundary_report_no_export_digest":
                hex_encode(&proof_report.no_export_boundary_report_digest),
            "payload_byte_model_report_digest":
                hex_encode(&report.payload_byte_model_report_digest),
            "payload_kat_boundary_report_digest":
                hex_encode(&report.payload_kat_boundary_report_digest)
        },
        "no_export_boundary": {
            "coverage": {
                "payload_public_field_count": report.payload_public_field_count,
                "public_payload_field_mutation_count":
                    report.public_payload_field_mutation_count,
                "no_export_vector_case_count": report.no_export_vector_case_count,
                "public_payload_exports_private_material":
                    report.public_payload_exports_private_material,
                "public_statement_exports_private_material":
                    report.public_statement_exports_private_material,
                "no_export_field_denylist_pinned": report.no_export_field_denylist_pinned,
                "simulator_input_allowlist_pinned":
                    report.simulator_input_allowlist_pinned,
                "nested_public_statement_audit_pinned":
                    report.nested_public_statement_audit_pinned,
                "public_syndrome_classified_as_allowed":
                    report.public_syndrome_classified_as_allowed,
                "byte_model_public_payload_only": report.byte_model_public_payload_only,
                "boundary_mapping_complete": report.boundary_mapping_complete,
                "roundtrip_regression_present": report.roundtrip_regression_present,
                "production_gate_fail_closed": report.production_gate_fail_closed,
                "no_export_vectors_present": report.no_export_vectors_present,
                "public_bundle_no_export_predicate_instantiated":
                    report.public_bundle_no_export_predicate_instantiated,
                "simulator_input_allowlist_predicate_instantiated":
                    report.simulator_input_allowlist_predicate_instantiated,
                "accepted_public_anchor_package_bound":
                    report.accepted_public_anchor_package_bound,
                "accepted_bundle_public_anchor_simulation_boundary_bound":
                    report.accepted_bundle_public_anchor_simulation_boundary_bound,
                "accepted_bundle_public_bundle_from_public_anchors_bound":
                    report.accepted_bundle_public_bundle_from_public_anchors_bound,
                "accepted_bundle_simulator_input_from_public_anchors_bound":
                    report.accepted_bundle_simulator_input_from_public_anchors_bound,
                "accepted_bundle_simulator_no_export_reduction_bound":
                    report.accepted_bundle_simulator_no_export_reduction_bound,
                "accepted_boundary_simulator_no_export_reduction_bound":
                    report.accepted_boundary_simulator_no_export_reduction_bound,
                "simulator_hybrid_reduction_bound":
                    report.simulator_hybrid_reduction_bound,
                "mask_translation_bijective_bound":
                    report.mask_translation_bijective_bound,
                "mask_prf_domain_lane": report.mask_prf_domain_lane,
                "mask_prf_input_lanes": report.mask_prf_input_lanes,
                "mask_prf_reduction_digits": report.mask_prf_reduction_digits,
                "mask_prf_squeezed_lanes": report.mask_prf_squeezed_lanes,
                "mask_prf_target_horizon_log2": report.mask_prf_target_horizon_log2,
                "mask_prf_domain_separation_bound":
                    report.mask_prf_domain_separation_bound,
                "mask_prf_squeeze_index_injective_bound":
                    report.mask_prf_squeeze_index_injective_bound,
                "mask_prf_wide_reduction_q128_bound":
                    report.mask_prf_wide_reduction_q128_bound,
                "mask_prf_freshness_bound": report.mask_prf_freshness_bound,
                "poseidon_mask_prf_theorem_instantiated":
                    report.poseidon_mask_prf_theorem_instantiated,
                "pad_commitment_domain_lane": report.pad_commitment_domain_lane,
                "opening_commitment_domain_lane": report.opening_commitment_domain_lane,
                "commitment_nonce_bytes": report.commitment_nonce_bytes,
                "commitment_nonce_bits": report.commitment_nonce_bits,
                "pad_commitment_input_lanes": report.pad_commitment_input_lanes,
                "opening_commitment_input_lanes": report.opening_commitment_input_lanes,
                "hiding_commitments_per_account": report.hiding_commitments_per_account,
                "commitment_target_horizon_log2": report.commitment_target_horizon_log2,
                "commitment_qrom_query_log2": report.commitment_qrom_query_log2,
                "commitment_domain_separation_bound":
                    report.commitment_domain_separation_bound,
                "commitment_frame_dimensions_bound":
                    report.commitment_frame_dimensions_bound,
                "commitment_qrom_search_q128_bound":
                    report.commitment_qrom_search_q128_bound,
                "deployment_commitment_nonce_csprng_bound":
                    report.deployment_commitment_nonce_csprng_bound,
                "poseidon_commitment_hiding_theorem_instantiated":
                    report.poseidon_commitment_hiding_theorem_instantiated,
                "hiding_rng_type": report.hiding_rng_type,
                "hiding_rng_seed_bits": report.hiding_rng_seed_bits,
                "hiding_rng_csprng_bound": report.hiding_rng_csprng_bound,
                "hiding_rng_substreams_domain_separated":
                    report.hiding_rng_substreams_domain_separated,
                "simulator_theorem_instantiated": report.simulator_theorem_instantiated,
                "no_export_theorem_instantiated": report.no_export_theorem_instantiated,
                "accepted_bundle_simulatable_from_public_anchors":
                    report.accepted_bundle_simulatable_from_public_anchors,
                "frozen_no_export_vectors_complete": report.frozen_no_export_vectors_complete,
                "external_review_complete": report.external_review_complete,
                "deployment_ready": report.deployment_ready,
                "blocker": report.blocker
            },
            "simulator_theorem_target": report.simulator_theorem_target,
            "no_export_theorem_target": report.no_export_theorem_target,
            "public_bundle_no_export_lean_theorem":
                report.public_bundle_no_export_lean_theorem,
            "simulator_input_allowlist_lean_theorem":
                report.simulator_input_allowlist_lean_theorem,
            "accepted_public_anchor_package_lean_theorem":
                report.accepted_public_anchor_package_lean_theorem,
            "accepted_bundle_public_anchor_simulation_boundary_lean_theorem":
                report.accepted_bundle_public_anchor_simulation_boundary_lean_theorem,
            "accepted_bundle_public_bundle_from_public_anchors_lean_theorem":
                report.accepted_bundle_public_bundle_from_public_anchors_lean_theorem,
            "accepted_bundle_simulator_input_from_public_anchors_lean_theorem":
                report.accepted_bundle_simulator_input_from_public_anchors_lean_theorem,
            "accepted_bundle_simulator_no_export_reduction_lean_theorem":
                report.accepted_bundle_simulator_no_export_reduction_lean_theorem,
            "accepted_boundary_simulator_no_export_reduction_lean_theorem":
                report.accepted_boundary_simulator_no_export_reduction_lean_theorem,
            "simulator_hybrid_lean_theorem": report.simulator_hybrid_lean_theorem,
            "mask_translation_bijective_lean_theorem":
                report.mask_translation_bijective_lean_theorem,
            "mask_prf_domain_separation_lean_theorem":
                report.mask_prf_domain_separation_lean_theorem,
            "mask_prf_squeeze_index_injective_lean_theorem":
                report.mask_prf_squeeze_index_injective_lean_theorem,
            "mask_prf_wide_reduction_q128_lean_theorem":
                report.mask_prf_wide_reduction_q128_lean_theorem,
            "mask_prf_freshness_lean_theorem": report.mask_prf_freshness_lean_theorem,
            "commitment_domain_separation_lean_theorem":
                report.commitment_domain_separation_lean_theorem,
            "commitment_frame_dimensions_lean_theorem":
                report.commitment_frame_dimensions_lean_theorem,
            "commitment_qrom_search_q128_lean_theorem":
                report.commitment_qrom_search_q128_lean_theorem,
            "no_export_vector_case_ids": report.no_export_vector_case_ids,
            "simulator_public_input_fields": report.simulator_public_input_fields,
            "account_stark_public_statement_fields":
                report.account_stark_public_statement_fields,
            "no_export_private_material_fields": report.no_export_private_material_fields
        }
    }))
}

fn main() -> Result<(), String> {
    let body = manifest_body()?;
    let body_json = serde_json::to_string_pretty(&body)
        .map_err(|err| format!("failed to serialize manifest body: {err}"))?;
    let manifest = json!({
        "manifest_digest": hex_encode(&Sha256::digest(body_json.as_bytes())),
        "manifest": body
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&manifest)
            .map_err(|err| format!("failed to serialize manifest: {err}"))?
    );
    Ok(())
}
