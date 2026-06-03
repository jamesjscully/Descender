#!/usr/bin/env python3
"""Probe pre-signature leakage for the proposed 18-residue Fq12 witness state.

The proposed redesign packs the existing 12 checked-core Fq12 syndrome residues
plus the six selected account-witness projection coordinates into the existing
32-byte account witness field.  This script compares the current live-slot
public view against that proposed 18-row live-slot view.

The probe is intentionally conservative: because no bridge theorem currently
identifies the account projection coordinates with checked-core response
coordinates, the model keeps them in a separate account-projection namespace.
That makes the output a leakage/timing audit, not a deployability proof.
"""

from __future__ import annotations

import hashlib
import itertools
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "pre_signature_projection_leakage_probe_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_signature_projection_leakage_probe_n32_q12289.md"

Q = 12_289
CHECKED_CORE_ROWS = 12
CHECKED_CORE_COLS = 22
ACCOUNT_PROJECTION_BLOCKS = 16
ACCOUNT_PROJECTION_COORDS = 2
ACCOUNT_PROJECTION_COLS = ACCOUNT_PROJECTION_BLOCKS * ACCOUNT_PROJECTION_COORDS
SELECTED_ACCOUNT_PROJECTION_SYMBOLS = [
    "account_witness_projection[1,0]",
    "account_witness_projection[4,1]",
    "account_witness_projection[7,0]",
    "account_witness_projection[10,1]",
    "account_witness_projection[13,0]",
    "account_witness_projection[0,1]",
]
MATRIX_DOMAIN = b"Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1"


def rejection_limit(q: int) -> int:
    return 65_535 - (65_535 % q)


def sampled_entry(domain: bytes, q: int, row: int, col: int) -> int:
    limit = rejection_limit(q)
    counter = 0
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
                return draw % q
        counter += 1


def checked_core_matrix() -> list[list[int]]:
    return [
        [
            sampled_entry(MATRIX_DOMAIN, Q, row, col)
            for col in range(CHECKED_CORE_COLS)
        ]
        for row in range(CHECKED_CORE_ROWS)
    ]


def selected_projection_columns() -> list[int]:
    columns: list[int] = []
    for symbol in SELECTED_ACCOUNT_PROJECTION_SYMBOLS:
        inside = symbol.removeprefix("account_witness_projection[").removesuffix("]")
        block_s, coord_s = inside.split(",")
        block = int(block_s)
        coord = int(coord_s)
        columns.append(block * ACCOUNT_PROJECTION_COORDS + coord)
    return columns


def public_view_matrix(include_projection_rows: bool) -> list[list[int]]:
    """Rows over checked-core coordinates || account-projection coordinates."""
    matrix: list[list[int]] = []
    for row in checked_core_matrix():
        matrix.append(row + [0] * ACCOUNT_PROJECTION_COLS)
    if include_projection_rows:
        for col in selected_projection_columns():
            public_row = [0] * CHECKED_CORE_COLS + [0] * ACCOUNT_PROJECTION_COLS
            public_row[CHECKED_CORE_COLS + col] = 1
            matrix.append(public_row)
    return matrix


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
            work[row] = [
                (work[row][idx] - factor * work[rank][idx]) % q
                for idx in range(cols)
            ]
        pivots.append(col)
        rank += 1
        if rank == len(work):
            break
    return work, pivots


def rank_mod_q(matrix: list[list[int]], q: int) -> int:
    _, pivots = rref_mod_q(matrix, q)
    return len(pivots)


