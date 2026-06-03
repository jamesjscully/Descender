#!/usr/bin/env python3
"""PTC-008 rogue-key/account-binding fixture analysis.

This is a deterministic adversarial fixture harness for reviewer-style account,
key, and commitment binding attacks.  It is not a proof object and does not
claim deployment readiness.  Its purpose is to make degenerate accepted states
explicit: if a mutation is accepted only after weakening a boundary check, the
case is classified as an extractor break, binding break, or both.
"""

from __future__ import annotations

import copy
import hashlib
import json
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = (
    READINESS_DIR / "pre_third_party_rogue_key_account_fixture_analysis_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "pre_third_party_rogue_key_account_fixture_analysis_n32_q12289.md"
)
ACCOUNT_SCHEMA_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_account_state_schema_contract_n32_q12289.json"
)
ACCOUNT_WIRING_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_account_state_opening_commitment_wiring_n32_q12289.json"
)
EXPANDED_BINDING_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_expanded_opening_commitment_binding_n32_q12289.json"
)
TRANSCRIPT_MALLEABILITY_JSON = (
    READINESS_DIR / "pre_third_party_transcript_malleability_probe_n32_q12289.json"
)

Q = 12289
PROFILE = "n32.q12289.fq12.state.v1"
STATE_DOMAIN = "Descender.pre_third_party.rogue_key_account.state.v1"
OPENING_DOMAIN = "Descender.pre_third_party.rogue_key_account.opening.v1"
PK_BINDING_DOMAIN = "Descender.pre_third_party.rogue_key_account.pk_binding.v1"
TRANSCRIPT_DOMAIN = "Descender.pre_third_party.rogue_key_account.transcript.v1"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text()) if path.exists() else {}


def sha256(data: bytes) -> bytes:
    return hashlib.sha256(data).digest()


def synthetic_bytes(label: str, length: int) -> bytes:
    output = b""
    counter = 0
    while len(output) < length:
        output += sha256(f"Descender.ptc008.rogue_key_account:{label}:{counter}".encode())
        counter += 1
    return output[:length]


def u16(value: int) -> bytes:
    return value.to_bytes(2, "little")


def u32(value: int) -> bytes:
    return value.to_bytes(4, "little")


def u64(value: int) -> bytes:
    return value.to_bytes(8, "little")


def framed(field_id: str, value: bytes) -> bytes:
    encoded_id = field_id.encode()
    return u16(len(encoded_id)) + encoded_id + u32(len(value)) + value


def is_hex32(value: str) -> bool:
    try:
        return len(bytes.fromhex(value)) == 32
    except ValueError:
        return False


def digest_list(values: list[str]) -> bytes:
    return b"".join(bytes.fromhex(value) for value in values)


def encode_source_values(values: list[int]) -> bytes:
    return b"".join(u16(value) for value in values)


def opening_digest(fixture: dict[str, Any]) -> str:
    account = fixture["account_state"]
    commitments = fixture["commitments"]
    parts = [
        OPENING_DOMAIN.encode(),
        PROFILE.encode(),
        u64(account["slot_id"]),
        digest_list(
            [
                commitments["witness_commitment"],
                commitments["randomizer_commitment"],
                commitments["restore_anchor_digest"],
                commitments["carrier_handle_commitment"],
            ]
        ),
        encode_source_values(fixture["opening"]["source_values"]),
        digest_list(
            [
                fixture["opening"]["witness_opening_digest"],
                fixture["opening"]["randomizer_opening_digest"],
            ]
        ),
    ]
    return sha256(b"".join(parts)).hex()


def account_state_commitment(fixture: dict[str, Any]) -> str:
    account = fixture["account_state"]
    commitments = fixture["commitments"]
    parts = [
        STATE_DOMAIN.encode(),
        PROFILE.encode(),
        bytes.fromhex(account["user_id"]),
        bytes.fromhex(account["account_id"]),
        bytes.fromhex(account["public_key_digest"]),
        u64(account["slot_id"]),
        bytes.fromhex(fixture["opening_digest"]),
        digest_list(
            [
                commitments["witness_commitment"],
                commitments["randomizer_commitment"],
                commitments["restore_anchor_digest"],
                commitments["carrier_handle_commitment"],
            ]
        ),
    ]
    return sha256(b"".join(parts)).hex()


