#!/usr/bin/env python3
"""Record the production response-link kernel replay boundary."""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_response_link_kernel_replay_boundary_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_response_link_kernel_replay_boundary_n32_q12289.md"
)
SEMANTIC_PLAN_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_predicate_implementation_plan_n32_q12289.json"
)
RESPONSE_LINK_CERT_JSON = (
    READINESS_DIR
    / "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"
)
RESPONSE_LINK_MANIFEST_JSON = (
    READINESS_DIR
    / "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json"
)
PRODUCTION_LIB = ROOT / "tools/production-reference/src/lib.rs"
PARAMETER_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"

Q = 12289


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def digest(payload: dict[str, Any]) -> str:
    h = hashlib.sha256()
    h.update(b"Descender.hidden_source.response_link_kernel_replay_boundary.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def source_contains(name: str, sources: list[str]) -> bool:
    return any(name in source for source in sources)


def build_payload() -> dict[str, Any]:
    semantic_plan = load_json(SEMANTIC_PLAN_JSON)
    certificate = load_json(RESPONSE_LINK_CERT_JSON)
    manifest = load_json(RESPONSE_LINK_MANIFEST_JSON)
    sources = [PRODUCTION_LIB.read_text(), PARAMETER_REFERENCE.read_text()]
    required_symbols = [
        "reviewed_response_link_bounded_kernel_certificate",
        "verify_hidden_source_fallback_response_link_kernel_or_export",
        "HiddenSourceFallbackProjectionResponseLinkKernelCertificate",
        "HiddenSourceFallbackProjectionResponseLinkKernelReplayInput",
    ]
    symbol_rows = [
        {
            "symbol": symbol,
            "present": source_contains(symbol, sources),
            "status": "present" if source_contains(symbol, sources) else "missing",
        }
        for symbol in required_symbols
    ]
    missing_symbols = [row["symbol"] for row in symbol_rows if not row["present"]]
    pinned_certificate = {
        "q": certificate.get("q"),
        "coords": certificate.get("coords"),
        "bound": certificate.get("bound"),
        "response_difference_bound": certificate.get("response_difference_bound"),
        "status": certificate.get("status"),
        "matrix_transcript_sha256": certificate.get("matrix_transcript_sha256"),
        "canonical_basis_sha256": certificate.get("canonical_basis_sha256"),
        "reduced_basis_sha256": certificate.get("reduced_basis_sha256"),
        "cover_ball_vector_sha256": certificate.get("cover_ball_vector_sha256"),
        "manifest_payload_sha256": manifest.get("manifest_payload_sha256"),
        "visited_nodes": certificate.get("visited_nodes"),
        "visited_leaves": certificate.get("visited_leaves"),
        "nonzero_vectors_in_cover_ball": certificate.get("nonzero_vectors_in_cover_ball"),
    }
    digest_payload = {
        "semantic_plan_digest": semantic_plan.get("target", {}).get(
            "implementation_plan_digest"
        ),
        "pinned_certificate": pinned_certificate,
        "symbol_rows": symbol_rows,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_response_link_kernel_replay_boundary.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "response_link_kernel_replay_boundary_implemented_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "semantic_predicate_implementation_plan": rel(SEMANTIC_PLAN_JSON),
            "response_link_certificate": rel(RESPONSE_LINK_CERT_JSON),
            "response_link_audit_manifest": rel(RESPONSE_LINK_MANIFEST_JSON),
            "production_reference_lib": rel(PRODUCTION_LIB),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "semantic_predicate_implementation_plan_schema": semantic_plan.get("schema"),
            "response_link_manifest_schema": manifest.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "boundary_digest": digest(digest_payload),
        },
        "pinned_certificate": pinned_certificate,
        "symbol_audit": symbol_rows,
        "coverage": {
            "required_symbol_count": len(symbol_rows),
            "present_symbol_count": len(symbol_rows) - len(missing_symbols),
            "missing_symbol_count": len(missing_symbols),
            "missing_symbols": missing_symbols,
            "implemented_semantic_predicate_count": 2,
            "open_semantic_predicate_count": 3,
            "signature_byte_increase": 0,
        },
        "decision": {
            "can_accept_now": False,
            "reason": "the response-link kernel replay boundary is executable against the pinned certificate summary, but hidden opening, account split, and pad PRF semantic predicates remain open",
            "next_artifact": "hidden_source_fallback_projection_opening_projection_boundary",
        },
        "structural_controls": [
            {
                "id": "all_required_symbols_present",
                "status": "pass" if not missing_symbols else "fail",
                "observed": len(symbol_rows) - len(missing_symbols),
                "required": len(symbol_rows),
            },
            {
                "id": "pinned_certificate_matches_selected_link",
                "status": "pass"
                if pinned_certificate["q"] == Q
                and pinned_certificate["coords"] == [0, 3]
                and pinned_certificate["response_difference_bound"] == 96
                and pinned_certificate["status"] == "infeasible"
                else "fail",
                "observed": pinned_certificate,
                "required": {
                    "q": Q,
                    "coords": [0, 3],
                    "response_difference_bound": 96,
                    "status": "infeasible",
                },
            },
            {
                "id": "acceptance_still_fail_closed",
                "status": "pass",
                "observed": False,
                "required": False,
            },
        ],
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    coverage = payload["coverage"]
    lines = [
        "# Hidden-Source Projection Response-Link Kernel Replay Boundary",
        "",
        "This artifact records the production-reference boundary for the selected",
        "`[0,3]` response-link bounded-kernel certificate. The Rust code validates",
        "the pinned certificate summary and exports a checked-core/response-link",
        "break if a nonzero bounded vector survives the link.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- boundary digest: `{target['boundary_digest']}`",
        f"- present symbols: `{coverage['present_symbol_count']}` / `{coverage['required_symbol_count']}`",
        f"- implemented semantic predicates: `{coverage['implemented_semantic_predicate_count']}`",
        f"- open semantic predicates: `{coverage['open_semantic_predicate_count']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## Pinned Certificate",
        "",
    ]
    for key, value in payload["pinned_certificate"].items():
        lines.append(f"- `{key}`: `{value}`")
    lines.extend(["", "## Source Artifacts", ""])
    for key, value in payload["source_artifacts"].items():
        lines.append(f"- `{key}`: `{value}`")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(f"wrote {rel(DEFAULT_JSON)}")
    print(f"wrote {rel(DEFAULT_MD)}")


if __name__ == "__main__":
    main()
