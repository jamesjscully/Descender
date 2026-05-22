#!/usr/bin/env python3
"""Generate the quotient-level link kernel-search baseline artifact.

This is a sidecar research generator.  It reads existing reviewed artifacts and
normalizes them into the quotient-level-link candidate schema.  It must not
overwrite or mutate the reviewed response-link certificates.
"""

from __future__ import annotations

import argparse
import json
import math
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import response_link_bounded_kernel_check as kernel_check


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "quotient_level_link_kernel_search.json"
DEFAULT_MD = READINESS_DIR / "quotient_level_link_kernel_search.md"

BARE_QUOTIENT_REVIEW = READINESS_DIR / "quotient_sis_lattice_attack_review_n32_q12289.json"
RESPONSE_LINK_CERT = (
    READINESS_DIR
    / "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"
)
RESPONSE_LINK_AUDIT = (
    READINESS_DIR
    / "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json"
)
STATEFUL_REFERENCE = (
    ROOT / "tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs"
)

Q = 12289
REPRESENTATIVE_RESIDUES = 23
SEMANTIC_QUOTIENT_ROWS = 12
MATRIX_DOMAIN = "Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1"
PACKED_Q_BITS = 14


def zero_row() -> list[int]:
    return [0] * REPRESENTATIVE_RESIDUES


def unit_row(coord: int) -> list[int]:
    row = zero_row()
    row[coord] = 1
    return row


def sparse_row(entries: list[tuple[int, int]]) -> list[int]:
    row = zero_row()
    for coord, coeff in entries:
        row[coord] = coeff % Q
    return row


def row_rank(rows: list[list[int]]) -> int:
    if not rows:
        return 0
    return len(kernel_check.rref_mod_q([[value % Q for value in row] for row in rows])[1])


def semantic_factorization_report(
    semantic_rows: list[list[int]], link_rows: list[list[int]]
) -> dict[str, Any]:
    """Check whether the semantic quotient factors through a candidate link.

    Standalone factorization means link equality implies semantic equality,
    equivalently rowspace(semantic) is contained in rowspace(link).  A product
    quotient carrying `(semantic, link)` satisfies that condition by projection,
    but then the link rows are auxiliary account/bounded-kernel data rather
    than a standalone replacement for the semantic quotient.
    """

    semantic_rank = row_rank(semantic_rows)
    link_rank = row_rank(link_rows)
    combined_rank = row_rank(semantic_rows + link_rows)
    standalone = combined_rank == link_rank
    product = True
    if standalone:
        classification = "standalone_refines_semantic"
    else:
        classification = "auxiliary_product_quotient_only"
    return {
        "semantic_rank": semantic_rank,
        "link_rank": link_rank,
        "combined_rank": combined_rank,
        "standalone_semantic_factors": standalone,
        "product_semantic_factors": product,
        "link_adds_independent_rows_to_semantic": combined_rank - semantic_rank,
        "product_fiber_dimension": REPRESENTATIVE_RESIDUES - combined_rank,
        "classification": classification,
    }


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def packed_q_residue_bytes(residues: int) -> int:
    return (residues * PACKED_Q_BITS + 7) // 8


def centered_lift(value: int) -> int:
    return value - Q if value > Q // 2 else value


def vector_coeff_bound(vector: list[int]) -> int:
    return max((abs(value) for value in vector), default=0)


def vector_stats(vector: list[int]) -> dict[str, Any]:
    return {
        "coeff_bound": vector_coeff_bound(vector),
        "l1_weight": sum(abs(value) for value in vector),
        "support": sum(1 for value in vector if value),
        "euclidean_squared": sum(value * value for value in vector),
        "prefix": vector[:8],
        "vector": vector,
    }


