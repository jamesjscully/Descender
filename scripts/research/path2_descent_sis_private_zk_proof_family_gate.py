#!/usr/bin/env python3
"""Generate the proof-family gate for the Path 2 private-ZK payload."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_descent_sis_private_zk_proof_family_gate.json"
OUT_MD = ROOT / "research/path2_descent_sis_private_zk_proof_family_gate.md"


REQUIREMENTS = [
    "post-quantum assumption surface",
    "zero knowledge for pad/projection/opening/range witnesses",
    "knowledge soundness or special soundness for the exact hidden-source relation",
    "range or digit constraints for q12289 semantic lanes",
    "Fiat-Shamir transform with explicit QROM accounting",
    "payload verifier can bind leaf_context_digest and creation_proof_binding_digest",
    "public transcript does not publish non-hiding C=A*v role commitments",
    "direct-mode transcript fields remain aggregation-neutral",
]

FAMILIES = [
    {
        "id": "lattice_sigma_rejection_zk",
        "status": "primary_research_candidate",
        "fit": (
            "best conceptual match for a descent/SIS relation if the proof can "
            "express linear equations plus digit/range constraints without "
            "publishing non-hiding role commitments"
        ),
        "main_risk": (
            "range/digit proof overhead and repetition may erase the byte advantage "
            "over transparent proofs"
        ),
        "next_test": (
            "instantiate a toy transcript model with private commitments, challenge "
            "response, simulator surface, and extractor assumptions"
        ),
    },
    {
        "id": "stern_like_code_lattice_zk",
        "status": "fallback_if_sigma_rejection_is_not_compact",
        "fit": (
            "mature zero-knowledge shape for small-secret / syndrome-style relations "
            "and supports proof-by-repetition reasoning"
        ),
        "main_risk": "typically larger proof sizes and more repetitions",
        "next_test": "rough byte model for the Path 2 relation and range constraints",
    },
    {
        "id": "transparent_hash_proof_fallback",
        "status": "fallback_baseline",
        "fit": (
            "already compatible with arbitrary relations and avoids delicate "
            "linear-commitment hiding questions"
        ),
        "main_risk": "likely larger account-creation payload and verifier cost",
        "next_test": "compare against the private-ZK lattice candidate once that candidate has a transcript model",
    },
    {
        "id": "small_noise_public_lwe_style_commitment",
        "status": "public_object_fallback_not_private_zk_primary",
        "fit": (
            "keeps a public algebraic object and short openings, but changes the "
            "assumption surface from SIS-only toward computational hiding"
        ),
        "main_risk": "adds a separate hiding assumption and sampler analysis burden",
        "next_test": "only pursue if private-ZK transcript proof looks too large or too hard to specify",
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_descent_sis_private_zk_proof_family_gate.v1",
        "status": "proof_family_gate_pinned",
        "requirements": REQUIREMENTS,
        "families": FAMILIES,
        "selected_next_probe": "lattice_sigma_rejection_zk",
        "selection_reason": (
            "It is the closest match to the existing descent/SIS thesis and keeps "
            "the spend signature unchanged while allowing the account-creation "
            "proof to be larger. It must be rejected quickly if range/digit proof "
            "overhead or repetition makes it worse than the transparent fallback."
        ),
        "toy_transcript_model_requirements": [
            "public statement exactly matches `path2_descent_sis_private_zk_payload_surface`",
            "public FS frame exactly matches `path2_descent_sis_private_zk_qrom_frame`",
            "private witness fields exactly match the no-export denylist",
            "extractor conclusion names pad_source_opening, projection_relation_opening, and opening_tuple",
            "simulator produces public transcript without private witness values",
            "malformed transcript classes map to named breaks",
        ],
        "go_no_go_rule": (
            "If the toy transcript model cannot state simulator and extractor "
            "assumptions without publishing non-hiding role commitments, stop the "
            "private-ZK branch and switch to the small-noise public fallback or "
            "transparent proof fallback."
        ),
        "non_claims": [
            "this does not prove the selected family is secure",
            "this does not estimate final payload bytes",
            "this does not instantiate Rust or Lean verifier code",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Descent/SIS Private-ZK Proof Family Gate",
        "",
        "Status: proof-family gate pinned.",
        "",
        "## Requirements",
        "",
    ]
    for item in model["requirements"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "## Candidate Families",
            "",
            "| id | status | fit | main risk | next test |",
            "|---|---|---|---|---|",
        ]
    )
    for item in model["families"]:
        lines.append(
            "| `{id}` | {status} | {fit} | {main_risk} | {next_test} |".format(
                **item
            )
        )
    lines.extend(
        [
            "",
            "## Selected Next Probe",
            "",
            f"`{model['selected_next_probe']}`",
            "",
            model["selection_reason"],
            "",
            "## Toy Transcript Model Requirements",
            "",
        ]
    )
    for item in model["toy_transcript_model_requirements"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "## Go/No-Go Rule",
            "",
            model["go_no_go_rule"],
            "",
            "## Non-Claims",
            "",
        ]
    )
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
