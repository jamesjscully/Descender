#!/usr/bin/env python3
"""Generate the pad/sampler independence boundary memo."""

from __future__ import annotations

import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
SOURCE = ROOT / "tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs"
DEFAULT_JSON = READINESS_DIR / "pad_sampler_boundary_memo_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pad_sampler_boundary_memo_n32_q12289.md"


def rust_usize_const(source: str, name: str, fallback: int) -> int:
    match = re.search(rf"const {name}:\s*usize\s*=\s*([0-9_]+);", source)
    return int(match.group(1).replace("_", "")) if match else fallback


def rust_bytes_const(source: str, name: str, fallback: str) -> str:
    match = re.search(rf'const {name}:\s*&\[u8\]\s*=\s*b"([^"]+)";', source)
    return match.group(1) if match else fallback


def build_payload() -> dict[str, Any]:
    source = SOURCE.read_text()
    pad_domain = rust_bytes_const(
        source, "PAD_DOMAIN", "Descender.Signature.q12289.local_delta.pad.v1"
    )
    pad_seed_bytes = rust_usize_const(source, "PAD_SEED_BYTES", 32)
    fiber_state_bytes = rust_usize_const(source, "FIBER_STATE_BYTES", 2)
    local_opening_bytes = rust_usize_const(source, "LOCAL_OPENING_BYTES_PER_REP", 4)
    q = 12_289
    public_rows = 12
    full_response_residues = 23
    fixed_state_rows = 2
    sampled_free_residues = full_response_residues - public_rows - fixed_state_rows
    return {
        "schema": "Descender.pad_sampler_boundary_memo.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source": str(SOURCE.relative_to(ROOT)),
        "pad_prf_assumption": {
            "game": "SHA256PadPRFGame",
            "lean_assumption": "SHA256RejectionPadPRFAssumption",
            "domain": pad_domain,
            "pad_frame": "pad-domain[45] || block-index[1]",
            "pad_frame_bytes": len(pad_domain) + 1,
            "pad_seed_bytes": pad_seed_bytes,
            "counter_encoding": "u32_le",
            "local_opening_bytes_per_repetition": local_opening_bytes,
            "residue_decoding": "four SHA-256-derived bytes -> two little-endian q residues",
            "rejection_cutoff": 61_445,
            "q": q,
        },
        "conditioned_quotient_fiber_sampler": {
            "sampler_name": "direct-conditioned-quotient-fiber-sampler",
            "public_syndrome_rows": public_rows,
            "full_response_residues": full_response_residues,
            "fixed_state_rows": fixed_state_rows,
            "fixed_state_constraints": [
                "residue0 equals the 2-byte local fiber state",
                "residue1 is sampled from the packed low-bit class",
            ],
            "sampled_free_residues": sampled_free_residues,
            "exact_support_formula": "12289^9 * 3073",
            "retry_loop_required": False,
            "max_retries": 1,
            "conditions_allowed": [
                "canonical 23-residue q packing",
                "fixed 2-byte local fiber-state equality",
                "response bounds/canonicality",
            ],
            "conditions_forbidden": [
                "active pad outputs",
                "active local opening bytes",
                "post-challenge local opening equations",
                "message-dependent acceptance beyond the Fiat-Shamir challenge frame",
            ],
        },
        "composition_rule": {
            "hidden_representative_sampler": (
                "samples inside the quotient fiber before active pad/local "
                "opening checks are applied"
            ),
            "pad_application": (
                "derive pad residues from pad seed and active block, then add "
                "them to the local mask view to serialize opening bytes"
            ),
            "verifier_order": [
                "recompute Fiat-Shamir challenge from prechallenge frame",
                "decode active responses and local openings",
                "check local opening equations after challenge derivation",
            ],
            "security_reason": (
                "The zero-TV quotient-fiber hiding certificate only applies if "
                "active pad outputs are outside the hidden representative "
                "sampler predicate."
            ),
        },
        "review_actions": [
            "Confirm production sampler code exposes an API boundary before pad/local-opening composition.",
            "Add tests that active pad outputs are not inputs to the quotient-fiber sampler.",
            "Audit that pad expansion uses the pinned pad domain, one-byte block index, and u32 counter encoding.",
            "Audit that sampler retry behavior is fixed: no representative-level rejection loop.",
            "Keep SHA-256 pad PRF assumptions separate from Fiat-Shamir QROM and matrix-sampler assumptions.",
        ],
    }


def markdown(payload: dict[str, Any]) -> str:
    pad = payload["pad_prf_assumption"]
    sampler = payload["conditioned_quotient_fiber_sampler"]
    lines = [
        "# Pad/Sampler Boundary Memo",
        "",
        "This memo fixes the implementation boundary between SHA-256-derived local",
        "pads and the conditioned quotient-fiber hidden-response sampler.  It is a",
        "review memo, not a proof of SHA-256 PRF security.",
        "",
        "## Pad PRF",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- source: `{payload['source']}`",
        f"- game: `{pad['game']}`",
        f"- Lean assumption: `{pad['lean_assumption']}`",
        f"- pad domain: `{pad['domain']}`",
        f"- pad frame: `{pad['pad_frame']}`",
        f"- pad frame bytes: `{pad['pad_frame_bytes']}`",
        f"- pad seed bytes: `{pad['pad_seed_bytes']}`",
        f"- counter encoding: `{pad['counter_encoding']}`",
        f"- rejection cutoff: `{pad['rejection_cutoff']}`",
        "",
        "## Hidden-Representative Sampler",
        "",
        f"- sampler: `{sampler['sampler_name']}`",
        f"- public syndrome rows: `{sampler['public_syndrome_rows']}`",
        f"- full response residues: `{sampler['full_response_residues']}`",
        f"- fixed state rows: `{sampler['fixed_state_rows']}`",
        f"- sampled free residues: `{sampler['sampled_free_residues']}`",
        f"- exact support: `{sampler['exact_support_formula']}`",
        f"- retry loop required: `{sampler['retry_loop_required']}`",
        "",
        "Allowed sampler conditions:",
    ]
    lines.extend(f"- {item}" for item in sampler["conditions_allowed"])
    lines.append("")
    lines.append("Forbidden sampler conditions:")
    lines.extend(f"- {item}" for item in sampler["conditions_forbidden"])
    lines.extend(
        [
            "",
            "## Composition Rule",
            "",
            payload["composition_rule"]["security_reason"],
            "",
            "Review actions:",
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
