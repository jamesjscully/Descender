#!/usr/bin/env python3
"""Compose the projection-mask leakage child contracts into one theorem target."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.md"
)
SIMULATOR_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_leakage_simulator_n32_q12289.json"
)
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
SHA256_RESOLUTION_JSON = (
    READINESS_DIR
    / "pre_signature_projection_sha256_mask_prf_assumption_resolution_n32_q12289.json"
)
GLOBAL_DOMAIN_REGISTRY_JSON = (
    READINESS_DIR / "pre_signature_projection_global_domain_registry_n32_q12289.json"
)
LIFECYCLE_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.json"
)
SPEND_BUDGET_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_spend_time_budget_discharge_n32_q12289.json"
)
REJECT_EQUIVALENCE_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_reject_equivalence_discharge_n32_q12289.json"
)
HELPER_API_EXPORT_BOUNDARY_JSON = (
    READINESS_DIR / "pre_signature_projection_helper_api_export_boundary_n32_q12289.json"
)
HELPER_API_EXPORT_BOUNDARY_CLOSURE_JSON = (
    READINESS_DIR
    / "pre_signature_projection_helper_api_export_boundary_closure_n32_q12289.json"
)
NO_EXPORT_NEGATIVE_TESTS_JSON = (
    READINESS_DIR / "pre_signature_projection_no_export_negative_tests_n32_q12289.json"
)
NO_EXPORT_NEGATIVE_TESTS_CLOSURE_JSON = (
    READINESS_DIR / "pre_signature_projection_no_export_negative_tests_closure_n32_q12289.json"
)
REQ_PAD_REJECT_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_req_pad_reject_discharge_n32_q12289.json"
)
REQ_PAD_REJECT_CLOSURE_JSON = (
    READINESS_DIR / "pre_signature_projection_req_pad_reject_closure_n32_q12289.json"
)
REQ_WITNESS_REJECT_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_req_witness_reject_discharge_n32_q12289.json"
)
REQ_WITNESS_REJECT_CLOSURE_JSON = (
    READINESS_DIR / "pre_signature_projection_req_witness_reject_closure_n32_q12289.json"
)
REQ_CARRIER_REJECT_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_req_carrier_reject_discharge_n32_q12289.json"
)
REQ_CARRIER_REJECT_CLOSURE_JSON = (
    READINESS_DIR / "pre_signature_projection_req_carrier_reject_closure_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    simulator = load_json(SIMULATOR_JSON)
    ro_reduction = load_json(RO_REDUCTION_JSON)
    same_game = load_json(SAME_GAME_JSON)
    spend_budget = load_json(SPEND_BUDGET_JSON)
    reject_equivalence = load_json(REJECT_EQUIVALENCE_JSON)
    sha256_resolution = load_json(SHA256_RESOLUTION_JSON)
    global_domain_registry = load_json(GLOBAL_DOMAIN_REGISTRY_JSON)
    lifecycle_discharge = load_json(LIFECYCLE_DISCHARGE_JSON)
    spend_budget_discharge = load_json(SPEND_BUDGET_DISCHARGE_JSON)
    reject_equivalence_discharge = load_json(REJECT_EQUIVALENCE_DISCHARGE_JSON)
    helper_api_export_boundary = load_json(HELPER_API_EXPORT_BOUNDARY_JSON)
    helper_api_export_boundary_closure = load_json(HELPER_API_EXPORT_BOUNDARY_CLOSURE_JSON)
    no_export_negative_tests = load_json(NO_EXPORT_NEGATIVE_TESTS_JSON)
    no_export_negative_tests_closure = load_json(NO_EXPORT_NEGATIVE_TESTS_CLOSURE_JSON)
    req_pad_reject_discharge = load_json(REQ_PAD_REJECT_DISCHARGE_JSON)
    req_pad_reject_closure = load_json(REQ_PAD_REJECT_CLOSURE_JSON)
    req_witness_reject_discharge = load_json(REQ_WITNESS_REJECT_DISCHARGE_JSON)
    req_witness_reject_closure = load_json(REQ_WITNESS_REJECT_CLOSURE_JSON)
    req_carrier_reject_discharge = load_json(REQ_CARRIER_REJECT_DISCHARGE_JSON)
    req_carrier_reject_closure = load_json(REQ_CARRIER_REJECT_CLOSURE_JSON)

    theorem = {
        "name": "pre_signature_projection_mask_prf_leakage_or_named_break",
        "status": "composition_target_fail_closed",
        "statement": (
            "For any adversary interacting with the selected 308-byte/Fq12 "
            "state-assisted masked projection-carrier live-slot view, the six "
            "pre-spend masked projection residues are indistinguishable from "
            "independent uniform q12289 residues, unless the adversary triggers "
            "one named SHA-256/RO, domain, lifecycle, spend-opening, or "
            "reject-equivalence break carried below."
        ),
        "scope": [
            "pre-spend account-state reads of live slots",
            "valid spend openings through the accepting certificate interface",
            "slot create/restore/reorg/sign/verify queries covered by the lifecycle same-game contract",
            "real and simulated rejection behavior covered by the reject-equivalence contract",
        ],
        "non_claims": [
            "does not prove SHA-256 behaves as a random oracle or PRF",
            "does not hide the six selected projection residues after a valid spend opening",
            "does not make the selected 308-byte/Fq12 deployment verifier ready",
            "does not close production constant-time or storage lifecycle review",
        ],
    }

    composition_hops = [
        {
            "id": "simulator_worlds_to_child_contracts",
            "status": simulator.get("status"),
            "evidence": rel(SIMULATOR_JSON),
            "claim": "The parent simulator defines real live-slot, ideal live-slot, and valid spend-opening worlds and delegates each remaining gap to a child contract.",
        },
        {
            "id": "child_contract_ro_reduction",
            "status": ro_reduction.get("status"),
            "evidence": rel(RO_REDUCTION_JSON),
            "claim": "The mask stream replacement is represented by a ROM/SHA-256 assumption boundary with explicit frame, query, hybrid, and break terms.",
        },
        {
            "id": "child_contract_lifecycle_same_game",
            "status": same_game.get("status"),
            "evidence": rel(SAME_GAME_JSON),
            "claim": "Pad seed, slot id, and packed witness fixation are lifted into one experiment with account-state reads, restore/reorg, spend, and verifier queries.",
        },
        {
            "id": "child_contract_spend_time_leakage",
            "status": spend_budget.get("status"),
            "evidence": rel(SPEND_BUDGET_JSON),
            "claim": "The accepting certificate disclosures are enumerated and the six selected projection residues are charged as intentional post-spend disclosure only.",
        },
        {
            "id": "child_contract_reject_equivalence",
            "status": reject_equivalence.get("status"),
            "evidence": rel(REJECT_EQUIVALENCE_JSON),
            "claim": "The reject surface is specified as one public-equivalence obligation across seven real/simulated failure classes.",
        },
        {
            "id": "carried_term_discharge_artifacts",
            "status": "partial_discharge_fail_closed",
            "evidence": [
                rel(SHA256_RESOLUTION_JSON),
                rel(GLOBAL_DOMAIN_REGISTRY_JSON),
                rel(LIFECYCLE_DISCHARGE_JSON),
                rel(SPEND_BUDGET_DISCHARGE_JSON),
                rel(REJECT_EQUIVALENCE_DISCHARGE_JSON),
            ],
            "claim": "The five carried assumptions have concrete discharge or fail-closed resolution artifacts; none removes deployment fail-closed status.",
        },
        {
            "id": "residual_blocker_refinement_artifacts",
            "status": "residuals_refined_fail_closed",
            "evidence": [
                rel(HELPER_API_EXPORT_BOUNDARY_JSON),
                rel(NO_EXPORT_NEGATIVE_TESTS_JSON),
                rel(REQ_PAD_REJECT_DISCHARGE_JSON),
                rel(REQ_WITNESS_REJECT_DISCHARGE_JSON),
                rel(REQ_CARRIER_REJECT_DISCHARGE_JSON),
            ],
            "claim": "The current residual helper/export, no-export-test, and open reject-class blockers have concrete fail-closed artifacts and exact missing tests/harnesses.",
        },
    ]

    named_breaks = [
        *[
            {
                "source": "ro_reduction",
                "id": item["id"],
                "advantage_term": item.get("advantage_term"),
                "status": item.get("status"),
            }
            for item in ro_reduction.get("named_breaks", [])
        ],
        *[
            {
                "source": "lifecycle_same_game",
                "id": item["id"],
                "advantage_term": item.get("advantage_term"),
                "status": "closed_by_policy" if item.get("closed_by_policy") else "open_or_assumption",
            }
            for item in same_game.get("named_lifecycle_breaks", [])
        ],
        {
            "source": "spend_time_leakage_budget",
            "id": "pre_spend_projection_export_break",
            "advantage_term": "Adv_projection_overread(q_sign, q_verify)",
            "status": "open_until_static_api_audit_and_negative_tests",
        },
        {
            "source": "reject_equivalence",
            "id": "constant_time_reject_equivalence_break",
            "advantage_term": "Adv_reject_mismatch(q_reject, t)",
            "status": reject_equivalence_discharge.get("status"),
        },
    ]

    carried_assumptions = [
        {
            "id": "A_sha256_projection_mask_frame_prf",
            "source": rel(SHA256_RESOLUTION_JSON),
            "term": "Adv_sha256_projection_mask_frame_prf(q_hash, t + O(q_hash))",
            "status": sha256_resolution.get("status"),
            "discharge_state": "not_discharged_explicit_assumption_carried",
            "required_resolution": "explicitly accept the exact SHA-256 frame-PRF assumption, prove a replacement theorem, or change construction",
        },
        {
            "id": "A_global_domain_registry",
            "source": rel(GLOBAL_DOMAIN_REGISTRY_JSON),
            "term": "Adv_frame_disjointness(q_hash)",
            "status": global_domain_registry.get("status"),
            "discharge_state": "registered_no_exact_collisions_but_not_discharged",
            "required_resolution": "prove byte-level disjointness across all registered frame encodings and cover ambiguity risks with negative tests",
        },
        {
            "id": "A_lifecycle_same_game",
            "source": rel(LIFECYCLE_DISCHARGE_JSON),
            "term": "Adv_postchallenge_slot_creation + Adv_pad_seed_duplicate + Adv_witness_rewrite + Adv_projection_overread",
            "status": lifecycle_discharge.get("status"),
            "discharge_state": "partially_discharged_reference_path",
            "required_resolution": "close postchallenge slot creation, full witness rewrite, projection overread, and deployment DB profile mismatch residuals",
        },
        {
            "id": "A_spend_time_projection_budget",
            "source": rel(SPEND_BUDGET_DISCHARGE_JSON),
            "term": "Adv_spend_opening_leakage(q_open)",
            "status": spend_budget_discharge.get("status"),
            "discharge_state": "fail_closed_not_proved",
            "required_resolution": "restrict helper/API surfaces and add no-export negative tests for pre-spend and reject paths",
        },
        {
            "id": "A_reject_equivalence",
            "source": rel(REJECT_EQUIVALENCE_DISCHARGE_JSON),
            "term": "Adv_reject_mismatch(q_reject, t)",
            "status": reject_equivalence_discharge.get("status"),
            "discharge_state": "fail_closed_not_discharged",
            "required_resolution": "add final selected-route timing/diagnostic harness and discharge open reject classes",
        },
    ]

    discharge_summary = {
        "sha256_resolution": {
            "status": sha256_resolution.get("status"),
            "can_discharge": sha256_resolution.get("decision", {}).get(
                "can_mark_A_sha256_projection_mask_frame_prf_discharged"
            ),
            "closure_path": sha256_resolution.get("decision", {}).get("closure_path"),
        },
        "global_domain_registry": {
            "status": global_domain_registry.get("status"),
            "can_discharge": global_domain_registry.get("decision", {}).get(
                "can_claim_global_frame_disjointness_now"
            ),
            "registered_families": len(global_domain_registry.get("registry_entries", [])),
            "byte_frame_ambiguity_risks": len(
                global_domain_registry.get("byte_frame_ambiguity_risks", [])
            ),
        },
        "lifecycle_same_game": {
            "status": lifecycle_discharge.get("status"),
            "discharged_terms": len(lifecycle_discharge.get("discharged_terms", [])),
            "residual_terms": len(lifecycle_discharge.get("residual_terms", [])),
        },
        "spend_time_projection_budget": {
            "status": spend_budget_discharge.get("status"),
            "discharged": spend_budget_discharge.get("disclosure_counts", {}).get(
                "discharged"
            ),
            "charged_after_valid_spend": spend_budget_discharge.get(
                "disclosure_counts", {}
            ).get("charged_after_valid_spend"),
            "residual": spend_budget_discharge.get("disclosure_counts", {}).get(
                "residual"
            ),
        },
        "reject_equivalence": {
            "status": reject_equivalence_discharge.get("status"),
            "status_counts": reject_equivalence_discharge.get("status_counts"),
        },
    }
    residual_refinement_summary = {
        "helper_api_export_boundary": {
            "status": helper_api_export_boundary_closure.get("status"),
            "risky_public_or_exportable_surface_count": helper_api_export_boundary.get(
                "risky_public_or_exportable_surface_count"
            ),
            "blockers_closed": len(
                helper_api_export_boundary_closure.get("blockers_closed_by_this_artifact", [])
            ),
            "wrapper_tests_required": len(
                helper_api_export_boundary_closure.get("wrapper_tests_required", [])
            ),
            "residual_blockers": len(
                helper_api_export_boundary_closure.get("blockers_remaining", [])
            ),
        },
        "no_export_negative_tests": {
            "status": no_export_negative_tests_closure.get("status"),
            "implemented_tests": no_export_negative_tests_closure.get("implemented_count"),
            "planned_tests": no_export_negative_tests_closure.get("planned_count"),
            "can_close_projection_overread_residual": no_export_negative_tests_closure.get(
                "acceptance_gate", {}
            ).get("can_close_projection_overread_residual"),
        },
        "req_pad_reject": {
            "status": req_pad_reject_closure.get("status"),
            "kat_count": req_pad_reject_closure.get("kat_count"),
            "five_required_kats_implemented": req_pad_reject_closure.get("decision", {}).get(
                "five_required_kats_implemented"
            ),
            "can_remove": req_pad_reject_closure.get("decision", {}).get(
                "can_remove_req_pad_from_A_reject_equivalence_without_timing_review"
            ),
        },
        "req_witness_reject": {
            "status": req_witness_reject_closure.get("status"),
            "failure_cases": len(
                req_witness_reject_closure.get("covered_failure_cases", [])
            ),
            "kat_gap_closed": req_witness_reject_closure.get("decision", {}).get(
                "kat_gap_closed"
            ),
            "can_remove": req_witness_reject_closure.get("decision", {}).get(
                "can_remove_REQ_WITNESS_from_A_reject_equivalence"
            ),
        },
        "req_carrier_reject": {
            "status": req_carrier_reject_closure.get("status"),
            "failure_cases": len(
                req_carrier_reject_closure.get("requested_kats", [])
            ),
            "kat_count": req_carrier_reject_closure.get("decision", {}).get("kat_count"),
            "can_remove": req_carrier_reject_closure.get("decision", {}).get(
                "can_close_req_carrier"
            ),
        },
    }

    final_advantage_bound = [
        "Adv_masked_projection_live_slot_leakage(q_slot, q_open, q_life, q_hash, q_reject, t)",
        "  <= Adv_sha256_projection_mask_frame_prf(q_hash, t + O(q_hash))",
        "   + Adv_frame_disjointness(q_hash)",
        "   + Adv_postchallenge_slot_creation(q_create)",
        "   + Adv_pad_seed_duplicate(q_slots)",
        "   + Adv_slot_reanimation(q_restore, q_slots)",
        "   + Adv_reorg_unburn(q_reorg)",
        "   + Adv_witness_rewrite(q_open)",
        "   + Adv_projection_overread(q_sign, q_verify)",
        "   + Adv_spend_opening_leakage(q_open)",
        "   + Adv_rejection_implementation_mismatch(q_mask, q_ctr)",
        "   + Adv_reject_mismatch(q_reject, t)",
    ]

    open_to_close = [
        {
            "id": item["id"],
            "source": "ro_reduction",
            "status": item["status"],
            "required_to_close": item["required_to_close"],
        }
        for item in ro_reduction.get("open_before_gap_closes", [])
    ]
    open_to_close.extend(
        {
            "id": item["id"],
            "source": "lifecycle_same_game",
            "status": "open",
            "required_to_close": item["detail"],
        }
        for item in same_game.get("remains_unproved", [])
    )
    open_to_close.extend(
        {
            "id": item["id"],
            "source": "spend_time_leakage_budget",
            "status": item["status"],
            "required_to_close": item["required_to_close"],
        }
        for item in spend_budget.get("remaining_proof_and_test_obligations", [])
    )
    open_to_close.extend(
        {
            "id": item["id"],
            "source": "reject_equivalence",
            "status": item["status"],
            "required_to_close": item["blocker"],
        }
        for item in reject_equivalence.get("remaining_gaps", [])
    )

    return {
        "schema": "Descender.pre_signature_projection_mask_prf_leakage_theorem_composition.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "mask_prf_leakage_theorem_composition_fail_closed",
        "deployment_ready": False,
        "target": simulator["target"],
        "source_artifacts": {
            "simulator": rel(SIMULATOR_JSON),
            "simulator_schema": simulator.get("schema"),
            "ro_reduction": rel(RO_REDUCTION_JSON),
            "ro_reduction_schema": ro_reduction.get("schema"),
            "lifecycle_same_game": rel(SAME_GAME_JSON),
            "lifecycle_same_game_schema": same_game.get("schema"),
            "spend_time_leakage_budget": rel(SPEND_BUDGET_JSON),
            "spend_time_leakage_budget_schema": spend_budget.get("schema"),
            "reject_equivalence": rel(REJECT_EQUIVALENCE_JSON),
            "reject_equivalence_schema": reject_equivalence.get("schema"),
            "sha256_resolution": rel(SHA256_RESOLUTION_JSON),
            "sha256_resolution_schema": sha256_resolution.get("schema"),
            "global_domain_registry": rel(GLOBAL_DOMAIN_REGISTRY_JSON),
            "global_domain_registry_schema": global_domain_registry.get("schema"),
            "lifecycle_discharge": rel(LIFECYCLE_DISCHARGE_JSON),
            "lifecycle_discharge_schema": lifecycle_discharge.get("schema"),
            "spend_budget_discharge": rel(SPEND_BUDGET_DISCHARGE_JSON),
            "spend_budget_discharge_schema": spend_budget_discharge.get("schema"),
            "reject_equivalence_discharge": rel(REJECT_EQUIVALENCE_DISCHARGE_JSON),
            "reject_equivalence_discharge_schema": reject_equivalence_discharge.get("schema"),
            "helper_api_export_boundary": rel(HELPER_API_EXPORT_BOUNDARY_JSON),
            "helper_api_export_boundary_schema": helper_api_export_boundary.get("schema"),
            "helper_api_export_boundary_closure": rel(HELPER_API_EXPORT_BOUNDARY_CLOSURE_JSON),
            "helper_api_export_boundary_closure_schema": helper_api_export_boundary_closure.get("schema"),
            "no_export_negative_tests": rel(NO_EXPORT_NEGATIVE_TESTS_JSON),
            "no_export_negative_tests_schema": no_export_negative_tests.get("schema"),
            "no_export_negative_tests_closure": rel(NO_EXPORT_NEGATIVE_TESTS_CLOSURE_JSON),
            "no_export_negative_tests_closure_schema": no_export_negative_tests_closure.get("schema"),
            "req_pad_reject_discharge": rel(REQ_PAD_REJECT_DISCHARGE_JSON),
            "req_pad_reject_discharge_schema": req_pad_reject_discharge.get("schema"),
            "req_pad_reject_closure": rel(REQ_PAD_REJECT_CLOSURE_JSON),
            "req_pad_reject_closure_schema": req_pad_reject_closure.get("schema"),
            "req_witness_reject_discharge": rel(REQ_WITNESS_REJECT_DISCHARGE_JSON),
            "req_witness_reject_discharge_schema": req_witness_reject_discharge.get("schema"),
            "req_witness_reject_closure": rel(REQ_WITNESS_REJECT_CLOSURE_JSON),
            "req_witness_reject_closure_schema": req_witness_reject_closure.get("schema"),
            "req_carrier_reject_discharge": rel(REQ_CARRIER_REJECT_DISCHARGE_JSON),
            "req_carrier_reject_discharge_schema": req_carrier_reject_discharge.get("schema"),
            "req_carrier_reject_closure": rel(REQ_CARRIER_REJECT_CLOSURE_JSON),
            "req_carrier_reject_closure_schema": req_carrier_reject_closure.get("schema"),
        },
        "theorem": theorem,
        "composition_hops": composition_hops,
        "named_breaks": named_breaks,
        "carried_assumptions": carried_assumptions,
        "discharge_summary": discharge_summary,
        "residual_refinement_summary": residual_refinement_summary,
        "final_advantage_bound": final_advantage_bound,
        "open_to_close": open_to_close,
        "decision": {
            "can_accept_now": False,
            "can_consume_in_global_soundness": True,
            "child_contract_count": 4,
            "discharge_artifact_count": 5,
            "residual_refinement_artifact_count": 5,
            "named_break_count": len(named_breaks),
            "carried_assumption_count": len(carried_assumptions),
            "open_item_count": len(open_to_close),
            "reason": "the child contracts and five carried-term discharge artifacts are composed into one theorem target and final bound, but residual assumptions and proof/test items remain fail-closed",
            "next_artifact": "discharge_or_assume_mask_prf_leakage_carried_terms",
        },
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    decision = payload["decision"]
    lines = [
        "# Pre-Signature Projection Mask PRF Leakage Theorem Composition",
        "",
        "This artifact composes the simulator and four child fail-closed contracts",
        "into one selected-path theorem target. It does not prove deployment",
        "readiness; it names the carried assumptions and open proof/test items",
        "that must be discharged or explicitly accepted in the final bound.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- theorem: `{payload['theorem']['name']}`",
        f"- child contracts: `{decision['child_contract_count']}`",
        f"- named breaks: `{decision['named_break_count']}`",
        f"- carried assumptions: `{decision['carried_assumption_count']}`",
        f"- open items: `{decision['open_item_count']}`",
        f"- can consume in global soundness: `{decision['can_consume_in_global_soundness']}`",
        "",
        "## Theorem",
        "",
        payload["theorem"]["statement"],
        "",
        "## Composition Hops",
        "",
        "| hop | status | claim | evidence |",
        "|---|---|---|---|",
    ]
    for hop in payload["composition_hops"]:
        lines.append(
            f"| `{hop['id']}` | `{hop['status']}` | {hop['claim']} | `{hop['evidence']}` |"
        )
    lines.extend(["", "## Final Bound", ""])
    lines.extend(f"- `{term}`" for term in payload["final_advantage_bound"])
    lines.extend(["", "## Carried Assumptions", "", "| id | term | required resolution |", "|---|---|---|"])
    for item in payload["carried_assumptions"]:
        lines.append(
            f"| `{item['id']}` | `{item['term']}` | {item['required_resolution']} |"
        )
    lines.extend(
        [
            "",
            "## Residual Refinements",
            "",
            "| blocker | status | counts |",
            "|---|---|---|",
        ]
    )
    for key, item in payload["residual_refinement_summary"].items():
        counts = ", ".join(
            f"{count_key} `{count_value}`"
            for count_key, count_value in item.items()
            if count_key != "status"
        )
        lines.append(f"| `{key}` | `{item['status']}` | {counts} |")
    lines.extend(["", "## Open To Close", "", "| source | id | status | required to close |", "|---|---|---|---|"])
    for item in payload["open_to_close"]:
        lines.append(
            f"| `{item['source']}` | `{item['id']}` | `{item['status']}` | {item['required_to_close']} |"
        )
    lines.extend(["", "## Decision", ""])
    lines.extend(
        [
            f"- can accept now: `{decision['can_accept_now']}`",
            f"- reason: {decision['reason']}",
            f"- next artifact: `{decision['next_artifact']}`",
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
                "open_items": payload["decision"]["open_item_count"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
