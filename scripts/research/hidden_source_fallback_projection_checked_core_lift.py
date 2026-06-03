#!/usr/bin/env python3
"""Build the checked-core lift ledger for the selected projection route."""

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
    / "hidden_source_fallback_projection_checked_core_lift_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "hidden_source_fallback_projection_checked_core_lift_n32_q12289.md"
)
PROJECTION_MATRIX_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_matrix_builder_n32_q12289.json"
)
RESPONSE_LINK_CERT_JSON = (
    READINESS_DIR
    / "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"
)
RESPONSE_LINK_MANIFEST_JSON = (
    READINESS_DIR
    / "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json"
)

Q = 12289
CHECKED_CORE_RESPONSE_COLUMNS = 23


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def lift_digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.projection_checked_core_lift.v1")
    encoded = json.dumps(payload, sort_keys=True, separators=(",", ":")).encode()
    h.update(encoded)
    return h.hexdigest()


def build_payload() -> dict[str, Any]:
    projection = load_json(PROJECTION_MATRIX_JSON)
    response = load_json(RESPONSE_LINK_CERT_JSON)
    manifest = load_json(RESPONSE_LINK_MANIFEST_JSON)

    projection_target = projection["target"]
    projection_rank = int(projection_target["rank"])
    projection_cols = int(projection_target["columns"])
    checked_core_rank = int(response["combined_constraint_rank"])
    checked_core_kernel_dim = int(response["combined_kernel_dimension"])
    checked_core_cols = CHECKED_CORE_RESPONSE_COLUMNS

    direct_sum_cols = checked_core_cols + projection_cols
    direct_sum_rank = checked_core_rank + projection_rank
    direct_sum_nullity = direct_sum_cols - direct_sum_rank
    linear_bridge_rank_to_zero_kernel = direct_sum_nullity
    exact_bounded_bridge_status = "not_run"

    rank_accounting = {
        "checked_core_response_columns": checked_core_cols,
        "checked_core_combined_rank": checked_core_rank,
        "checked_core_kernel_dimension_after_response_link": checked_core_kernel_dim,
        "projection_columns": projection_cols,
        "projection_rank": projection_rank,
        "projection_only_nullity": int(projection_target["nullity_projection_only"]),
        "direct_sum_columns": direct_sum_cols,
        "direct_sum_rank_without_bridges": direct_sum_rank,
        "direct_sum_nullity_without_bridges": direct_sum_nullity,
        "minimum_additional_independent_linear_bridge_rank_for_zero_kernel": linear_bridge_rank_to_zero_kernel,
        "bounded_infeasibility_may_require_less_than_zero_kernel": True,
    }
    bridge_interfaces = [
        {
            "id": "response_to_account_witness_projection",
            "status": "open",
            "source_space": "checked_core_response_difference",
            "target_space": "account_witness_projection",
            "requirement": "derive public equations linking accepted response differences to the six account-witness projection lanes",
        },
        {
            "id": "response_to_pad_seed_projection",
            "status": "open",
            "source_space": "checked_core_response_difference",
            "target_space": "pad_seed_projection",
            "requirement": "derive public equations linking accepted response differences to the six pad-source projection lanes",
        },
        {
            "id": "semantic_decoder_canonicality",
            "status": "open",
            "source_space": "serialized_308_byte_fq12_envelope",
            "target_space": "hidden_source_projection_space",
            "requirement": "prove the byte decoder rejects noncanonical q12289 encodings, malformed flags, digest-only carriers, and context replay",
        },
        {
            "id": "transcript_binding_bridge",
            "status": "open",
            "source_space": "checked_core_transcript",
            "target_space": "projection_carrier_transcript",
            "requirement": "prove the projection carrier is bound to the same public key, account state, message, slot, checked-core body, and statement digest",
        },
        {
            "id": "bounded_bad_vector_export",
            "status": "open",
            "source_space": "combined_lift_system",
            "target_space": "external_review_artifact",
            "requirement": "if the exact lift replay fails, export the smallest nonzero bounded survivor and classify the failing bridge",
        },
    ]
    structural_controls = [
        {
            "id": "checked_core_certificate_replayed",
            "status": "pass" if response.get("status") == "infeasible" else "fail",
            "observed": response.get("status"),
            "required": "infeasible",
        },
        {
            "id": "checked_core_manifest_replayed",
            "status": "pass"
            if manifest.get("replay", {}).get("status") == "pass"
            else "fail",
            "observed": manifest.get("replay", {}).get("status"),
            "required": "pass",
        },
        {
            "id": "projection_matrix_full_row_rank",
            "status": "pass"
            if projection_rank == int(projection_target["rows"])
            else "fail",
            "observed": projection_rank,
            "required": int(projection_target["rows"]),
        },
        {
            "id": "direct_sum_without_bridges_is_insufficient",
            "status": "expected_open",
            "observed": direct_sum_nullity,
            "required": "must be closed by bridge equations plus semantic decoder or by exact bounded infeasibility",
        },
        {
            "id": "response_link_certificate_not_reused_as_projection_certificate",
            "status": "pass",
            "observed": "separate variable spaces and separate source artifacts",
            "required": "do not treat the 23-column response-link certificate as proving the 64-column hidden-source projection map",
        },
    ]
    digest_payload = {
        "response_matrix_sha256": response.get("matrix_transcript_sha256"),
        "response_reduced_basis_sha256": response.get("reduced_basis_sha256"),
        "projection_matrix_sha256": projection_target.get("matrix_sha256"),
        "rank_accounting": rank_accounting,
        "bridge_interfaces": bridge_interfaces,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_checked_core_lift.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "selected_projection_checked_core_lift_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "projection_matrix": rel(PROJECTION_MATRIX_JSON),
            "projection_matrix_schema": projection.get("schema"),
            "response_link_certificate": rel(RESPONSE_LINK_CERT_JSON),
            "response_link_audit_manifest": rel(RESPONSE_LINK_MANIFEST_JSON),
            "response_link_certificate_status": response.get("status"),
            "response_link_manifest_status": manifest.get("replay", {}).get("status"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "checked_core_signature_bytes": 252,
            "projection_carrier_bytes": 56,
            "envelope_bytes": 308,
            "lift_digest": lift_digest(digest_payload),
        },
        "rank_accounting": rank_accounting,
        "structural_controls": structural_controls,
        "bridge_interfaces": bridge_interfaces,
        "exact_replay_boundary": {
            "status": exact_bounded_bridge_status,
            "must_construct": "single exact q12289/integer system with checked-core response variables, hidden-source projection variables, bridge equations, and semantic decoder rejection predicates",
            "must_output": "infeasible nonzero bounded survivor, or exported bad vector with source-space classification",
            "response_difference_bound": int(response.get("response_difference_bound")),
        },
        "decision": {
            "can_accept_now": False,
            "lift_certificate_complete": False,
            "reason": "checked-core and projection certificates are both replayed inputs, but without bridge equations and semantic decoder constraints their direct sum still has a large open nullity",
            "next_artifact": "hidden_source_fallback_projection_semantic_bridge_spec",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    rank = payload["rank_accounting"]
    lines = [
        "# Hidden-Source Projection Checked-Core Lift",
        "",
        "This artifact combines the replayed checked-core response-link certificate",
        "with the selected hidden-source projection matrix at the level of rank",
        "accounting and proof obligations.  It is fail-closed: without bridge",
        "equations and semantic decoder constraints, the direct-sum system is not",
        "an acceptance certificate.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- lift digest: `{target['lift_digest']}`",
        "",
        "## Rank Accounting",
        "",
        f"- checked-core response columns: `{rank['checked_core_response_columns']}`",
        f"- checked-core combined rank: `{rank['checked_core_combined_rank']}`",
        f"- checked-core kernel dimension after response link: `{rank['checked_core_kernel_dimension_after_response_link']}`",
        f"- projection columns: `{rank['projection_columns']}`",
        f"- projection rank: `{rank['projection_rank']}`",
        f"- projection-only nullity: `{rank['projection_only_nullity']}`",
        f"- direct-sum columns: `{rank['direct_sum_columns']}`",
        f"- direct-sum rank without bridges: `{rank['direct_sum_rank_without_bridges']}`",
        f"- direct-sum nullity without bridges: `{rank['direct_sum_nullity_without_bridges']}`",
        f"- minimum additional independent linear bridge rank for zero kernel: `{rank['minimum_additional_independent_linear_bridge_rank_for_zero_kernel']}`",
        "",
        "## Structural Controls",
        "",
        "| id | status | observed | required |",
        "|---|---|---|---|",
    ]
    for row in payload["structural_controls"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['observed']}` | `{row['required']}` |"
        )
    lines.extend(
        [
            "",
            "## Bridge Interfaces",
            "",
            "| id | status | source | target | requirement |",
            "|---|---|---|---|---|",
        ]
    )
    for row in payload["bridge_interfaces"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['source_space']}` | `{row['target_space']}` | {row['requirement']} |"
        )
    lines.extend(
        [
            "",
            "## Exact Replay Boundary",
            "",
            f"- status: `{payload['exact_replay_boundary']['status']}`",
            f"- must construct: {payload['exact_replay_boundary']['must_construct']}",
            f"- must output: {payload['exact_replay_boundary']['must_output']}",
            f"- response-difference bound: `{payload['exact_replay_boundary']['response_difference_bound']}`",
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- lift certificate complete: `{payload['decision']['lift_certificate_complete']}`",
            f"- reason: {payload['decision']['reason']}",
            f"- next artifact: `{payload['decision']['next_artifact']}`",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.parent.mkdir(parents=True, exist_ok=True)
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
