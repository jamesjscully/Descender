#!/usr/bin/env python3
"""Generate the projection-mask SHA-256 PRF/RO reduction contract."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "pre_signature_projection_mask_prf_ro_reduction_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_signature_projection_mask_prf_ro_reduction_n32_q12289.md"
ASSUMPTION_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_assumption_api_n32_q12289.json"
)
SIMULATOR_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_prf_leakage_simulator_n32_q12289.json"
)
FRAME_JSON = READINESS_DIR / "pre_signature_projection_mask_frame_disjointness_n32_q12289.json"
INDEPENDENCE_JSON = (
    READINESS_DIR / "pre_signature_projection_mask_independence_probe_n32_q12289.json"
)

Q = 12_289
N = 32
SELECTED_SIGNATURE_BYTES = 308
CHECKED_CORE_ROWS = 12
MASKED_PROJECTION_ROWS = 6
PACKED_WITNESS_ROWS = CHECKED_CORE_ROWS + MASKED_PROJECTION_ROWS
PAD_SEED_BYTES = 32
SLOT_ID_BYTES = 8
LANE_ID_BYTES = 8
COUNTER_BYTES = 4
SHA256_DIGEST_BYTES = 32
U16_PER_DIGEST = SHA256_DIGEST_BYTES // 2
CUTOFF_16 = (1 << 16) // Q * Q
REJECTED_U16 = (1 << 16) - CUTOFF_16
MASK_DOMAIN = (
    b"Descender.Signature.q12289.local_delta."
    b"reviewed_fq12_packed_witness_state18_projection_mask.v1"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def mask_ro_input_layout() -> list[dict[str, Any]]:
    offset = 0
    fields = []
    for name, size, value in [
        ("domain", len(MASK_DOMAIN), MASK_DOMAIN.decode()),
        ("pad_seed", PAD_SEED_BYTES, "secret live-slot pad seed"),
        ("slot_id_in_mask_seed", SLOT_ID_BYTES, "little-endian slot_id"),
        ("slot_id_stream", SLOT_ID_BYTES, "little-endian slot_id"),
        ("lane_id", LANE_ID_BYTES, "little-endian lane in [0, 5]"),
        ("counter", COUNTER_BYTES, "little-endian rejection counter"),
    ]:
        fields.append(
            {
                "field": name,
                "offset": offset,
                "bytes": size,
                "value": value,
            }
        )
        offset += size
    return fields


def build_payload() -> dict[str, Any]:
    assumption = load_json(ASSUMPTION_JSON)
    simulator = load_json(SIMULATOR_JSON)
    frame = load_json(FRAME_JSON)
    independence = load_json(INDEPENDENCE_JSON)

    mask_input_len = len(MASK_DOMAIN) + PAD_SEED_BYTES + SLOT_ID_BYTES + SLOT_ID_BYTES + LANE_ID_BYTES + COUNTER_BYTES
    if frame["mask_prf_input_len"] != mask_input_len:
        raise ValueError(
            f"mask input length mismatch: frame={frame['mask_prf_input_len']} local={mask_input_len}"
        )
    if frame["mask_domain"] != MASK_DOMAIN.decode():
        raise ValueError("mask domain mismatch against frame certificate")

    ro_game = {
        "id": "G_ro_projection_mask_stream",
        "kind": "random_oracle_contract",
        "oracle": "H_ro: {0,1}* -> {0,1}^256",
        "mask_stream_input": "MASK_DOMAIN || pad_seed || le64(slot_id) || le64(slot_id) || le64(lane) || le32(counter)",
        "mask_stream_output": (
            "scan SHA-256-sized oracle blocks as sixteen little-endian u16 candidates; "
            "accept the first candidate below 61445 and reduce it modulo 12289"
        ),
        "domains_and_inputs_consumed": {
            "mask_domain": MASK_DOMAIN.decode(),
            "mask_domain_bytes": len(MASK_DOMAIN),
            "pad_seed_bytes": PAD_SEED_BYTES,
            "slot_id_bytes_consumed_twice": SLOT_ID_BYTES,
            "lane_id_bytes": LANE_ID_BYTES,
            "counter_bytes": COUNTER_BYTES,
            "total_mask_ro_input_bytes": mask_input_len,
            "accepted_u16_cutoff": CUTOFF_16,
            "rejected_u16_values": REJECTED_U16,
            "u16_candidates_per_oracle_block": U16_PER_DIGEST,
        },
        "layout": mask_ro_input_layout(),
        "challenge": (
            "For each lifecycle-valid live slot requested by the adversary, either expose "
            "six masks generated from H_ro under the above frame, or expose six independent "
            "uniform residues in Z_q. The adversary receives the account-state view defined "
            "by the leakage simulator and outputs a bit."
        ),
        "win_condition": "absolute distinguishing advantage between the real-mask and uniform-mask branches",
    }

    prf_game = {
        "id": "G_sha256_projection_mask_prf",
        "kind": "explicit_assumption_not_theorem",
        "function_family": "F_K(x) = SHA256(K || x) is not the claim here; the implementation uses SHA256(full_frame)",
        "implemented_hash_call": "SHA256(MASK_DOMAIN || pad_seed || le64(slot_id) || le64(slot_id) || le64(lane) || le32(counter))",
        "assumption": (
            "For secret, prechallenge-fixed pad_seed values in lifecycle-valid slots, SHA-256 on "
            "the projection-mask frame is indistinguishable from the random oracle used in "
            "G_ro_projection_mask_stream for all adversarially reachable frames."
        ),
        "non_claim": "This artifact does not prove SHA-256 PRF security or instantiate the random oracle model.",
    }

    query_budget_symbols = {
        "q_slot": "number of lifecycle-valid live slots whose 308-byte/Fq12 packed witness state is exposed pre-spend",
        "q_lane": "number of projection-mask lanes requested; for this path q_lane <= 6 * q_slot",
        "q_ctr": "maximum rejection-sampling counter blocks consumed by any one lane",
        "q_mask": "total projection-mask random-oracle queries, bounded by q_lane * q_ctr",
        "q_ro_direct": "additional adversarial direct RO/SHA-256 queries in all modeled domains",
        "q_hash": "q_mask + q_ro_direct plus verifier hash queries modeled outside the mask stream",
        "q_open": "accepted spend-opening queries that intentionally reveal selected projection information",
        "q_life": "slot lifecycle create/restore/reorg attempts relevant to pad_seed and slot_id fixation",
        "t": "running time of the distinguisher including simulator overhead",
    }

    hybrid_steps = [
        {
            "hybrid": "H0_real_sha256_masks",
            "description": "The live-slot view contains raw_projection_i + mask_i mod q for six lanes, with masks derived by implemented SHA-256 frames.",
            "transition_to_next": "Replace SHA-256 on projection-mask frames with H_ro.",
            "charged_to": "Adv_sha256_projection_mask_frame_prf(q_hash, t)",
            "fail_closed_if_missing": True,
        },
        {
            "hybrid": "H1_ro_masks",
            "description": "Projection masks are sampled by rejection from H_ro on domain-separated mask frames.",
            "transition_to_next": "Abort if any mask frame equals a non-mask domain frame or if lifecycle lets pad_seed/slot_id be selected after challenge.",
            "charged_to": "Adv_frame_disjointness(q_hash) + Adv_lifecycle_fixation(q_life)",
            "fail_closed_if_missing": True,
        },
        {
            "hybrid": "H2_uniform_mask_residues",
            "description": "For each distinct mask frame, H_ro outputs independent 256-bit blocks, and rejection sampling gives exact uniform residues in Z_q.",
            "transition_to_next": "Replace raw_projection_i + U_i mod q by independent uniform residues.",
            "charged_to": "0 in ROM for unbounded rejection sampling on distinct frames",
            "fail_closed_if_missing": False,
        },
        {
            "hybrid": "H3_ideal_live_slot_view",
            "description": "The six masked projection carrier residues are simulated as uniform Z_q residues; checked-core rows and public metadata are unchanged.",
            "transition_to_next": "Condition accepted openings through the spend-time leakage interface.",
            "charged_to": "Adv_spend_opening_leakage(q_open)",
            "fail_closed_if_missing": True,
        },
    ]

    named_breaks = [
        {
            "id": "sha256_projection_mask_frame_prf_break",
            "event": "distinguish implemented SHA-256 projection-mask frames from the ROM oracle on adversarially reachable frames",
            "advantage_term": "Adv_sha256_projection_mask_frame_prf(q_hash, t)",
            "status": "assumption_required",
        },
        {
            "id": "mask_ro_frame_collision_break",
            "event": "a projection-mask byte string equals a pad PRF, transcript, opening, or other non-mask RO frame",
            "advantage_term": "Adv_frame_disjointness(q_hash)",
            "status": "partially_supported_by_byte_frame_certificate",
        },
        {
            "id": "mask_frame_repeat_or_adaptive_selection_break",
            "event": "two live lanes use the same full mask frame unexpectedly, or pad_seed/slot_id is chosen after the challenge bit",
            "advantage_term": "Adv_lifecycle_fixation(q_life)",
            "status": "open_same_game_lift",
        },
        {
            "id": "spend_time_projection_opening_break",
            "event": "selected projection information is obtained outside an accepted spend-opening certificate",
            "advantage_term": "Adv_spend_opening_leakage(q_open)",
            "status": "open_budget_contract",
        },
        {
            "id": "bounded_rejection_or_implementation_mismatch_break",
            "event": "implementation imposes a finite counter cap or parses candidates differently from the modeled stream",
            "advantage_term": "Adv_rejection_implementation_mismatch(q_mask, q_ctr)",
            "status": "open_until_tied_to_code",
        },
    ]

    advantage_bound = {
        "statement_type": "conditional_contract_not_sha256_theorem",
        "rom_bound": [
            "Adv_live_slot_projection_leakage_rom(q_slot, q_open, q_life, t)",
            "  <= Adv_frame_disjointness(q_hash)",
            "   + Adv_lifecycle_fixation(q_life)",
            "   + Adv_spend_opening_leakage(q_open)",
            "   + Adv_rejection_implementation_mismatch(q_mask, q_ctr)",
        ],
        "sha256_instantiated_bound": [
            "Adv_live_slot_projection_leakage_sha256(q_slot, q_open, q_life, t)",
            "  <= Adv_sha256_projection_mask_frame_prf(q_hash, t + O(q_hash))",
            "   + Adv_frame_disjointness(q_hash)",
            "   + Adv_lifecycle_fixation(q_life)",
            "   + Adv_spend_opening_leakage(q_open)",
            "   + Adv_rejection_implementation_mismatch(q_mask, q_ctr)",
        ],
        "exact_uniformity_note": (
            "In the ROM with distinct frames and unbounded rejection sampling, accepted u16 candidates "
            "below 61445 are exactly uniform modulo 12289 because 61445 = 5 * 12289."
        ),
    }

    open_before_gap_closes = [
        {
            "id": "sha256_instantiation",
            "status": "open_assumption",
            "required_to_close": "decide whether the global proof consumes Adv_sha256_projection_mask_frame_prf as an explicit assumption or stays in the random-oracle model",
        },
        {
            "id": "lifecycle_same_game_lift",
            "status": "open",
            "required_to_close": "lift pad_seed and slot_id uniqueness/fixation into the same adversarial experiment that exposes account states and spend attempts",
        },
        {
            "id": "spend_time_leakage_budget",
            "status": "open",
            "required_to_close": "prove selected projection openings are reachable only through accepted spend-opening certificates and are charged to q_open",
        },
        {
            "id": "implementation_rejection_equivalence",
            "status": "open",
            "required_to_close": "tie production mask generation to this exact rejection parser and counter behavior, including public reject behavior",
        },
        {
            "id": "global_domain_registry",
            "status": "open",
            "required_to_close": "register every SHA-256 frame family consumed by the signer/verifier so frame disjointness is global, not just pad-vs-mask",
        },
    ]

    return {
        "schema": "Descender.pre_signature_projection_mask_prf_ro_reduction.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "sha256_prf_or_ro_reduction_contract_fail_closed",
        "artifact_type": "conditional_reduction_contract",
        "is_theorem": False,
        "is_assumption": True,
        "deployment_ready": False,
        "target": {
            "n": N,
            "q": Q,
            "selected_signature_bytes": SELECTED_SIGNATURE_BYTES,
            "packed_witness_rows": PACKED_WITNESS_ROWS,
            "checked_core_rows": CHECKED_CORE_ROWS,
            "masked_projection_rows": MASKED_PROJECTION_ROWS,
            "state_path": "308-byte/Fq12 state-assisted masked projection-carrier path",
        },
        "source_artifacts": {
            "assumption_api": {"artifact": rel(ASSUMPTION_JSON), "status": assumption.get("status")},
            "leakage_simulator": {"artifact": rel(SIMULATOR_JSON), "status": simulator.get("status")},
            "frame_disjointness": {"artifact": rel(FRAME_JSON), "status": frame.get("status")},
            "independence_probe": {
                "artifact": rel(INDEPENDENCE_JSON),
                "status": independence.get("status"),
                "max_abs_pearson_centered": independence["summary"]["max_abs_pearson_centered"],
            },
        },
        "ro_game": ro_game,
        "prf_instantiation_game": prf_game,
        "query_budget_symbols": query_budget_symbols,
        "hybrid_steps": hybrid_steps,
        "named_breaks": named_breaks,
        "advantage_bound": advantage_bound,
        "decision": {
            "closes_simulator_gap_1": False,
            "fail_closed_progress": True,
            "can_consume_in_parent_simulator": True,
            "reason": (
                "The RO game, SHA-256 assumption boundary, mask inputs, query symbols, hybrids, "
                "named breaks, and bound terms are explicit. The simulator gap remains blocked "
                "until the parent proof chooses ROM-vs-SHA-256-assumption treatment and closes "
                "lifecycle/spend-opening/rejection equivalence in the same game."
            ),
        },
        "open_before_gap_closes": open_before_gap_closes,
        "non_claims": [
            "No proof of SHA-256 as a PRF or random oracle is claimed.",
            "No production-code equivalence is claimed for the rejection loop until tied to implementation.",
            "The independence probe is supporting evidence only and contributes no theorem credit.",
            "This artifact does not update shared trackers, indexes, Rust code, or the simulator script.",
        ],
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection Mask PRF/RO Reduction",
        "",
        "This artifact is a fail-closed reduction contract for simulator gap 1.",
        "It is not a theorem proving SHA-256 security. It specifies the ROM game,",
        "the explicit SHA-256 assumption boundary, the exact mask-stream inputs,",
        "query-budget symbols, hybrid transitions, named breaks, and remaining",
        "blockers for the 308-byte/Fq12 state-assisted masked projection-carrier path.",
        "",
        "## Status",
        "",
        f"- status: `{payload['status']}`",
        f"- artifact type: `{payload['artifact_type']}`",
        f"- theorem: `{payload['is_theorem']}`",
        f"- assumption: `{payload['is_assumption']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- closes simulator gap 1: `{payload['decision']['closes_simulator_gap_1']}`",
        f"- can consume in parent simulator: `{payload['decision']['can_consume_in_parent_simulator']}`",
        "",
        "## Target",
        "",
        f"- n: `{payload['target']['n']}`",
        f"- q: `{payload['target']['q']}`",
        f"- selected signature bytes: `{payload['target']['selected_signature_bytes']}`",
        f"- packed witness rows: `{payload['target']['packed_witness_rows']}`",
        f"- checked-core rows: `{payload['target']['checked_core_rows']}`",
        f"- masked projection rows: `{payload['target']['masked_projection_rows']}`",
        f"- state path: `{payload['target']['state_path']}`",
        "",
        "## Random-Oracle Game",
        "",
        f"- id: `{payload['ro_game']['id']}`",
        f"- oracle: `{payload['ro_game']['oracle']}`",
        f"- mask stream input: `{payload['ro_game']['mask_stream_input']}`",
        "",
        payload["ro_game"]["mask_stream_output"],
        "",
        "Layout:",
        "",
        "| field | offset | bytes | value |",
        "|---|---:|---:|---|",
    ]
    for item in payload["ro_game"]["layout"]:
        lines.append(
            f"| `{item['field']}` | {item['offset']} | {item['bytes']} | {item['value']} |"
        )
    lines.extend(
        [
            "",
            "Challenge:",
            "",
            payload["ro_game"]["challenge"],
            "",
            f"Win condition: {payload['ro_game']['win_condition']}",
            "",
            "## SHA-256 Instantiation Boundary",
            "",
            f"- id: `{payload['prf_instantiation_game']['id']}`",
            f"- kind: `{payload['prf_instantiation_game']['kind']}`",
            f"- implemented hash call: `{payload['prf_instantiation_game']['implemented_hash_call']}`",
            "",
            payload["prf_instantiation_game"]["assumption"],
            "",
            f"Non-claim: {payload['prf_instantiation_game']['non_claim']}",
            "",
            "## Query Budget Symbols",
            "",
            "| symbol | meaning |",
            "|---|---|",
        ]
    )
    for symbol, meaning in payload["query_budget_symbols"].items():
        lines.append(f"| `{symbol}` | {meaning} |")
    lines.extend(["", "## Hybrids", "", "| hybrid | transition | charged to | fail closed |", "|---|---|---|---|"])
    for step in payload["hybrid_steps"]:
        lines.append(
            f"| `{step['hybrid']}` | {step['transition_to_next']} | `{step['charged_to']}` | `{step['fail_closed_if_missing']}` |"
        )
    lines.extend(["", "## Named Breaks", "", "| break | event | advantage term | status |", "|---|---|---|---|"])
    for item in payload["named_breaks"]:
        lines.append(
            f"| `{item['id']}` | {item['event']} | `{item['advantage_term']}` | `{item['status']}` |"
        )
    lines.extend(["", "## Advantage Bound", ""])
    lines.append(f"- statement type: `{payload['advantage_bound']['statement_type']}`")
    lines.append("")
    lines.append("ROM bound:")
    lines.extend(f"- `{term}`" for term in payload["advantage_bound"]["rom_bound"])
    lines.append("")
    lines.append("SHA-256 instantiated bound:")
    lines.extend(f"- `{term}`" for term in payload["advantage_bound"]["sha256_instantiated_bound"])
    lines.extend(
        [
            "",
            payload["advantage_bound"]["exact_uniformity_note"],
            "",
            "## Source Artifacts",
            "",
            "| artifact | status | role |",
            "|---|---|---|",
        ]
    )
    roles = {
        "assumption_api": "upstream assumption boundary consumed by simulator",
        "leakage_simulator": "real/ideal live-slot leakage boundary",
        "frame_disjointness": "pad-vs-mask byte frame evidence",
        "independence_probe": "bounded stream-correlation evidence only",
    }
    for key, item in payload["source_artifacts"].items():
        lines.append(f"| `{item['artifact']}` | `{item['status']}` | {roles[key]} |")
    lines.extend(
        [
            "",
            "## Open Before Gap Closes",
            "",
            "| blocker | status | required to close |",
            "|---|---|---|",
        ]
    )
    for item in payload["open_before_gap_closes"]:
        lines.append(
            f"| `{item['id']}` | `{item['status']}` | {item['required_to_close']} |"
        )
    lines.extend(["", "## Decision", "", payload["decision"]["reason"], "", "## Non-Claims", ""])
    lines.extend(f"- {item}" for item in payload["non_claims"])
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(
        json.dumps(
            {
                "json": str(DEFAULT_JSON),
                "markdown": str(DEFAULT_MD),
                "status": payload["status"],
                "closes_simulator_gap_1": payload["decision"]["closes_simulator_gap_1"],
                "open_blockers": len(payload["open_before_gap_closes"]),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
