#!/usr/bin/env python3
"""Generate account-semantics branch analysis for quotient-level links.

This Workstream D sidecar consumes the Milestone 2 hiding result and records
what account object can still make sense after public link rows fail the hiding
screen.  It is descriptive: it does not change the deployable proof path.
"""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
HIDING_REVIEW = READINESS_DIR / "quotient_level_link_hiding_review.json"
PROGRAM = READINESS_DIR / "quotient_level_link_research_program.md"
DEFAULT_JSON = READINESS_DIR / "quotient_level_link_account_semantics.json"
DEFAULT_MD = READINESS_DIR / "quotient_level_link_account_semantics.md"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def candidate_rows(hiding: dict[str, Any]) -> list[dict[str, Any]]:
    top_rejections = [
        row
        for row in hiding["candidates"]
        if row["priority"] == "top_exact_coordinate_shortlist"
    ]
    best_margin = max(row["bounded_kernel_margin"] for row in top_rejections)
    return [
        {
            "candidate": "current-reviewed-split-account-link",
            "account_object": (
                "witness commitment + randomizer commitment + deterministic "
                "[0,3] response-link code"
            ),
            "link_visibility": "response-link code public in account state; hidden representatives committed separately",
            "account_bytes": 68,
            "spend_bytes_delta": 0,
            "bounded_kernel_margin": 43,
            "hiding_status": "reviewed_current_control",
            "support_floor_bits": 133,
            "proof_obligation_delta": "baseline",
            "status": "keep_as_deployable_control",
            "reason": (
                "This is the existing reviewed path.  The sampler is not "
                "conditioned on public quotient-level link values; account "
                "binding and bounded-kernel exclusion remain separate proof "
                "objects."
            ),
        },
        {
            "candidate": "public-quotient-link-top-coordinate-shortlist",
            "account_object": (
                "semantic quotient plus public/account-visible two-row link "
                "value"
            ),
            "link_visibility": "public/account-visible quotient data",
            "account_bytes": 4,
            "spend_bytes_delta": 0,
            "bounded_kernel_margin": best_margin,
            "hiding_status": "rejected_public_link_hiding",
            "support_floor_bits": max(
                row["sampler_replay"]["public_link_support_floor_bits"]
                for row in top_rejections
            ),
            "proof_obligation_delta": "would merge kernel exclusion but fails hiding",
            "status": "rejected",
            "reason": (
                "Candidate-specific sampler replays show support floors of "
                "106-108 bits once the link rows are public conditioning data, "
                "below the 128-bit target."
            ),
        },
        {
            "candidate": "hidden-commitment-quotient-link",
            "account_object": (
                "commitments to link(witness) and link(randomizer), with a "
                "non-public proof that response link value equals the committed "
                "linear combination"
            ),
            "link_visibility": "hidden behind binding/hiding commitments",
            "account_bytes": None,
            "spend_bytes_delta": None,
            "bounded_kernel_margin": best_margin,
            "hiding_status": "plausible_if_commitment_hides_link_values",
            "support_floor_bits": 133,
            "proof_obligation_delta": (
                "replace public-link conditioning with commitment hiding, "
                "binding extraction, non-public-projection proof, and bounded "
                "kernel equality over opened link values"
            ),
            "status": "research_candidate",
            "reason": (
                "This is the only branch that can preserve the current sampler "
                "surface while using the stronger exact-passed link rows.  It "
                "does not save state bytes unless the commitment object is "
                "smaller or already present; its value is proof unification, not "
                "a current compression claim."
            ),
        },
    ]


