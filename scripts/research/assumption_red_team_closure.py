#!/usr/bin/env python3
"""Generate a fail-closed red-team closure ledger for reviewed assumptions."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "assumption_red_team_closure_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "assumption_red_team_closure_n32_q12289.md"

KEYGEN_SURFACE = READINESS_DIR / "keygen_security_assumption_surface_n32_q12289.json"
ATTACK_MATRIX = READINESS_DIR / "assumption_attack_review_matrix_n32_q12289.json"
RED_TEAM = READINESS_DIR / "red_team_cryptoanalysis_n32_q12289.json"
CONDITIONAL_BUDGET = (
    READINESS_DIR / "conditional_soundness_budget_review_n32_q12289.json"
)
PRODUCTION_GAP = READINESS_DIR / "production_reference_gap_report_n32_q12289.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def by_id(rows: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    return {row["assumption_id"]: row for row in rows}


def selected_red_team(red_team: dict[str, Any]) -> dict[str, Any]:
    pair = (
        red_team.get("coordinate_projection_sweeps", {})
        .get("two_coordinate_bkz23", {})
        .get("selected_coords_row", {})
    )
    candidate = pair.get("candidate") or {}
    return {
        "selected_coords": pair.get("coords"),
        "selected_candidate_coeff_bound": candidate.get("coeff_bound"),
        "selected_candidate_inside_bound": candidate.get("inside_bound"),
        "accepted_bound": red_team.get("target", {}).get(
            "response_difference_bound"
        ),
        "bad_alternate_two_coordinate_codes_found": red_team.get("verdict", {}).get(
            "bad_alternate_two_coordinate_codes_found"
        ),
        "exact_bad_variant_witnesses": len(
            red_team.get("exact_bad_variant_witnesses", [])
        ),
    }


def closure_row(
    assumption: dict[str, Any],
    attack: dict[str, Any],
    *,
    closure_state: str,
    deployment_blocking: bool,
    kill_strategy: str,
    attacks_attempted: list[str],
    local_evidence: dict[str, Any],
    promotion_criteria: list[str],
    residual_risk: str,
) -> dict[str, Any]:
    return {
        "assumption_id": assumption["assumption_id"],
        "statement": assumption.get("statement"),
        "local_status": assumption.get("local_status"),
        "external_status": assumption.get("external_status"),
        "attack_review_class": attack.get("review_class"),
        "attack_review_status": attack.get("local_review_status"),
        "closure_state": closure_state,
        "deployment_blocking": deployment_blocking,
        "kill_strategy": kill_strategy,
        "attacks_attempted": attacks_attempted,
        "local_evidence": local_evidence,
        "evidence_artifacts": sorted(
            set(assumption.get("artifacts", []) + attack.get("executable_evidence", []))
        ),
        "evidence_artifact_status": {
            **assumption.get("artifact_status", {}),
            **attack.get("executable_evidence_status", {}),
        },
        "promotion_criteria": promotion_criteria,
        "residual_risk": residual_risk,
    }


def build_rows(
    surface: dict[str, Any],
    matrix: dict[str, Any],
    red_team: dict[str, Any],
    budget: dict[str, Any],
    production: dict[str, Any],
) -> list[dict[str, Any]]:
    assumptions = by_id(surface["assumptions"])
    attacks = by_id(matrix["rows"])
    selected = selected_red_team(red_team)
    preflight = production.get("deployment_preflight", {})

    return [
        closure_row(
            assumptions["matrix_sampler_and_public_key_distribution"],
            attacks["matrix_sampler_and_public_key_distribution"],
            closure_state="locally_supported_external_review_open",
            deployment_blocking=True,
            kill_strategy=(
                "Find a short exploitable q-ary relation, reconstruction mismatch, "
                "sampler bias, or external SIS estimate below the target security level."
            ),
            attacks_attempted=[
                "checked exact matrix transcript hash and rank/kernel dimension",
                "ran local quotient/SIS reduction harness",
                "probed sparse signed zero relations",
                "confirmed the bare short relation is caught by the selected response link",
            ],
            local_evidence={
                **attacks["matrix_sampler_and_public_key_distribution"].get(
                    "evidence", {}
                ),
                "red_team_bare_quotient_confirmed_broken": red_team.get(
                    "verdict", {}
                ).get("bare_public_quotient_confirmed_broken"),
            },
            promotion_criteria=[
                "independent matrix derivation from the public specification",
                "independent SIS/lattice-estimator run for the exact distribution",
                "reviewed rejection-sampling bias bounds and transcript domain separation",
            ],
            residual_risk=(
                "Local harnesses catch obvious relations but do not replace an "
                "independent estimator-grade keygen distribution review."
            ),
        ),
        closure_row(
            assumptions["bare_quotient_kernel_control"],
            attacks["bare_quotient_kernel_control"],
            closure_state="closed_negative_control",
            deployment_blocking=False,
            kill_strategy=(
                "Demonstrate that the bare public quotient relation admits an accepted "
                "bounded kernel vector; this should remain a regression test, not a live path."
            ),
            attacks_attempted=[
                "ran the quotient lattice reduction negative control",
                "verified the found vector is inside the response-difference bound",
                "verified the selected response-link coordinates reject it",
            ],
            local_evidence=attacks["bare_quotient_kernel_control"].get(
                "evidence", {}
            ),
            promotion_criteria=[
                "keep production verifiers from exposing a bare-quotient acceptance path",
                "keep this negative control wired into review replay",
            ],
            residual_risk=(
                "This assumption is closed only as a rejected construction. "
                "Using it directly would be a critical regression."
            ),
        ),
        closure_row(
            assumptions["bounded_response_link_kernel"],
            attacks["bounded_response_link_kernel"],
            closure_state="locally_supported_external_review_open",
            deployment_blocking=True,
            kill_strategy=(
                "Find a nonzero vector in the exact quotient kernel with coordinates "
                "[0,3] zero modulo q and all coefficients within [-96,96], or find "
                "a replay/canonical-basis mismatch in the certificate."
            ),
            attacks_attempted=[
                "replayed the exact bounded-kernel certificate",
                "ran the selected-coordinate local lattice attack review",
                "ran BKZ-23 sweeps over zero-, one-, and two-coordinate projections",
                "validated exact bad witnesses for nearby broken coordinate pairs",
            ],
            local_evidence={
                **attacks["bounded_response_link_kernel"].get("evidence", {}),
                **selected,
            },
            promotion_criteria=[
                "independent replay of the exact manifest and verifier",
                "independent derivation of the canonical intersection basis",
                "independent SVP/BKZ sanity run against the selected [0,3] relation",
            ],
            residual_risk=(
                "This is the strongest local artifact, but the red-team sweep also "
                "shows nearby coordinate choices fail; the selected relation needs "
                "independent replay before deployment."
            ),
        ),
        closure_row(
            assumptions["fiat_shamir_qrom_and_challenge_budget"],
            attacks["fiat_shamir_qrom_and_challenge_budget"],
            closure_state="locally_supported_external_review_open",
            deployment_blocking=True,
            kill_strategy=(
                "Invalidate the Fiat-Shamir game hop, find a transcript/domain "
                "ambiguity, exceed the multi-query ledger, or show the QROM loss "
                "drops a checked term below the target."
            ),
            attacks_attempted=[
                "checked byte-level challenge frame and challenge length",
                "checked local QROM/security-loss arithmetic",
                "checked conditional soundness budget terms",
                "kept the model fail-closed with explicit external QROM review open",
            ],
            local_evidence={
                **attacks["fiat_shamir_qrom_and_challenge_budget"].get(
                    "evidence", {}
                ),
                "conditional_budget_status": budget.get("status"),
                "deployment_ready": budget.get("deployment_ready"),
            },
            promotion_criteria=[
                "external QROM/Forking game-hop review",
                "reviewed blockchain adversary query ledger and multi-target interpretation",
                "reviewed byte transcript binding with no alternate encodings",
            ],
            residual_risk=(
                "The arithmetic passes locally with narrow minimum margin; the "
                "cryptographic model and query envelope remain judgment calls for review."
            ),
        ),
        closure_row(
            assumptions["pad_prf_and_sampler_independence"],
            attacks["pad_prf_and_sampler_independence"],
            closure_state="locally_supported_external_review_open",
            deployment_blocking=True,
            kill_strategy=(
                "Find pad-domain collision, SHA-256 PRF/RO misuse, secret-dependent "
                "retry shape, sampler leakage, or an active equation that conditions "
                "the hidden representative outside the reviewed boundary."
            ),
            attacks_attempted=[
                "checked pad/sampler boundary conditions",
                "checked forbidden active pad/local-opening conditions",
                "checked no retry loop is required for the reviewed sampler",
                "kept SHA-256 PRF and production sampler review explicit",
            ],
            local_evidence=attacks["pad_prf_and_sampler_independence"].get(
                "evidence", {}
            ),
            promotion_criteria=[
                "independent SHA-256 pad PRF/random-oracle review",
                "audit final sampler code against the boundary object",
                "constant-time review of sampler and secret-dependent memory accesses",
            ],
            residual_risk=(
                "The boundary is checkable, but pseudorandomness and final sampler "
                "implementation behavior remain assumption-level."
            ),
        ),
        closure_row(
            assumptions["production_keygen_and_state_lifecycle"],
            attacks["production_keygen_and_state_lifecycle"],
            closure_state="open_implementation_blocker",
            deployment_blocking=True,
            kill_strategy=(
                "Find mismatch between reviewed proof and executable verifier, "
                "state rollback/replay acceptance, constant-time leak, or unsafe "
                "keygen/signing lifecycle behavior."
            ),
            attacks_attempted=[
                "generated fail-closed production gap report",
                "generated lifecycle hardening and blockchain integration memos",
                "generated constant-time audit memo",
                "recorded that the executable still wraps the legacy 254-byte path",
            ],
            local_evidence={
                **attacks["production_keygen_and_state_lifecycle"].get(
                    "evidence", {}
                ),
                "deployment_ready": preflight.get("deployment_ready"),
                "blockers": preflight.get("blockers", []),
            },
            promotion_criteria=[
                "implement accepting 308-byte/Fq12 projection-carrier fallback semantics in Rust",
                "audit constant-time signer, sampler, keygen, verifier, and secret handling",
                "harden storage, restore anchor, reorg, and mempool replay semantics",
                "run KAT, codec, fuzz/property, and consensus integration tests",
            ],
            residual_risk=(
                "This is a hard deployment blocker: the executable implementation "
                "does not yet match the reviewed proof path."
            ),
        ),
    ]


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Assumption Red-Team Closure Ledger",
        "",
        "This generated ledger is fail-closed. It records what would break each",
        "assumption, what local attacks or checks have actually been run, and what",
        "would be required before the row can be treated as deployable.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- deployment ready: `{payload['summary']['deployment_ready']}`",
        f"- assumptions tracked: `{payload['summary']['assumptions_tracked']}`",
        f"- deployment-blocking open rows: `{payload['summary']['deployment_blocking_open_rows']}`",
        f"- closed negative controls: `{payload['summary']['closed_negative_control_rows']}`",
        f"- locally supported, external review open: `{payload['summary']['locally_supported_external_review_open_rows']}`",
        f"- implementation blockers: `{payload['summary']['open_implementation_blocker_rows']}`",
        f"- missing evidence artifacts: `{payload['summary']['missing_evidence_artifacts']}`",
        "",
        "## Closure Rows",
        "",
        "| assumption | closure state | blocking | kill strategy | promotion criteria |",
        "|---|---|:---:|---|---|",
    ]
    for row in payload["rows"]:
        criteria = "<br>".join(row["promotion_criteria"])
        lines.append(
            f"| `{row['assumption_id']}` | `{row['closure_state']}` | "
            f"`{row['deployment_blocking']}` | {row['kill_strategy']} | {criteria} |"
        )
    lines.extend(["", "## Local Attacks And Residual Risk", ""])
    for row in payload["rows"]:
        attacks = "; ".join(row["attacks_attempted"])
        lines.append(
            f"- `{row['assumption_id']}`: attacks/checks: {attacks}. "
            f"Residual risk: {row['residual_risk']}"
        )
    lines.append("")
    return "\n".join(lines)


def build_payload() -> dict[str, Any]:
    surface = load_json(KEYGEN_SURFACE)
    matrix = load_json(ATTACK_MATRIX)
    red_team = load_json(RED_TEAM)
    budget = load_json(CONDITIONAL_BUDGET)
    production = load_json(PRODUCTION_GAP)
    rows = build_rows(surface, matrix, red_team, budget, production)
    missing_artifacts = sorted(
        {
            path
            for row in rows
            for path, status in row["evidence_artifact_status"].items()
            if status != "present"
        }
    )
    state_counts: dict[str, int] = {}
    for row in rows:
        state = row["closure_state"]
        state_counts[state] = state_counts.get(state, 0) + 1
    deployment_blocking_open = sum(
        1
        for row in rows
        if row["deployment_blocking"] and row["closure_state"] != "closed_for_deployment"
    )
    return {
        "schema": "Descender.assumption_red_team_closure.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source_artifacts": [
            rel(KEYGEN_SURFACE),
            rel(ATTACK_MATRIX),
            rel(RED_TEAM),
            rel(CONDITIONAL_BUDGET),
            rel(PRODUCTION_GAP),
        ],
        "target": {
            "n": 32,
            "q": 12289,
            "transaction_bytes": 250,
            "account_link_bytes": 68,
            "public_key_bytes": 172,
        },
        "summary": {
            "deployment_ready": False,
            "assumptions_tracked": len(rows),
            "state_counts": state_counts,
            "closed_negative_control_rows": state_counts.get(
                "closed_negative_control", 0
            ),
            "locally_supported_external_review_open_rows": state_counts.get(
                "locally_supported_external_review_open", 0
            ),
            "open_implementation_blocker_rows": state_counts.get(
                "open_implementation_blocker", 0
            ),
            "deployment_blocking_open_rows": deployment_blocking_open,
            "missing_evidence_artifacts": len(missing_artifacts),
            "missing_evidence_artifact_paths": missing_artifacts,
            "selected_target_break_found": red_team.get("verdict", {}).get(
                "new_break_against_selected_250b_target_found"
            ),
        },
        "rows": rows,
    }


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))
    if payload["summary"]["missing_evidence_artifacts"]:
        raise SystemExit(1)
    if payload["summary"]["selected_target_break_found"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