def generic_intersection_basis(
    matrix: list[list[int]], link_rows: list[list[int]]
) -> tuple[list[list[int]], int, int]:
    """Integer basis for vectors satisfying matrix*x = 0 and link*x = 0 mod q."""

    rows = [row[:] for row in matrix]
    for link_row in link_rows:
        if len(link_row) != REPRESENTATIVE_RESIDUES:
            raise ValueError("link rows must have length 23")
        rows.append([value % Q for value in link_row])

    rref, pivots = kernel_check.rref_mod_q(rows)
    free_cols = [col for col in range(REPRESENTATIVE_RESIDUES) if col not in pivots]
    basis: list[list[int]] = []
    for free_col in free_cols:
        vector = [0] * REPRESENTATIVE_RESIDUES
        vector[free_col] = 1
        for row, pivot_col in enumerate(pivots):
            vector[pivot_col] = (-rref[row][free_col]) % Q
        basis.append([centered_lift(value) for value in vector])
    for pivot_col in pivots:
        vector = [0] * REPRESENTATIVE_RESIDUES
        vector[pivot_col] = Q
        basis.append(vector)
    return basis, len(pivots), len(free_cols)


def lll_probe(
    matrix: list[list[int]],
    link_rows: list[list[int]],
    response_bound: int,
) -> dict[str, Any]:
    basis, rank, fiber_dimension = generic_intersection_basis(matrix, link_rows)
    try:
        from fpylll import LLL, IntegerMatrix

        lattice = IntegerMatrix.from_matrix(basis)
        LLL.reduction(lattice, delta=0.99)
        reduced = [
            [int(lattice[row, col]) for col in range(lattice.ncols)]
            for row in range(lattice.nrows)
        ]
        nonzero = [row for row in reduced if any(row)]
        shortest = min(nonzero, key=lambda row: sum(value * value for value in row))
        stats = vector_stats(shortest)
        found_bad = stats["coeff_bound"] <= response_bound
        return {
            "probe": "lll",
            "rank": rank,
            "fiber_dimension": fiber_dimension,
            "reduced_basis_first_vector": vector_stats(reduced[0]) if reduced else None,
            "shortest_vector": stats,
            "found_bad_vector": found_bad,
            "margin_over_response_difference_bound": (
                stats["coeff_bound"] - response_bound
            ),
        }
    except Exception as exc:  # pragma: no cover - depends on local fpylll install
        return {
            "probe": "lll",
            "rank": rank,
            "fiber_dimension": fiber_dimension,
            "error": {"type": type(exc).__name__, "message": str(exc)},
            "found_bad_vector": None,
        }


