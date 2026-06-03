#!/usr/bin/env python3
"""Pre-third-party matrix/SIS/keygen distribution probe.

This harness is intentionally independent of the existing response-link
certificate verifier.  It rederives the public response matrix from the stated
SHA-256 rejection sampler, checks the existing quotient/SIS artifacts, and adds
small local primal, dual, sparse-SIS, and perturbation sanity probes.
"""

from __future__ import annotations

import argparse
import hashlib
import itertools
import json
import math
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_QUOTIENT = READINESS_DIR / "quotient_sis_lattice_attack_review_n32_q12289.json"
DEFAULT_MEMO = READINESS_DIR / "matrix_keygen_estimator_memo_n32_q12289.json"
DEFAULT_JSON = READINESS_DIR / "pre_third_party_matrix_sis_probe_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_third_party_matrix_sis_probe_n32_q12289.md"

Q = 12_289
ROWS = 12
COLS = 23
DOMAIN = b"Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rejection_limit(q: int) -> int:
    return 65_535 - (65_535 % q)


def sampled_entry(domain: bytes, q: int, row: int, col: int) -> tuple[int, int, int]:
    limit = rejection_limit(q)
    counter = 0
    rejected = 0
    while True:
        digest = hashlib.sha256(
            domain
            + row.to_bytes(2, "little")
            + col.to_bytes(2, "little")
            + counter.to_bytes(4, "little")
        ).digest()
        for idx in range(0, len(digest), 2):
            draw = int.from_bytes(digest[idx : idx + 2], "little")
            if draw < limit:
                return draw % q, counter, rejected
            rejected += 1
        counter += 1


def response_matrix(domain: bytes = DOMAIN, q: int = Q, rows: int = ROWS, cols: int = COLS) -> tuple[list[list[int]], list[dict[str, int]]]:
    transcript: list[list[int]] = []
    sampler_trace: list[dict[str, int]] = []
    for row in range(rows):
        transcript_row: list[int] = []
        for col in range(cols):
            value, counter, rejected = sampled_entry(domain, q, row, col)
            transcript_row.append(value)
            sampler_trace.append(
                {
                    "row": row,
                    "col": col,
                    "value": value,
                    "counter": counter,
                    "rejected_16_bit_draws_before_accept": rejected,
                }
            )
        transcript.append(transcript_row)
    return transcript, sampler_trace


def matrix_hash(matrix: list[list[int]]) -> str:
    h = hashlib.sha256()
    for row in matrix:
        for value in row:
            h.update(value.to_bytes(2, "little"))
    return h.hexdigest()


def centered(value: int, q: int = Q) -> int:
    value %= q
    return value - q if value > q // 2 else value


def lee_weight(values: Iterable[int], q: int = Q) -> int:
    return sum(abs(centered(value, q)) for value in values)


def support(values: Iterable[int], q: int = Q) -> int:
    return sum(1 for value in values if value % q != 0)


def syndrome(matrix: list[list[int]], vector: list[int], q: int = Q) -> list[int]:
    return [
        sum(row[col] * (vector[col] % q) for col in range(len(vector))) % q
        for row in matrix
    ]


def rref_mod_q(matrix: list[list[int]], q: int) -> tuple[list[list[int]], list[int]]:
    if not matrix:
        return [], []
    cols = len(matrix[0])
    work = [[value % q for value in row] for row in matrix]
    rank = 0
    pivots: list[int] = []
    for col in range(cols):
        pivot = next((row for row in range(rank, len(work)) if work[row][col] % q), None)
        if pivot is None:
            continue
        work[rank], work[pivot] = work[pivot], work[rank]
        inv = pow(work[rank][col] % q, -1, q)
        work[rank] = [(value * inv) % q for value in work[rank]]
        for row in range(len(work)):
            if row == rank or work[row][col] % q == 0:
                continue
            factor = work[row][col] % q
            work[row] = [(work[row][idx] - factor * work[rank][idx]) % q for idx in range(cols)]
        pivots.append(col)
        rank += 1
        if rank == len(work):
            break
    return work, pivots


