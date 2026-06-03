#!/usr/bin/env python3
"""Build a global SHA-256/domain registry for the selected projection route."""

from __future__ import annotations

import json
import re
from collections import defaultdict
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "pre_signature_projection_global_domain_registry_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_signature_projection_global_domain_registry_n32_q12289.md"
COMPOSITION_JSON = (
    READINESS_DIR
    / "pre_signature_projection_mask_prf_leakage_theorem_composition_n32_q12289.json"
)
FRAME_JSON = READINESS_DIR / "pre_signature_projection_mask_frame_disjointness_n32_q12289.json"
RO_REDUCTION_JSON = READINESS_DIR / "pre_signature_projection_mask_prf_ro_reduction_n32_q12289.json"
PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"

Q = 12_289
N = 32
SELECTED_SIGNATURE_BYTES = 308
DIGEST_BYTES = 32
GROUPS = 16
REPS = 26
LOCAL_FREE = 2
PAD_SEED_BYTES = 32
PAD_FRAME_BYTES = 46
SLOT_ID_BYTES_U32 = 4
SLOT_ID_BYTES_U64 = 8
MASK_LANES = 6
CARRIER_LANES = 12
CHECK_BYTES = 3
TRANSCRIPT_BINDING_BYTES = 29


@dataclass(frozen=True)
class RegistryEntry:
    id: str
    domain: str
    family: str
    role: str
    source: str
    rust_const: str | None
    frame_encoding: str
    frame_shape: str
    domain_offset: int | str
    min_input_bytes: int | None
    max_input_bytes: int | None
    selected_route_relevance: str
    notes: tuple[str, ...] = ()


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rust_string_consts(source: str) -> dict[str, str]:
    pattern = re.compile(
        r"pub const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*(.*?);",
        re.DOTALL,
    )
    out: dict[str, str] = {}
    for name, expr in pattern.findall(source):
        parts = re.findall(r'"([^"]*)"', expr)
        if parts:
            out[name] = "".join(parts)
    return out


def descender_string_occurrences(paths: list[Path]) -> dict[str, list[str]]:
    pattern = re.compile(r'"(Descender[^"]*)"|b"(Descender[^"]*)"')
    occurrences: dict[str, list[str]] = defaultdict(list)
    for path in paths:
        if not path.exists():
            continue
        text = path.read_text(errors="replace")
        for line_no, line in enumerate(text.splitlines(), 1):
            for match in pattern.finditer(line):
                value = match.group(1) or match.group(2)
                occurrences[value].append(f"{rel(path)}:{line_no}")
    return dict(sorted(occurrences.items()))


def fixed_len(*sizes: int) -> int:
    return sum(sizes)


def tlv_min(domain: str, required_fields: list[tuple[str, int]]) -> int:
    # append_frame encodes u16(label_len) || label || u64(data_len) || data.
    # append_u64 is the same wrapper with eight data bytes.
    return sum(2 + len(label) + 8 + size for label, size in [("domain", len(domain)), *required_fields])