def evaluated_link_candidates(
    matrix: list[list[int]], response_bound: int
) -> list[dict[str, Any]]:
    specs: list[dict[str, Any]] = [
        {
            "candidate": "coordinate-link-0-1",
            "family": "coordinate_projection",
            "link_rows": [unit_row(0), unit_row(1)],
            "description": "Two-coordinate comparison row from the existing Rust frontier.",
        },
        {
            "candidate": "coordinate-link-0-2",
            "family": "coordinate_projection",
            "link_rows": [unit_row(0), unit_row(2)],
            "description": "Two-coordinate comparison row from the existing Rust frontier.",
        },
        {
            "candidate": "coordinate-link-1-2",
            "family": "coordinate_projection",
            "link_rows": [unit_row(1), unit_row(2)],
            "description": "Two-coordinate comparison row from the existing Rust frontier.",
        },
        {
            "candidate": "sparse-sum-link-0p3-1p2",
            "family": "sparse_linear_rows",
            "link_rows": [
                sparse_row([(0, 1), (3, 1)]),
                sparse_row([(1, 1), (2, 1)]),
            ],
            "description": "First sparse two-row link: pair sums over the front coordinates.",
        },
        {
            "candidate": "sparse-difference-link-0m3-1m2",
            "family": "sparse_linear_rows",
            "link_rows": [
                sparse_row([(0, 1), (3, -1)]),
                sparse_row([(1, 1), (2, -1)]),
            ],
            "description": "First sparse two-row link: pair differences over the front coordinates.",
        },
        {
            "candidate": "sparse-cross-link-0p7-3p11",
            "family": "sparse_linear_rows",
            "link_rows": [
                sparse_row([(0, 1), (7, 1)]),
                sparse_row([(3, 1), (11, 1)]),
            ],
            "description": "Sparse two-row control mixing one current link coordinate with later coordinates.",
        },
    ]

    candidates: list[dict[str, Any]] = []
    for spec in specs:
        link_rows = spec["link_rows"]
        semantic_report = semantic_factorization_report(matrix, link_rows)
        probe = lll_probe(matrix, link_rows, response_bound)
        exact_path = (
            READINESS_DIR
            / f"quotient_level_link_exact_enum_{spec['candidate']}_b{response_bound}.json"
        )
        exact = load_json(exact_path) if exact_path.exists() else None
        shortest = probe.get("shortest_vector") or {}
        search_method = "generalized-link-row LLL probe"
        exact_artifact = None
        if exact is not None:
            exact_artifact = rel(exact_path)
            search_method = exact_artifact
            if exact.get("status") == "infeasible":
                bounded_status = "pass"
                status = "needs_hiding_review"
            elif exact.get("status") == "feasible":
                bounded_status = "fail"
                status = "rejected"
            elif exact.get("status") == "timeout":
                bounded_status = "unknown"
                status = "needs_exact_enum"
            else:
                bounded_status = "unknown"
                status = "needs_probe"
            shortest = exact.get("shortest_vector_in_cover_ball") or shortest
            shortest_bound = exact.get("shortest_coeff_bound")
            margin = exact.get("margin_over_response_difference_bound")
        else:
            found_bad = probe.get("found_bad_vector")
            if found_bad is True:
                bounded_status = "fail"
                status = "rejected"
            elif found_bad is False:
                bounded_status = "unknown"
                status = "needs_exact_enum"
            else:
                bounded_status = "unknown"
                status = "needs_probe"
            shortest_bound = shortest.get("coeff_bound")
            margin = probe.get("margin_over_response_difference_bound")
        if exact is None and probe.get("found_bad_vector") is True:
            bounded_status = "fail"
            status = "rejected"
        candidates.append(
            {
                "candidate": spec["candidate"],
                "family": spec["family"],
                "description": spec["description"],
                "link_rows": len(link_rows),
                "packed_bits": len(link_rows) * PACKED_Q_BITS,
                "packed_bytes": packed_q_residue_bytes(len(link_rows)),
                "rank": probe["rank"],
                "fiber_dimension": probe["fiber_dimension"],
                "semantic_factors": semantic_report["standalone_semantic_factors"],
                "semantic_factorization": semantic_report,
                "bounded_kernel_status": bounded_status,
                "shortest_bad_vector_bound": shortest_bound,
                "response_difference_bound": response_bound,
                "margin_over_response_difference_bound": margin,
                "search_method": search_method,
                "exact_artifact": exact_artifact,
                "status": status,
                "probe": probe,
                "exact_enum": exact,
            }
        )
    return candidates


def candidate_schema() -> dict[str, str]:
    return {
        "candidate": "stable string id",
        "family": (
            "coordinate_projection | sparse_linear_rows | block_local_rows | "
            "semantic_quotient_refinement | random_control | full_kernel_control"
        ),
        "link_rows": "number of link rows/residues exposed or committed",
        "packed_bits": "estimated packed bit cost for link rows",
        "packed_bytes": "estimated packed byte cost for link rows",
        "rank": "rank of semantic quotient plus link constraints when known",
        "fiber_dimension": "dimension remaining after semantic quotient plus link constraints when known",
        "semantic_factors": "true if semantic quotient provably factors through link; false if not; null if not checked",
        "semantic_factorization": (
            "rank-based rowspace check distinguishing standalone semantic "
            "factorization from product-quotient auxiliary links"
        ),
        "bounded_kernel_status": "pass | fail | unknown | control",
        "shortest_bad_vector_bound": (
            "coefficient bound of shortest detected vector in ker(semantic) "
            "cap ker(link), if known"
        ),
        "search_method": "source of evidence",
        "status": "control | candidate | rejected | needs_search | needs_exact_enum | needs_probe | needs_hiding_review",
    }


