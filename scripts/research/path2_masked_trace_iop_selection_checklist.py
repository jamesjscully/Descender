#!/usr/bin/env python3
"""Generate the Path 2 masked-trace IOP selection checklist."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/path2_masked_trace_iop_selection_checklist.json"
OUT_MD = ROOT / "research/path2_masked_trace_iop_selection_checklist.md"


CHECKS = [
    {
        "rank": 1,
        "id": "typed_role_subcommitment_api",
        "status": "blocking",
        "requirement": (
            "The commitment layer exposes parent trace root plus domain-separated "
            "role subroots/column roots that can be matched to "
            "Path2CreationProofBindingV1 role handles."
        ),
        "accept": (
            "A candidate has a formal commitment-opening theorem for the parent "
            "root and typed role subroots, with a named binding break for two "
            "different role vectors under one handle."
        ),
        "reject": (
            "Only a monolithic Merkle root is exposed, or role handles are merely "
            "SHA digest labels with no proof-system opening semantics."
        ),
    },
    {
        "rank": 2,
        "id": "masked_query_zk",
        "status": "blocking",
        "requirement": (
            "Every verifier-visible opening for hidden columns is masked, shared, "
            "or simulatable; no raw pad/projection/opening/range/account lane is "
            "ever opened."
        ),
        "accept": (
            "A simulator can sample query openings, paths, and consistency "
            "messages from public inputs and masked_projection without hidden "
            "witnesses."
        ),
        "reject": (
            "The verifier must see raw trace leaves or range witnesses at sampled "
            "positions."
        ),
    },
    {
        "rank": 3,
        "id": "extractor_for_unmasked_trace",
        "status": "blocking",
        "requirement": (
            "Knowledge soundness extracts the unmasked committed trace and role "
            "vectors, not just masked opened values or a satisfying tuple."
        ),
        "accept": (
            "Extractor theorem outputs pad_source_opening, "
            "projection_relation_opening, opening_tuple, range witnesses, masks, "
            "and role authentication paths for one parent root."
        ),
        "reject": (
            "Extraction only recovers queried masked values or relies on a "
            "non-binding post-hoc reconstruction."
        ),
    },
    {
        "rank": 4,
        "id": "mask_freshness_domain_separation",
        "status": "blocking",
        "requirement": (
            "Masks/shares are domain-separated by account, slot, role, row, "
            "column, query index, and proof-system transcript."
        ),
        "accept": (
            "The proof statement includes mask freshness and exports "
            "live_slot_leakage_or_keygen_break for reuse or solvable mask "
            "systems."
        ),
        "reject": (
            "The same mask can contribute to two public linear equations for a "
            "hidden lane across any replay surface."
        ),
    },
    {
        "rank": 5,
        "id": "range_gadget_zk",
        "status": "blocking",
        "requirement": (
            "Base128 digit range checks are proven without exposing raw digits or "
            "reusable range witnesses."
        ),
        "accept": (
            "Candidate either has a native ZK range-check gadget over the trace "
            "or an equivalent masked consistency proof with bounded overhead."
        ),
        "reject": (
            "Range checking requires opening unmasked digits, bit decompositions, "
            "or reusable auxiliary witnesses."
        ),
    },
    {
        "rank": 6,
        "id": "fiat_shamir_schedule",
        "status": "blocking",
        "requirement": (
            "Queries/challenges are derived after binding leaf context, binding "
            "digest, trace root, role handles, matrix transcript, aggregation "
            "context, proof system id, and column profile."
        ),
        "accept": "Transcript order is explicit and matches the QROM ledger.",
        "reject": "Any role handle, root, or context field can be chosen after a query.",
    },
    {
        "rank": 7,
        "id": "byte_threshold",
        "status": "decision_gate",
        "requirement": (
            "Concrete payload estimate remains materially below the 16416-byte "
            "lattice-sigma repetition floor or gives clearly better assumptions."
        ),
        "accept": (
            "Concrete masked-trace estimate stays in the current 6.1-10.7 KB "
            "planning band plus tolerable overhead."
        ),
        "reject": (
            "Concrete overhead approaches the sigma floor without better theorem "
            "quality."
        ),
    },
]


CANDIDATE_CLASSES = [
    {
        "id": "vanilla_merkle_fri_trace",
        "standing": "rejected_negative_control",
        "reason": "fails masked_query_zk because it opens raw hidden trace leaves",
    },
    {
        "id": "zk_masked_trace_iop",
        "standing": "active_candidate",
        "reason": (
            "only current route matching typed transparent commitments plus hidden "
            "query simulation, pending concrete commitment/range details"
        ),
    },
    {
        "id": "mpc_in_the_head_hash_zk",
        "standing": "backup_candidate",
        "reason": (
            "ZK is native, but it does not naturally satisfy the current typed "
            "trace-subcommitment interface and likely has larger payloads"
        ),
    },
    {
        "id": "explicit_algebraic_opening_rows",
        "standing": "structural_fallback",
        "reason": (
            "use if typed role subcommitments fail; requires a new byte model and "
            "careful hiding treatment"
        ),
    },
]


def build_model() -> dict[str, Any]:
    return {
        "schema": "Descender.path2_masked_trace_iop_selection_checklist.v1",
        "status": "selection_checklist_pinned",
        "purpose": (
            "Convert zkMaskedTraceIOPBindingAndHiding from a named assumption into "
            "a concrete pre-implementation gate. A proof family must satisfy the "
            "blocking checks before any accepting verifier work starts."
        ),
        "checks": CHECKS,
        "candidate_classes": CANDIDATE_CLASSES,
        "current_best_candidate": "zk_masked_trace_iop",
        "current_reject": "vanilla_merkle_fri_trace",
        "next_blocker": (
            "Instantiate the active candidate at the interface level: choose a "
            "commitment layout with parent root and typed role subroots, then "
            "state the simulator/extractor theorem for that layout."
        ),
        "non_claims": [
            "this does not select a concrete library or implementation",
            "this does not prove zkMaskedTraceIOPBindingAndHiding",
            "this does not finalize proof sizes",
            "this does not make Path 2 deployable",
        ],
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Masked-Trace IOP Selection Checklist",
        "",
        "Status: selection checklist pinned.",
        "",
        model["purpose"],
        "",
        "## Blocking Checks",
        "",
        "| rank | id | status | requirement | accept | reject |",
        "|---:|---|---|---|---|---|",
    ]
    for item in model["checks"]:
        lines.append(
            "| {rank} | `{id}` | `{status}` | {requirement} | {accept} | {reject} |".format(
                **item
            )
        )
    lines.extend(
        [
            "",
            "## Candidate Classes",
            "",
            "| id | standing | reason |",
            "|---|---|---|",
        ]
    )
    for item in model["candidate_classes"]:
        lines.append("| `{id}` | `{standing}` | {reason} |".format(**item))
    lines.extend(
        [
            "",
            "## Current Decision",
            "",
            f"- current best candidate: `{model['current_best_candidate']}`",
            f"- current reject: `{model['current_reject']}`",
            "",
            "## Next Blocker",
            "",
            model["next_blocker"],
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