def public_key_binding_commitment(fixture: dict[str, Any]) -> str:
    account = fixture["account_state"]
    parts = [
        PK_BINDING_DOMAIN.encode(),
        PROFILE.encode(),
        bytes.fromhex(account["user_id"]),
        bytes.fromhex(account["account_id"]),
        bytes.fromhex(account["public_key_digest"]),
        bytes.fromhex(fixture["account_state_commitment"]),
    ]
    return sha256(b"".join(parts)).hex()


def transcript_digest(fixture: dict[str, Any]) -> str:
    account = fixture["account_state"]
    payload = b"".join(
        [
            framed("domain", TRANSCRIPT_DOMAIN.encode()),
            framed("profile", PROFILE.encode()),
            framed("user_id", bytes.fromhex(account["user_id"])),
            framed("account_id", bytes.fromhex(account["account_id"])),
            framed("public_key_digest", bytes.fromhex(account["public_key_digest"])),
            framed("account_state_commitment", bytes.fromhex(fixture["account_state_commitment"])),
            framed("pk_binding_commitment", bytes.fromhex(fixture["pk_binding_commitment"])),
            framed("message_digest", bytes.fromhex(fixture["message_digest"])),
            framed("signature_digest", bytes.fromhex(fixture["signature_digest"])),
        ]
    )
    return sha256(payload).hex()


def recompute_all(fixture: dict[str, Any]) -> None:
    fixture["opening_digest"] = opening_digest(fixture)
    fixture["account_state_commitment"] = account_state_commitment(fixture)
    fixture["pk_binding_commitment"] = public_key_binding_commitment(fixture)
    fixture["transcript_digest"] = transcript_digest(fixture)


def make_reference(label: str, slot_id: int) -> dict[str, Any]:
    fixture = {
        "fixture_id": label,
        "account_state": {
            "user_id": synthetic_bytes(f"{label}:user_id", 16).hex(),
            "account_id": synthetic_bytes(f"{label}:account_id", 16).hex(),
            "public_key_digest": synthetic_bytes(f"{label}:public_key_digest", 32).hex(),
            "slot_id": slot_id,
        },
        "commitments": {
            "witness_commitment": synthetic_bytes(f"{label}:witness_commitment", 32).hex(),
            "randomizer_commitment": synthetic_bytes(f"{label}:randomizer_commitment", 32).hex(),
            "restore_anchor_digest": synthetic_bytes(f"{label}:restore_anchor", 32).hex(),
            "carrier_handle_commitment": synthetic_bytes(f"{label}:carrier_handle", 32).hex(),
        },
        "opening": {
            "source_values": [
                int.from_bytes(synthetic_bytes(f"{label}:source:{idx}", 2), "little") % Q
                for idx in range(6)
            ],
            "witness_opening_digest": synthetic_bytes(f"{label}:witness_opening", 32).hex(),
            "randomizer_opening_digest": synthetic_bytes(f"{label}:randomizer_opening", 32).hex(),
        },
        "message_digest": synthetic_bytes(f"{label}:message", 32).hex(),
        "signature_digest": synthetic_bytes(f"{label}:signature", 32).hex(),
    }
    recompute_all(fixture)
    return fixture


@dataclass(frozen=True)
class Policy:
    require_unique_commitments: bool = True
    reject_identity_like_values: bool = True
    bind_public_key_to_account: bool = True
    bind_user_to_account: bool = True
    require_canonical_opening: bool = True
    require_fresh_account_commitment: bool = True