def candidate_families() -> list[dict[str, str]]:
    return [
        {
            "family": "coordinate_projection",
            "description": (
                "Expose or commit selected representative coordinates as the link. "
                "This includes the current [0,3] response-link code."
            ),
        },
        {
            "family": "sparse_linear_rows",
            "description": (
                "Use sparse linear combinations of representative residues as link "
                "rows. Search should prefer low Hamming weight rows before dense "
                "random rows."
            ),
        },
        {
            "family": "block_local_rows",
            "description": (
                "Use rows aligned to local block structure. These are candidates "
                "for cleaner verifier/account semantics if they preserve challenge "
                "equivariance."
            ),
        },
        {
            "family": "semantic_quotient_refinement",
            "description": (
                "Refine the existing 12-row semantic quotient with a small number "
                "of extra quotient rows, aiming to absorb response-link soundness "
                "into the quotient itself."
            ),
        },
        {
            "family": "random_control",
            "description": (
                "Random low-row links used only as negative/scale controls; not "
                "deployment candidates without interpretability and replayable "
                "certificates."
            ),
        },
        {
            "family": "full_kernel_control",
            "description": (
                "Expose enough coordinates to kill the entire public quotient "
                "kernel. This should be sound but not hiding-friendly, so it is a "
                "control rather than a deployment target."
            ),
        },
    ]


def tool_inventory() -> list[dict[str, object]]:
    return [
        {
            "path": "scripts/research/response_link_bounded_kernel_check.py",
            "role": (
                "Reconstructs the 12x23 q12289 full-response matrix and checks "
                "coordinate-projection response-link kernels with LLL, lattice "
                "enumeration, exact enumeration, CP-SAT, or SCIP modes."
            ),
            "reusable_for_milestone_1": True,
            "missing_for_unified_link_search": (
                "Currently centered on coordinate projections; sparse linear row "
                "families need a generalized link-row input format."
            ),
        },
        {
            "path": "scripts/research/verify_response_link_bounded_kernel_certificate.py",
            "role": (
                "Replays an exact bounded-kernel certificate and validates "
                "basis/hash consistency."
            ),
            "reusable_for_milestone_1": True,
            "missing_for_unified_link_search": (
                "Needs generalized certificate schema if link rows are not "
                "coordinate projections."
            ),
        },
        {
            "path": "scripts/research/quotient_sis_lattice_attack_review.py",
            "role": (
                "Builds the bare 12x23 quotient/SIS q-kernel lattice and runs "
                "LLL/BKZ plus sparse signed relation probes."
            ),
            "reusable_for_milestone_1": True,
            "missing_for_unified_link_search": (
                "Useful as the bare-quotient negative control; does not enumerate "
                "link candidates."
            ),
        },
        {
            "path": "scripts/research/response_link_lattice_attack_review.py",
            "role": "Runs lattice attack review for the selected response-link target.",
            "reusable_for_milestone_1": True,
            "missing_for_unified_link_search": (
                "Needs candidate-parameterized invocation and result normalization."
            ),
        },
        {
            "path": rel(STATEFUL_REFERENCE),
            "role": (
                "Rust reference/report source for full-response quotient parameters, "
                "response-link code frontier, sampler and account-link surfaces."
            ),
            "reusable_for_milestone_1": True,
            "missing_for_unified_link_search": (
                "Useful for source-of-parameters and controls, but not a sidecar "
                "search harness by itself."
            ),
        },
    ]


