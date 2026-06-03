#!/usr/bin/env python3
"""Committed-sigma/account-carried verifier architecture cost model.

This is a small proof-facing ranking harness, not a cryptanalytic estimator.
It prices verifier architectures using the current Lean byte model:

    transaction = 181 + 32 + response_bytes + signature_link_bytes

and keeps account/state bytes separate from spend-transaction bytes.  Hard
filters mirror the current committed-sigma proof gates.
"""

from __future__ import annotations

import argparse
import json
import os
from dataclasses import asdict, dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_OUT = ROOT / "research/parameter_search/committed_sigma_account_carried_20260521"

BASE_SPEND_TRANSACTION_BYTES = 181
PROOF_COMMITMENT_BYTES = 32
REVIEWED_ACCOUNT_LINK_BYTES = 68
STATELESS_LINK_PROOF_TRANSACTION_BYTES = 282

READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
RESPONSE_LINK_MANIFEST = (
    READINESS_DIR
    / "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json"
)
RESPONSE_LINK_ATTACK_REVIEW = (
    READINESS_DIR
    / "response_link_lattice_attack_review_n32_q12289_coords_0_3_b96.json"
)
QUOTIENT_SIS_ATTACK_REVIEW = (
    READINESS_DIR / "quotient_sis_lattice_attack_review_n32_q12289.json"
)
REFERENCE_TIMING_REPORT = READINESS_DIR / "reference_timing_report_n32_q12289.json"
PRODUCTION_REFERENCE_FACADE_TIMING_REPORT = (
    READINESS_DIR / "production_reference_facade_timing_report_n32_q12289.json"
)
PRODUCTION_REFERENCE_GAP_REPORT = (
    READINESS_DIR / "production_reference_gap_report_n32_q12289.json"
)
KEYGEN_SECURITY_ASSUMPTION_SURFACE = (
    READINESS_DIR / "keygen_security_assumption_surface_n32_q12289.json"
)
ASSUMPTION_ATTACK_REVIEW_MATRIX = (
    READINESS_DIR / "assumption_attack_review_matrix_n32_q12289.json"
)
QROM_SECURITY_LOSS_REVIEW = READINESS_DIR / "qrom_security_loss_review_n32_q12289.json"
PAD_SAMPLER_BOUNDARY_REVIEW = (
    READINESS_DIR / "pad_sampler_boundary_review_n32_q12289.json"
)
CONDITIONAL_SOUNDNESS_BUDGET_REVIEW = (
    READINESS_DIR / "conditional_soundness_budget_review_n32_q12289.json"
)


@dataclass(frozen=True)
class Architecture:
    name: str
    response_bytes: int
    signature_link_bytes: int
    account_link_bytes: int
    proof_wired: bool
    account_backing: bool
    response_link_certified: bool
    rejects_postchallenge_representative: bool
    non_public_projection: bool
    witness_hiding: bool
    notes: str

    @property
    def proof_bytes(self) -> int:
        return PROOF_COMMITMENT_BYTES + self.response_bytes + self.signature_link_bytes

    @property
    def transaction_bytes(self) -> int:
        return BASE_SPEND_TRANSACTION_BYTES + self.proof_bytes

    @property
    def total_online_plus_account_bytes(self) -> int:
        return self.transaction_bytes + self.account_link_bytes


@dataclass(frozen=True)
class BenchmarkSignature:
    name: str
    signature_bytes: int
    public_key_bytes: int | None
    account_or_state_bytes: int
    security_label: str
    status: str
    source: str
    source_url: str
    notes: str

    @property
    def total_signature_plus_state_bytes(self) -> int:
        return self.signature_bytes + self.account_or_state_bytes


@dataclass(frozen=True)
class AccountLifecycleScenario:
    name: str
    slot_count: int
    spend_transaction_bytes: int
    account_link_bytes_per_slot: int
    replacement_write_bytes: int
    model: str
    notes: str

    @property
    def persistent_account_state_bytes(self) -> int:
        return self.slot_count * self.account_link_bytes_per_slot

    @property
    def verifier_live_read_bytes(self) -> int:
        return self.account_link_bytes_per_slot if self.slot_count > 0 else 0

    @property
    def online_plus_live_read_bytes(self) -> int:
        return self.spend_transaction_bytes + self.verifier_live_read_bytes

    @property
    def state_touched_per_spend_bytes(self) -> int:
        return self.verifier_live_read_bytes + self.replacement_write_bytes


def hard_failures(arch: Architecture) -> list[str]:
    failures: list[str] = []
    if not arch.proof_wired:
        failures.append("proof_not_wired_to_account_soundness")
    if not arch.account_backing:
        failures.append("missing_pre_response_account_backing")
    if not arch.response_link_certified:
        failures.append("missing_bounded_response_link_certificate")
    if not arch.rejects_postchallenge_representative:
        failures.append("accepts_public_postchallenge_representative")
    if not arch.non_public_projection:
        failures.append("public_projection_of_sigma_equation")
    if not arch.witness_hiding:
        failures.append("missing_witness_hiding_argument")
    return failures