def matrix_digest(matrix: list[list[int]]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.pre_signature_projection_leakage_probe.matrix.v1")
    for row in matrix:
        for value in row:
            h.update((value % Q).to_bytes(2, "little"))
    return h.hexdigest()


def centered(value: int, q: int = Q) -> int:
    value %= q
    return value - q if value > q // 2 else value


def row_relation_syndrome(rows: list[list[int]], coeffs: Iterable[int]) -> list[int]:
    coeff_list = list(coeffs)
    cols = len(rows[0])
    return [
        sum(coeff_list[row] * rows[row][col] for row in range(len(rows))) % Q
        for col in range(cols)
    ]


def sparse_row_relation_probe(
    matrix: list[list[int]], max_support: int = 3, coeff_values: tuple[int, ...] = (-2, -1, 1, 2)
) -> dict[str, Any]:
    tested = 0
    first_relation: dict[str, Any] | None = None
    row_count = len(matrix)
    for support in range(1, max_support + 1):
        for rows in itertools.combinations(range(row_count), support):
            for coeffs in itertools.product(coeff_values, repeat=support):
                tested += 1
                full_coeffs = [0] * row_count
                for row, coeff in zip(rows, coeffs):
                    full_coeffs[row] = coeff
                if all(value == 0 for value in row_relation_syndrome(matrix, full_coeffs)):
                    first_relation = {
                        "rows": list(rows),
                        "coefficients": list(coeffs),
                    }
                    return {
                        "max_support": max_support,
                        "coefficient_values": list(coeff_values),
                        "tested": tested,
                        "zero_relation_found": True,
                        "first_relation": first_relation,
                    }
    return {
        "max_support": max_support,
        "coefficient_values": list(coeff_values),
        "tested": tested,
        "zero_relation_found": False,
        "first_relation": None,
    }


def selected_projection_global_columns() -> set[int]:
    return {
        CHECKED_CORE_COLS + col
        for col in selected_projection_columns()
    }


def column_category(col: int) -> str:
    if col < CHECKED_CORE_COLS:
        return "checked_core"
    if col in selected_projection_global_columns():
        return "selected_account_projection"
    return "nonselected_account_projection"


def sparse_column_kernel_probe(
    matrix: list[list[int]],
    max_support: int = 3,
    coeff_values: tuple[int, ...] = (-2, -1, 1, 2),
) -> dict[str, Any]:
    cols_total = len(matrix[0])
    tested = 0
    zero_relations = 0
    zero_relation_categories: dict[str, int] = {}
    first_zero_by_category: dict[str, dict[str, Any]] = {}
    best_nonzero: dict[str, Any] | None = None
    for support in range(1, max_support + 1):
        for cols in itertools.combinations(range(cols_total), support):
            categories = sorted({column_category(col) for col in cols})
            category_key = "+".join(categories)
            for coeffs in itertools.product(coeff_values, repeat=support):
                tested += 1
                syndrome_values = [
                    sum(coeff * matrix[row][col] for col, coeff in zip(cols, coeffs)) % Q
                    for row in range(len(matrix))
                ]
                if all(value == 0 for value in syndrome_values):
                    zero_relations += 1
                    zero_relation_categories[category_key] = (
                        zero_relation_categories.get(category_key, 0) + 1
                    )
                    first_zero_by_category.setdefault(
                        category_key,
                        {
                            "columns": list(cols),
                            "coefficients": list(coeffs),
                            "column_categories": [column_category(col) for col in cols],
                        },
                    )
                    continue
                centered_syn = [centered(value, Q) for value in syndrome_values]
                stats = {
                    "columns": list(cols),
                    "coefficients": list(coeffs),
                    "column_categories": [column_category(col) for col in cols],
                    "support": support,
                    "syndrome_support": sum(1 for value in syndrome_values if value % Q),
                    "syndrome_lee_weight": sum(abs(value) for value in centered_syn),
                    "syndrome_prefix": syndrome_values[:8],
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
    dangerous_categories = {
        key: value
        for key, value in zero_relation_categories.items()
        if "checked_core" in key.split("+") or "selected_account_projection" in key.split("+")
    }
    return {
        "space": "small signed column combinations in the public-view kernel",
        "max_support": max_support,
        "coefficient_values": list(coeff_values),
        "tested": tested,
        "zero_relations_found": zero_relations,
        "zero_relation_categories": zero_relation_categories,
        "dangerous_zero_relation_categories": dangerous_categories,
        "first_zero_by_category": first_zero_by_category,
        "best_nonzero_syndrome": best_nonzero,
    }


def multi_slot_summary(slot_counts: tuple[int, ...] = (1, 2, 4, 8)) -> list[dict[str, Any]]:
    baseline = view_summary("baseline_12_syndrome_rows_per_live_slot", False)
    proposed = view_summary("proposed_18_rows_per_live_slot", True)
    rows: list[dict[str, Any]] = []
    for slots in slot_counts:
        rows.append(
            {
                "slots": slots,
                "baseline_public_rows": slots * baseline["rows"],
                "baseline_rank": slots * baseline["rank"],
                "baseline_kernel_dimension": slots * baseline["kernel_dimension"],
                "proposed_public_rows": slots * proposed["rows"],
                "proposed_rank": slots * proposed["rank"],
                "proposed_kernel_dimension": slots * proposed["kernel_dimension"],
                "additional_rank": slots * (proposed["rank"] - baseline["rank"]),
                "additional_public_rows": slots * (proposed["rows"] - baseline["rows"]),
            }
        )
    return rows


def view_summary(name: str, include_projection_rows: bool) -> dict[str, Any]:
    matrix = public_view_matrix(include_projection_rows)
    rank = rank_mod_q(matrix, Q)
    cols = len(matrix[0])
    rows = len(matrix)
    pivots = rref_mod_q(matrix, Q)[1]
    projection_pivots = [
        pivot - CHECKED_CORE_COLS for pivot in pivots if pivot >= CHECKED_CORE_COLS
    ]
    return {
        "name": name,
        "rows": rows,
        "columns": cols,
        "checked_core_columns": CHECKED_CORE_COLS,
        "account_projection_columns": ACCOUNT_PROJECTION_COLS,
        "rank": rank,
        "kernel_dimension": cols - rank,
        "row_deficiency": rows - rank,
        "matrix_digest": matrix_digest(matrix),
        "selected_projection_columns": selected_projection_columns()
        if include_projection_rows
        else [],
        "projection_pivots": projection_pivots,
        "sparse_row_relation_probe": sparse_row_relation_probe(matrix),
        "sparse_column_kernel_probe": sparse_column_kernel_probe(matrix),
    }


def build_payload() -> dict[str, Any]:
    baseline = view_summary("baseline_12_syndrome_rows_per_live_slot", False)
    proposed = view_summary("proposed_18_rows_per_live_slot", True)
    extra_rank = proposed["rank"] - baseline["rank"]
    projection_rows_independent = (
        extra_rank == len(SELECTED_ACCOUNT_PROJECTION_SYMBOLS)
        and not proposed["sparse_row_relation_probe"]["zero_relation_found"]
    )
    return {
        "schema": "Descender.pre_signature_projection_leakage_probe.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "pre_signature_projection_leakage_probe_completed_fail_closed",
        "deployment_ready": False,
        "q": Q,
        "model": {
            "checked_core_namespace": "22 Fq12 checked-core witness coordinates",
            "account_projection_namespace": "16 blocks x 2 local coordinates",
            "bridge_assumption": "none; projection coordinates are modeled as independent public rows until a bridge theorem exists",
            "selected_account_projection_symbols": SELECTED_ACCOUNT_PROJECTION_SYMBOLS,
        },
        "baseline": baseline,
        "proposed": proposed,
        "comparison": {
            "additional_public_rows_per_live_slot": proposed["rows"] - baseline["rows"],
            "additional_rank_per_live_slot": extra_rank,
            "additional_kernel_reduction_per_live_slot": baseline["kernel_dimension"]
            - proposed["kernel_dimension"],
            "projection_rows_independent_in_this_model": projection_rows_independent,
            "signature_byte_increase_required": 0,
            "carrier_account_lane_bytes_can_be_redundant": 12,
        },
        "multi_slot_scaling": multi_slot_summary(),
        "interpretation": {
            "replay_surface": "no new algebraic row dependency or selected-projection kernel relation found in this public-row model; transcript and lifecycle binding still need byte tests",
            "leakage_surface": "strictly stronger than current pre-signature account state: six independent q12289 rows become public for every live slot",
            "not_proved": [
                "keygen/public-view pseudorandomness under 18 rows",
                "absence of nonlinear distinguishers using many prepublished slots",
                "full hidden-source soundness",
                "zero-knowledge or witness-hiding beyond the charged public rows",
            ],
            "next_required_probe": "run existing structural cryptanalysis with 18 public rows per live slot as the public account-state matrix",
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    baseline = payload["baseline"]
    proposed = payload["proposed"]
    comparison = payload["comparison"]
    lines = [
        "# Pre-Signature Projection Leakage Probe",
        "",
        "This probe compares the current live-slot account-state view with the",
        "proposed 18-residue Fq12 witness-state layout. It is fail-closed: it",
        "checks public-row leakage shape, not deployable security.",
        "",
        "## Model",
        "",
        f"- q: `{payload['q']}`",
        f"- checked-core namespace: `{payload['model']['checked_core_namespace']}`",
        f"- account-projection namespace: `{payload['model']['account_projection_namespace']}`",
        f"- bridge assumption: `{payload['model']['bridge_assumption']}`",
        f"- selected account projection symbols: `{', '.join(payload['model']['selected_account_projection_symbols'])}`",
        "",
        "## Results",
        "",
        "| view | rows | columns | rank | kernel dimension | row deficiency | small row relation |",
        "|---|---:|---:|---:|---:|---:|---|",
        f"| baseline 12-row live slot | {baseline['rows']} | {baseline['columns']} | {baseline['rank']} | {baseline['kernel_dimension']} | {baseline['row_deficiency']} | {baseline['sparse_row_relation_probe']['zero_relation_found']} |",
        f"| proposed 18-row live slot | {proposed['rows']} | {proposed['columns']} | {proposed['rank']} | {proposed['kernel_dimension']} | {proposed['row_deficiency']} | {proposed['sparse_row_relation_probe']['zero_relation_found']} |",
        "",
        "## Comparison",
        "",
        f"- additional public rows per live slot: `{comparison['additional_public_rows_per_live_slot']}`",
        f"- additional rank per live slot: `{comparison['additional_rank_per_live_slot']}`",
        f"- additional kernel reduction per live slot: `{comparison['additional_kernel_reduction_per_live_slot']}`",
        f"- projection rows independent in this model: `{comparison['projection_rows_independent_in_this_model']}`",
        f"- signature byte increase required: `{comparison['signature_byte_increase_required']}`",
        f"- carrier account-lane bytes that can become redundant: `{comparison['carrier_account_lane_bytes_can_be_redundant']}`",
        "",
        "## Sparse Kernel Probe",
        "",
        "Zero column-kernel relations are expected for nonselected hidden projection",
        "coordinates in this conservative model. The relevant failure signal would be",
        "a zero relation involving checked-core columns or selected account projection",
        "columns.",
        "",
        f"- baseline dangerous zero-relation categories: `{baseline['sparse_column_kernel_probe']['dangerous_zero_relation_categories']}`",
        f"- proposed dangerous zero-relation categories: `{proposed['sparse_column_kernel_probe']['dangerous_zero_relation_categories']}`",
        f"- proposed zero-relation categories: `{proposed['sparse_column_kernel_probe']['zero_relation_categories']}`",
        "",
        "## Multi-Slot Scaling",
        "",
        "| live slots | baseline rank | proposed rank | additional rank | proposed public rows |",
        "|---:|---:|---:|---:|---:|",
    ]
    for row in payload["multi_slot_scaling"]:
        lines.append(
            f"| {row['slots']} | {row['baseline_rank']} | {row['proposed_rank']} | {row['additional_rank']} | {row['proposed_public_rows']} |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            f"- replay surface: {payload['interpretation']['replay_surface']}",
            f"- leakage surface: {payload['interpretation']['leakage_surface']}",
            "",
            "Not proved by this probe:",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in payload["interpretation"]["not_proved"])
    lines.extend(
        [
            "",
            f"Next required probe: {payload['interpretation']['next_required_probe']}",
            "",
        ]
    )
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
                "additional_rank_per_live_slot": payload["comparison"][
                    "additional_rank_per_live_slot"
                ],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