def build_payload() -> dict[str, Any]:
    bare = load_json(BARE_QUOTIENT_REVIEW)
    cert = load_json(RESPONSE_LINK_CERT)
    audit = load_json(RESPONSE_LINK_AUDIT)
    matrix = kernel_check.full_response_matrix()

    matrix_hash = bare["statement"]["matrix_transcript_sha256"]
    response_bound = int(cert["response_difference_bound"])
    current_shortest = cert["shortest_vector_in_cover_ball"]
    current_shortest_bound = int(current_shortest["coeff_bound"])
    bare_shortest_bound = int(bare["shortest_reduction_row"]["coeff_bound"])
    bare_kernel_dim = int(bare["statement"]["kernel_dimension"])
    bare_rank = int(bare["statement"]["rank"])
    current_rank = int(cert["combined_constraint_rank"])
    current_fiber_dim = int(cert["combined_kernel_dimension"])
    full_kernel_rows = bare_kernel_dim

    return {
        "schema": "Descender.quotient_level_link.kernel_search.v0",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "generalized_link_row_probe_generated",
        "firebreak": {
            "touches_deployable_path": False,
            "touches_current_verifier": False,
            "touches_current_proof_path": False,
            "notes": (
                "Milestone 1 sidecar artifact only. The current q12289 "
                "account-carried verifier and proof path remain unchanged."
            ),
        },
        "base_surface": {
            "q": Q,
            "representative_residues": REPRESENTATIVE_RESIDUES,
            "semantic_quotient_rows": SEMANTIC_QUOTIENT_ROWS,
            "matrix_domain": MATRIX_DOMAIN,
            "matrix_transcript_sha256": matrix_hash,
            "response_difference_bound": response_bound,
            "bare_quotient_rank": bare_rank,
            "bare_quotient_kernel_dimension": bare_kernel_dim,
        },
        "candidate_schema": candidate_schema(),
        "candidate_families": candidate_families(),
        "tool_inventory": tool_inventory(),
        "control_candidates": [
            {
                "candidate": "bare-quotient-negative-control",
                "family": "semantic_quotient_refinement",
                "link_rows": 0,
                "packed_bits": 0,
                "packed_bytes": 0,
                "rank": bare_rank,
                "fiber_dimension": bare_kernel_dim,
                "semantic_factors": True,
                "bounded_kernel_status": "fail",
                "shortest_bad_vector_bound": bare_shortest_bound,
                "search_method": rel(BARE_QUOTIENT_REVIEW),
                "status": "rejected",
                "interpretation": (
                    "Bare quotient has a short syndrome-zero vector inside the "
                    "response-difference bound, but the selected [0,3] "
                    "response-link catches it."
                ),
            },
            {
                "candidate": "current-coordinate-link-0-3",
                "family": "coordinate_projection",
                "link_rows": len(cert["coords"]),
                "packed_bits": len(cert["coords"]) * PACKED_Q_BITS,
                "packed_bytes": packed_q_residue_bytes(len(cert["coords"])),
                "rank": current_rank,
                "fiber_dimension": current_fiber_dim,
                "semantic_factors": False,
                "bounded_kernel_status": "pass",
                "shortest_bad_vector_bound": current_shortest_bound,
                "response_difference_bound": response_bound,
                "margin_over_response_difference_bound": (
                    current_shortest_bound - response_bound
                ),
                "search_method": rel(RESPONSE_LINK_CERT),
                "audit_manifest": rel(RESPONSE_LINK_AUDIT),
                "audit_manifest_payload_sha256": audit["manifest_payload_sha256"],
                "status": "control",
                "interpretation": (
                    "Current reviewed response-link target. It preserves a "
                    "9-dimensional algebraic kernel while exact enumeration finds "
                    "no nonzero vector in the accepted response-difference box."
                ),
            },
            {
                "candidate": "full-kernel-coordinate-code-control",
                "family": "full_kernel_control",
                "link_rows": full_kernel_rows,
                "packed_bits": full_kernel_rows * PACKED_Q_BITS,
                "packed_bytes": packed_q_residue_bytes(full_kernel_rows),
                "rank": REPRESENTATIVE_RESIDUES,
                "fiber_dimension": 0,
                "semantic_factors": True,
                "bounded_kernel_status": "pass",
                "shortest_bad_vector_bound": None,
                "search_method": (
                    rel(STATEFUL_REFERENCE) + " response_link_code_frontier"
                ),
                "status": "control",
                "interpretation": (
                    "Sound but not hiding-friendly: killing the full quotient "
                    "kernel collapses the public account witness fiber."
                ),
            },
        ],
        "evaluated_candidates": evaluated_link_candidates(matrix, response_bound),
        "source_artifacts": {
            "bare_quotient_review": rel(BARE_QUOTIENT_REVIEW),
            "response_link_certificate": rel(RESPONSE_LINK_CERT),
            "response_link_audit_manifest": rel(RESPONSE_LINK_AUDIT),
            "stateful_reference": rel(STATEFUL_REFERENCE),
        },
        "next_required_work": [
            "Run the first broader coordinate-pair sweep with current [0,3] as control.",
            "Promote any coordinate-pair sweep candidates near or above the current [0,3] margin into exact-enum certificates.",
            "Add sparse-row candidate generation rather than the current hand-picked sparse probes.",
            "Start the hiding ledger for exact-pass non-current candidates if their byte cost remains competitive.",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    base = payload["base_surface"]
    current = next(
        row
        for row in payload["control_candidates"]
        if row["candidate"] == "current-coordinate-link-0-3"
    )
    bare = next(
        row
        for row in payload["control_candidates"]
        if row["candidate"] == "bare-quotient-negative-control"
    )
    full = next(
        row
        for row in payload["control_candidates"]
        if row["candidate"] == "full-kernel-coordinate-code-control"
    )

    def cell(value: object) -> str:
        return str(value).replace("|", "\\|")

    lines = [
        "# Quotient-Level Link Kernel Search",
        "",
        "Status: generalized link-row LLL probes generated.",
        "",
        "This is a Milestone 1 sidecar artifact for the quotient-level account",
        "link research program.  It does not change the current q12289",
        "account-carried proof path, verifier, byte target, or implementation.",
        "",
        "## Scope",
        "",
        "Milestone 1 asks whether the current response-link map is close to",
        "Pareto-optimal among small links.  This pass normalizes the existing",
        "reviewed artifacts into a reusable candidate schema and adds a first",
        "hand-picked set of generalized linear-link LLL probes.  LLL-positive",
        "margins are evidence only; exact enumeration is still required before",
        "any pass claim.",
        "",
        "## Base Surface",
        "",
        "| item | value |",
        "|---|---:|",
        f"| modulus `q` | `{base['q']}` |",
        f"| representative residues | `{base['representative_residues']}` |",
        f"| semantic quotient rows | `{base['semantic_quotient_rows']}` |",
        f"| bare quotient rank | `{base['bare_quotient_rank']}` |",
        f"| bare quotient kernel dimension | `{base['bare_quotient_kernel_dimension']}` |",
        f"| response-difference bound | `{base['response_difference_bound']}` |",
        "",
        "Matrix domain:",
        "",
        "```text",
        base["matrix_domain"],
        "```",
        "",
        "Matrix transcript SHA-256:",
        "",
        "```text",
        base["matrix_transcript_sha256"],
        "```",
        "",
        "## Tool Inventory",
        "",
        "| tool | current role | Milestone 1 reuse | gap |",
        "|---|---|---|---|",
    ]
    for tool in payload["tool_inventory"]:
        lines.append(
            f"| `{cell(tool['path'])}` | {cell(tool['role'])} | "
            f"{'yes' if tool['reusable_for_milestone_1'] else 'no'} | "
            f"{cell(tool['missing_for_unified_link_search'])} |"
        )
    lines.extend(
        [
            "",
            "## Candidate Families",
            "",
            "| family | description |",
            "|---|---|",
        ]
    )
    for family in payload["candidate_families"]:
        lines.append(f"| `{cell(family['family'])}` | {cell(family['description'])} |")
    lines.extend(
        [
            "",
            "## Candidate Schema",
            "",
            "Each search result should emit:",
            "",
            "| field | meaning |",
            "|---|---|",
        ]
    )
    for field, meaning in payload["candidate_schema"].items():
        lines.append(f"| `{cell(field)}` | {cell(meaning)} |")
    lines.extend(
        [
            "",
            "## Control Rows",
            "",
            "| candidate | rows | bytes | rank | fiber dim | bounded-kernel status | shortest bound | status |",
            "|---|---:|---:|---:|---:|---|---:|---|",
        ]
    )
    for row in [bare, current, full]:
        shortest = row["shortest_bad_vector_bound"]
        shortest_text = "-" if shortest is None else str(shortest)
        lines.append(
            f"| {row['candidate']} | {row['link_rows']} | {row['packed_bytes']} | "
            f"{row['rank']} | {row['fiber_dimension']} | "
            f"{row['bounded_kernel_status']} | {shortest_text} | {row['status']} |"
        )
    lines.extend(
        [
            "",
            "## First Generalized Probes",
            "",
            "These rows use the generalized linear-link evaluator.  They are not",
            "certificates.  A `needs_exact_enum` row only means LLL did not find a",
            "bounded bad vector.  `auxiliary_product_quotient_only` means the",
            "candidate rows can accompany the semantic quotient but do not",
            "standalone determine it.",
            "",
            "| candidate | family | rows | bytes | rank | fiber dim | semantic class | shortest bound | margin | evidence/status |",
            "|---|---|---:|---:|---:|---:|---|---:|---:|---|",
        ]
    )
    for row in payload.get("evaluated_candidates", []):
        margin = row.get("margin_over_response_difference_bound")
        margin_text = "-" if margin is None else str(margin)
        shortest = row.get("shortest_bad_vector_bound")
        shortest_text = "-" if shortest is None else str(shortest)
        lines.append(
            f"| {row['candidate']} | {row['family']} | {row['link_rows']} | "
            f"{row['packed_bytes']} | {row['rank']} | {row['fiber_dimension']} | "
            f"{row['semantic_factorization']['classification']} | "
            f"{shortest_text} | {margin_text} | {row['status']} |"
        )
    lines.extend(
        [
            "",
            "The current `[0,3]` control has response-difference bound "
            f"`{current['response_difference_bound']}`; exact enumeration reports "
            "shortest intersection-vector coefficient bound "
            f"`{current['shortest_bad_vector_bound']}`, giving margin "
            f"`{current['margin_over_response_difference_bound']}`.  Its replay "
            "manifest payload hash is:",
            "",
            "```text",
            current["audit_manifest_payload_sha256"],
            "```",
            "",
            "The bare quotient negative control has a shortest reduced full-kernel",
            f"row with coefficient bound `{bare['shortest_bad_vector_bound']}`, "
            "inside the response-difference bound.  That row does not vanish on",
            "coordinates `[0,3]`, which is why the current response-link code",
            "catches it.",
            "",
            "## Current Interpretation",
            "",
            "The existing evidence supports the current `[0,3]` link as a working",
            "control, not as an optimality claim.  The next harness must search",
            "alternatives under the same result schema and make the comparison",
            "explicit.",
            "",
            "## Next Required Work",
            "",
        ]
    )
    for item in payload["next_required_work"]:
        lines.append(f"- {item}")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--write-json", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--write-md", type=Path, default=DEFAULT_MD)
    args = parser.parse_args()

    payload = build_payload()
    if args.write_json is not None:
        args.write_json.parent.mkdir(parents=True, exist_ok=True)
        args.write_json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    if args.write_md is not None:
        args.write_md.parent.mkdir(parents=True, exist_ok=True)
        args.write_md.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": str(args.write_json) if args.write_json else None,
                "markdown": str(args.write_md) if args.write_md else None,
                "status": payload["status"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
