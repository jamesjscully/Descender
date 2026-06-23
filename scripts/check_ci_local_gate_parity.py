#!/usr/bin/env python3
"""Check that the local build gate and GitHub CI cover the same required gates.

This is a small guardrail, not a full YAML interpreter. It protects the process
discipline invariant that `scripts/check_build.sh` is a faithful local mirror of
the CI checks that matter for the current pivot. The explicit gate inventory
below is the authority for named gates, and the discovered Python-gate comparison
keeps newly added `python3 scripts/...` checks from drifting silently. Every
named gate also needs a `TRACKER.md` owner row.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
LOCAL = ROOT / "scripts/check_build.sh"
CI = ROOT / ".github/workflows/ci.yml"
TRACKER = ROOT / "TRACKER.md"


GATES: tuple[tuple[str, tuple[str, ...], tuple[str, ...]], ...] = (
    (
        "Lean build",
        ("lake build",),
        ("leanprover/lean-action@v1",),
    ),
    (
        "documentation authority check",
        ("python3 scripts/check_doc_authority.py",),
        ("python3 scripts/check_doc_authority.py",),
    ),
    (
        "ADR immutable-section check",
        ("python3 scripts/check_adr_immutable_sections.py",),
        ("python3 scripts/check_adr_immutable_sections.py",),
    ),
    (
        "process gate coverage check",
        ("python3 scripts/check_process_gate_coverage.py",),
        ("python3 scripts/check_process_gate_coverage.py",),
    ),
    (
        "documentation classification inventory",
        (
            "python3 scripts/list_doc_classification_candidates.py --strict --include-status-lines --include-path2-research --include-cryptanalysis-root --include-all-markdown",
        ),
        (
            "python3 scripts/list_doc_classification_candidates.py --strict --include-status-lines --include-path2-research --include-cryptanalysis-root --include-all-markdown",
        ),
    ),
    (
        "archive mixed-status audit",
        ("python3 scripts/check_archive_mixed_status.py --max-mixed 0",),
        ("python3 scripts/check_archive_mixed_status.py --max-mixed 0",),
    ),
    (
        "archive legacy-count audit",
        (
            "python3 scripts/check_archive_legacy_counts.py --max-non-active 10 --max ARCHIVED_TARGET=0 --max LEGACY_FORMAL=4 --max MIXED_STATUS=0 --max NEGATIVE_CONTROL=6 --max SUPERSEDED_DECISION=0 --max SUPERSEDED_TRACKER=0",
        ),
        (
            "python3 scripts/check_archive_legacy_counts.py --max-non-active 10 --max ARCHIVED_TARGET=0 --max LEGACY_FORMAL=4 --max MIXED_STATUS=0 --max NEGATIVE_CONTROL=6 --max SUPERSEDED_DECISION=0 --max SUPERSEDED_TRACKER=0",
        ),
    ),
    (
        "cleanup queue check",
        ("python3 scripts/check_cleanup_queue.py",),
        ("python3 scripts/check_cleanup_queue.py",),
    ),
    (
        "retired path absence check",
        ("python3 scripts/check_retired_path_absence.py",),
        ("python3 scripts/check_retired_path_absence.py",),
    ),
    (
        "active support import audit",
        ("python3 scripts/check_active_support_imports.py --max-unanchored 0",),
        ("python3 scripts/check_active_support_imports.py --max-unanchored 0",),
    ),
    (
        "active authority stale-term check",
        ("python3 scripts/check_active_authority_stale_terms.py",),
        ("python3 scripts/check_active_authority_stale_terms.py",),
    ),
    (
        "legacy code framing check",
        ("python3 scripts/check_legacy_code_framing.py",),
        ("python3 scripts/check_legacy_code_framing.py",),
    ),
    (
        "tool manifest framing check",
        ("python3 scripts/check_tool_manifest_framing.py",),
        ("python3 scripts/check_tool_manifest_framing.py",),
    ),
    (
        "active source framing check",
        ("python3 scripts/check_active_source_framing.py",),
        ("python3 scripts/check_active_source_framing.py",),
    ),
    (
        "CI/local gate parity check",
        ("python3 scripts/check_ci_local_gate_parity.py",),
        ("python3 scripts/check_ci_local_gate_parity.py",),
    ),
    (
        "tracker integrity check",
        ("python3 scripts/check_tracker_integrity.py",),
        ("python3 scripts/check_tracker_integrity.py",),
    ),
    (
        "requirement traceability check",
        ("python3 scripts/check_requirement_traceability.py",),
        ("python3 scripts/check_requirement_traceability.py",),
    ),
    (
        "wire-format authority check",
        ("python3 scripts/check_wire_format_authority.py",),
        ("python3 scripts/check_wire_format_authority.py",),
    ),
    (
        "active Path 2 Rust crate checks",
        (
            'RUSTFLAGS="-D warnings" cargo check --manifest-path tools/descender-mask-prf/Cargo.toml --all-targets',
            'RUSTFLAGS="-D warnings" cargo check --manifest-path tools/descender-account-stark/Cargo.toml --all-targets',
            'RUSTFLAGS="-D warnings" cargo check --manifest-path tools/descender-account-verifier/Cargo.toml --all-targets',
            'RUSTFLAGS="-D warnings" cargo check --manifest-path tools/descender-recursion/Cargo.toml --all-targets',
        ),
        (
            'RUSTFLAGS="-D warnings" cargo check --manifest-path tools/descender-mask-prf/Cargo.toml --all-targets',
            'RUSTFLAGS="-D warnings" cargo check --manifest-path tools/descender-account-stark/Cargo.toml --all-targets',
            'RUSTFLAGS="-D warnings" cargo check --manifest-path tools/descender-account-verifier/Cargo.toml --all-targets',
            'RUSTFLAGS="-D warnings" cargo check --manifest-path tools/descender-recursion/Cargo.toml --all-targets',
        ),
    ),
    (
        "active Path 2 fast Rust tests",
        ("bash scripts/check_active_rust_fast_tests.sh",),
        ("bash scripts/check_active_rust_fast_tests.sh",),
    ),
    (
        "research automation routing check",
        ("python3 scripts/research/check_research_automation_routing.py",),
        ("python3 scripts/research/check_research_automation_routing.py",),
    ),
    (
        "research queue pivot audit",
        ("python3 scripts/research/audit_task_pivot_alignment.py --strict-queue",),
        ("python3 scripts/research/audit_task_pivot_alignment.py --strict-queue",),
    ),
    (
        "semantic theorem audit",
        ("lake env lean --run scripts/audit_theorems.lean",),
        ("lake env lean --run scripts/audit_theorems.lean",),
    ),
    (
        "Lean theorem statement drift check",
        ("git diff --exit-code audit/lean_theorem_statements.txt",),
        ("git diff --exit-code audit/lean_theorem_statements.txt",),
    ),
    (
        "Rust all-target check",
        ("cargo check --manifest-path tools/parameter-search/Cargo.toml --all-targets",),
        ("cargo check --all-targets",),
    ),
    (
        "account STARK payload KAT boundary manifest check",
        (
            "path2_account_stark_payload_kat_manifest",
            "path2_account_stark_payload_kat_boundary_v1.json",
        ),
        (
            "path2_account_stark_payload_kat_manifest",
            "path2_account_stark_payload_kat_boundary_v1.json",
        ),
    ),
    (
        "account STARK payload KAT vector corpus check",
        (
            "path2_account_stark_payload_kat_vectors",
            "path2_account_stark_payload_kat_vectors_v1.json",
            "account-creation-stark-verifier",
        ),
        (
            "path2_account_stark_payload_kat_vectors",
            "path2_account_stark_payload_kat_vectors_v1.json",
            "account-creation-stark-verifier",
        ),
    ),
    (
        "legacy q12289 negative-control bin tests",
        (
            "cargo test --manifest-path tools/parameter-search/Cargo.toml --bin signature_q12289_local_delta_stateful",
        ),
        ("cargo test --bin signature_q12289_local_delta_stateful",),
    ),
    (
        "legacy KAT v0 byte-conformance",
        (
            "cargo run --manifest-path tools/parameter-search/Cargo.toml --bin signature_q12289_local_delta_stateful -- --kat-check",
        ),
        ("cargo run --bin signature_q12289_local_delta_stateful -- --kat-check",),
    ),
    (
        "estimator/reference constant sync",
        ("python3 scripts/research/check_estimator_param_sync.py",),
        ("python3 scripts/research/check_estimator_param_sync.py",),
    ),
    (
        "QROM budget artifact regeneration",
        (
            "python3 scripts/research/qrom_challenge_budget_memo.py",
            "python3 scripts/research/qrom_security_loss_review.py",
            "python3 scripts/research/pre_third_party_qrom_frame_probe.py",
            "python3 scripts/research/qrom_multitarget_margin.py",
        ),
        (
            "python3 scripts/research/qrom_challenge_budget_memo.py",
            "python3 scripts/research/qrom_security_loss_review.py",
            "python3 scripts/research/pre_third_party_qrom_frame_probe.py",
            "python3 scripts/research/qrom_multitarget_margin.py",
        ),
    ),
    (
        "QROM budget/reference sync",
        ("python3 scripts/research/check_qrom_budget_sync.py",),
        ("python3 scripts/research/check_qrom_budget_sync.py",),
    ),
    (
        "QROM generated artifact drift check",
        ("python3 scripts/research/check_qrom_generated_artifact_drift.py",),
        ("python3 scripts/research/check_qrom_generated_artifact_drift.py",),
    ),
    (
        "Fq12 reference irreducibility check",
        ("python3 scripts/research/verify_fq12_irreducibility.py",),
        ("python3 scripts/research/verify_fq12_irreducibility.py",),
    ),
    (
        "checked-core concrete-security support smoke",
        ("python3 scripts/research/cryptanalysis_concrete_security_estimate.py",),
        ("python3 scripts/research/cryptanalysis_concrete_security_estimate.py",),
    ),
)

GATE_TRACKER_ROWS: dict[str, tuple[str, ...]] = {
    "Lean build": ("PROC-FORMAL-001",),
    "documentation authority check": ("PROC-LINT-001",),
    "ADR immutable-section check": ("PROC-ADR-006",),
    "process gate coverage check": ("PROC-PROCESS-001",),
    "documentation classification inventory": ("PROC-CLASSIFY-010",),
    "archive mixed-status audit": ("PROC-ARCHIVE-004",),
    "archive legacy-count audit": ("PROC-ARCHIVE-005", "PROC-ARCHIVE-006"),
    "cleanup queue check": ("PROC-ARCHIVE-007",),
    "retired path absence check": ("PROC-ARCHIVE-008",),
    "active support import audit": ("PROC-SUPPORT-001",),
    "active authority stale-term check": ("PROC-STALE-001",),
    "legacy code framing check": ("PROC-STALE-002",),
    "tool manifest framing check": ("PROC-MANIFEST-001",),
    "active source framing check": ("PROC-SOURCE-001",),
    "CI/local gate parity check": ("PROC-CI-001",),
    "tracker integrity check": ("PROC-TRACKER-002",),
    "requirement traceability check": ("PROC-REQ-001",),
    "wire-format authority check": ("DIR-CC-001",),
    "active Path 2 Rust crate checks": (
        "DIR-STARK-001",
        "DIR-BIND-001",
        "AGG-ROLLUP-001",
        "ADOPT-HW-ENT-001",
    ),
    "active Path 2 fast Rust tests": ("DIR-TEST-001",),
    "research automation routing check": ("PROC-AUTO-002", "PROC-AUTO-003"),
    "research queue pivot audit": ("PROC-AUTO-002",),
    "semantic theorem audit": ("PROC-FORMAL-001",),
    "Lean theorem statement drift check": ("PROC-FORMAL-001",),
    "Rust all-target check": ("PROC-CI-001",),
    "account STARK payload KAT boundary manifest check": (
        "DIR-KAT-001",
        "DIR-STARK-001",
        "DIR-REJECT-001",
    ),
    "account STARK payload KAT vector corpus check": (
        "DIR-KAT-001",
        "DIR-STARK-001",
        "DIR-REJECT-001",
    ),
    "legacy q12289 negative-control bin tests": ("PROC-LEGACY-GATE-001",),
    "legacy KAT v0 byte-conformance": ("PROC-LEGACY-GATE-001",),
    "estimator/reference constant sync": ("DIR-QROM-001",),
    "QROM budget artifact regeneration": ("DIR-QROM-001",),
    "QROM budget/reference sync": ("DIR-QROM-001",),
    "QROM generated artifact drift check": ("DIR-QROM-001",),
    "Fq12 reference irreducibility check": ("DIR-ACTION-001",),
    "checked-core concrete-security support smoke": ("DIR-ACTION-001",),
}

TRACKER_ITEM_ID = re.compile(r"^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*-\d{3}$")
ACTIVE_TRACKER_STATUSES = {
    "proposed",
    "selected",
    "in_progress",
    "implemented",
    "verified",
    "blocked",
}


def require(text: str, patterns: tuple[str, ...], label: str, path: Path, errors: list[str]) -> None:
    for pattern in patterns:
        if pattern not in text:
            errors.append(f"{path.relative_to(ROOT)}: missing {label}: {pattern}")


def discovered_python_gate_commands(text: str) -> set[str]:
    """Return normalized Python gate commands that call repo scripts."""

    commands: set[str] = set()
    for match in re.finditer(r"python3 scripts/[^\n]+", text):
        commands.add(match.group(0).strip().rstrip())
    return commands


def local_gate_labels(text: str) -> set[str]:
    return {match.group(1).casefold() for match in re.finditer(r'^echo "== (.*?) =="', text, re.MULTILINE)}


def ci_gate_labels(text: str) -> set[str]:
    return {match.group(1).casefold() for match in re.finditer(r"^\s+- name: (.*?)$", text, re.MULTILINE)}


def tracker_item_ids() -> set[str]:
    return set(tracker_item_statuses())


def tracker_item_statuses() -> dict[str, str]:
    statuses: dict[str, str] = {}
    for line in TRACKER.read_text(encoding="utf-8").splitlines():
        if not line.startswith("|"):
            continue
        cells = [cell.strip().strip("`") for cell in line.strip().strip("|").split("|")]
        if len(cells) >= 2 and TRACKER_ITEM_ID.match(cells[0]):
            statuses[cells[0]] = cells[1]
    return statuses


def check_gate_inventory_shape(errors: list[str]) -> None:
    seen: set[str] = set()
    for label, local_patterns, ci_patterns in GATES:
        if label in seen:
            errors.append(
                "scripts/check_ci_local_gate_parity.py: duplicate gate label: "
                f"{label}"
            )
        seen.add(label)
        if not local_patterns:
            errors.append(
                "scripts/check_ci_local_gate_parity.py: gate has no local "
                f"patterns: {label}"
            )
        if not ci_patterns:
            errors.append(
                "scripts/check_ci_local_gate_parity.py: gate has no CI "
                f"patterns: {label}"
            )
        for pattern in (*local_patterns, *ci_patterns):
            if not pattern.strip():
                errors.append(
                    "scripts/check_ci_local_gate_parity.py: gate has an empty "
                    f"pattern: {label}"
                )


def check_gate_tracker_owners(errors: list[str]) -> None:
    gate_labels = {label for label, _local_patterns, _ci_patterns in GATES}
    owner_labels = set(GATE_TRACKER_ROWS)
    missing = sorted(gate_labels - owner_labels)
    extra = sorted(owner_labels - gate_labels)
    if missing:
        errors.append(
            "scripts/check_ci_local_gate_parity.py: gates missing tracker owners: "
            + ", ".join(missing)
        )
    if extra:
        errors.append(
            "scripts/check_ci_local_gate_parity.py: tracker-owner map has "
            "unknown gates: "
            + ", ".join(extra)
        )

    tracker_statuses = tracker_item_statuses()
    for label, owners in GATE_TRACKER_ROWS.items():
        if not owners:
            errors.append(
                "scripts/check_ci_local_gate_parity.py: gate has no tracker "
                f"owner rows: {label}"
            )
            continue
        for owner in owners:
            status = tracker_statuses.get(owner)
            if status is None:
                errors.append(
                    "scripts/check_ci_local_gate_parity.py: gate owner row does "
                    f"not exist for {label}: {owner}"
                )
            elif status not in ACTIVE_TRACKER_STATUSES:
                errors.append(
                    "scripts/check_ci_local_gate_parity.py: gate owner row is "
                    f"not active for {label}: {owner} has status {status!r}"
                )


def check_gate_labels(local: str, ci: str, errors: list[str]) -> None:
    local_labels = local_gate_labels(local)
    ci_labels = ci_gate_labels(ci)
    for label, _local_patterns, _ci_patterns in GATES:
        folded = label.casefold()
        if folded not in local_labels:
            errors.append(
                "scripts/check_build.sh: missing visible gate label: "
                f"{label}"
            )
        if folded not in ci_labels:
            errors.append(
                ".github/workflows/ci.yml: missing visible gate label: "
                f"{label}"
            )


def main() -> int:
    local = LOCAL.read_text(encoding="utf-8")
    ci = CI.read_text(encoding="utf-8")
    errors: list[str] = []
    check_gate_inventory_shape(errors)
    check_gate_labels(local, ci, errors)
    for label, local_patterns, ci_patterns in GATES:
        require(local, local_patterns, label, LOCAL, errors)
        require(ci, ci_patterns, label, CI, errors)
    check_gate_tracker_owners(errors)

    local_python_gates = discovered_python_gate_commands(local)
    ci_python_gates = discovered_python_gate_commands(ci)
    for command in sorted(local_python_gates - ci_python_gates):
        errors.append(
            ".github/workflows/ci.yml: missing local Python gate command: "
            f"{command}"
        )
    for command in sorted(ci_python_gates - local_python_gates):
        errors.append(
            "scripts/check_build.sh: missing CI Python gate command: "
            f"{command}"
        )

    if errors:
        print("CI/local gate parity check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("CI/local gate parity check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
