#!/usr/bin/env python3
"""Build the Path 2 descent/SIS proof-obligation matrix."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
FEASIBILITY_JSON = ROOT / "research/path2_descent_sis_feasibility.json"
OUT_JSON = ROOT / "research/path2_descent_sis_obligation_matrix.json"
OUT_MD = ROOT / "research/path2_descent_sis_obligation_matrix.md"
TYPE_MAP_LEAN = ROOT / "Descender/Path2DescentSIS.lean"
MATRIX_DOMAINS_DOC = ROOT / "research/path2_descent_sis_matrix_domains.md"
BINDING_SURFACE_DOC = ROOT / "research/path2_descent_sis_binding_surface.md"
NORM_RANGE_SURFACE_DOC = ROOT / "research/path2_descent_sis_norm_range_surface.md"
RANK_HIDING_PROBE_DOC = ROOT / "research/path2_descent_sis_rank_hiding_probe.md"
HIDING_REDESIGN_OPTIONS_DOC = (
    ROOT / "research/path2_descent_sis_hiding_redesign_options.md"
)
PRIVATE_ZK_PAYLOAD_SURFACE_DOC = (
    ROOT / "research/path2_descent_sis_private_zk_payload_surface.md"
)
PRIVATE_ZK_QROM_FRAME_DOC = (
    ROOT / "research/path2_descent_sis_private_zk_qrom_frame.md"
)
PRIVATE_ZK_NO_EXPORT_CONTRACT_DOC = (
    ROOT / "research/path2_descent_sis_private_zk_no_export_contract.md"
)
PRIVATE_ZK_PROOF_FAMILY_GATE_DOC = (
    ROOT / "research/path2_descent_sis_private_zk_proof_family_gate.md"
)
LATTICE_SIGMA_TOY_MODEL_DOC = (
    ROOT / "research/path2_descent_sis_lattice_sigma_toy_model.md"
)
PRIVATE_ZK_VS_TRANSPARENT_GATE_DOC = (
    ROOT / "research/path2_private_zk_vs_transparent_gate.md"
)
TRANSPARENT_PROOF_BYTE_MODEL_DOC = (
    ROOT / "research/path2_transparent_proof_byte_model.md"
)
PAYLOAD_PROOF_CANDIDATE_DECISION_DOC = (
    ROOT / "research/path2_payload_proof_candidate_decision.md"
)
TRANSPARENT_PAYLOAD_CONTRACT_DOC = (
    ROOT / "research/path2_transparent_payload_contract.md"
)
TRANSPARENT_EXTRACTOR_SOUNDNESS_BRIDGE_DOC = (
    ROOT / "research/path2_transparent_extractor_soundness_bridge.md"
)
PRODUCTION_REFERENCE_RS = ROOT / "tools/parameter-search/src/production_reference.rs"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def row(
    obligation_id: str,
    theorem_role: str,
    current_local_evidence: str,
    missing_work: str,
    closure_gate: str,
    named_break_if_false: str,
    applies_to: list[str],
    status: str = "open",
) -> dict[str, Any]:
    return {
        "obligation_id": obligation_id,
        "theorem_role": theorem_role,
        "current_local_evidence": current_local_evidence,
        "missing_work": missing_work,
        "closure_gate": closure_gate,
        "named_break_if_false": named_break_if_false,
        "applies_to": applies_to,
        "status": status,
    }


def build_model() -> dict[str, Any]:
    feasibility = load_json(FEASIBILITY_JSON)
    primary = [
        item
        for item in feasibility["candidate_rows"]
        if item["status"] == "primary_feasibility_band"
    ]
    primary_ids = [item["id"] for item in primary]
    row_summary = {
        item["id"]: {
            "payload_bytes_lower_bound": item["payload_bytes_lower_bound"],
            "verifier_mul_adds_for_commitment_checks": item[
                "verifier_mul_adds_for_commitment_checks"
            ],
            "commitment_bytes_total": item["commitment_bytes_total"],
            "sis_rows_per_commitment": item["sis_rows_per_commitment"],
        }
        for item in primary
    }
    obligations = [
        row(
            "p2_sis_public_quotient_types",
            "instantiate LocalQuotientDescent public quotient, representative, hidden, and evidence types",
            "Descender.Path2DescentSIS builds and names Public, Representative, MaskedProjection, HiddenOpening, AccountEvidence, BreakEvent, Predicates, localQuotientDescent, and accepts_glues_or_breaks",
            "connect the concrete Rust payload byte verifier to these abstract fields after matrix/proof domains are selected",
            "Lean sidecar `Descender/Path2DescentSIS.lean` plus prose type-map artifact",
            "qrom_or_domain_separation_break",
            primary_ids,
            "interface_closed",
        ),
        row(
            "p2_sis_matrix_domains",
            "derive role-separated SIS matrices for pad, projection, opening, and transcript commitments",
            "Rust constants and KAT define role-separated matrix domains, 48/64 row bands, column counts, rejection sampling, transcript digests, and deterministic regeneration",
            "connect the concrete matrices to SIS binding/hiding analysis and future payload verifier",
            "`path2_descent_sis_matrix_domains.md` plus KAT `path2_descent_sis_matrix_domains_are_role_and_row_band_separated`",
            "qrom_or_domain_separation_break",
            primary_ids,
            "executable_closed",
        ),
        row(
            "p2_sis_binding",
            "two distinct short openings to the same commitment imply SIS/module-SIS binding break",
            "Path 2-specific binding game is pinned in `path2_descent_sis_binding_surface.md`: a double opening exports delta = v0 - v1 with A_role,rowband * delta = 0 mod q for exact role domains, columns, and 48/64 row bands",
            "define norm/range bounds, then run SIS/module-SIS hardness/sanity model for the exact matrix dimensions and bounds",
            "binding theorem or carried SIS/module-SIS binding assumption with exact role, row-band, and norm parameters",
            "path2_sis_binding_break",
            primary_ids,
            "assumption_surface_pinned",
        ),
        row(
            "p2_sis_hiding_sampler",
            "commitment/proof transcript hides pad lanes, projection lanes, and opening tuple scalars",
            "transparent payload contract pins the selected outer proof frame, extractor/soundness surface, and zero-knowledge/no-export surface over the Path 2 descent relation",
            "instantiate transparent proof verifier/serializer KATs and quantify concrete simulator/no-export assumptions",
            "simulator or statistical/computational hiding theorem for leaf + proof binding + public commitments + proof transcript",
            "samplerHidingBreak",
            primary_ids,
            "contract_pinned",
        ),
        row(
            "p2_sis_non_public_link_equation",
            "linear relation proof is not a public projection shortcut",
            "transparent payload contract pins the rule: masked_projection is public, but pad lanes and account-projection lanes appear only inside the hidden trace/relation and not as serialized public fields",
            "implement typed transparent payload serializer/reject tests and prove/simulate that query openings do not expose link values outside the proof's ZK policy",
            "negative control rejects public-projection shortcut payloads; theorem names publicProjectionBreak",
            "publicProjectionBreak",
            primary_ids,
            "contract_pinned",
        ),
        row(
            "p2_sis_norm_range",
            "openings are short/canonical and correspond to intended q12289 semantic lanes",
            "norm/range blocker is pinned in `path2_descent_sis_norm_range_surface.md`: raw q12289 residue lanes are canonical but not automatically short enough for a clean SIS binding claim; base-128 digit openings are the primary next probe",
            "choose final per-role representation and bounds, update feasibility byte model if digit openings are promoted, and add rejection behavior plus modular-wrap controls",
            "negative controls for non-short modular witnesses and malformed range proofs, plus hardness sanity check under the selected bounds",
            "path2_sis_norm_or_range_break",
            primary_ids,
            "blocker_surface_pinned",
        ),
        row(
            "p2_sis_fiat_shamir_qrom",
            "Fiat-Shamir challenge binds leaf, proof binding, commitments, transcript, and relation statement",
            "transparent payload contract pins domain, proof_system_id, relation_id, trace_shape, leaf/proof/matrix context, masked_projection, aggregation context, trace roots, and query index; Rust constants, relation-id helper, transcript digest helper, context-binding KAT, and active query-budget symbols exist",
            "extend KATs once the concrete transparent proof verifier exists and quantify final proof-system query counts",
            "QROM ledger row with payload-specific query symbols and domain KATs",
            "qrom_or_domain_separation_break",
            primary_ids,
            "contract_pinned",
        ),
        row(
            "p2_sis_extractor",
            "accepting payload yields valid hidden opening or named break",
            "transparent extractor/soundness bridge is pinned in `path2_transparent_extractor_soundness_bridge.md`; Rust now has a fail-closed transparent descent payload boundary verifier that binds proof kind, proof-system id, public view, leaf context, creation proof binding, payload digest, and query-opening digests before rejecting because the proof verifier is not implemented",
            "implement concrete transparent proof verifier and trace/relation encoding, instantiate proof-system-specific soundness parameters, and replace the fail-closed boundary with accepting honest-proof coverage plus malformed-proof negative controls",
            "direct_replayable_relation_certificate_soundness_or_break consumes the bridge at replay_relation_checks, after public-view binding and before CreationHiddenSourceRelationV1 replay",
            "creation_time_hidden_source_soundness_break",
            primary_ids,
            "bridge_pinned",
        ),
        row(
            "p2_sis_no_export",
            "reject and serialization paths do not expose private material",
            "transparent payload contract pins a public serializer allowlist and private denylist; Rust public-view serializer and no-export KAT exist for the selected transparent proof metadata",
            "extend no-export KATs to concrete query openings once the transparent proof verifier is instantiated",
            "KATs prove no raw pad lanes, projection lanes, masks, or private opening frames appear in public outputs",
            "live_slot_leakage_or_keygen_break",
            primary_ids,
            "contract_pinned",
        ),
    ]
    return {
        "schema": "Descender.path2_descent_sis_obligation_matrix.v1",
        "status": "open_obligations_pinned",
        "primary_candidate_rows": row_summary,
        "deployment_ready": False,
        "closed_obligation_count": sum(
            1
            for item in obligations
            if item["status"] in {"interface_closed", "executable_closed"}
        ),
        "assumption_surface_pinned_count": sum(
            1 for item in obligations if item["status"] == "assumption_surface_pinned"
        ),
        "blocker_surface_pinned_count": sum(
            1 for item in obligations if item["status"] == "blocker_surface_pinned"
        ),
        "critical_blocker_count": sum(
            1 for item in obligations if item["status"] == "critical_blocker_identified"
        ),
        "route_surface_pinned_count": sum(
            1 for item in obligations if item["status"] == "route_surface_pinned"
        ),
        "bridge_pinned_count": sum(
            1 for item in obligations if item["status"] == "bridge_pinned"
        ),
        "contract_pinned_count": sum(
            1 for item in obligations if item["status"] == "contract_pinned"
        ),
        "open_obligation_count": sum(1 for item in obligations if item["status"] == "open"),
        "type_map_lean_artifact": str(TYPE_MAP_LEAN.relative_to(ROOT)),
        "matrix_domain_artifacts": [
            str(MATRIX_DOMAINS_DOC.relative_to(ROOT)),
            str(PRODUCTION_REFERENCE_RS.relative_to(ROOT)),
        ],
        "binding_surface_artifact": str(BINDING_SURFACE_DOC.relative_to(ROOT)),
        "norm_range_surface_artifact": str(NORM_RANGE_SURFACE_DOC.relative_to(ROOT)),
        "rank_hiding_probe_artifact": str(RANK_HIDING_PROBE_DOC.relative_to(ROOT)),
        "hiding_redesign_options_artifact": str(
            HIDING_REDESIGN_OPTIONS_DOC.relative_to(ROOT)
        ),
        "private_zk_payload_surface_artifact": str(
            PRIVATE_ZK_PAYLOAD_SURFACE_DOC.relative_to(ROOT)
        ),
        "private_zk_qrom_frame_artifact": str(
            PRIVATE_ZK_QROM_FRAME_DOC.relative_to(ROOT)
        ),
        "private_zk_no_export_contract_artifact": str(
            PRIVATE_ZK_NO_EXPORT_CONTRACT_DOC.relative_to(ROOT)
        ),
        "private_zk_proof_family_gate_artifact": str(
            PRIVATE_ZK_PROOF_FAMILY_GATE_DOC.relative_to(ROOT)
        ),
        "lattice_sigma_toy_model_artifact": str(
            LATTICE_SIGMA_TOY_MODEL_DOC.relative_to(ROOT)
        ),
        "private_zk_vs_transparent_gate_artifact": str(
            PRIVATE_ZK_VS_TRANSPARENT_GATE_DOC.relative_to(ROOT)
        ),
        "transparent_proof_byte_model_artifact": str(
            TRANSPARENT_PROOF_BYTE_MODEL_DOC.relative_to(ROOT)
        ),
        "payload_proof_candidate_decision_artifact": str(
            PAYLOAD_PROOF_CANDIDATE_DECISION_DOC.relative_to(ROOT)
        ),
        "transparent_payload_contract_artifact": str(
            TRANSPARENT_PAYLOAD_CONTRACT_DOC.relative_to(ROOT)
        ),
        "transparent_extractor_soundness_bridge_artifact": str(
            TRANSPARENT_EXTRACTOR_SOUNDNESS_BRIDGE_DOC.relative_to(ROOT)
        ),
        "theorem_closure_rule": (
            "account_creation_prevalidated_slot_authorization_or_break cannot be "
            "marked closed until every obligation is either proved by executable/"
            "formal evidence or listed as a named carried assumption in the proof "
            "boundary ledger"
        ),
        "obligations": obligations,
    }


def write_markdown(model: dict[str, Any]) -> str:
    lines = [
        "# Path 2 Descent/SIS Obligation Matrix",
        "",
        "Status: open obligations pinned.",
        "",
        "## Closure Rule",
        "",
        model["theorem_closure_rule"],
        "",
        "## Primary Candidate Rows",
        "",
        "| candidate | rows | commitment bytes | payload lower bound | verifier mul-adds |",
        "|---|---:|---:|---:|---:|",
    ]
    for candidate_id, item in model["primary_candidate_rows"].items():
        lines.append(
            f"| `{candidate_id}` | {item['sis_rows_per_commitment']} | "
            f"{item['commitment_bytes_total']} | {item['payload_bytes_lower_bound']} | "
            f"{item['verifier_mul_adds_for_commitment_checks']} |"
        )
    lines.extend(
        [
            "",
            "## Obligations",
            "",
            "| id | theorem role | current evidence | missing work | closure gate | break if false |",
            "|---|---|---|---|---|---|",
        ]
    )
    for item in model["obligations"]:
        lines.append(
            "| `{obligation_id}` ({status}) | {theorem_role} | {current_local_evidence} | "
            "{missing_work} | {closure_gate} | `{named_break_if_false}` |".format(**item)
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "The 48-row and 64-row bands remain plausible only if all rows above can be",
            "closed without adding proof bytes beyond the transparent-proof fallback.",
            "The matrix deliberately blocks theorem closure by default: every row starts",
            "open and must become either proved evidence or an explicit carried",
            "assumption with a named break.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    model = build_model()
    OUT_JSON.write_text(json.dumps(model, indent=2, sort_keys=True) + "\n")
    OUT_MD.write_text(write_markdown(model))
    print(json.dumps({"json": str(OUT_JSON), "markdown": str(OUT_MD)}, indent=2))


if __name__ == "__main__":
    main()
