#!/usr/bin/env python3
"""Review whether existing account commitments can carry hidden quotient links.

This sidecar closes the remaining hidden-commitment branch from the byte/state
review.  It is descriptive only: it does not touch the q12289 verifier, proof
path, byte target, or implementation.
"""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
BYTE_STATE_REVIEW = READINESS_DIR / "quotient_level_link_hidden_commitment_byte_state_review.json"
LOCAL_DELTA = ROOT / "Descender/LocalDeltaByteCommitment.lean"
COMMITTED_SIGMA = ROOT / "Descender/CommittedSigma.lean"
HIDDEN_INTERFACE = ROOT / "Descender/HiddenCommittedQuotientLink.lean"
DEFAULT_JSON = READINESS_DIR / "quotient_level_link_existing_commitment_reuse_review.json"
DEFAULT_MD = READINESS_DIR / "quotient_level_link_existing_commitment_reuse_review.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def contains(path: Path, needle: str) -> bool:
    return needle in path.read_text()


def source_checks() -> list[dict[str, Any]]:
    checks = [
        {
            "source": LOCAL_DELTA,
            "symbol": "ByteCommitmentConfig",
            "meaning": "existing commitment object hashes an encoded hidden state",
            "implication": "binds bytes only through collision exclusion",
        },
        {
            "source": LOCAL_DELTA,
            "symbol": "ByteHashBinding",
            "meaning": "fixed-domain byte hash binding assumption",
            "implication": "collision-free digest gives equality of encoded preimages after equal digest",
        },
        {
            "source": LOCAL_DELTA,
            "symbol": "collisionFreeOnDomain",
            "meaning": "binding is stated at the encoded byte-preimage boundary",
            "implication": "no homomorphic operation or committed linear link value is exposed",
        },
        {
            "source": COMMITTED_SIGMA,
            "symbol": "HomomorphicAccountLinkInterface",
            "meaning": "separate abstraction for commitments that can prove response-link equations",
            "implication": "the current byte hash commitment is not itself this interface",
        },
        {
            "source": COMMITTED_SIGMA,
            "symbol": "linkProofAccepts_of_publicProjection",
            "meaning": "public projections of the sigma equation add no security constraint",
            "implication": "a reused commitment proof must avoid becoming a public projection shortcut",
        },
        {
            "source": COMMITTED_SIGMA,
            "symbol": "reviewedAccountHiddenLinkBytes_eq",
            "meaning": "current reviewed account object is fixed at 68 bytes",
            "implication": "reuse is only interesting if it adds no bytes and no larger proof object",
        },
        {
            "source": HIDDEN_INTERFACE,
            "symbol": "HiddenCommittedQuotientLink.Interface",
            "needle": "structure Interface",
            "meaning": "sidecar hidden-link theorem shape",
            "implication": "requires binding, hiding, non-public projection, and sampler-surface gates",
        },
    ]
    return [
        {
            "source": rel(check["source"]),
            "symbol": check["symbol"],
            "present": contains(check["source"], check.get("needle", check["symbol"])),
            "meaning": check["meaning"],
            "implication": check["implication"],
        }
        for check in checks
    ]


def requirement_rows() -> list[dict[str, Any]]:
    return [
        {
            "requirement": "binds_full_hidden_preimage_after_opening",
            "available_from_existing_commitments": True,
            "usable_for_hidden_quotient_link": "partial",
            "reason": (
                "The byte-hash path can bind an opened encoded hidden state, "
                "assuming collision freedom and an injective encoding."
            ),
        },
        {
            "requirement": "exposes_link_value_without_opening_hidden_state",
            "available_from_existing_commitments": False,
            "usable_for_hidden_quotient_link": "no",
            "reason": (
                "A digest of the full hidden preimage does not expose a "
                "committed quotient-link value or a verifier-checkable link "
                "projection."
            ),
        },
        {
            "requirement": "homomorphic_link_equation_over_commitments",
            "available_from_existing_commitments": False,
            "usable_for_hidden_quotient_link": "no",
            "reason": (
                "The byte-hash commitment has no add/scale structure for "
                "checking link(z) = link(r) + c * link(w) on commitments."
            ),
        },
        {
            "requirement": "preserves_sampler_surface_without_public_link_rows",
            "available_from_existing_commitments": True,
            "usable_for_hidden_quotient_link": "conditional",
            "reason": (
                "The sampler surface is preserved only if link values remain "
                "hidden; opening them publicly reintroduces the public-link "
                "hiding failure."
            ),
        },
        {
            "requirement": "proves_response_link_soundness_without_new_proof",
            "available_from_existing_commitments": False,
            "usable_for_hidden_quotient_link": "no",
            "reason": (
                "Existing commitment binding can support an opening check, but "
                "does not by itself prove the hidden linear link equation."
            ),
        },
        {
            "requirement": "deployable_byte_win_over_68b_split_design",
            "available_from_existing_commitments": False,
            "usable_for_hidden_quotient_link": "no",
            "reason": (
                "No extra account bytes are needed only if reuse is free, but "
                "the missing non-public equation proof would add unknown proof "
                "bytes or assumptions."
            ),
        },
    ]


