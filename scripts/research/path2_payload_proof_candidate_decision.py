#!/usr/bin/env python3
"""Select the active Path 2 payload proof candidate after comparison."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_payload_proof_candidate_decision.json"
OUT_MD = ROOT / "research/path2_payload_proof_candidate_decision.md"


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_payload_proof_candidate_decision.v1",
        "status": "active_candidate_selected_not_theorem_closed",
        "active_candidate": "transparent_hash_proof_over_path2_descent_relation",
        "decision": (
            "Use the transparent proof as the active deployable account-creation "
            "payload candidate. Keep the descent/SIS work as the semantic relation, "
            "matrix/domain framework, and assumption vocabulary, but do not use "
            "the repetition-heavy lattice-sigma transcript as the primary outer "
            "proof candidate."
        ),
        "reasons": [
            "current public C=A*v role commitments are non-hiding",
            "private lattice-sigma avoids that leak but has an optimistic 64-round floor of 16416 bytes before concrete overhead",
            "transparent planning rows for the same 80-lane / 66-row relation sit around 4896 to 8848 bytes before concrete overhead",
            "transparent proof avoids bespoke lattice ZK simulator/rejection-sampling risk for the first deployable account-creation proof",
            "spend signature bytes remain unchanged because this is account-creation/admission payload material",
        ],
        "parked_paths": [
            {
                "path": "lattice_sigma_rejection_zk",
                "status": "parked_until_non_repetition_heavy_argument_found",
                "revival_condition": (
                    "identify a concrete proof family whose byte model beats the "
                    "transparent planning rows while preserving simulator and extractor assumptions"
                ),
            },
            {
                "path": "small_noise_public_lwe_style_commitment",
                "status": "fallback_only",
                "revival_condition": (
                    "transparent proof is too costly and a public algebraic object "
                    "with a named computational hiding assumption is acceptable"
                ),
            },
        ],
        "next_requirements": [
            "turn the transparent planning model into a concrete transcript frame",
            "state transparent proof extractor/soundness assumption for the Path 2 relation",
            "state transparent proof zero-knowledge/no-export assumption",
            "add payload-specific Rust public-view serializer and reject-path KAT plan",
            "update QROM/hash ledger with transparent proof query symbols",
        ],
        "non_claims": [
            "this does not instantiate the transparent proof system",
            "this does not close proof semantics",
            "this does not claim the planning byte rows are deployable estimates",
            "this does not remove the reviewed 308-byte/Fq12 fail-closed baseline",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Payload Proof Candidate Decision",
        "",
        "Status: active candidate selected; theorem not closed.",
        "",
        f"Active candidate: `{model['active_candidate']}`.",
        "",
        "## Decision",
        "",
        model["decision"],
        "",
        "## Reasons",
        "",
    ]
    for item in model["reasons"]:
        lines.append(f"- {item}")
    lines.extend(["", "## Parked Paths", "", "| path | status | revival condition |", "|---|---|---|"])
    for item in model["parked_paths"]:
        lines.append(
            "| `{path}` | {status} | {revival_condition} |".format(**item)
        )
    lines.extend(["", "## Next Requirements", ""])
    for item in model["next_requirements"]:
        lines.append(f"- {item}")
    lines.extend(["", "## Non-Claims", ""])
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
