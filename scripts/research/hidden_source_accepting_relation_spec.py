#!/usr/bin/env python3
"""Generate the byte-level accepting relation for the hidden-source envelope."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "hidden_source_accepting_relation_spec_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "hidden_source_accepting_relation_spec_n32_q12289.md"
SEMANTICS_TARGET_JSON = (
    READINESS_DIR / "hidden_source_proof_semantics_target_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    target = load_json(SEMANTICS_TARGET_JSON)
    verifier_phases = [
        {
            "phase": "decode_envelope",
            "input_bytes": "signature",
            "rule": "length must be 284 bytes and split as 252-byte checked core followed by 32-byte carrier",
            "failure": "malformed_hidden_source_envelope",
        },
        {
            "phase": "check_core_signature",
            "input_bytes": "252-byte reviewed Fq12 core",
            "rule": "run the reviewed account-carried Fq12 core checks: codec, challenge binding, strict sigma equation, local-fiber consistency, and public pad-opening consistency",
            "failure": "checked_core_rejected",
        },
        {
            "phase": "recompute_public_pad_statement",
            "input_bytes": "checked core public openings and active challenge pattern",
            "rule": "derive observed-block bitmap/count and inferred per-block pad residues, then hash the canonical public pad-opening statement",
            "failure": "public_pad_statement_reconstruction_failed",
        },
        {
            "phase": "recompute_hidden_source_statement",
            "input_bytes": "slot id, account commitments, public key digest, message digest, signature digest, and pad-opening statement digest",
            "rule": "hash the canonical hidden-source public statement frame from verifier-owned bytes only",
            "failure": "hidden_source_statement_reconstruction_failed",
        },
        {
            "phase": "recompute_proof_transcript",
            "input_bytes": "hidden-source statement digest and carrier bytes",
            "rule": "hash the domain-separated proof transcript with q, n, dimensions, proof byte count, statement digest, and carrier bytes",
            "failure": "proof_transcript_reconstruction_failed",
        },
        {
            "phase": "evaluate_carrier_semantics",
            "input_bytes": "canonical statement, canonical transcript, and carrier bytes",
            "rule": "run the selected compact proof verifier profile and accept only if it proves the hidden-source witness relation",
            "failure": "hidden_source_proof_rejected",
        },
        {
            "phase": "consume_state",
            "input_bytes": "accepted slot id and account state transition",
            "rule": "acceptance is valid only with an atomic one-use slot transition under the chain replay/rollback policy",
            "failure": "state_lifecycle_rejected",
        },
    ]
    witness_relation = [
        {
            "predicate": "account_witness_commitment_opens",
            "statement_side": "account_witness_commitment",
            "witness_side": "account_witness_syndrome_opening",
            "must_prove": "the hidden witness syndrome packs to the account witness commitment",
        },
        {
            "predicate": "witness_matches_core_equation",
            "statement_side": "checked 252-byte core signature context",
            "witness_side": "account_witness_syndrome_opening",
            "must_prove": "the same hidden witness is the one charged by the reviewed Fq12 sigma equation",
        },
        {
            "predicate": "pad_seed_opens_public_statement",
            "statement_side": "public_pad_opening_statement_digest",
            "witness_side": "pad_seed_opening",
            "must_prove": "the pad seed derives exactly the inferred public pad residues and observed-block bitmap/count",
        },
        {
            "predicate": "transcript_binds_context",
            "statement_side": "slot, account commitments, public key, message, signature, pad statement, and carrier",
            "witness_side": "proof randomness or auxiliary witness if used",
            "must_prove": "the proof cannot be moved to a different account slot, key, message, signature core, pad statement, or carrier",
        },
    ]
    theorem_hooks = [
        {
            "hook": "completeness",
            "claim": "honest signer with valid account witness and pad seed can produce an accepting carrier for the exact relation",
            "status": "open",
        },
        {
            "hook": "knowledge_soundness",
            "claim": "an accepting forgery extracts the hidden account witness and pad source, or breaks a named assumption/certificate",
            "status": "open",
        },
        {
            "hook": "zero_knowledge_or_leakage_bound",
            "claim": "carrier and public residues leak only the information named in the leakage theorem",
            "status": "open",
        },
        {
            "hook": "qrom_fiat_shamir_composition",
            "claim": "challenge, carrier transcript, pad source, and blockchain query budgets compose under the selected model",
            "status": "open",
        },
        {
            "hook": "implementation_refinement",
            "claim": "Rust deployment verifier accepts exactly this relation and rejects every named failure mode without side-channel or lifecycle gaps",
            "status": "open",
        },
    ]
    negative_controls = [
        "284-byte envelope with malformed length",
        "valid carrier bytes attached to a different message digest",
        "valid carrier bytes attached to a different slot id",
        "valid carrier bytes attached to a different public key digest",
        "carrier transcript computed from cached rather than recomputed statement digest",
        "pad seed that opens a different observed-block bitmap/count",
        "witness opening that matches commitment but not the checked core equation",
        "accepted carrier under replayed or rolled-back account slot state",
        "bare public quotient reconstruction path",
        "bad one-coordinate or two-coordinate response-link variants",
    ]
    return {
        "schema": "Descender.hidden_source_accepting_relation_spec.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "relation_spec_fail_closed",
        "deployment_ready": False,
        "source_target": {
            "artifact": rel(SEMANTICS_TARGET_JSON),
            "schema": target.get("schema"),
            "status": target.get("status"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "checked_core_signature_bytes": 252,
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "account_link_bytes": 68,
            "public_key_bytes": 172,
            "relation_name": "hidden_source_accepting_relation_n32_q12289",
        },
        "acceptance_semantics": {
            "accepts_if_and_only_if": (
                "all verifier phases pass, the selected compact carrier semantics "
                "prove the witness relation, and the account slot transition is valid"
            ),
            "current_implementation": (
                "fail-closed: the byte envelope and transcript are present, but "
                "carrier semantics are intentionally unimplemented"
            ),
        },
        "verifier_phases": verifier_phases,
        "witness_relation": witness_relation,
        "theorem_hooks": theorem_hooks,
        "negative_controls_required": negative_controls,
        "next_artifact": {
            "name": "carrier_construction_decision",
            "purpose": "decide whether the 32-byte carrier can realize this relation or record a minimal byte increase",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    lines = [
        "# Hidden-Source Accepting Relation Spec",
        "",
        "This artifact turns the hidden-source proof target into an exact",
        "byte-level verifier relation. It remains fail-closed: it specifies what",
        "acceptance must mean, but it does not assert that the current 32-byte",
        "carrier already realizes that relation.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- relation: `{target['relation_name']}`",
        f"- n: `{target['n']}`",
        f"- q: `{target['q']}`",
        f"- checked core signature bytes: `{target['checked_core_signature_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- account-link bytes: `{target['account_link_bytes']}`",
        f"- public-key bytes: `{target['public_key_bytes']}`",
        f"- source target: `{payload['source_target']['artifact']}`",
        "",
        "## Acceptance Semantics",
        "",
        f"- accepts iff: {payload['acceptance_semantics']['accepts_if_and_only_if']}",
        f"- current implementation: {payload['acceptance_semantics']['current_implementation']}",
        "",
        "## Verifier Phases",
        "",
        "| phase | input bytes | rule | failure |",
        "|---|---|---|---|",
    ]
    for row in payload["verifier_phases"]:
        lines.append(
            f"| `{row['phase']}` | {row['input_bytes']} | {row['rule']} | `{row['failure']}` |"
        )
    lines.extend(
        [
            "",
            "## Witness Relation",
            "",
            "| predicate | statement side | witness side | must prove |",
            "|---|---|---|---|",
        ]
    )
    for row in payload["witness_relation"]:
        lines.append(
            f"| `{row['predicate']}` | {row['statement_side']} | {row['witness_side']} | {row['must_prove']} |"
        )
    lines.extend(
        [
            "",
            "## Theorem Hooks",
            "",
            "| hook | status | claim |",
            "|---|---|---|",
        ]
    )
    for row in payload["theorem_hooks"]:
        lines.append(f"| `{row['hook']}` | `{row['status']}` | {row['claim']} |")
    lines.extend(["", "## Negative Controls Required", ""])
    lines.extend(f"- {item}" for item in payload["negative_controls_required"])
    lines.extend(
        [
            "",
            "## Next Artifact",
            "",
            f"- name: `{payload['next_artifact']['name']}`",
            f"- purpose: {payload['next_artifact']['purpose']}",
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