STRICT_POLICY = Policy()
WEAK_POLICY = Policy(
    require_unique_commitments=False,
    reject_identity_like_values=False,
    bind_public_key_to_account=False,
    bind_user_to_account=False,
    require_canonical_opening=False,
    require_fresh_account_commitment=False,
)


def parse_errors(fixture: dict[str, Any], policy: Policy) -> list[str]:
    errors: list[str] = []
    account = fixture["account_state"]
    for field in ["user_id", "account_id"]:
        try:
            if len(bytes.fromhex(account[field])) != 16:
                errors.append(f"account_state.{field}_length")
        except ValueError:
            errors.append(f"account_state.{field}_not_hex")
    for field in [
        "public_key_digest",
        "opening_digest",
        "account_state_commitment",
        "pk_binding_commitment",
        "message_digest",
        "signature_digest",
        "transcript_digest",
    ]:
        target = account[field] if field == "public_key_digest" else fixture[field]
        if not is_hex32(target):
            errors.append(f"{field}_not_32_byte_hex")
    for field, value in fixture["commitments"].items():
        if not is_hex32(value):
            errors.append(f"commitments.{field}_not_32_byte_hex")
    for field in ["witness_opening_digest", "randomizer_opening_digest"]:
        if not is_hex32(fixture["opening"][field]):
            errors.append(f"opening.{field}_not_32_byte_hex")
    if not isinstance(account["slot_id"], int) or account["slot_id"] < 0:
        errors.append("account_state.slot_id_not_canonical_u64")
    values = fixture["opening"].get("source_values", [])
    if policy.require_canonical_opening and (
        not isinstance(values, list)
        or len(values) != 6
        or any((not isinstance(value, int)) or value < 0 or value >= Q for value in values)
    ):
        errors.append("opening.source_values_not_six_canonical_q12289_values")
    return errors


def boundary_errors(fixture: dict[str, Any], policy: Policy, references: list[dict[str, Any]]) -> list[str]:
    errors: list[str] = []
    account = fixture["account_state"]
    commitments = fixture["commitments"]
    commitment_values = list(commitments.values())
    zero32 = "00" * 32

    if policy.require_unique_commitments and len(set(commitment_values)) != len(commitment_values):
        errors.append("duplicate_commitment_digest")
    if policy.reject_identity_like_values:
        if account["public_key_digest"] == zero32:
            errors.append("identity_like_public_key_digest")
        for field, value in commitments.items():
            if value == zero32:
                errors.append(f"identity_like_commitments.{field}")
        if all(value == 0 for value in fixture["opening"].get("source_values", [])):
            errors.append("identity_like_zero_opening_source_values")
    if fixture["opening_digest"] != opening_digest(fixture):
        errors.append("opening_digest_mismatch")
    if fixture["account_state_commitment"] != account_state_commitment(fixture):
        errors.append("account_state_commitment_mismatch")
    if policy.bind_public_key_to_account and fixture["pk_binding_commitment"] != public_key_binding_commitment(fixture):
        errors.append("public_key_binding_commitment_mismatch")
    if fixture["transcript_digest"] != transcript_digest(fixture):
        errors.append("whole_account_transcript_digest_mismatch")
    if policy.bind_user_to_account:
        for reference in references:
            same_registered_account = (
                account["user_id"] == reference["account_state"]["user_id"]
                and account["account_id"] == reference["account_state"]["account_id"]
            )
            different_public_key = (
                account["public_key_digest"]
                != reference["account_state"]["public_key_digest"]
            )
            if same_registered_account and different_public_key:
                errors.append("public_key_substitution_against_registered_account")
            same_account = account["account_id"] == reference["account_state"]["account_id"]
            different_user = account["user_id"] != reference["account_state"]["user_id"]
            if same_account and different_user:
                errors.append("cross_user_account_id_replay")
    if policy.require_fresh_account_commitment:
        for reference in references:
            same_commitment = fixture["account_state_commitment"] == reference["account_state_commitment"]
            different_identity = (
                account["user_id"],
                account["account_id"],
                account["public_key_digest"],
            ) != (
                reference["account_state"]["user_id"],
                reference["account_state"]["account_id"],
                reference["account_state"]["public_key_digest"],
            )
            if same_commitment and different_identity:
                errors.append("account_state_commitment_replay_across_identity")
    return errors


