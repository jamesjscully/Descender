#!/usr/bin/env python3
from __future__ import annotations

import os


PIVOT_ALIGNMENT_PHRASES = (
    "trackers.md",  # typo guard for generated tasks; harmless if unused
    "tracker.md",
    "spec/",
    "decisions/",
    "adr-",
    "archive_index",
    "chain-native",
    "pq authorization",
    "post-quantum authorization",
    "path 2",
    "path2",
    "prevalidated",
    "slot_authorization",
    "slot authorization",
    "account creation",
    "account-creation",
    "poseidon2",
    "plonky3",
    "koalabear",
    "checked-core",
    "rollup",
    "light-client",
    "bridge committee",
    "hardware-wallet",
    "ethereum",
)

LEGACY_PRE_PIVOT_PHRASES = (
    "research_program.md",
    "lean_proof_plan.md",
    "guarded tilt",
    "guarded-tilt",
    "cauchy",
    "wedge",
    "peeling rank",
    "deterministic transversality",
    "p-tnd",
    "kem",
)

LEGACY_PRODUCT_PHRASES = (
    "selected 308-byte",
    "selected 308 b",
    "308-byte/fq12",
    "308 b/fq12",
    "projection-carrier deployment",
    "projection-carrier verifier",
    "deployable_signature_target",
    "deployable signature target",
    "current deployable signature",
    "active deployable-signature",
)

LEGACY_ALLOWED_CONTEXT_PHRASES = (
    "archive",
    "archived",
    "archive_index",
    "negative-control",
    "negative control",
    "legacy",
    "stale",
    "cleanup",
    "clean up",
    "migration",
    "migrate",
    "fail-closed",
    "regression",
    "retired",
    "superseded",
    "adr-0005",
)


def text_fields(task: dict) -> str:
    parts = [
        str(task.get("track", "")),
        str(task.get("kind", "")),
        str(task.get("title", "")),
        str(task.get("summary", "")),
        str(task.get("prompt", "")),
    ]
    parts.extend(str(item) for item in task.get("acceptance", []))
    return "\n".join(parts).lower()


def task_pivot_alignment_reason(task: dict) -> tuple[bool, str]:
    if os.environ.get("DESCENDER_ALLOW_LEGACY_TASKS") == "1":
        return True, "legacy_override"

    text = text_fields(task)
    has_legacy_product_claim = any(
        phrase in text for phrase in LEGACY_PRODUCT_PHRASES
    )
    has_legacy_allowed_context = any(
        phrase in text for phrase in LEGACY_ALLOWED_CONTEXT_PHRASES
    )
    if has_legacy_product_claim and not has_legacy_allowed_context:
        return False, "legacy_product_claim_without_archive_context"

    has_pre_pivot_math_claim = any(
        phrase in text for phrase in LEGACY_PRE_PIVOT_PHRASES
    )
    if has_pre_pivot_math_claim and not has_legacy_allowed_context:
        return False, "pre_pivot_math_claim_without_archive_context"

    if task.get("pivot_aligned") is True:
        return True, "explicit_pivot_aligned"
    if task.get("pivot_aligned") is False:
        return False, "explicit_pivot_rejected"

    if any(phrase in text for phrase in PIVOT_ALIGNMENT_PHRASES):
        return True, "matched_active_pivot_phrase"
    return False, "no_active_pivot_evidence"


def task_pivot_aligned(task: dict) -> bool:
    aligned, _reason = task_pivot_alignment_reason(task)
    return aligned