def build_registry(consts: dict[str, str]) -> list[RegistryEntry]:
    profile = consts["PROFILE"]

    entries = [
        RegistryEntry(
            id="challenge_hash_frame",
            domain=consts["CHALLENGE_HASH_DOMAIN"],
            family="check",
            role="Fiat-Shamir challenge binding for signer/verifier checked core",
            source=f"{rel(PRODUCTION_REFERENCE)}:10748",
            rust_const="CHALLENGE_HASH_DOMAIN",
            frame_encoding="append_frame_tlv",
            frame_shape="domain, profile, q, groups, local_pivot, local_free, repetitions, slot_id, public_key, message, slot_digest",
            domain_offset=10,
            min_input_bytes=tlv_min(
                consts["CHALLENGE_HASH_DOMAIN"],
                [
                    ("profile", len(profile)),
                    ("q", 8),
                    ("groups", 8),
                    ("local_pivot", 8),
                    ("local_free", 8),
                    ("repetitions", 8),
                    ("slot_id", 8),
                    ("public_key", 0),
                    ("message", 0),
                    ("slot_digest", DIGEST_BYTES),
                ],
            ),
            max_input_bytes=None,
            selected_route_relevance="signer_and_verifier",
            notes=("message is variable length in the current frame",),
        ),
        RegistryEntry(
            id="reviewed_fq12_account_carried_matrix",
            domain=consts["REVIEWED_FQ12_ACCOUNT_CARRIED_MATRIX_DOMAIN"],
            family="check",
            role="Fq12 checked-core account-carried matrix expansion",
            source=f"{rel(PRODUCTION_REFERENCE)}:4446",
            rust_const="REVIEWED_FQ12_ACCOUNT_CARRIED_MATRIX_DOMAIN",
            frame_encoding="raw_domain_row_col_counter",
            frame_shape="domain || le16(row) || le16(col) || le32(counter)",
            domain_offset=0,
            min_input_bytes=fixed_len(
                len(consts["REVIEWED_FQ12_ACCOUNT_CARRIED_MATRIX_DOMAIN"]), 2, 2, 4
            ),
            max_input_bytes=fixed_len(
                len(consts["REVIEWED_FQ12_ACCOUNT_CARRIED_MATRIX_DOMAIN"]), 2, 2, 4
            ),
            selected_route_relevance="signer_and_verifier",
        ),
        RegistryEntry(
            id="reviewed_fq12_challenge_action",
            domain=consts["REVIEWED_FQ12_ACCOUNT_CARRIED_CHALLENGE_ACTION_DOMAIN"],
            family="check",
            role="Fq12 challenge-action vector expansion",
            source=f"{rel(PRODUCTION_REFERENCE)}:4501",
            rust_const="REVIEWED_FQ12_ACCOUNT_CARRIED_CHALLENGE_ACTION_DOMAIN",
            frame_encoding="raw_domain_seed_stream_index_counter",
            frame_shape="domain || challenge[21] || le64(stream=0) || le64(index) || le32(counter)",
            domain_offset=0,
            min_input_bytes=fixed_len(
                len(consts["REVIEWED_FQ12_ACCOUNT_CARRIED_CHALLENGE_ACTION_DOMAIN"]), 21, 8, 8, 4
            ),
            max_input_bytes=fixed_len(
                len(consts["REVIEWED_FQ12_ACCOUNT_CARRIED_CHALLENGE_ACTION_DOMAIN"]), 21, 8, 8, 4
            ),
            selected_route_relevance="signer_and_verifier",
        ),
        RegistryEntry(
            id="reviewed_fq12_pad_prf",
            domain=consts["REVIEWED_FQ12_PAD_DOMAIN"],
            family="pad",
            role="state-assisted local pad PRF rejection stream",
            source=f"{rel(PRODUCTION_REFERENCE)}:11761",
            rust_const="REVIEWED_FQ12_PAD_DOMAIN",
            frame_encoding="raw_secret_prefix_fixed_pad_frame",
            frame_shape="pad_seed[32] || pad_frame[46: domain at offset 0, block byte, zero padding] || le32(counter)",
            domain_offset=PAD_SEED_BYTES,
            min_input_bytes=PAD_SEED_BYTES + PAD_FRAME_BYTES + 4,
            max_input_bytes=PAD_SEED_BYTES + PAD_FRAME_BYTES + 4,
            selected_route_relevance="signer_state_and_verifier_opening_checks",
            notes=("domain is not at byte offset zero in the full SHA-256 input",),
        ),
        RegistryEntry(
            id="reviewed_fq12_pad_opening_view",
            domain=consts["REVIEWED_FQ12_PAD_OPENING_VIEW_DOMAIN"],
            family="opening",
            role="public digest for observed pad-opening consistency",
            source=f"{rel(PRODUCTION_REFERENCE)}:10772",
            rust_const="REVIEWED_FQ12_PAD_OPENING_VIEW_DOMAIN",
            frame_encoding="append_frame_tlv",
            frame_shape="domain, profile, q, groups, local_free, repetitions, observed_bitmap, observed block pads",
            domain_offset=10,
            min_input_bytes=tlv_min(
                consts["REVIEWED_FQ12_PAD_OPENING_VIEW_DOMAIN"],
                [
                    ("profile", len(profile)),
                    ("q", 8),
                    ("groups", 8),
                    ("local_free", 8),
                    ("repetitions", 8),
                    ("observed_bitmap", 2),
                ],
            ),
            max_input_bytes=None,
            selected_route_relevance="verifier_opening_checks",
        ),
        RegistryEntry(
            id="reviewed_fq12_pad_seed_opening",
            domain=consts["REVIEWED_FQ12_PAD_SEED_OPENING_DOMAIN"],
            family="opening",
            role="pad-seed opening digest bound to the selected route profile",
            source=f"{rel(PRODUCTION_REFERENCE)}:4765",
            rust_const="REVIEWED_FQ12_PAD_SEED_OPENING_DOMAIN",
            frame_encoding="append_frame_tlv",
            frame_shape="domain, profile, q, groups, local_free, pad_seed_bytes, pad_seed",
            domain_offset=10,
            min_input_bytes=tlv_min(
                consts["REVIEWED_FQ12_PAD_SEED_OPENING_DOMAIN"],
                [
                    ("profile", len(profile)),
                    ("q", 8),
                    ("groups", 8),
                    ("local_free", 8),
                    ("pad_seed_bytes", 8),
                    ("pad_seed", PAD_SEED_BYTES),
                ],
            ),
            max_input_bytes=tlv_min(
                consts["REVIEWED_FQ12_PAD_SEED_OPENING_DOMAIN"],
                [
                    ("profile", len(profile)),
                    ("q", 8),
                    ("groups", 8),
                    ("local_free", 8),
                    ("pad_seed_bytes", 8),
                    ("pad_seed", PAD_SEED_BYTES),
                ],
            ),
            selected_route_relevance="verifier_opening_checks",
        ),
        RegistryEntry(
            id="reviewed_fq12_account_witness_projection_opening",
            domain=consts["REVIEWED_FQ12_ACCOUNT_WITNESS_PROJECTION_OPENING_DOMAIN"],
            family="opening",
            role="account witness projection opening digest",
            source=f"{rel(PRODUCTION_REFERENCE)}:1822",
            rust_const="REVIEWED_FQ12_ACCOUNT_WITNESS_PROJECTION_OPENING_DOMAIN",
            frame_encoding="append_frame_tlv",
            frame_shape="domain, profile, q, groups, local_free, projection_lanes, account_witness_opening_frame, semantic_value, expanded_opening_commitment",
            domain_offset=10,
            min_input_bytes=tlv_min(
                consts["REVIEWED_FQ12_ACCOUNT_WITNESS_PROJECTION_OPENING_DOMAIN"],
                [
                    ("profile", len(profile)),
                    ("q", 8),
                    ("groups", 8),
                    ("local_free", 8),
                    ("projection_lanes", 8),
                    ("account_witness_opening_frame", 0),
                    ("semantic_value", 0),
                    ("expanded_opening_commitment", DIGEST_BYTES),
                ],
            ),
            max_input_bytes=None,
            selected_route_relevance="verifier_opening_checks",
        ),
        RegistryEntry(
            id="reviewed_fq12_projection_mask",
            domain=consts["REVIEWED_FQ12_PACKED_WITNESS_STATE18_PROJECTION_MASK_DOMAIN"],
            family="mask",
            role="six-lane state-assisted projection mask PRF rejection stream",
            source=f"{rel(PRODUCTION_REFERENCE)}:1899",
            rust_const="REVIEWED_FQ12_PACKED_WITNESS_STATE18_PROJECTION_MASK_DOMAIN",
            frame_encoding="raw_leading_domain_seed_stream_index_counter",
            frame_shape="domain || pad_seed[32] || le64(slot_id) || le64(slot_id stream) || le64(lane) || le32(counter)",
            domain_offset=0,
            min_input_bytes=fixed_len(
                len(consts["REVIEWED_FQ12_PACKED_WITNESS_STATE18_PROJECTION_MASK_DOMAIN"]),
                PAD_SEED_BYTES,
                SLOT_ID_BYTES_U64,
                SLOT_ID_BYTES_U64,
                8,
                4,
            ),
            max_input_bytes=fixed_len(
                len(consts["REVIEWED_FQ12_PACKED_WITNESS_STATE18_PROJECTION_MASK_DOMAIN"]),
                PAD_SEED_BYTES,
                SLOT_ID_BYTES_U64,
                SLOT_ID_BYTES_U64,
                8,
                4,
            ),
            selected_route_relevance="signer_state_and_live_slot_masking",
        ),
        RegistryEntry(
            id="reviewed_fq12_masked_state18_packed_witness_opening",
            domain=consts["REVIEWED_FQ12_MASKED_STATE18_PACKED_WITNESS_OPENING_DOMAIN"],
            family="opening",
            role="masked packed-witness state18 spend-time opening digest",
            source=f"{rel(PRODUCTION_REFERENCE)}:1952",
            rust_const="REVIEWED_FQ12_MASKED_STATE18_PACKED_WITNESS_OPENING_DOMAIN",
            frame_encoding="append_frame_tlv",
            frame_shape="domain, profile, q, checked_core_rows, projection_lanes, slot_id, witness_commitment, syndrome rows, masked projection, masks, account projection, repacked_witness_commitment",
            domain_offset=10,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="verifier_opening_checks",
        ),
        RegistryEntry(
            id="reviewed_fq12_state_assisted_projection_carrier_binding",
            domain=consts["REVIEWED_FQ12_STATE_ASSISTED_PROJECTION_CARRIER_BINDING_DOMAIN"],
            family="carrier",
            role="carrier check bytes and transcript-binding derivation",
            source=f"{rel(PRODUCTION_REFERENCE)}:2266",
            rust_const="REVIEWED_FQ12_STATE_ASSISTED_PROJECTION_CARRIER_BINDING_DOMAIN",
            frame_encoding="append_frame_tlv_truncated_sha256",
            frame_shape="domain, profile, q, groups, local_free, projection_lanes, opening_commitment, carrier_handle_secret_digest, statement_digest, signature_digest, public_key_digest, message_digest, slot_id, lane records",
            domain_offset=10,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="signer_and_verifier_carrier",
            notes=(f"SHA-256 output split into {CHECK_BYTES} check bytes and {TRANSCRIPT_BINDING_BYTES} transcript-binding bytes",),
        ),
        RegistryEntry(
            id="reviewed_fq12_hidden_source_statement",
            domain=consts["REVIEWED_FQ12_HIDDEN_SOURCE_STATEMENT_DOMAIN"],
            family="transcript",
            role="hidden-source public statement digest",
            source=f"{rel(PRODUCTION_REFERENCE)}:10937",
            rust_const="REVIEWED_FQ12_HIDDEN_SOURCE_STATEMENT_DOMAIN",
            frame_encoding="append_frame_tlv",
            frame_shape="domain, profile, q, groups, local_pivot, local_free, repetitions, slot_id, commitments, public/message/signature digests, pad opening digest and bitmap",
            domain_offset=10,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="verifier_transcript",
        ),
        RegistryEntry(
            id="reviewed_fq12_hidden_source_proof_transcript",
            domain=consts["REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_DOMAIN"],
            family="transcript",
            role="hidden-source proof transcript digest",
            source=f"{rel(PRODUCTION_REFERENCE)}:11000",
            rust_const="REVIEWED_FQ12_HIDDEN_SOURCE_PROOF_DOMAIN",
            frame_encoding="append_frame_tlv",
            frame_shape="domain, profile, q, groups, local_pivot, local_free, repetitions, proof_bytes, statement_digest, proof bytes",
            domain_offset=10,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="verifier_transcript",
        ),
        RegistryEntry(
            id="reviewed_fq12_state_assisted_opening_commitment",
            domain=consts["REVIEWED_FQ12_STATE_ASSISTED_OPENING_DOMAIN"],
            family="opening",
            role="state-assisted opening commitment",
            source=f"{rel(PRODUCTION_REFERENCE)}:10810",
            rust_const="REVIEWED_FQ12_STATE_ASSISTED_OPENING_DOMAIN",
            frame_encoding="raw_leading_domain_profile_fixed",
            frame_shape="domain || profile || le64(slot_id) || six 32-byte digests",
            domain_offset=0,
            min_input_bytes=fixed_len(
                len(consts["REVIEWED_FQ12_STATE_ASSISTED_OPENING_DOMAIN"]),
                len(consts["REVIEWED_FQ12_STATE_ASSISTED_OPENING_PROFILE"]),
                SLOT_ID_BYTES_U64,
                6 * DIGEST_BYTES,
            ),
            max_input_bytes=fixed_len(
                len(consts["REVIEWED_FQ12_STATE_ASSISTED_OPENING_DOMAIN"]),
                len(consts["REVIEWED_FQ12_STATE_ASSISTED_OPENING_PROFILE"]),
                SLOT_ID_BYTES_U64,
                6 * DIGEST_BYTES,
            ),
            selected_route_relevance="signer_state_and_verifier_opening_checks",
            notes=("raw profile concatenation has no length tag but fixed constants and trailing fields",),
        ),
        RegistryEntry(
            id="reviewed_fq12_state_assisted_carrier_handle",
            domain=consts["REVIEWED_FQ12_STATE_ASSISTED_CARRIER_DOMAIN"],
            family="carrier",
            role="carrier handle commitment checked by hidden-source proof bytes",
            source=f"{rel(PRODUCTION_REFERENCE)}:10839",
            rust_const="REVIEWED_FQ12_STATE_ASSISTED_CARRIER_DOMAIN",
            frame_encoding="raw_leading_domain_fixed",
            frame_shape="domain || five 32-byte digests || le64(slot_id) || carrier_handle_secret_digest",
            domain_offset=0,
            min_input_bytes=fixed_len(
                len(consts["REVIEWED_FQ12_STATE_ASSISTED_CARRIER_DOMAIN"]),
                5 * DIGEST_BYTES,
                SLOT_ID_BYTES_U64,
                DIGEST_BYTES,
            ),
            max_input_bytes=fixed_len(
                len(consts["REVIEWED_FQ12_STATE_ASSISTED_CARRIER_DOMAIN"]),
                5 * DIGEST_BYTES,
                SLOT_ID_BYTES_U64,
                DIGEST_BYTES,
            ),
            selected_route_relevance="signer_and_verifier_carrier",
        ),
        RegistryEntry(
            id="reviewed_fq12_state_assisted_ro_pok_transcript",
            domain=consts["REVIEWED_FQ12_STATE_ASSISTED_RO_POK_DOMAIN"],
            family="transcript",
            role="state-assisted random-oracle proof-of-knowledge transcript",
            source=f"{rel(PRODUCTION_REFERENCE)}:10886",
            rust_const="REVIEWED_FQ12_STATE_ASSISTED_RO_POK_DOMAIN",
            frame_encoding="raw_leading_domain_profile_len_frame",
            frame_shape="domain || profile || le32(statement_frame_len) || statement_frame || opening_commitment || proof_transcript_digest || carrier",
            domain_offset=0,
            min_input_bytes=fixed_len(
                len(consts["REVIEWED_FQ12_STATE_ASSISTED_RO_POK_DOMAIN"]),
                len(consts["REVIEWED_FQ12_STATE_ASSISTED_RO_POK_PROFILE"]),
                4,
                0,
                3 * DIGEST_BYTES,
            ),
            max_input_bytes=None,
            selected_route_relevance="verifier_transcript",
        ),
        RegistryEntry(
            id="production_deployment_preflight_report",
            domain=consts["PRODUCTION_DEPLOYMENT_PREFLIGHT_REPORT_DOMAIN"],
            family="preflight",
            role="deployment preflight report digest",
            source=f"{rel(PRODUCTION_REFERENCE)}:11394",
            rust_const="PRODUCTION_DEPLOYMENT_PREFLIGHT_REPORT_DOMAIN",
            frame_encoding="append_frame_tlv",
            frame_shape="domain, profile, parameters, implementation booleans, blockers",
            domain_offset=10,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="preflight_only",
        ),
        RegistryEntry(
            id="production_deployment_preflight_verdict",
            domain=consts["PRODUCTION_DEPLOYMENT_PREFLIGHT_VERDICT_DOMAIN"],
            family="preflight",
            role="deployment preflight verdict digest",
            source=f"{rel(PRODUCTION_REFERENCE)}:11585",
            rust_const="PRODUCTION_DEPLOYMENT_PREFLIGHT_VERDICT_DOMAIN",
            frame_encoding="append_frame_tlv",
            frame_shape="domain, profile, report digests, verdict, deployment_ready, blocker_count, primary_blocker",
            domain_offset=10,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="preflight_only",
        ),
        RegistryEntry(
            id="production_deployment_preflight_blocker_report",
            domain=consts["PRODUCTION_DEPLOYMENT_PREFLIGHT_BLOCKER_REPORT_DOMAIN"],
            family="preflight",
            role="deployment preflight blocker report digest",
            source=f"{rel(PRODUCTION_REFERENCE)}:11625",
            rust_const="PRODUCTION_DEPLOYMENT_PREFLIGHT_BLOCKER_REPORT_DOMAIN",
            frame_encoding="append_frame_tlv",
            frame_shape="domain, profile, preflight_report_digest, blocker_count, blocker digests",
            domain_offset=10,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="preflight_only",
        ),
        RegistryEntry(
            id="production_deployment_preflight_ready_assertion",
            domain=consts["PRODUCTION_DEPLOYMENT_PREFLIGHT_READY_ASSERTION_DOMAIN"],
            family="preflight",
            role="deployment preflight ready assertion digest",
            source=f"{rel(PRODUCTION_REFERENCE)}:11657",
            rust_const="PRODUCTION_DEPLOYMENT_PREFLIGHT_READY_ASSERTION_DOMAIN",
            frame_encoding="append_frame_tlv",
            frame_shape="domain, profile, preflight report/verdict/blocker digests, assertion_passed, blocker",
            domain_offset=10,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="preflight_only",
        ),
        RegistryEntry(
            id="production_deployment_preflight_certificate",
            domain=consts["PRODUCTION_DEPLOYMENT_PREFLIGHT_CERTIFICATE_DOMAIN"],
            family="preflight",
            role="deployment preflight certificate digest",
            source=f"{rel(PRODUCTION_REFERENCE)}:11692",
            rust_const="PRODUCTION_DEPLOYMENT_PREFLIGHT_CERTIFICATE_DOMAIN",
            frame_encoding="append_frame_tlv",
            frame_shape="domain, profile, report/verdict/assertion digests, deployment_ready, blocker_count, primary_blocker",
            domain_offset=10,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="preflight_only",
        ),
        RegistryEntry(
            id="production_deployment_preflight_blocker",
            domain=consts["PRODUCTION_DEPLOYMENT_PREFLIGHT_BLOCKER_DOMAIN"],
            family="preflight",
            role="individual deployment preflight blocker digest",
            source=f"{rel(PRODUCTION_REFERENCE)}:11741",
            rust_const="PRODUCTION_DEPLOYMENT_PREFLIGHT_BLOCKER_DOMAIN",
            frame_encoding="append_frame_tlv",
            frame_shape="domain, profile, preflight_report_digest, index, blocker",
            domain_offset=10,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="preflight_only",
        ),
        RegistryEntry(
            id="path2_prevalidated_slot_leaf_v1",
            domain="DESCENDER_SLOT_LEAF_V1",
            family="path2_account_creation",
            role="canonical prevalidated slot leaf frame",
            source="research/prevalidated_slot_leaf_v1_frame.md",
            rust_const=None,
            frame_encoding="raw_leading_domain_fixed",
            frame_shape="domain || le16(frame_version=1) || ten 32-byte digests/identities || le64(slot_id) || live_status_marker",
            domain_offset=0,
            min_input_bytes=353,
            max_input_bytes=353,
            selected_route_relevance="path2_planning_reserved",
            notes=("reserved planning domain; Rust constant not implemented yet",),
        ),
        RegistryEntry(
            id="path2_slot_authorization_digest_v1",
            domain="DESCENDER_SLOT_AUTHORIZATION_V1",
            family="path2_account_creation",
            role="normalized slot authorization digest bound by checked-core spend frame",
            source="research/prevalidated_slot_leaf_v1_frame.md",
            rust_const=None,
            frame_encoding="raw_leading_domain_fixed",
            frame_shape="domain || PrevalidatedSlotLeafV1_bytes",
            domain_offset=0,
            min_input_bytes=31 + 353,
            max_input_bytes=31 + 353,
            selected_route_relevance="path2_planning_reserved",
            notes=("reserved planning domain; Rust constant not implemented yet",),
        ),
        RegistryEntry(
            id="path2_slot_prevalidate_single_v1",
            domain="DESCENDER_SLOT_PREVALIDATE_SINGLE_V1",
            family="path2_account_creation",
            role="direct single-slot account-creation evidence transcript",
            source="research/account_creation_evidence_v1_spec.md",
            rust_const=None,
            frame_encoding="raw_leading_domain_variable",
            frame_shape="domain || evidence_version || evidence_kind || leaf/auth/evidence binding digests || concrete evidence payload",
            domain_offset=0,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="path2_planning_reserved",
            notes=("reserved planning domain; concrete evidence payload not implemented yet",),
        ),
        RegistryEntry(
            id="path2_slot_prevalidate_batch_v1",
            domain="DESCENDER_SLOT_PREVALIDATE_BATCH_V1",
            family="path2_optional_aggregation",
            role="optional batch slot-admission transcript",
            source="research/account_creation_aggregation_readiness_contract.md",
            rust_const=None,
            frame_encoding="raw_leading_domain_variable",
            frame_shape="domain || ordered leaf digests || batch proof/root update payload",
            domain_offset=0,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="optional_aggregation_reserved",
            notes=("inactive unless optional aggregation mode is enabled",),
        ),
        RegistryEntry(
            id="path2_slot_root_update_v1",
            domain="DESCENDER_SLOT_ROOT_UPDATE_V1",
            family="path2_optional_aggregation",
            role="optional account slot-root update transcript",
            source="research/account_creation_aggregation_readiness_contract.md",
            rust_const=None,
            frame_encoding="raw_leading_domain_variable",
            frame_shape="domain || old_root || new_root || ordered leaf digest list || update metadata",
            domain_offset=0,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="optional_aggregation_reserved",
            notes=("inactive unless accumulator/root mode is enabled",),
        ),
        RegistryEntry(
            id="path2_slot_consume_single_v1",
            domain="DESCENDER_SLOT_CONSUME_SINGLE_V1",
            family="path2_account_creation",
            role="direct single-slot consume authorization transcript",
            source="research/slot_authorization_handle_v1_spec.md",
            rust_const=None,
            frame_encoding="raw_leading_domain_variable",
            frame_shape="domain || slot_authorization_digest || account state root || checked-core message binding context",
            domain_offset=0,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="path2_planning_reserved",
            notes=("reserved planning domain; checked-core frame binding not implemented yet",),
        ),
        RegistryEntry(
            id="path2_slot_consume_batch_v1",
            domain="DESCENDER_SLOT_CONSUME_BATCH_V1",
            family="path2_optional_aggregation",
            role="optional batch consume-once transcript",
            source="research/account_creation_aggregation_readiness_contract.md",
            rust_const=None,
            frame_encoding="raw_leading_domain_variable",
            frame_shape="domain || ordered consumed authorization digests || batch proof/root update payload",
            domain_offset=0,
            min_input_bytes=None,
            max_input_bytes=None,
            selected_route_relevance="optional_aggregation_reserved",
            notes=("inactive unless optional aggregation mode is enabled",),
        ),
    ]

    return entries


