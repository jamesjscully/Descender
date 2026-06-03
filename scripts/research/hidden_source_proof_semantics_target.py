#!/usr/bin/env python3
"""Generate the hidden-source proof-semantics target for the 308 B envelope."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "hidden_source_proof_semantics_target_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "hidden_source_proof_semantics_target_n32_q12289.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def build_payload() -> dict[str, Any]:
    public_inputs = [
        {
            "name": "slot_id",
            "binding": "four-byte account slot identifier consumed by the state machine",
        },
        {
            "name": "account_witness_commitment",
            "binding": "32-byte digest committed in account state before challenge derivation",
        },
        {
            "name": "account_randomizer_commitment",
            "binding": "32-byte digest committed in account state before challenge derivation",
        },
        {
            "name": "public_key_digest",
            "binding": "domain-separated digest of the canonical public key bytes",
        },
        {
            "name": "message_digest",
            "binding": "domain-separated digest of the signed message frame",
        },
        {
            "name": "signature_digest",
            "binding": "domain-separated digest of the checked 252 B/Fq12 signature core",
        },
        {
            "name": "public_pad_opening_statement_digest",
            "binding": "digest of observed block bitmap/count and inferred public pad residues",
        },
        {
            "name": "hidden_source_statement_digest",
            "binding": "canonical statement digest recomputed from verifier context",
        },
        {
            "name": "proof_transcript_digest",
            "binding": "Fiat-Shamir/public-input transcript digest over statement and carrier bytes",
        },
        {
            "name": "carrier_bytes",
            "binding": "56-byte projection carrier bound to the state-assisted opening commitment and hidden-source statement",
        },
    ]
    witness_inputs = [
        {
            "name": "account_witness_syndrome_opening",
            "obligation": "opens the packed masked state18 commitment into 12 checked-core residues and six unmasked selected projection residues",
        },
        {
            "name": "pad_seed_opening",
            "obligation": "opens the public pad-opening statement through the canonical pad-source relation",
        },
        {
            "name": "proof_randomness_or_auxiliary_witness",
            "obligation": "only present if the chosen compact carrier construction needs it; must be leakage-accounted",
        },
    ]
    obligations = [
        {
            "id": "statement_binding",
            "status": "closed",
            "acceptance_condition": "candidate verifier recomputes all public-input digests from bytes and account state",
        },
        {
            "id": "witness_commitment_opening",
            "status": "closed",
            "acceptance_condition": "masked state18 extractor decodes the committed 32-byte witness field, unmasks the six projection residues with pad seed and slot id, and repacks to the same account commitment",
        },
        {
            "id": "pad_seed_opening",
            "status": "closed",
            "acceptance_condition": "candidate verifier binds a pad seed to the public pad-opening statement digest and state-assisted opening digest",
        },
        {
            "id": "transcript_binding",
            "status": "closed",
            "acceptance_condition": "candidate verifier binds carrier transcript bytes to slot, key, message, signature core, opening commitment, and pad statement",
        },
        {
            "id": "soundness_theorem",
            "status": "open",
            "acceptance_condition": "any accepting forgery extracts a hidden account witness/pad source or names an assumption break",
        },
        {
            "id": "zero_knowledge_and_leakage_theorem",
            "status": "open",
            "acceptance_condition": "carrier and public residues leak only the theorem-accounted information",
        },
        {
            "id": "sampler_and_pad_source_theorem",
            "status": "open",
            "acceptance_condition": "production sampler is secret-independent or has an explicit leakage budget",
        },
        {
            "id": "qrom_query_budget_integration",
            "status": "open",
            "acceptance_condition": "challenge, carrier, pad, and blockchain-query budgets are composed in one game",
        },
        {
            "id": "implementation_boundary",
            "status": "open",
            "acceptance_condition": "Rust verifier/signing boundary is constant-time audited for final semantics",
        },
        {
            "id": "state_lifecycle_boundary",
            "status": "open",
            "acceptance_condition": "slot consumption, replay, rollback, and restore-anchor rules match the theorem",
        },
    ]
    carrier_options = [
        {
            "id": "opaque_32_byte_carrier",
            "transaction_bytes": 284,
            "state": "archived path",
            "promise": "smaller legacy envelope",
            "risk": "a 32-byte digest is not automatically a proof and this is no longer the selected deployment target",
            "decision_gate": "dead path unless the selected 308-byte design is explicitly abandoned",
        },
        {
            "id": "state_assisted_masked_projection_carrier",
            "transaction_bytes": 308,
            "state": "selected candidate",
            "promise": "keeps spend bytes compact while the account state precommits the packed witness and opening lifecycle material",
            "risk": "still needs soundness, leakage/ZK, mask-PRF, and lifecycle theorem composition",
            "decision_gate": "accept only after the exact candidate relation has a soundness/leakage theorem and preflight blocker removal",
        },
        {
            "id": "small_algebraic_opening_carrier",
            "transaction_bytes": "greater_than_308_if_needed",
            "state": "fallback if 308 bytes cannot carry the proof",
            "promise": "more room for an algebraic response/opening object while staying compact for blockchain use",
            "risk": "byte increase and new q-ary relation analysis",
            "decision_gate": "record the minimal byte increase justified by proof obligations",
        },
        {
            "id": "account_state_assisted_precommitment",
            "transaction_bytes": 308,
            "state": "component of the selected candidate",
            "promise": "keeps spend bytes small by moving structure into precommitted state",
            "risk": "lifecycle, trusted restore-anchor, and state availability become security-critical",
            "decision_gate": "the theorem must bind state creation, spend, rollback, and replay as one relation",
        },
        {
            "id": "pay_to_hash_or_delayed_reveal",
            "transaction_bytes": "small_spend_then_larger_resolution",
            "state": "fallback architecture",
            "promise": "can keep immediate spend bytes small in some chain designs",
            "risk": "changes UX, finality, data availability, and what counts as immediate signature verification",
            "decision_gate": "treat as a separate protocol, not as the selected account-carried verifier",
        },
    ]
    next_steps = [
        "write the exact byte-level accepting relation for the selected 308 B/Fq12 projection-carrier envelope",
        "prove the selected carrier construction or record the minimum byte increase needed for soundness",
        "prove soundness, zero-knowledge/leakage, sampler, pad-source, and QROM composition obligations",
        "add negative controls for malformed carriers, replayed slots, rollback, transcript mismatch, and bad quotient variants",
        "rerun byte/security budgets only after the accepting relation is fixed",
        "promote Rust facade acceptance only after the preflight blocker set becomes empty",
    ]
    return {
        "schema": "Descender.hidden_source_proof_semantics_target.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "research_target_fail_closed",
        "deployment_ready": False,
        "target": {
            "n": 32,
            "q": 12289,
            "checked_core_signature_bytes": 252,
            "current_carrier_bytes": 56,
            "current_envelope_bytes": 308,
            "account_link_bytes": 68,
            "public_key_bytes": 172,
            "branch": "selected 308 B/Fq12 state-assisted masked projection-carrier envelope",
        },
        "canonical_relation": {
            "name": "hidden_source_accepting_relation_n32_q12289",
            "informal_statement": (
                "An accepting verifier must bind a checked 252 B/Fq12 signature core "
                "to an account witness commitment opening and a pad-seed opening for "
                "the canonical public pad-opening statement, under the byte-level "
                "transcript for the signature, account slot, public key, and message."
            ),
            "public_inputs": public_inputs,
            "witness_inputs": witness_inputs,
        },
        "accepting_verifier_obligations": obligations,
        "carrier_options": carrier_options,
        "decision_gates": {
            "rust_facade_acceptance": "blocked until hidden-source proof semantics accept the canonical relation",
            "preflight_blocker_removal": "blocked until every accepting-verifier obligation is implemented and reviewed",
            "byte_target_policy": "do not force 284 B if the proof requires more carrier bytes; record the minimal justified increase",
            "external_review_policy": "still required after local obligations pass",
        },
        "next_steps": next_steps,
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    relation = payload["canonical_relation"]
    lines = [
        "# Hidden-Source Proof Semantics Target",
        "",
        "This artifact fixes the next proof target for the account-carried compact",
        "signature branch. It is not a completed proof and it does not make the",
        "deployment verifier accept: the current posture remains fail-closed.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- n: `{target['n']}`",
        f"- q: `{target['q']}`",
        f"- checked core signature bytes: `{target['checked_core_signature_bytes']}`",
        f"- current carrier bytes: `{target['current_carrier_bytes']}`",
        f"- current envelope bytes: `{target['current_envelope_bytes']}`",
        f"- account-link bytes: `{target['account_link_bytes']}`",
        f"- public-key bytes: `{target['public_key_bytes']}`",
        f"- branch: `{target['branch']}`",
        "",
        "## Canonical Relation",
        "",
        f"- name: `{relation['name']}`",
        f"- statement: {relation['informal_statement']}",
        "",
        "### Public Inputs",
        "",
        "| input | binding |",
        "|---|---|",
    ]
    for item in relation["public_inputs"]:
        lines.append(f"| `{item['name']}` | {item['binding']} |")
    lines.extend(["", "### Witness Inputs", "", "| witness | obligation |", "|---|---|"])
    for item in relation["witness_inputs"]:
        lines.append(f"| `{item['name']}` | {item['obligation']} |")
    lines.extend(
        [
            "",
            "## Accepting-Verifier Obligations",
            "",
            "| obligation | status | acceptance condition |",
            "|---|---|---|",
        ]
    )
    for item in payload["accepting_verifier_obligations"]:
        lines.append(
            f"| `{item['id']}` | `{item['status']}` | {item['acceptance_condition']} |"
        )
    lines.extend(
        [
            "",
            "## Carrier Options",
            "",
            "| option | transaction bytes | state | risk | decision gate |",
            "|---|---:|---|---|---|",
        ]
    )
    for item in payload["carrier_options"]:
        lines.append(
            f"| `{item['id']}` | `{item['transaction_bytes']}` | {item['state']} | "
            f"{item['risk']} | {item['decision_gate']} |"
        )
    lines.extend(
        [
            "",
            "## Decision Gates",
            "",
        ]
    )
    for name, value in payload["decision_gates"].items():
        lines.append(f"- `{name}`: {value}")
    lines.extend(["", "## Next Steps", ""])
    lines.extend(f"{index}. {step}" for index, step in enumerate(payload["next_steps"], 1))
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.parent.mkdir(parents=True, exist_ok=True)
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
