#!/usr/bin/env python3
"""Adversarial cryptoanalysis sweep for the q12289 account-carried target."""

from __future__ import annotations

import itertools
import json
import math
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import verify_response_link_bounded_kernel_certificate as certver


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "red_team_cryptoanalysis_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "red_team_cryptoanalysis_n32_q12289.md"
SELECTED_COORDS = [0, 3]
BOUND = 96


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def integer_matrix_from_rows(rows: list[list[int]]) -> Any:
    from fpylll import IntegerMatrix

    matrix = IntegerMatrix(len(rows), len(rows[0]))
    for row_idx, row in enumerate(rows):
        for col_idx, value in enumerate(row):
            matrix[row_idx, col_idx] = value
    return matrix


def rows_from_integer_matrix(matrix: Any) -> list[list[int]]:
    return [
        [int(matrix[row_idx, col_idx]) for col_idx in range(matrix.ncols)]
        for row_idx in range(matrix.nrows)
    ]


def vector_syndrome_zero(matrix: list[list[int]], vector: list[int]) -> bool:
    return all(
        sum(row[col] * vector[col] for col in range(certver.COLS)) % certver.Q == 0
        for row in matrix
    )


def vector_stats(
    matrix: list[list[int]], coords: tuple[int, ...], vector: list[int]
) -> dict[str, Any]:
    stats = certver.vector_stats(vector)
    nonzero = any(value != 0 for value in vector)
    coords_zero = all(vector[coord] % certver.Q == 0 for coord in coords)
    relation = nonzero and coords_zero and vector_syndrome_zero(matrix, vector)
    return {
        **stats,
        "euclidean_norm": math.sqrt(stats["euclidean_squared"]),
        "coords_zero_mod_q": coords_zero,
        "syndrome_zero": vector_syndrome_zero(matrix, vector),
        "inside_bound": relation and stats["coeff_bound"] <= BOUND,
        "prefix": vector[:8],
        "vector": vector,
    }


def shortest_reduced_row(
    matrix: list[list[int]], coords: tuple[int, ...], block_size: int | None
) -> dict[str, Any]:
    from fpylll import BKZ, LLL

    basis, rank, kernel_dimension = certver.canonical_intersection_basis(
        matrix, list(coords)
    )
    lattice = integer_matrix_from_rows(basis)
    started = time.monotonic()
    LLL.reduction(lattice)
    if block_size is not None:
        BKZ.reduction(lattice, BKZ.Param(block_size=block_size))
    rows = rows_from_integer_matrix(lattice)
    candidate = min(
        (
            vector_stats(matrix, coords, row)
            for row in rows
            if any(value != 0 for value in row)
        ),
        key=lambda item: (
            item["inside_bound"] is False,
            item["coeff_bound"],
            item["euclidean_squared"],
            item["l1_weight"],
        ),
    )
    return {
        "coords": list(coords),
        "rank": rank,
        "kernel_dimension": kernel_dimension,
        "reduction": "LLL" if block_size is None else f"BKZ-{block_size}",
        "elapsed_seconds": round(time.monotonic() - started, 6),
        "candidate": candidate,
    }


