#!/usr/bin/env python3
"""Generate the Fiat-Shamir/QROM challenge-budget memo."""

from __future__ import annotations

import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
PRODUCTION_REFERENCE_LIB = ROOT / "tools/parameter-search/src/production_reference.rs"
DEFAULT_JSON = READINESS_DIR / "qrom_challenge_budget_memo_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "qrom_challenge_budget_memo_n32_q12289.md"


def rust_string_const(source: str, name: str, fallback: str) -> str:
    match = re.search(rf'pub const {name}:\s*&str\s*=\s*"([^"]+)";', source)
    return match.group(1) if match else fallback


def rust_usize_const(source: str, name: str, fallback: int) -> int:
    match = re.search(rf"pub const {name}:\s*usize\s*=\s*([0-9_]+);", source)
    return int(match.group(1).replace("_", "")) if match else fallback


def challenge_frame_fields() -> list[dict[str, Any]]:
    return [
        {"label": "domain", "type": "bytes", "secret": False},
        {"label": "profile", "type": "bytes", "secret": False},
        {"label": "q", "type": "u64_le", "secret": False},
        {"label": "groups", "type": "u64_le", "secret": False},
        {"label": "local_pivot", "type": "u64_le", "secret": False},
        {"label": "local_free", "type": "u64_le", "secret": False},
        {"label": "repetitions", "type": "u64_le", "secret": False},
        {"label": "slot_id", "type": "u64_le", "secret": False},
        {"label": "public_key", "type": "bytes", "secret": False},
        {"label": "message", "type": "bytes", "secret": False},
        {"label": "slot_digest", "type": "bytes32", "secret": False},
        {"label": "proof_commitment", "type": "bytes32", "secret": False},
    ]


