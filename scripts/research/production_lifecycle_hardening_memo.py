#!/usr/bin/env python3
"""Generate the production lifecycle hardening memo."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
PRODUCTION_REFERENCE_GAP_REPORT = (
    READINESS_DIR / "production_reference_gap_report_n32_q12289.json"
)
REFERENCE_TIMING_REPORT = READINESS_DIR / "reference_timing_report_n32_q12289.json"
DEFAULT_JSON = READINESS_DIR / "production_lifecycle_hardening_memo_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "production_lifecycle_hardening_memo_n32_q12289.md"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def timing_avg_us(timing_report: dict[str, Any], operation: str) -> float | None:
    timing = timing_report.get("timing_report", {})
    return next(
        (
            row.get("avg_us")
            for row in timing.get("timings", [])
            if row.get("operation") == operation
        ),
        None,
    )


def build_payload() -> dict[str, Any]:
    gap = load_json(PRODUCTION_REFERENCE_GAP_REPORT)
    timing = load_json(REFERENCE_TIMING_REPORT)
    facade = gap.get("production_facade", {})
    capability_gates = {
        key: value
        for key, value in facade.items()
        if key.startswith("has_library_") or key.startswith("has_production_")
    }
    lifecycle_manager_present = bool(capability_gates.get("has_library_lifecycle_manager"))
    lifecycle_file_store_present = bool(
        capability_gates.get("has_library_lifecycle_file_store")
    )
    selected_projection_lifecycle_file_store_present = bool(
        capability_gates.get("has_library_selected_projection_lifecycle_file_store")
    )
    lifecycle_backend_blocker = (
        "The library now has an in-memory lifecycle manager and file-backed reference stores covering reservation, successful reserved signing consumption, accepted verify-and-consume, replay rejection, snapshot/restore validation, epoch-checked reservation tokens, monotonic snapshot sequences, restore-anchor rollback rejection, reorg revealed-signature burn records, public-key-hash/slot-id spent-ledger keys, spent-ledger duplicate rejection, schema-checked spent-ledger persistence, combined chain-state file-store transitions, atomic temp-write/rename persistence, schema checks, lock-file writer serialization, timestamped lock records, stale-lock breaking, and recovery burn semantics. The selected projection route additionally has a state-assisted lifecycle snapshot/store plus a consensus-facing transition that persists hidden_source_opening_commitment and restore_anchor_digest, checks trusted restore-anchor freshness, consumes the slot, records the spent key, wires the transition through separate lifecycle, spent-ledger, and trusted-anchor stores with writer locks, exposes the same transition through a target-chain database adapter contract with an executable in-memory conformance backend, binds that contract to deterministic schema-tagged transactional KV keys, includes a durable file-backed transactional KV reference engine with writer locks, temp-file fsync, and atomic rename, has a target-chain database adapter profile validator that rejects reference engines, unsafe anchor placement, non-atomic semantics, schema drift, and missing external-review digest, has a profiled KV adapter that refuses in-memory/file-backed reference KV engines, has a canonical-node store adapter boundary that passes the validated profile into every chain DB read/write batch, includes a reusable canonical-store conformance check for initialize/commit/reopen/replay behavior, includes a concrete local canonical JSON backend with profile-digest persistence, writer locks, temp-file fsync, atomic rename, reload tests, profile-mismatch rejection, and conformance coverage, includes a concrete SQLite canonical-store adapter with schema metadata, profile-digest binding, WAL journaling, FULL synchronous durability, SQL transactions, reopen tests, profile-mismatch rejection, and conformance coverage, and includes a mock finalized blockchain simulator that finalizes selected-projection spends, publishes deterministic state roots, rejects replay, and reloads finalized state; selecting the production chain node database and completing external storage review remain open."
        if lifecycle_file_store_present and selected_projection_lifecycle_file_store_present
        else "The library now has an in-memory lifecycle manager and file-backed reference stores covering reservation, successful reserved signing consumption, accepted verify-and-consume, replay rejection, snapshot/restore validation, epoch-checked reservation tokens, monotonic snapshot sequences, restore-anchor rollback rejection, reorg revealed-signature burn records, public-key-hash/slot-id spent-ledger keys, spent-ledger duplicate rejection, schema-checked spent-ledger persistence, combined chain-state file-store transitions, atomic temp-write/rename persistence, schema checks, lock-file writer serialization, timestamped lock records, stale-lock breaking, and recovery burn semantics; production storage hardening, trusted anchor placement, and deployment-specific chain/state integration remain absent."
        if lifecycle_file_store_present
        else (
        "The library now has an in-memory lifecycle manager covering reservation, successful reserved signing consumption, accepted verify-and-consume, replay rejection, snapshot/restore validation, epoch-checked reservation tokens, and recovery burn semantics; durable storage, interprocess locks, backup/restore policy, reorg policy, and consensus spent-ledger integration remain absent."
        if lifecycle_manager_present
        else "A durable wallet/account-state manager with reservation, consumption, crash recovery, and concurrency semantics is absent."
        )
    )
    return {
        "schema": "Descender.production_lifecycle_hardening_memo.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source_artifacts": [
            rel(PRODUCTION_REFERENCE_GAP_REPORT),
            rel(REFERENCE_TIMING_REPORT),
        ],
        "current_facade_evidence": {
            "account_link_bytes": facade.get("account_commitment_bytes"),
            "reviewed_target_transaction_bytes": facade.get(
                "target_transaction_signature_bytes"
            ),
            "executable_signature_bytes": facade.get("executable_signature_bytes"),
            "executable_matches_target": facade.get("executable_matches_target"),
            "capability_gates": capability_gates,
            "timing_caveat": timing.get("timing_report", {}).get("caveat"),
            "production_facade_keygen_avg_us": timing_avg_us(
                timing, "production_facade_keygen"
            ),
            "production_facade_sign_avg_us": timing_avg_us(
                timing, "production_facade_sign"
            ),
            "production_facade_verify_avg_us": timing_avg_us(
                timing, "production_facade_verify"
            ),
        },
        "lifecycle_requirements": [
            {
                "name": "reserve_before_sign",
                "requirement": (
                    "A signer must durably reserve a slot before releasing any "
                    "signature bytes that consume that slot."
                ),
                "failure_mode": "crash or retry can reuse the same hidden slot",
            },
            {
                "name": "monotonic_accept_consume",
                "requirement": (
                    "An accepted verification must advance the account state "
                    "monotonically and make the consumed slot unavailable."
                ),
                "failure_mode": "accepted replay can spend the same account link twice",
            },
            {
                "name": "replay_rejection",
                "requirement": (
                    "Verifiers and account managers must reject signatures whose "
                    "slot id, nullifier, or account transition is already spent."
                ),
                "failure_mode": "network replay preserves proof validity",
            },
            {
                "name": "crash_safe_commit",
                "requirement": (
                    "Reservation, signature release, and account-state commit "
                    "must have a recovery protocol with no ambiguous reused state."
                ),
                "failure_mode": "power loss exposes two valid signatures for one slot",
            },
            {
                "name": "concurrent_signing_lock",
                "requirement": (
                    "Wallets must serialize signing per account slot or use a "
                    "lease/lock that survives process races."
                ),
                "failure_mode": "parallel signers allocate the same slot",
            },
            {
                "name": "backup_restore_protection",
                "requirement": (
                    "Restored backups must not silently restore spent slots; "
                    "deployments need epoch, chain, or server-side spent-state checks."
                ),
                "failure_mode": "old wallet backup resurrects consumed hidden state",
            },
            {
                "name": "reorg_policy",
                "requirement": (
                    "Blockchain deployments must define whether reserved or "
                    "broadcast slots are burned across reorgs and mempool eviction."
                ),
                "failure_mode": "fork/reorg handling causes unsafe state rollback",
            },
            {
                "name": "spent_slot_ledger",
                "requirement": (
                    "The account layer must expose a canonical spent-slot or "
                    "nullifier ledger policy for full nodes and light clients."
                ),
                "failure_mode": "different verifiers disagree about consumed state",
            },
            {
                "name": "atomic_next_slot_write",
                "requirement": (
                    "Rolling account-state updates must atomically write the next "
                    "68-byte account link with the accepted consumption event."
                ),
                "failure_mode": "account state and transaction acceptance diverge",
            },
            {
                "name": "selected_projection_state_record",
                "requirement": (
                    "The selected 308-byte/Fq12 projection route must persist "
                    "hidden_source_opening_commitment and restore_anchor_digest "
                    "in the consensus-facing lifecycle snapshot."
                ),
                "failure_mode": (
                    "durable state drops the hidden-source opening binding or "
                    "trusted-anchor binding required by the proof"
                ),
            },
        ],
        "constant_time_requirements": [
            {
                "name": "secret_independent_sampler_timing",
                "requirement": (
                    "Production signing and sampling must not have secret-dependent "
                    "rejection loops or data-dependent retry counts."
                ),
            },
            {
                "name": "fixed_failure_behavior",
                "requirement": (
                    "Secret-side failures must be classified, bounded, and reviewed "
                    "so timing or error channels do not identify hidden state."
                ),
            },
            {
                "name": "secret_independent_memory_access",
                "requirement": (
                    "Sampler/keygen memory access should be secret-independent where "
                    "the implementation handles hidden witnesses or randomizers."
                ),
            },
            {
                "name": "no_secret_logging",
                "requirement": (
                    "Logs, traces, panic messages, and test diagnostics must never "
                    "emit hidden witness, randomizer, seed, or active pad material."
                ),
            },
            {
                "name": "constant_time_regression_tests",
                "requirement": (
                    "The hardened crate needs dudect/ctgrind-style regression "
                    "instrumentation or an equivalent side-channel review packet."
                ),
            },
        ],
        "open_blockers": [
            "The selected 308-byte/Fq12 projection-carrier verifier is fail-closed until exact fallback certificate and leakage semantics are implemented; the facade still wraps the legacy 254-byte callback.",
            lifecycle_backend_blocker,
            "Constant-time and side-channel audit is absent for production signing, sampling, and secret-key handling.",
            "Production entropy hooks and deterministic KAT vectors are not yet split into a hardened reference crate.",
            "Broader fuzz/property tests for public-key, account-state, signature, and challenge-frame codecs remain open.",
        ],
        "review_actions": [
            "Review the lifecycle requirements as consensus-critical state-machine rules, not wallet UX guidance.",
            "Implement a minimal durable account-state manager and test double-spend/replay/crash/concurrency cases.",
            "Implement accepting 308-byte/Fq12 projection-carrier fallback semantics and promote that verifier through the facade before deployment claims.",
            "Add constant-time review artifacts for all secret-side production functions.",
            "Add KATs and property/fuzz tests for all byte encoders and state transitions.",
        ],
        "interpretation": (
            "The current production facade is useful smoke evidence for the typed "
            "boundary and checked account-state handoff.  It is not a deployment "
            "claim because the executable path still uses the legacy 254-byte proof "
            "callback and lacks durable lifecycle and side-channel hardening."
        ),
    }


def markdown(payload: dict[str, Any]) -> str:
    evidence = payload["current_facade_evidence"]
    capability_gates = evidence["capability_gates"]
    lines = [
        "# Production Lifecycle Hardening Memo",
        "",
        "This memo records the production state-lifecycle and side-channel",
        "requirements for the q12289 account-carried signature target.  It is a",
        "review checklist for hardening; it is not a deployment claim.",
        "",
        "## Current Facade Evidence",
        "",
        f"- generated at: `{payload['generated_at']}`",
        f"- source artifacts: `{', '.join(payload['source_artifacts'])}`",
        f"- account-link bytes: `{evidence['account_link_bytes']}`",
        f"- reviewed target transaction bytes: `{evidence['reviewed_target_transaction_bytes']}`",
        f"- executable signature bytes: `{evidence['executable_signature_bytes']}`",
        f"- executable matches reviewed target: `{evidence['executable_matches_target']}`",
        f"- production-facade keygen avg us: `{evidence['production_facade_keygen_avg_us']}`",
        f"- production-facade sign avg us: `{evidence['production_facade_sign_avg_us']}`",
        f"- production-facade verify avg us: `{evidence['production_facade_verify_avg_us']}`",
        f"- timing caveat: {evidence['timing_caveat']}",
        "",
        "## Capability Gates",
        "",
    ]
    lines.extend(f"- {key}: `{value}`" for key, value in sorted(capability_gates.items()))
    lines.extend(["", "## Lifecycle Requirements", ""])
    lines.extend(
        f"- `{item['name']}`: {item['requirement']}  Failure mode: {item['failure_mode']}."
        for item in payload["lifecycle_requirements"]
    )
    lines.extend(["", "## Constant-Time Requirements", ""])
    lines.extend(
        f"- `{item['name']}`: {item['requirement']}"
        for item in payload["constant_time_requirements"]
    )
    lines.extend(["", "## Open Blockers", ""])
    lines.extend(f"- {item}" for item in payload["open_blockers"])
    lines.extend(["", "## Review Actions", ""])
    lines.extend(f"- {item}" for item in payload["review_actions"])
    lines.extend(["", "## Interpretation", "", payload["interpretation"], ""])
    return "\n".join(lines)


def main() -> None:
    payload = build_payload()
    DEFAULT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    DEFAULT_MD.write_text(markdown(payload))
    print(json.dumps({"json": str(DEFAULT_JSON), "markdown": str(DEFAULT_MD)}, indent=2))


if __name__ == "__main__":
    main()
