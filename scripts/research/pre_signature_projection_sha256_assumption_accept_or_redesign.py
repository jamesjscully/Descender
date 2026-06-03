#!/usr/bin/env python3
"""Decide whether to accept the SHA-256 mask assumption or redesign it away."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_sha256_assumption_accept_or_redesign_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_signature_projection_sha256_assumption_accept_or_redesign_n32_q12289.md"
)
SHA256_RESOLUTION_JSON = (
    READINESS_DIR
    / "pre_signature_projection_sha256_mask_prf_assumption_resolution_n32_q12289.json"
)
RO_REDUCTION_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_ro_reduction_n32_q12289.json"
)
COMPOSITION_JSON = (
    READINESS_DIR
    / "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.json"
)
QROM_REVIEW_JSON = READINESS_DIR / "qrom_security_loss_review_n32_q12289.json"
SOUNDNESS_BUDGET_JSON = (
    READINESS_DIR / "conditional_soundness_budget_review_n32_q12289.json"
)

Q = 12_289
N = 32
SELECTED_SIGNATURE_BYTES = 308
CHECKED_CORE_ROWS = 12
MASKED_PROJECTION_ROWS = 6
PACKED_WITNESS_ROWS = CHECKED_CORE_ROWS + MASKED_PROJECTION_ROWS


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def carried_assumption(composition: dict[str, Any]) -> dict[str, Any]:
    for item in composition.get("carried_assumptions", []):
        if item.get("id") == "A_sha256_projection_mask_frame_prf":
            return item
    raise ValueError("composition does not carry A_sha256_projection_mask_frame_prf")


def named_break(ro_reduction: dict[str, Any]) -> dict[str, Any]:
    for item in ro_reduction.get("named_breaks", []):
        if item.get("id") == "sha256_projection_mask_frame_prf_break":
            return item
    raise ValueError("RO reduction does not name sha256_projection_mask_frame_prf_break")


def build_payload() -> dict[str, Any]:
    sha256_resolution = load_json(SHA256_RESOLUTION_JSON)
    ro_reduction = load_json(RO_REDUCTION_JSON)
    composition = load_json(COMPOSITION_JSON)
    qrom_review = load_json(QROM_REVIEW_JSON)
    soundness_budget = load_json(SOUNDNESS_BUDGET_JSON)

    carried = carried_assumption(composition)
    break_row = named_break(ro_reduction)
    exact_assumption = sha256_resolution["exact_assumption"]
    qrom_scope = qrom_review["review_scope"]

    shared_preconditions = [
        "global byte-frame registry remains enforced for all signer/verifier hash frames",
        "pad_seed, slot_id, packed witness bytes, and selected lanes are fixed before the mask challenge",
        "accepted spend openings are charged through q_open and do not create pre-spend raw projection exports",
        "production mask generation matches the modeled rejection parser and counter behavior",
        "the parent composition continues to carry lifecycle, spend-opening, and reject-equivalence terms separately",
    ]

    closure_paths = [
        {
            "id": "accept_exact_sha256_frame_prf_assumption",
            "decision_status": "available_as_explicit_assumption_only",
            "what_changes": [
                "record A_sha256_projection_mask_frame_prf as an accepted project assumption for this exact frame",
                "keep the deployed SHA-256 mask stream and frame layout unchanged",
                "update parent theorem language from open blocker to explicitly assumed term",
            ],
            "assumption_consumed": exact_assumption["advantage_term"],
            "proof_impact": [
                "no ROM-only discharge is claimed for the concrete SHA-256 path",
                "the H0_real_sha256_masks -> H1_ro_masks hop remains charged to the explicit assumption",
                "existing RO reduction, frame registry, lifecycle, spend-budget, and reject-equivalence artifacts remain structurally usable",
                "the selected theorem can be stated fail-closed as conditional on A_sha256_projection_mask_frame_prf plus the other carried terms",
            ],
            "deployment_impact": [
                "no signature-format change",
                "no account-state, verifier, or mask-frame migration",
                "no new consensus or storage versioning for the mask stream",
                "external review must accept the exact assumption boundary before deployment readiness can improve",
            ],
            "costs": {
                "engineering": "low",
                "proof_rework": "low_to_medium",
                "review_governance": "high",
                "deployment_migration": "none",
            },
            "benefits": [
                "fastest way to make the blocker a conscious assumption instead of an unresolved gap",
                "preserves the reviewed 308-byte/Fq12 selected path",
                "keeps implementation risk concentrated in review of one exact byte frame and query budget",
            ],
            "risks": [
                "security rests on a non-proved SHA-256 frame-PRF/RO-instantiation judgment",
                "reviewers may reject adding a new exact SHA-256 PRF-style assumption",
                "must avoid wording that treats statistical probes or frame disjointness as proof of SHA-256 pseudorandomness",
            ],
            "deployment_ready_after_path": False,
            "can_remove_adv_sha256_term": False,
        },
        {
            "id": "redesign_mask_stream_to_accepted_ro_model",
            "decision_status": "available_only_as_construction_redesign",
            "what_changes": [
                "replace the concrete SHA-256 mask-frame instantiation in the theorem target with an ideal H_ro/QROM mask oracle",
                "bind the mask stream to the existing random-oracle model and query ledger instead of a new SHA-256 frame-PRF assumption",
                "treat any concrete SHA-256 deployment as a separate future instantiation step, not as discharged by the redesign",
            ],
            "assumption_consumed": "existing H_ro/QROM model used by the proof ledger, not a proof of SHA-256",
            "proof_impact": [
                "removes Adv_sha256_projection_mask_frame_prf only from the idealized mask-stream theorem",
                "keeps Adv_frame_disjointness, Adv_lifecycle_fixation, Adv_spend_opening_leakage, and Adv_rejection_implementation_mismatch",
                "requires regenerating the RO reduction, composition theorem, frame registry references, and implementation-equivalence artifacts around the new oracle boundary",
                "requires a theorem statement that is explicit about being ideal-RO/QROM unless a future concrete instantiation is separately accepted",
            ],
            "deployment_impact": [
                "cannot be deployed as a concrete SHA-256 claim without reintroducing an instantiation assumption",
                "likely requires API, test-vector, transcript, and review-packet changes if production keeps a concrete mask generator",
                "may require versioned signature/account-state handling if the mask frame or transcript-visible semantics change",
                "reduces assumption novelty only by moving the mask stream into an already modeled oracle boundary",
            ],
            "costs": {
                "engineering": "medium_to_high",
                "proof_rework": "high",
                "review_governance": "medium",
                "deployment_migration": "medium_if_existing_test_vectors_or_state_are_kept",
            },
            "benefits": [
                "does not ask reviewers to accept a new SHA-256 frame-PRF assumption for masks",
                "aligns the mask stream with the already reviewed random-oracle/QROM proof style",
                "makes the idealized theorem cleaner by deleting the SHA-256 instantiation hop",
            ],
            "risks": [
                "does not justify the currently deployed concrete SHA-256 mask stream",
                "may create a proof/deployment split if the implementation still hashes concrete frames",
                "touches more artifacts and can disturb lifecycle and reject-equivalence reviews",
            ],
            "deployment_ready_after_path": False,
            "can_remove_adv_sha256_term": True,
            "removal_scope": "idealized_H_ro_mask_stream_only",
        },
    ]

    comparison = {
        "decision_axis": [
            {
                "axis": "assumption novelty",
                "accept_exact_sha256": "adds or accepts one exact SHA-256 frame-PRF assumption",
                "redesign_to_ro": "uses the existing H_ro/QROM model but only for an idealized construction",
            },
            {
                "axis": "proof churn",
                "accept_exact_sha256": "localized theorem wording and assumption ledger updates",
                "redesign_to_ro": "regenerate reduction, composition, implementation-equivalence, and review packet references",
            },
            {
                "axis": "deployment churn",
                "accept_exact_sha256": "none for mask bytes or state",
                "redesign_to_ro": "none only if kept proof-only; nontrivial if production semantics are changed",
            },
            {
                "axis": "review risk",
                "accept_exact_sha256": "high cryptographic-governance risk",
                "redesign_to_ro": "medium model-scoping risk and high proof/deployment-split risk",
            },
            {
                "axis": "blocker closure quality",
                "accept_exact_sha256": "closes as an explicit accepted assumption, not as a theorem",
                "redesign_to_ro": "closes only by changing the construction/proof target away from concrete SHA-256",
            },
        ],
        "strict_non_claim": (
            "Neither path proves SHA-256. Accepting the exact assumption consumes it; "
            "redesigning to H_ro removes it only from the idealized mask-stream theorem."
        ),
    }

    recommendation = {
        "selected_path": "accept_exact_sha256_frame_prf_assumption_for_current_308_byte_path",
        "confidence": "conditional_on_external_assumption_acceptance",
        "rationale": (
            "For the current selected 308-byte/Fq12 path, the least disruptive closure is to "
            "make A_sha256_projection_mask_frame_prf an explicit accepted assumption with the "
            "exact game, frame, parser, and q_hash budget already pinned. Redesigning to the "
            "accepted H_ro/QROM model is cleaner mathematically, but it does not justify the "
            "current concrete SHA-256 deployment and would require broader proof and artifact churn."
        ),
        "fallback_if_rejected": (
            "If reviewers will not accept a new SHA-256 frame-PRF-style assumption, redesign the "
            "mask stream as an ideal H_ro/QROM construction and mark the concrete SHA-256 "
            "instantiation as out of scope until separately reviewed."
        ),
        "immediate_next_actions": [
            "obtain explicit review sign-off or rejection for A_sha256_projection_mask_frame_prf",
            "if accepted, update the parent composition/index artifacts to say accepted assumption rather than open blocker",
            "if rejected, open a construction-redesign task that regenerates the mask RO reduction and composition artifacts",
        ],
    }

    residual_blockers = [
        {
            "id": "external_acceptance_of_A_sha256_projection_mask_frame_prf",
            "status": "open",
            "blocks": "turning the recommendation into a discharged decision for the concrete SHA-256 path",
        },
        {
            "id": "global_domain_registry",
            "status": "still_carried",
            "blocks": "cross-domain frame ambiguity and q_hash accounting closure",
        },
        {
            "id": "lifecycle_same_game",
            "status": "still_carried",
            "blocks": "same-game fixation of pad_seed, slot_id, and packed witness bytes",
        },
        {
            "id": "spend_time_projection_budget",
            "status": "still_carried",
            "blocks": "pre-spend projection export and accepted-opening leakage accounting",
        },
        {
            "id": "reject_equivalence_and_implementation_equivalence",
            "status": "still_carried",
            "blocks": "finite counter/parser behavior and public rejection surface matching the proof game",
        },
    ]

    return {
        "schema": "Descender.pre_signature_projection_sha256_assumption_accept_or_redesign.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "sha256_assumption_decision_recommend_accept_explicit_assumption_fail_closed",
        "deployment_ready": False,
        "target": {
            "n": N,
            "q": Q,
            "selected_signature_bytes": SELECTED_SIGNATURE_BYTES,
            "checked_core_rows": CHECKED_CORE_ROWS,
            "masked_projection_rows": MASKED_PROJECTION_ROWS,
            "packed_witness_rows": PACKED_WITNESS_ROWS,
            "state_path": "308-byte/Fq12 state-assisted masked projection-carrier path",
        },
        "source_artifacts": {
            "sha256_resolution": {
                "artifact": rel(SHA256_RESOLUTION_JSON),
                "status": sha256_resolution.get("status"),
                "can_mark_discharged": sha256_resolution.get("decision", {}).get(
                    "can_mark_A_sha256_projection_mask_frame_prf_discharged"
                ),
            },
            "ro_reduction": {
                "artifact": rel(RO_REDUCTION_JSON),
                "status": ro_reduction.get("status"),
                "named_break": break_row["id"],
            },
            "composition": {
                "artifact": rel(COMPOSITION_JSON),
                "status": composition.get("status"),
                "carried_term": carried["term"],
            },
            "qrom_security_loss_review": {
                "artifact": rel(QROM_REVIEW_JSON),
                "status": qrom_review.get("status"),
                "conditional_on": qrom_scope.get("conditional_on", []),
                "not_claimed": qrom_scope.get("not_claimed", []),
            },
            "conditional_soundness_budget": {
                "artifact": rel(SOUNDNESS_BUDGET_JSON),
                "status": soundness_budget.get("status"),
                "minimum_checked_term_bits": soundness_budget.get(
                    "local_check_summary", {}
                ).get("minimum_checked_term_bits"),
            },
        },
        "shared_preconditions": shared_preconditions,
        "closure_paths": closure_paths,
        "comparison": comparison,
        "recommendation": recommendation,
        "decision_flags": {
            "sha256_proved": False,
            "can_mark_A_sha256_projection_mask_frame_prf_discharged_now": False,
            "recommend_accepting_as_named_assumption": True,
            "recommend_redesign_now": False,
            "redesign_required_if_assumption_rejected": True,
            "rom_only_closes_current_sha256_deployment": False,
        },
        "residual_blockers": residual_blockers,
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    rec = payload["recommendation"]
    flags = payload["decision_flags"]
    lines = [
        "# SHA-256 Assumption Accept-or-Redesign Decision",
        "",
        "This decision artifact compares the two concrete closure paths for the",
        "projection-mask SHA-256 frame-PRF blocker. It does not prove SHA-256.",
        "",
        "## Decision",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected path: `{rec['selected_path']}`",
        f"- confidence: `{rec['confidence']}`",
        f"- SHA-256 proved: `{flags['sha256_proved']}`",
        f"- can mark `A_sha256_projection_mask_frame_prf` discharged now: `{flags['can_mark_A_sha256_projection_mask_frame_prf_discharged_now']}`",
        f"- redesign required if assumption rejected: `{flags['redesign_required_if_assumption_rejected']}`",
        "",
        rec["rationale"],
        "",
        f"Fallback if rejected: {rec['fallback_if_rejected']}",
        "",
        "## Shared Preconditions",
        "",
    ]
    lines.extend(f"- {item}" for item in payload["shared_preconditions"])

    lines.extend(
        [
            "",
            "## Closure Paths",
            "",
        ]
    )
    for path in payload["closure_paths"]:
        lines.extend(
            [
                f"### `{path['id']}`",
                "",
                f"- status: `{path['decision_status']}`",
                f"- assumption consumed: `{path['assumption_consumed']}`",
                f"- deployment ready after path: `{path['deployment_ready_after_path']}`",
                f"- can remove SHA-256 advantage term: `{path['can_remove_adv_sha256_term']}`",
                "",
                "Costs:",
                "",
            ]
        )
        for key, value in path["costs"].items():
            lines.append(f"- `{key}`: `{value}`")
        lines.extend(["", "What changes:", ""])
        lines.extend(f"- {item}" for item in path["what_changes"])
        lines.extend(["", "Proof impact:", ""])
        lines.extend(f"- {item}" for item in path["proof_impact"])
        lines.extend(["", "Deployment impact:", ""])
        lines.extend(f"- {item}" for item in path["deployment_impact"])
        lines.extend(["", "Benefits:", ""])
        lines.extend(f"- {item}" for item in path["benefits"])
        lines.extend(["", "Risks:", ""])
        lines.extend(f"- {item}" for item in path["risks"])
        lines.append("")

    lines.extend(
        [
            "## Comparison",
            "",
            "| axis | accept exact SHA-256 assumption | redesign to RO/QROM model |",
            "|---|---|---|",
        ]
    )
    for row in payload["comparison"]["decision_axis"]:
        lines.append(
            f"| `{row['axis']}` | {row['accept_exact_sha256']} | {row['redesign_to_ro']} |"
        )
    lines.extend(["", payload["comparison"]["strict_non_claim"], ""])

    lines.extend(["## Immediate Next Actions", ""])
    lines.extend(f"- {item}" for item in rec["immediate_next_actions"])

    lines.extend(["", "## Residual Blockers", "", "| id | status | blocks |", "|---|---|---|"])
    for row in payload["residual_blockers"]:
        lines.append(f"| `{row['id']}` | `{row['status']}` | {row['blocks']} |")

    lines.extend(["", "## Source Artifacts", "", "| artifact | status |", "|---|---|"])
    for row in payload["source_artifacts"].values():
        lines.append(f"| `{row['artifact']}` | `{row['status']}` |")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": rel(DEFAULT_JSON),
                "markdown": rel(DEFAULT_MD),
                "status": payload["status"],
                "recommendation": payload["recommendation"]["selected_path"],
                "sha256_proved": payload["decision_flags"]["sha256_proved"],
                "discharged_now": payload["decision_flags"][
                    "can_mark_A_sha256_projection_mask_frame_prf_discharged_now"
                ],
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