def entry_to_dict(entry: RegistryEntry) -> dict[str, Any]:
    return {
        "id": entry.id,
        "domain": entry.domain,
        "domain_len": len(entry.domain),
        "family": entry.family,
        "role": entry.role,
        "source": entry.source,
        "rust_const": entry.rust_const,
        "frame_encoding": entry.frame_encoding,
        "frame_shape": entry.frame_shape,
        "domain_offset": entry.domain_offset,
        "min_input_bytes": entry.min_input_bytes,
        "max_input_bytes": entry.max_input_bytes,
        "selected_route_relevance": entry.selected_route_relevance,
        "notes": list(entry.notes),
    }


def duplicate_domains(entries: list[RegistryEntry]) -> list[dict[str, Any]]:
    by_domain: dict[str, list[RegistryEntry]] = defaultdict(list)
    for entry in entries:
        by_domain[entry.domain].append(entry)
    return [
        {
            "domain": domain,
            "domain_len": len(domain),
            "registry_ids": [entry.id for entry in items],
            "families": sorted({entry.family for entry in items}),
        }
        for domain, items in sorted(by_domain.items())
        if len(items) > 1
    ]


def prefix_relations(entries: list[RegistryEntry]) -> list[dict[str, str]]:
    out = []
    for lhs in entries:
        for rhs in entries:
            if lhs.id == rhs.id:
                continue
            if rhs.domain.startswith(lhs.domain):
                out.append(
                    {
                        "prefix_domain": lhs.domain,
                        "prefix_id": lhs.id,
                        "longer_domain": rhs.domain,
                        "longer_id": rhs.id,
                    }
                )
    return out


