use descender_parameter_search::production_reference::{
    path2_account_stark_payload_byte_model_report_v1,
    path2_account_stark_payload_byte_model_report_v1_digest,
    validate_path2_account_stark_payload_byte_model_report_v1,
};
use serde_json::{Map, Value};
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

fn array_at<'a>(value: &'a Value, pointer: &str) -> Result<&'a [Value], String> {
    at(value, pointer)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| format!("JSON pointer {pointer} is not an array"))
}

fn object_at<'a>(value: &'a Value, pointer: &str) -> Result<&'a Map<String, Value>, String> {
    at(value, pointer)?
        .as_object()
        .ok_or_else(|| format!("JSON pointer {pointer} is not an object"))
}

fn optional_u64(value: &Value, pointer: &str) -> Result<Option<u64>, String> {
    match value.pointer(pointer) {
        Some(value) => value
            .as_u64()
            .map(Some)
            .ok_or_else(|| format!("JSON pointer {pointer} is not an unsigned integer")),
        None => Ok(None),
    }
}

fn optional_str<'a>(value: &'a Value, pointer: &str) -> Result<Option<&'a str>, String> {
    match value.pointer(pointer) {
        Some(value) => value
            .as_str()
            .map(Some)
            .ok_or_else(|| format!("JSON pointer {pointer} is not a string")),
        None => Ok(None),
    }
}

