#!/usr/bin/env python3
"""Generate the state-assisted hidden-source hidden-relation verifier contract."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.md"
)
ACCEPTING_RELATION_JSON = (
    READINESS_DIR / "hidden_source_accepting_relation_spec_n32_q12289.json"
)
CARRIER_VERIFIER_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_carrier_verifier_contract_n32_q12289.json"
)
ACCOUNT_STATE_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    accepting_relation = load_json(ACCEPTING_RELATION_JSON)
    carrier_verifier = load_json(CARRIER_VERIFIER_JSON)
    account_state = load_json(ACCOUNT_STATE_JSON)
    public_statement_fields = [
        {
            "field": "public_key_digest",
            "source": "canonical public key bytes",
            "role": "binds the hidden relation to one verifier key",
        },
        {
            "field": "message_digest",
            "source": "transaction message bytes",
            "role": "binds the hidden relation to one signed payload",
        },
        {
            "field": "signature_digest",
            "source": "strict 284-byte envelope bytes",
            "role": "prevents carrier replay across signatures",
        },
        {
            "field": "proof_transcript_digest",
            "source": "reviewed Fq12 hidden-source transcript frame",
            "role": "binds the opening to the verifier-owned transcript",
        },
        {
            "field": "slot_id",
            "source": "account-state sidecar index",
            "role": "binds acceptance to one lifecycle slot",
        },
        {
            "field": "account_witness_commitment",
            "source": "prechallenge account state",
            "role": "commits the hidden account witness syndrome",
        },
        {
            "field": "account_randomizer_commitment",
            "source": "prechallenge account state",
            "role": "commits the pad/randomizer source",
        },
        {
            "field": "hidden_source_opening_commitment",
            "source": "prechallenge account-state sidecar",
            "role": "commits the private opening material before challenge derivation",
        },
        {
            "field": "restore_anchor_digest",
            "source": "active state-root or restore anchor",
            "role": "rules out rollback-backed opening reuse",
        },
    ]
    private_opening_fields = [
        {
            "field": "witness_opening_digest",
            "semantics": "opens the account witness commitment to the syndrome used by the hidden-source statement",
            "must_remain_hidden": True,
        },
        {
            "field": "pad_seed_digest",
            "semantics": "opens the pad/randomizer commitment to the pad source used by the public pad-opening relation",
            "must_remain_hidden": True,
        },
        {
            "field": "carrier_handle_secret_digest",
            "semantics": "domain-separated handle secret authenticating this precommitted opening",
            "must_remain_hidden": True,
        },
    ]
    verifier_checks = [
        {
            "check": "recompute_statement",
            "acceptance": "construct the canonical hidden-source public statement from verifier-owned public bytes and account-state fields",
            "rejects": ["cached_statement_digest", "wrong_public_context"],
        },
        {
            "check": "verify_slot_opening_commitment",
            "acceptance": "run verify_reviewed_fq12_state_assisted_slot_opening before any hidden-relation acceptance",
            "rejects": ["postchallenge_opening", "wrong_opening_commitment", "slot_replay"],
        },
        {
            "check": "verify_carrier_handle",
            "acceptance": "run verify_reviewed_fq12_state_assisted_carrier_handle and bind the 32-byte carrier to the same opening commitment",
            "rejects": ["digest_only_carrier", "carrier_context_replay"],
        },
        {
            "check": "verify_witness_commitment_opening",
            "acceptance": "prove the hidden witness opening digest is an opening of account_witness_commitment",
            "rejects": ["witness_digest_substitution"],
        },
        {
            "check": "verify_pad_seed_opening",
            "acceptance": "prove the pad seed digest is an opening of account_randomizer_commitment and the public pad-opening statement",
            "rejects": ["pad_seed_substitution", "public_pad_statement_mismatch"],
        },
        {
            "check": "verify_hidden_source_relation",
            "acceptance": "prove the opened witness and pad source satisfy the byte-level accepting relation for the recomputed statement",
            "rejects": ["witness_pad_mismatch", "relation_projection_mismatch"],
        },
    ]
    extraction_cases = [
        {
            "case": "all_checks_accept",
            "extracts": [
                "account witness opening",
                "pad seed opening",
                "carrier handle opening",
                "canonical hidden-source statement",
            ],
            "or_break": "none",
        },
        {
            "case": "same_public_context_two_hidden_openings",
            "extracts": [],
            "or_break": "opening commitment binding or SHA-256 collision/preimage break",
        },
        {
            "case": "carrier_reused_with_different_context",
            "extracts": [],
            "or_break": "carrier handle binding or transcript-domain separation break",
        },
        {
            "case": "relation_accepts_false_statement",
            "extracts": [],
            "or_break": "hidden-source proof soundness assumption break",
        },
    ]
    implementation_targets = [
        {
            "target": "Rust statement builder",
            "status": "partial",
            "required_next": "expose one canonical state-assisted hidden-relation statement object instead of composing checks ad hoc",
        },
        {
            "target": "opening verifier",
            "status": "partial",
            "required_next": "connect slot-opening equality to witness/pad commitment-opening semantics",
        },
        {
            "target": "compact proof verifier",
            "status": "missing",
            "required_next": "instantiate the proof system or named assumption for proving the hidden relation inside the carrier budget",
        },
        {
            "target": "negative controls",
            "status": "partial",
            "required_next": "add executable tests for witness digest substitution, pad seed substitution, and relation projection mismatch",
        },
    ]
    blockers = [
        {
            "blocker": "compact_proof_system_not_instantiated",
            "reason": "the contract names the relation, but no accepting proof verifier proves it within the 32-byte carrier",
        },
        {
            "blocker": "commitment_opening_semantics_not_connected",
            "reason": "current Rust checks equality of digests; it does not prove those digests open the account commitments",
        },
        {
            "blocker": "extractor_theorem_not_proved",
            "reason": "acceptance is not yet reduced to extracted witness/pad openings or named assumption breaks",
        },
    ]
    return {
        "schema": "Descender.hidden_source_state_assisted_hidden_relation_verifier_contract.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "hidden_relation_verifier_contract_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "accepting_relation": rel(ACCEPTING_RELATION_JSON),
            "carrier_verifier_contract": rel(CARRIER_VERIFIER_JSON),
            "account_state_schema_contract": rel(ACCOUNT_STATE_JSON),
            "accepting_relation_schema": accepting_relation.get("schema"),
            "carrier_verifier_schema": carrier_verifier.get("schema"),
            "account_state_schema": account_state.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "checked_core_signature_bytes": 252,
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "selected_candidate": "state_assisted_32_byte_carrier",
        },
        "public_statement_fields": public_statement_fields,
        "private_opening_fields": private_opening_fields,
        "verifier_checks": verifier_checks,
        "extraction_cases": extraction_cases,
        "implementation_targets": implementation_targets,
        "blockers": blockers,
        "decision": {
            "can_accept_now": False,
            "reason": "the hidden relation is now byte-level specified for the state-assisted path, but the compact proof verifier and extractor theorem remain open",
            "next_artifact": "state_assisted_hidden_relation_negative_controls_or_extractor_theorem",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    lines = [
        "# Hidden-Source State-Assisted Hidden-Relation Verifier Contract",
        "",
        "This artifact pins the hidden-relation verifier contract for the",
        "state-assisted 32-byte carrier route.  It separates the already-checked",
        "account-state opening commitment from the still-open proof that the hidden",
        "witness and pad source satisfy the canonical hidden-source relation.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected candidate: `{target['selected_candidate']}`",
        f"- checked-core signature bytes: `{target['checked_core_signature_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- accepting relation: `{payload['source_artifacts']['accepting_relation']}`",
        f"- carrier verifier contract: `{payload['source_artifacts']['carrier_verifier_contract']}`",
        f"- account-state schema contract: `{payload['source_artifacts']['account_state_schema_contract']}`",
        "",
        "## Public Statement Fields",
        "",
        "| field | source | role |",
        "|---|---|---|",
    ]
    for row in payload["public_statement_fields"]:
        lines.append(f"| `{row['field']}` | {row['source']} | {row['role']} |")
    lines.extend(
        [
            "",
            "## Private Opening Fields",
            "",
            "| field | semantics | hidden |",
            "|---|---|:---:|",
        ]
    )
    for row in payload["private_opening_fields"]:
        lines.append(
            f"| `{row['field']}` | {row['semantics']} | `{row['must_remain_hidden']}` |"
        )
    lines.extend(["", "## Verifier Checks", "", "| check | acceptance | rejects |", "|---|---|---|"])
    for row in payload["verifier_checks"]:
        lines.append(f"| `{row['check']}` | {row['acceptance']} | `{row['rejects']}` |")
    lines.extend(["", "## Extraction Cases", "", "| case | extracts | or break |", "|---|---|---|"])
    for row in payload["extraction_cases"]:
        lines.append(f"| `{row['case']}` | `{row['extracts']}` | {row['or_break']} |")
    lines.extend(["", "## Implementation Targets", "", "| target | status | required next |", "|---|---|---|"])
    for row in payload["implementation_targets"]:
        lines.append(f"| {row['target']} | `{row['status']}` | {row['required_next']} |")
    lines.extend(["", "## Blockers", "", "| blocker | reason |", "|---|---|"])
    for row in payload["blockers"]:
        lines.append(f"| `{row['blocker']}` | {row['reason']} |")
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{payload['decision']['can_accept_now']}`",
            f"- reason: {payload['decision']['reason']}",
            f"- next artifact: `{payload['decision']['next_artifact']}`",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.parent.mkdir(parents=True, exist_ok=True)
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