def ambiguity_risks(entries: list[RegistryEntry]) -> list[dict[str, Any]]:
    risks: list[dict[str, Any]] = []
    if prefix_relations(entries):
        risks.append(
            {
                "id": "domain_string_prefix_relation",
                "severity": "review_required",
                "detail": "At least one registered domain is a prefix of another registered domain.",
            }
        )
    pad = next(entry for entry in entries if entry.id == "reviewed_fq12_pad_prf")
    risks.append(
        {
            "id": "pad_prf_secret_prefix_domain_offset",
            "severity": "open_global_disjointness_obligation",
            "registry_id": pad.id,
            "detail": (
                "The pad PRF full SHA-256 input starts with the secret pad seed, not the domain. "
                "The existing pad-vs-mask certificate handles the selected mask length relation, "
                "but a global proof must account for this non-leading domain layout."
            ),
        }
    )
    for entry in entries:
        if entry.frame_encoding.startswith("raw_leading_domain") and "profile" in entry.frame_shape:
            risks.append(
                {
                    "id": f"{entry.id}_raw_profile_concat",
                    "severity": "bounded_by_fixed_constants",
                    "registry_id": entry.id,
                    "detail": "Raw domain/profile concatenation uses fixed constants, but the global proof should record exact lengths.",
                }
            )
    for entry in entries:
        if entry.max_input_bytes is None:
            risks.append(
                {
                    "id": f"{entry.id}_variable_length_frame",
                    "severity": "needs_shape_proof",
                    "registry_id": entry.id,
                    "detail": "Frame has variable-length data; TLV or explicit length fields must be part of the disjointness proof.",
                }
            )
    return risks


