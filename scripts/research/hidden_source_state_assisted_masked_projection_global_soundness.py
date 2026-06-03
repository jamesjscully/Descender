#!/usr/bin/env python3
"""Generate the selected-path global soundness composition target."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_masked_projection_global_soundness_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_state_assisted_masked_projection_global_soundness_n32_q12289.md"
)

EXTRACTOR_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.json"
)
HIDDEN_TARGET_JSON = READINESS_DIR / "hidden_source_proof_semantics_target_n32_q12289.json"
PREFLIGHT_JSON = (
    READINESS_DIR / "production_preflight_certificate_verification_n32_q12289.json"
)
MASK_PRF_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_assumption_api_n32_q12289.json"
)
MASK_PRF_SIMULATOR_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_leakage_simulator_n32_q12289.json"
)
MASK_PRF_RO_REDUCTION_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_ro_reduction_n32_q12289.json"
)
MASK_PRF_LIFECYCLE_SAME_GAME_JSON = (
    READINESS_DIR / "pre_signature_projection_lifecycle_same_game_lift_n32_q12289.json"
)
MASK_PRF_SPEND_BUDGET_JSON = (
    READINESS_DIR / "pre_signature_projection_spend_time_leakage_budget_n32_q12289.json"
)
MASK_PRF_REJECT_EQUIVALENCE_JSON = (
    READINESS_DIR / "pre_signature_projection_reject_equivalence_n32_q12289.json"
)
MASK_PRF_LEAKAGE_COMPOSITION_JSON = (
    READINESS_DIR
    / "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.json"
)
MASK_PRF_SHA256_RESOLUTION_JSON = (
    READINESS_DIR
    / "pre_signature_projection_sha256_mask_prf_assumption_resolution_n32_q12289.json"
)
MASK_PRF_GLOBAL_DOMAIN_REGISTRY_JSON = (
    READINESS_DIR / "pre_signature_projection_global_domain_registry_n32_q12289.json"
)
MASK_PRF_LIFECYCLE_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_lifecycle_same_game_discharge_n32_q12289.json"
)
MASK_PRF_SPEND_BUDGET_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_spend_time_budget_discharge_n32_q12289.json"
)
MASK_PRF_REJECT_EQUIVALENCE_DISCHARGE_JSON = (
    READINESS_DIR / "pre_signature_projection_reject_equivalence_discharge_n32_q12289.json"
)
QROM_JSON = READINESS_DIR / "qrom_security_loss_review_n32_q12289.json"
PAD_JSON = READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.json"
KEYGEN_JSON = READINESS_DIR / "keygen_security_assumption_surface_n32_q12289.json"
RESPONSE_LINK_JSON = (
    READINESS_DIR / "response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json"
)
LIFECYCLE_JSON = READINESS_DIR / "state_lifecycle_closure_n32_q12289.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    extractor = load_json(EXTRACTOR_JSON)
    hidden_target = load_json(HIDDEN_TARGET_JSON)
    preflight = load_json(PREFLIGHT_JSON)
    mask_prf = load_json(MASK_PRF_JSON)
    mask_prf_simulator = load_json(MASK_PRF_SIMULATOR_JSON)
    mask_prf_ro_reduction = load_json(MASK_PRF_RO_REDUCTION_JSON)
    mask_prf_lifecycle_same_game = load_json(MASK_PRF_LIFECYCLE_SAME_GAME_JSON)
    mask_prf_spend_budget = load_json(MASK_PRF_SPEND_BUDGET_JSON)
    mask_prf_reject_equivalence = load_json(MASK_PRF_REJECT_EQUIVALENCE_JSON)
    mask_prf_leakage_composition = load_json(MASK_PRF_LEAKAGE_COMPOSITION_JSON)
    mask_prf_sha256_resolution = load_json(MASK_PRF_SHA256_RESOLUTION_JSON)
    mask_prf_global_domain_registry = load_json(MASK_PRF_GLOBAL_DOMAIN_REGISTRY_JSON)
    mask_prf_lifecycle_discharge = load_json(MASK_PRF_LIFECYCLE_DISCHARGE_JSON)
    mask_prf_spend_budget_discharge = load_json(MASK_PRF_SPEND_BUDGET_DISCHARGE_JSON)
    mask_prf_reject_equivalence_discharge = load_json(
        MASK_PRF_REJECT_EQUIVALENCE_DISCHARGE_JSON
    )
    qrom = load_json(QROM_JSON)
    pad = load_json(PAD_JSON)
    keygen = load_json(KEYGEN_JSON)
    response_link = load_json(RESPONSE_LINK_JSON)
    lifecycle = load_json(LIFECYCLE_JSON)

    local_certificate = {
        "rust_symbol": "verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate",
        "input_scope": [
            "public_key",
            "message",
            "state_assisted_account",
            "308_byte_projection_carrier_signature",
            "state_assisted_opening",
            "canonical_hidden_source_statement",
            "pad_seed",
        ],
        "output_fields": [
            "slot_id",
            "statement_digest",
            "opening_commitment",
            "checked_core_challenge",
            "checked_core_witness_syndrome",
            "checked_core_response_syndrome",
            "masked_state18_packed_witness_opening",
            "account_witness_projection_opening_frame",
            "pad_seed_digest",
            "carrier_lanes",
            "carrier_check_bytes",
            "carrier_transcript_binding",
        ],
        "status": "implemented_local_certificate",
    }

    game_hops = [
        {
            "id": "strict_parse_to_candidate_acceptance",
            "status": "implemented_local",
            "claim": "A syntactically valid selected 308-byte/Fq12 signature is decoded as a checked 252-byte Fq12 core plus a 56-byte projection carrier before candidate verification.",
            "if_fails": "implementation_boundary_break",
        },
        {
            "id": "candidate_acceptance_to_local_certificate",
            "status": "implemented_local",
            "claim": "Candidate acceptance deterministically yields the local soundness certificate fields listed in the Rust API.",
            "if_fails": "candidate_certificate_extraction_break",
        },
        {
            "id": "local_certificate_to_checked_core_soundness",
            "status": "open_global",
            "claim": "The checked-core challenge, witness syndrome, and response syndrome compose with response-link and bounded-kernel assumptions.",
            "if_fails": "checked_core_or_response_link_break",
        },
        {
            "id": "local_certificate_to_masked_projection_hiding",
            "status": "open_global",
            "claim": "The six masked projection residues are simulatable or indistinguishable under the projection-mask PRF assumption and lifecycle-unique pad seeds.",
            "if_fails": "mask_prf_or_leakage_break",
        },
        {
            "id": "state_assisted_lifecycle_composition",
            "status": "open_global",
            "claim": "The precommitted opening, pad-seed digest uniqueness, restore anchor, and spent-slot ledger compose into one single-use stateful security game.",
            "if_fails": "state_lifecycle_or_restore_anchor_break",
        },
        {
            "id": "qrom_and_keygen_composition",
            "status": "open_global",
            "claim": "Fiat-Shamir/QROM loss, pad source assumptions, and keygen assumptions are counted once across checked core and hidden-source carrier layers.",
            "if_fails": "qrom_fiat_shamir_or_keygen_break",
        },
        {
            "id": "constant_time_implementation_boundary",
            "status": "open_global",
            "claim": "Production parsing, rejection, and verifier control flow leak only public information.",
            "if_fails": "constant_time_uniform_reject_break",
        },
    ]

    named_breaks = [
        "implementation_boundary_break",
        "candidate_certificate_extraction_break",
        "checked_core_or_response_link_break",
        "mask_prf_or_leakage_break",
        "state_lifecycle_or_restore_anchor_break",
        "qrom_fiat_shamir_or_keygen_break",
        "constant_time_uniform_reject_break",
    ]

    proof_obligations = [
        {
            "id": "checked_core_response_link_composition",
            "status": "open",
            "evidence": [rel(RESPONSE_LINK_JSON), rel(QROM_JSON)],
            "closure": "prove the certificate's checked-core syndrome/response fields satisfy the existing response-link theorem under the same Fiat-Shamir challenge frame",
        },
        {
            "id": "mask_prf_leakage_simulator",
            "status": mask_prf_leakage_composition.get("status"),
            "evidence": rel(MASK_PRF_LEAKAGE_COMPOSITION_JSON),
            "closure": "discharge or explicitly assume the composed mask-PRF leakage theorem's carried SHA-256/RO, lifecycle, spend-opening, and reject-equivalence terms",
        },
        {
            "id": "state_lifecycle_single_use_game",
            "status": lifecycle.get("closure_status"),
            "evidence": rel(LIFECYCLE_JSON),
            "closure": "lift lifecycle policy artifacts into the same game as the candidate certificate and target-chain replay model",
        },
        {
            "id": "pad_keygen_qrom_budget",
            "status": "open",
            "evidence": [rel(PAD_JSON), rel(KEYGEN_JSON), rel(QROM_JSON)],
            "closure": "compose pad-source, keygen, and QROM losses without double-counting oracle queries or hidden inputs",
        },
        {
            "id": "constant_time_uniform_reject",
            "status": "open",
            "evidence": "constant-time audit artifacts",
            "closure": "prove or measure that all secret-dependent failure classes are eliminated or public-only",
        },
    ]

    return {
        "schema": "Descender.hidden_source_state_assisted_masked_projection_global_soundness.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "selected_masked_projection_global_soundness_fail_closed",
        "deployment_ready": False,
        "target": {
            "n": 32,
            "q": 12289,
            "checked_core_signature_bytes": 252,
            "projection_carrier_bytes": 56,
            "selected_envelope_bytes": 308,
            "selected_candidate": "state_assisted_masked_projection_carrier",
        },
        "source_artifacts": {
            "extractor_theorem": rel(EXTRACTOR_JSON),
            "extractor_theorem_schema": extractor.get("schema"),
            "hidden_source_target": rel(HIDDEN_TARGET_JSON),
            "hidden_source_target_schema": hidden_target.get("schema"),
            "preflight_certificate_verification": rel(PREFLIGHT_JSON),
            "preflight_certificate_status": preflight.get("status"),
            "mask_prf_assumption_api": rel(MASK_PRF_JSON),
            "mask_prf_schema": mask_prf.get("schema"),
            "mask_prf_leakage_simulator": rel(MASK_PRF_SIMULATOR_JSON),
            "mask_prf_leakage_simulator_schema": mask_prf_simulator.get("schema"),
            "mask_prf_ro_reduction": rel(MASK_PRF_RO_REDUCTION_JSON),
            "mask_prf_ro_reduction_schema": mask_prf_ro_reduction.get("schema"),
            "mask_prf_lifecycle_same_game": rel(MASK_PRF_LIFECYCLE_SAME_GAME_JSON),
            "mask_prf_lifecycle_same_game_schema": mask_prf_lifecycle_same_game.get("schema"),
            "mask_prf_spend_time_leakage_budget": rel(MASK_PRF_SPEND_BUDGET_JSON),
            "mask_prf_spend_time_leakage_budget_schema": mask_prf_spend_budget.get("schema"),
            "mask_prf_reject_equivalence": rel(MASK_PRF_REJECT_EQUIVALENCE_JSON),
            "mask_prf_reject_equivalence_schema": mask_prf_reject_equivalence.get("schema"),
            "mask_prf_leakage_theorem_composition": rel(MASK_PRF_LEAKAGE_COMPOSITION_JSON),
            "mask_prf_leakage_theorem_composition_schema": mask_prf_leakage_composition.get("schema"),
            "mask_prf_sha256_resolution_schema": mask_prf_sha256_resolution.get("schema"),
            "mask_prf_global_domain_registry_schema": mask_prf_global_domain_registry.get("schema"),
            "mask_prf_lifecycle_discharge_schema": mask_prf_lifecycle_discharge.get("schema"),
            "mask_prf_spend_budget_discharge_schema": mask_prf_spend_budget_discharge.get("schema"),
            "mask_prf_reject_equivalence_discharge_schema": mask_prf_reject_equivalence_discharge.get("schema"),
            "qrom_review_schema": qrom.get("schema"),
            "pad_review_schema": pad.get("schema"),
            "keygen_surface_schema": keygen.get("schema"),
            "response_link_schema": response_link.get("schema"),
            "lifecycle_schema": lifecycle.get("schema"),
        },
        "theorem": {
            "name": "state_assisted_masked_projection_global_soundness_or_break",
            "informal_statement": (
                "Any accepting transcript for the selected 308-byte/Fq12 state-assisted "
                "masked projection-carrier signature either yields the executable local "
                "soundness certificate and composes with checked-core, QROM, lifecycle, "
                "keygen, pad-source, and mask-PRF assumptions, or exports one named break."
            ),
            "non_claim": "This artifact does not remove the deployment preflight blocker; it defines the remaining global composition target.",
        },
        "local_certificate": local_certificate,
        "game_hops": game_hops,
        "proof_obligations": proof_obligations,
        "named_breaks": named_breaks,
        "decision": {
            "can_accept_now": False,
            "implemented_local_hops": sum(1 for hop in game_hops if hop["status"] == "implemented_local"),
            "open_global_hops": sum(1 for hop in game_hops if hop["status"] == "open_global"),
            "next_artifact": mask_prf_leakage_composition.get("decision", {}).get(
                "next_artifact", "discharge_or_assume_mask_prf_leakage_carried_terms"
            ),
            "reason": "local candidate extraction is executable, but global composition and leakage/QROM/lifecycle assumptions remain open",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    theorem = payload["theorem"]
    decision = payload["decision"]
    lines = [
        "# Hidden-Source Masked Projection Global Soundness",
        "",
        "This artifact is the selected 308-byte/Fq12 global soundness composition target.",
        "It starts from the executable local candidate certificate and names the remaining",
        "game hops or break events needed before deployment preflight can accept.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected candidate: `{target['selected_candidate']}`",
        f"- selected envelope bytes: `{target['selected_envelope_bytes']}`",
        f"- projection carrier bytes: `{target['projection_carrier_bytes']}`",
        f"- implemented local hops: `{decision['implemented_local_hops']}`",
        f"- open global hops: `{decision['open_global_hops']}`",
        "",
        "## Theorem",
        "",
        f"- name: `{theorem['name']}`",
        f"- statement: {theorem['informal_statement']}",
        f"- non-claim: {theorem['non_claim']}",
        "",
        "## Local Certificate",
        "",
        f"- Rust symbol: `{payload['local_certificate']['rust_symbol']}`",
        f"- status: `{payload['local_certificate']['status']}`",
        f"- input scope: `{payload['local_certificate']['input_scope']}`",
        f"- output fields: `{payload['local_certificate']['output_fields']}`",
        "",
        "## Game Hops",
        "",
        "| hop | status | claim | break if false |",
        "|---|---|---|---|",
    ]
    for hop in payload["game_hops"]:
        lines.append(
            f"| `{hop['id']}` | `{hop['status']}` | {hop['claim']} | `{hop['if_fails']}` |"
        )
    lines.extend(["", "## Proof Obligations", "", "| obligation | status | closure | evidence |", "|---|---|---|---|"])
    for item in payload["proof_obligations"]:
        lines.append(
            f"| `{item['id']}` | `{item['status']}` | {item['closure']} | `{item['evidence']}` |"
        )
    lines.extend(["", "## Named Breaks", ""])
    for name in payload["named_breaks"]:
        lines.append(f"- `{name}`")
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{decision['can_accept_now']}`",
            f"- next artifact: `{decision['next_artifact']}`",
            f"- reason: {decision['reason']}",
            "",
            "## Artifacts",
            "",
            f"- json: `{payload['artifacts']['json']}`",
            f"- markdown: `{payload['artifacts']['markdown']}`",
        ]
    )
    return "\n".join(lines) + "\n"


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