def candidate_architectures() -> list[Architecture]:
    return [
        Architecture(
            name="reviewed-account-carried-bounded-kernel-37b",
            response_bytes=37,
            signature_link_bytes=0,
            account_link_bytes=REVIEWED_ACCOUNT_LINK_BYTES,
            proof_wired=True,
            account_backing=True,
            response_link_certified=True,
            rejects_postchallenge_representative=True,
            non_public_projection=True,
            witness_hiding=True,
            notes=(
                "Current Lean target: 250-byte spend transaction plus 68 account/state bytes; "
                "uses [0,3] bounded response-link certificate."
            ),
        ),
        Architecture(
            name="fiber-state-preserving-39b-with-reviewed-link",
            response_bytes=39,
            signature_link_bytes=0,
            account_link_bytes=REVIEWED_ACCOUNT_LINK_BYTES,
            proof_wired=True,
            account_backing=True,
            response_link_certified=True,
            rejects_postchallenge_representative=True,
            non_public_projection=True,
            witness_hiding=True,
            notes=(
                "Slightly larger response row that preserves extra local fiber-state equality; "
                "kept as a robustness fallback under the same account-link assumptions."
            ),
        ),
        Architecture(
            name="current-41b-public-sigma-equation",
            response_bytes=41,
            signature_link_bytes=0,
            account_link_bytes=0,
            proof_wired=False,
            account_backing=False,
            response_link_certified=False,
            rejects_postchallenge_representative=False,
            non_public_projection=False,
            witness_hiding=True,
            notes=(
                "254-byte public equation/control path. Byte-competitive but structurally rejected "
                "because public post-challenge representatives still pass."
            ),
        ),
        Architecture(
            name="bare-account-hash-37b",
            response_bytes=37,
            signature_link_bytes=0,
            account_link_bytes=32,
            proof_wired=False,
            account_backing=False,
            response_link_certified=False,
            rejects_postchallenge_representative=False,
            non_public_projection=True,
            witness_hiding=True,
            notes=(
                "Cheap account hash without an account-link proof system. Rejected as a useful "
                "negative control."
            ),
        ),
        Architecture(
            name="public-projection-homomorphic-link-37b",
            response_bytes=37,
            signature_link_bytes=0,
            account_link_bytes=REVIEWED_ACCOUNT_LINK_BYTES,
            proof_wired=False,
            account_backing=True,
            response_link_certified=False,
            rejects_postchallenge_representative=False,
            non_public_projection=False,
            witness_hiding=True,
            notes=(
                "Homomorphic link that is only a projection of the public sigma equation; "
                "rejected by the non-public-projection gate."
            ),
        ),
        Architecture(
            name="stateless-link-proof-37b-plus-32b",
            response_bytes=37,
            signature_link_bytes=32,
            account_link_bytes=0,
            proof_wired=True,
            account_backing=False,
            response_link_certified=True,
            rejects_postchallenge_representative=True,
            non_public_projection=True,
            witness_hiding=True,
            notes=(
                "Illustrative stateless fallback: avoids account link state but loses the "
                "pre-response account-backed theorem and grows the transaction."
            ),
        ),
    ]


def benchmark_signatures() -> list[BenchmarkSignature]:
    return [
        BenchmarkSignature(
            name="Descender reviewed account-carried target",
            signature_bytes=250,
            public_key_bytes=172,
            account_or_state_bytes=68,
            security_label="research target; not externally reviewed",
            status="conditional proof target",
            source="Descender Lean/cost model in this repository",
            source_url="committed_sigma_account_carried_cost_model.json",
            notes="Spend transaction bytes plus account-link state; account state is consumed/replaced by the stateful protocol.",
        ),
        BenchmarkSignature(
            name="Descender fiber-state-preserving fallback",
            signature_bytes=252,
            public_key_bytes=172,
            account_or_state_bytes=68,
            security_label="research fallback; not externally reviewed",
            status="conditional proof target",
            source="Descender Lean/cost model in this repository",
            source_url="committed_sigma_account_carried_cost_model.json",
            notes="Admissible robustness fallback with the same reviewed account-link state.",
        ),
        BenchmarkSignature(
            name="Ed25519",
            signature_bytes=64,
            public_key_bytes=32,
            account_or_state_bytes=0,
            security_label="classical, not post-quantum",
            status="standardized",
            source="RFC 8032, Section 7",
            source_url="https://www.rfc-editor.org/rfc/rfc8032.html",
            notes="Included as a classical blockchain-size baseline only.",
        ),
        BenchmarkSignature(
            name="Falcon/FN-DSA-512 padded",
            signature_bytes=666,
            public_key_bytes=897,
            account_or_state_bytes=0,
            security_label="NIST level 1 style parameter set",
            status="selected / draft standardization ecosystem",
            source="IETF draft-ietf-cose-falcon-03 / Open Quantum Safe Falcon table",
            source_url="https://datatracker.ietf.org/doc/html/draft-ietf-cose-falcon-03",
            notes="Compact lattice signature benchmark; implementation complexity and sampling side channels are separate concerns.",
        ),
        BenchmarkSignature(
            name="ML-DSA-44",
            signature_bytes=2420,
            public_key_bytes=1312,
            account_or_state_bytes=0,
            security_label="FIPS 204 category 2",
            status="FIPS 204 standardized",
            source="FIPS 204 Table 2",
            source_url="https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.204.pdf",
            notes="Formerly Dilithium-style module-lattice signature.",
        ),
        BenchmarkSignature(
            name="ML-DSA-65",
            signature_bytes=3309,
            public_key_bytes=1952,
            account_or_state_bytes=0,
            security_label="FIPS 204 category 3",
            status="FIPS 204 standardized",
            source="FIPS 204 Table 2",
            source_url="https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.204.pdf",
            notes="Common higher-security ML-DSA benchmark row.",
        ),
        BenchmarkSignature(
            name="SLH-DSA-SHA2-128s",
            signature_bytes=7856,
            public_key_bytes=32,
            account_or_state_bytes=0,
            security_label="FIPS 205 category 1",
            status="FIPS 205 standardized",
            source="FIPS 205 Table 2",
            source_url="https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.205.pdf",
            notes="Small SLH-DSA signature variant among FIPS 205 category-1 rows.",
        ),
        BenchmarkSignature(
            name="SLH-DSA-SHA2-128f",
            signature_bytes=17088,
            public_key_bytes=32,
            account_or_state_bytes=0,
            security_label="FIPS 205 category 1",
            status="FIPS 205 standardized",
            source="FIPS 205 Table 2",
            source_url="https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.205.pdf",
            notes="Fast SLH-DSA category-1 variant; much larger signatures.",
        ),
    ]