def source_duplicate_report(
    occurrences: dict[str, list[str]], entries: list[RegistryEntry]
) -> list[dict[str, Any]]:
    registered = {entry.domain for entry in entries}
    return [
        {
            "domain": domain,
            "registered": domain in registered,
            "occurrence_count": len(locs),
            "sample_locations": locs[:8],
        }
        for domain, locs in occurrences.items()
        if len(locs) > 1
    ]


def classify_discovered_string(value: str, entries: list[RegistryEntry]) -> str:
    by_domain = {entry.domain: entry for entry in entries}
    if value in by_domain:
        return f"registered_{by_domain[value].family}_domain"
    if value.startswith("Descender.pre_signature_projection_"):
        return "artifact_schema"
    if value.startswith("Descender.Signature.q12289.local_delta.production_"):
        return "production_non_registered_or_schema"
    if value.startswith("Descender.Signature.q12289.local_delta.reviewed_fq12_"):
        return "reviewed_fq12_non_registered_or_profile"
    if value.startswith("Descender.hidden_source.fallback_projection."):
        return "archived_or_fallback_projection_domain"
    if value.startswith("Descender.hidden_source."):
        return "hidden_source_domain_or_profile"
    if value.startswith("Descender.Signature.q12289.local_delta."):
        return "signature_domain_or_profile"
    return "descender_auxiliary_string"


