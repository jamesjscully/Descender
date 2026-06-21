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


def rust_int_const(source: str, name: str, fallback: int) -> int:
    match = re.search(
        rf"pub const {name}:\s*(?:u(?:8|16|32|64)|usize|i(?:8|16|32|64))\s*=\s*(0x[0-9a-fA-F_]+|[0-9_]+);",
        source,
    )
    return int(match.group(1).replace("_", ""), 0) if match else fallback


def rust_budget_field(source: str, name: str, fallback: int) -> int:
    match = re.search(rf"\b{name}:\s*([0-9_]+),", source)
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
    algebraic_bytes = rust_usize_const(source, "ALGEBRAIC_CHALLENGE_BYTES", 23)
    hardening_bytes = rust_usize_const(source, "CHALLENGE_HARDENING_BYTES", 8)
    algebraic_mask = rust_int_const(source, "ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK", 0x3F)
    bits_per_rep = rust_usize_const(source, "CHALLENGE_BITS_PER_REP", 5)
    reps = rust_usize_const(source, "REPS", 26)
    decoded_local_control_bits = reps * bits_per_rep
    algebraic_bits = (algebraic_bytes - 1) * 8 + algebraic_mask.bit_count()
    hardening_bits = hardening_bytes * 8
    challenge_bits = algebraic_bits + hardening_bits
    target_security_bits = rust_budget_field(source, "target_security_bits", 128)
    deployment_quantum_queries_log2 = rust_budget_field(
        source, "deployment_quantum_hash_query_bound_log2", 16
    )
    deployment_classical_queries_log2 = rust_budget_field(
        source, "deployment_classical_hash_query_bound_log2", 33
    )
    qrom_quadratic_loss_bits = rust_budget_field(
        source, "qrom_quadratic_loss_bits_at_deployment_bound", 32
    )
    transcript_term_bits_after_qrom_loss = rust_budget_field(
        source, "transcript_term_bits_after_qrom_loss", 182
    )
    selected_multi_target_horizon_log2 = rust_budget_field(
        source, "selected_multi_target_horizon_log2", 32
    )
    minimum_deployment_margin_bits = rust_budget_field(
        source, "minimum_deployment_margin_bits", 53
    )
    negative_control_horizon_log2 = rust_budget_field(
        source, "negative_control_horizon_log2", 54
    )
    single_term_quantum_bound_log2 = (challenge_bits - target_security_bits) // 2
    single_term_classical_bound_log2 = challenge_bits - target_security_bits
    return {
        "schema": "Descender.qrom_challenge_budget_memo.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source": str(PRODUCTION_REFERENCE_LIB.relative_to(ROOT)),
        "assumption": {
            "name": "Fiat-Shamir transaction challenge as Poseidon2 QROM",
            "scope": (
                "The transaction Fiat-Shamir challenge sponge is modeled as a "
                "quantum random oracle under the Poseidon2 transaction domain. "
                "The byte frame still includes the historical CHALLENGE_HASH_DOMAIN "
                "field; that is frame data, not the active sponge primitive. Pad "
                "expansion, matrix sampling, and public consensus-object SHA-256 "
                "digests are separate assumptions."
            ),
            "not_claimed": [
                "standard-model proof of Poseidon2",
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
            "hash": "Poseidon2",
            "sponge_domain": rust_string_const(
                source,
                "PATH2_TXN_FS_POSEIDON2_DOMAIN",
                "Descender.Path2.transaction.fiat_shamir.poseidon2.v1",
            ),
            "output_bytes_taken": algebraic_bytes + hardening_bytes,
            "algebraic_challenge_bytes": algebraic_bytes,
            "hardening_bytes": hardening_bytes,
            "repetitions": reps,
            "bits_per_repetition": bits_per_rep,
            "decoded_local_control_bits": decoded_local_control_bits,
            "algebraic_challenge_bits": algebraic_bits,
            "hardening_bits": hardening_bits,
            "total_transcript_challenge_bits": challenge_bits,
            "algebraic_last_byte_mask": f"0x{algebraic_mask:02x}",
        },
        "query_budget": {
            "target_security_bits": target_security_bits,
            "single_term_quantum_query_bound_log2_for_128": single_term_quantum_bound_log2,
            "single_term_classical_query_bound_log2_for_128": single_term_classical_bound_log2,
            "deployment_quantum_hash_query_bound_log2": deployment_quantum_queries_log2,
            "deployment_classical_hash_query_bound_log2": deployment_classical_queries_log2,
            "qrom_quadratic_loss_bits_at_deployment_bound": qrom_quadratic_loss_bits,
            "selected_multi_target_horizon_log2": selected_multi_target_horizon_log2,
            "transcript_term_bits_after_qrom_loss": transcript_term_bits_after_qrom_loss,
            "minimum_deployment_margin_bits": minimum_deployment_margin_bits,
            "negative_control_horizon_log2": negative_control_horizon_log2,
            "review_position": (
                "The active reference object prices the 246-bit masked challenge "
                "down to a 182-bit selected deployment transcript term after the "
                "current QROM and multi-target ledger. This memo deliberately "
                "separates that transcript budget from the 130 decoded local-control "
                "bits consumed by the parser."
            ),
        },
        "adversary_interface": {
            "hash_oracle_query": "arbitrary byte string to the Poseidon2 transaction challenge sponge under the recorded domain",
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
            "Review whether q <= 2^16 quantum challenge-sponge queries plus the selected multi-target horizon is an acceptable deployment ledger.",
            "Keep pad PRF, matrix rejection sampling, and ordinary SHA-256 public-object digests separate from this QROM challenge-frame assumption.",
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
        "proof of Poseidon2.",
        "",
        "## Scope",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- source: `{payload['source']}`",
        f"- frame domain field: `{frame['domain']}`",
        f"- challenge sponge domain: `{derivation['sponge_domain']}`",
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
            f"- challenge primitive: `{derivation['hash']}`",
            f"- challenge bytes taken: `{derivation['output_bytes_taken']}`",
            f"- algebraic challenge bytes: `{derivation['algebraic_challenge_bytes']}`",
            f"- hardening bytes: `{derivation['hardening_bytes']}`",
            f"- repetitions: `{derivation['repetitions']}`",
            f"- bits per repetition: `{derivation['bits_per_repetition']}`",
            f"- decoded local-control bits: `{derivation['decoded_local_control_bits']}`",
            f"- algebraic byte/mask challenge bits: `{derivation['algebraic_challenge_bits']}`",
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
            f"- selected multi-target horizon: `2^{budget['selected_multi_target_horizon_log2']}`",
            f"- selected deployment transcript term: `{budget['transcript_term_bits_after_qrom_loss']}` bits",
            f"- minimum deployment margin: `{budget['minimum_deployment_margin_bits']}` bits",
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