def build_payload() -> dict[str, Any]:
    hiding = load_json(HIDING_REVIEW)
    rows = candidate_rows(hiding)
    hidden = next(row for row in rows if row["candidate"] == "hidden-commitment-quotient-link")
    return {
        "schema": "Descender.quotient_level_link.account_semantics.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "firebreak": {
            "touches_deployable_path": False,
            "touches_current_verifier": False,
            "touches_current_proof_path": False,
        },
        "inputs": {
            "hiding_review": rel(HIDING_REVIEW),
            "program_state": rel(PROGRAM),
        },
        "summary": {
            "public_quotient_link_status": "rejected_by_hiding_support",
            "hidden_commitment_link_status": hidden["status"],
            "recommended_branch": "hidden-commitment-quotient-link",
            "deployment_recommendation": "keep_current_reviewed_split_design",
        },
        "account_object_candidates": rows,
        "hidden_commitment_requirements": [
            {
                "requirement": "commitment_hiding",
                "meaning": (
                    "The account object must not reveal link(w) or link(r), "
                    "otherwise the sampler is conditioned on public link rows "
                    "and the support floor falls below target."
                ),
            },
            {
                "requirement": "commitment_binding_or_extraction",
                "meaning": (
                    "A successful opening must determine the committed link "
                    "values well enough to apply bounded-kernel soundness, or "
                    "produce a named binding break."
                ),
            },
            {
                "requirement": "non_public_projection",
                "meaning": (
                    "The link equation proof cannot be a public projection of "
                    "the sigma equation; that shortcut is already known to add "
                    "no security."
                ),
            },
            {
                "requirement": "homomorphic_or_linear_link_proof",
                "meaning": (
                    "The verifier needs evidence for link(z) = link(r) + c * "
                    "link(w) without learning the hidden link values."
                ),
            },
            {
                "requirement": "sampler_surface_preservation",
                "meaning": (
                    "The hidden representative sampler may condition on the "
                    "semantic quotient, canonical packing, fiber-state equality, "
                    "and bounds, but not on revealed link rows or active pads."
                ),
            },
        ],
        "lean_facing_shape": {
            "structure_name": "HiddenCommittedQuotientLink.Interface",
            "fields": [
                "link : R -> L",
                "commit : L -> Nonce -> Commitment",
                "hiding : commitment view does not condition the representative sampler on L",
                "binding_or_break : two openings of one commitment imply equal link value or BindingBreak",
                "link_equation_proof : accepts proof of committed link(z) = committed link(r) + c * committed link(w)",
                "non_public_projection : link equation acceptance is not implied by the public sigma equation alone",
                "bounded_kernel : ker(semantic) cap ker(link) cap responseBox = {0} or LinkKernelBreak",
            ],
            "target_theorem": (
                "accepted + hidden_committed_link + no binding/link/projection/"
                "hiding breaks -> account-backed quotient witness"
            ),
        },
        "next_required_work": [
            "Record byte/state possibilities for hash commitment, Pedersen-style homomorphic commitment, and existing account commitment reuse.",
            "Compare each commitment option against the sidecar HiddenCommittedQuotientLink.Interface theorem shape.",
            "Reject the branch if the proof object requires public link values or consumes more state than the current 68-byte account object without proof simplification.",
            "Instantiate the interface abstractly against coordinate-link-1-14 only after a commitment option survives the byte/state screen.",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    summary = payload["summary"]
    lines = [
        "# Quotient-Level Link Account Semantics",
        "",
        "Status: hidden-commitment branch selected for research; current",
        "deployable split design remains the control.",
        "",
        "This sidecar artifact does not change the current q12289 verifier, proof",
        "path, byte target, or implementation.",
        "",
        "## Summary",
        "",
        "| item | value |",
        "|---|---|",
        f"| public quotient link | `{summary['public_quotient_link_status']}` |",
        f"| hidden commitment link | `{summary['hidden_commitment_link_status']}` |",
        f"| recommended research branch | `{summary['recommended_branch']}` |",
        f"| deployment recommendation | `{summary['deployment_recommendation']}` |",
        "",
        "## Account Object Candidates",
        "",
        "| candidate | visibility | account bytes | kernel margin | hiding/support | status |",
        "|---|---|---:|---:|---|---|",
    ]
    for row in payload["account_object_candidates"]:
        account_bytes = "-" if row["account_bytes"] is None else str(row["account_bytes"])
        support = "-" if row["support_floor_bits"] is None else str(row["support_floor_bits"])
        lines.append(
            f"| {row['candidate']} | {row['link_visibility']} | "
            f"{account_bytes} | {row['bounded_kernel_margin']} | "
            f"{row['hiding_status']} / {support} bits | {row['status']} |"
        )
    lines.extend(["", "## Hidden Commitment Requirements", ""])
    for item in payload["hidden_commitment_requirements"]:
        lines.append(f"- `{item['requirement']}`: {item['meaning']}")
    shape = payload["lean_facing_shape"]
    lines.extend(
        [
            "",
            "## Lean-Facing Shape",
            "",
            f"- structure: `{shape['structure_name']}`",
            f"- target theorem: `{shape['target_theorem']}`",
            "",
            "Fields:",
        ]
    )
    for field in shape["fields"]:
        lines.append(f"- `{field}`")
    lines.extend(["", "## Next Required Work", ""])
    for item in payload["next_required_work"]:
        lines.append(f"- {item}")
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
                "recommended_branch": payload["summary"]["recommended_branch"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
