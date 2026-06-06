#![recursion_limit = "256"]

use descender_parameter_search::production_reference::{
    path2_reviewed_fq12_boundary_base_evidence_all_facts_true,
    path2_reviewed_fq12_boundary_base_evidence_v1,
    path2_reviewed_fq12_boundary_base_evidence_v1_digest,
    path2_reviewed_fq12_field_local_evidence_all_facts_true,
    path2_reviewed_fq12_field_local_evidence_v1,
    path2_reviewed_fq12_field_local_evidence_v1_digest,
    path2_reviewed_fq12_five_source_closure_evidence_all_facts_true,
    path2_reviewed_fq12_five_source_closure_evidence_v1,
    path2_reviewed_fq12_five_source_closure_evidence_v1_digest,
    path2_reviewed_fq12_native_boundary_predicate_model_evidence_all_facts_true,
    path2_reviewed_fq12_native_boundary_predicate_model_evidence_report_verified_v1,
    path2_reviewed_fq12_native_boundary_predicate_model_evidence_v1,
    path2_reviewed_fq12_native_boundary_predicate_model_evidence_v1_digest,
    path2_reviewed_fq12_role_root_lift_evidence_all_facts_true,
    path2_reviewed_fq12_role_root_lift_evidence_v1,
    path2_reviewed_fq12_role_root_lift_evidence_v1_digest,
    path2_reviewed_fq12_selected_source_route_byte_source_evidence_all_facts_true,
    path2_reviewed_fq12_selected_source_route_byte_source_evidence_v1,
    path2_reviewed_fq12_selected_source_route_byte_source_evidence_v1_digest,
    path2_reviewed_fq12_shared_digest_frame_witness_evidence_all_facts_true,
    path2_reviewed_fq12_shared_digest_frame_witness_evidence_v1,
    path2_reviewed_fq12_shared_digest_frame_witness_evidence_v1_digest,
    path2_selected_rust_model_concrete_definition_checklist_v1,
    path2_selected_rust_model_source_record_subcheck_v1,
    verify_path2_selected_rust_model_concrete_definition_checklist_v1,
};
use serde_json::json;

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn main() -> Result<(), String> {
    let five_source = path2_reviewed_fq12_five_source_closure_evidence_v1()?;
    let boundary_base = path2_reviewed_fq12_boundary_base_evidence_v1()?;
    let role_root_lift = path2_reviewed_fq12_role_root_lift_evidence_v1()?;
    let source_route = path2_reviewed_fq12_selected_source_route_byte_source_evidence_v1()?;
    let field_local = path2_reviewed_fq12_field_local_evidence_v1()?;
    let shared_witness = path2_reviewed_fq12_shared_digest_frame_witness_evidence_v1()?;
    let predicate_model = path2_reviewed_fq12_native_boundary_predicate_model_evidence_v1()?;
    let predicate_model_report_digest =
        path2_reviewed_fq12_native_boundary_predicate_model_evidence_report_verified_v1(
            &predicate_model,
        )?;
    let concrete_definition_checklist =
        path2_selected_rust_model_concrete_definition_checklist_v1();
    let source_record_subcheck = path2_selected_rust_model_source_record_subcheck_v1();
    let concrete_definition_checklist_digest =
        verify_path2_selected_rust_model_concrete_definition_checklist_v1(
            &concrete_definition_checklist,
        )?;
    let mut missing_definition_categories = Vec::new();
    if !concrete_definition_checklist.evidence_proof_definitions_present {
        missing_definition_categories.push("evidence_proof_definitions");
    }
    if !concrete_definition_checklist.predicate_model_certificate_definitions_present {
        missing_definition_categories.push("predicate_model_certificate_definitions");
    }
    if !concrete_definition_checklist.source_record_definitions_present {
        missing_definition_categories.push("source_record_definitions");
    }
    if !concrete_definition_checklist.source_record_evidence_source_bindings_present {
        missing_definition_categories.push("source_record_evidence_source_bindings");
    }
    if !concrete_definition_checklist.sub_evidence_digest_match_proof_present {
        missing_definition_categories.push("sub_evidence_digest_match_proof");
    }

    let payload = json!({
        "schema": "Descender.Path2.reviewed_fq12.selected_evidence_digest_manifest.v1",
        "digests": {
            "selectedRustFiveSourceEvidenceDigest": hex_encode(
                &path2_reviewed_fq12_five_source_closure_evidence_v1_digest(&five_source)?
            ),
            "selectedRustBoundaryBaseEvidenceDigest": hex_encode(
                &path2_reviewed_fq12_boundary_base_evidence_v1_digest(&boundary_base)?
            ),
            "selectedRustRoleRootLiftEvidenceDigest": hex_encode(
                &path2_reviewed_fq12_role_root_lift_evidence_v1_digest(&role_root_lift)?
            ),
            "selectedRustSourceRouteEvidenceDigest": hex_encode(
                &path2_reviewed_fq12_selected_source_route_byte_source_evidence_v1_digest(
                    &source_route
                )?
            ),
            "selectedRustFieldLocalEvidenceDigest": hex_encode(
                &path2_reviewed_fq12_field_local_evidence_v1_digest(&field_local)?
            ),
            "selectedRustSharedFrameWitnessEvidenceDigest": hex_encode(
                &path2_reviewed_fq12_shared_digest_frame_witness_evidence_v1_digest(
                    &shared_witness
                )?
            ),
            "selectedRustNativeBoundaryPredicateModelEvidenceDigest": hex_encode(
                &path2_reviewed_fq12_native_boundary_predicate_model_evidence_v1_digest(
                    &predicate_model
                )?
            ),
        },
        "all_facts": {
            "five_source": path2_reviewed_fq12_five_source_closure_evidence_all_facts_true(
                &five_source
            ),
            "boundary_base": path2_reviewed_fq12_boundary_base_evidence_all_facts_true(
                &boundary_base
            ),
            "role_root_lift": path2_reviewed_fq12_role_root_lift_evidence_all_facts_true(
                &role_root_lift
            ),
            "source_route":
                path2_reviewed_fq12_selected_source_route_byte_source_evidence_all_facts_true(
                    &source_route
                ),
            "field_local": path2_reviewed_fq12_field_local_evidence_all_facts_true(
                &field_local
            ),
            "shared_witness":
                path2_reviewed_fq12_shared_digest_frame_witness_evidence_all_facts_true(
                    &shared_witness
                ),
            "predicate_model":
                path2_reviewed_fq12_native_boundary_predicate_model_evidence_all_facts_true(
                    &predicate_model
                ),
        },
        "predicate_model_evidence": {
            "report_digest": hex_encode(&predicate_model_report_digest),
            "report_verified": true,
            "active_boundary_predicate_model_pinned":
                predicate_model.active_boundary_predicate_model_pinned,
            "payload_acceptance_model_pinned":
                predicate_model.payload_acceptance_model_pinned,
            "extraction_certificate_model_pinned":
                predicate_model.extraction_certificate_model_pinned,
            "full_opening_and_assembly_model_pinned":
                predicate_model.full_opening_and_assembly_model_pinned,
            "compatibility_and_extraction_model_pinned":
                predicate_model.compatibility_and_extraction_model_pinned,
            "native_boundary_break_model_pinned":
                predicate_model.native_boundary_break_model_pinned,
            "native_boundary_predicate_record_complete":
                predicate_model.native_boundary_predicate_record_complete,
            "active_verifier_route_pinned":
                predicate_model.active_verifier_route_pinned,
            "concrete_predicate_model_present":
                predicate_model.concrete_predicate_model_present,
            "certified_semantics_source_package_target_present":
                predicate_model.certified_semantics_source_package_target_present,
            "predicate_negative_controls_passed":
                predicate_model.predicate_negative_controls_passed,
        },
        "closing_status": {
            "extractor_theorem_instantiated": five_source.extractor_theorem_instantiated,
            "semantics_bridge_complete": five_source.semantics_bridge_complete,
        },
        "concrete_definition_checklist": {
            "digest": hex_encode(&concrete_definition_checklist_digest),
            "verified": true,
            "evidence_report_definitions_present":
                concrete_definition_checklist.evidence_report_definitions_present,
            "evidence_digest_definitions_present":
                concrete_definition_checklist.evidence_digest_definitions_present,
            "evidence_proof_definitions_present":
                concrete_definition_checklist.evidence_proof_definitions_present,
            "predicate_model_report_surface_definitions_present":
                concrete_definition_checklist
                    .predicate_model_report_surface_definitions_present,
            "predicate_model_facts_proof_definition_present":
                concrete_definition_checklist
                    .predicate_model_facts_proof_definition_present,
            "predicate_model_certificate_definition_present":
                concrete_definition_checklist.predicate_model_certificate_definition_present,
            "predicate_model_certificate_definitions_present":
                concrete_definition_checklist.predicate_model_certificate_definitions_present,
            "source_record_definitions_present":
                concrete_definition_checklist.source_record_definitions_present,
            "source_record_evidence_source_bindings_present":
                concrete_definition_checklist.source_record_evidence_source_bindings_present,
            "sub_evidence_digest_match_proof_present":
                concrete_definition_checklist.sub_evidence_digest_match_proof_present,
            "all_definitions_present":
                concrete_definition_checklist.all_definitions_present,
            "missing_definition_category_count":
                concrete_definition_checklist.missing_definition_category_count,
            "missing_definition_categories": missing_definition_categories,
        },
        "source_record_subcheck": {
            "native_boundary_predicates_record_present":
                source_record_subcheck.native_boundary_predicates_record_present,
            "native_boundary_predicate_model_source_target_present":
                source_record_subcheck.native_boundary_predicate_model_source_target_present,
            "native_boundary_predicate_model_evidence_report_target_present":
                source_record_subcheck
                    .native_boundary_predicate_model_evidence_report_target_present,
            "native_boundary_predicate_model_certificate_target_present":
                source_record_subcheck.native_boundary_predicate_model_certificate_target_present,
            "native_boundary_predicate_model_certificate_to_source_present":
                source_record_subcheck
                    .native_boundary_predicate_model_certificate_to_source_present,
            "native_boundary_predicate_model_current_report_certificate_impossible_present":
                source_record_subcheck
                    .native_boundary_predicate_model_current_report_certificate_impossible_present,
            "native_boundary_predicate_model_generic_evidence_labels_present":
                source_record_subcheck
                    .native_boundary_predicate_model_generic_evidence_labels_present,
            "native_boundary_predicate_model_remaining_generic_evidence_label_count":
                source_record_subcheck
                    .native_boundary_predicate_model_remaining_generic_evidence_label_count,
            "native_boundary_payload_acceptance_antivacuity_target_present":
                source_record_subcheck
                    .native_boundary_payload_acceptance_antivacuity_target_present,
            "native_boundary_break_antivacuity_target_present":
                source_record_subcheck.native_boundary_break_antivacuity_target_present,
            "native_boundary_payload_acceptance_boundary_match_target_present":
                source_record_subcheck
                    .native_boundary_payload_acceptance_boundary_match_target_present,
            "native_boundary_extraction_certificate_payload_match_target_present":
                source_record_subcheck
                    .native_boundary_extraction_certificate_payload_match_target_present,
            "native_boundary_extraction_certificate_validity_match_target_present":
                source_record_subcheck
                    .native_boundary_extraction_certificate_validity_match_target_present,
            "native_boundary_certificate_extraction_payload_consistency_target_present":
                source_record_subcheck
                    .native_boundary_certificate_extraction_payload_consistency_target_present,
            "native_boundary_payload_extraction_fiber_target_present":
                source_record_subcheck.native_boundary_payload_extraction_fiber_target_present,
            "native_boundary_payload_extraction_native_valid_target_present":
                source_record_subcheck
                    .native_boundary_payload_extraction_native_valid_target_present,
            "native_boundary_role_lift_validity_binds_extraction_target_present":
                source_record_subcheck
                    .native_boundary_role_lift_validity_binds_extraction_target_present,
            "native_boundary_role_lift_opening_certificate_lift_target_present":
                source_record_subcheck
                    .native_boundary_role_lift_opening_certificate_lift_target_present,
            "native_boundary_assembly_validity_binds_opening_target_present":
                source_record_subcheck
                    .native_boundary_assembly_validity_binds_opening_target_present,
            "native_boundary_assembly_certificate_full_native_target_present":
                source_record_subcheck
                    .native_boundary_assembly_certificate_full_native_target_present,
            "native_boundary_compatibility_validity_binds_target_present":
                source_record_subcheck
                    .native_boundary_compatibility_validity_binds_target_present,
            "native_boundary_compatibility_extraction_reviewed_target_present":
                source_record_subcheck
                    .native_boundary_compatibility_extraction_reviewed_target_present,
            "native_boundary_full_opening_bridge_digest_binds_target_present":
                source_record_subcheck
                    .native_boundary_full_opening_bridge_digest_binds_target_present,
            "native_boundary_semantics_package_target_present":
                source_record_subcheck.native_boundary_semantics_package_target_present,
            "native_boundary_semantics_source_package_target_present":
                source_record_subcheck.native_boundary_semantics_source_package_target_present,
            "native_boundary_certified_semantics_source_package_target_present":
                source_record_subcheck
                    .native_boundary_certified_semantics_source_package_target_present,
            "native_boundary_certified_semantics_current_report_impossible_present":
                source_record_subcheck
                    .native_boundary_certified_semantics_current_report_impossible_present,
            "native_boundary_semantics_package_handoff_present":
                source_record_subcheck.native_boundary_semantics_package_handoff_present,
            "native_boundary_semantics_source_package_handoff_present":
                source_record_subcheck.native_boundary_semantics_source_package_handoff_present,
            "native_boundary_certified_semantics_source_package_handoff_present":
                source_record_subcheck
                    .native_boundary_certified_semantics_source_package_handoff_present,
            "native_boundary_semantics_package_bound_handoff_present":
                source_record_subcheck.native_boundary_semantics_package_bound_handoff_present,
            "native_boundary_semantics_source_package_bound_handoff_present":
                source_record_subcheck
                    .native_boundary_semantics_source_package_bound_handoff_present,
            "native_boundary_certified_semantics_source_package_bound_handoff_present":
                source_record_subcheck
                    .native_boundary_certified_semantics_source_package_bound_handoff_present,
            "full_spend_concrete_predicates_are_separate_model":
                source_record_subcheck.full_spend_concrete_predicates_are_separate_model,
            "native_boundary_predicate_model_forbidden_vacuous_literals_present":
                source_record_subcheck
                    .native_boundary_predicate_model_forbidden_vacuous_literals_present,
            "native_boundary_predicate_model_field_sources_defined_present":
                source_record_subcheck
                    .native_boundary_predicate_model_field_sources_defined_present,
            "native_boundary_predicate_model_forbidden_vacuous_field_sources_present":
                source_record_subcheck
                    .native_boundary_predicate_model_forbidden_vacuous_field_sources_present,
            "native_boundary_predicate_model_stub_field_sources_present":
                source_record_subcheck
                    .native_boundary_predicate_model_stub_field_sources_present,
            "native_boundary_predicate_model_incomplete_proof_terms_present":
                source_record_subcheck
                    .native_boundary_predicate_model_incomplete_proof_terms_present,
            "native_boundary_concrete_predicate_model_present":
                source_record_subcheck.native_boundary_concrete_predicate_model_present,
            "selected_source_records_blocked_on_native_boundary_predicate_model":
                source_record_subcheck
                    .selected_source_records_blocked_on_native_boundary_predicate_model,
            "source_record_package_surface_present":
                source_record_subcheck.source_record_package_surface_present,
            "package_sub_evidence_match_reduction_present":
                source_record_subcheck.package_sub_evidence_match_reduction_present,
            "package_to_closure_inputs_reduction_present":
                source_record_subcheck.package_to_closure_inputs_reduction_present,
            "package_to_byte_component_inputs_reduction_present":
                source_record_subcheck.package_to_byte_component_inputs_reduction_present,
            "package_to_field_local_shared_digest_frame_inputs_reduction_present":
                source_record_subcheck
                    .package_to_field_local_shared_digest_frame_inputs_reduction_present,
            "package_to_hidden_source_closure_reduction_present":
                source_record_subcheck.package_to_hidden_source_closure_reduction_present,
            "boundary_base_report_only_not_source_record_closure":
                source_record_subcheck.boundary_base_report_only_not_source_record_closure,
            "role_root_lift_report_only_not_source_record_closure":
                source_record_subcheck.role_root_lift_report_only_not_source_record_closure,
            "source_route_report_only_not_source_record_closure":
                source_record_subcheck.source_route_report_only_not_source_record_closure,
            "field_local_report_only_not_source_record_closure":
                source_record_subcheck.field_local_report_only_not_source_record_closure,
            "shared_witness_report_only_not_source_record_closure":
                source_record_subcheck.shared_witness_report_only_not_source_record_closure,
            "payload_projection_law_definition_present":
                source_record_subcheck.payload_projection_law_definition_present,
            "extraction_certificate_availability_definition_present":
                source_record_subcheck
                    .extraction_certificate_availability_definition_present,
            "role_root_lift_source_definition_present":
                source_record_subcheck.role_root_lift_source_definition_present,
            "source_route_byte_source_definition_present":
                source_record_subcheck.source_route_byte_source_definition_present,
            "assembly_source_definition_present":
                source_record_subcheck.assembly_source_definition_present,
            "compatibility_source_definition_present":
                source_record_subcheck.compatibility_source_definition_present,
            "shared_frame_witness_sources_definition_present":
                source_record_subcheck.shared_frame_witness_sources_definition_present,
            "boundary_base_certificate_constructor_bound":
                source_record_subcheck.boundary_base_certificate_constructor_bound,
            "role_root_lift_certificate_constructor_bound":
                source_record_subcheck.role_root_lift_certificate_constructor_bound,
            "source_route_certificate_constructor_bound":
                source_record_subcheck.source_route_certificate_constructor_bound,
            "field_local_certificate_constructor_bound":
                source_record_subcheck.field_local_certificate_constructor_bound,
            "shared_witness_certificate_constructor_bound":
                source_record_subcheck.shared_witness_certificate_constructor_bound,
            "selected_source_record_package_definition_present":
                source_record_subcheck.selected_source_record_package_definition_present,
            "canonical_witness_to_byte_source_adapter_present":
                source_record_subcheck.canonical_witness_to_byte_source_adapter_present,
            "byte_and_witness_packaged_route_endpoint_present":
                source_record_subcheck.byte_and_witness_packaged_route_endpoint_present,
            "missing_selected_source_record_count":
                source_record_subcheck.missing_selected_source_record_count,
            "all_selected_source_records_present":
                source_record_subcheck.all_selected_source_records_present,
        },
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&payload)
            .map_err(|err| format!("failed to serialize Path 2 digest manifest: {err}"))?
    );
    Ok(())
}
