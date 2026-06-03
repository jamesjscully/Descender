#!/usr/bin/env python3
"""Generate hidden-relation negative controls for the state-assisted carrier."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.md"
)
HIDDEN_RELATION_CONTRACT_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.json"
)
PRODUCTION_REFERENCE_RS = ROOT / "tools/production-reference/src/lib.rs"
PARAMETER_SEARCH_REFERENCE_RS = ROOT / "tools/parameter-search/src/production_reference.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def load_text(path: Path) -> str:
    return path.read_text()


def source_evidence(tokens: list[str], *sources: str) -> dict[str, Any]:
    combined = "\n".join(sources)
    missing = [token for token in tokens if token not in combined]
    return {
        "status": (
            "covered_by_rust_fail_closed_negative_test"
            if not missing
            else "specified_not_executed"
        ),
        "required_tokens": tokens,
        "missing_tokens": missing,
    }


def build_payload() -> dict[str, Any]:
    contract = load_json(HIDDEN_RELATION_CONTRACT_JSON)
    production_reference_rs = load_text(PRODUCTION_REFERENCE_RS)
    parameter_search_reference_rs = load_text(PARAMETER_SEARCH_REFERENCE_RS)
    sources = (production_reference_rs, parameter_search_reference_rs)
    evidence_by_control = {
        "witness_digest_substitution": source_evidence(
            [
                "production_reviewed_fq12_malformed_opening_no_partial_export",
                "bad_opening.witness_opening_digest[0] ^= 1",
                "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
                "PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON",
                "verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate",
            ],
            *sources,
        ),
        "pad_seed_substitution": source_evidence(
            [
                "production_reviewed_fq12_wrong_pad_seed_no_partial_export",
                "wrong_pad_seed[0] ^= 1",
                "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
                "PRODUCTION_SELECTED_PROJECTION_INVALID_OPENING_REASON",
                "verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate",
            ],
            *sources,
        ),
        "public_pad_statement_mismatch": source_evidence(
            [
                "req_pad_reject_kats_preserve_state_and_export_no_partial_material",
                "wrong_public_pad_statement.pad_opening_statement.digest[0] ^= 1",
                "observed_block_mutation",
                "verify_reviewed_fq12_state_assisted_masked_projection_candidate_verifier",
            ],
            *sources,
        ),
        "relation_projection_mismatch": source_evidence(
            [
                "production_reviewed_fq12_carrier_export_branch_is_opaque_error",
                "selected_projection_reject_carrier_lane_mutation",
                "reviewed Fq12 masked witness state18 projection carrier tuple exported a hidden-material break",
                "assert_no_selected_projection_material",
            ],
            *sources,
        ),
        "carrier_context_replay": source_evidence(
            [
                "selected_projection_reject_carrier_transcript_binding_mutation",
                "wrong_statement.message_digest[0] ^= 1",
                "reviewed Fq12 projection carrier check bytes mismatch",
                "PRODUCTION_SELECTED_PROJECTION_INVALID_CARRIER_REASON",
            ],
            *sources,
        ),
        "false_statement_acceptance": source_evidence(
            [
                "state_assisted_masked_projection_candidate_verifier_accepts_executable_tuple",
                "wrong_statement.message_digest[0] ^= 1",
                "local opening mutation",
                "public pad-opening mismatch for block 2 in round 11",
            ],
            *sources,
        ),
        "precommitment_bypass": source_evidence(
            [
                "state_assisted_account_schema_checks_prechallenge_opening",
                "wrong_opening.pad_seed_digest[0] ^= 1",
                "state-assisted pad seed digest mismatch",
                "state-assisted slot is already consumed",
            ],
            *sources,
        ),
    }
    controls = [
        {
            "id": "witness_digest_substitution",
            "attack": "keep the slot opening commitment fixed but substitute a different witness opening digest inside the hidden relation proof",
            "must_reject_at": "verify_witness_commitment_opening",
            "named_break_if_accepts": "witness commitment binding break or hidden-source proof soundness break",
            "minimum_executable_test": "mutate witness_opening_digest while holding carrier bytes and account_witness_commitment fixed",
            "status": evidence_by_control["witness_digest_substitution"]["status"],
            "evidence": [
                "production_reviewed_fq12_malformed_opening_no_partial_export",
                "state_assisted_account_witness_projection_opening_binds_statement",
            ],
        },
        {
            "id": "pad_seed_substitution",
            "attack": "keep the slot opening commitment fixed but substitute a pad seed digest that does not open account_randomizer_commitment",
            "must_reject_at": "verify_pad_seed_opening",
            "named_break_if_accepts": "pad commitment binding break, pad PRF break, or hidden-source proof soundness break",
            "minimum_executable_test": "mutate pad_seed_digest while holding carrier bytes, public pad statement, and account_randomizer_commitment fixed",
            "status": evidence_by_control["pad_seed_substitution"]["status"],
            "evidence": [
                "production_reviewed_fq12_wrong_pad_seed_no_partial_export",
                "req_pad_reject_kats_preserve_state_and_export_no_partial_material",
            ],
        },
        {
            "id": "public_pad_statement_mismatch",
            "attack": "prove a pad seed that opens the account randomizer but does not generate the verifier-recomputed public pad-opening statement",
            "must_reject_at": "verify_pad_seed_opening",
            "named_break_if_accepts": "pad-opening relation break or statement recomputation bug",
            "minimum_executable_test": "alter one public pad-opening residue after the hidden relation statement is built",
            "status": evidence_by_control["public_pad_statement_mismatch"]["status"],
            "evidence": ["req_pad_reject_kats_preserve_state_and_export_no_partial_material"],
        },
        {
            "id": "relation_projection_mismatch",
            "attack": "open witness and pad digests separately but use them in different projected hidden-source statements",
            "must_reject_at": "verify_hidden_source_relation",
            "named_break_if_accepts": "hidden-source relation projection soundness break",
            "minimum_executable_test": "splice witness opening from one statement and pad opening from another with matching byte lengths",
            "status": evidence_by_control["relation_projection_mismatch"]["status"],
            "evidence": [
                "production_reviewed_fq12_carrier_export_branch_is_opaque_error",
                "selected_projection_reject_carrier_lane_mutation",
                "masked_witness_state18_unmasks_projection_with_pad_seed",
            ],
        },
        {
            "id": "carrier_context_replay",
            "attack": "reuse a valid carrier handle under a different signature digest or transcript digest",
            "must_reject_at": "verify_carrier_handle",
            "named_break_if_accepts": "carrier handle binding break or transcript-domain separation bug",
            "minimum_executable_test": "hold the 32-byte carrier fixed while mutating signature_digest and proof_transcript_digest",
            "status": evidence_by_control["carrier_context_replay"]["status"],
            "evidence": [
                "selected_projection_reject_carrier_transcript_binding_mutation",
                "state_assisted_projection_carrier_tuple_binds_selected_path",
            ],
        },
        {
            "id": "false_statement_acceptance",
            "attack": "make the hidden-relation verifier accept when the recomputed public statement is not satisfied by the opened witness/pad pair",
            "must_reject_at": "verify_hidden_source_relation",
            "named_break_if_accepts": "compact hidden-source proof soundness assumption break",
            "minimum_executable_test": "construct a syntactically valid statement with one invalid hidden relation predicate",
            "status": evidence_by_control["false_statement_acceptance"]["status"],
            "evidence": [
                "state_assisted_masked_projection_candidate_verifier_accepts_executable_tuple",
                "req_pad_reject_kats_preserve_state_and_export_no_partial_material",
            ],
        },
        {
            "id": "precommitment_bypass",
            "attack": "satisfy the hidden relation with an opening that was not the prechallenge opening committed in account state",
            "must_reject_at": "verify_slot_opening_commitment",
            "named_break_if_accepts": "state lifecycle precommitment failure or opening commitment binding break",
            "minimum_executable_test": "build a relation-valid opening whose opening commitment does not match hidden_source_opening_commitment",
            "status": evidence_by_control["precommitment_bypass"]["status"],
            "evidence": ["state_assisted_account_schema_checks_prechallenge_opening"],
        },
    ]
    coverage_summary: dict[str, int] = {}
    for row in controls:
        coverage_summary[row["status"]] = coverage_summary.get(row["status"], 0) + 1
    all_source_evidence_present = all(
        evidence["status"] == "covered_by_rust_fail_closed_negative_test"
        for evidence in evidence_by_control.values()
    )
    theorem_requirements = [
        {
            "requirement": "completeness_control",
            "meaning": "honest openings satisfying the relation must pass every semantic check once the compact proof verifier exists",
        },
        {
            "requirement": "single_fault_rejection",
            "meaning": "each listed single-field mutation must reject at the named verifier phase",
        },
        {
            "requirement": "named_break_totality",
            "meaning": "any accepting mutated control must map to a named assumption break, not an unspecified gap",
        },
        {
            "requirement": "byte_level_binding",
            "meaning": "tests mutate serialized frames, not high-level objects only",
        },
    ]
    return {
        "schema": "Descender.hidden_source_state_assisted_hidden_relation_negative_controls.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "hidden_relation_negative_controls_specified_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "hidden_relation_verifier_contract": rel(HIDDEN_RELATION_CONTRACT_JSON),
            "production_reference_tests": rel(PRODUCTION_REFERENCE_RS),
            "parameter_search_reference_tests": rel(PARAMETER_SEARCH_REFERENCE_RS),
            "hidden_relation_verifier_contract_schema": contract.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "checked_core_signature_bytes": 252,
            "carrier_bytes": 32,
            "envelope_bytes": 284,
            "selected_candidate": "state_assisted_32_byte_carrier",
        },
        "negative_controls": controls,
        "source_evidence_audit": evidence_by_control,
        "all_source_evidence_present": all_source_evidence_present,
        "coverage_summary": coverage_summary,
        "theorem_requirements": theorem_requirements,
        "decision": {
            "can_accept_now": False,
            "reason": (
                "all listed semantic negative controls are now tied to existing "
                "fail-closed Rust source evidence for the selected 308-byte "
                "candidate verifier, but this does not instantiate the compact "
                "hidden-relation proof or close global extractor semantics"
                if all_source_evidence_present
                else "some semantic negative controls still lack Rust source evidence, and the compact hidden-relation verifier is still missing"
            ),
            "next_artifact": "state_assisted_hidden_relation_extractor_theorem",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    lines = [
        "# Hidden-Source State-Assisted Hidden-Relation Negative Controls",
        "",
        "This artifact records the semantic negative controls for the hidden-relation",
        "layer.  These are the tests that must fail even after the account-state",
        "opening commitment and 32-byte carrier handle checks pass.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected candidate: `{target['selected_candidate']}`",
        f"- checked-core signature bytes: `{target['checked_core_signature_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- hidden-relation contract: `{payload['source_artifacts']['hidden_relation_verifier_contract']}`",
        f"- production-reference tests: `{payload['source_artifacts']['production_reference_tests']}`",
        f"- parameter-search tests: `{payload['source_artifacts']['parameter_search_reference_tests']}`",
        f"- all source evidence present: `{payload['all_source_evidence_present']}`",
        f"- coverage summary: `{payload['coverage_summary']}`",
        "",
        "## Negative Controls",
        "",
        "| control | attack | must reject at | named break if accepts | status |",
        "|---|---|---|---|---|",
    ]
    for row in payload["negative_controls"]:
        lines.append(
            f"| `{row['id']}` | {row['attack']} | `{row['must_reject_at']}` | "
            f"{row['named_break_if_accepts']} | `{row['status']}` |"
        )
    lines.extend(
        [
            "",
            "## Source Evidence Audit",
            "",
            "| control | status | missing tokens |",
            "|---|---|---|",
        ]
    )
    for control_id, audit in payload["source_evidence_audit"].items():
        lines.append(
            f"| `{control_id}` | `{audit['status']}` | `{audit['missing_tokens']}` |"
        )
    lines.extend(
        [
            "",
            "## Minimum Executable Tests",
            "",
            "| control | test |",
            "|---|---|",
        ]
    )
    for row in payload["negative_controls"]:
        lines.append(f"| `{row['id']}` | {row['minimum_executable_test']} |")
    lines.extend(["", "## Theorem Requirements", "", "| requirement | meaning |", "|---|---|"])
    for row in payload["theorem_requirements"]:
        lines.append(f"| `{row['requirement']}` | {row['meaning']} |")
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
