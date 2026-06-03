#!/usr/bin/env python3
"""Generate the projection-mask PRF leakage simulator contract."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_leakage_simulator_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "pre_signature_projection_mask_prf_leakage_simulator_n32_q12289.md"
)
ASSUMPTION_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_assumption_api_n32_q12289.json"
)
LEAKAGE_JSON = READINESS_DIR / "pre_signature_projection_leakage_probe_n32_q12289.json"
STATISTICAL_JSON = (
    READINESS_DIR / "pre_signature_projection_statistical_probe_n32_q12289.json"
)
INDEPENDENCE_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_independence_probe_n32_q12289.json"
)
FRAME_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_frame_disjointness_n32_q12289.json"
)
REUSE_JSON = (
    READINESS_DIR / "pre_signature_projection_pad_seed_reuse_surface_n32_q12289.json"
)
LIFECYCLE_JSON = READINESS_DIR / "state_lifecycle_closure_n32_q12289.json"
RO_REDUCTION_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_ro_reduction_n32_q12289.json"
)
SAME_GAME_JSON = (
    READINESS_DIR / "pre_signature_projection_lifecycle_same_game_lift_n32_q12289.json"
)
SPEND_BUDGET_JSON = (
    READINESS_DIR / "pre_signature_projection_spend_time_leakage_budget_n32_q12289.json"
)
REJECT_EQUIVALENCE_JSON = (
    READINESS_DIR / "pre_signature_projection_reject_equivalence_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    assumption = load_json(ASSUMPTION_JSON)
    leakage = load_json(LEAKAGE_JSON)
    statistical = load_json(STATISTICAL_JSON)
    independence = load_json(INDEPENDENCE_JSON)
    frame = load_json(FRAME_JSON)
    reuse = load_json(REUSE_JSON)
    lifecycle = load_json(LIFECYCLE_JSON)
    ro_reduction = load_json(RO_REDUCTION_JSON)
    same_game = load_json(SAME_GAME_JSON)
    spend_budget = load_json(SPEND_BUDGET_JSON)
    reject_equivalence = load_json(REJECT_EQUIVALENCE_JSON)

    real_world = {
        "id": "real_live_slot_account_state",
        "phase": "pre_spend_live_slot",
        "visible_values": [
            "public account-state slot metadata",
            "32-byte packed witness commitment",
            "12 checked-core packed residues inside the committed witness state",
            "six masked projection residues raw_projection_i + mask_i mod 12289",
            "opening commitment digest and restore anchor digest",
            "public key, message namespace, and transcript binding domains",
        ],
        "hidden_values": [
            "six raw selected account projection residues",
            "pad seed before valid slot opening",
            "projection-mask PRF lane outputs before opening",
            "account witness opening preimage",
        ],
    }
    ideal_world = {
        "id": "ideal_live_slot_account_state",
        "phase": "pre_spend_live_slot",
        "replacement": (
            "replace the six masked projection residues inside the packed witness "
            "field with six independent uniform residues in Z_q while preserving "
            "checked-core residues and every public field not deterministically "
            "derived from those six packed residue bytes"
        ),
        "requires": [
            "projection-mask PRF outputs are indistinguishable from uniform residues",
            "mask domain is separated from every pad PRF and transcript domain",
            "pad_seed, slot_id, and packed witness bytes are fixed before challenge",
            "no accepted lifecycle path reuses or rewrites a live pad seed",
        ],
    }
    spend_opening_world = {
        "id": "valid_spend_opening_view",
        "phase": "spend_or_verification",
        "modeled_disclosure": (
            "a valid opening reveals enough pad-seed material to unmask and bind "
            "the six selected account projection residues; this is charged as "
            "intentional spend-time opening leakage, not live-slot pre-signature leakage"
        ),
        "simulator_rule": (
            "condition on the opened slot and expose the packed-witness opening, "
            "pad-seed digest, selected projection opening frame, and carrier lanes "
            "only through the accepting certificate interface"
        ),
    }
    simulator_steps = [
        {
            "step": "parse_live_slot_view",
            "input": ["account_state_slot", "packed_witness_commitment"],
            "output": ["checked_core_residue_commitment", "masked_projection_residue_slots"],
            "uses_secret": False,
        },
        {
            "step": "hybrid_replace_mask_stream",
            "input": ["projection_mask_domain", "pad_seed_digest", "slot_id", "lane_ids"],
            "output": ["six_uniform_q_residue_masks"],
            "uses_secret": False,
            "charged_to": "Adv_mask_prf_projection_domain",
        },
        {
            "step": "hybrid_replace_masked_projection_residues",
            "input": ["six_uniform_q_residue_masks", "hidden_raw_projection_residue_distribution"],
            "output": ["six_uniform_looking_masked_projection_residues"],
            "uses_secret": False,
            "charged_to": "perfect_one_time_pad_over_Zq_given_uniform_masks",
        },
        {
            "step": "condition_valid_spend_opening",
            "input": ["accepted_slot_opening", "pad_seed", "packed_witness_opening"],
            "output": ["selected_projection_opening_leakage_record"],
            "uses_secret": False,
            "charged_to": "spend_time_projection_opening_budget",
        },
        {
            "step": "export_named_break",
            "input": ["distinguisher_output", "lifecycle_trace", "domain_trace"],
            "output": ["one_named_break_or_simulated_view"],
            "uses_secret": False,
        },
    ]
    named_breaks = [
        {
            "id": "mask_prf_distinguishing_break",
            "event": "adversary distinguishes projection-mask residues from independent uniform q12289 residues",
            "advantage_term": "Adv_mask_prf_projection_domain(q_mask, t)",
        },
        {
            "id": "mask_pad_prf_domain_collision_break",
            "event": "projection-mask frame collides with pad PRF, transcript, or opening-frame domain inputs",
            "advantage_term": "Adv_ro_domain_collision(q_hash)",
        },
        {
            "id": "pad_seed_lifecycle_reuse_or_rewrite_break",
            "event": "accepted lifecycle trace reuses, rewrites, or postchallenge-selects a pad seed or slot id",
            "advantage_term": "Adv_lifecycle_single_use(q_slots, q_reorg)",
        },
        {
            "id": "projection_leakage_budget_break",
            "event": "adversary obtains selected projection information outside valid spend-opening certificate disclosure",
            "advantage_term": "Adv_projection_opening_leakage(q_open)",
        },
    ]
    advantage_bound = [
        "Adv_live_slot_projection_leakage <= Adv_mask_prf_projection_domain(q_mask, t)",
        "  + Adv_ro_domain_collision(q_hash)",
        "  + Adv_lifecycle_single_use(q_slots, q_reorg)",
        "  + Adv_projection_opening_leakage(q_open)",
        "  + negligible_rejection_mismatch",
    ]
    evidence = [
        {
            "artifact": rel(ASSUMPTION_JSON),
            "status": assumption.get("status"),
            "role": "states the exact mask PRF assumption consumed by the simulator",
        },
        {
            "artifact": rel(INDEPENDENCE_JSON),
            "status": independence.get("status"),
            "role": (
                "bounded pairwise evidence: max centered Pearson "
                f"{independence['summary']['max_abs_pearson_centered']:.6f}, "
                "not theorem credit"
            ),
        },
        {
            "artifact": rel(FRAME_JSON),
            "status": frame.get("status"),
            "role": (
                "byte-frame disjointness: pad PRF input length "
                f"{frame['pad_prf_input_len']}, mask PRF input length "
                f"{frame['mask_prf_input_len']}"
            ),
        },
        {
            "artifact": rel(STATISTICAL_JSON),
            "status": statistical.get("status"),
            "role": "bounded marginal probe; confirms why unmasked residues cannot be public",
        },
        {
            "artifact": rel(LEAKAGE_JSON),
            "status": leakage.get("status"),
            "role": "bounded linear leakage probe for 18-row packed state",
        },
        {
            "artifact": rel(REUSE_JSON),
            "status": reuse.get("status"),
            "role": "documents pad-seed reuse/replay surface that the lifecycle term must close",
        },
        {
            "artifact": rel(LIFECYCLE_JSON),
            "status": lifecycle.get("closure_status"),
            "role": "policy-level lifecycle closure input; still must be lifted into the same game",
        },
        {
            "artifact": rel(RO_REDUCTION_JSON),
            "status": ro_reduction.get("status"),
            "role": "fail-closed RO/PRF reduction contract for the mask stream",
        },
        {
            "artifact": rel(SAME_GAME_JSON),
            "status": same_game.get("status"),
            "role": "fail-closed lifecycle same-game lift for account reads, restore/reorg, spend, and verifier queries",
        },
        {
            "artifact": rel(SPEND_BUDGET_JSON),
            "status": spend_budget.get("status"),
            "role": "fail-closed spend-time disclosure budget for the accepting certificate path",
        },
        {
            "artifact": rel(REJECT_EQUIVALENCE_JSON),
            "status": reject_equivalence.get("status"),
            "role": "fail-closed reject-equivalence contract for real and simulated failure classes",
        },
    ]
    open_gaps = [
        {
            "id": "sha256_prf_or_ro_reduction",
            "status": ro_reduction.get("status"),
            "evidence": rel(RO_REDUCTION_JSON),
            "required_to_close": "discharge the reduction contract's SHA-256 instantiation, global-domain registry, lifecycle, spend-time, and rejection-equivalence dependencies",
        },
        {
            "id": "lifecycle_same_game_lift",
            "status": same_game.get("status"),
            "evidence": rel(SAME_GAME_JSON),
            "required_to_close": "turn the same-game contract into executable negative controls and enforce pad-seed digest uniqueness and packed-witness immutability",
        },
        {
            "id": "spend_time_leakage_budget",
            "status": spend_budget.get("status"),
            "evidence": rel(SPEND_BUDGET_JSON),
            "required_to_close": "complete the no-pre-spend-export audit and negative tests for malformed openings, snapshots, rebinding, and replay",
        },
        {
            "id": "constant_time_reject_equivalence",
            "status": reject_equivalence.get("status"),
            "evidence": rel(REJECT_EQUIVALENCE_JSON),
            "required_to_close": "produce accepting-verifier timing evidence, constant-shape projection-carrier review, parser/account timing equalization, and selected-route diagnostic tests",
        },
    ]
    return {
        "schema": "Descender.pre_signature_projection_mask_prf_leakage_simulator.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "mask_prf_leakage_simulator_contract_fail_closed",
        "deployment_ready": False,
        "target": {
            "n": 32,
            "q": 12289,
            "selected_signature_bytes": 308,
            "packed_witness_rows": 18,
            "checked_core_rows": 12,
            "masked_projection_rows": 6,
        },
        "worlds": [real_world, ideal_world, spend_opening_world],
        "simulator_steps": simulator_steps,
        "named_breaks": named_breaks,
        "advantage_bound": advantage_bound,
        "evidence": evidence,
        "open_gaps": open_gaps,
        "child_gap_contracts": {
            "sha256_prf_or_ro_reduction": rel(RO_REDUCTION_JSON),
            "lifecycle_same_game_lift": rel(SAME_GAME_JSON),
            "spend_time_leakage_budget": rel(SPEND_BUDGET_JSON),
            "constant_time_reject_equivalence": rel(REJECT_EQUIVALENCE_JSON),
        },
        "non_claims": [
            "bounded statistical and independence probes do not prove the PRF claim",
            "the simulator does not hide the selected projection residues after a valid spend opening",
            "this artifact does not remove the production preflight hidden-source semantics blocker",
        ],
        "decision": {
            "can_accept_now": False,
            "can_consume_in_global_soundness": True,
            "reason": (
                "the real/ideal leakage boundary, named breaks, and four child "
                "gap contracts are explicit, but those child contracts remain "
                "fail-closed and do not yet prove deployable leakage hiding"
            ),
            "next_artifact": "compose_child_gap_contracts_into_mask_prf_leakage_theorem",
        },
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection Mask PRF Leakage Simulator",
        "",
        "This artifact makes the Option A leakage boundary explicit. It separates",
        "the live-slot account-state view, where the six projection residues must",
        "be simulatable as uniform masked values, from the valid spend-opening view,",
        "where the selected projection opening is intentionally disclosed and must",
        "be charged in the spend-time leakage budget.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected signature bytes: `{payload['target']['selected_signature_bytes']}`",
        f"- packed witness rows: `{payload['target']['packed_witness_rows']}`",
        f"- masked projection rows: `{payload['target']['masked_projection_rows']}`",
        f"- can consume in global soundness: `{payload['decision']['can_consume_in_global_soundness']}`",
        "",
        "## Worlds",
        "",
        "| world | phase | rule |",
        "|---|---|---|",
    ]
    for world in payload["worlds"]:
        rule = world.get("replacement") or world.get("modeled_disclosure") or ""
        lines.append(f"| `{world['id']}` | `{world['phase']}` | {rule} |")
    lines.extend(["", "## Simulator Steps", "", "| step | charged to | uses secret |", "|---|---|---|"])
    for step in payload["simulator_steps"]:
        lines.append(
            f"| `{step['step']}` | `{step.get('charged_to', 'named-break export')}` | `{step['uses_secret']}` |"
        )
    lines.extend(["", "## Named Breaks", "", "| break | event | advantage term |", "|---|---|---|"])
    for item in payload["named_breaks"]:
        lines.append(
            f"| `{item['id']}` | {item['event']} | `{item['advantage_term']}` |"
        )
    lines.extend(["", "## Advantage Bound", ""])
    lines.extend(f"- `{term}`" for term in payload["advantage_bound"])
    lines.extend(["", "## Evidence", "", "| artifact | status | role |", "|---|---|---|"])
    for item in payload["evidence"]:
        lines.append(f"| `{item['artifact']}` | `{item['status']}` | {item['role']} |")
    lines.extend(["", "## Open Gaps", "", "| gap | status | required to close |", "|---|---|---|"])
    for item in payload["open_gaps"]:
        lines.append(
            f"| `{item['id']}` | `{item['status']}` | {item['required_to_close']} |"
        )
    lines.extend(["", "## Non-Claims", ""])
    lines.extend(f"- {item}" for item in payload["non_claims"])
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
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": str(DEFAULT_JSON),
                "markdown": str(DEFAULT_MD),
                "status": payload["status"],
                "deployment_ready": payload["deployment_ready"],
                "open_gaps": len(payload["open_gaps"]),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
