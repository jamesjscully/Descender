#!/usr/bin/env python3
"""Generate the first Path 2 descent/SIS commitment feasibility model."""

from __future__ import annotations

import json
from dataclasses import dataclass, asdict
from math import ceil
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_descent_sis_feasibility.json"
OUT_MD = ROOT / "research/path2_descent_sis_feasibility.md"

Q = 12_289
Q_BITS = 14
ACTIVE_PROJECTION_LANES = 6
PACKED_WITNESS_RESIDUES = 18


@dataclass(frozen=True)
class RoleVector:
    role: str
    secret_lanes: int
    blinding_lanes: int
    range_witness_lanes: int
    notes: str

    @property
    def total_lanes(self) -> int:
        return self.secret_lanes + self.blinding_lanes + self.range_witness_lanes


@dataclass(frozen=True)
class CandidateRow:
    id: str
    sis_rows_per_commitment: int
    sis_columns_total: int
    commitment_roles: int
    commitment_bytes_each: int
    commitment_bytes_total: int
    response_bytes_lower_bound: int
    transcript_commitment_bytes_lower_bound: int
    payload_bytes_lower_bound: int
    verifier_mul_adds_for_commitment_checks: int
    public_relation_q_ops: int
    status: str
    blocker: str


ROLE_VECTORS = [
    RoleVector(
        role="pad_source",
        secret_lanes=ACTIVE_PROJECTION_LANES,
        blinding_lanes=24,
        range_witness_lanes=0,
        notes="six hidden pad lanes plus randomness/noise lanes for hiding",
    ),
    RoleVector(
        role="projection_relation",
        secret_lanes=ACTIVE_PROJECTION_LANES,
        blinding_lanes=24,
        range_witness_lanes=0,
        notes="six hidden account-projection lanes plus randomness/noise lanes",
    ),
    RoleVector(
        role="opening_tuple",
        secret_lanes=8,
        blinding_lanes=32,
        range_witness_lanes=8,
        notes=(
            "placeholder scalarization of opening tuple context; exact frame "
            "mapping is still a proof obligation"
        ),
    ),
]


def packed_q_bytes(lanes: int) -> int:
    return ceil(lanes * Q_BITS / 8)


def build_candidate(row_count: int) -> CandidateRow:
    roles = len(ROLE_VECTORS)
    columns = sum(role.total_lanes for role in ROLE_VECTORS)
    commitment_bytes_each = packed_q_bytes(row_count)
    commitment_bytes_total = roles * commitment_bytes_each

    # Lower-bound transcript shape for a Fiat-Shamir linear proof:
    # one first-message commitment per role plus one response vector opening.
    # This is not a deployable proof-size estimate; it excludes rejection-
    # sampling slack, norm proof overhead, challenge repetition, and extractor
    # machinery.
    transcript_commitment_bytes_lower_bound = commitment_bytes_total
    response_bytes_lower_bound = packed_q_bytes(columns)
    payload_bytes_lower_bound = (
        commitment_bytes_total
        + transcript_commitment_bytes_lower_bound
        + response_bytes_lower_bound
        + 32  # proof_system_id or challenge-binding digest floor
    )
    verifier_mul_adds = row_count * columns
    public_relation_q_ops = ACTIVE_PROJECTION_LANES * 2

    if row_count < 48:
        status = "too_small_for_security_claim"
        blocker = "row count is useful for wiring only; no SIS hardness claim"
    elif row_count <= 64:
        status = "primary_feasibility_band"
        blocker = "needs concrete SIS hardness, hiding, norm, and ZK proof analysis"
    else:
        status = "larger_fallback_band"
        blocker = "higher byte and verifier cost; compare against transparent proof fallback"

    return CandidateRow(
        id=f"path2_descent_sis_rows_{row_count}",
        sis_rows_per_commitment=row_count,
        sis_columns_total=columns,
        commitment_roles=roles,
        commitment_bytes_each=commitment_bytes_each,
        commitment_bytes_total=commitment_bytes_total,
        response_bytes_lower_bound=response_bytes_lower_bound,
        transcript_commitment_bytes_lower_bound=transcript_commitment_bytes_lower_bound,
        payload_bytes_lower_bound=payload_bytes_lower_bound,
        verifier_mul_adds_for_commitment_checks=verifier_mul_adds,
        public_relation_q_ops=public_relation_q_ops,
        status=status,
        blocker=blocker,
    )