fn require_keys(
    object: &Map<String, Value>,
    allowed: &BTreeSet<&'static str>,
    label: &str,
) -> Result<(), String> {
    for key in object.keys() {
        if !allowed.contains(key.as_str()) {
            return Err(format!("{label} exposes unexpected key {key:?}"));
        }
    }
    for key in allowed {
        if !object.contains_key(*key) {
            return Err(format!("{label} is missing required key {key:?}"));
        }
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let byte_model = load_json("formal/kat/path2_account_stark_payload_byte_model_v1.json")?;
    let vectors = load_json("formal/kat/path2_account_stark_payload_kat_vectors_v1.json")?;

    let report = path2_account_stark_payload_byte_model_report_v1();
    validate_path2_account_stark_payload_byte_model_report_v1(&report)?;
    let report_digest = hex_encode(&report.digest);
    let recomputed_digest = hex_encode(&path2_account_stark_payload_byte_model_report_v1_digest(
        &report,
    ));

    if str_at(&byte_model, "/manifest/schema")?
        != "Descender.Path2.AccountStarkPayloadByteModelManifest.v1"
    {
        return Err("payload byte model manifest schema mismatch".to_string());
    }
    if str_at(&byte_model, "/manifest/frozen_status")? != "draft_byte_model_manifest" {
        return Err("payload byte model manifest frozen_status drifted".to_string());
    }
    if str_at(&vectors, "/corpus/schema")? != "Descender.Path2.AccountStarkPayloadKatVectors.v1" {
        return Err("payload KAT vector corpus schema mismatch".to_string());
    }
    if str_at(&vectors, "/corpus/status")? != "digest_pinned_boundary_vectors" {
        return Err("payload KAT vector corpus status drifted".to_string());
    }

    if str_at(
        &byte_model,
        "/manifest/digests/payload_byte_model_report_digest",
    )? != report_digest
    {
        return Err("payload byte model manifest digest is not bound to report".to_string());
    }
    if str_at(
        &byte_model,
        "/manifest/digests/payload_byte_model_report_recomputed_digest",
    )? != recomputed_digest
    {
        return Err("payload byte model manifest recomputed digest mismatch".to_string());
    }
    if str_at(&byte_model, "/manifest/authority_snapshot/selected_payload_type")?
        != str_at(&vectors, "/corpus/authority/selected_payload_type")?
    {
        return Err("payload byte model selected payload type drifted from vectors".to_string());
    }
    if str_at(&byte_model, "/manifest/authority_snapshot/payload_frame_function")?
        != str_at(&vectors, "/corpus/authority/payload_frame_function")?
    {
        return Err("payload byte model frame function drifted from vectors".to_string());
    }
    if str_at(
        &byte_model,
        "/manifest/authority_snapshot/payload_digest_function",
    )? != str_at(&vectors, "/corpus/authority/payload_digest_function")?
    {
        return Err("payload byte model digest function drifted from vectors".to_string());
    }

    let expected_field_order = [
        "schema",
        "payload_version",
        "account_stark_verifier_version",
        "leaf_context_digest",
        "creation_proof_binding_digest",
        "native_role_root_commitment",
        "aggregation_context_digest",
        "slot_id",
        "masked_projection_lane_count",
        "masked_projection_le",
        "account_stark_bundle",
    ];
    let fields = array_at(&byte_model, "/manifest/byte_model/field_order")?;
    if fields.len() != expected_field_order.len() {
        return Err(format!(
            "payload byte model field_count mismatch: got {}, expected {}",
            fields.len(),
            expected_field_order.len()
        ));
    }
    if u64_at(&byte_model, "/manifest/byte_model/field_count")? != fields.len() as u64 {
        return Err("payload byte model field_count does not match field_order length".to_string());
    }
    for (index, expected_name) in expected_field_order.iter().enumerate() {
        let field_name = str_at(
            &byte_model,
            &format!("/manifest/byte_model/field_order/{index}/field_name"),
        )?;
        if field_name != *expected_name {
            return Err(format!(
                "payload byte model field {index} drifted: got {field_name}, expected {expected_name}"
            ));
        }
    }
    let lane_count = u64_at(
        &byte_model,
        "/manifest/byte_model/masked_projection_lane_count",
    )?;
    if lane_count != 6 {
        return Err(format!(
            "payload byte model lane count drifted: got {lane_count}, expected 6"
        ));
    }
    if u64_at(&byte_model, "/manifest/byte_model/masked_projection_bytes")? != 12 {
        return Err("payload byte model masked_projection byte width drifted".to_string());
    }
    let fixed_overhead = u64_at(
        &byte_model,
        "/manifest/byte_model/fixed_frame_overhead_excluding_bundle",
    )?;
    if fixed_overhead != 558 {
        return Err(format!(
            "payload byte model fixed frame overhead drifted: got {fixed_overhead}, expected 558"
        ));
    }
    if u64_at(
        &byte_model,
        "/manifest/byte_model/fixed_data_bytes_excluding_bundle",
    )? != 221
    {
        return Err("payload byte model fixed data byte count drifted".to_string());
    }
    if !bool_at(&byte_model, "/manifest/byte_model/bundle_nonempty_required")?
        || !bool_at(&byte_model, "/manifest/byte_model/bundle_variable_length")?
        || !bool_at(
            &byte_model,
            "/manifest/byte_model/deterministic_kat_digest_pins_bundle_bytes",
        )?
    {
        return Err("payload byte model no longer pins bundle policy".to_string());
    }
    if bool_at(
        &byte_model,
        "/manifest/byte_model/full_proof_bytes_inlined_in_manifest",
    )? || bool_at(
        &byte_model,
        "/manifest/byte_model/public_payload_exports_private_material",
    )? || bool_at(&byte_model, "/manifest/byte_model/deployment_ready")?
    {
        return Err("payload byte model unexpectedly claims deployment surface readiness".to_string());
    }

    let payload_allowed = BTreeSet::from([
        "account_stark_bundle_len",
        "aggregation_context_digest",
        "creation_proof_binding_digest",
        "leaf_context_digest",
        "masked_projection",
        "native_role_root_commitment",
        "slot_id",
    ]);
    let payload_bytes_allowed = BTreeSet::from([
        "account_stark_bundle_len",
        "account_stark_bundle_sha256",
        "payload_digest",
        "payload_frame_len",
        "payload_frame_sha256",
        "proof_bytes_policy",
        "payload_digest_error",
    ]);

    for case in array_at(&vectors, "/corpus/cases")? {
        let case_id = str_at(case, "/case_id")?;
        let payload = object_at(case, "/payload")?;
        let payload_bytes = object_at(case, "/payload_bytes")?;
        require_keys(payload, &payload_allowed, &format!("payload case {case_id}"))?;
        for key in payload_bytes.keys() {
            if !payload_bytes_allowed.contains(key.as_str()) {
                return Err(format!(
                    "payload byte evidence case {case_id} exposes unexpected key {key:?}"
                ));
            }
        }

        let payload_bundle_len = u64_at(case, "/payload/account_stark_bundle_len")?;
        let byte_bundle_len = u64_at(case, "/payload_bytes/account_stark_bundle_len")?;
        if payload_bundle_len != byte_bundle_len {
            return Err(format!(
                "payload case {case_id} bundle length mismatch: payload={payload_bundle_len}, bytes={byte_bundle_len}"
            ));
        }
        let masked_projection = array_at(case, "/payload/masked_projection")?;
        if masked_projection.len() as u64 != lane_count {
            return Err(format!(
                "payload case {case_id} masked_projection lane count {} != {lane_count}",
                masked_projection.len()
            ));
        }

        match optional_u64(case, "/payload_bytes/payload_frame_len")? {
            Some(frame_len) => {
                if payload_bundle_len == 0 {
                    return Err(format!(
                        "payload case {case_id} has a frame length despite empty bundle"
                    ));
                }
                if frame_len != payload_bundle_len + fixed_overhead {
                    return Err(format!(
                        "payload case {case_id} frame length {frame_len} != bundle {payload_bundle_len} + overhead {fixed_overhead}"
                    ));
                }
                let payload_digest = str_at(case, "/payload_bytes/payload_digest")?;
                let frame_digest = str_at(case, "/payload_bytes/payload_frame_sha256")?;
                if payload_digest != frame_digest {
                    return Err(format!(
                        "payload case {case_id} digest {payload_digest} != frame sha256 {frame_digest}"
                    ));
                }
                let proof_policy = str_at(case, "/payload_bytes/proof_bytes_policy")?;
                if !proof_policy.contains("digest-pinned")
                    || !proof_policy.contains("not inlined")
                {
                    return Err(format!(
                        "payload case {case_id} proof bytes policy is not digest-pinned/non-inlined"
                    ));
                }
            }
            None => {
                let digest_error = optional_str(case, "/payload_bytes/payload_digest_error")?
                    .ok_or_else(|| {
                        format!(
                            "payload case {case_id} has no frame length and no payload_digest_error"
                        )
                    })?;
                if payload_bundle_len != 0
                    || !digest_error.contains("account-creation STARK: payload bundle is empty")
                {
                    return Err(format!(
                        "payload case {case_id} missing frame for reason other than empty bundle: {digest_error:?}"
                    ));
                }
                if case.pointer("/payload_bytes/payload_digest").is_some()
                    || case.pointer("/payload_bytes/payload_frame_sha256").is_some()
                {
                    return Err(format!(
                        "payload case {case_id} has digest bytes despite empty-bundle error"
                    ));
                }
            }
        }
    }

    println!("Path 2 account-STARK payload byte model consistency check passed.");
    Ok(())
}
