#!/usr/bin/env python3
"""Generate the hidden-relation extractor theorem target for the state-assisted carrier."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_extractor_theorem_n32_q12289.md"
)
HIDDEN_RELATION_CONTRACT_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_verifier_contract_n32_q12289.json"
)
NEGATIVE_CONTROLS_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_hidden_relation_negative_controls_n32_q12289.json"
)
EXTRACTOR_OBLIGATIONS_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_extractor_obligations_n32_q12289.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    contract = load_json(HIDDEN_RELATION_CONTRACT_JSON)
    controls = load_json(NEGATIVE_CONTROLS_JSON)
    obligations = load_json(EXTRACTOR_OBLIGATIONS_JSON)
    source_covered_controls = [
        row["id"]
        for row in controls.get("negative_controls", [])
        if row.get("status") == "covered_by_rust_fail_closed_negative_test"
    ]
    missing_control_evidence = [
        row["id"]
        for row in controls.get("negative_controls", [])
        if row.get("status") != "covered_by_rust_fail_closed_negative_test"
    ]
    local_negative_control_evidence = {
        "status": (
            "all_semantic_controls_have_fail_closed_rust_evidence"
            if controls.get("all_source_evidence_present")
            else "semantic_controls_missing_rust_evidence"
        ),
        "covered_count": len(source_covered_controls),
        "total_count": len(controls.get("negative_controls", [])),
        "covered_controls": source_covered_controls,
        "missing_controls": missing_control_evidence,
        "non_claim": (
            "These fail-closed tests discharge only local semantic mutation "
            "rejection for the candidate verifier and soundness-certificate "
            "wrapper; they do not instantiate a compact proof system or prove "
            "global accepting-verifier extraction."
        ),
    }
    theorem_statement = {
        "name": "state_assisted_hidden_relation_extractor_or_break",
        "informal_statement": (
            "For n=32, q=12289, if the state-assisted hidden-relation verifier "
            "accepts the selected 308-byte/Fq12 projection-carrier envelope, account-state slot, "
            "state-assisted opening, hidden-source statement, and pad seed, then the executable "
            "soundness certificate returns the checked-core syndrome, response syndrome, full "
            "masked-state18 packed witness opening, pad-seed opening digest, carrier lanes, "
            "and canonical hidden-source statement satisfying the verifier-owned byte relation; "
            "global unforgeability then reduces any remaining failure to one named lifecycle, "
            "transcript, proof, pad, QROM, response-link, keygen, mask-PRF, or leakage break."
        ),
        "acceptance_scope": "candidate verifier after strict 308-byte envelope parse, checked Fq12 core, account-state opening equality, masked state18 packed-opening extraction, pad-seed opening, and projection-carrier binding",
        "non_claim": "does not yet make the selected fail-closed deployment verifier accepting",
    }
    hypotheses = [
        {
            "id": "canonical_statement_recomputed",
            "source": "hidden-relation verifier contract",
            "condition": "public statement bytes are recomputed from public key, message, signature envelope, transcript, slot state, and restore anchor",
        },
        {
            "id": "prechallenge_opening_fixed",
            "source": "account-state schema contract",
            "condition": "hidden_source_opening_commitment is stored before challenge derivation and matches the opening frame digest",
        },
        {
            "id": "packed_state18_opening_extracts",
            "source": "Rust candidate soundness certificate",
            "condition": "reviewed_fq12_masked_state18_packed_witness_opening decodes the committed 32-byte witness field, unmasks six projection residues with pad seed and slot id, and repacks to the same commitment",
        },
        {
            "id": "projection_carrier_bound",
            "source": "Rust candidate soundness certificate",
            "condition": "56-byte carrier lanes, check bytes, and transcript binding match the extracted packed opening, pad seed opening, hidden-source statement, and opening commitment",
        },
        {
            "id": "negative_controls_rejected",
            "source": "hidden-relation negative controls",
            "condition": (
                "all semantic single-fault mutations have fail-closed Rust "
                "source evidence for the selected 308-byte candidate verifier; "
                "global accepting-verifier proof semantics still require a "
                "compact proof system or named proof-handle assumption"
            ),
        },
    ]
    extractor_outputs = [
        {
            "name": "canonical_hidden_source_statement",
            "meaning": "the exact byte-level statement recomputed by the verifier",
            "public": True,
        },
        {
            "name": "account_witness_opening",
            "meaning": "full masked-state18 packed opening of account_witness_commitment to 12 checked-core residues and six unmasked selected projection residues",
            "public": False,
        },
        {
            "name": "pad_seed_opening",
            "meaning": "opening of account_randomizer_commitment to the pad source used by the public pad-opening relation",
            "public": False,
        },
        {
            "name": "projection_carrier_binding",
            "meaning": "56-byte carrier lanes, check bytes, and transcript binding tied to the extracted opening and statement",
            "public": True,
        },
        {
            "name": "slot_lifecycle_witness",
            "meaning": "unused slot, restore anchor, and consumed transition evidence for the accepted spend",
            "public": True,
        },
    ]
    named_breaks = [
        {
            "id": "sha256_or_random_oracle_binding_break",
            "covers": ["opening commitment collision", "carrier handle replay", "statement digest collision"],
            "evidence": "two distinct serialized frames with the same required digest or a successful RO-programming inconsistency",
        },
        {
            "id": "account_commitment_binding_break",
            "covers": ["witness_digest_substitution", "pad_seed_substitution"],
            "evidence": "two openings of the same account witness/randomizer commitment to different semantic values",
        },
        {
            "id": "hidden_relation_proof_soundness_break",
            "covers": ["false_statement_acceptance", "relation_projection_mismatch", "masked_state18_extraction_failure"],
            "evidence": "accepted candidate transcript without the executable soundness certificate fields satisfying their verifier-owned relations",
        },
        {
            "id": "pad_prf_or_pad_opening_break",
            "covers": ["public_pad_statement_mismatch"],
            "evidence": "pad seed opening does not derive the public pad-opening statement but verifier accepts",
        },
        {
            "id": "state_lifecycle_or_restore_anchor_break",
            "covers": ["precommitment_bypass", "slot replay", "state rollback"],
            "evidence": "accepted state transition contradicting spent-slot or restore-anchor policy",
        },
        {
            "id": "checked_core_or_response_link_break",
            "covers": ["accepted hidden relation composed with invalid checked Fq12 core"],
            "evidence": "counterexample to checked-core equation, response-link certificate, or bounded-kernel assumption",
        },
        {
            "id": "qrom_fiat_shamir_or_keygen_break",
            "covers": ["challenge programmability gap", "matrix/keygen distribution gap"],
            "evidence": "forgery not covered by extractor under the recorded QROM/query and keygen assumptions",
        },
        {
            "id": "mask_prf_or_leakage_break",
            "covers": ["projection mask distinguishability", "pad/projection mask stream collision", "unaccounted selected-projection leakage"],
            "evidence": "adversary distinguishes or correlates the masked projection residues beyond the theorem-accounted mask-PRF and leakage model",
        },
    ]
    proof_skeleton = [
        {
            "step": "normalize_acceptance",
            "claim": "accepted verifier transcript determines one canonical serialized statement and one slot opening commitment",
            "depends_on": ["canonical_statement_recomputed", "prechallenge_opening_fixed"],
        },
        {
            "step": "bind_carrier_to_opening",
            "claim": "accepted 56-byte projection carrier authenticates that statement/opening pair unless SHA-256/RO binding fails",
            "depends_on": ["projection_carrier_bound"],
        },
        {
            "step": "extract_hidden_relation_openings",
            "claim": "accepted candidate verifier yields the masked-state18 packed witness opening and pad opening or rejects before acceptance",
            "depends_on": ["packed_state18_opening_extracts", "projection_carrier_bound"],
        },
        {
            "step": "compose_lifecycle",
            "claim": "extracted opening was fixed before the challenge and consumed once unless state lifecycle breaks",
            "depends_on": ["prechallenge_opening_fixed"],
        },
        {
            "step": "close_negative_controls",
            "claim": "each semantic mutation maps to a named break and no accepting mutation falls through an unspecified gap",
            "depends_on": ["negative_controls_rejected"],
        },
        {
            "step": "compose_global_theorem",
            "claim": "successful forgery implies extracted witness/pad source or one named break from this theorem plus checked-core assumptions",
            "depends_on": ["all previous steps"],
        },
    ]
    open_obligations = [
        {
            "id": "promote_executable_candidate_soundness_to_global_unforgeability",
            "why_open": "the Rust certificate proves local extraction from candidate acceptance, but the global game hop from arbitrary forgery to selected-relation acceptance is not yet formalized",
            "required_to_close": "one theorem composing candidate acceptance, checked-core soundness, state lifecycle, QROM, keygen, and public-key/account registration assumptions",
        },
        {
            "id": "prove_mask_prf_and_leakage_independence",
            "why_open": "the masked projection residues have executable disjointness/statistical probes, but no PRF/RO theorem yet",
            "required_to_close": "mask-PRF domain-separation theorem and leakage simulator/budget composed with pad PRF and lifecycle uniqueness",
        },
        {
            "id": "compose_with_checked_core_and_qrom",
            "why_open": "the hidden-relation extractor must be composed with existing checked-core, response-link, pad, QROM, and keygen assumptions",
            "required_to_close": "global accepting-verifier theorem artifact",
        },
    ]
    return {
        "schema": "Descender.hidden_source_state_assisted_hidden_relation_extractor_theorem.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "hidden_relation_extractor_theorem_target_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "hidden_relation_verifier_contract": rel(HIDDEN_RELATION_CONTRACT_JSON),
            "hidden_relation_negative_controls": rel(NEGATIVE_CONTROLS_JSON),
            "state_assisted_extractor_obligations": rel(EXTRACTOR_OBLIGATIONS_JSON),
            "hidden_relation_verifier_contract_schema": contract.get("schema"),
            "hidden_relation_negative_controls_schema": controls.get("schema"),
            "state_assisted_extractor_obligations_schema": obligations.get("schema"),
        },
        "target": {
            "n": 32,
            "q": 12289,
            "checked_core_signature_bytes": 252,
            "carrier_bytes": 56,
            "envelope_bytes": 308,
            "selected_candidate": "state_assisted_masked_projection_carrier",
        },
        "theorem_statement": theorem_statement,
        "hypotheses": hypotheses,
        "extractor_outputs": extractor_outputs,
        "named_breaks": named_breaks,
        "local_negative_control_evidence": local_negative_control_evidence,
        "proof_skeleton": proof_skeleton,
        "open_obligations": open_obligations,
        "decision": {
            "can_accept_now": False,
            "reason": (
                "the executable candidate extractor is implemented and the "
                "semantic negative controls now have local fail-closed Rust "
                "evidence, but compact proof instantiation, global "
                "unforgeability, mask-PRF/leakage, QROM, checked-core, and "
                "lifecycle composition remain open"
            ),
            "next_artifact": "state_assisted_masked_projection_global_soundness_composition",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    target = payload["target"]
    theorem = payload["theorem_statement"]
    lines = [
        "# Hidden-Source State-Assisted Hidden-Relation Extractor Theorem",
        "",
        "This artifact states the extractor-or-break theorem target for the",
        "state-assisted hidden-relation layer.  It is still fail-closed: it states",
        "the theorem and names the breaks, but does not discharge the compact proof",
        "instantiation or global composition obligations.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- selected candidate: `{target['selected_candidate']}`",
        f"- checked-core signature bytes: `{target['checked_core_signature_bytes']}`",
        f"- carrier bytes: `{target['carrier_bytes']}`",
        f"- envelope bytes: `{target['envelope_bytes']}`",
        f"- theorem: `{theorem['name']}`",
        f"- non-claim: {theorem['non_claim']}",
        "",
        "## Informal Statement",
        "",
        theorem["informal_statement"],
        "",
        "## Hypotheses",
        "",
        "| id | source | condition |",
        "|---|---|---|",
    ]
    for row in payload["hypotheses"]:
        lines.append(f"| `{row['id']}` | {row['source']} | {row['condition']} |")
    lines.extend(["", "## Extractor Outputs", "", "| output | meaning | public |", "|---|---|:---:|"])
    for row in payload["extractor_outputs"]:
        lines.append(f"| `{row['name']}` | {row['meaning']} | `{row['public']}` |")
    lines.extend(["", "## Named Breaks", "", "| break | covers | evidence |", "|---|---|---|"])
    for row in payload["named_breaks"]:
        lines.append(f"| `{row['id']}` | `{row['covers']}` | {row['evidence']} |")
    local = payload["local_negative_control_evidence"]
    lines.extend(
        [
            "",
            "## Local Negative-Control Evidence",
            "",
            f"- status: `{local['status']}`",
            f"- covered controls: `{local['covered_count']}` / `{local['total_count']}`",
            f"- missing controls: `{local['missing_controls']}`",
            f"- non-claim: {local['non_claim']}",
        ]
    )
    lines.extend(["", "## Proof Skeleton", "", "| step | claim | depends on |", "|---|---|---|"])
    for row in payload["proof_skeleton"]:
        lines.append(f"| `{row['step']}` | {row['claim']} | `{row['depends_on']}` |")
    lines.extend(["", "## Open Obligations", "", "| obligation | why open | required to close |", "|---|---|---|"])
    for row in payload["open_obligations"]:
        lines.append(
            f"| `{row['id']}` | {row['why_open']} | {row['required_to_close']} |"
        )
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
