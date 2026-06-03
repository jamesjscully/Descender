#!/usr/bin/env python3
"""PTC-002 response-link attack sanity probe.

This probe is deliberately not a proof object.  The standalone exact verifier
and its certificate remain authoritative for the selected [0,3], bound-96
response-link exclusion.  The goal here is reviewer-shaped negative testing:
malformed certificate inputs, local reduction sweeps, deterministic random
basis perturbations, and exact validation/rejection of nearby bad coordinate
variants.
"""

from __future__ import annotations

import argparse
import copy
import json
import math
import random
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import verify_response_link_bounded_kernel_certificate as certver


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_CERTIFICATE = (
    READINESS_DIR
    / "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"
)
DEFAULT_JSON = (
    READINESS_DIR
    / "pre_third_party_response_link_attack_sanity_probe_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "pre_third_party_response_link_attack_sanity_probe_n32_q12289.md"
)
SELECTED_COORDS = [0, 3]
BOUND = 96


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def parse_blocks(text: str) -> list[int]:
    if not text.strip():
        return []
    return [int(item) for item in text.split(",") if item.strip()]


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def validate_payload_policy(payload: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if payload.get("q") != certver.Q:
        errors.append("q_mismatch")
    if payload.get("coords") != SELECTED_COORDS:
        errors.append("coords_not_selected_0_3")
    if payload.get("bound") != BOUND:
        errors.append("bound_not_96")
    basis = payload.get("reduced_basis")
    if not isinstance(basis, list):
        errors.append("reduced_basis_not_list")
    elif len(basis) != certver.COLS:
        errors.append("reduced_basis_row_count")
    elif any(not isinstance(row, list) or len(row) != certver.COLS for row in basis):
        errors.append("reduced_basis_shape")
    return errors


def verifier_rejection_reason(
    payload: dict[str, Any], matrix: list[list[int]], canonical_basis: list[list[int]]
) -> dict[str, Any]:
    policy_errors = validate_payload_policy(payload)
    try:
        coords = [int(value) for value in payload["coords"]]
        reduced_basis = [[int(value) for value in row] for row in payload["reduced_basis"]]
        validation = certver.verify_embedded_basis(
            matrix, coords, canonical_basis, reduced_basis
        )
    except Exception as exc:
        return {
            "accepted": False,
            "policy_errors": policy_errors,
            "verifier_stage": "parse_or_shape",
            "exception_type": type(exc).__name__,
            "exception_message": str(exc),
        }
    return {
        "accepted": not policy_errors and bool(validation.get("valid")),
        "policy_errors": policy_errors,
        "verifier_stage": "embedded_basis",
        "validation": validation,
    }


def malformed_certificate_tests(
    certificate: dict[str, Any],
    matrix: list[list[int]],
    canonical_basis: list[list[int]],
) -> list[dict[str, Any]]:
    tests: list[tuple[str, dict[str, Any]]] = []

    missing_basis = copy.deepcopy(certificate)
    missing_basis.pop("reduced_basis", None)
    tests.append(("missing_reduced_basis", missing_basis))

    short_row = copy.deepcopy(certificate)
    short_row["reduced_basis"] = copy.deepcopy(certificate["reduced_basis"])
    short_row["reduced_basis"][0] = short_row["reduced_basis"][0][:-1]
    tests.append(("short_reduced_basis_row", short_row))

    relation_mutation = copy.deepcopy(certificate)
    relation_mutation["reduced_basis"] = copy.deepcopy(certificate["reduced_basis"])
    relation_mutation["reduced_basis"][0][0] += 1
    tests.append(("single_entry_relation_mutation", relation_mutation))

    determinant_mutation = copy.deepcopy(certificate)
    determinant_mutation["reduced_basis"] = copy.deepcopy(certificate["reduced_basis"])
    determinant_mutation["reduced_basis"][1] = determinant_mutation["reduced_basis"][0][:]
    tests.append(("duplicate_row_determinant_zero", determinant_mutation))

    coord_substitution = copy.deepcopy(certificate)
    coord_substitution["coords"] = [0, 2]
    tests.append(("coordinate_pair_substitution_0_2", coord_substitution))

    duplicate_coord = copy.deepcopy(certificate)
    duplicate_coord["coords"] = [0, 0]
    tests.append(("duplicate_coordinate_pair_0_0", duplicate_coord))

    loose_bound = copy.deepcopy(certificate)
    loose_bound["bound"] = BOUND + 1
    tests.append(("loosened_bound_97", loose_bound))

    wrong_q = copy.deepcopy(certificate)
    wrong_q["q"] = certver.Q + 2
    tests.append(("wrong_q_field", wrong_q))

    rows = []
    for name, payload in tests:
        rejection = verifier_rejection_reason(payload, matrix, canonical_basis)
        rows.append(
            {
                "name": name,
                "accepted": rejection["accepted"],
                "expected": "reject",
                **rejection,
            }
        )
    return rows


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


def candidate_stats(
    matrix: list[list[int]], coords: list[int], vector: list[int], bound: int
) -> dict[str, Any]:
    stats = certver.vector_stats(vector)
    relation = certver.row_satisfies_relation(matrix, coords, vector)
    inside_box = bool(any(vector)) and relation and stats["coeff_bound"] <= bound
    return {
        **stats,
        "euclidean_norm": math.sqrt(stats["euclidean_squared"]),
        "relation_satisfied": relation,
        "inside_response_difference_box": inside_box,
        "prefix": vector[:8],
        "vector": vector,
    }


def shortest_row(
    rows: list[list[int]], matrix: list[list[int]], coords: list[int], bound: int
) -> dict[str, Any]:
    return min(
        (
            candidate_stats(matrix, coords, row, bound)
            for row in rows
            if any(value != 0 for value in row)
        ),
        key=lambda item: (
            item["inside_response_difference_box"] is False,
            item["coeff_bound"],
            item["euclidean_squared"],
            item["l1_weight"],
        ),
    )


def perturb_basis(
    basis: list[list[int]], seed: int, operations: int = 96
) -> list[list[int]]:
    rng = random.Random(seed)
    rows = [row[:] for row in basis]
    for _ in range(operations):
        op = rng.randrange(3)
        i = rng.randrange(len(rows))
        j = rng.randrange(len(rows))
        if op == 0 and i != j:
            k = rng.choice([-2, -1, 1, 2])
            rows[i] = [left + k * right for left, right in zip(rows[i], rows[j])]
        elif op == 1 and i != j:
            rows[i], rows[j] = rows[j], rows[i]
        else:
            rows[i] = [-value for value in rows[i]]
    return rows


def run_reduction(
    name: str,
    basis: list[list[int]],
    matrix: list[list[int]],
    coords: list[int],
    bound: int,
    block_size: int | None,
) -> dict[str, Any]:
    from fpylll import BKZ, LLL

    lattice = integer_matrix_from_rows(basis)
    started = time.monotonic()
    LLL.reduction(lattice)
    if block_size is not None:
        BKZ.reduction(lattice, BKZ.Param(block_size=block_size))
    rows = rows_from_integer_matrix(lattice)
    candidate = shortest_row(rows, matrix, coords, bound)
    return {
        "name": name,
        "block_size": block_size,
        "wall_time_seconds": round(time.monotonic() - started, 6),
        "shortest_reduced_basis_row": candidate,
        "found_box_vector": bool(candidate["inside_response_difference_box"]),
    }


def reduction_sweeps(
    basis: list[list[int]],
    matrix: list[list[int]],
    coords: list[int],
    bound: int,
    blocks: list[int],
    perturbation_seeds: list[int],
) -> dict[str, Any]:
    reductions: list[dict[str, Any]] = []
    errors: list[dict[str, str]] = []
    try:
        reductions.append(run_reduction("selected_basis_LLL", basis, matrix, coords, bound, None))
        for block in blocks:
            reductions.append(
                run_reduction(
                    f"selected_basis_BKZ_{block}", basis, matrix, coords, bound, block
                )
            )
        for seed in perturbation_seeds:
            perturbed = perturb_basis(basis, seed)
            reductions.append(
                run_reduction(
                    f"perturbed_seed_{seed}_BKZ_{max(blocks) if blocks else 'LLL'}",
                    perturbed,
                    matrix,
                    coords,
                    bound,
                    max(blocks) if blocks else None,
                )
            )
    except Exception as exc:  # pragma: no cover - depends on local fpylll behavior
        errors.append({"type": type(exc).__name__, "message": str(exc)})

    found = [item for item in reductions if item.get("found_box_vector")]
    coeff_bounds = [
        int(item["shortest_reduced_basis_row"]["coeff_bound"]) for item in reductions
    ]
    return {
        "fpylll_reductions_attempted": bool(reductions),
        "blocks_requested": blocks,
        "perturbation_seeds": perturbation_seeds,
        "runs": reductions,
        "errors": errors,
        "found_box_vector": bool(found),
        "best_coeff_bound_seen": min(coeff_bounds) if coeff_bounds else None,
        "strongest_available_block_used": max(blocks) if blocks else None,
    }


def bad_coordinate_variant_checks(matrix: list[list[int]]) -> dict[str, Any]:
    red_team_path = READINESS_DIR / "red_team_cryptoanalysis_n32_q12289.json"
    if not red_team_path.exists():
        return {"source": rel(red_team_path), "available": False, "checks": []}
    red_team = load_json(red_team_path)
    witnesses = red_team.get("exact_bad_variant_witnesses", [])
    checks = []
    for witness in witnesses:
        vector = [int(value) for value in witness["vector"]]
        variant_coords = [int(value) for value in witness["coords"]]
        variant_stats = candidate_stats(matrix, variant_coords, vector, BOUND)
        selected_stats = candidate_stats(matrix, SELECTED_COORDS, vector, BOUND)
        checks.append(
            {
                "variant_coords": variant_coords,
                "variant_accepts_exactly": bool(
                    variant_stats["inside_response_difference_box"]
                ),
                "selected_0_3_accepts_same_vector": bool(
                    selected_stats["inside_response_difference_box"]
                ),
                "selected_0_3_rejects_same_vector": not bool(
                    selected_stats["inside_response_difference_box"]
                ),
                "variant_coeff_bound": variant_stats["coeff_bound"],
                "selected_coord_values": {
                    str(coord): vector[coord] for coord in SELECTED_COORDS
                },
                "prefix": vector[:8],
            }
        )
    return {
        "source": rel(red_team_path),
        "available": True,
        "checks": checks,
        "all_bad_variant_witnesses_exact": all(
            item["variant_accepts_exactly"] for item in checks
        ),
        "all_bad_variant_witnesses_rejected_by_selected_0_3": all(
            item["selected_0_3_rejects_same_vector"] for item in checks
        ),
    }


def build_payload(args: argparse.Namespace) -> dict[str, Any]:
    certificate = load_json(args.certificate)
    matrix = certver.full_response_matrix()
    canonical_basis, rank, kernel_dimension = certver.canonical_intersection_basis(
        matrix, SELECTED_COORDS
    )
    reduced_basis = [[int(value) for value in row] for row in certificate["reduced_basis"]]
    validation = certver.verify_embedded_basis(
        matrix, SELECTED_COORDS, canonical_basis, reduced_basis
    )
    expected_fields = {
        "matrix_transcript_sha256": certver.matrix_transcript_sha256(matrix),
        "combined_constraint_rank": rank,
        "combined_kernel_dimension": kernel_dimension,
        "canonical_basis_sha256": validation.get("canonical_basis_sha256"),
        "reduced_basis_sha256": validation.get("reduced_basis_sha256"),
        "status": "infeasible",
        "euclidean_cover_radius_squared": certver.COLS * BOUND * BOUND,
    }
    expected_mismatches = [
        {"field": key, "expected": expected, "actual": certificate.get(key)}
        for key, expected in expected_fields.items()
        if certificate.get(key) != expected
    ]

    blocks = parse_blocks(args.bkz_blocks)
    perturbation_seeds = [int(item) for item in args.perturbation_seeds.split(",") if item]
    reduction_report = reduction_sweeps(
        reduced_basis, matrix, SELECTED_COORDS, BOUND, blocks, perturbation_seeds
    )
    malformed = malformed_certificate_tests(certificate, matrix, canonical_basis)
    bad_variants = bad_coordinate_variant_checks(matrix)
    malformed_accepts = [item for item in malformed if item["accepted"]]

    return {
        "schema": "Descender.pre_third_party_response_link.attack_sanity_probe.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "worker": "PTC-002",
        "campaign": "bounded_response_link_kernel",
        "target": {
            "n": 32,
            "q": certver.Q,
            "matrix_rows": certver.ROWS,
            "matrix_cols": certver.COLS,
            "selected_response_link_coords": SELECTED_COORDS,
            "response_difference_bound": BOUND,
        },
        "certificate": {
            "path": rel(args.certificate),
            "file_sha256": certver.file_sha256(args.certificate),
            "canonical_json_sha256": certver.canonical_json_sha256(certificate),
            "matrix_transcript_sha256": certver.matrix_transcript_sha256(matrix),
            "embedded_basis_validation": validation,
            "expected_field_mismatches": expected_mismatches,
            "interpretation": "Exact bounded enumeration remains delegated to the checked certificate/verifier replay.",
        },
        "malformed_certificate_inputs": {
            "tests": malformed,
            "accepted_unexpectedly": [item["name"] for item in malformed_accepts],
            "all_rejected": not malformed_accepts,
        },
        "random_basis_and_bkz_sweep": reduction_report,
        "bad_coordinate_variant_rejection": bad_variants,
        "verdict": {
            "selected_certificate_metadata_matches": not expected_mismatches
            and bool(validation.get("valid")),
            "malformed_inputs_fail_closed": not malformed_accepts,
            "local_reductions_found_selected_box_vector": reduction_report[
                "found_box_vector"
            ],
            "bad_coordinate_witnesses_rejected_by_selected_link": bad_variants.get(
                "all_bad_variant_witnesses_rejected_by_selected_0_3"
            ),
            "proof_claim_scope": (
                "No additional proof is claimed here; the existing exact certificate "
                "is the only proof object for the selected bounded-kernel statement."
            ),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    verdict = payload["verdict"]
    reductions = payload["random_basis_and_bkz_sweep"]
    malformed = payload["malformed_certificate_inputs"]
    bad_variants = payload["bad_coordinate_variant_rejection"]
    lines = [
        "# Pre-Third-Party Response-Link Attack Sanity Probe",
        "",
        "PTC-002 probe for the bounded response-link kernel campaign.  This is",
        "attack-sanity evidence only; the exact certificate verifier remains the",
        "proof object for the selected `[0,3]`, bound-96 claim.",
        "",
        "## Verdict",
        "",
        f"- selected certificate metadata matches: `{verdict['selected_certificate_metadata_matches']}`",
        f"- malformed inputs fail closed: `{verdict['malformed_inputs_fail_closed']}`",
        f"- local reductions found selected box vector: `{verdict['local_reductions_found_selected_box_vector']}`",
        f"- bad coordinate witnesses rejected by selected link: `{verdict['bad_coordinate_witnesses_rejected_by_selected_link']}`",
        f"- proof scope: {verdict['proof_claim_scope']}",
        "",
        "## Certificate Verifier Boundary",
        "",
        f"- certificate: `{payload['certificate']['path']}`",
        f"- matrix transcript: `{payload['certificate']['matrix_transcript_sha256']}`",
        f"- embedded basis valid: `{payload['certificate']['embedded_basis_validation'].get('valid')}`",
        f"- expected field mismatches: `{len(payload['certificate']['expected_field_mismatches'])}`",
        "",
        "## Malformed Certificate Inputs",
        "",
        f"- tests: `{len(malformed['tests'])}`",
        f"- all rejected: `{malformed['all_rejected']}`",
        "",
        "| input | accepted | policy errors | verifier reason |",
        "|---|:---:|---|---|",
    ]
    for item in malformed["tests"]:
        reason = item.get("validation", {}).get("reason") or item.get("exception_type") or ""
        lines.append(
            f"| `{item['name']}` | {item['accepted']} | `{item['policy_errors']}` | `{reason}` |"
        )
    lines.extend(
        [
            "",
            "## BKZ And Perturbation Sweep",
            "",
            f"- blocks requested: `{reductions['blocks_requested']}`",
            f"- perturbation seeds: `{reductions['perturbation_seeds']}`",
            f"- strongest available block used: `{reductions['strongest_available_block_used']}`",
            f"- best coefficient bound seen: `{reductions['best_coeff_bound_seen']}`",
            f"- selected box vector found: `{reductions['found_box_vector']}`",
            "",
            "| run | block | coeff bound | norm | inside selected box |",
            "|---|---:|---:|---:|:---:|",
        ]
    )
    for row in reductions["runs"]:
        candidate = row["shortest_reduced_basis_row"]
        lines.append(
            f"| `{row['name']}` | {row['block_size']} | "
            f"{candidate['coeff_bound']} | {candidate['euclidean_norm']:.3f} | "
            f"{candidate['inside_response_difference_box']} |"
        )
    lines.extend(
        [
            "",
            "## Bad Coordinate Variants",
            "",
            f"- source: `{bad_variants['source']}`",
            f"- all variant witnesses exact: `{bad_variants.get('all_bad_variant_witnesses_exact')}`",
            f"- all rejected by selected `[0,3]`: `{bad_variants.get('all_bad_variant_witnesses_rejected_by_selected_0_3')}`",
            "",
            "| variant coords | variant accepts | selected accepts same vector | selected coord values |",
            "|---|:---:|:---:|---|",
        ]
    )
    for item in bad_variants.get("checks", []):
        lines.append(
            f"| `{item['variant_coords']}` | {item['variant_accepts_exactly']} | "
            f"{item['selected_0_3_accepts_same_vector']} | `{item['selected_coord_values']}` |"
        )
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--certificate", type=Path, default=DEFAULT_CERTIFICATE)
    parser.add_argument("--write-json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--write-md", type=Path, default=DEFAULT_MD)
    parser.add_argument("--bkz-blocks", default="10,15,20,23")
    parser.add_argument("--perturbation-seeds", default="1701,1702,1703,1704")
    args = parser.parse_args()

    payload = build_payload(args)
    args.write_json.parent.mkdir(parents=True, exist_ok=True)
    args.write_json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    args.write_md.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": rel(args.write_json),
                "markdown": rel(args.write_md),
                "status": "written",
                "local_reductions_found_selected_box_vector": payload["verdict"][
                    "local_reductions_found_selected_box_vector"
                ],
                "malformed_inputs_fail_closed": payload["verdict"][
                    "malformed_inputs_fail_closed"
                ],
            },
            indent=2,
            sort_keys=True,
        )
    )
    if payload["verdict"]["local_reductions_found_selected_box_vector"]:
        raise SystemExit(1)
    if not payload["verdict"]["malformed_inputs_fail_closed"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