def account_lifecycle_scenarios() -> list[AccountLifecycleScenario]:
    return [
        AccountLifecycleScenario(
            name="rolling-one-slot-account",
            slot_count=1,
            spend_transaction_bytes=250,
            account_link_bytes_per_slot=REVIEWED_ACCOUNT_LINK_BYTES,
            replacement_write_bytes=REVIEWED_ACCOUNT_LINK_BYTES,
            model="stateful rolling slot",
            notes=(
                "Smallest blockchain state footprint: one live 68-byte account-link "
                "slot is consumed and atomically replaced by the next 68-byte slot."
            ),
        ),
        AccountLifecycleScenario(
            name="precommitted-16-slot-pool",
            slot_count=16,
            spend_transaction_bytes=250,
            account_link_bytes_per_slot=REVIEWED_ACCOUNT_LINK_BYTES,
            replacement_write_bytes=0,
            model="finite precommitted pool",
            notes=(
                "Avoids a fresh replacement commitment in the same spend, but ledger "
                "logic still needs spent-slot/nullifier accounting outside this proof "
                "byte model."
            ),
        ),
        AccountLifecycleScenario(
            name="precommitted-256-slot-pool",
            slot_count=256,
            spend_transaction_bytes=250,
            account_link_bytes_per_slot=REVIEWED_ACCOUNT_LINK_BYTES,
            replacement_write_bytes=0,
            model="finite precommitted pool",
            notes=(
                "Useful for high-throughput accounts if the chain can tolerate the "
                "larger persistent state and has clean spent-slot accounting."
            ),
        ),
        AccountLifecycleScenario(
            name="precommitted-1024-slot-pool",
            slot_count=1024,
            spend_transaction_bytes=250,
            account_link_bytes_per_slot=REVIEWED_ACCOUNT_LINK_BYTES,
            replacement_write_bytes=0,
            model="finite precommitted pool",
            notes=(
                "Stress row for bulk account provisioning; cryptographic marginal "
                "verification still reads one 68-byte slot per spend."
            ),
        ),
        AccountLifecycleScenario(
            name="stateless-link-proof-control",
            slot_count=0,
            spend_transaction_bytes=STATELESS_LINK_PROOF_TRANSACTION_BYTES,
            account_link_bytes_per_slot=0,
            replacement_write_bytes=0,
            model="stateless negative/control row",
            notes=(
                "No account state, but this row is not the current account-backed "
                "theorem target and is retained only as a deployment comparison."
            ),
        ),
    ]


def row(arch: Architecture) -> dict[str, Any]:
    failures = hard_failures(arch)
    return {
        "architecture": asdict(arch),
        "proof_bytes": arch.proof_bytes,
        "transaction_bytes": arch.transaction_bytes,
        "account_link_bytes": arch.account_link_bytes,
        "total_online_plus_account_bytes": arch.total_online_plus_account_bytes,
        "hard_failures": failures,
        "structurally_admissible": not failures,
        "under_256_transaction": arch.transaction_bytes <= 256,
        "under_269_transaction": arch.transaction_bytes <= 269,
    }


