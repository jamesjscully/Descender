#!/usr/bin/env python3
"""Generate the matrix/keygen estimator memo."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
QUOTIENT_SIS_ATTACK_REVIEW = (
    READINESS_DIR / "quotient_sis_lattice_attack_review_n32_q12289.json"
)
DEFAULT_JSON = READINESS_DIR / "matrix_keygen_estimator_memo_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "matrix_keygen_estimator_memo_n32_q12289.md"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    quotient = load_json(QUOTIENT_SIS_ATTACK_REVIEW)
    statement = quotient.get("statement", {})
    geometry = quotient.get("geometry", {})
    shortest = quotient.get("shortest_reduction_row", {})
    sparse = quotient.get("sparse_signed_relation_probe", {})
    return {
        "schema": "Descender.matrix_keygen_estimator_memo.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source_artifact": str(QUOTIENT_SIS_ATTACK_REVIEW.relative_to(ROOT)),
        "matrix_sampler": {
            "q": statement.get("q", 12_289),
            "rows": statement.get("matrix_rows", 12),
            "cols": statement.get("matrix_cols", 23),
            "domain": statement.get(
                "matrix_domain",
                "Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1",
            ),
            "entry_sampler": (
                "SHA-256 rejection sampling over domain, row, column, and "
                "u32 little-endian counter"
            ),
            "rejection_cutoff": 61_445,
            "matrix_transcript_sha256": statement.get("matrix_transcript_sha256"),
            "rank": statement.get("rank"),
            "kernel_dimension": statement.get("kernel_dimension"),
        },
        "local_lattice_evidence": {
            "integer_lattice_dimension": geometry.get("integer_lattice_dimension"),
            "determinant_abs": geometry.get("determinant_abs"),
            "determinant_root": geometry.get("determinant_root"),
            "gaussian_heuristic_norm": geometry.get("gaussian_heuristic_norm"),
            "kernel_basis_sha256": geometry.get("kernel_basis_sha256"),
            "reductions_run": [
                {
                    "name": item.get("name"),
                    "block_size": item.get("block_size"),
                    "wall_time_seconds": item.get("wall_time_seconds"),
                }
                for item in quotient.get("reductions", [])
            ],
        },
        "negative_control_vector": {
            "coeff_bound": shortest.get("coeff_bound"),
            "euclidean_norm": shortest.get("euclidean_norm"),
            "l1_weight": shortest.get("l1_weight"),
            "inside_response_difference_bound_96": shortest.get(
                "inside_response_difference_bound_96"
            ),
            "response_link_coords_0_3_zero_mod_q": shortest.get(
                "response_link_coords_0_3_zero_mod_q"
            ),
            "caught_by_response_link_code_0_3": shortest.get(
                "caught_by_response_link_code_0_3"
            ),
            "vector": shortest.get("vector"),
        },
        "sparse_probe": {
            "relation_space": sparse.get("relation_space"),
            "max_support": sparse.get("max_support"),
            "coefficient_bound": sparse.get("coefficient_bound"),
            "candidates_tested": sparse.get("candidates_tested"),
            "zero_relations_found": sparse.get("zero_relations_found"),
            "shortest_nonzero_syndrome": sparse.get("shortest_nonzero_syndrome"),
        },
        "estimator_scope": {
            "local_result": (
                "Local LLL/BKZ and sparse probes are sanity checks only. They "
                "confirm the bare quotient has a short bounded kernel vector "
                "that must be caught by the account response-link code."
            ),
            "not_claimed": [
                "concrete SIS hardness estimate",
                "proof of keygen/public-view pseudorandomness",
                "knowledge soundness of the account-carried proof",
                "security of the bare quotient verifier",
            ],
        },
        "required_external_review": [
            "Independently reconstruct the SHA-256 rejection-sampled matrix.",
            "Check rejection-sampling bias for each q entry under cutoff 61445.",
            "Run an external lattice-estimator/SIS cost analysis for the exact 12x23 public matrix relation and accepted-response constraints.",
            "Confirm the short bare-quotient vector is treated only as a negative control and does not survive the [0,3] response-link code.",
            "State clearly whether this small fixed-dimension matrix should be treated as a structural proof object rather than a standard asymptotic SIS hardness claim.",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    matrix = payload["matrix_sampler"]
    lattice = payload["local_lattice_evidence"]
    vector = payload["negative_control_vector"]
    sparse = payload["sparse_probe"]
    lines = [
        "# Matrix/Keygen Estimator Memo",
        "",
        "This memo fixes the local evidence and external review scope for the",
        "q12289 matrix/keygen security-estimator assumption.  It is not itself a",
        "concrete SIS hardness estimate.",
        "",
        "## Matrix Sampler",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- source artifact: `{payload['source_artifact']}`",
        f"- q: `{matrix['q']}`",
        f"- shape: `{matrix['rows']} x {matrix['cols']}`",
        f"- domain: `{matrix['domain']}`",
        f"- entry sampler: `{matrix['entry_sampler']}`",
        f"- rejection cutoff: `{matrix['rejection_cutoff']}`",
        f"- transcript hash: `{matrix['matrix_transcript_sha256']}`",
        f"- rank: `{matrix['rank']}`",
        f"- kernel dimension: `{matrix['kernel_dimension']}`",
        "",
        "## Local Lattice Evidence",
        "",
        f"- integer lattice dimension: `{lattice['integer_lattice_dimension']}`",
        f"- determinant root: `{lattice['determinant_root']}`",
        f"- Gaussian heuristic norm: `{lattice['gaussian_heuristic_norm']}`",
        f"- kernel basis hash: `{lattice['kernel_basis_sha256']}`",
        f"- reductions run: `{', '.join(item['name'] for item in lattice['reductions_run'])}`",
        "",
        "## Negative Control",
        "",
        f"- shortest coeff bound: `{vector['coeff_bound']}`",
        f"- inside response-difference bound 96: `{vector['inside_response_difference_bound_96']}`",
        f"- response-link coords [0,3] zero: `{vector['response_link_coords_0_3_zero_mod_q']}`",
        f"- caught by response-link code [0,3]: `{vector['caught_by_response_link_code_0_3']}`",
        "",
        "The short bare-quotient vector is expected.  It demonstrates why the",
        "bare public quotient verifier is not a deployable security claim.",
        "",
        "## Sparse Probe",
        "",
        f"- max support: `{sparse['max_support']}`",
        f"- coefficient bound: `{sparse['coefficient_bound']}`",
        f"- candidates tested: `{sparse['candidates_tested']}`",
        f"- zero relations found: `{sparse['zero_relations_found']}`",
        "",
        "## Required External Review",
        "",
    ]
    lines.extend(f"- {item}" for item in payload["required_external_review"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
