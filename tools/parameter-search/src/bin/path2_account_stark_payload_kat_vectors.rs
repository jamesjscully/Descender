#[cfg(not(feature = "account-creation-stark-verifier"))]
fn main() -> Result<(), String> {
    Err(
        "path2_account_stark_payload_kat_vectors requires --features account-creation-stark-verifier"
            .to_string(),
    )
}

#[cfg(feature = "account-creation-stark-verifier")]
mod enabled {
    use descender_parameter_search::production_reference::{
        DIGEST_BYTES, PROFILE, Path2AccountCreationEvidenceV1,
        Path2AccountStarkCreationPayloadV1, Path2AccountStarkPayloadKatFixtureV1,
        Path2CreationProofBindingV1, Path2DirectReplayableRelationCertificateV1,
        Path2PrevalidatedSlotLeafV1, path2_account_creation_evidence_transcript_digest,
        path2_account_stark_creation_payload_v1_digest,
        path2_account_stark_creation_payload_v1_frame,
        path2_account_stark_payload_kat_boundary_report_v1,
        path2_account_stark_payload_kat_fixture_v1,
        path2_account_stark_payload_public_rejection_class,
        path2_direct_replayable_relation_certificate_digest, path2_prevalidated_slot_leaf_digest,
        path2_slot_authorization_digest,
        verify_path2_direct_creation_account_stark_payload_boundary,
        verify_path2_direct_creation_account_stark_payload_boundary_for_production,
    };
    use serde_json::{Value, json};
    use sha2::{Digest, Sha256};

    #[derive(Clone, Copy)]
    struct Context {
        leaf: Path2PrevalidatedSlotLeafV1,
        evidence: Path2AccountCreationEvidenceV1,
        certificate: Path2DirectReplayableRelationCertificateV1,
    }