def pareto_frontier(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    accepted = [r for r in rows if r["structurally_admissible"]]
    frontier: list[dict[str, Any]] = []
    for candidate in accepted:
        dominated = False
        for other in accepted:
            if other is candidate:
                continue
            if (
                other["transaction_bytes"] <= candidate["transaction_bytes"]
                and other["account_link_bytes"] <= candidate["account_link_bytes"]
                and other["proof_bytes"] <= candidate["proof_bytes"]
                and (
                    other["transaction_bytes"] < candidate["transaction_bytes"]
                    or other["account_link_bytes"] < candidate["account_link_bytes"]
                    or other["proof_bytes"] < candidate["proof_bytes"]
                )
            ):
                dominated = True
                break
        if not dominated:
            frontier.append(candidate)
    return sorted(frontier, key=lambda r: (r["transaction_bytes"], r["account_link_bytes"], r["proof_bytes"]))


def benchmark_row(benchmark: BenchmarkSignature) -> dict[str, Any]:
    return {
        **asdict(benchmark),
        "total_signature_plus_state_bytes": benchmark.total_signature_plus_state_bytes,
        "descender_size_ratio_vs_signature": round(250 / benchmark.signature_bytes, 6),
        "descender_total_ratio_vs_signature": round(318 / benchmark.signature_bytes, 6),
    }


def account_lifecycle_row(scenario: AccountLifecycleScenario) -> dict[str, Any]:
    return {
        **asdict(scenario),
        "persistent_account_state_bytes": scenario.persistent_account_state_bytes,
        "verifier_live_read_bytes": scenario.verifier_live_read_bytes,
        "online_plus_live_read_bytes": scenario.online_plus_live_read_bytes,
        "state_touched_per_spend_bytes": scenario.state_touched_per_spend_bytes,
    }


def read_json_or_none(path: Path) -> dict[str, Any] | None:
    if not path.exists():
        return None
    return json.loads(path.read_text())


def relative_to_root(path: Path) -> str:
    try:
        return str(path.relative_to(ROOT))
    except ValueError:
        return str(path)


def review_gate_rows() -> list[dict[str, Any]]:
    manifest = read_json_or_none(RESPONSE_LINK_MANIFEST)
    response_review = read_json_or_none(RESPONSE_LINK_ATTACK_REVIEW)
    quotient_review = read_json_or_none(QUOTIENT_SIS_ATTACK_REVIEW)
    timing_review = read_json_or_none(REFERENCE_TIMING_REPORT)
    facade_timing_review = read_json_or_none(PRODUCTION_REFERENCE_FACADE_TIMING_REPORT)
    production_gap = read_json_or_none(PRODUCTION_REFERENCE_GAP_REPORT)
    keygen_surface = read_json_or_none(KEYGEN_SECURITY_ASSUMPTION_SURFACE)
    assumption_matrix = read_json_or_none(ASSUMPTION_ATTACK_REVIEW_MATRIX)
    qrom_review = read_json_or_none(QROM_SECURITY_LOSS_REVIEW)
    pad_review = read_json_or_none(PAD_SAMPLER_BOUNDARY_REVIEW)
    soundness_budget = read_json_or_none(CONDITIONAL_SOUNDNESS_BUDGET_REVIEW)

    gates: list[dict[str, Any]] = []
    if manifest is None:
        gates.append(
            {
                "gate": "response_link_exact_certificate",
                "status": "missing_artifact",
                "artifact": relative_to_root(RESPONSE_LINK_MANIFEST),
                "summary": "Audit manifest is missing.",
            }
        )
    else:
        replay = manifest.get("replay", {})
        basis = replay.get("basis_validation", {})
        ok = (
            replay.get("status") == "verified"
            and replay.get("mismatches") == []
            and basis.get("valid") is True
        )
        gates.append(
            {
                "gate": "response_link_exact_certificate",
                "status": "pass" if ok else "review_needed",
                "artifact": relative_to_root(RESPONSE_LINK_MANIFEST),
                "summary": (
                    "Exact bounded enumeration manifest verifies the [0,3] "
                    "response-link kernel exclusion."
                ),
                "key_metrics": {
                    "manifest_payload_sha256": manifest.get("manifest_payload_sha256"),
                    "visited_nodes": replay.get("visited_nodes"),
                    "visited_leaves": replay.get("visited_leaves"),
                    "nonzero_vectors_in_cover_ball": replay.get("nonzero_vectors_in_cover_ball"),
                    "shortest_cover_ball_coeff_bound": replay.get(
                        "shortest_vector_in_cover_ball", {}
                    ).get("coeff_bound"),
                },
            }
        )

    if response_review is None:
        gates.append(
            {
                "gate": "response_link_lattice_attack_review",
                "status": "missing_artifact",
                "artifact": relative_to_root(RESPONSE_LINK_ATTACK_REVIEW),
                "summary": "Local LLL/BKZ response-link attack review is missing.",
            }
        )
    else:
        reductions = response_review.get("reductions", [])
        found_box_vector = any(item.get("found_box_vector") for item in reductions)
        geometry = response_review.get("geometry", {})
        gates.append(
            {
                "gate": "response_link_lattice_attack_review",
                "status": "pass" if not found_box_vector else "fail",
                "artifact": relative_to_root(RESPONSE_LINK_ATTACK_REVIEW),
                "summary": (
                    "LLL/BKZ attack review found no vector inside the accepted "
                    "response-link box."
                ),
                "key_metrics": {
                    "bkz_blocks": [
                        item.get("block_size")
                        for item in reductions
                        if item.get("block_size") is not None
                    ],
                    "shortest_coeff_bound": geometry.get(
                        "shortest_cover_ball_vector_coeff_bound"
                    ),
                    "accepted_bound": geometry.get("accepted_box_coeff_bound"),
                    "margin_over_bound": geometry.get(
                        "shortest_cover_ball_vector_margin_over_box_bound"
                    ),
                },
            }
        )

    if quotient_review is None:
        gates.append(
            {
                "gate": "quotient_sis_bare_quotient_control",
                "status": "missing_artifact",
                "artifact": relative_to_root(QUOTIENT_SIS_ATTACK_REVIEW),
                "summary": "Quotient/SIS lattice attack review is missing.",
            }
        )
    else:
        shortest = quotient_review.get("shortest_reduction_row", {})
        caught_by_link = shortest.get("caught_by_response_link_code_0_3") is True
        inside_bound = shortest.get("inside_response_difference_bound_96") is True
        gates.append(
            {
                "gate": "quotient_sis_bare_quotient_control",
                "status": "pass" if caught_by_link and inside_bound else "review_needed",
                "artifact": relative_to_root(QUOTIENT_SIS_ATTACK_REVIEW),
                "summary": (
                    "Bare quotient has a short bounded kernel vector, and the "
                    "selected [0,3] account response-link code catches it."
                ),
                "key_metrics": {
                    "rank": quotient_review.get("statement", {}).get("rank"),
                    "kernel_dimension": quotient_review.get("statement", {}).get(
                        "kernel_dimension"
                    ),
                    "shortest_coeff_bound": shortest.get("coeff_bound"),
                    "inside_response_bound_96": inside_bound,
                    "response_link_coords_0_3_zero": shortest.get(
                        "response_link_coords_0_3_zero_mod_q"
                    ),
                    "sparse_zero_relations": quotient_review.get(
                        "sparse_signed_relation_probe", {}
                    ).get("zero_relations_found"),
                },
            }
        )

    if timing_review is None:
        gates.append(
            {
                "gate": "reference_timing_smoke_benchmark",
                "status": "missing_artifact",
                "artifact": relative_to_root(REFERENCE_TIMING_REPORT),
                "summary": "Reference timing report is missing.",
            }
        )
    else:
        timing = timing_review.get("timing_report", {})
        timing_rows = {
            row.get("operation"): row for row in timing.get("timings", [])
        }
        all_ok = all(row.get("accepted_or_ok") is True for row in timing_rows.values())
        gates.append(
            {
                "gate": "reference_timing_smoke_benchmark",
                "status": "pass" if all_ok else "review_needed",
                "artifact": relative_to_root(REFERENCE_TIMING_REPORT),
                "summary": (
                    "Reference binary timing smoke benchmark generated with "
                    "direct reference and production-facade rows; still not a "
                    "production constant-time claim."
                ),
                "key_metrics": {
                    "iterations": timing.get("iterations"),
                    "signature_bytes": timing.get("signature_bytes"),
                    "keygen_avg_us": timing_rows.get("reference_keygen", {}).get("avg_us"),
                    "sign_avg_us": timing_rows.get(
                        "reference_sign_with_proof", {}
                    ).get("avg_us"),
                    "verify_avg_us": timing_rows.get(
                        "reference_verify_with_proof", {}
                    ).get("avg_us"),
                    "production_facade_keygen_avg_us": timing_rows.get(
                        "production_facade_keygen", {}
                    ).get("avg_us"),
                    "production_facade_sign_avg_us": timing_rows.get(
                        "production_facade_sign", {}
                    ).get("avg_us"),
                    "production_facade_verify_avg_us": timing_rows.get(
                        "production_facade_verify", {}
                    ).get("avg_us"),
                },
            }
        )

    if facade_timing_review is None:
        gates.append(
            {
                "gate": "production_reference_crate_facade_timing",
                "status": "missing_artifact",
                "artifact": relative_to_root(PRODUCTION_REFERENCE_FACADE_TIMING_REPORT),
                "summary": "Separated production-reference crate facade timing report is missing.",
            }
        )
    else:
        timing = facade_timing_review.get("timing_report", {})
        timing_rows = {
            row.get("operation"): row for row in timing.get("timings", [])
        }
        all_ok = all(row.get("accepted_or_ok") is True for row in timing_rows.values())
        gate_ok = (
            all_ok
            and timing.get("signature_bytes") == 254
            and timing.get("reviewed_target_signature_bytes") == 250
            and "no accepting 284-byte/Fq12 hidden-source proof semantics"
            in str(timing.get("caveat", ""))
        )
        gates.append(
            {
                "gate": "production_reference_crate_facade_timing",
                "status": "pass" if gate_ok else "review_needed",
                "artifact": relative_to_root(PRODUCTION_REFERENCE_FACADE_TIMING_REPORT),
                "summary": (
                    "Separated production-reference crate facade timing is present "
                    "with deterministic dummy callbacks; this measures wrapper/API "
                    "overhead only."
                ),
                "key_metrics": {
                    "iterations": timing.get("iterations"),
                    "signature_bytes": timing.get("signature_bytes"),
                    "reviewed_target_signature_bytes": timing.get(
                        "reviewed_target_signature_bytes"
                    ),
                    "account_link_bytes": timing.get("account_link_bytes"),
                    "sign_facade_avg_us": timing_rows.get(
                        "deployment_sign_with_secret_key_dummy_callback", {}
                    ).get("avg_us"),
                    "verify_accept_facade_avg_us": timing_rows.get(
                        "deployment_verify_with_account_binding_dummy_accept", {}
                    ).get("avg_us"),
                    "verify_reject_facade_avg_us": timing_rows.get(
                        "deployment_verify_with_account_binding_sanitized_callback_reject",
                        {},
                    ).get("avg_us"),
                    "reserved_sign_consume_facade_avg_us": timing_rows.get(
                        "deployment_sign_reserved_and_consume_fresh_manager_dummy_callback",
                        {},
                    ).get("avg_us"),
                },
            }
        )

    if keygen_surface is None:
        gates.append(
            {
                "gate": "keygen_security_assumption_surface",
                "status": "missing_artifact",
                "artifact": relative_to_root(KEYGEN_SECURITY_ASSUMPTION_SURFACE),
                "summary": "Keygen/security-estimator assumption surface is missing.",
            }
        )
    else:
        summary = keygen_surface.get("summary", {})
        open_external = int(summary.get("open_external_items") or 0)
        missing_local = int(summary.get("missing_local_artifacts") or 0)
        gates.append(
            {
                "gate": "keygen_security_assumption_surface",
                "status": "review_needed"
                if open_external or missing_local
                else "pass",
                "artifact": relative_to_root(KEYGEN_SECURITY_ASSUMPTION_SURFACE),
                "summary": (
                    "Machine-readable assumption map generated for matrix/keygen, "
                    "bare quotient, response-link, QROM, pad/sampler, and "
                    "production lifecycle review."
                ),
                "key_metrics": {
                    "assumptions_tracked": len(keygen_surface.get("assumptions", [])),
                    "missing_local_artifacts": missing_local,
                    "open_external_items": open_external,
                    "open_assumption_ids": [
                        item.get("assumption_id")
                        for item in keygen_surface.get("assumptions", [])
                        if str(item.get("external_status", "")).startswith("open")
                    ],
                },
            }
        )

    if assumption_matrix is None:
        gates.append(
            {
                "gate": "assumption_attack_review_matrix",
                "status": "missing_artifact",
                "artifact": relative_to_root(ASSUMPTION_ATTACK_REVIEW_MATRIX),
                "summary": "Assumption-level attack review matrix is missing.",
            }
        )
    else:
        summary = assumption_matrix.get("summary", {})
        memo_only = int(summary.get("memo_only_rows") or 0)
        deployment_ready = int(summary.get("deployment_ready_rows") or 0)
        gates.append(
            {
                "gate": "assumption_attack_review_matrix",
                "status": "review_needed"
                if memo_only or deployment_ready == 0
                else "pass",
                "artifact": relative_to_root(ASSUMPTION_ATTACK_REVIEW_MATRIX),
                "summary": (
                    "Every keygen/security assumption now has local executable "
                    "or generated review evidence, but no row is deployment-ready."
                ),
                "key_metrics": {
                    "assumptions_mapped": summary.get("assumptions_mapped"),
                    "executable_backed_rows": summary.get("executable_backed_rows"),
                    "memo_only_rows": memo_only,
                    "deployment_ready_rows": deployment_ready,
                },
            }
        )

    if qrom_review is None:
        gates.append(
            {
                "gate": "qrom_security_loss_review",
                "status": "missing_artifact",
                "artifact": relative_to_root(QROM_SECURITY_LOSS_REVIEW),
                "summary": "QROM security-loss arithmetic review is missing.",
            }
        )
    else:
        failed = len(qrom_review.get("failed_checks", []))
        derived = qrom_review.get("derived_terms", {})
        gates.append(
            {
                "gate": "qrom_security_loss_review",
                "status": "pass" if qrom_review.get("status") == "checks_passed" else "fail",
                "artifact": relative_to_root(QROM_SECURITY_LOSS_REVIEW),
                "summary": (
                    "Concrete challenge-frame/QROM ledger arithmetic is checked; "
                    "QROM model review remains external."
                ),
                "key_metrics": {
                    "failed_checks": failed,
                    "deployment_min_term_bits": derived.get("deployment_min_term_bits"),
                    "deployment_min_margin_bits": derived.get(
                        "deployment_min_margin_bits"
                    ),
                    "quantum_transcript_term_bits": derived.get(
                        "quantum_transcript_term_bits"
                    ),
                    "classical_transcript_term_bits": derived.get(
                        "classical_transcript_term_bits"
                    ),
                },
            }
        )

    if pad_review is None:
        gates.append(
            {
                "gate": "pad_sampler_boundary_review",
                "status": "missing_artifact",
                "artifact": relative_to_root(PAD_SAMPLER_BOUNDARY_REVIEW),
                "summary": "Pad/sampler boundary review is missing.",
            }
        )
    else:
        source_kats = pad_review.get("source_kats", {})
        gates.append(
            {
                "gate": "pad_sampler_boundary_review",
                "status": "pass" if pad_review.get("status") == "checks_passed" else "fail",
                "artifact": relative_to_root(PAD_SAMPLER_BOUNDARY_REVIEW),
                "summary": (
                    "Pad/sampler arithmetic and forbidden active-pad/opening "
                    "conditioning boundary are checked locally."
                ),
                "key_metrics": {
                    "failed_checks": len(pad_review.get("failed_checks", [])),
                    "source_kats_present": sum(
                        1 for present in source_kats.values() if present
                    ),
                    "source_kats_total": len(source_kats),
                },
            }
        )

    if soundness_budget is None:
        gates.append(
            {
                "gate": "conditional_soundness_budget_review",
                "status": "missing_artifact",
                "artifact": relative_to_root(CONDITIONAL_SOUNDNESS_BUDGET_REVIEW),
                "summary": "Conditional soundness budget ledger is missing.",
            }
        )
    else:
        local_summary = soundness_budget.get("local_check_summary", {})
        gates.append(
            {
                "gate": "conditional_soundness_budget_review",
                "status": "review_needed"
                if soundness_budget.get("deployment_ready") is not True
                else "pass",
                "artifact": relative_to_root(CONDITIONAL_SOUNDNESS_BUDGET_REVIEW),
                "summary": (
                    "Named break terms have concrete local evidence and charged "
                    "bounds; external/model review and production preflight remain open."
                ),
                "key_metrics": {
                    "budget_terms": len(soundness_budget.get("budget_terms", [])),
                    "minimum_checked_term_bits": local_summary.get(
                        "minimum_checked_term_bits"
                    ),
                    "all_local_checks_passed": local_summary.get(
                        "all_local_checks_passed"
                    ),
                    "external_obligations": len(
                        soundness_budget.get("external_obligations", [])
                    ),
                    "deployment_ready": soundness_budget.get("deployment_ready"),
                },
            }
        )

    gates.extend(
        [
            {
                "gate": "external_independent_replay",
                "status": "open",
                "artifact": "third-party process",
                "summary": (
                    "Needs replay by someone outside this workspace, ideally "
                    "including independent SVP/BKZ validation."
                ),
            },
            {
                "gate": "production_constant_time_state_lifecycle",
                "status": "open",
                "artifact": relative_to_root(PRODUCTION_REFERENCE_GAP_REPORT)
                if production_gap is not None
                else "not yet split from research fixture",
                "summary": (
                    "A production-shaped facade now pins the 68-byte account "
                    "layout and the separated crate exposes wrapper/API timing, "
                    "but constant-time signing, durable state lifecycle review, "
                    "and accepting 284-byte/Fq12 proof semantics remain open."
                ),
                "key_metrics": {
                    "open_or_partial_api_rows": sum(
                        1
                        for row in (production_gap or {}).get("api_rows", [])
                        if row.get("status") in {"open", "partial"}
                    )
                }
                if production_gap is not None
                else {},
            },
        ]
    )
    return gates


def timing_summary_rows() -> list[dict[str, Any]]:
    timing_review = read_json_or_none(REFERENCE_TIMING_REPORT)
    rows = []
    if timing_review is not None:
        timing = timing_review.get("timing_report", {})
        for item in timing.get("timings", []):
            rows.append(
                {
                    "timing_layer": "research-reference-binary",
                    "operation": item.get("operation"),
                    "avg_us": item.get("avg_us"),
                    "iterations": item.get("iterations"),
                    "accepted_or_ok": item.get("accepted_or_ok"),
                    "signature_bytes": timing.get("signature_bytes"),
                    "reviewed_target_signature_bytes": None,
                    "caveat": timing.get("caveat"),
                }
            )
    facade_timing_review = read_json_or_none(PRODUCTION_REFERENCE_FACADE_TIMING_REPORT)
    if facade_timing_review is not None:
        timing = facade_timing_review.get("timing_report", {})
        for item in timing.get("timings", []):
            rows.append(
                {
                    "timing_layer": "production-reference-crate-facade",
                    "operation": item.get("operation"),
                    "avg_us": item.get("avg_us"),
                    "iterations": item.get("iterations"),
                    "accepted_or_ok": item.get("accepted_or_ok"),
                    "signature_bytes": timing.get("signature_bytes"),
                    "reviewed_target_signature_bytes": timing.get(
                        "reviewed_target_signature_bytes"
                    ),
                    "caveat": timing.get("caveat"),
                }
            )
    return rows


def markdown_link_target(path_text: str, out_dir: Path) -> str:
    path = Path(path_text)
    if path_text.startswith("http://") or path_text.startswith("https://"):
        return path_text
    if not (path_text.endswith(".json") or path_text.endswith(".md")):
        return path_text
    absolute = path if path.is_absolute() else ROOT / path
    return os.path.relpath(absolute, out_dir)


def markdown_report(payload: dict[str, Any], out_dir: Path) -> str:
    lines = [
        "# Committed-Sigma Account-Carried Cost Model",
        "",
        "This is a proof-facing byte model. It ranks verifier architectures only after",
        "applying the current committed-sigma structural gates; it is not a security",
        "estimator.",
        "",
        "## Constants",
        "",
        f"- base spend transaction bytes: `{BASE_SPEND_TRANSACTION_BYTES}`",
        f"- proof commitment bytes: `{PROOF_COMMITMENT_BYTES}`",
        f"- reviewed account-link bytes: `{REVIEWED_ACCOUNT_LINK_BYTES}`",
        "",
        "## Account State Lifecycle Costs",
        "",
        "These rows distinguish cryptographic payload size from ledger state.",
        "The live verifier reads one 68-byte account-link slot for the current",
        "target; larger slot pools increase persistent account state, not spend",
        "signature bytes.  Spent-slot/nullifier machinery for finite pools remains",
        "a ledger-design cost outside the current proof byte model.",
        "",
        "| scenario | model | slots | tx bytes | persistent account state | live read | replacement write | tx+live read | state touched/spend |",
        "|---|---|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for item in payload["account_lifecycle"]:
        lines.append(
            f"| {item['name']} | {item['model']} | {item['slot_count']} | "
            f"{item['spend_transaction_bytes']} | {item['persistent_account_state_bytes']} | "
            f"{item['verifier_live_read_bytes']} | {item['replacement_write_bytes']} | "
            f"{item['online_plus_live_read_bytes']} | {item['state_touched_per_spend_bytes']} |"
        )
    lines.extend(
        [
            "",
            "## Pareto Frontier",
            "",
            "| architecture | tx bytes | account bytes | proof bytes | total tx+account |",
            "|---|---:|---:|---:|---:|",
        ]
    )
    for item in payload["pareto_frontier"]:
        arch = item["architecture"]
        lines.append(
            f"| {arch['name']} | {item['transaction_bytes']} | {item['account_link_bytes']} | "
            f"{item['proof_bytes']} | {item['total_online_plus_account_bytes']} |"
        )
    lines.extend(
        [
            "",
            "## All Candidates",
            "",
            "| architecture | admissible | tx bytes | account bytes | failures |",
            "|---|:---:|---:|---:|---|",
        ]
    )
    for item in payload["rows"]:
        arch = item["architecture"]
        failures = ", ".join(item["hard_failures"]) if item["hard_failures"] else "-"
        admissible = "yes" if item["structurally_admissible"] else "no"
        lines.append(
            f"| {arch['name']} | {admissible} | {item['transaction_bytes']} | "
            f"{item['account_link_bytes']} | {failures} |"
        )
    lines.extend(
        [
            "",
            "## External Size Benchmarks",
            "",
            "These rows are byte-size comparators only. They do not imply equivalent",
            "security review status or deployment assumptions.",
            "",
            "| scheme | sig bytes | pk bytes | state bytes | sig+state | status | source |",
            "|---|---:|---:|---:|---:|---|---|",
        ]
    )
    for item in payload["external_benchmarks"]:
        pk = "-" if item["public_key_bytes"] is None else str(item["public_key_bytes"])
        lines.append(
            f"| {item['name']} | {item['signature_bytes']} | {pk} | "
            f"{item['account_or_state_bytes']} | {item['total_signature_plus_state_bytes']} | "
            f"{item['status']} | [{item['source']}]({item['source_url']}) |"
        )
    if payload["timing_summary"]:
        lines.extend(
            [
                "",
                "## Local Timing Summary",
                "",
                "These rows are local smoke timings only. They are not constant-time",
                "claims. The research-reference rows still wrap the legacy 254-byte",
                "proof callback, and the production-reference crate rows use dummy",
                "callbacks to measure wrapper/API overhead only.",
                "",
                "| layer | operation | avg us | iterations | executable sig bytes | target sig bytes | accepted/ok |",
                "|---|---|---:|---:|---:|---:|:---:|",
            ]
        )
        for item in payload["timing_summary"]:
            accepted = "yes" if item["accepted_or_ok"] else "no"
            target_bytes = (
                "-"
                if item["reviewed_target_signature_bytes"] is None
                else str(item["reviewed_target_signature_bytes"])
            )
            lines.append(
                f"| {item['timing_layer']} | {item['operation']} | {item['avg_us']} | "
                f"{item['iterations']} | {item['signature_bytes']} | {target_bytes} | "
                f"{accepted} |"
            )
    lines.extend(
        [
            "",
            "## Security Review Gates",
            "",
            "| gate | status | artifact | summary |",
            "|---|---|---|---|",
        ]
    )
    for item in payload["review_gates"]:
        artifact = item["artifact"]
        if artifact.endswith(".json") or artifact.endswith(".md"):
            artifact_cell = f"[{Path(artifact).name}]({markdown_link_target(artifact, out_dir)})"
        else:
            artifact_cell = artifact
        lines.append(
            f"| {item['gate']} | {item['status']} | {artifact_cell} | {item['summary']} |"
        )
    lines.extend(
        [
            "",
            "## Interpretation",
            "",
            "The current winner is the reviewed account-carried bounded-kernel row:",
            "`250 B` spend transaction plus `68 B` account/state bytes. The old `254 B`",
            "public sigma-equation row remains useful as a negative control, but it is",
            "structurally rejected by the post-challenge representative and public",
            "projection gates. Against external rows, the important caveat is proof",
            "maturity: Descender is far smaller on bytes, but ML-DSA, SLH-DSA, Ed25519,",
            "and Falcon/FN-DSA rows have much more mature public review status. The",
            "review gates above separate local internal evidence from the remaining",
            "external replay, production timing, and deployment-hardening work.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out-dir", type=Path, default=DEFAULT_OUT)
    args = parser.parse_args()

    rows = [row(arch) for arch in candidate_architectures()]
    payload = {
        "schema": "Descender.committed_sigma.account_carried_cost_model.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "constants": {
            "base_spend_transaction_bytes": BASE_SPEND_TRANSACTION_BYTES,
            "proof_commitment_bytes": PROOF_COMMITMENT_BYTES,
            "reviewed_account_link_bytes": REVIEWED_ACCOUNT_LINK_BYTES,
        },
        "rows": sorted(rows, key=lambda r: (not r["structurally_admissible"], r["transaction_bytes"], r["account_link_bytes"])),
        "pareto_frontier": pareto_frontier(rows),
        "external_benchmarks": [benchmark_row(item) for item in benchmark_signatures()],
        "account_lifecycle": [
            account_lifecycle_row(item) for item in account_lifecycle_scenarios()
        ],
        "timing_summary": timing_summary_rows(),
        "review_gates": review_gate_rows(),
    }

    args.out_dir.mkdir(parents=True, exist_ok=True)
    json_path = args.out_dir / "committed_sigma_account_carried_cost_model.json"
    md_path = args.out_dir / "committed_sigma_account_carried_cost_model.md"
    json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    md_path.write_text(markdown_report(payload, args.out_dir))
    print(json.dumps({"json_path": str(json_path), "markdown_path": str(md_path), "frontier_count": len(payload["pareto_frontier"])}, indent=2))


if __name__ == "__main__":
    main()
