#!/usr/bin/env python3
"""Generate a checkable pad/sampler boundary review object.

This consumes the pad/sampler boundary memo and checks the concrete arithmetic
and API-boundary invariants that keep pad/local-opening data out of the hidden
representative sampler.  SHA-256 PRF security remains an explicit external
assumption; this object makes the implementation boundary reviewable.
"""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
PAD_MEMO = READINESS_DIR / "pad_sampler_boundary_memo_n32_q12289.json"
SOURCE = ROOT / "tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs"
DEFAULT_JSON = READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.md"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def checked(condition: bool, description: str, evidence: dict[str, Any]) -> dict[str, Any]:
    return {
        "description": description,
        "holds": condition,
        "evidence": evidence,
    }


def source_contains_all(source: str, needles: list[str]) -> bool:
    return all(needle in source for needle in needles)


def build_payload() -> dict[str, Any]:
    memo = load_json(PAD_MEMO)
    source = SOURCE.read_text()
    pad = memo["pad_prf_assumption"]
    sampler = memo["conditioned_quotient_fiber_sampler"]
    composition = memo["composition_rule"]
    q = int(pad["q"])
    rejection_cutoff = int(pad["rejection_cutoff"])
    expected_cutoff = (2**16 // q) * q
    expected_free_residues = (
        int(sampler["full_response_residues"])
        - int(sampler["public_syndrome_rows"])
        - int(sampler["fixed_state_rows"])
    )
    expected_residue1_allowed = (q + 3) // 4
    expected_support_formula = f"{q}^{expected_free_residues} * {expected_residue1_allowed}"
    allowed = set(sampler["conditions_allowed"])
    forbidden = set(sampler["conditions_forbidden"])

    source_kats = {
        "pad_frame_is_domain_plus_block": "fn pad_frame_is_domain_plus_block" in source,
        "sample_vector_is_stable": "fn sample_vector_is_stable" in source,
        "conditioned_sampler_preserves_digest_and_fiber_state": (
            "fn conditioned_sampler_preserves_digest_and_fiber_state" in source
        ),
        "sampler_certificate_excludes_active_pad_outputs": source_contains_all(
            source,
            [
                ".masked_response_sampler_certificate",
                ".active_pad_outputs_conditioned",
                ".excluded_conditioning",
                '"active pad outputs"',
            ],
        ),
    }

    checks = [
        checked(
            int(pad["pad_frame_bytes"]) == len(pad["domain"]) + 1,
            "pad frame is domain bytes plus one block-index byte",
            {
                "pad_frame_bytes": pad["pad_frame_bytes"],
                "domain_len": len(pad["domain"]),
            },
        ),
        checked(
            pad["counter_encoding"] == "u32_le",
            "pad rejection sampler counter encoding is pinned to u32_le",
            {"counter_encoding": pad["counter_encoding"]},
        ),
        checked(
            rejection_cutoff == expected_cutoff,
            "pad rejection cutoff is floor(2^16/q)*q",
            {
                "rejection_cutoff": rejection_cutoff,
                "expected_cutoff": expected_cutoff,
                "q": q,
            },
        ),
        checked(
            int(pad["local_opening_bytes_per_repetition"]) == 4,
            "each active local opening serializes two q residues as four bytes",
            {
                "local_opening_bytes_per_repetition": pad[
                    "local_opening_bytes_per_repetition"
                ]
            },
        ),
        checked(
            int(sampler["sampled_free_residues"]) == expected_free_residues,
            "sampled free residues equal full response minus public rows and fixed state rows",
            {
                "sampled_free_residues": sampler["sampled_free_residues"],
                "full_response_residues": sampler["full_response_residues"],
                "public_syndrome_rows": sampler["public_syndrome_rows"],
                "fixed_state_rows": sampler["fixed_state_rows"],
            },
        ),
        checked(
            sampler["exact_support_formula"] == expected_support_formula,
            "exact support formula matches q^free_residues times low-bit-class count",
            {
                "exact_support_formula": sampler["exact_support_formula"],
                "expected_support_formula": expected_support_formula,
            },
        ),
        checked(
            sampler["retry_loop_required"] is False and int(sampler["max_retries"]) == 1,
            "conditioned quotient-fiber sampler has no representative-level retry loop",
            {
                "retry_loop_required": sampler["retry_loop_required"],
                "max_retries": sampler["max_retries"],
            },
        ),
        checked(
            allowed.isdisjoint(forbidden),
            "allowed sampler conditions are disjoint from forbidden active conditions",
            {
                "allowed_conditions": sorted(allowed),
                "forbidden_conditions": sorted(forbidden),
            },
        ),
        checked(
            "active pad outputs" in forbidden
            and "active local opening bytes" in forbidden
            and "post-challenge local opening equations" in forbidden,
            "active pad/opening data is explicitly forbidden from sampler conditioning",
            {"forbidden_conditions": sorted(forbidden)},
        ),
        checked(
            composition["verifier_order"]
            == [
                "recompute Fiat-Shamir challenge from prechallenge frame",
                "decode active responses and local openings",
                "check local opening equations after challenge derivation",
            ],
            "verifier order keeps local opening checks after challenge derivation",
            {"verifier_order": composition["verifier_order"]},
        ),
        checked(
            all(source_kats.values()),
            "source contains KATs for pad frame, stable vector, sampler preservation, and pad exclusion",
            source_kats,
        ),
    ]
    failed_checks = [item for item in checks if not item["holds"]]
    return {
        "schema": "Descender.pad_sampler_boundary_review.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source_memo": rel(PAD_MEMO),
        "source": rel(SOURCE),
        "review_scope": {
            "claim": (
                "The local pad/opening layer is composed outside the hidden "
                "representative sampler, and the concrete pad/sampler arithmetic "
                "matches the reviewed q=12289 boundary."
            ),
            "conditional_on": [
                "SHA-256 pad expansion is pseudorandom under the named PadPRFBinding assumption",
                "the final production signer preserves this API boundary",
                "constant-time sampler/keygen review closes separately",
            ],
            "not_claimed": [
                "proof of SHA-256 PRF security",
                "constant-time implementation security",
                "deployment readiness before the selected 308-byte/Fq12 projection-carrier semantics land",
            ],
        },
        "source_kats": source_kats,
        "checks": checks,
        "failed_checks": failed_checks,
        "status": "checks_passed" if not failed_checks else "checks_failed",
        "external_review_needed": [
            "Review the final production sampler API for the same forbidden-condition boundary.",
            "Review SHA-256 pad PRF domain separation and rejection-sampling independence.",
            "Add differential tests once the production crate replaces the research fixture.",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pad/Sampler Boundary Review",
        "",
        "This generated review object checks the concrete pad/sampler arithmetic",
        "and API-boundary invariants from the pad/sampler memo.  It does not prove",
        "SHA-256 PRF security.",
        "",
        "## Summary",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- source memo: `{payload['source_memo']}`",
        f"- source: `{payload['source']}`",
        f"- status: `{payload['status']}`",
        f"- failed checks: `{len(payload['failed_checks'])}`",
        "",
        "## Checks",
        "",
        "| check | holds | evidence |",
        "|---|:---:|---|",
    ]
    for item in payload["checks"]:
        evidence = ", ".join(f"{key}={value}" for key, value in item["evidence"].items())
        lines.append(
            f"| {item['description']} | {'yes' if item['holds'] else 'no'} | `{evidence}` |"
        )
    lines.extend(["", "## External Review Still Needed", ""])
    lines.extend(f"- {item}" for item in payload["external_review_needed"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))
    if payload["failed_checks"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