    fn hex_encode(bytes: &[u8]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut out = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            out.push(HEX[(byte >> 4) as usize] as char);
            out.push(HEX[(byte & 0x0f) as usize] as char);
        }
        out
    }

    fn digest_hex(bytes: &[u8]) -> String {
        hex_encode(&Sha256::digest(bytes))
    }

    fn digest_field_hex(digest: &[u8; DIGEST_BYTES]) -> String {
        hex_encode(digest)
    }

    fn refresh_context(
        leaf: &mut Path2PrevalidatedSlotLeafV1,
        evidence: &mut Path2AccountCreationEvidenceV1,
    ) -> Result<(), String> {
        evidence.slot_leaf_digest = path2_prevalidated_slot_leaf_digest(leaf)?;
        evidence.slot_authorization_digest = path2_slot_authorization_digest(leaf)?;
        Ok(())
    }

    fn rebind_context_to_payload(
        base: &Context,
        payload: &Path2AccountStarkCreationPayloadV1,
    ) -> Result<Context, String> {
        let mut certificate = base.certificate;
        certificate.public_replay_payload_digest =
            path2_account_stark_creation_payload_v1_digest(payload)?;
        let mut evidence = base.evidence;
        evidence.evidence_payload_digest =
            path2_direct_replayable_relation_certificate_digest(&certificate);
        let mut leaf = base.leaf;
        leaf.prevalidation_transcript_digest =
            path2_account_creation_evidence_transcript_digest(&evidence)?;
        refresh_context(&mut leaf, &mut evidence)?;
        Ok(Context {
            leaf,
            evidence,
            certificate,
        })
    }

    fn context_with_wrong_replay_digest(base: &Context) -> Result<Context, String> {
        let mut certificate = base.certificate;
        certificate.public_replay_payload_digest[0] ^= 1;
        let mut evidence = base.evidence;
        evidence.evidence_payload_digest =
            path2_direct_replayable_relation_certificate_digest(&certificate);
        let mut leaf = base.leaf;
        leaf.prevalidation_transcript_digest =
            path2_account_creation_evidence_transcript_digest(&evidence)?;
        refresh_context(&mut leaf, &mut evidence)?;
        Ok(Context {
            leaf,
            evidence,
            certificate,
        })
    }

    fn payload_bytes_json(payload: &Path2AccountStarkCreationPayloadV1) -> Value {
        match path2_account_stark_creation_payload_v1_frame(payload) {
            Ok(frame) => json!({
                "payload_frame_len": frame.len(),
                "account_stark_bundle_len": payload.account_stark_bundle.len(),
                "proof_bytes_policy":
                    "not pinned: Plonky3 ZK proof bytes are randomized until a deterministic KAT prover mode exists"
            }),
            Err(error) => json!({
                "payload_digest_error": error,
                "account_stark_bundle_len": payload.account_stark_bundle.len()
            }),
        }
    }

    fn payload_fields_json(payload: &Path2AccountStarkCreationPayloadV1) -> Value {
        json!({
            "leaf_context_digest": digest_field_hex(&payload.leaf_context_digest),
            "creation_proof_binding_digest": digest_field_hex(
                &payload.creation_proof_binding_digest
            ),
            "native_role_root_commitment": digest_field_hex(&payload.native_role_root_commitment),
            "aggregation_context_digest": digest_field_hex(&payload.aggregation_context_digest),
            "slot_id": payload.slot_id,
            "masked_projection": payload.masked_projection,
            "account_stark_bundle_len": payload.account_stark_bundle.len()
        })
    }

    fn context_json(context: &Context, binding: &Path2CreationProofBindingV1) -> Value {
        json!({
            "account_id": digest_field_hex(&context.leaf.account_id),
            "public_key_identity": digest_field_hex(&context.leaf.public_key_identity),
            "slot_id": context.leaf.slot_id,
            "account_state_root_before": digest_field_hex(&context.leaf.account_state_root_before),
            "opening_commitment": digest_field_hex(&context.leaf.opening_commitment),
            "pad_seed_digest": digest_field_hex(&context.leaf.pad_seed_digest),
            "hidden_source_statement_digest": digest_field_hex(
                &context.leaf.hidden_source_statement_digest
            ),
            "lifecycle_anchor": digest_field_hex(&context.leaf.lifecycle_anchor),
            "binding_digest": digest_field_hex(&binding.digest),
            "proof_byte_digest_policy":
                "context replay-payload digest is intentionally not pinned because it includes randomized STARK proof bytes"
        })
    }

    fn rejection_case(
        case_id: &str,
        mutated_surface: &str,
        context_kind: &str,
        context: &Context,
        binding: &Path2CreationProofBindingV1,
        payload: &Path2AccountStarkCreationPayloadV1,
        mutation: Value,
    ) -> Result<Value, String> {
        let error = verify_path2_direct_creation_account_stark_payload_boundary(
            &context.leaf,
            &context.evidence,
            &context.certificate,
            binding,
            payload,
        )
        .expect_err("rejection KAT case accepted");
        Ok(json!({
            "case_id": case_id,
            "kind": "reject",
            "mutated_surface": mutated_surface,
            "mutation": mutation,
            "context_kind": context_kind,
            "observed_error": error,
            "public_rejection_class": path2_account_stark_payload_public_rejection_class(&error),
            "context": context_json(context, binding),
            "payload": payload_fields_json(payload),
            "payload_bytes": payload_bytes_json(payload)
        }))
    }

    fn production_gate_case(
        context: &Context,
        binding: &Path2CreationProofBindingV1,
        payload: &Path2AccountStarkCreationPayloadV1,
    ) -> Value {
        let error = verify_path2_direct_creation_account_stark_payload_boundary_for_production(
            &context.leaf,
            &context.evidence,
            &context.certificate,
            binding,
            payload,
        )
        .expect_err("production gate KAT case accepted");
        json!({
            "case_id": "production_gate_closed",
            "kind": "reject",
            "mutated_surface": "production_admission_gate",
            "context_kind": "honest_context",
            "observed_error": error,
            "public_rejection_class": path2_account_stark_payload_public_rejection_class(&error),
            "context": context_json(context, binding),
            "payload": payload_fields_json(payload),
            "payload_bytes": payload_bytes_json(payload)
        })
    }

    fn positive_case(
        context: &Context,
        binding: &Path2CreationProofBindingV1,
        payload: &Path2AccountStarkCreationPayloadV1,
    ) -> Result<Value, String> {
        verify_path2_direct_creation_account_stark_payload_boundary(
            &context.leaf,
            &context.evidence,
            &context.certificate,
            binding,
            payload,
        )?;
        Ok(json!({
            "case_id": "honest_payload_accepts",
            "kind": "accept",
            "context_kind": "honest_context",
            "context": context_json(context, binding),
            "payload": payload_fields_json(payload),
            "payload_bytes": payload_bytes_json(payload)
        }))
    }

    fn build_vectors() -> Result<Value, String> {
        let fixture = path2_account_stark_payload_kat_fixture_v1()?;
        let Path2AccountStarkPayloadKatFixtureV1 {
            leaf,
            evidence,
            certificate,
            binding,
            payload,
            payload_frame_bytes,
            account_stark_bundle_bytes,
            ..
        } = fixture;
        let base = Context {
            leaf,
            evidence,
            certificate,
        };
        let boundary_report = path2_account_stark_payload_kat_boundary_report_v1();
        let mut cases = vec![positive_case(&base, &binding, &payload)?];

        let mut bad = payload.clone();
        bad.leaf_context_digest[0] ^= 1;
        cases.push(rejection_case(
            "payload_leaf_context_digest_mutation",
            "leaf_context_digest",
            "honest_context",
            &base,
            &binding,
            &bad,
            json!({"op": "xor_u8", "field": "leaf_context_digest", "offset": 0, "xor": 1}),
        )?);

        let mut bad = payload.clone();
        bad.creation_proof_binding_digest[0] ^= 1;
        cases.push(rejection_case(
            "creation_proof_binding_digest_mutation",
            "creation_proof_binding_digest",
            "honest_context",
            &base,
            &binding,
            &bad,
            json!({"op": "xor_u8", "field": "creation_proof_binding_digest", "offset": 0, "xor": 1}),
        )?);

        let mut bad = payload.clone();
        bad.aggregation_context_digest[0] ^= 1;
        cases.push(rejection_case(
            "aggregation_context_digest_mutation",
            "aggregation_context_digest",
            "honest_context",
            &base,
            &binding,
            &bad,
            json!({"op": "xor_u8", "field": "aggregation_context_digest", "offset": 0, "xor": 1}),
        )?);

        let mut bad = payload.clone();
        bad.slot_id ^= 1;
        cases.push(rejection_case(
            "slot_id_mutation",
            "slot_id",
            "honest_context",
            &base,
            &binding,
            &bad,
            json!({"op": "xor_u64", "field": "slot_id", "xor": 1}),
        )?);

        let mut bad = payload.clone();
        bad.native_role_root_commitment[0] ^= 1;
        cases.push(rejection_case(
            "native_role_root_commitment_digest_mutation",
            "native_role_root_commitment",
            "honest_context",
            &base,
            &binding,
            &bad,
            json!({"op": "xor_u8", "field": "native_role_root_commitment", "offset": 0, "xor": 1}),
        )?);

        let mut bad = payload.clone();
        bad.masked_projection[0] ^= 1;
        cases.push(rejection_case(
            "masked_projection_digest_mutation",
            "masked_projection",
            "honest_context",
            &base,
            &binding,
            &bad,
            json!({"op": "xor_u16", "field": "masked_projection", "index": 0, "xor": 1}),
        )?);

        let mut bad = payload.clone();
        bad.account_stark_bundle[0] ^= 1;
        cases.push(rejection_case(
            "account_stark_bundle_digest_mutation",
            "account_stark_bundle",
            "honest_context",
            &base,
            &binding,
            &bad,
            json!({"op": "xor_u8", "field": "account_stark_bundle", "offset": 0, "xor": 1}),
        )?);

        let mut bad = payload.clone();
        bad.account_stark_bundle.clear();
        cases.push(rejection_case(
            "empty_account_stark_bundle",
            "account_stark_bundle",
            "honest_context",
            &base,
            &binding,
            &bad,
            json!({"op": "clear_vec", "field": "account_stark_bundle"}),
        )?);

        let mut bad = payload.clone();
        bad.native_role_root_commitment[0] ^= 1;
        let rebound = rebind_context_to_payload(&base, &bad)?;
        cases.push(rejection_case(
            "rebound_native_role_root_anchor",
            "native_role_root_commitment",
            "rebound_to_mutated_payload_digest",
            &rebound,
            &binding,
            &bad,
            json!({
                "op": "xor_u8_then_rebind_context_to_mutated_payload_digest",
                "field": "native_role_root_commitment",
                "offset": 0,
                "xor": 1
            }),
        )?);

        let mut bad = payload.clone();
        bad.masked_projection[0] ^= 1;
        let rebound = rebind_context_to_payload(&base, &bad)?;
        cases.push(rejection_case(
            "rebound_masked_projection_anchor",
            "masked_projection",
            "rebound_to_mutated_payload_digest",
            &rebound,
            &binding,
            &bad,
            json!({
                "op": "xor_u16_then_rebind_context_to_mutated_payload_digest",
                "field": "masked_projection",
                "index": 0,
                "xor": 1
            }),
        )?);

        let mut bad = payload.clone();
        bad.account_stark_bundle[0] ^= 1;
        let rebound = rebind_context_to_payload(&base, &bad)?;
        cases.push(rejection_case(
            "rebound_corrupted_account_stark_bundle",
            "account_stark_bundle",
            "rebound_to_mutated_payload_digest",
            &rebound,
            &binding,
            &bad,
            json!({
                "op": "xor_u8_then_rebind_context_to_mutated_payload_digest",
                "field": "account_stark_bundle",
                "offset": 0,
                "xor": 1
            }),
        )?);

        let wrong_replay = context_with_wrong_replay_digest(&base)?;
        cases.push(rejection_case(
            "wrong_replay_payload_digest",
            "public_replay_payload_digest",
            "wrong_replay_payload_digest",
            &wrong_replay,
            &binding,
            &payload,
            json!({
                "op": "xor_u8_on_context",
                "field": "certificate.public_replay_payload_digest",
                "offset": 0,
                "xor": 1
            }),
        )?);

        cases.push(production_gate_case(&base, &binding, &payload));

        let generated_rejection_ids: Vec<&str> = cases
            .iter()
            .filter_map(|case| {
                let case_id = case.get("case_id")?.as_str()?;
                (case_id != "honest_payload_accepts").then_some(case_id)
            })
            .collect();
        let report_ids: Vec<&str> = boundary_report.rows.iter().map(|row| row.case_id).collect();
        if generated_rejection_ids != report_ids {
            return Err(format!(
                "generated KAT case ids do not match boundary report: {:?} != {:?}",
                generated_rejection_ids, report_ids
            ));
        }
        for row in boundary_report.rows {
            let case = cases
                .iter()
                .find(|case| case["case_id"] == row.case_id)
                .ok_or_else(|| format!("missing generated KAT case {}", row.case_id))?;
            if case["public_rejection_class"] != row.public_rejection_class {
                return Err(format!(
                    "generated KAT public class mismatch for {}",
                    row.case_id
                ));
            }
            let observed = case["observed_error"]
                .as_str()
                .ok_or_else(|| format!("generated KAT case {} missing observed error", row.case_id))?;
            if !observed.contains(row.internal_error_match) {
                return Err(format!(
                    "generated KAT observed error mismatch for {}: {:?}",
                    row.case_id, observed
                ));
            }
        }

        let body = json!({
            "schema": "Descender.Path2.AccountStarkPayloadKatVectors.v1",
            "corpus_id": "Descender.Path2.AccountStarkPayloadKatBoundaryVectors.v1",
            "corpus_version": 1,
            "profile_id": PROFILE,
            "status": "digest_pinned_boundary_vectors",
            "proof_byte_policy": "the generator runs a real Plonky3 ZK proof, but randomized proof bytes and any digest that includes them are excluded from this stable corpus until a deterministic KAT prover mode exists",
            "generator": {
                "command": "cargo run --manifest-path tools/parameter-search/Cargo.toml --features account-creation-stark-verifier --bin path2_account_stark_payload_kat_vectors",
                "source": "tools/parameter-search/src/bin/path2_account_stark_payload_kat_vectors.rs"
            },
            "authority": {
                "fixture_id": "Path2AccountStarkPayloadKatFixture.v1",
                "selected_payload_type": "Path2AccountStarkCreationPayloadV1",
                "payload_digest_function": "path2_account_stark_creation_payload_v1_digest",
                "payload_frame_function": "path2_account_stark_creation_payload_v1_frame",
                "verifier_boundary_endpoint":
                    "verify_path2_direct_creation_account_stark_payload_boundary",
                "production_boundary_endpoint":
                    "verify_path2_direct_creation_account_stark_payload_boundary_for_production"
            },
            "boundary_report": {
                "domain": boundary_report.domain,
                "digest": digest_field_hex(&boundary_report.digest),
                "row_count": boundary_report.rows.len(),
                "every_current_row_has_generated_vector": true
            },
            "honest_payload": {
                "payload_frame_bytes": payload_frame_bytes,
                "account_stark_bundle_bytes": account_stark_bundle_bytes,
                "byte_policy": "full frame bytes are regenerated by the feature-gated generator; the checked artifact pins stable lengths and public fields while excluding randomized proof-byte digests"
            },
            "case_count": cases.len(),
            "rejection_case_count": cases.len() - 1,
            "cases": cases
        });
        let body_json = serde_json::to_string_pretty(&body)
            .map_err(|err| format!("failed to serialize vector body: {err}"))?;
        Ok(json!({
            "vector_corpus_digest": digest_hex(body_json.as_bytes()),
            "corpus": body
        }))
    }

    pub fn run() -> Result<(), String> {
        println!(
            "{}",
            serde_json::to_string_pretty(&build_vectors()?)
                .map_err(|err| format!("failed to serialize vector corpus: {err}"))?
        );
        Ok(())
    }
}

#[cfg(feature = "account-creation-stark-verifier")]
fn main() -> Result<(), String> {
    enabled::run()
}
