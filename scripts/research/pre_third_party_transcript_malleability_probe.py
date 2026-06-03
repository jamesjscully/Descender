#!/usr/bin/env python3
"""Whole-transcript mutation probe for PTC-005.

This is a deterministic fuzzer-style harness over the public byte surfaces
already pinned by the state-assisted opening/carrier artifacts.  It models the
minimum fail-closed property reviewers will expect at the facade boundary: any
mutation or cross-transcript field swap either fails canonical parsing,
changes a bound digest, or is classified as a named binding break rather than
an accepted transcript.
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
    READINESS_DIR / "pre_third_party_transcript_malleability_probe_n32_q12289.json"
)
DEFAULT_MD = (
    READINESS_DIR / "pre_third_party_transcript_malleability_probe_n32_q12289.md"
)
OPENING_FRAME_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_opening_frame_n32_q12289.json"
)
OPENING_VECTORS_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_opening_frame_test_vectors_n32_q12289.json"
)
NEGATIVE_CONTROLS_JSON = (
    READINESS_DIR / "hidden_source_state_assisted_negative_controls_n32_q12289.json"
)
BREAK_EXPORT_JSON = (
    READINESS_DIR
    / "hidden_source_fallback_projection_break_export_negative_tests_n32_q12289.json"
)
RO_POK_NEGATIVE_JSON = (
    READINESS_DIR
    / "hidden_source_state_assisted_ro_pok_byte_negative_tests_n32_q12289.json"
)

Q = 12289
PROFILE = "n32.q12289.fq12.state.v1"
WHOLE_TRANSCRIPT_DOMAIN = "Descender.pre_third_party.transcript_malleability.v1"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def sha256(data: bytes) -> bytes:
    return hashlib.sha256(data).digest()


def synthetic_bytes(label: str, length: int) -> bytes:
    output = b""
    counter = 0
    while len(output) < length:
        output += sha256(f"Descender.ptc005.transcript:{label}:{counter}".encode())
        counter += 1
    return output[:length]


def u16(value: int) -> bytes:
    return value.to_bytes(2, "little")


def u32(value: int) -> bytes:
    return value.to_bytes(4, "little")


def u64(value: int) -> bytes:
    return value.to_bytes(8, "little")


def framed(field_id: str, value: bytes, declared_len: int | None = None) -> bytes:
    encoded_id = field_id.encode()
    length = len(value) if declared_len is None else declared_len
    return u16(len(encoded_id)) + encoded_id + u32(length) + value


def is_hex_bytes(value: str) -> bool:
    if len(value) == 0 or len(value) % 2 != 0:
        return False
    try:
        bytes.fromhex(value)
    except ValueError:
        return False
    return True


def serialize_field_value(value: Any) -> bytes:
    if isinstance(value, str):
        if is_hex_bytes(value):
            return bytes.fromhex(value)
        return value.encode()
    if isinstance(value, int):
        return u64(value)
    if isinstance(value, list):
        return b"".join(u16(int(item)) for item in value)
    if isinstance(value, dict):
        return b"".join(
            framed(str(key), serialize_field_value(value[key])) for key in sorted(value)
        )
    raise TypeError(f"unsupported field value {type(value)!r}")


def serialize_transcript(transcript: dict[str, Any]) -> bytes:
    order = [
        "domain",
        "profile_id",
        "slot_id",
        "public_key_digest",
        "message_digest",
        "signature_digest",
        "proof_transcript_digest",
        "account_state",
        "opening_commitment",
        "proof_handle",
        "residues",
        "commitments",
    ]
    return b"".join(
        framed(field, serialize_field_value(transcript[field]), transcript.get("declared_lengths", {}).get(field))
        for field in order
    )


def transcript_digest(transcript: dict[str, Any]) -> str:
    return sha256(serialize_transcript(transcript)).hex()


def opening_commitment(transcript: dict[str, Any]) -> str:
    parts = [
        "Descender.hidden_source.opening.v1".encode(),
        PROFILE.encode(),
        u64(transcript["slot_id"]),
        bytes.fromhex(transcript["account_state"]["witness_commitment"]),
        bytes.fromhex(transcript["account_state"]["randomizer_commitment"]),
        bytes.fromhex(transcript["account_state"]["restore_anchor_digest"]),
        bytes.fromhex(transcript["commitments"]["witness_opening_digest"]),
        bytes.fromhex(transcript["commitments"]["pad_seed_digest"]),
        bytes.fromhex(transcript["commitments"]["carrier_handle_secret_digest"]),
    ]
    return sha256(b"".join(parts)).hex()


def proof_handle(transcript: dict[str, Any]) -> str:
    parts = [
        "Descender.hidden_source.carrier.v1".encode(),
        bytes.fromhex(transcript["opening_commitment"]),
        bytes.fromhex(transcript["proof_transcript_digest"]),
        bytes.fromhex(transcript["signature_digest"]),
        bytes.fromhex(transcript["public_key_digest"]),
        bytes.fromhex(transcript["message_digest"]),
        u64(transcript["slot_id"]),
        bytes.fromhex(transcript["commitments"]["carrier_handle_secret_digest"]),
    ]
    return sha256(b"".join(parts)).hex()


def make_reference(label: str, slot_id: int) -> dict[str, Any]:
    transcript = {
        "domain": WHOLE_TRANSCRIPT_DOMAIN,
        "profile_id": PROFILE,
        "slot_id": slot_id,
        "public_key_digest": synthetic_bytes(f"{label}:pk", 32).hex(),
        "message_digest": synthetic_bytes(f"{label}:message", 32).hex(),
        "signature_digest": synthetic_bytes(f"{label}:signature", 32).hex(),
        "proof_transcript_digest": synthetic_bytes(f"{label}:proof_transcript", 32).hex(),
        "account_state": {
            "account_id": synthetic_bytes(f"{label}:account_id", 16).hex(),
            "witness_commitment": synthetic_bytes(f"{label}:witness_commitment", 32).hex(),
            "randomizer_commitment": synthetic_bytes(f"{label}:randomizer_commitment", 32).hex(),
            "restore_anchor_digest": synthetic_bytes(f"{label}:restore_anchor", 32).hex(),
            "spent_slot_root": synthetic_bytes(f"{label}:spent_slot_root", 32).hex(),
        },
        "commitments": {
            "witness_opening_digest": synthetic_bytes(f"{label}:witness_opening", 32).hex(),
            "pad_seed_digest": synthetic_bytes(f"{label}:pad_seed", 32).hex(),
            "carrier_handle_secret_digest": synthetic_bytes(f"{label}:carrier_secret", 32).hex(),
            "product_block_commitment": synthetic_bytes(f"{label}:product_block", 32).hex(),
            "local_delta_commitment": synthetic_bytes(f"{label}:local_delta", 32).hex(),
        },
        "residues": [int.from_bytes(synthetic_bytes(f"{label}:residue:{i}", 2), "little") % Q for i in range(12)],
        "declared_lengths": {},
    }
    transcript["opening_commitment"] = opening_commitment(transcript)
    transcript["proof_handle"] = proof_handle(transcript)
    transcript["transcript_digest"] = transcript_digest(transcript)
    return transcript


def flip_hex(value: str) -> str:
    data = bytearray.fromhex(value)
    data[0] ^= 1
    return bytes(data).hex()


def set_path(obj: dict[str, Any], path: tuple[str, ...], value: Any) -> None:
    cursor: Any = obj
    for key in path[:-1]:
        cursor = cursor[key]
    cursor[path[-1]] = value


def get_path(obj: dict[str, Any], path: tuple[str, ...]) -> Any:
    cursor: Any = obj
    for key in path:
        cursor = cursor[key]
    return cursor


def classify(candidate: dict[str, Any], baseline_digest: str) -> dict[str, Any]:
    parse_errors: list[str] = []
    binding_errors: list[str] = []

    for field, declared_len in candidate.get("declared_lengths", {}).items():
        actual_len = len(serialize_field_value(candidate[field]))
        if declared_len != actual_len:
            parse_errors.append(f"{field}_declared_length_{declared_len}_actual_{actual_len}")

    for field in [
        "public_key_digest",
        "message_digest",
        "signature_digest",
        "proof_transcript_digest",
        "opening_commitment",
        "proof_handle",
    ]:
        try:
            raw = bytes.fromhex(candidate[field])
        except ValueError:
            parse_errors.append(f"{field}_not_hex")
            continue
        if len(raw) != 32:
            parse_errors.append(f"{field}_length_{len(raw)}")

    for field in [
        "witness_commitment",
        "randomizer_commitment",
        "restore_anchor_digest",
        "spent_slot_root",
    ]:
        raw = bytes.fromhex(candidate["account_state"][field])
        if len(raw) != 32 and field != "account_id":
            parse_errors.append(f"account_state.{field}_length_{len(raw)}")

    if candidate["domain"] != WHOLE_TRANSCRIPT_DOMAIN:
        binding_errors.append("wrong_whole_transcript_domain")
    if candidate["profile_id"] != PROFILE:
        binding_errors.append("wrong_profile")
    if not isinstance(candidate["slot_id"], int) or candidate["slot_id"] < 0:
        parse_errors.append("slot_not_canonical_u64")
    if any((not isinstance(r, int)) or r < 0 or r >= Q for r in candidate["residues"]):
        parse_errors.append("noncanonical_residue")

    recomputed_opening = opening_commitment(candidate)
    recomputed_handle = proof_handle(candidate)
    recomputed_transcript = transcript_digest(candidate)
    if candidate["opening_commitment"] != recomputed_opening:
        binding_errors.append("opening_commitment_mismatch")
    if candidate["proof_handle"] != recomputed_handle:
        binding_errors.append("proof_handle_context_mismatch")
    if candidate.get("transcript_digest") != recomputed_transcript:
        binding_errors.append("whole_transcript_digest_mismatch")
    if recomputed_transcript == baseline_digest:
        binding_errors.append("mutation_did_not_change_whole_transcript_digest")

    if parse_errors:
        classification = "reject_parse"
    elif binding_errors:
        classification = "reject_binding_or_named_break"
    else:
        classification = "unexpected_accept"

    return {
        "classification": classification,
        "parse_errors": parse_errors,
        "binding_errors": binding_errors,
        "recomputed_transcript_digest": recomputed_transcript,
        "fail_closed": classification != "unexpected_accept",
    }


@dataclass(frozen=True)
class Mutation:
    mutation_id: str
    attack_surface: str
    operation: str
    mutate: Callable[[dict[str, Any], dict[str, Any]], None]


def path_flip(path: tuple[str, ...]) -> Callable[[dict[str, Any], dict[str, Any]], None]:
    def inner(candidate: dict[str, Any], _: dict[str, Any]) -> None:
        set_path(candidate, path, flip_hex(get_path(candidate, path)))

    return inner


def path_swap(path: tuple[str, ...]) -> Callable[[dict[str, Any], dict[str, Any]], None]:
    def inner(candidate: dict[str, Any], donor: dict[str, Any]) -> None:
        set_path(candidate, path, copy.deepcopy(get_path(donor, path)))

    return inner


def build_mutations() -> list[Mutation]:
    return [
        Mutation("length_public_key_short", "length", "declare public_key_digest as 31 bytes", lambda c, _: c["declared_lengths"].update({"public_key_digest": 31})),
        Mutation("length_proof_handle_long", "length", "declare proof_handle as 33 bytes", lambda c, _: c["declared_lengths"].update({"proof_handle": 33})),
        Mutation("domain_flip", "domain", "flip whole-transcript domain bytes", lambda c, _: c.update({"domain": "Descender.pre_third_party.transcript_malleability.v0"})),
        Mutation("profile_swap", "domain", "replace profile_id with a neighboring q12289 profile string", lambda c, _: c.update({"profile_id": "n32.q12289.fq12.fallback.v1"})),
        Mutation("slot_increment", "slot", "increment slot_id with proof handle fixed", lambda c, _: c.update({"slot_id": c["slot_id"] + 1})),
        Mutation("slot_swap", "slot", "swap slot_id from a second accepted transcript", path_swap(("slot_id",))),
        Mutation("key_bitflip", "key", "flip public_key_digest bit with proof handle fixed", path_flip(("public_key_digest",))),
        Mutation("key_swap", "key", "swap public_key_digest from a second accepted transcript", path_swap(("public_key_digest",))),
        Mutation("message_bitflip", "message", "flip message_digest bit with proof handle fixed", path_flip(("message_digest",))),
        Mutation("message_swap", "message", "swap message_digest from a second accepted transcript", path_swap(("message_digest",))),
        Mutation("account_witness_swap", "account_state", "swap account witness commitment from a second accepted transcript", path_swap(("account_state", "witness_commitment"))),
        Mutation("account_randomizer_bitflip", "account_state", "flip account randomizer commitment", path_flip(("account_state", "randomizer_commitment"))),
        Mutation("restore_anchor_swap", "account_state", "swap restore anchor from a second accepted transcript", path_swap(("account_state", "restore_anchor_digest"))),
        Mutation("proof_handle_bitflip", "proof_handle", "flip proof handle bit", path_flip(("proof_handle",))),
        Mutation("proof_handle_swap", "proof_handle", "swap proof handle from a second accepted transcript", path_swap(("proof_handle",))),
        Mutation("proof_transcript_swap", "proof_handle", "swap proof transcript digest from a second accepted transcript", path_swap(("proof_transcript_digest",))),
        Mutation("residue_noncanonical", "residues", "set one residue to q", lambda c, _: c["residues"].__setitem__(0, Q)),
        Mutation("residue_swap", "residues", "swap residue vector from a second accepted transcript", path_swap(("residues",))),
        Mutation("commitment_product_bitflip", "commitments", "flip product block commitment", path_flip(("commitments", "product_block_commitment"))),
        Mutation("commitment_pad_seed_swap", "commitments", "swap pad seed digest from a second accepted transcript", path_swap(("commitments", "pad_seed_digest"))),
        Mutation("commitment_carrier_secret_bitflip", "commitments", "flip carrier handle secret digest", path_flip(("commitments", "carrier_handle_secret_digest"))),
        Mutation("opening_commitment_swap", "commitments", "swap opening commitment from a second accepted transcript", path_swap(("opening_commitment",))),
    ]


def build_payload() -> dict[str, Any]:
    opening_frame = load_json(OPENING_FRAME_JSON)
    opening_vectors = load_json(OPENING_VECTORS_JSON)
    negative_controls = load_json(NEGATIVE_CONTROLS_JSON)
    break_export = load_json(BREAK_EXPORT_JSON)
    ro_pok = load_json(RO_POK_NEGATIVE_JSON)

    corpus = [make_reference("alpha", 7), make_reference("beta", 11)]
    mutations = build_mutations()
    cases = []
    for mutation in mutations:
        candidate = copy.deepcopy(corpus[0])
        mutation.mutate(candidate, corpus[1])
        result = classify(candidate, corpus[0]["transcript_digest"])
        cases.append(
            {
                "id": mutation.mutation_id,
                "attack_surface": mutation.attack_surface,
                "operation": mutation.operation,
                "classification": result["classification"],
                "fail_closed": result["fail_closed"],
                "parse_errors": result["parse_errors"],
                "binding_errors": result["binding_errors"],
                "recomputed_transcript_digest": result["recomputed_transcript_digest"],
            }
        )

    unexpected = [case["id"] for case in cases if not case["fail_closed"]]
    if unexpected:
        raise AssertionError(f"unexpected accepting mutations: {unexpected}")

    by_surface: dict[str, dict[str, int]] = {}
    for case in cases:
        surface = by_surface.setdefault(case["attack_surface"], {"cases": 0, "fail_closed": 0})
        surface["cases"] += 1
        surface["fail_closed"] += int(case["fail_closed"])

    return {
        "schema": "Descender.pre_third_party_transcript_malleability_probe.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "campaign_id": "PTC-005",
        "assumption_class": "transcript_binding_and_malleability",
        "status": "whole_transcript_mutation_probe_generated_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "opening_frame": rel(OPENING_FRAME_JSON),
            "opening_frame_schema": opening_frame.get("schema"),
            "opening_vectors": rel(OPENING_VECTORS_JSON),
            "opening_vectors_schema": opening_vectors.get("schema"),
            "state_assisted_negative_controls": rel(NEGATIVE_CONTROLS_JSON),
            "state_assisted_negative_controls_schema": negative_controls.get("schema"),
            "fallback_break_export_negative_tests": rel(BREAK_EXPORT_JSON),
            "fallback_break_export_schema": break_export.get("schema"),
            "ro_pok_byte_negative_tests": rel(RO_POK_NEGATIVE_JSON),
            "ro_pok_byte_negative_tests_schema": ro_pok.get("schema"),
        },
        "target": {
            "n": 32,
            "q": Q,
            "profile_id": PROFILE,
            "whole_transcript_domain": WHOLE_TRANSCRIPT_DOMAIN,
            "reference_corpus_size": len(corpus),
        },
        "reference_corpus": [
            {
                "id": f"reference_{index}",
                "slot_id": transcript["slot_id"],
                "transcript_bytes": len(serialize_transcript(transcript)),
                "transcript_digest": transcript["transcript_digest"],
                "opening_commitment": transcript["opening_commitment"],
                "proof_handle": transcript["proof_handle"],
            }
            for index, transcript in enumerate(corpus)
        ],
        "mutation_summary": {
            "case_count": len(cases),
            "fail_closed_count": sum(int(case["fail_closed"]) for case in cases),
            "unexpected_accept_count": len(unexpected),
            "by_surface": by_surface,
        },
        "mutation_cases": cases,
        "classifier": {
            "reject_parse": "declared length mismatch, nonhex digest, non-u64 slot, or noncanonical q-residue",
            "reject_binding_or_named_break": "canonical bytes parse but recomputed opening, proof handle, or whole-transcript digest disagrees",
            "unexpected_accept": "mutation preserved every checked binding and would require immediate verifier investigation",
        },
        "assertions": {
            "all_mutations_fail_closed": len(unexpected) == 0,
            "length_mutations_reject_parse": all(
                case["classification"] == "reject_parse"
                for case in cases
                if case["attack_surface"] == "length"
            ),
            "covered_attack_surfaces": sorted(by_surface),
        },
        "decision": {
            "can_accept_now": False,
            "reason": "the whole-transcript fuzzer gives deterministic fail-closed coverage for reviewer-style field swaps, but current evidence remains a modeled boundary until run against an audited accepting verifier/facade",
            "promotion_requirement": "wire these cases to the production verifier once accepted reference vectors are exported; any unexpected_accept must block third-party review packet finalization",
        },
        "artifacts": {
            "json": rel(DEFAULT_JSON),
            "markdown": rel(DEFAULT_MD),
        },
    }


def markdown(payload: dict[str, Any]) -> str:
    summary = payload["mutation_summary"]
    lines = [
        "# Pre-Third-Party Transcript Malleability Probe",
        "",
        "This PTC-005 artifact builds a deterministic whole-transcript mutation",
        "corpus from the pinned opening/carrier frame artifacts and classifies",
        "reviewer-style swaps as fail-closed unless they would require a named",
        "binding break.",
        "",
        "## Summary",
        "",
        f"- campaign: `{payload['campaign_id']}`",
        f"- assumption class: `{payload['assumption_class']}`",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- profile: `{payload['target']['profile_id']}`",
        f"- q: `{payload['target']['q']}`",
        f"- reference corpus size: `{payload['target']['reference_corpus_size']}`",
        f"- mutation cases: `{summary['case_count']}`",
        f"- fail-closed cases: `{summary['fail_closed_count']}`",
        f"- unexpected accepts: `{summary['unexpected_accept_count']}`",
        "",
        "## Source Artifacts",
        "",
    ]
    for key, value in payload["source_artifacts"].items():
        lines.append(f"- {key}: `{value}`")

    lines.extend(["", "## Reference Corpus", "", "| id | slot | bytes | transcript digest | opening commitment | proof handle |", "|---|---:|---:|---|---|---|"])
    for row in payload["reference_corpus"]:
        lines.append(
            f"| `{row['id']}` | {row['slot_id']} | {row['transcript_bytes']} | "
            f"`{row['transcript_digest']}` | `{row['opening_commitment']}` | `{row['proof_handle']}` |"
        )

    lines.extend(["", "## Mutation Results", "", "| case | surface | operation | classification | fail closed | errors |", "|---|---|---|---|:---:|---|"])
    for case in payload["mutation_cases"]:
        errors = case["parse_errors"] + case["binding_errors"]
        lines.append(
            f"| `{case['id']}` | `{case['attack_surface']}` | {case['operation']} | "
            f"`{case['classification']}` | `{case['fail_closed']}` | {', '.join(f'`{error}`' for error in errors)} |"
        )

    lines.extend(["", "## Surface Coverage", "", "| surface | cases | fail closed |", "|---|---:|---:|"])
    for surface, counts in sorted(summary["by_surface"].items()):
        lines.append(f"| `{surface}` | {counts['cases']} | {counts['fail_closed']} |")

    assertions = payload["assertions"]
    lines.extend(
        [
            "",
            "## Assertions",
            "",
            f"- all mutations fail closed: `{assertions['all_mutations_fail_closed']}`",
            f"- length mutations reject parse: `{assertions['length_mutations_reject_parse']}`",
            f"- covered attack surfaces: `{', '.join(assertions['covered_attack_surfaces'])}`",
        ]
    )

    lines.extend(
        [
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