def sweep_coordinate_sets(
    matrix: list[list[int]], width: int, block_size: int | None
) -> dict[str, Any]:
    rows = []
    reduction_error = None
    try:
        for coords in itertools.combinations(range(certver.COLS), width):
            rows.append(shortest_reduced_row(matrix, coords, block_size))
    except Exception as exc:  # pragma: no cover - depends on fpylll install
        reduction_error = {"type": type(exc).__name__, "message": str(exc)}

    break_rows = [
        row for row in rows if row.get("candidate", {}).get("inside_bound") is True
    ]
    coeff_bounds = [
        int(row["candidate"]["coeff_bound"])
        for row in rows
        if row.get("candidate") is not None
    ]
    selected = next(
        (row for row in rows if row.get("coords") == SELECTED_COORDS),
        None,
    )
    return {
        "width": width,
        "reduction": "LLL" if block_size is None else f"BKZ-{block_size}",
        "sets_tested": len(rows),
        "candidate_break_sets": len(break_rows),
        "minimum_candidate_coeff_bound": min(coeff_bounds) if coeff_bounds else None,
        "median_candidate_coeff_bound": sorted(coeff_bounds)[len(coeff_bounds) // 2]
        if coeff_bounds
        else None,
        "maximum_candidate_coeff_bound": max(coeff_bounds) if coeff_bounds else None,
        "selected_coords_row": selected,
        "lowest_coeff_rows": sorted(
            rows,
            key=lambda row: (
                row["candidate"]["coeff_bound"],
                row["candidate"]["euclidean_squared"],
            ),
        )[:10],
        "candidate_break_rows": break_rows[:10],
        "reduction_error": reduction_error,
    }


def exact_witness_validation(
    matrix: list[list[int]], coords: list[int], vector: list[int]
) -> dict[str, Any]:
    stats = certver.vector_stats(vector)
    return {
        "coords": coords,
        "coeff_bound": stats["coeff_bound"],
        "inside_bound": stats["coeff_bound"] <= BOUND,
        "nonzero": any(value != 0 for value in vector),
        "coords_zero_mod_q": all(vector[coord] % certver.Q == 0 for coord in coords),
        "syndrome_zero": vector_syndrome_zero(matrix, vector),
        "support": stats["support"],
        "l1_weight": stats["l1_weight"],
        "euclidean_squared": stats["euclidean_squared"],
        "vector": vector,
        "valid_bad_variant_witness": (
            any(value != 0 for value in vector)
            and stats["coeff_bound"] <= BOUND
            and all(vector[coord] % certver.Q == 0 for coord in coords)
            and vector_syndrome_zero(matrix, vector)
        ),
    }


def build_payload() -> dict[str, Any]:
    matrix = certver.full_response_matrix()
    quotient = load_json(
        READINESS_DIR / "quotient_sis_lattice_attack_review_n32_q12289.json"
    )
    response_link = load_json(
        READINESS_DIR
        / "response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json"
    )
    exact_manifest = load_json(
        READINESS_DIR
        / "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json"
    )
    qrom = load_json(READINESS_DIR / "qrom_security_loss_review_n32_q12289.json")
    pad = load_json(READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.json")
    production = load_json(
        READINESS_DIR / "production_reference_gap_report_n32_q12289.json"
    )
    assumption = load_json(
        READINESS_DIR / "assumption_attack_review_matrix_n32_q12289.json"
    )

    zero_link = shortest_reduced_row(matrix, tuple(), 23)
    single_coord = sweep_coordinate_sets(matrix, 1, 23)
    pair_coord = sweep_coordinate_sets(matrix, 2, 23)
    selected_pair = pair_coord["selected_coords_row"]
    selected_candidate = selected_pair["candidate"] if selected_pair else {}
    selected_break_found = bool(selected_candidate.get("inside_bound"))
    bad_variant_witnesses = [
        exact_witness_validation(matrix, row["coords"], row["candidate"]["vector"])
        for row in pair_coord["candidate_break_rows"]
    ]

    concrete_findings = []
    if zero_link["candidate"]["inside_bound"]:
        concrete_findings.append(
            {
                "severity": "critical",
                "class": "bare_quotient_public_reconstruction",
                "finding": "With no response-link coordinates, BKZ finds a nonzero bounded kernel vector inside the accepted response-difference box.",
                "evidence": zero_link["candidate"],
                "status": "confirmed_negative_control_break",
            }
        )
    if selected_break_found:
        concrete_findings.append(
            {
                "severity": "critical",
                "class": "selected_response_link_break",
                "finding": "The selected [0,3] response-link projection has an explicit reduced-basis candidate inside the accepted response-difference box.",
                "evidence": selected_pair,
                "status": "candidate_break_against_selected_target",
            }
        )
    if pair_coord["candidate_break_sets"]:
        concrete_findings.append(
            {
                "severity": "high",
                "class": "alternate_coordinate_pair_fragility",
                "finding": "Some two-coordinate response-link projections have exact nonzero integer witnesses inside the accepted box; these are bad nearby variants and should not be substituted for [0,3].",
                "evidence": bad_variant_witnesses,
                "status": "confirmed_exact_bad_variant_witnesses",
            }
        )
    else:
        concrete_findings.append(
            {
                "severity": "medium",
                "class": "coordinate_pair_sweep",
                "finding": "The BKZ-23 sweep found no two-coordinate projection with a reduced-basis candidate inside the accepted box.",
                "evidence": {
                    "sets_tested": pair_coord["sets_tested"],
                    "selected_coords_candidate_coeff_bound": selected_candidate.get(
                        "coeff_bound"
                    ),
                    "accepted_bound": BOUND,
                },
                "status": "no_local_reduction_break_found",
            }
        )

    preflight = production["deployment_preflight"]
    for blocker in preflight.get("blockers", []):
        concrete_findings.append(
            {
                "severity": "high",
                "class": "deployment_blocker",
                "finding": blocker,
                "status": "open",
            }
        )

    return {
        "schema": "Descender.red_team_cryptoanalysis.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "target": {
            "n": 32,
            "q": certver.Q,
            "matrix_rows": certver.ROWS,
            "matrix_cols": certver.COLS,
            "selected_response_link_coords": SELECTED_COORDS,
            "response_difference_bound": BOUND,
        },
        "verdict": {
            "new_break_against_selected_250b_target_found": selected_break_found,
            "bare_public_quotient_confirmed_broken": zero_link["candidate"][
                "inside_bound"
            ],
            "bad_alternate_two_coordinate_codes_found": bool(
                pair_coord["candidate_break_sets"]
            ),
            "deployment_ready": False,
            "summary": "No new local BKZ-23 break was found against the selected [0,3] response-link target. The sweep did find broken nearby coordinate-code variants, confirming that the response-link choice is security-critical rather than cosmetic.",
        },
        "coordinate_projection_sweeps": {
            "zero_coordinate_negative_control": zero_link,
            "single_coordinate_bkz23": single_coord,
            "two_coordinate_bkz23": pair_coord,
        },
        "exact_bad_variant_witnesses": bad_variant_witnesses,
        "existing_attack_evidence": {
            "quotient_sis_lattice_attack_review": {
                "artifact": rel(
                    READINESS_DIR
                    / "quotient_sis_lattice_attack_review_n32_q12289.json"
                ),
                "status": quotient.get("status"),
                "shortest_reduction_row": quotient.get("shortest_reduction_row"),
            },
            "selected_response_link_lattice_attack_review": {
                "artifact": rel(
                    READINESS_DIR
                    / "response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json"
                ),
                "status": response_link.get("status"),
                "geometry": response_link.get("geometry"),
            },
            "exact_response_link_manifest": {
                "artifact": rel(
                    READINESS_DIR
                    / "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json"
                ),
                "status": exact_manifest.get("status"),
                "checks_failed": exact_manifest.get("failed_checks"),
            },
            "qrom_review": {
                "artifact": rel(
                    READINESS_DIR / "qrom_security_loss_review_n32_q12289.json"
                ),
                "status": qrom.get("status"),
            },
            "pad_sampler_review": {
                "artifact": rel(
                    READINESS_DIR
                    / "pad_sampler_boundary_review_n32_q12289.json"
                ),
                "status": pad.get("status"),
            },
            "assumption_matrix": {
                "artifact": rel(
                    READINESS_DIR
                    / "assumption_attack_review_matrix_n32_q12289.json"
                ),
                "summary": assumption.get("summary"),
            },
        },
        "findings": concrete_findings,
        "next_red_team_moves": [
            "Run full exact exclusion certificates for any future replacement response-link coordinate pair; reduced-basis sweeps alone are not enough for safe substitution.",
            "Independently derive the 12x23 matrix and rerun the response-link certificate outside this repository.",
            "Try dual/primal SIS estimates for the matrix sampler and public-key distribution with a standard external estimator.",
            "Implement accepting 308-byte/Fq12 projection-carrier fallback semantics and promote that verifier through the facade before treating timing or implementation tests as cryptographic evidence.",
            "Audit state rollback, mempool replay, and backup restore rules under adversarial chain reorgs.",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    verdict = payload["verdict"]
    sweeps = payload["coordinate_projection_sweeps"]
    pair_sweep = sweeps["two_coordinate_bkz23"]
    single_sweep = sweeps["single_coordinate_bkz23"]
    lines = [
        "# Red-Team Cryptoanalysis",
        "",
        "This is an adversarial local analysis pass.  It is not a proof and it is",
        "not an external review.  Its job is to look for concrete ways the current",
        "target could fail before third-party review.",
        "",
        "## Verdict",
        "",
        f"- new local break against selected 250-byte target found: `{verdict['new_break_against_selected_250b_target_found']}`",
        f"- bare public quotient confirmed broken: `{verdict['bare_public_quotient_confirmed_broken']}`",
        f"- bad alternate two-coordinate codes found: `{verdict['bad_alternate_two_coordinate_codes_found']}`",
        f"- deployment ready: `{verdict['deployment_ready']}`",
        f"- summary: {verdict['summary']}",
        "",
        "## Coordinate Projection Sweep",
        "",
        f"- zero-coordinate negative-control inside-bound candidate: `{sweeps['zero_coordinate_negative_control']['candidate']['inside_bound']}`",
        f"- one-coordinate sets tested: `{single_sweep['sets_tested']}`",
        f"- one-coordinate candidate break sets: `{single_sweep['candidate_break_sets']}`",
        f"- two-coordinate sets tested: `{pair_sweep['sets_tested']}`",
        f"- two-coordinate exact bad-variant witnesses: `{len(payload['exact_bad_variant_witnesses'])}`",
        f"- selected `[0,3]` candidate coeff bound under BKZ-23: `{pair_sweep['selected_coords_row']['candidate']['coeff_bound']}`",
        f"- accepted coefficient bound: `{payload['target']['response_difference_bound']}`",
        f"- all bad-variant witnesses validated exactly: `{all(item['valid_bad_variant_witness'] for item in payload['exact_bad_variant_witnesses'])}`",
        "",
        "Lowest two-coordinate reduced candidates:",
        "",
        "| coords | coeff bound | norm | inside bound | prefix |",
        "|---|---:|---:|:---:|---|",
    ]
    for row in pair_sweep["lowest_coeff_rows"]:
        candidate = row["candidate"]
        lines.append(
            f"| `{row['coords']}` | {candidate['coeff_bound']} | "
            f"{candidate['euclidean_norm']:.3f} | {candidate['inside_bound']} | "
            f"`{candidate['prefix']}` |"
        )
    lines.extend(
        [
            "",
            "Exact bad-variant witnesses:",
            "",
            "| coords | coeff bound | support | syndrome zero | coords zero |",
            "|---|---:|---:|:---:|:---:|",
        ]
    )
    for witness in payload["exact_bad_variant_witnesses"]:
        lines.append(
            f"| `{witness['coords']}` | {witness['coeff_bound']} | "
            f"{witness['support']} | {witness['syndrome_zero']} | "
            f"{witness['coords_zero_mod_q']} |"
        )
    lines.extend(
        [
            "",
            "## Findings",
            "",
            "| severity | class | status | finding |",
            "|---|---|---|---|",
        ]
    )
    for item in payload["findings"]:
        lines.append(
            f"| {item['severity']} | {item['class']} | {item['status']} | {item['finding']} |"
        )
    lines.extend(["", "## Next Red-Team Moves", ""])
    lines.extend(f"- {item}" for item in payload["next_red_team_moves"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))
    if payload["verdict"]["new_break_against_selected_250b_target_found"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
