#!/usr/bin/env python3
"""Check active QROM budget artifacts against the current Rust reference.

The generated QROM reports live under research/ and are intentionally ignored by
the repository's generated-report rules. This checker makes them usable as a
disciplined build artifact: regenerate them first, then verify the contents here.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"

QROM_MEMO = READINESS_DIR / "qrom_challenge_budget_memo_n32_q12289.json"
QROM_REVIEW = READINESS_DIR / "qrom_security_loss_review_n32_q12289.json"
QROM_FRAME_PROBE = READINESS_DIR / "pre_third_party_qrom_frame_probe_n32_q12289.json"
QROM_MULTITARGET = READINESS_DIR / "qrom_multitarget_margin_n32_q12289.json"


def load_json(path: Path) -> dict[str, Any]:
    if not path.exists():
        raise SystemExit(
            f"FAIL: missing {path.relative_to(ROOT)}; regenerate QROM artifacts first"
        )
    return json.loads(path.read_text(encoding="utf-8"))


def rust_int(source: str, name: str) -> int:
    match = re.search(
        rf"pub const {name}:\s*(?:u(?:8|16|32|64)|usize|i(?:8|16|32|64))\s*=\s*(0x[0-9a-fA-F_]+|[0-9_]+);",
        source,
    )
    if not match:
        raise SystemExit(f"FAIL: Rust const {name} not found")
    return int(match.group(1).replace("_", ""), 0)


def rust_str(source: str, name: str) -> str:
    match = re.search(rf'pub const {name}:\s*&str\s*=\s*"([^"]+)";', source)
    if not match:
        raise SystemExit(f"FAIL: Rust string const {name} not found")
    return match.group(1)


def rust_budget_field(source: str, name: str) -> int:
    match = re.search(rf"\b{name}:\s*([0-9_]+),", source)
    if not match:
        raise SystemExit(f"FAIL: Rust budget field {name} not found")
    return int(match.group(1).replace("_", ""))


def check_equal(errors: list[str], label: str, actual: Any, expected: Any) -> None:
    if actual != expected:
        errors.append(f"{label}: got {actual!r}, expected {expected!r}")
    else:
        print(f"[OK ] {label}: {actual!r}")


def main() -> int:
    source = PRODUCTION_REFERENCE.read_text(encoding="utf-8")
    memo = load_json(QROM_MEMO)
    review = load_json(QROM_REVIEW)
    frame_probe = load_json(QROM_FRAME_PROBE)
    multitarget = load_json(QROM_MULTITARGET)

    errors: list[str] = []

    algebraic_bytes = rust_int(source, "ALGEBRAIC_CHALLENGE_BYTES")
    hardening_bytes = rust_int(source, "CHALLENGE_HARDENING_BYTES")
    algebraic_mask = rust_int(source, "ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK")
    reps = rust_int(source, "REPS")
    bits_per_rep = rust_int(source, "CHALLENGE_BITS_PER_REP")
    expected_algebraic_bits = (algebraic_bytes - 1) * 8 + algebraic_mask.bit_count()
    expected_hardening_bits = 8 * hardening_bytes
    expected_total_bits = expected_algebraic_bits + expected_hardening_bits
    expected_decoded_bits = reps * bits_per_rep
    selected_horizon = rust_budget_field(source, "selected_multi_target_horizon_log2")
    qrom_loss = rust_budget_field(source, "qrom_quadratic_loss_bits_at_deployment_bound")
    selected_term = rust_budget_field(source, "transcript_term_bits_after_qrom_loss")
    target_bits = rust_budget_field(source, "target_security_bits")
    expected_fail_closed_cap = expected_algebraic_bits - target_bits

    derivation = memo["challenge_derivation"]
    budget = memo["query_budget"]
    check_equal(errors, "memo hash", derivation.get("hash"), "Poseidon2")
    check_equal(
        errors,
        "memo sponge domain",
        derivation.get("sponge_domain"),
        rust_str(source, "PATH2_TXN_FS_POSEIDON2_DOMAIN"),
    )
    check_equal(errors, "memo output bytes", derivation.get("output_bytes_taken"), algebraic_bytes + hardening_bytes)
    check_equal(errors, "memo decoded local-control bits", derivation.get("decoded_local_control_bits"), expected_decoded_bits)
    check_equal(errors, "memo algebraic byte/mask bits", derivation.get("algebraic_challenge_bits"), expected_algebraic_bits)
    check_equal(errors, "memo hardening bits", derivation.get("hardening_bits"), expected_hardening_bits)
    check_equal(errors, "memo total transcript bits", derivation.get("total_transcript_challenge_bits"), expected_total_bits)
    check_equal(errors, "memo last byte mask", derivation.get("algebraic_last_byte_mask"), f"0x{algebraic_mask:02x}")
    check_equal(errors, "memo selected multi-target horizon", budget.get("selected_multi_target_horizon_log2"), selected_horizon)
    check_equal(errors, "memo selected transcript term", budget.get("transcript_term_bits_after_qrom_loss"), selected_term)

    expected_selected_term = expected_total_bits - qrom_loss - selected_horizon
    check_equal(errors, "derived selected transcript term", selected_term, expected_selected_term)
    check_equal(errors, "review status", review.get("status"), "checks_passed")
    check_equal(errors, "review failed checks", review.get("failed_checks"), [])
    check_equal(
        errors,
        "review selected transcript term",
        review.get("derived_terms", {}).get("selected_transcript_term_bits"),
        selected_term,
    )
    check_equal(
        errors,
        "review selected transcript margin",
        review.get("derived_terms", {}).get("selected_transcript_margin_bits"),
        selected_term - target_bits,
    )

    check_equal(errors, "frame probe status", frame_probe.get("status"), "checks_passed")
    check_equal(
        errors,
        "frame probe challenge model",
        frame_probe.get("frame_spec", {}).get("challenge_value_model"),
        "sha256_frame_mutation_canary_not_active_challenge",
    )
    check_equal(
        errors,
        "frame probe max target cap",
        frame_probe.get("query_budget", {}).get("max_target_count_log2_at_128_bits"),
        expected_fail_closed_cap,
    )

    check_equal(
        errors,
        "multi-target status",
        multitarget.get("status"),
        "closed_by_fail_closed_deployment_cap",
    )
    check_equal(
        errors,
        "multi-target ready flag",
        multitarget.get("deployment_ready_for_realistic_multitarget"),
        True,
    )
    check_equal(
        errors,
        "multi-target fail-closed cap",
        multitarget.get("fail_closed_limit", {}).get("maximum_covered_target_count_log2"),
        expected_fail_closed_cap,
    )
    check_equal(
        errors,
        "multi-target first failing table row",
        multitarget.get("fail_closed_limit", {}).get("first_failing_target_count_log2_in_table"),
        expected_fail_closed_cap + 1,
    )

    if errors:
        print("FAIL: QROM budget artifacts are not synchronized with the current reference")
        for error in errors:
            print(f"  - {error}")
        return 1
    print("OK: QROM budget artifacts match the current Poseidon2 byte/mask ledger.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