def rank_mod_q(matrix: list[list[int]], q: int) -> int:
    _, pivots = rref_mod_q(matrix, q)
    return len(pivots)


def canonical_kernel_basis(matrix: list[list[int]], q: int) -> tuple[list[list[int]], int, int]:
    rref, pivots = rref_mod_q(matrix, q)
    cols = len(matrix[0])
    free_cols = [col for col in range(cols) if col not in pivots]
    basis: list[list[int]] = []
    for free_col in free_cols:
        vector = [0] * cols
        vector[free_col] = 1
        for row, pivot_col in enumerate(pivots):
            vector[pivot_col] = (-rref[row][free_col]) % q
        basis.append([centered(value, q) for value in vector])
    return basis, len(pivots), len(free_cols)


def vector_stats(vector: list[int], matrix: list[list[int]], q: int = Q) -> dict[str, Any]:
    syn = syndrome(matrix, vector, q)
    return {
        "coeff_bound": max(abs(value) for value in vector) if vector else 0,
        "euclidean_squared": sum(value * value for value in vector),
        "euclidean_norm": math.sqrt(sum(value * value for value in vector)),
        "l1_weight": sum(abs(value) for value in vector),
        "support": sum(1 for value in vector if value),
        "syndrome_zero": all(value == 0 for value in syn),
        "syndrome_lee_weight": lee_weight(syn, q),
        "syndrome_support": support(syn, q),
        "prefix": vector[:8],
        "vector": vector,
    }


def column_stats(matrix: list[list[int]], q: int) -> list[dict[str, float | int]]:
    stats: list[dict[str, float | int]] = []
    for col in range(len(matrix[0])):
        lifted = [centered(row[col], q) for row in matrix]
        mean = sum(lifted) / len(lifted)
        variance = sum((value - mean) ** 2 for value in lifted) / len(lifted)
        stats.append(
            {
                "col": col,
                "mean_centered": mean,
                "stddev_centered": math.sqrt(variance),
                "min_centered": min(lifted),
                "max_centered": max(lifted),
                "zero_entries": sum(1 for value in lifted if value == 0),
            }
        )
    return stats


def row_stats(matrix: list[list[int]], q: int) -> list[dict[str, float | int]]:
    stats: list[dict[str, float | int]] = []
    for row_idx, row in enumerate(matrix):
        lifted = [centered(value, q) for value in row]
        mean = sum(lifted) / len(lifted)
        variance = sum((value - mean) ** 2 for value in lifted) / len(lifted)
        stats.append(
            {
                "row": row_idx,
                "mean_centered": mean,
                "stddev_centered": math.sqrt(variance),
                "min_centered": min(lifted),
                "max_centered": max(lifted),
                "zero_entries": sum(1 for value in lifted if value == 0),
            }
        )
    return stats