def build_model() -> dict[str, Any]:
    rows = [build_candidate(row_count) for row_count in (32, 48, 64, 96)]
    return {
        "schema": "Descender.path2_descent_sis_feasibility.v1",
        "status": "candidate_shape_pinned_not_deployable",
        "current_shape_blocker": {
            "id": "path2_descent_sis_rank_hiding_probe",
            "artifact": "research/path2_descent_sis_rank_hiding_probe.md",
            "finding": (
                "the current three-public-commitment shape is full-column-rank "
                "for the main pad/projection/opening roles and therefore reveals "
                "openings if those commitments are public"
            ),
        },
        "decision": {
            "primary_path": "Path2DescentSisCommitmentV1",
            "fallback": "transparent_hash_based_proof_over_proof_friendly_relation",
            "spend_signature_bytes_delta": 0,
            "state_leaf_bytes_delta": 32,
            "reason": (
                "descent/SIS matches the existing Descender gluing theorem shape "
                "and preserves a post-quantum account-creation proof path"
            ),
        },
        "public_quotient": {
            "field": "masked_projection",
            "lanes": ACTIVE_PROJECTION_LANES,
            "relation": "masked_projection_i = pad_lane_i + account_projection_i mod q",
            "q": Q,
            "source": (
                "lanes 12..17 of the canonical 18-residue packed witness commitment"
            ),
        },
        "role_vectors": [asdict(role) | {"total_lanes": role.total_lanes} for role in ROLE_VECTORS],
        "candidate_rows": [asdict(row) for row in rows],
        "required_theorem_obligations": [
            "instantiate LocalQuotientDescent public/representative/hidden/evidence types",
            "define role-separated matrix derivation domains",
            "prove commitment binding or export SIS/module-SIS binding break",
            "prove hiding/sampler preservation or export samplerHidingBreak",
            "prove link equation is non-public and not a projection shortcut",
            "define norm/range bounds for all committed openings",
            "define Fiat-Shamir/QROM challenge budget for the payload proof",
            "define extractor from accepting payload to hidden account opening or named break",
        ],
        "negative_controls": [
            "wrong masked projection rejects",
            "wrong leaf or creation_proof_binding_digest rejects",
            "public projection shortcut is not accepted as proof",
            "non-short modular witness opening rejects",
            "duplicate or malleated commitment rejects",
            "sampler-conditioning replay stays above hiding target",
            "reject paths do not export pad lanes, projection lanes, masks, or opening frames",
        ],
        "non_claims": [
            "lower-bound payload bytes are not a deployable proof-size estimate",
            "no SIS hardness level is claimed by this model",
            "opening_tuple scalarization is a placeholder until the exact frame map is defined",
            "the model does not replace production preflight's fail-closed blocker",
            "the initial public commitment dimensions are blocked by the rank/hiding probe",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines: list[str] = [
        "# Path 2 Descent/SIS Feasibility",
        "",
        "Status: candidate shape pinned; not deployable.",
        "",
        "## Decision",
        "",
        f"- primary path: `{model['decision']['primary_path']}`",
        f"- fallback: `{model['decision']['fallback']}`",
        f"- spend signature byte delta: `{model['decision']['spend_signature_bytes_delta']}`",
        f"- state leaf byte delta: `{model['decision']['state_leaf_bytes_delta']}`",
        f"- reason: {model['decision']['reason']}",
        f"- current shape blocker: {model['current_shape_blocker']['finding']}",
        "",
        "## Public Quotient",
        "",
        "```text",
        model["public_quotient"]["relation"],
        "```",
        "",
        f"- q: `{model['public_quotient']['q']}`",
        f"- lanes: `{model['public_quotient']['lanes']}`",
        f"- source: {model['public_quotient']['source']}",
        "",
        "## Committed Role Vectors",
        "",
        "| role | secret lanes | blinding lanes | range lanes | total | notes |",
        "|---|---:|---:|---:|---:|---|",
    ]
    for role in model["role_vectors"]:
        lines.append(
            "| {role} | {secret_lanes} | {blinding_lanes} | "
            "{range_witness_lanes} | {total_lanes} | {notes} |".format(**role)
        )

    lines.extend(
        [
            "",
            "## Candidate Byte And Q-Op Rows",
            "",
            "| candidate | rows/commitment | columns | commitment bytes | response lower bound | payload lower bound | verifier mul-adds | status |",
            "|---|---:|---:|---:|---:|---:|---:|---|",
        ]
    )
    for row in model["candidate_rows"]:
        lines.append(
            "| {id} | {sis_rows_per_commitment} | {sis_columns_total} | "
            "{commitment_bytes_total} | {response_bytes_lower_bound} | "
            "{payload_bytes_lower_bound} | {verifier_mul_adds_for_commitment_checks} | "
            "{status} |".format(**row)
        )

    lines.extend(["", "## Required Theorem Obligations", ""])
    for item in model["required_theorem_obligations"]:
        lines.append(f"- {item}")

    lines.extend(["", "## Negative Controls", ""])
    for item in model["negative_controls"]:
        lines.append(f"- {item}")

    lines.extend(["", "## Non-Claims", ""])
    for item in model["non_claims"]:
        lines.append(f"- {item}")

    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "The 48-row and 64-row bands are the first plausible engineering targets,",
            "but this artifact deliberately does not claim security. It only fixes the",
            "candidate public quotient, committed vectors, rough account-creation byte",
            "floor, and verifier arithmetic shape so the real SIS/hiding/extractor",
            "analysis has a concrete object to attack.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    model = build_model()
    OUT_JSON.write_text(json.dumps(model, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(write_markdown(model))
    print(json.dumps({"json": str(OUT_JSON), "markdown": str(OUT_MD)}, indent=2))


if __name__ == "__main__":
    main()