def proof_routes() -> list[dict[str, Any]]:
    return [
        {
            "route": "open_full_hidden_preimages",
            "state_delta_bytes": 0,
            "proof_delta_bytes": "large_or_contextual",
            "hiding_status": "fails_if_link_rows_become_public",
            "binding_status": "works_after_opening",
            "decision": "reject",
            "reason": (
                "Opening enough data to recompute link(w) and link(r) gives "
                "binding, but it either reveals the link rows or moves the "
                "cost into a large opening proof."
            ),
        },
        {
            "route": "zk_proof_from_hash_commitments",
            "state_delta_bytes": 0,
            "proof_delta_bytes": "unknown_nonzero",
            "hiding_status": "plausible",
            "binding_status": "requires_new_hash_circuit_soundness_argument",
            "decision": "research_only",
            "reason": (
                "A zero-knowledge circuit could prove committed preimages have "
                "the desired link relation, but that is a new proof system and "
                "is unlikely to beat a 68-byte account object without a concrete "
                "cost model."
            ),
        },
        {
            "route": "derive_homomorphic_link_commitment_from_hash_digest",
            "state_delta_bytes": 0,
            "proof_delta_bytes": None,
            "hiding_status": "not_applicable",
            "binding_status": "not_available",
            "decision": "reject",
            "reason": (
                "A general byte hash digest does not supply add/scale operations "
                "or a response commitment equation."
            ),
        },
    ]


def build_payload() -> dict[str, Any]:
    byte_state = load_json(BYTE_STATE_REVIEW)
    checks = source_checks()
    rows = requirement_rows()
    routes = proof_routes()
    missing = [check for check in checks if not check["present"]]
    return {
        "schema": "Descender.quotient_level_link.existing_commitment_reuse_review.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "firebreak": {
            "touches_deployable_path": False,
            "touches_current_verifier": False,
            "touches_current_proof_path": False,
        },
        "inputs": {
            "byte_state_review": rel(BYTE_STATE_REVIEW),
            "local_delta_byte_commitment": rel(LOCAL_DELTA),
            "committed_sigma": rel(COMMITTED_SIGMA),
            "hidden_committed_quotient_link": rel(HIDDEN_INTERFACE),
        },
        "source_checks": checks,
        "source_check_failures": missing,
        "requirements": rows,
        "proof_routes": routes,
        "summary": {
            "previous_best_remaining_branch": byte_state["summary"][
                "best_remaining_research_branch"
            ],
            "existing_commitment_reuse_status": "rejected_for_deployment",
            "deployment_recommendation": "keep_current_reviewed_split_design",
            "byte_saving_candidate_found": False,
            "proof_simplification_candidate_found": False,
            "reason": (
                "Existing account commitments bind full hidden preimages only "
                "after opening; they do not expose a hidden quotient-link "
                "commitment or homomorphic link equation.  Any route that keeps "
                "the link hidden needs a new zero-knowledge/hash-circuit proof, "
                "and any route that opens link values reintroduces the public "
                "link hiding failure."
            ),
        },
        "next_required_work": [
            "Create the Milestone 5 promotion gate and classify the result as no replacement.",
            "Record artifact paths, replay commands, Lean theorem names, and assumption deltas.",
            "Tell the implementation agent to keep ignoring this sidecar research unless a later promotion gate changes the recommendation.",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    summary = payload["summary"]
    lines = [
        "# Existing Commitment Reuse Review",
        "",
        "Status: reject as a deployable hidden quotient-link replacement.",
        "",
        "This sidecar review does not change the current q12289 verifier, proof path, byte target, or implementation.",
        "",
        "## Summary",
        "",
        "| item | value |",
        "|---|---|",
        f"| previous best branch | `{summary['previous_best_remaining_branch']}` |",
        f"| reuse status | `{summary['existing_commitment_reuse_status']}` |",
        f"| deployment recommendation | `{summary['deployment_recommendation']}` |",
        f"| byte-saving candidate found | `{summary['byte_saving_candidate_found']}` |",
        f"| proof-simplification candidate found | `{summary['proof_simplification_candidate_found']}` |",
        "",
        summary["reason"],
        "",
        "## Source Checks",
        "",
        "| source | symbol | present | implication |",
        "|---|---|---:|---|",
    ]
    for check in payload["source_checks"]:
        lines.append(
            f"| {check['source']} | `{check['symbol']}` | "
            f"`{check['present']}` | {check['implication']} |"
        )
    lines.extend(["", "## Requirement Ledger", ""])
    lines.extend([
        "| requirement | available | usable | reason |",
        "|---|---:|---|---|",
    ])
    for row in payload["requirements"]:
        lines.append(
            f"| `{row['requirement']}` | `{row['available_from_existing_commitments']}` | "
            f"`{row['usable_for_hidden_quotient_link']}` | {row['reason']} |"
        )
    lines.extend(["", "## Proof Routes", ""])
    lines.extend([
        "| route | proof bytes | hiding | binding | decision |",
        "|---|---|---|---|---|",
    ])
    for route in payload["proof_routes"]:
        proof_bytes = "-" if route["proof_delta_bytes"] is None else route["proof_delta_bytes"]
        lines.append(
            f"| `{route['route']}` | `{proof_bytes}` | `{route['hiding_status']}` | "
            f"`{route['binding_status']}` | `{route['decision']}` |"
        )
    lines.extend(["", "## Decision", ""])
    lines.append(
        "The existing commitments do not give a free quotient-level commitment link. "
        "They are useful for full-preimage binding after an opening, but the hidden "
        "link equation still needs a separate proof object or a different "
        "commitment algebra."
    )
    lines.extend(["", "## Next Required Work", ""])
    for item in payload["next_required_work"]:
        lines.append(f"- {item}")
    lines.append("")
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
                "status": payload["summary"]["existing_commitment_reuse_status"],
                "deployment_recommendation": payload["summary"][
                    "deployment_recommendation"
                ],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
