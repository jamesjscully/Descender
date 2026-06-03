#!/usr/bin/env python3
"""Generate the leakage simulator contract for the 32-byte RO-PoK route."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "hidden_source_ro_pok_leakage_simulator_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "hidden_source_ro_pok_leakage_simulator_n32_q12289.md"
COMPOSITION_JSON = (
    READINESS_DIR / "hidden_source_ro_pok_leakage_global_composition_theorem_n32_q12289.json"
)
OPENING_FRAME_JSON = READINESS_DIR / "hidden_source_state_assisted_opening_frame_n32_q12289.json"
RO_POK_API_JSON = READINESS_DIR / "hidden_source_state_assisted_ro_pok_assumption_api_n32_q12289.json"
NEGATIVE_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    composition = load_json(COMPOSITION_JSON)
    opening = load_json(OPENING_FRAME_JSON)
    api = load_json(RO_POK_API_JSON)
    negative = load_json(NEGATIVE_JSON)
    public_inputs = [
        {
            "id": "strict_envelope_bytes",
            "source": "signature",
            "simulator_access": "public",
            "purpose": "reproduce parse success/failure and checked-core byte layout",
        },
        {
            "id": "canonical_statement_frame",
            "source": "verifier recomputation",
            "simulator_access": "public deterministic",
            "purpose": "derive the same public statement digest without hidden openings",
        },
        {
            "id": "account_state_public_view",
            "source": "blockchain account state",
            "simulator_access": "public",
            "purpose": "reproduce slot id, commitment digest, restore anchor, and consumed-state checks",
        },
        {
            "id": "checked_core_response_body",
            "source": "252-byte Fq12 checked-core signature body",
            "simulator_access": "public",
            "purpose": "defer checked-core leakage to the existing checked-core/QROM assumptions",
        },
    ]
    hidden_values = [
        "account_witness_opening",
        "pad_seed_opening",
        "carrier_handle_opening",
        "signer_randomness",
        "unused_slot_secret_state",
    ]
    simulator_steps = [
        {
            "step": "parse_public_view",
            "input": ["strict_envelope_bytes", "account_state_public_view"],
            "output": ["parse_status", "public_slot_view"],
            "hidden_value_access": False,
        },
        {
            "step": "recompute_statement",
            "input": ["canonical_statement_frame"],
            "output": ["statement_digest", "opening_commitment_digest", "transcript_digest"],
            "hidden_value_access": False,
        },
        {
            "step": "simulate_checked_core",
            "input": ["checked_core_response_body", "statement_digest"],
            "output": ["checked_core_public_leakage"],
            "hidden_value_access": False,
        },
        {
            "step": "sample_oracle_handle",
            "input": ["statement_digest", "opening_commitment_digest", "RO-PoK simulation oracle"],
            "output": ["32_byte_carrier_handle"],
            "hidden_value_access": False,
        },
        {
            "step": "uniform_reject_projection",
            "input": ["parse_status", "byte_negative_control_class"],
            "output": ["accept_or_uniform_reject_bit"],
            "hidden_value_access": False,
        },
    ]
    indistinguishability_claims = [
        {
            "id": "carrier_handle_distribution",
            "claim": "the simulated 32-byte carrier is indistinguishable from the real handle for any verifier-visible public statement",
            "depends_on": "RO-PoK zero-knowledge/leakage-bounded simulation assumption",
            "failure_break": "leakage_simulation_break",
        },
        {
            "id": "public_statement_distribution",
            "claim": "the simulated statement frame equals the verifier-recomputed public statement byte for byte",
            "depends_on": "deterministic transcript binding and opening-frame schema",
            "failure_break": "transcript_binding_break",
        },
        {
            "id": "reject_behavior_distribution",
            "claim": "all failed paths expose only a uniform reject result in production",
            "depends_on": "constant-time uniform reject implementation and diagnostic stripping",
            "failure_break": "constant_time_uniform_reject_break",
        },
        {
            "id": "checked_core_composition",
            "claim": "checked-core response leakage is exactly the existing checked-core public leakage and is not amplified by the hidden-source carrier",
            "depends_on": "checked-core leakage and QROM composition theorem",
            "failure_break": "checked_core_or_response_link_break",
        },
    ]
    required_tests = [
        {
            "id": "public_frame_reconstruction",
            "status": "specified_not_executed",
            "test": "serialize public view twice, once from structured inputs and once from byte parser, and compare transcript digests",
        },
        {
            "id": "handle_context_swap",
            "status": negative.get("executable_status", {}).get("current"),
            "test": "swap message, slot id, public-key digest, account-state digest, or pad statement digest and require rejection",
        },
        {
            "id": "uniform_reject_api",
            "status": "open",
            "test": "production verifier returns one reject code and no secret-correlated diagnostics for every failed simulator step",
        },
        {
            "id": "oracle_handle_simulation_hook",
            "status": "open",
            "test": "accepting-model harness can replace real RO-PoK handle generation with simulator oracle output",
        },
    ]
    open_gaps = [
        {
            "id": "ro_pok_simulation_oracle",
            "why_open": "the 32-byte handle has no concrete simulator without accepting the nonstandard RO-PoK assumption",
            "close_by": "external review accepts the simulation clause, or switch to the 308-byte projection fallback",
        },
        {
            "id": "uniform_reject_constant_time",
            "why_open": "the current reference boundary is fail-closed and not a production accepting verifier",
            "close_by": "implement accepting verifier path and audit all reject paths for uniform output and timing",
        },
        {
            "id": "checked_core_leakage_composition",
            "why_open": "the simulator delegates 252 bytes of checked-core response leakage to existing assumptions",
            "close_by": "global theorem composes checked-core leakage with hidden-source handle leakage without double counting",
        },
    ]
    return {
        "schema": "Descender.hidden_source_ro_pok_leakage_simulator.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "ro_pok_leakage_simulator_contract_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "composition_theorem": rel(COMPOSITION_JSON),
            "composition_theorem_schema": composition.get("schema"),
            "opening_frame": rel(OPENING_FRAME_JSON),
            "opening_frame_schema": opening.get("schema"),
            "ro_pok_api": rel(RO_POK_API_JSON),
            "ro_pok_api_schema": api.get("schema"),
            "negative_controls": rel(NEGATIVE_JSON),
            "negative_controls_schema": negative.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "selected_branch": "fail-closed `284 B/Fq12` account-carried hidden-source envelope",
            "carrier_bytes": 32,
            "checked_core_signature_bytes": 252,
            "envelope_bytes": 284,
        },
        "public_inputs": public_inputs,
        "forbidden_hidden_values": hidden_values,
        "simulator_steps": simulator_steps,
        "indistinguishability_claims": indistinguishability_claims,
        "required_tests": required_tests,
        "open_gaps": open_gaps,
        "decision": {
            "can_accept_now": False,
            "selected_now": "32_byte_ro_pok_fail_closed",
            "fallback_selected_now": False,
            "reason": "the simulator interface is explicit, but the RO-PoK simulation oracle, uniform reject implementation, and checked-core leakage composition remain open",
            "next_artifact": "hidden_source_ro_pok_accepting_model_controls_or_fallback_projection_certificate",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Hidden-Source RO-PoK Leakage Simulator",
        "",
        "This artifact specifies the leakage simulator contract for the selected",
        "32-byte RO-PoK route.  It remains fail-closed because the simulator still",
        "depends on the nonstandard RO-PoK simulation oracle and on a future",
        "uniform-reject accepting verifier implementation.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {payload['target']['selected_branch']}",
        f"- carrier bytes: `{payload['target']['carrier_bytes']}`",
        f"- envelope bytes: `{payload['target']['envelope_bytes']}`",
        "",
        "## Public Inputs",
        "",
        "| id | source | simulator access | purpose |",
        "|---|---|---|---|",
    ]
    for row in payload["public_inputs"]:
        lines.append(
            f"| `{row['id']}` | {row['source']} | `{row['simulator_access']}` | {row['purpose']} |"
        )
    lines.extend(["", "## Forbidden Hidden Values", ""])
    lines.extend(f"- `{item}`" for item in payload["forbidden_hidden_values"])
    lines.extend(
        [
            "",
            "## Simulator Steps",
            "",
            "| step | input | output | hidden access |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["simulator_steps"]:
        lines.append(
            f"| `{row['step']}` | {', '.join(f'`{x}`' for x in row['input'])} | "
            f"{', '.join(f'`{x}`' for x in row['output'])} | `{row['hidden_value_access']}` |"
        )
    lines.extend(
        [
            "",
            "## Indistinguishability Claims",
            "",
            "| id | claim | depends on | failure break |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["indistinguishability_claims"]:
        lines.append(
            f"| `{row['id']}` | {row['claim']} | {row['depends_on']} | `{row['failure_break']}` |"
        )
    lines.extend(
        [
            "",
            "## Required Tests",
            "",
            "| id | status | test |",
            "|---|---|---|",
        ]
    )
    for row in payload["required_tests"]:
        lines.append(f"| `{row['id']}` | `{row['status']}` | {row['test']} |")
    lines.extend(
        [
            "",
            "## Open Gaps",
            "",
            "| id | why open | close by |",
            "|---|---|---|",
        ]
    )
    for row in payload["open_gaps"]:
        lines.append(f"| `{row['id']}` | {row['why_open']} | {row['close_by']} |")
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- selected now: `{payload['decision']['selected_now']}`",
            f"- fallback selected now: `{payload['decision']['fallback_selected_now']}`",
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
