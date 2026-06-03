#!/usr/bin/env python3
"""Record the remaining semantic invocation contract for projection exports."""

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
    / "hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_fallback_projection_semantic_classifier_invocation_contract_n32_q12289.md"
)
API_STUBS_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_verifier_exporter_api_stubs_n32_q12289.json"
)
VERIFIER_EXPORTER_CONTRACT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_verifier_exporter_contract_n32_q12289.json"
)
BREAK_EXPORT_CONTRACT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_break_export_contract_n32_q12289.json"
)
NEGATIVE_TESTS_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json"
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
    h.update(b"Descender.hidden_source.semantic_classifier_invocation_contract.v1")
    h.update(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode())
    return h.hexdigest()


def source_contains(name: str, sources: list[str]) -> bool:
    return any(name in source for source in sources)


def build_payload() -> dict[str, Any]:
    api = load_json(API_STUBS_JSON)
    verifier = load_json(VERIFIER_EXPORTER_CONTRACT_JSON)
    export_contract = load_json(BREAK_EXPORT_CONTRACT_JSON)
    negative = load_json(NEGATIVE_TESTS_JSON)
    sources = [PRODUCTION_LIB.read_text(), PARAMETER_REFERENCE.read_text()]
    export_records = {row["id"]: row for row in export_contract.get("export_records", [])}
    routing_rules = verifier.get("routing_rules", [])
    invocation_rules = [
        {
            "id": "checked_core_response_relation_mismatch",
            "classifier": "classify_hidden_source_fallback_checked_core_break",
            "failure_variant": "ResponseRelationMismatch",
            "required_export": "checked_core_or_response_link_break_export",
            "must_be_invoked_after": [
                "carrier_decode_accepts",
                "public_bridge_predicate_accepts",
                "checked_core_equation_replay_fails",
            ],
            "implemented_boundary": True,
            "semantic_predicate_implemented": source_contains(
                "verify_hidden_source_fallback_checked_core_or_export", sources
            ),
        },
        {
            "id": "response_link_kernel_certificate_mismatch",
            "classifier": "classify_hidden_source_fallback_checked_core_break",
            "failure_variant": "ResponseLinkKernelCertificateMismatch",
            "required_export": "checked_core_or_response_link_break_export",
            "must_be_invoked_after": [
                "carrier_decode_accepts",
                "public_bridge_predicate_accepts",
                "bounded_kernel_certificate_replay_fails",
            ],
            "implemented_boundary": True,
            "semantic_predicate_implemented": source_contains(
                "verify_hidden_source_fallback_response_link_kernel_or_export",
                sources,
            ),
        },
        {
            "id": "hidden_source_opening_projection_mismatch",
            "classifier": "classify_hidden_source_fallback_opening_break",
            "failure_variant": "OpeningProjectionMismatch",
            "required_export": "hidden_source_opening_binding_break_export",
            "must_be_invoked_after": [
                "carrier_decode_accepts",
                "public_bridge_predicate_accepts",
                "hidden_relation_opening_replay_fails_projection_equality",
            ],
            "implemented_boundary": True,
            "semantic_predicate_implemented": source_contains(
                "verify_hidden_source_fallback_hidden_material_frame_projection_or_export",
                sources,
            ),
        },
        {
            "id": "account_commitment_split_opening",
            "classifier": "classify_hidden_source_fallback_opening_break",
            "failure_variant": "AccountCommitmentSplitOpening",
            "required_export": "account_commitment_binding_break_export",
            "must_be_invoked_after": [
                "same_account_commitment_has_two_accepted_openings",
                "semantic_values_differ",
            ],
            "implemented_boundary": True,
            "semantic_predicate_implemented": source_contains(
                "verify_hidden_source_fallback_account_commitment_split_frames_or_export",
                sources,
            ),
        },
        {
            "id": "pad_prf_projection_derivation_mismatch",
            "classifier": "classify_hidden_source_fallback_opening_break",
            "failure_variant": "PadPrfProjectionDerivationMismatch",
            "required_export": "pad_prf_or_pad_opening_break_export",
            "must_be_invoked_after": [
                "pad_opening_accepts",
                "pad_prf_or_projection_replay_fails",
            ],
            "implemented_boundary": True,
            "semantic_predicate_implemented": source_contains(
                "verify_hidden_source_fallback_pad_prf_seed_or_export", sources
            ),
        },
    ]
    missing_exports = sorted(
        {
            row["required_export"]
            for row in invocation_rules
            if row["required_export"] not in export_records
        }
    )
    implemented_boundaries = sum(1 for row in invocation_rules if row["implemented_boundary"])
    implemented_semantics = sum(
        1 for row in invocation_rules if row["semantic_predicate_implemented"]
    )
    digest_payload = {
        "api_stub_digest": api.get("target", {}).get("api_stub_digest"),
        "contract_digest": verifier.get("target", {}).get("contract_digest"),
        "break_export_contract_digest": export_contract.get("target", {}).get(
            "contract_digest"
        ),
        "negative_test_digest": negative.get("target", {}).get("negative_test_digest"),
        "invocation_rules": invocation_rules,
    }
    return {
        "schema": "Descender.hidden_source_fallback_projection_semantic_classifier_invocation_contract.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": (
            "semantic_classifier_invocation_contract_implemented_fail_closed"
            if implemented_semantics == len(invocation_rules)
            else "semantic_classifier_invocation_contract_open_fail_closed"
        ),
        "deployment_ready": False,
        "source_artifacts": {
            "verifier_exporter_api_stubs": rel(API_STUBS_JSON),
            "verifier_exporter_contract": rel(VERIFIER_EXPORTER_CONTRACT_JSON),
            "break_export_contract": rel(BREAK_EXPORT_CONTRACT_JSON),
            "break_export_negative_tests": rel(NEGATIVE_TESTS_JSON),
            "production_reference_lib": rel(PRODUCTION_LIB),
            "parameter_reference_lib": rel(PARAMETER_REFERENCE),
            "verifier_exporter_api_stubs_schema": api.get("schema"),
            "verifier_exporter_contract_schema": verifier.get("schema"),
            "break_export_contract_schema": export_contract.get("schema"),
            "break_export_negative_tests_schema": negative.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "selected_branch": "fail-closed `308 B/Fq12` projection-carrier hidden-source envelope",
            "envelope_bytes": 308,
            "carrier_bytes": 56,
            "invocation_contract_digest": digest(digest_payload),
        },
        "invocation_rules": invocation_rules,
        "coverage": {
            "invocation_rule_count": len(invocation_rules),
            "implemented_classifier_boundary_count": implemented_boundaries,
            "implemented_semantic_predicate_count": implemented_semantics,
            "missing_required_export_records": missing_exports,
            "negative_test_count": negative.get("coverage", {}).get("test_count"),
            "routed_negative_test_count": verifier.get("coverage", {}).get(
                "routed_negative_test_count"
            ),
            "signature_byte_increase": 0,
        },
        "decision": {
            "can_accept_now": False,
            "reason": (
                "all classifier export boundaries and semantic predicate "
                "invocations are executable; acceptance remains fail-closed "
                "until the exact bridge certificate/map survivor is resolved"
                if implemented_semantics == len(invocation_rules)
                else "all classifier export boundaries exist, but the verifier still lacks executable semantic predicates that decide when to invoke them"
            ),
            "next_artifact": "hidden_source_fallback_projection_semantic_predicate_implementation_plan",
        },
        "structural_controls": [
            {
                "id": "all_named_export_records_available",
                "status": "pass" if not missing_exports else "fail",
                "observed": missing_exports,
                "required": [],
            },
            {
                "id": "all_classifier_boundaries_implemented",
                "status": "pass" if implemented_boundaries == len(invocation_rules) else "fail",
                "observed": implemented_boundaries,
                "required": len(invocation_rules),
            },
            {
                "id": "semantic_predicates_still_open",
                "status": "pass"
                if implemented_semantics == len(invocation_rules)
                else "expected_open",
                "observed": implemented_semantics,
                "required": len(invocation_rules),
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
        "# Hidden-Source Projection Semantic Classifier Invocation Contract",
        "",
        "This artifact records the remaining verifier work after the classifier",
        "export boundaries were added. The implementation can now package typed",
        "break records, but it still needs executable semantic predicates that",
        "decide when each classifier is invoked.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected branch: {target['selected_branch']}",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- invocation contract digest: `{target['invocation_contract_digest']}`",
        f"- invocation rules: `{coverage['invocation_rule_count']}`",
        f"- implemented classifier boundaries: `{coverage['implemented_classifier_boundary_count']}`",
        f"- implemented semantic predicates: `{coverage['implemented_semantic_predicate_count']}`",
        f"- signature byte increase: `{coverage['signature_byte_increase']}`",
        f"- next artifact: `{payload['decision']['next_artifact']}`",
        "",
        "## Invocation Rules",
        "",
        "| rule | classifier | failure variant | export | semantic implemented |",
        "|---|---|---|---|---|",
    ]
    for row in payload["invocation_rules"]:
        lines.append(
            f"| `{row['id']}` | `{row['classifier']}` | `{row['failure_variant']}` | `{row['required_export']}` | `{row['semantic_predicate_implemented']}` |"
        )
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