def verify_fixture(
    fixture: dict[str, Any], policy: Policy, references: list[dict[str, Any]]
) -> dict[str, Any]:
    parsing = parse_errors(fixture, policy)
    binding = [] if parsing else boundary_errors(fixture, policy, references)
    accepted = not parsing and not binding
    return {
        "accepted": accepted,
        "parse_errors": parsing,
        "boundary_errors": binding,
    }


@dataclass(frozen=True)
class FixtureMutation:
    mutation_id: str
    attack_surface: str
    operation: str
    expected_break_classification: list[str]
    feasible_reduction: str
    mutate: Callable[[dict[str, Any], dict[str, Any]], None]


def recomputed(mutator: Callable[[dict[str, Any], dict[str, Any]], None]) -> Callable[[dict[str, Any], dict[str, Any]], None]:
    def inner(candidate: dict[str, Any], donor: dict[str, Any]) -> None:
        mutator(candidate, donor)
        recompute_all(candidate)

    return inner


def stale(mutator: Callable[[dict[str, Any], dict[str, Any]], None]) -> Callable[[dict[str, Any], dict[str, Any]], None]:
    return mutator


def build_mutations() -> list[FixtureMutation]:
    zero32 = "00" * 32
    return [
        FixtureMutation(
            "duplicate_witness_randomizer_commitments",
            "duplicate_commitments",
            "alias randomizer commitment to witness commitment and recompute dependent digests",
            ["binding_break"],
            "accepted duplicate commitments would make two logical openings share one digest; extraction cannot distinguish the witness/randomizer binding claim",
            recomputed(lambda c, _: c["commitments"].update({"randomizer_commitment": c["commitments"]["witness_commitment"]})),
        ),
        FixtureMutation(
            "zero_witness_commitment",
            "identity_like_state",
            "set witness commitment to all-zero digest and recompute dependent digests",
            ["extractor_break"],
            "an identity-like witness commitment makes the account opening relation vacuous unless the extractor obtains nonzero witness entropy elsewhere",
            recomputed(lambda c, _: c["commitments"].update({"witness_commitment": zero32})),
        ),
        FixtureMutation(
            "zero_public_key_digest",
            "rogue_public_key",
            "set public-key digest to all-zero identity-like digest and recompute bindings",
            ["extractor_break", "binding_break"],
            "acceptance would let a malicious registrant choose a key outside the reviewed keygen image and bypass account/key binding",
            recomputed(lambda c, _: c["account_state"].update({"public_key_digest": zero32})),
        ),
        FixtureMutation(
            "cross_user_account_id_replay",
            "cross_user_replay",
            "reuse account_id from another user with fresh public key binding recomputed",
            ["binding_break"],
            "acceptance would bind one account handle to two user identities, giving a direct account-binding collision",
            recomputed(lambda c, d: c["account_state"].update({"account_id": d["account_state"]["account_id"]})),
        ),
        FixtureMutation(
            "public_key_substitution_with_stale_pk_binding",
            "public_key_substitution",
            "swap public_key_digest from another user while leaving pk_binding_commitment stale",
            ["binding_break"],
            "if accepted, the verifier ignored the explicit account/key binding digest",
            stale(lambda c, d: c["account_state"].update({"public_key_digest": d["account_state"]["public_key_digest"]})),
        ),
        FixtureMutation(
            "public_key_substitution_rebound",
            "public_key_substitution",
            "swap public_key_digest from another user and recompute account/key bindings",
            ["binding_break"],
            "if accepted without provenance checks, a rogue key can be rebound to an existing account state by recomputing public commitments",
            recomputed(lambda c, d: c["account_state"].update({"public_key_digest": d["account_state"]["public_key_digest"]})),
        ),
        FixtureMutation(
            "malformed_short_opening_source_values",
            "malformed_opening",
            "truncate the six-value account opening source vector to five values",
            ["extractor_break"],
            "acceptance would leave one selected opening lane unconstrained, so extractor coverage is incomplete",
            recomputed(lambda c, _: c["opening"].update({"source_values": c["opening"]["source_values"][:-1]})),
        ),
        FixtureMutation(
            "malformed_noncanonical_opening_source",
            "malformed_opening",
            "set one account opening source value to q",
            ["extractor_break"],
            "acceptance of a noncanonical q-residue makes the byte/opening relation ambiguous across representatives",
            recomputed(lambda c, _: c["opening"]["source_values"].__setitem__(0, Q)),
        ),
        FixtureMutation(
            "zero_opening_source_vector",
            "identity_like_state",
            "set all six opening source values to zero and recompute dependent digests",
            ["extractor_break"],
            "an all-zero selected opening makes the hidden-source lane check identity-like unless rejected at account creation",
            recomputed(lambda c, _: c["opening"].update({"source_values": [0, 0, 0, 0, 0, 0]})),
        ),
        FixtureMutation(
            "opening_digest_replay_from_other_account",
            "commitment_replay",
            "copy opening digest from another account while leaving local opening values unchanged",
            ["binding_break"],
            "if accepted, account state no longer binds the stored opening digest to the supplied opening frame",
            stale(lambda c, d: c.update({"opening_digest": d["opening_digest"]})),
        ),
        FixtureMutation(
            "account_state_commitment_replay_from_other_user",
            "commitment_replay",
            "copy account_state_commitment from another user and recompute only transcript digest",
            ["binding_break"],
            "acceptance would be a public account-state commitment collision across identity/key context",
            lambda c, d: (c.update({"account_state_commitment": d["account_state_commitment"]}), c.update({"transcript_digest": transcript_digest(c)})),
        ),
    ]