def build_payload() -> dict[str, Any]:
    source = PRODUCTION_REFERENCE_LIB.read_text()
    algebraic_bytes = rust_usize_const(source, "ALGEBRAIC_CHALLENGE_BYTES", 17)
    hardening_bytes = rust_usize_const(source, "CHALLENGE_HARDENING_BYTES", 4)
    bits_per_rep = rust_usize_const(source, "CHALLENGE_BITS_PER_REP", 5)
    reps = rust_usize_const(source, "REPS", 26)
    algebraic_bits = reps * bits_per_rep
    hardening_bits = hardening_bytes * 8
    challenge_bits = algebraic_bits + hardening_bits
    deployment_quantum_queries_log2 = 16
    deployment_classical_queries_log2 = 33
    single_term_quantum_bound_log2 = (challenge_bits - 128) // 2
    single_term_classical_bound_log2 = challenge_bits - 128
    qrom_quadratic_loss_bits = 2 * deployment_quantum_queries_log2
    return {
        "schema": "Descender.qrom_challenge_budget_memo.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source": str(PRODUCTION_REFERENCE_LIB.relative_to(ROOT)),
        "assumption": {
            "name": "Fiat-Shamir challenge frame as SHA-256 QROM",
            "scope": (
                "SHA-256 is modeled as a quantum random oracle only for the "
                "proof-bound challenge frame. Pad expansion and matrix sampling "
                "are separate assumptions."
            ),
            "not_claimed": [
                "standard-model proof of SHA-256",
                "constant-time implementation security",
                "security of the legacy 254-byte public quotient verifier",
            ],
        },
        "challenge_frame": {
            "domain": rust_string_const(
                source,
                "CHALLENGE_HASH_DOMAIN",
                "Descender.Signature.q12289.local_delta.challenge.v1",
            ),
            "profile": rust_string_const(
                source,
                "PROFILE",
                "Descender.Signature.q12289.local_delta.stateful.v1",
            ),
            "encoding": "repeat label_len:u16_le || label || data_len:u64_le || data",
            "fields": challenge_frame_fields(),
            "first_message_field": "proof_commitment",
            "first_message_bytes": 32,
            "verifier_recomputes_frame": True,
        },
        "challenge_derivation": {
            "hash": "SHA-256",
            "output_bytes_taken": algebraic_bytes + hardening_bytes,
            "algebraic_challenge_bytes": algebraic_bytes,
            "hardening_bytes": hardening_bytes,
            "repetitions": reps,
            "bits_per_repetition": bits_per_rep,
            "algebraic_challenge_bits": algebraic_bits,
            "hardening_bits": hardening_bits,
            "total_transcript_challenge_bits": challenge_bits,
            "algebraic_last_byte_mask": "0x03",
        },
        "query_budget": {
            "target_security_bits": 128,
            "single_term_quantum_query_bound_log2_for_128": single_term_quantum_bound_log2,
            "single_term_classical_query_bound_log2_for_128": single_term_classical_bound_log2,
            "deployment_quantum_hash_query_bound_log2": deployment_quantum_queries_log2,
            "deployment_classical_hash_query_bound_log2": deployment_classical_queries_log2,
            "qrom_quadratic_loss_bits_at_deployment_bound": qrom_quadratic_loss_bits,
            "transcript_term_bits_after_qrom_loss": challenge_bits
            - qrom_quadratic_loss_bits,
            "review_position": (
                "The deployment ledger reserves one bit of margin below the "
                "single-term 2^17 quantum-query bound and uses q <= 2^16 for "
                "the QROM challenge-frame oracle."
            ),
        },
        "adversary_interface": {
            "hash_oracle_query": "arbitrary byte string to SHA-256 challenge-frame oracle",
            "signing_query": (
                "message plus an unused account slot; signer must choose the "
                "proof commitment before deriving the challenge"
            ),
            "forgery_output": (
                "public key, account state/slot, message, and serialized "
                "signature whose challenge bytes equal the recomputed frame hash"
            ),
            "charged_bad_events": [
                "QROM measure-and-reprogram/fork-generation failure",
                "same-commitment fork not obtained",
                "Fq12 challenge-action collision",
                "frame mismatch between byte verifier and proof oracle",
            ],
        },
        "review_actions": [
            "Check every frame label and length encoding against production_reference.rs.",
            "Confirm proof_commitment is prechallenge first-message data.",
            "Confirm verifier recomputes challenge bytes from the same frame before acceptance.",
            "Review whether q <= 2^16 quantum challenge-frame queries is an acceptable deployment ledger.",
            "Keep pad PRF and matrix rejection-sampling assumptions separate from this QROM challenge-frame assumption.",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    frame = payload["challenge_frame"]
    derivation = payload["challenge_derivation"]
    budget = payload["query_budget"]
    lines = [
        "# QROM Challenge Budget Memo",
        "",
        "This memo fixes the byte-level Fiat-Shamir/QROM assumption for the",
        "account-carried committed-sigma target.  It is an assumption memo, not a",
        "proof of SHA-256.",
        "",
        "## Scope",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- source: `{payload['source']}`",
        f"- domain: `{frame['domain']}`",
        f"- profile: `{frame['profile']}`",
        f"- frame encoding: `{frame['encoding']}`",
        "",
        "## Frame Fields",
        "",
        "| label | type | secret? |",
        "|---|---|:---:|",
    ]
    for field in frame["fields"]:
        lines.append(
            f"| {field['label']} | {field['type']} | "
            f"{'yes' if field['secret'] else 'no'} |"
        )
    lines.extend(
        [
            "",
            "The `proof_commitment` field is the prechallenge first message bound",
            "into the Fiat-Shamir frame.  The verifier must recompute this same",
            "frame before accepting.",
            "",
            "## Challenge Bytes",
            "",
            f"- SHA-256 bytes taken: `{derivation['output_bytes_taken']}`",
            f"- algebraic challenge bytes: `{derivation['algebraic_challenge_bytes']}`",
            f"- hardening bytes: `{derivation['hardening_bytes']}`",
            f"- repetitions: `{derivation['repetitions']}`",
            f"- bits per repetition: `{derivation['bits_per_repetition']}`",
            f"- algebraic challenge bits: `{derivation['algebraic_challenge_bits']}`",
            f"- transcript-bound challenge bits: `{derivation['total_transcript_challenge_bits']}`",
            f"- algebraic last-byte mask: `{derivation['algebraic_last_byte_mask']}`",
            "",
            "## Query Ledger",
            "",
            f"- target security bits: `{budget['target_security_bits']}`",
            f"- single-term quantum query bound for 128 bits: `2^{budget['single_term_quantum_query_bound_log2_for_128']}`",
            f"- single-term classical query bound for 128 bits: `2^{budget['single_term_classical_query_bound_log2_for_128']}`",
            f"- deployment quantum challenge-frame query bound: `2^{budget['deployment_quantum_hash_query_bound_log2']}`",
            f"- deployment classical challenge-frame query bound: `2^{budget['deployment_classical_hash_query_bound_log2']}`",
            f"- QROM quadratic loss at deployment bound: `{budget['qrom_quadratic_loss_bits_at_deployment_bound']}` bits",
            f"- transcript term after QROM loss: `{budget['transcript_term_bits_after_qrom_loss']}` bits",
            "",
            "## Review Actions",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in payload["review_actions"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