def discovered_string_report(
    occurrences: dict[str, list[str]], entries: list[RegistryEntry]
) -> list[dict[str, Any]]:
    return [
        {
            "value": value,
            "classification": classify_discovered_string(value, entries),
            "registered": any(value == entry.domain for entry in entries),
            "occurrence_count": len(locs),
            "sample_locations": locs[:8],
        }
        for value, locs in occurrences.items()
    ]


def build_payload() -> dict[str, Any]:
    source = PRODUCTION_REFERENCE.read_text()
    consts = rust_string_consts(source)
    entries = build_registry(consts)
    composition = load_json(COMPOSITION_JSON)
    frame = load_json(FRAME_JSON)
    ro_reduction = load_json(RO_REDUCTION_JSON)

    scan_paths = [
        PRODUCTION_REFERENCE,
        ROOT / "tools/production-reference/src/lib.rs",
        *sorted((ROOT / "scripts/research").glob("pre_signature_projection_*.py")),
        *sorted(READINESS_DIR.glob("pre_signature_projection_*.json")),
        *sorted(READINESS_DIR.glob("pre_signature_projection_*.md")),
    ]
    occurrences = descender_string_occurrences(scan_paths)
    duplicate_registry_domains = duplicate_domains(entries)
    prefix = prefix_relations(entries)
    risks = ambiguity_risks(entries)

    families: dict[str, list[str]] = defaultdict(list)
    for entry in entries:
        families[entry.family].append(entry.id)

    remaining_before_global_disjointness = [
        "Prove exact byte-frame disjointness, not just domain-string uniqueness, across raw, raw-with-secret-prefix, raw-with-profile, and append_frame TLV encodings.",
        "Include variable-length transcript/opening/preflight frames in the proof by field labels, explicit lengths, and accepted maximums where applicable.",
        "Tie this registry to executable negative tests that mutate every registered domain and reject swapped-frame attempts across signer/verifier entry points.",
        "Decide whether non-selected archived fallback domains are outside scope or must be carried in a separate all-production registry.",
        "Keep A_sha256_projection_mask_frame_prf separate: this registry only addresses byte-frame/domain disjointness, not SHA-256 PRF security.",
    ]

    return {
        "schema": "Descender.pre_signature_projection_global_domain_registry.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "global_domain_registry_concrete_fail_closed",
        "deployment_ready": False,
        "closes_A_global_domain_registry": False,
        "target": {
            "n": N,
            "q": Q,
            "selected_signature_bytes": SELECTED_SIGNATURE_BYTES,
            "state_path": "308-byte/Fq12 state-assisted masked projection-carrier path",
            "mask_lanes": MASK_LANES,
            "carrier_lanes": CARRIER_LANES,
        },
        "carried_term": {
            "id": "A_global_domain_registry",
            "term": "Adv_frame_disjointness(q_hash)",
            "source": rel(COMPOSITION_JSON),
            "composition_status": composition.get("status"),
            "ro_reduction_status": ro_reduction.get("status"),
        },
        "source_artifacts": {
            "composition": rel(COMPOSITION_JSON),
            "frame_disjointness": rel(FRAME_JSON),
            "ro_reduction": rel(RO_REDUCTION_JSON),
            "production_reference": rel(PRODUCTION_REFERENCE),
        },
        "upstream_pad_vs_mask_certificate": {
            "status": frame.get("status"),
            "pad_domain": frame.get("pad_domain"),
            "mask_domain": frame.get("mask_domain"),
            "pad_prf_input_len": frame.get("pad_prf_input_len"),
            "mask_prf_input_len": frame.get("mask_prf_input_len"),
            "bounded_byte_equal_collisions": len(frame.get("bounded_byte_equal_collisions", [])),
            "bounded_disjoint": frame.get("bounded_disjoint"),
        },
        "registry_entries": [entry_to_dict(entry) for entry in entries],
        "family_index": {family: sorted(ids) for family, ids in sorted(families.items())},
        "domain_string_uniqueness": {
            "registered_domain_count": len(entries),
            "unique_registered_domain_count": len({entry.domain for entry in entries}),
            "exact_duplicate_registered_domains": duplicate_registry_domains,
            "prefix_relations": prefix,
            "all_registered_domains_unique": not duplicate_registry_domains,
            "no_registered_domain_is_prefix_of_another": not prefix,
        },
        "source_scan": {
            "paths_scanned": [rel(path) for path in scan_paths if path.exists()],
            "descender_string_count": len(occurrences),
            "discovered_descender_strings": discovered_string_report(occurrences, entries),
            "duplicate_descender_strings_in_sources": source_duplicate_report(occurrences, entries),
        },
        "byte_frame_ambiguity_risks": risks,
        "remaining_before_global_frame_disjointness_claim": remaining_before_global_disjointness,
        "decision": {
            "can_claim_global_frame_disjointness_now": False,
            "can_consume_as_registry_artifact": True,
            "reason": (
                "The selected signer/verifier frame families are concretely registered and exact "
                "registered domain strings are unique. The carried Adv_frame_disjointness term "
                "remains open until byte-level disjointness is proved across all listed frame "
                "encodings and covered by executable negative tests."
            ),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection Global Domain Registry",
        "",
        "This artifact carries term 2, `A_global_domain_registry`, for the selected",
        "308-byte/Fq12 state-assisted masked projection-carrier path.",
        "",
        "## Status",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- closes `A_global_domain_registry`: `{payload['closes_A_global_domain_registry']}`",
        f"- registered domains: `{payload['domain_string_uniqueness']['registered_domain_count']}`",
        f"- discovered Descender strings in scanned scripts/Rust/artifacts: `{payload['source_scan']['descender_string_count']}`",
        f"- exact duplicate registered domain strings: `{len(payload['domain_string_uniqueness']['exact_duplicate_registered_domains'])}`",
        f"- registered domain prefix relations: `{len(payload['domain_string_uniqueness']['prefix_relations'])}`",
        f"- byte-frame ambiguity risks: `{len(payload['byte_frame_ambiguity_risks'])}`",
        "",
        "## Registered Families",
        "",
        "| family | entries |",
        "|---|---:|",
    ]
    for family, ids in payload["family_index"].items():
        lines.append(f"| `{family}` | {len(ids)} |")

    lines.extend(
        [
            "",
            "## Registry",
            "",
            "| id | family | domain length | frame encoding | source |",
            "|---|---|---:|---|---|",
        ]
    )
    for entry in payload["registry_entries"]:
        lines.append(
            f"| `{entry['id']}` | `{entry['family']}` | {entry['domain_len']} | `{entry['frame_encoding']}` | `{entry['source']}` |"
        )

    lines.extend(
        [
            "",
            "## Duplicate And Prefix Checks",
            "",
            f"- all registered domains unique: `{payload['domain_string_uniqueness']['all_registered_domains_unique']}`",
            f"- no registered domain is a prefix of another: `{payload['domain_string_uniqueness']['no_registered_domain_is_prefix_of_another']}`",
            f"- duplicate Descender strings in scanned sources: `{len(payload['source_scan']['duplicate_descender_strings_in_sources'])}`",
            "",
            "Registered duplicate domains:",
            "",
        ]
    )
    if payload["domain_string_uniqueness"]["exact_duplicate_registered_domains"]:
        for item in payload["domain_string_uniqueness"]["exact_duplicate_registered_domains"]:
            lines.append(f"- `{item['domain']}`: {', '.join(item['registry_ids'])}")
    else:
        lines.append("- none")

    lines.extend(["", "Registered prefix relations:", ""])
    if payload["domain_string_uniqueness"]["prefix_relations"]:
        for item in payload["domain_string_uniqueness"]["prefix_relations"]:
            lines.append(f"- `{item['prefix_id']}` prefixes `{item['longer_id']}`")
    else:
        lines.append("- none")

    lines.extend(
        [
            "",
            "## Byte-Frame Ambiguity Risks",
            "",
            "| risk | severity | registry id | detail |",
            "|---|---|---|---|",
        ]
    )
    for risk in payload["byte_frame_ambiguity_risks"]:
        lines.append(
            f"| `{risk['id']}` | `{risk['severity']}` | `{risk.get('registry_id', '')}` | {risk['detail']} |"
        )

    lines.extend(["", "## Remains Before Global Frame Disjointness", ""])
    lines.extend(f"- {item}" for item in payload["remaining_before_global_frame_disjointness_claim"])

    lines.extend(
        [
            "",
            "## Decision",
            "",
            payload["decision"]["reason"],
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload) + "\n")
    print(
        json.dumps(
            {
                "json": str(DEFAULT_JSON),
                "markdown": str(DEFAULT_MD),
                "status": payload["status"],
                "closes_A_global_domain_registry": payload["closes_A_global_domain_registry"],
            "registered_domains": payload["domain_string_uniqueness"][
                "registered_domain_count"
            ],
            "discovered_descender_strings": payload["source_scan"]["descender_string_count"],
            "exact_duplicate_registered_domains": len(
                payload["domain_string_uniqueness"]["exact_duplicate_registered_domains"]
            ),
                "prefix_relations": len(
                    payload["domain_string_uniqueness"]["prefix_relations"]
                ),
                "byte_frame_ambiguity_risks": len(payload["byte_frame_ambiguity_risks"]),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