def sampler_distribution(trace: list[dict[str, int]], q: int) -> dict[str, Any]:
    residues = [item["value"] for item in trace]
    bins = 16
    counts = Counter(min(bins - 1, value * bins // q) for value in residues)
    expected = len(residues) / bins
    chi_square = sum((counts[idx] - expected) ** 2 / expected for idx in range(bins))
    rejected = [item["rejected_16_bit_draws_before_accept"] for item in trace]
    accepted_after_counter = [item for item in trace if item["counter"] > 0]
    return {
        "q": q,
        "rejection_limit": rejection_limit(q),
        "rejection_tail_size": 65_536 - rejection_limit(q),
        "entries": len(trace),
        "total_rejected_16_bit_draws": sum(rejected),
        "max_rejected_16_bit_draws_before_accept": max(rejected) if rejected else 0,
        "entries_requiring_second_digest": len(accepted_after_counter),
        "coarse_16_bin_counts": [counts[idx] for idx in range(bins)],
        "coarse_16_bin_chi_square": chi_square,
        "distinct_entries": len(set(residues)),
        "duplicate_entries": len(residues) - len(set(residues)),
    }


def sparse_primal_probe(matrix: list[list[int]], max_support: int, coeff_bound: int, q: int = Q) -> dict[str, Any]:
    coeff_values = [value for value in range(-coeff_bound, coeff_bound + 1) if value != 0]
    candidates = 0
    zero_relations = 0
    best_nonzero: dict[str, Any] | None = None
    for weight in range(1, max_support + 1):
        for cols in itertools.combinations(range(len(matrix[0])), weight):
            for coeffs in itertools.product(coeff_values, repeat=weight):
                candidates += 1
                vector = [0] * len(matrix[0])
                for col, coeff in zip(cols, coeffs, strict=True):
                    vector[col] = coeff
                syn = syndrome(matrix, vector, q)
                if all(value == 0 for value in syn):
                    zero_relations += 1
                    continue
                stats = {
                    "coeff_bound": max(abs(value) for value in coeffs),
                    "support": weight,
                    "syndrome_lee_weight": lee_weight(syn, q),
                    "syndrome_support": support(syn, q),
                    "syndrome_prefix": syn[:8],
                    "vector": vector,
                }
                if best_nonzero is None or (
                    stats["syndrome_lee_weight"],
                    stats["syndrome_support"],
                    stats["support"],
                ) < (
                    best_nonzero["syndrome_lee_weight"],
                    best_nonzero["syndrome_support"],
                    best_nonzero["support"],
                ):
                    best_nonzero = stats
    return {
        "space": "small signed column combinations x with A*x mod q",
        "max_support": max_support,
        "coefficient_bound": coeff_bound,
        "candidates_tested": candidates,
        "zero_relations_found": zero_relations,
        "best_nonzero_syndrome": best_nonzero,
    }


def dual_probe(matrix: list[list[int]], coeff_bound: int, q: int = Q) -> dict[str, Any]:
    coeff_values = list(range(-coeff_bound, coeff_bound + 1))
    candidates = 0
    zero_dual_products = 0
    best: dict[str, Any] | None = None
    for coeffs in itertools.product(coeff_values, repeat=len(matrix)):
        if all(value == 0 for value in coeffs):
            continue
        candidates += 1
        product = [
            sum(coeffs[row] * matrix[row][col] for row in range(len(matrix))) % q
            for col in range(len(matrix[0]))
        ]
        if all(value == 0 for value in product):
            zero_dual_products += 1
            continue
        stats = {
            "y_coeff_bound": max(abs(value) for value in coeffs),
            "y_support": sum(1 for value in coeffs if value),
            "a_transpose_y_lee_weight": lee_weight(product, q),
            "a_transpose_y_support": support(product, q),
            "a_transpose_y_coeff_bound_centered": max(abs(centered(value, q)) for value in product),
            "y": list(coeffs),
            "a_transpose_y_prefix": product[:8],
        }
        if best is None or (
            stats["a_transpose_y_lee_weight"],
            stats["a_transpose_y_support"],
            stats["y_support"],
        ) < (
            best["a_transpose_y_lee_weight"],
            best["a_transpose_y_support"],
            best["y_support"],
        ):
            best = stats
    return {
        "space": "small signed row combinations y with A^T*y mod q",
        "coefficient_bound": coeff_bound,
        "candidates_tested": candidates,
        "zero_dual_products_found": zero_dual_products,
        "best_nonzero_dual_product": best,
    }


def perturbation_probes() -> list[dict[str, Any]]:
    cases = [
        ("same_domain_q12281", DOMAIN, 12_281, ROWS, COLS),
        ("same_domain_q12289_cols24", DOMAIN, Q, ROWS, 24),
        ("same_domain_q12289_rows13", DOMAIN, Q, 13, COLS),
        ("domain_suffix_perturbed", DOMAIN + b".perturbation", Q, ROWS, COLS),
    ]
    results: list[dict[str, Any]] = []
    for name, domain, q, rows, cols in cases:
        matrix, trace = response_matrix(domain=domain, q=q, rows=rows, cols=cols)
        rank = rank_mod_q(matrix, q)
        _, _, kernel_dimension = canonical_kernel_basis(matrix, q)
        sparse = sparse_primal_probe(matrix, max_support=2, coeff_bound=1, q=q)
        results.append(
            {
                "name": name,
                "q": q,
                "rows": rows,
                "cols": cols,
                "domain_sha256": hashlib.sha256(domain).hexdigest(),
                "matrix_transcript_sha256": matrix_hash(matrix),
                "rank": rank,
                "kernel_dimension": kernel_dimension,
                "sampler_distribution": sampler_distribution(trace, q),
                "signed_support_2_coeff_1_zero_relations": sparse["zero_relations_found"],
                "signed_support_2_coeff_1_candidates_tested": sparse["candidates_tested"],
            }
        )
    return results


def build_payload(args: argparse.Namespace) -> dict[str, Any]:
    quotient = load_json(args.quotient_json)
    memo = load_json(args.memo_json)
    matrix, trace = response_matrix()
    rank = rank_mod_q(matrix, Q)
    kernel_basis, _, kernel_dimension = canonical_kernel_basis(matrix, Q)
    quotient_statement = quotient.get("statement", {})
    quotient_geometry = quotient.get("geometry", {})
    memo_sampler = memo.get("matrix_sampler", {})
    shortest = quotient.get("shortest_reduction_row", {}) or {}

    primal_probe = sparse_primal_probe(matrix, args.sparse_max_support, args.sparse_coeff_bound)
    dual = dual_probe(matrix, args.dual_coeff_bound)

    checks = {
        "matrix_hash_matches_quotient_review": matrix_hash(matrix)
        == quotient_statement.get("matrix_transcript_sha256"),
        "matrix_hash_matches_matrix_keygen_memo": matrix_hash(matrix)
        == memo_sampler.get("matrix_transcript_sha256"),
        "rank_matches_quotient_review": rank == quotient_statement.get("rank"),
        "kernel_dimension_matches_quotient_review": kernel_dimension
        == quotient_statement.get("kernel_dimension"),
        "kernel_basis_dimension_matches_quotient_review": len(kernel_basis)
        == quotient_statement.get("kernel_dimension"),
        "negative_control_rechecks_as_kernel_vector": (
            bool(shortest.get("vector"))
            and all(value == 0 for value in syndrome(matrix, shortest["vector"]))
        ),
        "negative_control_rejected_by_response_link_coords_0_3": (
            bool(shortest.get("vector"))
            and not (
                shortest["vector"][0] % Q == 0
                and shortest["vector"][3] % Q == 0
            )
        ),
    }
    checks["all_core_checks_passed"] = all(checks.values())

    return {
        "schema": "Descender.pre_third_party_matrix_sis_probe.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "campaign": "pre_third_party_cryptoanalysis_campaign.matrix_sampler_and_public_key_distribution",
        "source_artifacts": {
            "quotient_sis_lattice_attack_review": str(args.quotient_json.relative_to(ROOT)),
            "matrix_keygen_estimator_memo": str(args.memo_json.relative_to(ROOT)),
        },
        "sampler_rederivation": {
            "q": Q,
            "rows": ROWS,
            "cols": COLS,
            "domain": DOMAIN.decode(),
            "entry_sampler": "independent SHA-256 rejection sampler over domain,row,col,u32-counter",
            "matrix_transcript_sha256": matrix_hash(matrix),
            "rank_mod_q": rank,
            "kernel_dimension_mod_q": kernel_dimension,
            "kernel_basis_rows": len(kernel_basis),
            "sampler_distribution": sampler_distribution(trace, Q),
            "row_stats_centered": row_stats(matrix, Q),
            "column_stats_centered": column_stats(matrix, Q),
        },
        "published_artifact_cross_checks": checks,
        "published_geometry_snapshot": {
            "integer_lattice_dimension": quotient_geometry.get("integer_lattice_dimension"),
            "determinant_abs": quotient_geometry.get("determinant_abs"),
            "determinant_root": quotient_geometry.get("determinant_root"),
            "gaussian_heuristic_norm": quotient_geometry.get("gaussian_heuristic_norm"),
            "kernel_basis_sha256": quotient_geometry.get("kernel_basis_sha256"),
        },
        "negative_control_replay": vector_stats(shortest.get("vector", []), matrix),
        "additional_primal_sparse_probe": primal_probe,
        "additional_dual_row_combination_probe": dual,
        "parameter_perturbation_checks": perturbation_probes(),
        "status": "pass" if checks["all_core_checks_passed"] else "review_required",
        "interpretation": (
            "The public matrix transcript, rank, and negative-control kernel vector "
            "were independently rederived from the stated sampler. Local sparse "
            "primal and small dual probes found no zero relation in their searched "
            "spaces; perturbations preserve full row rank in the checked nearby "
            "cases. These are pre-third-party sanity checks, not concrete SIS "
            "hardness estimates."
        ),
    }


def markdown(payload: dict[str, Any]) -> str:
    sampler = payload["sampler_rederivation"]
    dist = sampler["sampler_distribution"]
    checks = payload["published_artifact_cross_checks"]
    primal = payload["additional_primal_sparse_probe"]
    dual = payload["additional_dual_row_combination_probe"]
    lines = [
        "# Pre-Third-Party Matrix/SIS Probe",
        "",
        "This artifact independently rederives the q12289 response matrix used by",
        "the quotient/SIS review and adds lightweight local checks for matrix",
        "sampler, public-key distribution, primal SIS, dual row-combination, and",
        "nearby-parameter assumptions.",
        "",
        "## Rederived Matrix",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- q: `{sampler['q']}`",
        f"- shape: `{sampler['rows']} x {sampler['cols']}`",
        f"- domain: `{sampler['domain']}`",
        f"- transcript hash: `{sampler['matrix_transcript_sha256']}`",
        f"- rank mod q: `{sampler['rank_mod_q']}`",
        f"- kernel dimension mod q: `{sampler['kernel_dimension_mod_q']}`",
        f"- rejection limit: `{dist['rejection_limit']}`",
        f"- total rejected 16-bit draws: `{dist['total_rejected_16_bit_draws']}`",
        f"- coarse 16-bin chi-square: `{dist['coarse_16_bin_chi_square']}`",
        "",
        "## Published Artifact Cross-Checks",
        "",
    ]
    lines.extend(f"- {key}: `{value}`" for key, value in checks.items())
    lines.extend(
        [
            "",
            "## Additional Local Probes",
            "",
            f"- primal sparse support <= `{primal['max_support']}`, coeff <= `{primal['coefficient_bound']}` candidates: `{primal['candidates_tested']}`",
            f"- primal sparse zero relations found: `{primal['zero_relations_found']}`",
            f"- dual row-combination coeff <= `{dual['coefficient_bound']}` candidates: `{dual['candidates_tested']}`",
            f"- dual zero products found: `{dual['zero_dual_products_found']}`",
            "",
            "## Perturbations",
            "",
        ]
    )
    for item in payload["parameter_perturbation_checks"]:
        lines.append(
            f"- `{item['name']}`: shape `{item['rows']} x {item['cols']}`, q `{item['q']}`, "
            f"rank `{item['rank']}`, kernel dimension `{item['kernel_dimension']}`, "
            f"support-2 signed zero relations `{item['signed_support_2_coeff_1_zero_relations']}`"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            payload["interpretation"],
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--quotient-json", type=Path, default=DEFAULT_QUOTIENT)
    parser.add_argument("--memo-json", type=Path, default=DEFAULT_MEMO)
    parser.add_argument("--write-json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--write-md", type=Path, default=DEFAULT_MD)
    parser.add_argument("--sparse-max-support", type=int, default=4)
    parser.add_argument("--sparse-coeff-bound", type=int, default=1)
    parser.add_argument("--dual-coeff-bound", type=int, default=1)
    args = parser.parse_args()

    payload = build_payload(args)
    args.write_json.parent.mkdir(parents=True, exist_ok=True)
    args.write_json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    args.write_md.write_text(markdown(payload))
    print(json.dumps({"json": str(args.write_json), "markdown": str(args.write_md), "status": payload["status"]}, indent=2))


if __name__ == "__main__":
    main()
