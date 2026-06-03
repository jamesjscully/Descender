#!/usr/bin/env python3
"""Define concrete break-export records for the projection-carrier fallback."""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_break_export_contract_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_break_export_contract_n32_q12289.md"
)
PUBLIC_BRIDGE_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_public_bridge_predicate_n32_q12289.json"
)
BREAK_REDUCTION_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.json"
)
EXTRACTOR_THEOREM_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.json"
)
RESPONSE_LINK_CERT_JSON = (
    READINESS_DIR
    / "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"
)

Q = 12289


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.break_export_contract.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def named_break_ids(extractor: dict[str, Any]) -> set[str]:
    return {row["id"] for row in extractor.get("named_breaks", []) if "id" in row}


def build_payload() -> dict[str, Any]:
    bridge = load_json(PUBLIC_BRIDGE_JSON)
    reduction = load_json(BREAK_REDUCTION_JSON)
    extractor = load_json(EXTRACTOR_THEOREM_JSON)
    response = load_json(RESPONSE_LINK_CERT_JSON)
    breaks = named_break_ids(extractor)
    export_records = [
        {
            "id": "checked_core_or_response_link_break_export",
            "named_break": "checked_core_or_response_link_break",
            "status": "specified_not_implemented",
            "trigger": "public bridge predicate accepts, but checked-core response relation or response-link bounded-kernel certificate is violated",
            "required_fields": [
                "serialized_checked_core_transcript",
                "projection_carrier_56_bytes",
                "decoded_12_q12289_lanes",
                "response_difference_23_field_vector",
                "checked_core_matrix_domain",
                "checked_core_matrix_transcript_sha256",
                "failed_equation_index_or_kernel_certificate_id",
                "field_modulus_q",
            ],
            "replay_rule": "recompute the checked-core response matrix from the domain, replay the 23-column response vector, and verify the stated checked-core or bounded-kernel violation",
            "byte_status": "external_witness_record_not_signature_bytes",
        },
        {
            "id": "hidden_source_opening_binding_break_export",
            "named_break": "hidden_relation_proof_soundness_break",
            "status": "specified_not_implemented",
            "trigger": "public bridge predicate accepts, but selected projection coordinates are not computed from the hidden-source opening used by the accepted relation",
            "required_fields": [
                "serialized_hidden_relation_statement",
                "projection_carrier_56_bytes",
                "decoded_12_q12289_lanes",
                "account_witness_opening_frame",
                "pad_seed_opening_frame",
                "claimed_projection_symbols",
                "recomputed_projection_values",
                "mismatching_lane_or_symbol",
            ],
            "replay_rule": "decode the opening frames, recompute selected projection symbols, and verify either relation soundness failure or commitment/opening binding failure",
            "byte_status": "external_witness_record_not_signature_bytes",
        },
        {
            "id": "account_commitment_binding_break_export",
            "named_break": "account_commitment_binding_break",
            "status": "specified_not_implemented",
            "trigger": "two accepted openings bind the same account commitment to different witness or pad semantic values",
            "required_fields": [
                "commitment_bytes",
                "opening_frame_a",
                "opening_frame_b",
                "semantic_value_a",
                "semantic_value_b",
                "statement_digest",
            ],
            "replay_rule": "verify both openings against the same commitment and verify their semantic values differ",
            "byte_status": "external_witness_record_not_signature_bytes",
        },
        {
            "id": "pad_prf_or_pad_opening_break_export",
            "named_break": "pad_prf_or_pad_opening_break",
            "status": "specified_not_implemented",
            "trigger": "pad opening is accepted but does not derive the public pad statement or projection symbols",
            "required_fields": [
                "pad_seed_opening_frame",
                "pad_statement_bytes",
                "projection_carrier_56_bytes",
                "pad_projection_symbols",
                "recomputed_pad_projection_values",
                "sha256_pad_prf_domain",
            ],
            "replay_rule": "recompute the pad PRF and selected pad projections, then verify the mismatch",
            "byte_status": "external_witness_record_not_signature_bytes",
        },
    ]
    available_count = sum(1 for row in export_records if row["named_break"] in breaks)
    implemented_count = sum(1 for row in export_records if row["status"] == "implemented")
    review_checks = [
        {
            "id": "all_exports_use_named_breaks",
            "status": "pass" if available_count == len(export_records) else "fail",
            "observed": sorted(row["named_break"] for row in export_records),
            "required": sorted(breaks),
        },
        {
            "id": "checked_core_export_replayable",
            "status": "specified",
            "observed": response.get("matrix_transcript_sha256"),
            "required": "checked-core matrix transcript hash plus response vector",
        },
        {
            "id": "exports_do_not_increase_signature_bytes",
            "status": "pass",
            "observed": "external_witness_record_not_signature_bytes",
            "required": "break exports are review artifacts, not deployed signature payload",
        },
        {
            "id": "exports_implemented",
            "status": "open" if implemented_count == 0 else "partial",
            "observed": implemented_count,
            "required": len(export_records),
        },
    ]
    digest_payload = {
        "predicate_digest": bridge.get("target", {}).get("predicate_digest"),
        "reduction_digest": reduction.get("target", {}).get("reduction_digest"),
        "response_matrix_sha256": response.get("matrix_transcript_sha256"),
        "export_records": export_records,
        "review_checks": review_checks,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_break_export_contract.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "break_export_contract_specified_not_implemented_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "public_bridge_predicate": rel(PUBLIC_BRIDGE_JSON),
            "checked_core_break_reduction": rel(BREAK_REDUCTION_JSON),
            "extractor_theorem": rel(EXTRACTOR_THEOREM_JSON),
            "response_link_certificate": rel(RESPONSE_LINK_CERT_JSON),
            "public_bridge_schema": bridge.get("schema"),
            "checked_core_break_reduction_schema": reduction.get("schema"),
            "extractor_theorem_schema": extractor.get("schema"),
            "response_link_matrix_domain": response.get("matrix_domain"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "contract_digest": digest(digest_payload),
        },
        "export_records": export_records,
        "review_checks": review_checks,
        "coverage": {
            "export_record_count": len(export_records),
            "named_break_exports_available": available_count,
            "implemented_export_count": implemented_count,
            "signature_byte_increase": 0,
            "public_bridge_lane_checks": bridge.get("coverage", {}).get(
                "lane_checks_specified"
            ),
        },
        "decision": {
            "can_accept_now": False,
            "can_run_exact_solver_now": False,
            "reason": "break export records are specified, but exporter code and negative tests are not implemented",
            "next_artifact": "hidden_source_fallback_projection_break_export_negative_tests",
        },
        "structural_controls": [
            {
                "id": "all_export_records_named",
                "status": "pass" if available_count == len(export_records) else "fail",
                "observed": available_count,
                "required": len(export_records),
            },
            {
                "id": "no_signature_byte_growth",
                "status": "pass",
                "observed": 0,
                "required": 0,
            },
            {
                "id": "acceptance_still_fail_closed",
                "status": "pass",
                "observed": False,
                "required": False,
            },
        ],
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    coverage = payload["coverage"]
    lines = [
        "# Hidden-Source Projection Break Export Contract",
        "",
        "This artifact specifies the concrete break records needed by the",
        "projection-carrier fallback reduction. These records are external review",
        "witnesses; they do not add bytes to deployed signatures.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- contract digest: `{target['contract_digest']}`",
        f"- export records: `{coverage['export_record_count']}`",
        f"- named break exports available: `{coverage['named_break_exports_available']}`",
        f"- implemented exports: `{coverage['implemented_export_count']}`",
        f"- signature byte increase: `{coverage['signature_byte_increase']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## Export Records",
        "",
        "| export | named break | status | trigger |",
        "|---|---|---|---|",
    ]
    for row in payload["export_records"]:
        lines.append(
            f"| `{row['id']}` | `{row['named_break']}` | `{row['status']}` | {row['trigger']} |"
        )
    lines.extend(
        [
            "",
            "## Review Checks",
            "",
            "| check | status | observed | required |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["review_checks"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['observed']}` | `{row['required']}` |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "The reduction now has concrete output shapes for each bridge failure class.",
            "This still is not proof closure: the exporter and negative tests must be",
            "implemented so accepting a bad bridge either rejects or emits one of these",
            "records.",
            "",
            "## Source Artifacts",
            "",
        ]
    )
    for key, value in payload["source_artifacts"].items():
        lines.append(f"- `{key}`: `{value}`")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(f"wrote {rel(DEFAULT_JSON)}")
    print(f"wrote {rel(DEFAULT_MD)}")


if __name__ == "__main__":
    main()
