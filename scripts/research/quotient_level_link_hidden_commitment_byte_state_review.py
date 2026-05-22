#!/usr/bin/env python3
"""Byte/state review for hidden-commitment quotient-link options."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
ACCOUNT_SEMANTICS = READINESS_DIR / "quotient_level_link_account_semantics.json"
LEAN_COMPARISON = READINESS_DIR / "quotient_level_link_lean_interface_comparison.md"
DEFAULT_JSON = READINESS_DIR / "quotient_level_link_hidden_commitment_byte_state_review.json"
DEFAULT_MD = READINESS_DIR / "quotient_level_link_hidden_commitment_byte_state_review.md"

CURRENT_SPEND_BYTES = 250
CURRENT_ACCOUNT_BYTES = 68
CURRENT_ONLINE_PLUS_ACCOUNT = CURRENT_SPEND_BYTES + CURRENT_ACCOUNT_BYTES
HASH_COMMITMENT_BYTES = 32
LINK_VALUE_BYTES = 4


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def row(
    *,
    candidate: str,
    account_object: str,
    account_bytes: int | None,
    spend_bytes_delta: int | None,
    proof_bytes_delta: int | None,
    live_read_bytes: int | None,
    writeback_bytes: int | None,
    commitment_hiding: str,
    binding_or_extraction: str,
    link_equation: str,
    non_public_projection: str,
    sampler_surface: str,
    bounded_kernel_margin: int | None,
    status: str,
    recommendation: str,
    notes: str,
) -> dict[str, Any]:
    spend_bytes = None if spend_bytes_delta is None else CURRENT_SPEND_BYTES + spend_bytes_delta
    online_plus_account = (
        None if spend_bytes is None or account_bytes is None else spend_bytes + account_bytes
    )
    state_touched = (
        None if live_read_bytes is None or writeback_bytes is None else live_read_bytes + writeback_bytes
    )
    return {
        "candidate": candidate,
        "account_object": account_object,
        "account_bytes": account_bytes,
        "spend_bytes": spend_bytes,
        "spend_bytes_delta": spend_bytes_delta,
        "proof_bytes_delta": proof_bytes_delta,
        "live_read_bytes": live_read_bytes,
        "writeback_bytes": writeback_bytes,
        "state_touched_per_spend_bytes": state_touched,
        "online_plus_account_bytes": online_plus_account,
        "delta_online_plus_account_vs_current": (
            None if online_plus_account is None else online_plus_account - CURRENT_ONLINE_PLUS_ACCOUNT
        ),
        "commitment_hiding": commitment_hiding,
        "binding_or_extraction": binding_or_extraction,
        "link_equation": link_equation,
        "non_public_projection": non_public_projection,
        "sampler_surface": sampler_surface,
        "bounded_kernel_margin": bounded_kernel_margin,
        "status": status,
        "recommendation": recommendation,
        "notes": notes,
    }


def build_payload() -> dict[str, Any]:
    account_semantics = load_json(ACCOUNT_SEMANTICS)
    rows = [
        row(
            candidate="current-reviewed-split-account-link",
            account_object="32B witness commitment + 32B randomizer commitment + 4B public response-link code",
            account_bytes=CURRENT_ACCOUNT_BYTES,
            spend_bytes_delta=0,
            proof_bytes_delta=0,
            live_read_bytes=CURRENT_ACCOUNT_BYTES,
            writeback_bytes=CURRENT_ACCOUNT_BYTES,
            commitment_hiding="reviewed_current_surface",
            binding_or_extraction="reviewed_pre_response_backing",
            link_equation="bounded_kernel_response_link_certificate",
            non_public_projection="reviewed_gate",
            sampler_surface="reviewed_133_bit_floor",
            bounded_kernel_margin=43,
            status="control",
            recommendation="keep",
            notes="Deployable research control.  It keeps account binding and response-link kernel exclusion separate.",
        ),
        row(
            candidate="public-two-row-quotient-link",
            account_object="semantic quotient plus public packed two-row link value",
            account_bytes=LINK_VALUE_BYTES,
            spend_bytes_delta=0,
            proof_bytes_delta=0,
            live_read_bytes=LINK_VALUE_BYTES,
            writeback_bytes=LINK_VALUE_BYTES,
            commitment_hiding="none",
            binding_or_extraction="public_value_only",
            link_equation="direct_public_check",
            non_public_projection="not_sufficient",
            sampler_surface="fails_public_link_conditioning",
            bounded_kernel_margin=94,
            status="rejected",
            recommendation="do_not_pursue",
            notes="Rejected by sampler replay: the top exact coordinate rows fall to 106-108 hiding bits.",
        ),
        row(
            candidate="hash-hidden-link-only",
            account_object="H(link(w)) + H(link(r))",
            account_bytes=2 * HASH_COMMITMENT_BYTES,
            spend_bytes_delta=None,
            proof_bytes_delta=None,
            live_read_bytes=2 * HASH_COMMITMENT_BYTES,
            writeback_bytes=2 * HASH_COMMITMENT_BYTES,
            commitment_hiding="computational_hash_preimage_hiding_only",
            binding_or_extraction="binds_link_values_but_not_full_hidden_state",
            link_equation="needs_new_zk_or_opening_proof",
            non_public_projection="possible_if_proof_not_public_projection",
            sampler_surface="preserved_if_link_values_never_open_publicly",
            bounded_kernel_margin=94,
            status="blocked",
            recommendation="reject_unless_full_pre_response_backing_is_recovered",
            notes="Saves 4 account bytes versus current control but drops explicit witness/randomizer backing unless another proof supplies it.",
        ),
        row(
            candidate="hash-hidden-link-plus-current-commitments",
            account_object="current 64B witness/randomizer commitments + H(link(w)) + H(link(r))",
            account_bytes=4 * HASH_COMMITMENT_BYTES,
            spend_bytes_delta=None,
            proof_bytes_delta=None,
            live_read_bytes=4 * HASH_COMMITMENT_BYTES,
            writeback_bytes=4 * HASH_COMMITMENT_BYTES,
            commitment_hiding="hash_commitments_hide_link_values",
            binding_or_extraction="current_backing_plus_new_link_binding",
            link_equation="needs_new_zk_or_opening_proof",
            non_public_projection="possible_if_proof_not_public_projection",
            sampler_surface="preserved_if_link_values_never_open_publicly",
            bounded_kernel_margin=94,
            status="dominated",
            recommendation="do_not_pursue",
            notes="Strictly larger account state than the 68B control and still adds a new proof system.",
        ),
        row(
            candidate="pedersen-style-homomorphic-hidden-link",
            account_object="two hiding homomorphic commitments to link(w) and link(r)",
            account_bytes=2 * HASH_COMMITMENT_BYTES,
            spend_bytes_delta=None,
            proof_bytes_delta=None,
            live_read_bytes=2 * HASH_COMMITMENT_BYTES,
            writeback_bytes=2 * HASH_COMMITMENT_BYTES,
            commitment_hiding="plausible_with_new_group_assumption",
            binding_or_extraction="plausible_with_discrete_log_binding_assumption",
            link_equation="homomorphic_but_needs_non_public_projection_argument",
            non_public_projection="open",
            sampler_surface="preserved_if_commitments_are_hiding",
            bounded_kernel_margin=94,
            status="research_only",
            recommendation="only_pursue_for_proof_simplification",
            notes="Potentially 64B account state, but introduces a new algebraic commitment assumption and likely nonzero proof/opening bytes.",
        ),
        row(
            candidate="reuse-existing-account-commitments-for-link",
            account_object="derive committed link values from existing witness/randomizer commitments",
            account_bytes=2 * HASH_COMMITMENT_BYTES,
            spend_bytes_delta=None,
            proof_bytes_delta=None,
            live_read_bytes=2 * HASH_COMMITMENT_BYTES,
            writeback_bytes=2 * HASH_COMMITMENT_BYTES,
            commitment_hiding="inherits_existing_commitment_hiding_if_available",
            binding_or_extraction="open_needs_link_extraction_from_existing_commitments",
            link_equation="open_needs_non_public_link_equation_proof",
            non_public_projection="open",
            sampler_surface="preserved_if_link_values_not_revealed",
            bounded_kernel_margin=94,
            status="research_only",
            recommendation="best_remaining_branch",
            notes="Only plausible branch with no account-byte increase.  It must prove the existing commitments bind enough link information without revealing link values.",
        ),
    ]
    viable = [
        item
        for item in rows
        if item["recommendation"] in {"best_remaining_branch", "only_pursue_for_proof_simplification"}
    ]
    return {
        "schema": "Descender.quotient_level_link.hidden_commitment_byte_state_review.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "firebreak": {
            "touches_deployable_path": False,
            "touches_current_verifier": False,
            "touches_current_proof_path": False,
        },
        "inputs": {
            "account_semantics": rel(ACCOUNT_SEMANTICS),
            "lean_interface_comparison": rel(LEAN_COMPARISON),
            "baseline_cost_model": "research/parameter_search/committed_sigma_account_carried_20260521/committed_sigma_account_carried_cost_model.json",
        },
        "baseline": {
            "current_spend_bytes": CURRENT_SPEND_BYTES,
            "current_account_bytes": CURRENT_ACCOUNT_BYTES,
            "current_online_plus_account_bytes": CURRENT_ONLINE_PLUS_ACCOUNT,
        },
        "summary": {
            "rows_reviewed": len(rows),
            "deployment_recommendation": "keep_current_reviewed_split_design",
            "public_link_rejected": True,
            "byte_saving_candidate_found": False,
            "best_remaining_research_branch": "reuse-existing-account-commitments-for-link",
            "reason": (
                "No hidden-commitment option currently gives a deployable byte win. "
                "The only branch worth further research is reuse of existing account commitments, "
                "because it might improve proof organization without increasing state."
            ),
        },
        "commitment_options": rows,
        "viable_research_only_options": viable,
        "next_required_work": [
            "Test whether existing witness/randomizer commitments can expose a binding theorem for linkOfWitness/linkOfRandomizer without revealing link values.",
            "If not, reject hidden-commitment quotient links as dominated by the current 68B split design.",
            "Do not pursue hash-hidden-link-only unless a separate theorem restores full pre-response backing.",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    summary = payload["summary"]
    baseline = payload["baseline"]
    lines = [
        "# Hidden-Commitment Quotient-Link Byte/State Review",
        "",
        "Status: no deployable byte win found; one proof-organization branch remains.",
        "",
        "This sidecar review does not change the q12289 verifier, proof path, byte target, or implementation.",
        "",
        "## Summary",
        "",
        "| item | value |",
        "|---|---:|",
        f"| rows reviewed | `{summary['rows_reviewed']}` |",
        f"| current spend bytes | `{baseline['current_spend_bytes']}` |",
        f"| current account bytes | `{baseline['current_account_bytes']}` |",
        f"| current online plus account bytes | `{baseline['current_online_plus_account_bytes']}` |",
        f"| byte-saving candidate found | `{summary['byte_saving_candidate_found']}` |",
        f"| deployment recommendation | `{summary['deployment_recommendation']}` |",
        "",
        summary["reason"],
        "",
        "## Commitment Options",
        "",
        "| candidate | account bytes | tx+account | kernel margin | status | recommendation |",
        "|---|---:|---:|---:|---|---|",
    ]
    for item in payload["commitment_options"]:
        account_bytes = "-" if item["account_bytes"] is None else str(item["account_bytes"])
        total = "-" if item["online_plus_account_bytes"] is None else str(item["online_plus_account_bytes"])
        margin = "-" if item["bounded_kernel_margin"] is None else str(item["bounded_kernel_margin"])
        lines.append(
            f"| {item['candidate']} | {account_bytes} | {total} | {margin} | "
            f"{item['status']} | {item['recommendation']} |"
        )
    lines.extend(["", "## Proof Obligations", ""])
    lines.extend([
        "| candidate | hiding | binding/extraction | link equation | non-public projection | sampler surface |",
        "|---|---|---|---|---|---|",
    ])
    for item in payload["commitment_options"]:
        lines.append(
            f"| {item['candidate']} | {item['commitment_hiding']} | "
            f"{item['binding_or_extraction']} | {item['link_equation']} | "
            f"{item['non_public_projection']} | {item['sampler_surface']} |"
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
                "deployment_recommendation": payload["summary"]["deployment_recommendation"],
                "best_remaining_research_branch": payload["summary"]["best_remaining_research_branch"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
