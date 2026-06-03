#!/usr/bin/env python3
"""Build a pre-third-party adversarial cryptoanalysis campaign.

This is intentionally not a deployment gate.  It is a reviewer-shaped attack
plan over the assumptions that remain outside Lean: what a cryptographer is
likely to try, what local evidence already exists, and what should be attacked
next before external review.
"""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "pre_third_party_cryptoanalysis_campaign_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_third_party_cryptoanalysis_campaign_n32_q12289.md"


def load_json(name: str) -> dict[str, Any]:
    path = READINESS_DIR / name
    if not path.exists():
        return {}
    return json.loads(path.read_text())


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def existing(path: Path) -> str:
    return "present" if path.exists() else "missing"


def campaign_row(
    campaign_id: str,
    assumption_class: str,
    reviewer_attack: str,
    current_local_coverage: str,
    local_evidence: list[Path],
    residual_gap: str,
    recommended_probe: str,
    priority: str,
) -> dict[str, Any]:
    return {
        "campaign_id": campaign_id,
        "assumption_class": assumption_class,
        "reviewer_attack": reviewer_attack,
        "current_local_coverage": current_local_coverage,
        "local_evidence": [rel(path) for path in local_evidence],
        "local_evidence_status": {rel(path): existing(path) for path in local_evidence},
        "residual_gap": residual_gap,
        "recommended_probe": recommended_probe,
        "priority": priority,
    }


