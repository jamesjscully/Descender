#!/usr/bin/env python3
"""Plan closure of global byte-frame disjointness ambiguity risks."""

from __future__ import annotations

import json
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
REGISTRY_JSON = (
    READINESS_DIR / "pre_signature_projection_global_domain_registry_n32_q12289.json"
)
MASK_FRAME_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_frame_disjointness_n32_q12289.json"
)
DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_global_frame_disjointness_plan_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "pre_signature_projection_global_frame_disjointness_plan_n32_q12289.md"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def by_id(entries: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    return {entry["id"]: entry for entry in entries}


def risk_root_cause(risk: dict[str, Any], entry: dict[str, Any] | None) -> str:
    risk_id = risk["id"]
    if risk_id == "pad_prf_secret_prefix_domain_offset":
        return "secret_prefixed_pad_prf"
    if risk_id.endswith("_raw_profile_concat"):
        return "raw_domain_profile_concat"
    if risk_id.endswith("_variable_length_frame"):
        if entry and entry["family"] == "preflight":
            return "variable_length_preflight_tlv"
        if entry and entry["frame_encoding"] == "raw_leading_domain_profile_len_frame":
            return "raw_profile_len_variable_tail"
        return "variable_length_selected_tlv"
    return "unclassified"


def action_class(root_cause: str, entry: dict[str, Any] | None) -> str:
    if root_cause == "secret_prefixed_pad_prf":
        return "proof_and_negative_tests"
    if root_cause == "raw_domain_profile_concat":
        return "byte_level_certificate_and_negative_tests"
    if root_cause == "raw_profile_len_variable_tail":
        return "proof_artifact_plus_length_bound_test"
    if root_cause == "variable_length_preflight_tlv":
        return "scope_decision_then_proof_or_tests"
    if entry and entry["min_input_bytes"] is None:
        return "code_probe_or_registry_extension_then_proof"
    return "proof_artifact_and_negative_tests"


def required_code_change(root_cause: str, entry: dict[str, Any] | None) -> str:
    if root_cause == "variable_length_preflight_tlv":
        return (
            "No selected-route code change if preflight stays outside the signer/verifier "
            "frame theorem; otherwise add executable frame builders or accepted-size caps "
            "before promoting the all-production registry."
        )
    if root_cause == "raw_domain_profile_concat":
        return (
            "No production redesign expected while constants remain fixed; add tests or a "
            "small certificate helper that records exact split points."
        )
    if root_cause == "secret_prefixed_pad_prf":
        return (
            "No production redesign expected if the global byte certificate proves the "
            "82-byte secret-prefixed frame cannot equal any registered public frame; add "
            "test builders if the current APIs do not expose canonical SHA-256 inputs."
        )
    if root_cause == "raw_profile_len_variable_tail":
        return (
            "Likely proof/test artifact only because the statement length is le32-delimited; "
            "add a bound check if the accepted statement frame length is not already capped "
            "at the verifier boundary."
        )
    if entry and entry["min_input_bytes"] is None:
        return (
            "Add or expose canonical byte-frame builders, then record exact minimum lengths "
            "and accepted variable-field bounds in the registry artifact."
        )
    return (
        "No production code change expected; close with TLV-injectivity proof and negative "
        "tests that mutate domains, labels, lengths, and field order."
    )


def group_action_class(root_cause: str, unknown_length_entries: list[str]) -> str:
    if root_cause == "variable_length_preflight_tlv":
        return "scope_decision_then_proof_or_tests"
    if unknown_length_entries:
        return "registry_extension_or_builder_probe_then_proof_tests"
    return action_class(root_cause, None)


def group_required_code_change(
    root_cause: str, unknown_length_entries: list[str], representative: dict[str, Any] | None
) -> str:
    if unknown_length_entries and root_cause != "variable_length_preflight_tlv":
        return (
            "Add or expose canonical byte-frame builders, then record exact minimum "
            "lengths and accepted variable-field bounds for: "
            f"{', '.join(unknown_length_entries)}."
        )
    return required_code_change(root_cause, representative)


def certificate_spec(root_cause: str, risks: list[dict[str, Any]]) -> list[str]:
    if root_cause == "secret_prefixed_pad_prf":
        return [
            "Record pad PRF length 82, domain offset 32, pad-frame length 46, block byte offset 77, and counter offset 78.",
            "For every registered frame, prove either length disjointness, leading-byte/domain disjointness, or TLV label-prefix disjointness.",
            "Enumerate bounded pad tuples for all 16 blocks and counters 0..15 against every fixed-length raw frame builder.",
            "Use adversarial seeds that start with each registered leading-domain prefix and TLV domain label bytes to show secret-prefix control does not create accepted frame equality.",
        ]
    if root_cause == "raw_domain_profile_concat":
        return [
            "Record domain length, profile length, fixed suffix length, and total input length for each raw domain/profile frame.",
            "Prove the full domain strings are unique and not split-compatible with any other raw domain/profile pair.",
            "Cross-check every other registered frame at the same total length for byte equality using canonical all-zero and all-0xff payload families.",
        ]
    if root_cause == "raw_profile_len_variable_tail":
        return [
            "Record the le32 statement_frame_len offset and the minimum 179-byte frame length.",
            "Prove the variable tail starts only after an explicit little-endian length field.",
            "Add malformed-length certificates for length-short, length-long, and trailing-byte variants.",
        ]
    if root_cause == "variable_length_preflight_tlv":
        return [
            "Decide whether preflight-only frames are in the selected signer/verifier theorem or only in an all-production registry.",
            "If included, record exact append_frame labels, variable-field bounds, and canonical order for each preflight frame.",
            "Prove TLV injectivity from u16(label_len), label bytes, u64(data_len), data bytes, and fixed field order.",
        ]
    return [
        "Record exact append_frame TLV label sequence, minimum length, and accepted bounds for every variable field.",
        "Prove no accepted frame can be parsed under another registered label sequence or domain.",
        "Emit pairwise certificate rows for all fixed-vs-variable and variable-vs-variable frame-family pairs.",
    ]


def negative_test_spec(root_cause: str, risks: list[dict[str, Any]]) -> list[str]:
    ids = [risk["registry_id"] for risk in risks if "registry_id" in risk]
    joined = ", ".join(ids)
    if root_cause == "secret_prefixed_pad_prf":
        return [
            "Generate pad PRF inputs whose first 32 seed bytes equal each registered domain prefix and assert no registered public-frame verifier accepts the bytes.",
            "Swap pad PRF bytes into projection-mask, matrix, challenge-action, carrier, and opening verifier entry points and require deterministic rejection.",
            "Check all selected pad blocks and counter values against mask-lane frame inputs for byte equality; preserve the existing pad-vs-mask zero-collision assertion.",
        ]
    if root_cause == "raw_domain_profile_concat":
        return [
            f"For {joined}, mutate one byte in the domain/profile boundary and require verifier rejection.",
            "Try domain/profile boundary-shift encodings with the same concatenated bytes and assert no alternate frame builder accepts them.",
            "Pair each raw profile frame with every fixed-length raw frame of different domain and assert bytes differ before hashing.",
        ]
    if root_cause == "raw_profile_len_variable_tail":
        return [
            f"For {joined}, reject statement_frame_len values that are one byte short, one byte long, or inconsistent with the remaining input.",
            "Swap a valid TLV transcript body into the raw profile length field frame and require rejection.",
        ]
    if root_cause == "variable_length_preflight_tlv":
        return [
            f"For {joined}, mutate append_frame label lengths, data lengths, field order, and domain bytes and require digest mismatch or parser rejection.",
            "Run the preflight frames in a separate all-production theorem fixture if selected-route scope excludes them.",
        ]
    return [
        f"For {joined}, mutate the TLV domain field, each label length, each data length, and field order.",
        "Try every same-family variable frame body under every other registered variable-frame domain and require digest mismatch or parser rejection.",
        "Add minimum-length, maximum-accepted-length, and one-over-maximum test vectors once accepted bounds are recorded.",
    ]


def redesign_flag(root_cause: str, entries: list[dict[str, Any]]) -> dict[str, Any]:
    unknown_bounds = [
        entry["id"]
        for entry in entries
        if entry.get("min_input_bytes") is None or entry.get("max_input_bytes") is None
    ]
    if root_cause == "secret_prefixed_pad_prf":
        return {
            "requires_redesign_now": False,
            "conditional_trigger": (
                "Redesign only if a canonical public frame can accept an 82-byte input "
                "whose first 32 bytes are adversary-controlled seed material and whose "
                "remaining bytes match the pad frame/counter layout."
            ),
            "preferred_redesign_if_triggered": "prefix pad PRF inputs with an immutable public domain before the secret seed, e.g. domain || pad_seed || block || counter.",
        }
    if unknown_bounds:
        return {
            "requires_redesign_now": False,
            "conditional_trigger": (
                "Redesign only if exact builders show two distinct registered frame "
                "families can emit identical byte strings under accepted inputs."
            ),
            "preferred_redesign_if_triggered": "add a fixed public frame tag or length-delimited envelope before hashing the colliding family.",
        }
    return {
        "requires_redesign_now": False,
        "conditional_trigger": "No collision-class trigger identified from the current registry metadata.",
        "preferred_redesign_if_triggered": None,
    }


def build_payload() -> dict[str, Any]:
    registry = load_json(REGISTRY_JSON)
    mask_frame = load_json(MASK_FRAME_JSON)
    entries = by_id(registry["registry_entries"])

    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for risk in registry["byte_frame_ambiguity_risks"]:
        entry = entries.get(risk.get("registry_id", ""))
        grouped[risk_root_cause(risk, entry)].append(risk)

    closure_groups = []
    for root_cause in sorted(grouped):
        risks = grouped[root_cause]
        group_entries = [entries[risk["registry_id"]] for risk in risks if "registry_id" in risk]
        min_lengths = sorted(
            {
                entry["min_input_bytes"]
                for entry in group_entries
                if entry.get("min_input_bytes") is not None
            }
        )
        unknown_length_entries = [
            entry["id"]
            for entry in group_entries
            if entry.get("min_input_bytes") is None or entry.get("max_input_bytes") is None
        ]
        representative_entry = group_entries[0] if group_entries else None
        closure_groups.append(
            {
                "root_cause": root_cause,
                "risk_count": len(risks),
                "risk_ids": [risk["id"] for risk in risks],
                "registry_ids": [entry["id"] for entry in group_entries],
                "families": sorted({entry["family"] for entry in group_entries}),
                "frame_encodings": sorted({entry["frame_encoding"] for entry in group_entries}),
                "known_min_input_bytes": min_lengths,
                "unknown_length_entries": unknown_length_entries,
                "action_class": group_action_class(root_cause, unknown_length_entries),
                "required_code_change": group_required_code_change(
                    root_cause, unknown_length_entries, representative_entry
                ),
                "byte_level_certificate": certificate_spec(root_cause, risks),
                "negative_tests": negative_test_spec(root_cause, risks),
                "redesign": redesign_flag(root_cause, group_entries),
            }
        )

    implementation_targets = [
        {
            "id": "global_frame_builder_inventory",
            "kind": "proof_test_artifact",
            "scope": "Add canonical byte-frame builders or builder probes for every registered SHA-256 input family.",
            "closes_groups": [
                "secret_prefixed_pad_prf",
                "raw_domain_profile_concat",
                "raw_profile_len_variable_tail",
                "variable_length_selected_tlv",
                "variable_length_preflight_tlv",
            ],
        },
        {
            "id": "append_frame_tlv_injectivity_certificate",
            "kind": "proof_artifact",
            "scope": "Emit per-frame TLV label order, label lengths, data-length offsets, minimum lengths, and accepted bounds.",
            "closes_groups": ["variable_length_selected_tlv", "variable_length_preflight_tlv"],
        },
        {
            "id": "pad_prf_global_cross_family_negative_tests",
            "kind": "test_artifact",
            "scope": "Extend the current pad-vs-mask certificate to every registered frame family and adversarial seed prefix.",
            "closes_groups": ["secret_prefixed_pad_prf"],
        },
        {
            "id": "raw_profile_boundary_negative_tests",
            "kind": "test_artifact",
            "scope": "Assert fixed domain/profile split points for raw profile frames and reject boundary-shift encodings.",
            "closes_groups": ["raw_domain_profile_concat", "raw_profile_len_variable_tail"],
        },
        {
            "id": "preflight_scope_decision",
            "kind": "scope_or_code_decision",
            "scope": "Either exclude preflight-only frames from the selected signer/verifier theorem or give them full all-production byte-frame certificates.",
            "closes_groups": ["variable_length_preflight_tlv"],
        },
    ]

    redesign_groups = [
        group["root_cause"]
        for group in closure_groups
        if group["redesign"]["requires_redesign_now"]
    ]
    return {
        "schema": "Descender.pre_signature_projection_global_frame_disjointness_plan.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "global_frame_disjointness_closure_plan_fail_closed",
        "deployment_ready": False,
        "closes_global_frame_disjointness": False,
        "source_artifacts": {
            "global_domain_registry": rel(REGISTRY_JSON),
            "mask_frame_disjointness": rel(MASK_FRAME_JSON),
            "global_domain_registry_status": registry.get("status"),
            "mask_frame_disjointness_status": mask_frame.get("status"),
        },
        "target": registry["target"],
        "risk_summary": {
            "total_ambiguity_risks": len(registry["byte_frame_ambiguity_risks"]),
            "root_cause_group_count": len(closure_groups),
            "groups_requiring_code_change_or_probe": [
                group["root_cause"]
                for group in closure_groups
                if "code" in group["action_class"]
                or "probe" in group["action_class"]
                or "registry_extension" in group["action_class"]
                or "scope" in group["action_class"]
            ],
            "groups_requiring_proof_or_test_artifacts": [
                group["root_cause"]
                for group in closure_groups
                if "proof" in group["action_class"]
                or "test" in group["action_class"]
                or "certificate" in group["action_class"]
            ],
            "collision_classes_requiring_redesign_now": redesign_groups,
        },
        "closure_groups": closure_groups,
        "next_implementation_targets": implementation_targets,
        "decision": {
            "can_discharge_global_frame_disjointness_now": False,
            "reason": (
                "The ambiguity risks are now grouped into concrete closure work, but "
                "global disjointness remains open until canonical byte-frame builders, "
                "TLV injectivity certificates, and cross-family negative tests are emitted."
            ),
            "redesign_required_now": bool(redesign_groups),
            "redesign_reason": (
                "No concrete byte-equality collision class is known from the registry. "
                "Redesign is deferred unless the planned certificates find accepted "
                "equal byte strings across distinct registered families."
            ),
        },
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    summary = payload["risk_summary"]
    lines = [
        "# Pre-Signature Projection Global Frame Disjointness Plan",
        "",
        "This artifact turns the global domain registry's byte-frame ambiguity",
        "risks into a concrete closure plan for the selected 308-byte/Fq12 route.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- closes global frame disjointness: `{payload['closes_global_frame_disjointness']}`",
        f"- ambiguity risks: `{summary['total_ambiguity_risks']}`",
        f"- root-cause groups: `{summary['root_cause_group_count']}`",
        f"- collision classes requiring redesign now: `{len(summary['collision_classes_requiring_redesign_now'])}`",
        "",
        "## Risk Groups",
        "",
        "| root cause | risks | families | action class | redesign now |",
        "|---|---:|---|---|---|",
    ]
    for group in payload["closure_groups"]:
        lines.append(
            "| `{}` | {} | `{}` | `{}` | `{}` |".format(
                group["root_cause"],
                group["risk_count"],
                ", ".join(group["families"]),
                group["action_class"],
                group["redesign"]["requires_redesign_now"],
            )
        )

    lines.extend(["", "## Closure Detail", ""])
    for group in payload["closure_groups"]:
        lines.extend(
            [
                f"### {group['root_cause']}",
                "",
                f"Risks: `{group['risk_count']}`",
                "",
                f"Registry ids: `{', '.join(group['registry_ids'])}`",
                "",
                f"Required code change: {group['required_code_change']}",
                "",
                "Byte-level certificate:",
            ]
        )
        lines.extend(f"- {item}" for item in group["byte_level_certificate"])
        lines.extend(["", "Negative tests:"])
        lines.extend(f"- {item}" for item in group["negative_tests"])
        lines.extend(
            [
                "",
                f"Redesign trigger: {group['redesign']['conditional_trigger']}",
                "",
            ]
        )

    lines.extend(
        [
            "## Next Implementation Targets",
            "",
            "| target | kind | closes groups |",
            "|---|---|---|",
        ]
    )
    for target in payload["next_implementation_targets"]:
        lines.append(
            f"| `{target['id']}` | `{target['kind']}` | `{', '.join(target['closes_groups'])}` |"
        )
    lines.extend(
        [
            "",
            "## Decision",
            "",
            payload["decision"]["reason"],
            "",
            payload["decision"]["redesign_reason"],
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload) + "\n")
    print(
        json.dumps(
            {
                "json": rel(DEFAULT_JSON),
                "markdown": rel(DEFAULT_MD),
                "status": payload["status"],
                "ambiguity_risks": payload["risk_summary"]["total_ambiguity_risks"],
                "root_cause_groups": payload["risk_summary"]["root_cause_group_count"],
                "redesign_required_now": payload["decision"]["redesign_required_now"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
