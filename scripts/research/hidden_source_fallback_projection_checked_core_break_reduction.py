#!/usr/bin/env python3
"""Specify the checked-core break reduction fallback for the projection carrier."""

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
    / "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_checked_core_break_reduction_n32_q12289.md"
)
DIRECT_LIFT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_direct_linear_lift_attempt_n32_q12289.json"
)
BRIDGE_DERIVATION_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_bridge_equation_derivation_n32_q12289.json"
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
    h.update(b"Descender.hidden_source.checked_core_break_reduction.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def named_break_ids(extractor: dict[str, Any]) -> list[str]:
    return [row.get("id") for row in extractor.get("named_breaks", []) if row.get("id")]


def build_payload() -> dict[str, Any]:
    direct_lift = load_json(DIRECT_LIFT_JSON)
    derivation = load_json(BRIDGE_DERIVATION_JSON)
    extractor = load_json(EXTRACTOR_THEOREM_JSON)
    response = load_json(RESPONSE_LINK_CERT_JSON)
    breaks = named_break_ids(extractor)
    required_break = "checked_core_or_response_link_break"
    required_break_available = required_break in breaks
    reduction_obligations = [
        {
            "id": "name_break_target",
            "status": "closed" if required_break_available else "open",
            "required": required_break,
            "observed": breaks,
            "review_rule": "the fallback must cite an externally reviewable break target rather than an opaque extractor event",
        },
        {
            "id": "public_bridge_predicate",
            "status": "open",
            "required": "byte-level verifier predicate tying carrier projection lanes to checked-core response coordinates",
            "observed": "missing_after_direct_linear_lift",
            "review_rule": "without this predicate the reduction cannot distinguish honest mismatch from an unconstrained projection carrier",
        },
        {
            "id": "extractor_output_contract",
            "status": "open",
            "required": "given two accepting transcripts with same public carrier and incompatible checked-core views, export a checked-core/response-link break witness",
            "observed": "not_yet_implemented",
            "review_rule": "the reduction must output concrete bytes and field coordinates, not a semantic placeholder",
        },
        {
            "id": "negative_acceptance_test",
            "status": "open",
            "required": "mutating any bridged projection lane must either reject or export the named break event",
            "observed": "not_yet_implemented",
            "review_rule": "this is the implementation-side fail-closed guard for the reduction",
        },
        {
            "id": "exact_replay_hook",
            "status": "blocked",
            "required": "nonzero checked-core coupling rows or an implemented break-export checker",
            "observed": direct_lift.get("attempt", {}).get(
                "derived_checked_core_coupling_rows"
            ),
            "review_rule": "the existing exact solver remains blocked until the reduction has concrete bridge rows or concrete exported break witnesses",
        },
    ]
    closed_count = sum(1 for row in reduction_obligations if row["status"] == "closed")
    open_count = sum(1 for row in reduction_obligations if row["status"] == "open")
    blocked_count = sum(1 for row in reduction_obligations if row["status"] == "blocked")
    theorem_shape = {
        "premise": "an adversary produces an accepting projection-carrier transcript whose carrier lanes pass byte decoding while the checked-core response relation is not coupled to the same hidden-source opening",
        "conclusion": "either the transcript violates the public bridge predicate, or it yields checked_core_or_response_link_break, or it yields a hidden-source opening/binding break named by the extractor theorem",
        "current_status": "skeleton_only_until_public_bridge_predicate_and_export_contract_are_instantiated",
    }
    digest_payload = {
        "direct_lift_digest": direct_lift.get("target", {}).get("attempt_digest"),
        "derivation_digest": derivation.get("target", {}).get("derivation_digest"),
        "response_matrix_sha256": response.get("matrix_transcript_sha256"),
        "required_break_available": required_break_available,
        "reduction_obligations": reduction_obligations,
        "theorem_shape": theorem_shape,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_checked_core_break_reduction.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "checked_core_break_reduction_skeleton_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "direct_linear_lift_attempt": rel(DIRECT_LIFT_JSON),
            "bridge_equation_derivation": rel(BRIDGE_DERIVATION_JSON),
            "extractor_theorem": rel(EXTRACTOR_THEOREM_JSON),
            "response_link_certificate": rel(RESPONSE_LINK_CERT_JSON),
            "direct_linear_lift_attempt_schema": direct_lift.get("schema"),
            "bridge_equation_derivation_schema": derivation.get("schema"),
            "extractor_theorem_schema": extractor.get("schema"),
            "response_link_matrix_domain": response.get("matrix_domain"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "reduction_digest": digest(digest_payload),
        },
        "named_break_target": {
            "id": required_break,
            "available_in_extractor_theorem": required_break_available,
            "all_available_named_breaks": breaks,
        },
        "theorem_shape": theorem_shape,
        "reduction_obligations": reduction_obligations,
        "obligation_counts": {
            "closed": closed_count,
            "open": open_count,
            "blocked": blocked_count,
        },
        "decision": {
            "can_accept_now": False,
            "can_run_exact_solver_now": False,
            "reason": "the named break target exists, but the fallback reduction still needs a public bridge predicate, concrete extractor output contract, and negative acceptance test",
            "next_artifact": "hidden_source_fallback_projection_public_bridge_predicate",
        },
        "structural_controls": [
            {
                "id": "required_break_is_named",
                "status": "pass" if required_break_available else "fail",
                "observed": required_break_available,
                "required": True,
            },
            {
                "id": "missing_bridge_not_accepted_as_proof",
                "status": "pass",
                "observed": False,
                "required": False,
            },
            {
                "id": "fallback_is_fail_closed",
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
    theorem = payload["theorem_shape"]
    named = payload["named_break_target"]
    counts = payload["obligation_counts"]
    lines = [
        "# Hidden-Source Projection Checked-Core Break Reduction",
        "",
        "This artifact starts the fallback reduction selected after the direct",
        "linear lift failed to find a public response-to-projection map. It is a",
        "reduction skeleton and remains fail-closed.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- reduction digest: `{target['reduction_digest']}`",
        f"- named break target: `{named['id']}`",
        f"- named break available: `{named['available_in_extractor_theorem']}`",
        f"- obligations closed/open/blocked: `{counts['closed']}/{counts['open']}/{counts['blocked']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## Theorem Shape",
        "",
        f"- premise: {theorem['premise']}",
        f"- conclusion: {theorem['conclusion']}",
        f"- current status: `{theorem['current_status']}`",
        "",
        "## Obligations",
        "",
        "| obligation | status | observed | required |",
        "|---|---|---|---|",
    ]
    for row in payload["reduction_obligations"]:
        lines.append(
            f"| `{row['id']}` | `{row['status']}` | `{row['observed']}` | `{row['required']}` |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "The fallback has a named break target, so it is cleaner than adding a",
            "speculative map assumption. It is not a proof closure yet: the public",
            "bridge predicate and concrete break-export contract are the remaining",
            "work needed before the reduction can replace explicit coupling rows.",
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