def build_payload() -> dict[str, Any]:
    red_team = load_json("red_team_cryptoanalysis_n32_q12289.json")
    attack_matrix = load_json("assumption_attack_review_matrix_n32_q12289.json")
    keygen = load_json("keygen_security_assumption_surface_n32_q12289.json")
    qrom = load_json("qrom_security_loss_review_n32_q12289.json")
    pad = load_json("pad_sampler_boundary_review_n32_q12289.json")
    timing = load_json("production_reference_facade_timing_report_n32_q12289.json")
    preflight = load_json("production_preflight_certificate_verification_n32_q12289.json")

    campaigns = [
        campaign_row(
            "PTC-001",
            "matrix_sampler_and_public_key_distribution",
            (
                "Reconstruct the exact public matrix distribution independently, "
                "then run primal/dual SIS and lattice-estimator cost models."
            ),
            "local quotient/SIS lattice sanity harness exists; external estimator still open",
            [
                READINESS_DIR / "quotient_sis_lattice_attack_review_n32_q12289.json",
                READINESS_DIR / "matrix_keygen_estimator_memo_n32_q12289.json",
            ],
            (
                "The local harness can find obvious short relations, but it is "
                "not an independent estimator for the exact keygen distribution."
            ),
            (
                "Run a separate estimator implementation against independently "
                "rederived matrix samples, including parameter perturbations around q=12289 and n=32."
            ),
            "critical",
        ),
        campaign_row(
            "PTC-002",
            "bounded_response_link_kernel",
            (
                "Search for bounded kernel witnesses, bad coordinate projections, "
                "or near-bound vectors that survive the selected [0,3] response link."
            ),
            "exact selected-certificate replay plus local BKZ-23/bad-variant sweeps",
            [
                READINESS_DIR
                / "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json",
                READINESS_DIR
                / "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json",
                READINESS_DIR
                / "red_team_cryptoanalysis_n32_q12289.json",
            ],
            (
                "Selected bound-96 exclusion is strong locally, but reviewers may "
                "try independent enumeration, stronger reductions, and malformed certificate inputs."
            ),
            (
                "Replay the exact certificate with an independent implementation; "
                "run BKZ/block-size and random-basis perturbation sweeps as attack sanity, not proof."
            ),
            "critical",
        ),
        campaign_row(
            "PTC-003",
            "fiat_shamir_qrom_and_challenge_budget",
            (
                "Attack the Fiat-Shamir transform: domain confusion, query-budget "
                "understatement, proof-commitment rebinding, and multi-target losses."
            ),
            "byte-frame arithmetic checks pass; QROM model review remains external",
            [
                READINESS_DIR / "qrom_challenge_budget_memo_n32_q12289.json",
                READINESS_DIR / "qrom_security_loss_review_n32_q12289.json",
                READINESS_DIR / "conditional_soundness_budget_review_n32_q12289.json",
            ],
            (
                "Minimum deployment margin is narrow, and the QROM game hop is a "
                "model assumption rather than a local empirical property."
            ),
            (
                "Build a transcript-frame mutation and multi-target ledger probe "
                "that enumerates every accepted public byte field and confirms no verifier path can omit or reorder it."
            ),
            "high",
        ),
        campaign_row(
            "PTC-004",
            "pad_prf_and_sampler_independence",
            (
                "Distinguish pad residues, exploit rejection-sampling bias, or "
                "turn active equations into hidden representative leakage."
            ),
            "boundary memo and bounded PRF semantics exist; statistical/differential PRF probes not yet exhaustive",
            [
                READINESS_DIR / "pad_sampler_boundary_memo_n32_q12289.json",
                READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.json",
                READINESS_DIR / "bucket2_universal_pad_prf_tracker_n32_q12289.json",
            ],
            (
                "SHA-256 PRF security is external, but we can still locally test "
                "framing, counter exhaustion, residue bias, seed mutation, and forbidden-condition composition."
            ),
            (
                "Run large deterministic seed sweeps for residue uniformity, counter-exhaustion behavior, "
                "domain separation, one-bit seed/message/slot avalanche, and forbidden active-condition reuse."
            ),
            "high",
        ),
        campaign_row(
            "PTC-005",
            "transcript_binding_and_malleability",
            (
                "Swap slot/account/key/message/signature/proof fields across valid "
                "transcripts, or exploit parser ambiguity to replay acceptance."
            ),
            "negative controls exist for selected hidden-source/projection components",
            [
                READINESS_DIR
                / "hidden_source_state_assisted_negative_controls_n32_q12289.json",
                READINESS_DIR
                / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json",
                READINESS_DIR / "production_preflight_certificate_verification_n32_q12289.json",
            ],
            (
                "The local evidence is component-level; a whole-transcript mutation "
                "fuzzer should try reviewer-style field swaps across the facade boundary."
            ),
            (
                "Create a corpus from accepted reference vectors and mutate every length, domain, slot, key, "
                "message, account state, proof, and residue field with fail-closed assertions."
            ),
            "high",
        ),
        campaign_row(
            "PTC-006",
            "side_channel_and_fault_behavior",
            (
                "Use timing, retry, error-code, panic, or fault-injection behavior "
                "as a distinguisher for witness/pad/account state."
            ),
            "timing memo exists; full constant-time/fault campaign remains open",
            [
                READINESS_DIR / "reference_timing_report_n32_q12289.json",
                READINESS_DIR / "production_reference_facade_timing_report_n32_q12289.json",
                READINESS_DIR / "constant_time_side_channel_audit_memo_n32_q12289.json",
            ],
            (
                "The current reference is not a hardened production implementation; "
                "timing and fault evidence must be treated as attack surface, not assurance."
            ),
            (
                "Run differential timing distributions by secret class, rejection class, malformed input class, "
                "and injected parse/hash/PRF failures; require fail-closed uniformity."
            ),
            "high",
        ),
        campaign_row(
            "PTC-007",
            "state_lifecycle_and_replay",
            (
                "Exploit rollback, backup restore, chain reorg, mempool replay, "
                "slot reuse, or trusted-anchor placement to reuse signing state."
            ),
            "lifecycle and blockchain integration memos exist; adversarial simulation remains open",
            [
                READINESS_DIR / "production_lifecycle_hardening_memo_n32_q12289.json",
                READINESS_DIR / "blockchain_deployment_integration_memo_n32_q12289.json",
            ],
            (
                "This is cryptographic in effect even if engineering-shaped: state "
                "reuse can invalidate the security model."
            ),
            (
                "Build a state-machine adversary simulator for reorg/rollback/restore/slot-reuse cases, "
                "then assert no accepted transcript can reuse forbidden state."
            ),
            "high",
        ),
        campaign_row(
            "PTC-008",
            "rogue_key_and_account_binding",
            (
                "Choose malicious public keys, account states, or commitments that "
                "make extraction/binding assumptions vacuous or cross-bind users."
            ),
            "account/frame binding reports exist, but rogue-key search is not exhaustive",
            [
                READINESS_DIR
                / "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.json",
                READINESS_DIR
                / "hidden_source_fallback_projection_account_state_opening_commitment_wiring_n32_q12289.json",
                READINESS_DIR
                / "hidden_source_fallback_projection_expanded_opening_commitment_binding_n32_q12289.json",
            ],
            (
                "A reviewer may try degenerate account states, duplicate commitments, "
                "cross-user replay, and public-key substitution rather than lattice attacks."
            ),
            (
                "Generate degenerate and adversarial account/key/commitment fixtures and assert extractor-or-binding-break classification "
                "for every accepted mutation."
            ),
            "medium",
        ),
    ]

    campaigns_with_any_local_evidence = [
        row
        for row in campaigns
        if any(status == "present" for status in row["local_evidence_status"].values())
    ]
    high_priority_open = [
        row
        for row in campaigns
        if row["priority"] in {"critical", "high"}
    ]
    return {
        "schema": "Descender.pre_third_party_cryptoanalysis_campaign.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "scope": "n32_q12289 account-carried committed-sigma signature readiness",
        "non_goal": (
            "This does not claim to exhaust every conceivable attack.  It turns "
            "the remaining assumptions into concrete reviewer-style attack campaigns."
        ),
        "source_status": {
            "red_team_deployment_ready": red_team.get("verdict", {}).get(
                "deployment_ready"
            ),
            "red_team_new_local_break_found": red_team.get("verdict", {}).get(
                "new_break_against_selected_250b_target_found"
            ),
            "assumptions_mapped": attack_matrix.get("summary", {}).get("assumptions_mapped"),
            "keygen_external_review_items_open": keygen.get("summary", {}).get(
                "open_external_items"
            ),
            "qrom_status": qrom.get("status"),
            "qrom_minimum_deployment_margin_bits": qrom.get("derived_terms", {}).get(
                "deployment_min_margin_bits"
            ),
            "pad_boundary_status": pad.get("status"),
            "timing_iterations": timing.get("timing_report", {}).get("iterations"),
            "preflight_status": preflight.get("status"),
            "preflight_blocker_count": preflight.get("blocker_count"),
        },
        "campaign_count": len(campaigns),
        "campaigns_with_any_local_evidence_count": len(campaigns_with_any_local_evidence),
        "high_priority_open_campaign_count": len(high_priority_open),
        "campaigns": campaigns,
        "recommended_order": [
            "PTC-004",
            "PTC-005",
            "PTC-001",
            "PTC-003",
            "PTC-006",
            "PTC-007",
            "PTC-008",
            "PTC-002",
        ],
        "bottom_line": (
            "The best next local value is not more proof packaging; it is targeted "
            "adversarial testing of pad/PRF statistics, whole-transcript malleability, "
            "independent matrix/lattice estimates, QROM frame multi-target accounting, "
            "side-channel/fault behavior, lifecycle replay, and rogue-key/account binding."
        ),
    }


