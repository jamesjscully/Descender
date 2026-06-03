#!/usr/bin/env python3
"""Generate byte-level negative tests for the state-assisted RO-PoK API."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.md"
)
RO_POK_API_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.json"
)
SEMANTIC_CONTROLS_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    api = load_json(RO_POK_API_JSON)
    semantic_controls = load_json(SEMANTIC_CONTROLS_JSON)
    base_frame = {
        "domain": "Descender.hidden_source.state_assisted.ro_pok.v1",
        "profile": "n32.q12289.fq12.state.ro_pok.v1",
        "statement_frame_len": "u32 little-endian",
        "statement_frame": "canonical verifier-owned public statement bytes",
        "opening_commitment": "32-byte prechallenge account-state opening commitment",
        "transcript_digest": "32-byte reviewed hidden-source proof transcript digest",
        "carrier": "32-byte proof handle",
    }
    byte_negative_tests = [
        {
            "id": "mutate_statement_frame_byte",
            "semantic_control": "false_statement_acceptance",
            "mutation": "flip one byte in statement_frame after transcript_digest and carrier are fixed",
            "must_reject": "canonical_frame_recompute",
            "accepts_only_if_break": "statement binding or RO proof-handle soundness break",
        },
        {
            "id": "mutate_statement_frame_length",
            "semantic_control": "relation_projection_mismatch",
            "mutation": "change statement_frame_len without changing statement_frame bytes",
            "must_reject": "canonical_frame_recompute",
            "accepts_only_if_break": "byte-frame parser ambiguity or transcript binding break",
        },
        {
            "id": "mutate_opening_commitment",
            "semantic_control": "precommitment_bypass",
            "mutation": "replace opening_commitment with a different nonzero 32-byte value while holding carrier fixed",
            "must_reject": "opening commitment binding",
            "accepts_only_if_break": "opening commitment binding or lifecycle precommitment break",
        },
        {
            "id": "zero_opening_commitment",
            "semantic_control": "precommitment_bypass",
            "mutation": "set opening_commitment to all zero bytes",
            "must_reject": "state schema validation",
            "accepts_only_if_break": "state schema fail-open bug",
        },
        {
            "id": "mutate_transcript_digest",
            "semantic_control": "carrier_context_replay",
            "mutation": "flip one byte in transcript_digest while holding statement_frame, opening_commitment, and carrier fixed",
            "must_reject": "carrier context binding",
            "accepts_only_if_break": "transcript-domain separation or RO binding break",
        },
        {
            "id": "mutate_carrier_byte",
            "semantic_control": "carrier_context_replay",
            "mutation": "flip one byte in the 32-byte carrier",
            "must_reject": "proof-handle verification",
            "accepts_only_if_break": "RO proof-handle soundness break",
        },
        {
            "id": "wrong_domain",
            "semantic_control": "relation_projection_mismatch",
            "mutation": "replace the domain separator with another valid Descender domain",
            "must_reject": "domain separation",
            "accepts_only_if_break": "cross-domain transcript collision",
        },
        {
            "id": "wrong_profile",
            "semantic_control": "relation_projection_mismatch",
            "mutation": "replace the profile string with another q12289/Fq profile",
            "must_reject": "profile binding",
            "accepts_only_if_break": "profile confusion or transcript binding break",
        },
        {
            "id": "digest_only_oracle_acceptance",
            "semantic_control": "witness_digest_substitution",
            "mutation": "use carrier = SHA256(frame_without_carrier) and no proof oracle witness",
            "must_reject": "no_digest_only_acceptance",
            "accepts_only_if_break": "the implementation treated digest equality as proof semantics",
        },
        {
            "id": "fixed_error_surface_probe",
            "semantic_control": "pad_seed_substitution",
            "mutation": "run all prior mutations and compare public error output",
            "must_reject": "fixed_error_surface",
            "accepts_only_if_break": "secret- or class-dependent diagnostic leakage",
        },
    ]
    executable_status = {
        "current": "default_fail_closed_rust_boundary_implemented",
        "reason": "the Rust reference boundary now exposes the public-only transcript frame/digest and fixed public rejection, but no audited accepting RO-PoK verifier exists",
        "promotion_requirement": "all acceptance-sensitive byte negative tests must be re-run against an audited accepting verifier before the 32-byte route can be externally reviewed",
        "remaining_gap": "the Rust boundary is not an accepting proof verifier and therefore does not discharge the explicit RO proof-handle assumption",
        "rust_boundary": {
            "crate": "tools/production-reference",
            "module": "tools/parameter-search/src/production_reference.rs",
            "public_input_type": "ProductionReviewedFq12StateAssistedRoPokPublicInput",
            "frame_function": "reviewed_fq12_state_assisted_ro_pok_transcript_frame",
            "digest_function": "reviewed_fq12_state_assisted_ro_pok_transcript_digest",
            "default_verifier": "verify_reviewed_fq12_state_assisted_ro_pok",
            "test": "state_assisted_ro_pok_boundary_is_byte_bound_and_fail_closed",
        },
        "rust_coverage": [
            "domain/profile constants are fixed at the artifact values",
            "canonical frame length is checked from public bytes",
            "statement-frame byte mutation changes transcript digest",
            "statement-frame length mutation changes transcript digest",
            "opening-commitment mutation changes transcript digest",
            "transcript-digest mutation changes transcript digest",
            "carrier mutation changes transcript digest",
            "wrong-domain frame changes transcript digest",
            "wrong-profile frame changes transcript digest",
            "digest-only carrier remains rejected by the default verifier",
            "all default verifier failures return the fixed hidden-source proof rejection reason",
        ],
    }
    return {
        "schema": "Descender.hidden_source_state_assisted_ro_pok_byte_negative_tests.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "ro_pok_byte_negative_tests_specified_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "ro_pok_assumption_api": rel(RO_POK_API_JSON),
            "semantic_negative_controls": rel(SEMANTIC_CONTROLS_JSON),
            "ro_pok_assumption_api_schema": api.get("schema"),
            "semantic_negative_controls_schema": semantic_controls.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "checked_core_signature_bytes": 252,
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "selected_candidate": "state_assisted_32_byte_random_oracle_proof_handle",
        },
        "base_frame": base_frame,
        "byte_negative_tests": byte_negative_tests,
        "test_count": len(byte_negative_tests),
        "executable_status": executable_status,
        "decision": {
            "can_accept_now": False,
            "reason": "byte-level RO-PoK binding and fixed-error controls now have a fail-closed Rust boundary, but the proof-handle assumption remains unaccepted and lacks an audited accepting verifier",
            "next_artifact": "state_assisted_ro_pok_accepting_verifier_or_algebraic_carrier_fallback",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    status = payload["executable_status"]
    lines = [
        "# Hidden-Source State-Assisted RO-PoK Byte Negative Tests",
        "",
        "This artifact converts the RO proof-handle semantic controls into",
        "serialized-frame mutations.  It remains fail-closed: these are required",
        "tests, not an accepting verifier.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected candidate: `{target['selected_candidate']}`",
        f"- checked-core signature bytes: `{target['checked_core_signature_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- test count: `{payload['test_count']}`",
        f"- RO-PoK API: `{payload['source_artifacts']['ro_pok_assumption_api']}`",
        f"- semantic controls: `{payload['source_artifacts']['semantic_negative_controls']}`",
        "",
        "## Base Frame",
        "",
        "| field | encoding/value |",
        "|---|---|",
    ]
    for field, value in payload["base_frame"].items():
        lines.append(f"| `{field}` | {value} |")
    lines.extend(
        [
            "",
            "## Byte Negative Tests",
            "",
            "| test | semantic control | mutation | must reject | accepts only if break |",
            "|---|---|---|---|---|",
        ]
    )
    for row in payload["byte_negative_tests"]:
        lines.append(
            f"| `{row['id']}` | `{row['semantic_control']}` | {row['mutation']} | "
            f"`{row['must_reject']}` | {row['accepts_only_if_break']} |"
        )
    lines.extend(
        [
            "",
            "## Executable Status",
            "",
            f"- current: `{status['current']}`",
            f"- reason: {status['reason']}",
            f"- promotion requirement: {status['promotion_requirement']}",
            f"- remaining gap: {status['remaining_gap']}",
            "",
            "## Rust Boundary",
            "",
            f"- crate: `{status['rust_boundary']['crate']}`",
            f"- module: `{status['rust_boundary']['module']}`",
            f"- public input type: `{status['rust_boundary']['public_input_type']}`",
            f"- frame function: `{status['rust_boundary']['frame_function']}`",
            f"- digest function: `{status['rust_boundary']['digest_function']}`",
            f"- default verifier: `{status['rust_boundary']['default_verifier']}`",
            f"- test: `{status['rust_boundary']['test']}`",
            "",
            "## Rust Coverage",
            "",
        ]
    )
    for coverage in status["rust_coverage"]:
        lines.append(f"- {coverage}")
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
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
