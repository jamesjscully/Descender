#!/usr/bin/env python3
"""Resolve the carried SHA-256 projection-mask frame-PRF term fail-closed."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "pre_signature_projection_sha256_mask_prf_assumption_resolution_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_signature_projection_sha256_mask_prf_assumption_resolution_n32_q12289.md"
)
COMPOSITION_JSON = (
    READINESS_DIR
    / "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.json"
)
RO_REDUCTION_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_ro_reduction_n32_q12289.json"
)
ASSUMPTION_API_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_assumption_api_n32_q12289.json"
)

Q = 12_289
N = 32
SELECTED_SIGNATURE_BYTES = 308
CHECKED_CORE_ROWS = 12
MASKED_PROJECTION_ROWS = 6
PACKED_WITNESS_ROWS = CHECKED_CORE_ROWS + MASKED_PROJECTION_ROWS
MASK_DOMAIN = (
    "Descender.Signature.q12289.local_delta."
    "reviewed_fq12_packed_witness_state18_projection_mask.v1"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def require_carried_term(composition: dict[str, Any]) -> dict[str, Any]:
    for item in composition.get("carried_assumptions", []):
        if item.get("id") == "A_sha256_projection_mask_frame_prf":
            return item
    raise ValueError("composition artifact does not carry A_sha256_projection_mask_frame_prf")


def require_named_break(ro_reduction: dict[str, Any]) -> dict[str, Any]:
    for item in ro_reduction.get("named_breaks", []):
        if item.get("id") == "sha256_projection_mask_frame_prf_break":
            return item
    raise ValueError("RO reduction artifact does not name sha256_projection_mask_frame_prf_break")


def build_payload() -> dict[str, Any]:
    composition = load_json(COMPOSITION_JSON)
    ro_reduction = load_json(RO_REDUCTION_JSON)
    assumption_api = load_json(ASSUMPTION_API_JSON)
    carried = require_carried_term(composition)
    named_break = require_named_break(ro_reduction)

    ro_game = ro_reduction["ro_game"]
    prf_game = ro_reduction["prf_instantiation_game"]
    query_budget_symbols = ro_reduction["query_budget_symbols"]

    exact_assumption = {
        "id": "A_sha256_projection_mask_frame_prf",
        "status": "explicit_assumption_carried_fail_closed",
        "game": "G_sha256_projection_mask_frame_prf",
        "advantage_term": "Adv_sha256_projection_mask_frame_prf(q_hash, t + O(q_hash))",
        "challenger_setup": [
            "The challenger samples the lifecycle-valid slot secrets used by the target path, including secret prechallenge-fixed pad_seed values.",
            "The adversary may adaptively request account-state, lifecycle, verifier, and direct hash-oracle views allowed by the composed leakage experiment.",
            "All queried projection-mask frames must use the registered mask domain and layout from the RO reduction artifact.",
        ],
        "challenge": (
            "For every adversarially reachable projection-mask frame "
            "MASK_DOMAIN || pad_seed || le64(slot_id) || le64(slot_id) || "
            "le64(lane) || le32(counter), the challenger answers either with "
            "the implemented SHA-256 digest of the full frame or with a lazy "
            "random 256-bit value that is consistent on repeated identical "
            "frames. The adversary wins by distinguishing the branch."
        ),
        "frame": {
            "domain": MASK_DOMAIN,
            "hash_call": prf_game["implemented_hash_call"],
            "layout": ro_game["layout"],
            "output_parser": ro_game["mask_stream_output"],
        },
        "adversary_restrictions": [
            "pad_seed, slot_id, packed witness bytes, and lane set are fixed before the challenge branch is sampled",
            "the adversary does not receive raw pre-spend selected projection residues or pad_seed except through accepted openings already charged elsewhere",
            "direct SHA-256 queries outside the mask domain are counted in q_hash and covered only through the stated multi-domain frame boundary",
        ],
        "not_in_assumption": [
            "no collision-resistance, preimage-resistance, or generic SHA-256 security theorem is derived",
            "no claim that SHA-256 is a random oracle for unrelated protocol frames",
            "no lifecycle, spend-opening, rejection-equivalence, or global frame-registry gap is discharged by this term",
        ],
    }

    decision = {
        "closure_path": "carry_explicit_sha256_frame_prf_assumption",
        "rom_only_closes_selected_sha256_path": False,
        "rom_only_closes_idealized_ro_path": True,
        "must_change_construction_if_assumption_rejected": True,
        "can_mark_A_sha256_projection_mask_frame_prf_discharged": False,
        "reason": (
            "The RO reduction proves only the idealized H_ro replacement once distinct frames, "
            "lifecycle fixation, spend-opening leakage, and rejection behavior are separately "
            "handled. The deployed path hashes concrete SHA-256 frames, so replacing SHA-256 "
            "with H_ro is exactly the carried Adv_sha256_projection_mask_frame_prf term."
        ),
    }

    bound_placement = {
        "composition_bound_line": carried["term"],
        "ro_reduction_transition": "H0_real_sha256_masks -> H1_ro_masks",
        "charged_break": named_break["id"],
        "charged_event": named_break["event"],
        "charged_to": "Adv_sha256_projection_mask_frame_prf(q_hash, t + O(q_hash))",
        "parent_bound_fragment": [
            "Adv_masked_projection_live_slot_leakage(...)",
            "  <= Adv_sha256_projection_mask_frame_prf(q_hash, t + O(q_hash))",
            "   + Adv_frame_disjointness(q_hash)",
            "   + Adv_lifecycle_fixation(q_life)",
            "   + Adv_spend_opening_leakage(q_open)",
            "   + Adv_rejection_implementation_mismatch(q_mask, q_ctr)",
            "   + carried lifecycle/reject terms from the composition artifact",
        ],
        "rom_only_variant": [
            "If the construction and proof are explicitly idealized to H_ro, remove only",
            "Adv_sha256_projection_mask_frame_prf and keep all other frame, lifecycle,",
            "spend-opening, and rejection-equivalence terms.",
        ],
    }

    query_terms = {
        key: query_budget_symbols[key]
        for key in [
            "q_slot",
            "q_lane",
            "q_ctr",
            "q_mask",
            "q_ro_direct",
            "q_hash",
            "q_open",
            "q_life",
            "t",
        ]
    }
    query_terms["placement_constraints"] = [
        "q_lane <= 6 * q_slot for the six selected masked projection rows",
        "q_mask <= q_lane * q_ctr for rejection-sampling counter blocks",
        "q_hash includes q_mask, direct adversarial SHA-256/RO queries, and modeled verifier hash queries outside the mask stream",
        "the SHA-256 assumption term receives time t + O(q_hash) for oracle simulation and table consistency",
    ]

    alternatives = [
        {
            "path": "keep_current_sha256_frame_construction",
            "status": "allowed_only_with_explicit_assumption",
            "effect": "carry Adv_sha256_projection_mask_frame_prf in every selected-path theorem statement",
        },
        {
            "path": "rom_only_theorem",
            "status": "not_a_closure_for_deployed_sha256",
            "effect": "valid only for an ideal H_ro construction or a proof statement that intentionally remains in the random-oracle model",
        },
        {
            "path": "different_construction",
            "status": "required_if_explicit_sha256_assumption_is_unacceptable",
            "effect": (
                "replace this boundary with a construction that has an accepted theorem or "
                "standard assumption in the project model, then regenerate the RO reduction, "
                "frame registry, implementation-equivalence, and composition artifacts"
            ),
        },
    ]

    discharge_requirements = [
        {
            "id": "formal_sha256_frame_prf_analysis",
            "required": (
                "provide an accepted project theorem or cryptanalytic assumption review that "
                "bounds Adv_sha256_projection_mask_frame_prf for the exact full-frame SHA-256 "
                "interface and multi-user lifecycle-visible query model"
            ),
        },
        {
            "id": "global_frame_registry",
            "required": "prove all signer/verifier SHA-256 frames are byte-disjoint or account for cross-domain queries in q_hash",
        },
        {
            "id": "same_game_reduction_update",
            "required": "update the parent leakage theorem so the H0 -> H1 transition cites the accepted theorem/assumption and no longer lists this term as open",
        },
        {
            "id": "implementation_equivalence",
            "required": "tie production mask generation to the exact frame layout, counter parser, and rejection behavior used in the game",
        },
        {
            "id": "regenerated_artifacts",
            "required": "regenerate this resolution, RO reduction, and composition artifacts with A_sha256_projection_mask_frame_prf marked discharged",
        },
    ]

    return {
        "schema": "Descender.pre_signature_projection_sha256_mask_prf_assumption_resolution.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "sha256_projection_mask_frame_prf_assumption_carried_fail_closed",
        "deployment_ready": False,
        "target": {
            "n": N,
            "q": Q,
            "selected_signature_bytes": SELECTED_SIGNATURE_BYTES,
            "packed_witness_rows": PACKED_WITNESS_ROWS,
            "checked_core_rows": CHECKED_CORE_ROWS,
            "masked_projection_rows": MASKED_PROJECTION_ROWS,
            "state_path": "308-byte/Fq12 state-assisted masked projection-carrier path",
        },
        "source_artifacts": {
            "composition": {
                "artifact": rel(COMPOSITION_JSON),
                "status": composition.get("status"),
            },
            "ro_reduction": {
                "artifact": rel(RO_REDUCTION_JSON),
                "status": ro_reduction.get("status"),
            },
            "assumption_api": {
                "artifact": rel(ASSUMPTION_API_JSON),
                "status": assumption_api.get("status"),
            },
        },
        "decision": decision,
        "exact_assumption": exact_assumption,
        "query_terms": query_terms,
        "bound_placement": bound_placement,
        "alternatives": alternatives,
        "non_claims": [
            "This artifact does not prove SHA-256 is a PRF or a random oracle.",
            "This artifact does not discharge A_sha256_projection_mask_frame_prf.",
            "This artifact does not claim ROM-only security for the concrete SHA-256 selected path.",
            "This artifact does not close lifecycle fixation, global frame disjointness, spend-opening leakage, rejection equivalence, or implementation equivalence.",
            "Statistical and independence probes remain supporting evidence only and contribute no theorem credit to SHA-256 security.",
        ],
        "discharge_requirements": discharge_requirements,
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# SHA-256 Projection Mask Frame-PRF Assumption Resolution",
        "",
        "This fail-closed artifact resolves carried term 1 for the selected",
        "308-byte/Fq12 state-assisted masked projection-carrier path. It decides",
        "where the SHA-256/RO boundary sits; it does not prove SHA-256 security.",
        "",
        "## Status",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- closure path: `{payload['decision']['closure_path']}`",
        f"- ROM-only closes selected SHA-256 path: `{payload['decision']['rom_only_closes_selected_sha256_path']}`",
        f"- ROM-only closes idealized RO path: `{payload['decision']['rom_only_closes_idealized_ro_path']}`",
        f"- can mark `A_sha256_projection_mask_frame_prf` discharged: `{payload['decision']['can_mark_A_sha256_projection_mask_frame_prf_discharged']}`",
        "",
        payload["decision"]["reason"],
        "",
        "## Exact Assumption",
        "",
        f"- id: `{payload['exact_assumption']['id']}`",
        f"- status: `{payload['exact_assumption']['status']}`",
        f"- game: `{payload['exact_assumption']['game']}`",
        f"- advantage term: `{payload['exact_assumption']['advantage_term']}`",
        "",
        "Challenge:",
        "",
        payload["exact_assumption"]["challenge"],
        "",
        "Frame:",
        "",
        f"- domain: `{payload['exact_assumption']['frame']['domain']}`",
        f"- hash call: `{payload['exact_assumption']['frame']['hash_call']}`",
        f"- parser: {payload['exact_assumption']['frame']['output_parser']}",
        "",
        "| field | offset | bytes | value |",
        "|---|---:|---:|---|",
    ]
    for item in payload["exact_assumption"]["frame"]["layout"]:
        lines.append(
            f"| `{item['field']}` | {item['offset']} | {item['bytes']} | {item['value']} |"
        )

    lines.extend(["", "Challenger setup:", ""])
    lines.extend(f"- {item}" for item in payload["exact_assumption"]["challenger_setup"])
    lines.extend(["", "Adversary restrictions:", ""])
    lines.extend(f"- {item}" for item in payload["exact_assumption"]["adversary_restrictions"])

    lines.extend(
        [
            "",
            "## Query Terms",
            "",
            "| symbol | meaning |",
            "|---|---|",
        ]
    )
    for key, value in payload["query_terms"].items():
        if key == "placement_constraints":
            continue
        lines.append(f"| `{key}` | {value} |")
    lines.extend(["", "Placement constraints:", ""])
    lines.extend(f"- {item}" for item in payload["query_terms"]["placement_constraints"])

    lines.extend(
        [
            "",
            "## Bound Placement",
            "",
            f"- transition: `{payload['bound_placement']['ro_reduction_transition']}`",
            f"- charged break: `{payload['bound_placement']['charged_break']}`",
            f"- charged to: `{payload['bound_placement']['charged_to']}`",
            "",
            "Parent bound fragment:",
            "",
        ]
    )
    lines.extend(f"- `{item}`" for item in payload["bound_placement"]["parent_bound_fragment"])
    lines.extend(["", "ROM-only variant:", ""])
    lines.extend(f"- {item}" for item in payload["bound_placement"]["rom_only_variant"])

    lines.extend(
        [
            "",
            "## Closure Alternatives",
            "",
            "| path | status | effect |",
            "|---|---|---|",
        ]
    )
    for item in payload["alternatives"]:
        lines.append(f"| `{item['path']}` | `{item['status']}` | {item['effect']} |")

    lines.extend(
        [
            "",
            "## Needed To Mark Discharged",
            "",
            "| requirement | required evidence |",
            "|---|---|",
        ]
    )
    for item in payload["discharge_requirements"]:
        lines.append(f"| `{item['id']}` | {item['required']} |")

    lines.extend(["", "## Non-Claims", ""])
    lines.extend(f"- {item}" for item in payload["non_claims"])
    lines.extend(["", "## Source Artifacts", "", "| artifact | status |", "|---|---|"])
    for item in payload["source_artifacts"].values():
        lines.append(f"| `{item['artifact']}` | `{item['status']}` |")
    lines.append("")
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
                "closure_path": payload["decision"]["closure_path"],
                "discharged": payload["decision"][
                    "can_mark_A_sha256_projection_mask_frame_prf_discharged"
                ],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
