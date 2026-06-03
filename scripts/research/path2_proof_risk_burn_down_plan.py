#!/usr/bin/env python3
"""Generate the risk-first Path 2 proof burn-down plan."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_proof_risk_burn_down_plan.json"
OUT_MD = ROOT / "research/path2_proof_risk_burn_down_plan.md"


RISKS = [
    {
        "rank": 1,
        "id": "transparent_relation_knowledge_soundness",
        "risk": "highest",
        "question": (
            "Does acceptance of the transparent payload actually imply a valid "
            "opening of the 80-lane / 66-row Path 2 descent relation, or a named "
            "transparentProofSoundnessBreak?"
        ),
        "why_first": (
            "If this fails, the selected Path 2 proof object is wrong and byte/"
            "verifier engineering is wasted."
        ),
        "next_proof_move": (
            "Define the exact arithmetized relation and extraction target before "
            "building a verifier: trace columns, row constraints, public inputs, "
            "opened query policy, and extractor output map."
        ),
        "redesign_trigger": (
            "No compact transparent proof family gives extractability for this "
            "relation without leaking hidden lanes or exceeding the account-"
            "creation byte budget."
        ),
        "current_artifacts": [
            "research/path2_transparent_relation_arithmetization_target.md",
            "research/path2_inner_commitment_relation_refinement.md",
            "research/path2_trace_commitment_binding_audit.md",
            "research/path2_role_commitment_link_equation_options.md",
            "research/path2_transparent_commitment_api.md",
            "research/path2_transparent_proof_family_gate.md",
            "research/path2_masked_trace_transform.md",
            "research/path2_masked_trace_range_account_delta.md",
            "research/path2_zk_masked_trace_iop_assumption.md",
            "research/path2_masked_trace_iop_selection_checklist.md",
            "research/path2_masked_trace_commitment_layout.md",
            "research/path2_masked_trace_leaf_encoding_fork.md",
            "research/path2_two_view_trace_theorem_interface.md",
            "research/path2_two_view_consistency_obligations.md",
            "research/path2_two_view_linear_leakage_probe.md",
            "research/path2_two_view_column_profile_rules.md",
            "research/path2_top_proof_risk_resolution_options.md",
            "research/path2_transparent_extractor_soundness_bridge.md",
            "research/path2_transparent_payload_contract.md",
            "research/path2_descent_sis_obligation_matrix.md",
        ],
    },
    {
        "rank": 2,
        "id": "query_opening_zero_knowledge_no_export",
        "risk": "high",
        "question": (
            "Do concrete query openings, Merkle paths, and consistency messages "
            "avoid exporting pad lanes, projection lanes, masks, opening scalars, "
            "or range witnesses?"
        ),
        "why_first": (
            "Transparent systems naturally reveal sampled trace cells; if the "
            "trace layout places secrets in opened cells, the design leaks even "
            "when soundness is fine."
        ),
        "next_proof_move": (
            "Classify every trace column as public, masked, committed-hidden, or "
            "derived-public; then prove sampled openings are simulatable or "
            "one-time masked under the transcript schedule."
        ),
        "redesign_trigger": (
            "Any required query opening exposes an unmasked pad/projection/"
            "opening lane or a reusable mask relation."
        ),
        "current_artifacts": [
            "research/path2_transparent_relation_arithmetization_target.md",
            "research/path2_inner_commitment_relation_refinement.md",
            "research/path2_top_proof_risk_resolution_options.md",
            "research/path2_transparent_payload_contract.md",
            "research/path2_transparent_extractor_soundness_bridge.md",
        ],
    },
    {
        "rank": 3,
        "id": "norm_range_binding_semantics",
        "risk": "high",
        "question": (
            "Are the extracted q12289 semantic lanes canonical and short enough "
            "for the binding/norm claim, or do we need base-128 digit openings?"
        ),
        "why_first": (
            "This is the main place the descent/SIS vocabulary can silently overclaim."
        ),
        "next_proof_move": (
            "Compare raw-centered residues and base-128 digit openings as theorem "
            "statements, not implementations: exact bounds, modular-wrap failure "
            "cases, and byte impact."
        ),
        "redesign_trigger": (
            "The only clean binding statement requires too many range/digit "
            "columns for the transparent proof budget."
        ),
        "current_artifacts": [
            "research/path2_descent_sis_norm_range_surface.md",
            "research/path2_descent_sis_binding_surface.md",
        ],
    },
    {
        "rank": 4,
        "id": "public_binding_and_leaf_context_composition",
        "risk": "medium_high",
        "question": (
            "Does the public view bind the exact leaf/evidence/certificate and "
            "creation-proof binding without circularity or rebinding?"
        ),
        "why_first": (
            "A bad public binding can make a correct proof verify for the wrong "
            "slot, account, state root, or creation statement."
        ),
        "next_proof_move": (
            "Keep the nonrecursive leaf-context digest theorem explicit and prove "
            "the direct certificate consumes the same public fields."
        ),
        "redesign_trigger": (
            "Any required binding needs payload bytes to define the leaf meaning "
            "or lets the payload move between leaves."
        ),
        "current_artifacts": [
            "research/direct_replayable_relation_certificate_soundness_theorem.md",
            "tools/parameter-search/src/production_reference.rs",
        ],
    },
    {
        "rank": 5,
        "id": "fiat_shamir_qrom_budget",
        "risk": "medium",
        "question": (
            "Are the transparent proof queries and all Path 2 public-input hashes "
            "accounted for under one coherent QROM/domain-separation loss?"
        ),
        "why_first": (
            "This usually does not force structural redesign, but it can kill a "
            "parameter set."
        ),
        "next_proof_move": (
            "Only after the proof family/query schedule is fixed, instantiate the "
            "exact query counts and update the QROM ledger."
        ),
        "redesign_trigger": (
            "Concrete query counts erase the target security margin."
        ),
        "current_artifacts": [
            "research/path2_hash_qrom_assumption_ledger.md",
            "research/path2_transparent_payload_contract.md",
        ],
    },
    {
        "rank": 6,
        "id": "implementation_verifier_kats",
        "risk": "later",
        "question": (
            "Does the Rust verifier implement the proved relation and reject all "
            "malformed payloads without private diagnostics?"
        ),
        "why_first": (
            "This is necessary, but it should follow the proof-risk burn-down so "
            "we do not harden the wrong relation."
        ),
        "next_proof_move": (
            "Delay broad implementation until risks 1-4 have stable theorem "
            "statements; keep only small executable probes that expose proof bugs."
        ),
        "redesign_trigger": (
            "Executable probes contradict the theorem statement or reveal a "
            "missing public binding."
        ),
        "current_artifacts": [
            "tools/parameter-search/src/production_reference.rs",
        ],
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_proof_risk_burn_down_plan.v1",
        "status": "risk_first_proof_order_pinned",
        "principle": (
            "Do proof work in the order most likely to expose a redesign: "
            "extractability/soundness, no-export, norm/range semantics, public "
            "binding composition, then QROM parameters and implementation KATs."
        ),
        "implementation_rule": (
            "Do not build broad verifier machinery until the theorem statements "
            "for risks 1-4 are stable. Small executable probes are allowed when "
            "they directly test a proof assumption or expose circularity."
        ),
        "risks": RISKS,
        "immediate_next": (
            "Define the concrete two-view column-profile enum consumed by "
            "Path2TransparentDescentPublicViewV1: role ids, column-set ids, "
            "mask-domain labels, public/hidden classes, and profile-version "
            "rejection rules."
        ),
        "non_claims": [
            "this does not prove transparent proof soundness",
            "this does not select final transparent proof parameters",
            "this does not mark implementation work complete",
            "this does not make Path 2 deployable",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Proof Risk Burn-Down Plan",
        "",
        "Status: risk-first proof order pinned.",
        "",
        "## Principle",
        "",
        model["principle"],
        "",
        "## Implementation Rule",
        "",
        model["implementation_rule"],
        "",
        "## Risk Order",
        "",
        "| rank | risk id | risk | question | next proof move | redesign trigger |",
        "|---:|---|---|---|---|---|",
    ]
    for item in model["risks"]:
        lines.append(
            "| {rank} | `{id}` | {risk} | {question} | {next_proof_move} | "
            "{redesign_trigger} |".format(**item)
        )
    lines.extend(["", "## Current Artifacts", ""])
    for item in model["risks"]:
        lines.append(f"- `{item['id']}`:")
        for artifact in item["current_artifacts"]:
            lines.append(f"  - `{artifact}`")
    lines.extend(["", "## Immediate Next", "", model["immediate_next"], "", "## Non-Claims", ""])
    for item in model["non_claims"]:
        lines.append(f"- {item}")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    model = build_model()
    OUT_JSON.write_text(json.dumps(model, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(write_markdown(model))
    print(json.dumps({"json": str(OUT_JSON), "markdown": str(OUT_MD)}, indent=2))


if __name__ == "__main__":
    main()