def write_markdown(payload: dict[str, Any], path: Path) -> None:
    lines = [
        "# Pre-Third-Party Cryptoanalysis Campaign",
        "",
        "This is an adversarial pre-review plan for assumptions outside Lean.",
        "",
        "## Summary",
        "",
        f"- campaigns: `{payload['campaign_count']}`",
        f"- campaigns with local evidence: `{payload['campaigns_with_any_local_evidence_count']}`",
        f"- high-priority open campaigns: `{payload['high_priority_open_campaign_count']}`",
        f"- bottom line: {payload['bottom_line']}",
        "",
        "## Campaigns",
        "",
        "| id | priority | assumption | local coverage | residual gap | recommended probe |",
        "|---|---|---|---|---|---|",
    ]
    for row in payload["campaigns"]:
        lines.append(
            "| {campaign_id} | {priority} | {assumption_class} | {current_local_coverage} | {residual_gap} | {recommended_probe} |".format(
                **row
            )
        )
    lines.extend(
        [
            "",
            "## Recommended Order",
            "",
            ", ".join(f"`{item}`" for item in payload["recommended_order"]),
            "",
            "## Non-Goal",
            "",
            payload["non_goal"],
            "",
        ]
    )
    path.write_text("\n".join(lines))


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    write_markdown(payload, DEFAULT_MD)
    print(json.dumps({
        "json": rel(DEFAULT_JSON),
        "markdown": rel(DEFAULT_MD),
        "campaign_count": payload["campaign_count"],
        "high_priority_open_campaign_count": payload["high_priority_open_campaign_count"],
        "status": "written",
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
