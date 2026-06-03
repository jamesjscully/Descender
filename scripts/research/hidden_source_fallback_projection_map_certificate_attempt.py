#!/usr/bin/env python3
"""Generate the certificate-attempt target for the top fallback projection map."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_certificate_attempt_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_map_certificate_attempt_n32_q12289.md"
)
ENUMERATOR_JSON = (
    READINESS_DIR / "hidden_source_fallback_projection_map_enumerator_n32_q12289.json"
)
RESPONSE_LINK_CERT_JSON = (
    READINESS_DIR
    / "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"
)
PARAMETER_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def source_contains(name: str, source: str) -> bool:
    return name in source


def build_payload() -> dict[str, Any]:
    enumerator = load_json(ENUMERATOR_JSON)
    response_link = load_json(RESPONSE_LINK_CERT_JSON)
    parameter_source = PARAMETER_REFERENCE.read_text()
    best = enumerator["candidate_maps"][0]
    certificate_target = {
        "candidate_family": best["family"],
        "map_digest": best["map_digest"],
        "projection_elements": best["projection_elements"],
        "fallback_carrier_bytes": 56,
        "fallback_envelope_bytes": 308,
        "response_difference_bound": 96,
        "certificate_mode": "not_run",
        "certificate_kind": "exact_bounded_kernel_and_semantic_control_attempt",
    }
    exact_replay_requirements = [
        {
            "id": "projection_matrix_construction",
            "requirement": "derive the twelve-row fallback projection matrix from the recorded candidate rows and domain-separated q12289 arithmetic",
            "status": "open",
        },
        {
            "id": "bounded_kernel_exact_replay",
            "requirement": "prove no nonzero accepted response-difference vector survives the checked-core equations and all twelve fallback projections inside the bound",
            "status": "open",
        },
        {
            "id": "semantic_projection_extraction",
            "requirement": "show accepted projections force both account-witness and pad-source openings or produce a named break",
            "status": "open",
        },
        {
            "id": "leakage_charge",
            "requirement": "charge twelve q12289 elements plus three check/flag bytes in the leakage theorem",
            "status": "closed"
            if source_contains(
                "hidden_source_fallback_projection_leakage_accounting",
                parameter_source,
            )
            else "open",
        },
        {
            "id": "byte_decoder_controls",
            "requirement": "reject noncanonical q12289 packing, malformed flags, replayed context digest, and digest-only carriers",
            "status": "open",
        },
    ]
    pass_conditions = [
        "exact bounded-kernel replay status is verified",
        "semantic negative controls reject without secret verifier inputs",
        "projection leakage theorem is written and charges every public fallback byte",
        "fallback decoder is implemented with fixed public rejection",
        "all 308-byte fallback claims are updated if and only if the fallback is selected",
    ]
    fail_conditions = [
        "nonzero bad vector survives the projection map in the accepted bound",
        "projection extraction needs secret verifier inputs",
        "projection leaks uncharged witness or pad-source information",
        "carrier requires more than 56 bytes or variable-length payloads",
        "fallback theorem cannot compose with checked-core, QROM, pad, keygen, and lifecycle assumptions",
    ]
    return {
        "schema": "Descender.hidden_source_fallback_projection_map_certificate_attempt.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "selected_projection_certificate_attempt_not_run_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "projection_map_enumerator": rel(ENUMERATOR_JSON),
            "projection_map_enumerator_schema": enumerator.get("schema"),
            "response_link_certificate": rel(RESPONSE_LINK_CERT_JSON),
            "response_link_certificate_schema": response_link.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "fallback_selected_now": True,
        },
        "certificate_target": certificate_target,
        "candidate_rows": best["rows"],
        "exact_replay_requirements": exact_replay_requirements,
        "pass_conditions": pass_conditions,
        "fail_conditions": fail_conditions,
        "decision": {
            "can_accept_now": False,
            "certificate_attempt_has_run": False,
            "reason": "the selected projection map is bound to a certificate target, but no exact projection certificate or leakage theorem has been run",
            "next_artifact": "fallback_projection_map_exact_certificate",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["certificate_target"]
    lines = [
        "# Hidden-Source Fallback Projection Map Certificate Attempt",
        "",
        "This artifact binds the top-ranked 56-byte projection map to the",
        "exact certificate attempt required for the selected 308-byte route.",
        "It remains fail-closed and does not run the exact",
        "certificate.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {payload['target']['selected_branch']}",
        f"- fallback selected now: `{payload['target']['fallback_selected_now']}`",
        f"- candidate family: `{target['candidate_family']}`",
        f"- map digest: `{target['map_digest']}`",
        f"- fallback carrier bytes: `{target['fallback_carrier_bytes']}`",
        f"- fallback envelope bytes: `{target['fallback_envelope_bytes']}`",
        f"- certificate mode: `{target['certificate_mode']}`",
        "",
        "## Candidate Rows",
        "",
        "| lane | kind | block | coord | coefficient mod q |",
        "|---:|---|---:|---:|---:|",
    ]
    for row in payload["candidate_rows"]:
        lines.append(
            f"| `{row['lane']}` | `{row['kind']}` | `{row['block']}` | "
            f"`{row['coord']}` | `{row['coefficient_mod_q']}` |"
        )
    lines.extend(
        [
            "",
            "## Exact Replay Requirements",
            "",
            "| id | status | requirement |",
            "|---|---|---|",
        ]
    )
    for row in payload["exact_replay_requirements"]:
        lines.append(f"| `{row['id']}` | `{row['status']}` | {row['requirement']} |")
    lines.extend(["", "## Pass Conditions", ""])
    lines.extend(f"- {condition}" for condition in payload["pass_conditions"])
    lines.extend(["", "## Fail Conditions", ""])
    lines.extend(f"- {condition}" for condition in payload["fail_conditions"])
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- certificate attempt has run: `{payload['decision']['certificate_attempt_has_run']}`",
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