def build_payload() -> dict[str, Any]:
    source_artifacts = {
        "account_state_schema_contract": ACCOUNT_SCHEMA_JSON,
        "account_opening_commitment_wiring": ACCOUNT_WIRING_JSON,
        "expanded_opening_commitment_binding": EXPANDED_BINDING_JSON,
        "transcript_malleability_probe": TRANSCRIPT_MALLEABILITY_JSON,
    }
    loaded_sources = {key: load_json(path) for key, path in source_artifacts.items()}

    references = [make_reference("alpha", 7), make_reference("beta", 11)]
    reference_index = [
        {
            "fixture_id": fixture["fixture_id"],
            "slot_id": fixture["account_state"]["slot_id"],
            "user_id": fixture["account_state"]["user_id"],
            "account_id": fixture["account_state"]["account_id"],
            "public_key_digest": fixture["account_state"]["public_key_digest"],
            "account_state_commitment": fixture["account_state_commitment"],
            "pk_binding_commitment": fixture["pk_binding_commitment"],
            "transcript_digest": fixture["transcript_digest"],
        }
        for fixture in references
    ]

    cases = []
    for mutation in build_mutations():
        candidate = copy.deepcopy(references[0])
        mutation.mutate(candidate, references[1])
        strict = verify_fixture(candidate, STRICT_POLICY, references)
        weakened = verify_fixture(candidate, WEAK_POLICY, references)
        accepted_break_classification = (
            mutation.expected_break_classification if weakened["accepted"] else []
        )
        cases.append(
            {
                "id": mutation.mutation_id,
                "attack_surface": mutation.attack_surface,
                "operation": mutation.operation,
                "strict_verifier": strict,
                "weakened_boundary": weakened,
                "accepted_under_weakened_boundary": weakened["accepted"],
                "accepted_break_classification": accepted_break_classification,
                "feasible_reduction": mutation.feasible_reduction if weakened["accepted"] else "not accepted even under weakened boundary",
                "mutated_fixture_digest": candidate["transcript_digest"],
            }
        )

    strict_accepts = [case["id"] for case in cases if case["strict_verifier"]["accepted"]]
    weakened_accepts = [
        case for case in cases if case["accepted_under_weakened_boundary"]
    ]
    unclassified_weakened_accepts = [
        case["id"] for case in weakened_accepts if not case["accepted_break_classification"]
    ]
    if strict_accepts:
        raise AssertionError(f"strict verifier accepted adversarial fixtures: {strict_accepts}")
    if unclassified_weakened_accepts:
        raise AssertionError(
            f"weakened accepted fixtures lack break classifications: {unclassified_weakened_accepts}"
        )

    by_surface: dict[str, dict[str, int]] = {}
    for case in cases:
        surface = by_surface.setdefault(
            case["attack_surface"],
            {"cases": 0, "strict_rejects": 0, "weakened_accepts": 0},
        )
        surface["cases"] += 1
        surface["strict_rejects"] += int(not case["strict_verifier"]["accepted"])
        surface["weakened_accepts"] += int(case["accepted_under_weakened_boundary"])

    return {
        "schema": "Descender.pre_third_party_rogue_key_account_fixture_analysis.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "campaign_id": "PTC-008",
        "assumption_class": "rogue_key_and_account_binding",
        "status": "strict_boundary_rejects_all_degenerate_fixtures_weakened_accepts_classified",
        "deployment_ready": False,
        "source_artifacts": {
            key: {
                "path": rel(path),
                "schema": loaded_sources[key].get("schema"),
                "status": loaded_sources[key].get("status"),
            }
            for key, path in source_artifacts.items()
        },
        "target": {
            "n": 32,
            "q": Q,
            "profile_id": PROFILE,
            "state_domain": STATE_DOMAIN,
            "opening_domain": OPENING_DOMAIN,
            "public_key_binding_domain": PK_BINDING_DOMAIN,
            "transcript_domain": TRANSCRIPT_DOMAIN,
        },
        "policies": {
            "strict": STRICT_POLICY.__dict__,
            "weakened_boundary_for_break_classification": WEAK_POLICY.__dict__,
        },
        "reference_fixtures": reference_index,
        "fixture_summary": {
            "case_count": len(cases),
            "strict_reject_count": len(cases) - len(strict_accepts),
            "strict_unexpected_accept_count": len(strict_accepts),
            "weakened_accept_count": len(weakened_accepts),
            "unclassified_weakened_accept_count": len(unclassified_weakened_accepts),
            "by_surface": by_surface,
        },
        "fixture_cases": cases,
        "classifier": {
            "extractor_break": "accepted fixture makes a required witness/opening component identity-like, malformed, noncanonical, or outside the reviewed keygen/opening image",
            "binding_break": "accepted fixture gives a collision, replay, substitution, or context mismatch in account/key/commitment binding",
        },
        "assertions": {
            "strict_boundary_rejects_all_adversarial_fixtures": len(strict_accepts) == 0,
            "weakened_accepts_are_break_classified": len(unclassified_weakened_accepts) == 0,
            "covered_attack_surfaces": sorted(by_surface),
            "required_user_surfaces_present": all(
                surface in by_surface
                for surface in [
                    "duplicate_commitments",
                    "identity_like_state",
                    "cross_user_replay",
                    "public_key_substitution",
                    "malformed_opening",
                    "commitment_replay",
                ]
            ),
        },
        "decision": {
            "can_accept_now": False,
            "reason": "the harness gives deterministic fail-closed evidence and break labels for weakened accepted mutations, but it remains a modeled fixture analysis until wired to audited production account/key registration vectors",
            "promotion_requirement": "run the same fixtures through the production facade; any strict acceptance without extractor_break or binding_break classification blocks third-party review",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    summary = payload["fixture_summary"]
    lines = [
        "# Pre-Third-Party Rogue-Key Account Fixture Analysis",
        "",
        "This PTC-008 artifact builds deterministic degenerate account/key/",
        "commitment fixtures and checks that the strict modeled boundary rejects",
        "them.  It also runs the same fixtures against a deliberately weakened",
        "boundary so any accepted mutation is labeled as an extractor break, a",
        "binding break, or both.",
        "",
        "## Summary",
        "",
        f"- campaign: `{payload['campaign_id']}`",
        f"- assumption class: `{payload['assumption_class']}`",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- profile: `{payload['target']['profile_id']}`",
        f"- q: `{payload['target']['q']}`",
        f"- fixture cases: `{summary['case_count']}`",
        f"- strict rejects: `{summary['strict_reject_count']}`",
        f"- strict unexpected accepts: `{summary['strict_unexpected_accept_count']}`",
        f"- weakened-boundary accepts: `{summary['weakened_accept_count']}`",
        f"- unclassified weakened accepts: `{summary['unclassified_weakened_accept_count']}`",
        "",
        "## Source Artifacts",
        "",
    ]
    for key, source in payload["source_artifacts"].items():
        lines.append(
            f"- {key}: `{source['path']}`"
            f" (schema `{source['schema']}`, status `{source['status']}`)"
        )

    lines.extend(
        [
            "",
            "## Reference Fixtures",
            "",
            "| id | slot | user id | account id | public key digest | account commitment |",
            "|---|---:|---|---|---|---|",
        ]
    )
    for row in payload["reference_fixtures"]:
        lines.append(
            f"| `{row['fixture_id']}` | {row['slot_id']} | `{row['user_id']}` | "
            f"`{row['account_id']}` | `{row['public_key_digest']}` | "
            f"`{row['account_state_commitment']}` |"
        )

    lines.extend(
        [
            "",
            "## Fixture Results",
            "",
            "| case | surface | strict accepted | weakened accepted | break classification | strict errors | reduction note |",
            "|---|---|:---:|:---:|---|---|---|",
        ]
    )
    for case in payload["fixture_cases"]:
        strict_errors = case["strict_verifier"]["parse_errors"] + case["strict_verifier"]["boundary_errors"]
        classifications = case["accepted_break_classification"] or ["not_accepted"]
        lines.append(
            f"| `{case['id']}` | `{case['attack_surface']}` | "
            f"`{case['strict_verifier']['accepted']}` | "
            f"`{case['accepted_under_weakened_boundary']}` | "
            f"{', '.join(f'`{item}`' for item in classifications)} | "
            f"{', '.join(f'`{error}`' for error in strict_errors)} | "
            f"{case['feasible_reduction']} |"
        )

    lines.extend(["", "## Surface Coverage", "", "| surface | cases | strict rejects | weakened accepts |", "|---|---:|---:|---:|"])
    for surface, counts in sorted(summary["by_surface"].items()):
        lines.append(
            f"| `{surface}` | {counts['cases']} | {counts['strict_rejects']} | {counts['weakened_accepts']} |"
        )

    assertions = payload["assertions"]
    lines.extend(
        [
            "",
            "## Assertions",
            "",
            f"- strict boundary rejects all adversarial fixtures: `{assertions['strict_boundary_rejects_all_adversarial_fixtures']}`",
            f"- weakened accepts are break classified: `{assertions['weakened_accepts_are_break_classified']}`",
            f"- required user surfaces present: `{assertions['required_user_surfaces_present']}`",
            f"- covered attack surfaces: `{', '.join(assertions['covered_attack_surfaces'])}`",
            "",
            "## Classifier",
            "",
        ]
    )
    for label, meaning in payload["classifier"].items():
        lines.append(f"- `{label}`: {meaning}")

    decision = payload["decision"]
    lines.extend(
        [
            "",
            "## Decision",
            "",
            f"- can accept now: `{decision['can_accept_now']}`",
            f"- reason: {decision['reason']}",
            f"- promotion requirement: {decision['promotion_requirement']}",
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
