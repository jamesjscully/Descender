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
    let concrete_definition_checklist =
        path2_selected_rust_model_concrete_definition_checklist_v1();
    let concrete_definition_checklist_digest =
        verify_path2_selected_rust_model_concrete_definition_checklist_v1(
            &concrete_definition_checklist,
        )?;
    let mut missing_definition_categories = Vec::new();
    if !concrete_definition_checklist.evidence_proof_definitions_present {
        missing_definition_categories.push("evidence_proof_definitions");
    }
    if !concrete_definition_checklist.source_record_definitions_present {
        missing_definition_categories.push("source_record_definitions");
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
            "source_record_definitions_present":
                concrete_definition_checklist.source_record_definitions_present,
            "sub_evidence_digest_match_proof_present":
                concrete_definition_checklist.sub_evidence_digest_match_proof_present,
            "all_definitions_present":
                concrete_definition_checklist.all_definitions_present,
            "missing_definition_category_count":
                concrete_definition_checklist.missing_definition_category_count,
            "missing_definition_categories": missing_definition_categories,
        },
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&payload)
            .map_err(|err| format!("failed to serialize Path 2 digest manifest: {err}"))?
    );
    Ok(())
}
