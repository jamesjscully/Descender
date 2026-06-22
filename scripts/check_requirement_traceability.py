#!/usr/bin/env python3
"""Check that blockchain requirements trace to consolidated tracker rows.

This also guards selected normative spec sections that intentionally describe
unfinished evidence work. Those sections must name the tracker rows that own the
work, so the spec spine does not accumulate shadow TODOs.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SPEC = ROOT / "spec/blockchain_requirements.md"
SECURITY_MODEL = ROOT / "spec/security_model.md"
RUST_PRODUCTION_REFERENCE = ROOT / "tools/parameter-search/src/production_reference.rs"
TRACKER = ROOT / "TRACKER.md"
ETHEREUM_POSITIONING = ROOT / "research/ethereum_account_abstraction_positioning.md"

TRACKER_ITEM_ID = re.compile(r"^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*-\d{3}$")
TRACKER_REF = re.compile(r"`([A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*-\d{3})`")
READINESS_STATUSES = {"not_ready", "partial", "ready"}
OBJECTIVE_PHRASES = [
    "not trying to be only a smaller post-quantum `CHECKSIG`",
    "authorization stack a chain, rollup, bridge, or custody system can adopt",
]
REQUIREMENT_CLASS_CONTRACTS = [
    ("`direct`", "required for Mode A single-slot deployment"),
    ("`mode_b`", "required for aggregation, rollups, committees, or light clients"),
    ("`policy`", "required above the slot abstraction in account/state logic"),
]
NON_CLAIMS_CONTRACTS = [
    "native BLS-style aggregation",
    "Schnorr-style adaptor signatures",
    "threshold signing as one compact native Descender signature",
    "validator finality or bridge safety without explicit account/committee state",
    "hard 128-bit security while the active path is still using the 124-bit Poseidon2/Merkle floor",
    "deployability based only on internal notes or prototype tests",
]
PRODUCT_BAR_LINES = [
    "post-quantum spend authorization",
    "+ account-state lifecycle safety",
    "+ policy expressiveness",
    "+ optional proof aggregation",
    "+ conservative signer/verifier engineering",
    "+ explicit adoption evidence",
]
DIRECT_THEOREM_PREFLIGHT_MARKERS = [
    "PRODUCTION_DEPLOYMENT_BLOCKER_DIRECT_THEOREM_COMPOSITION",
    "PATH2_DIRECT_THEOREM_BOUNDARY_REPORT_DOMAIN",
    "Path2DirectTheoremBoundaryReportV1",
    "PATH2_DIRECT_THEOREM_ACCEPTED_TUPLE_FIELDS",
    "PATH2_DIRECT_THEOREM_ACCEPTANCE_HYPOTHESES",
    "PATH2_DIRECT_THEOREM_NAMED_BREAK_CLASSES",
    "path2_direct_theorem_boundary_report_v1",
    "validate_path2_direct_theorem_boundary_report_v1",
    "production_deployment_direct_theorem_blockers",
]
ACCOUNT_STARK_RESIDUAL_PREFLIGHT_MARKERS = [
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_PAYLOAD_KATS",
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_AIR_REVIEW",
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_FRI_AOK_EXTRACTOR",
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_SIMULATOR",
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_FRI_QROM_RESIDUAL",
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_PRODUCTION_GATE",
    "PRODUCTION_DEPLOYMENT_BLOCKER_ACCOUNT_STARK_LEGACY_ROUTE_ADR",
    "production_deployment_account_stark_residual_blockers",
]
SPEC_OPEN_WORK_SECTIONS = {
    "spec/conformance_kats.md": {
        "Required Evidence": {
            "SPEC-KAT-001",
            "DIR-KAT-001",
            "DIR-LEAF-001",
            "DIR-EVID-001",
            "DIR-SPEND-001",
            "DIR-CC-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
            "SPEC-VERIFY-001",
            "SPEC-DEPLOY-001",
            "POL-REQ-001",
            "ADOPT-HW-API-001",
            "ADOPT-HW-ENT-001",
            "ADOPT-ETH-AA-001",
        },
    },
    "spec/consensus_verifier.md": {
        "Required Evidence": {
            "SPEC-VERIFY-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
            "DIR-CC-001",
            "DIR-LIFE-001",
            "POL-REQ-001",
            "ADOPT-ETH-AA-001",
            "ADOPT-ETH-7932-001",
            "ADOPT-ETH-8141-001",
            "ADOPT-HW-API-001",
        },
    },
    "spec/deployment_profiles.md": {
        "Required Evidence": {
            "SPEC-DEPLOY-001",
            "SPEC-LIFE-001",
            "SPEC-VERIFY-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
            "AGG-DA-001",
            "AGG-LC-001",
            "AGG-COMMITTEE-001",
            "ADOPT-ETH-AA-001",
            "ADOPT-ETH-7702-001",
            "ADOPT-ETH-7932-001",
            "ADOPT-ETH-8141-001",
            "ADOPT-ETH-ROLLUP-001",
            "ADOPT-HW-STATE-001",
            "ADOPT-HW-ENT-001",
            "SPEC-STATE-001",
            "SPEC-ETH-001",
        },
    },
    "spec/ethereum_profiles.md": {
        "Required Evidence": {
            "SPEC-ETH-001",
            "SPEC-DEPLOY-001",
            "SPEC-VERIFY-001",
            "SPEC-KAT-001",
            "SPEC-LIFE-001",
            "SPEC-POLICY-001",
            "SPEC-STATE-001",
            "ADOPT-ETH-001",
            "ADOPT-ETH-AA-001",
            "ADOPT-ETH-7702-001",
            "ADOPT-ETH-7932-001",
            "ADOPT-ETH-8141-001",
            "ADOPT-ETH-ROLLUP-001",
            "DIR-CC-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
            "AGG-ROLLUP-001",
            "AGG-DA-001",
            "AGG-LC-001",
            "POL-REQ-001",
            "POL-RECOVERY-001",
            "ADOPT-HW-STATE-001",
        },
    },
    "spec/entropy_profile.md": {
        "Required Evidence": {
            "ADOPT-HW-ENT-001",
            "SPEC-SIGNER-001",
            "DIR-KAT-001",
            "SPEC-KAT-001",
            "ADOPT-HW-API-001",
            "ADOPT-HW-STATE-001",
            "ADOPT-HW-CT-001",
            "DIR-CT-001",
            "SPEC-DEPLOY-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
        },
    },
    "spec/wire_formats.md": {
        "Path2AccountStarkCreationPayloadV1": {
            "DIR-STARK-001",
            "DIR-BIND-001",
            "DIR-KAT-001",
            "DIR-REJECT-001",
        },
        "Rule For New Wire Formats": {
            "DIR-LEAF-001",
            "DIR-EVID-001",
            "DIR-CC-001",
            "DIR-KAT-001",
            "DIR-REJECT-001",
        },
    },
    "spec/security_model.md": {
        "Direct Theorem Acceptance Boundary": {
            "DIR-SEC-001",
            "DIR-CREATE-001",
            "DIR-HANDLE-001",
            "DIR-CC-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
        },
        "Account-Creation Proof Boundary Contract": {
            "DIR-CREATE-001",
            "DIR-PROOF-001",
            "DIR-PROOF-002",
            "DIR-STARK-001",
            "DIR-BIND-001",
            "DIR-PAYLOAD-001",
            "DIR-AUTHCLASS-001",
            "DIR-KAT-001",
            "DIR-REJECT-001",
            "DIR-CT-001",
            "DIR-QROM-001",
            "DIR-REVIEW-001",
        },
        "Selected Account-STARK Residual Matrix": {
            "DIR-PROOF-001",
            "DIR-PROOF-002",
            "DIR-STARK-001",
            "DIR-BIND-001",
            "DIR-KAT-001",
            "DIR-REJECT-001",
            "DIR-QROM-001",
            "DIR-REVIEW-001",
        },
        "Risk-First Deployment Blocker Order": {
            "DIR-RISK-001",
            "DIR-SEC-001",
            "DIR-CREATE-001",
            "DIR-PROOF-001",
            "DIR-PROOF-002",
            "DIR-STARK-001",
            "DIR-BIND-001",
            "DIR-PAYLOAD-001",
            "DIR-AUTHCLASS-001",
            "DIR-CC-001",
            "DIR-LEAF-001",
            "DIR-EVID-001",
            "DIR-KAT-001",
            "SPEC-KAT-001",
            "DIR-REJECT-001",
            "DIR-CT-001",
            "SPEC-SIGNER-001",
            "SPEC-LEAKAGE-001",
            "ADOPT-HW-001",
            "ADOPT-HW-CT-001",
            "ADOPT-HW-ENT-001",
            "ADOPT-HW-STATE-001",
            "DIR-LIFE-001",
            "SPEC-LIFE-001",
            "SPEC-STATE-001",
            "SPEC-DEPLOY-001",
            "DIR-REVIEW-001",
            "SPEC-REVIEW-001",
            "DIR-QROM-001",
            "DIR-ACTION-001",
        },
        "Required Evidence Before Deployment Claim": {
            "DIR-RISK-001",
            "DIR-SEC-001",
            "DIR-CREATE-001",
            "DIR-PROOF-001",
            "DIR-PROOF-002",
            "DIR-AUTHCLASS-001",
            "DIR-KAT-001",
            "SPEC-KAT-001",
            "DIR-REJECT-001",
            "SPEC-SIGNER-001",
            "SPEC-LEAKAGE-001",
            "DIR-CT-001",
            "DIR-QROM-001",
            "DIR-REVIEW-001",
            "DIR-LIFE-001",
            "SPEC-DEPLOY-001",
            "SPEC-REVIEW-001",
        },
    },
    "spec/review_package.md": {
        "Required Evidence": {
            "SPEC-REVIEW-001",
            "DIR-REVIEW-001",
            "DIR-SEC-001",
            "DIR-QROM-001",
            "DIR-ACTION-001",
            "DIR-KAT-001",
            "DIR-REJECT-001",
            "DIR-LIFE-001",
            "DIR-CT-001",
            "SPEC-DEPLOY-001",
            "SPEC-SIGNER-001",
            "SPEC-LEAKAGE-001",
            "AGG-MODE-001",
            "AGG-ROLLUP-001",
            "AGG-COMMITTEE-001",
            "POL-REQ-001",
            "ADOPT-HW-ENT-001",
            "SPEC-STATE-001",
            "SPEC-ETH-001",
            "ADOPT-128-001",
        },
    },
    "spec/leakage_profile.md": {
        "Required Evidence": {
            "SPEC-LEAKAGE-001",
            "DIR-CT-001",
            "ADOPT-HW-CT-001",
            "SPEC-SIGNER-001",
            "ADOPT-HW-API-001",
            "ADOPT-HW-ENT-001",
            "ADOPT-HW-STATE-001",
            "DIR-KAT-001",
            "SPEC-KAT-001",
            "SPEC-DEPLOY-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
            "DIR-REVIEW-001",
        },
    },
    "spec/signer_state_profile.md": {
        "Required Evidence": {
            "SPEC-STATE-001",
            "ADOPT-HW-STATE-001",
            "SPEC-LIFE-001",
            "DIR-LIFE-001",
            "SPEC-SIGNER-001",
            "ADOPT-HW-API-001",
            "SPEC-DEPLOY-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
            "SPEC-KAT-001",
            "SPEC-LEAKAGE-001",
            "POL-RECOVERY-001",
            "DIR-REVIEW-001",
        },
    },
    "spec/signer_api.md": {
        "Required Evidence": {
            "ADOPT-HW-API-001",
            "SPEC-SIGNER-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
            "SPEC-KAT-001",
            "SPEC-VERIFY-001",
            "SPEC-DEPLOY-001",
            "DIR-LIFE-001",
            "ADOPT-HW-STATE-001",
            "ADOPT-HW-ENT-001",
            "ADOPT-HW-CT-001",
        },
    },
    "spec/lifecycle_model.md": {
        "Required Evidence": {
            "DIR-LIFE-001",
            "SPEC-DEPLOY-001",
            "SPEC-STATE-001",
            "ADOPT-HW-STATE-001",
            "AGG-ORDER-001",
            "AGG-DA-001",
        },
    },
    "spec/policy_model.md": {
        "Required Evidence": {
            "SPEC-POLICY-001",
            "POL-REQ-001",
            "POL-MSIG-001",
            "POL-THRESH-001",
            "POL-RECOVERY-001",
            "POL-ADAPTOR-001",
            "AGG-COMMITTEE-001",
            "DIR-LIFE-001",
            "DIR-REJECT-001",
            "DIR-KAT-001",
            "ADOPT-HW-STATE-001",
        },
    },
    "spec/signer_profile.md": {
        "Wallet / HSM API Shape": {
            "SPEC-SIGNER-001",
            "ADOPT-HW-API-001",
            "DIR-REJECT-001",
        },
        "Entropy And Domain Separation": {
            "SPEC-SIGNER-001",
            "ADOPT-HW-ENT-001",
            "DIR-KAT-001",
        },
        "Constant-Time And Leakage Evidence": {
            "SPEC-SIGNER-001",
            "DIR-CT-001",
            "ADOPT-HW-CT-001",
        },
        "Fault And Recovery Requirements": {
            "SPEC-SIGNER-001",
            "DIR-LIFE-001",
            "ADOPT-HW-STATE-001",
        },
    },
}
POLICY_REQUIREMENT_REFS = [
    ("policy domain separator and version", {"POL-REQ-001"}),
    ("account namespace", {"POL-REQ-001", "DIR-CC-001"}),
    (
        "signer/guardian/committee set representation",
        {"POL-MSIG-001", "POL-THRESH-001", "AGG-COMMITTEE-001"},
    ),
    ("threshold or weight rule", {"POL-THRESH-001", "AGG-COMMITTEE-001"}),
    (
        "timelock, recovery, or rotation semantics",
        {"POL-RECOVERY-001", "ADOPT-HW-STATE-001"},
    ),
    ("replay and duplicate-prevention rule", {"DIR-LIFE-001", "AGG-ORDER-001"}),
    ("lifecycle interaction with consumed slots", {"DIR-LIFE-001", "POL-REQ-001"}),
    (
        "state root fields bound into the spend or aggregate proof",
        {"DIR-CC-001", "POL-REQ-001", "AGG-LC-001"},
    ),
]
DIRECT_MODE_GATE_CONTRACTS = {
    1: (
        "final direct-mode theorem statement",
        {"DIR-RISK-001", "DIR-SEC-001", "DIR-QROM-001", "DIR-ACTION-001"},
    ),
    2: (
        "public wire formats are fixed and versioned",
        {"DIR-LEAF-001", "DIR-EVID-001", "DIR-CC-001"},
    ),
    3: (
        "every public field has positive and negative KAT coverage",
        {"DIR-KAT-001", "DIR-CC-001"},
    ),
    4: (
        "direct creation and direct spend have end-to-end vectors",
        {"DIR-KAT-001"},
    ),
    5: (
        "signer/keygen/sampler behavior satisfies",
        {"SPEC-SIGNER-001", "DIR-CT-001", "ADOPT-HW-001"},
    ),
    6: (
        "verifier rejection is deterministic, panic-free, and consensus-safe",
        {"DIR-REJECT-001", "DIR-CC-001"},
    ),
    7: (
        "lifecycle/reorg/finality policy is specified",
        {"SPEC-LIFE-001", "DIR-LIFE-001"},
    ),
    8: (
        "security level is stated honestly",
        {"DIR-QROM-001", "ADOPT-128-001"},
    ),
    9: (
        "third-party cryptanalysis has reviewed",
        {"DIR-REVIEW-001"},
    ),
}
MODE_B_GATE_CONTRACTS = {
    1: (
        "batch or recursive proof system is named and parameterized",
        {"AGG-BATCH-001", "AGG-ROLLUP-001"},
    ),
    2: (
        "aggregate statement format is fixed",
        {"AGG-SPEND-001", "AGG-LC-001"},
    ),
    3: (
        "canonical batch ordering and duplicate rejection",
        {"AGG-ORDER-001"},
    ),
    4: (
        "data availability rules exist",
        {"AGG-DA-001"},
    ),
    5: (
        "aggregate consume-once semantics",
        {"AGG-SPEND-001", "DIR-LIFE-001"},
    ),
    6: (
        "light-client proof semantics are specified",
        {"AGG-LC-001"},
    ),
    7: (
        "proof-system soundness, Fiat-Shamir/QROM assumptions",
        {"AGG-ROLLUP-001", "DIR-QROM-001", "DIR-SEC-001"},
    ),
}
CAPABILITY_MATRIX_CONTRACTS = [
    (
        "Simple transaction authorization",
        "`direct`",
        "checked-core spend",
        "required first deployable path",
        {"DIR-SPEND-001", "DIR-CC-001", "DIR-SEC-001"},
    ),
    (
        "One-use authorization lifecycle",
        "`direct`",
        "live slot registry",
        "target-chain reorg/finality profile still open",
        {"SPEC-LIFE-001", "DIR-LIFE-001"},
    ),
    (
        "Canonical public wire objects",
        "`direct`",
        "PrevalidatedSlotLeafV1",
        "KAT corpus still required",
        {"DIR-LEAF-001", "DIR-EVID-001", "DIR-CC-001", "DIR-KAT-001"},
    ),
    (
        "Deterministic consensus verification",
        "`direct`",
        "canonical parse",
        "required before deployment claim",
        {"DIR-REJECT-001", "DIR-CC-001", "DIR-KAT-001"},
    ),
    (
        "Hardware-wallet/HSM safety",
        "`direct`",
        "no floating point",
        "required before deployment claim",
        {"SPEC-SIGNER-001", "DIR-CT-001", "ADOPT-HW-001"},
    ),
    (
        "M-of-N multisig",
        "`policy`",
        "account policy over multiple admitted slots",
        "not native signature aggregation",
        {"POL-MSIG-001", "POL-REQ-001"},
    ),
    (
        "Threshold custody",
        "`policy` or `mode_b`",
        "true threshold construction or proof-compressed M-of-N",
        "open design decision",
        {"POL-THRESH-001", "AGG-COMMITTEE-001"},
    ),
    (
        "Validator quorum authorization",
        "`policy` and `mode_b`",
        "weighted signer set",
        "not a BLS replacement claim",
        {"POL-REQ-001", "AGG-COMMITTEE-001"},
    ),
    (
        "Bridge committee authorization",
        "`policy` and `mode_b`",
        "committee state root",
        "requires Mode B proof semantics",
        {"AGG-COMMITTEE-001", "AGG-DA-001", "AGG-LC-001"},
    ),
    (
        "Account recovery",
        "`policy`",
        "guardian/timelock/rotation rules",
        "must be explicit state policy",
        {"POL-RECOVERY-001", "ADOPT-HW-STATE-001"},
    ),
    (
        "Adaptor-style workflows",
        "`policy`",
        "state/protocol conditions",
        "not native Schnorr-style adaptor algebra",
        {"POL-ADAPTOR-001", "POL-REQ-001"},
    ),
    (
        "Batch verification",
        "`mode_b`",
        "batch account creation or batch spend proof",
        "optional after direct theorem closes",
        {"AGG-BATCH-001", "AGG-SPEND-001", "AGG-ORDER-001"},
    ),
    (
        "Signature/proof aggregation",
        "`mode_b`",
        "proof/state aggregation",
        "not native signature combine",
        {"AGG-MODE-001", "AGG-ROLLUP-001", "AGG-SPEND-001"},
    ),
    (
        "Rollup compression",
        "`mode_b`",
        "recursive proof",
        "concrete recursion proof still under construction",
        {"AGG-ROLLUP-001", "ADOPT-ETH-ROLLUP-001"},
    ),
    (
        "Light-client proofs",
        "`mode_b`",
        "compact state/proof handle",
        "needs final aggregate proof format",
        {"AGG-LC-001", "AGG-DA-001"},
    ),
]
SPEC_BASELINE_SURFACE_CONTRACTS = [
    (
        "Normative spec spine",
        "`direct`",
        "`partial`",
        "indexed current spec spine",
        {"SPEC-DIR-001", "DIR-RISK-001", "DIR-SEC-001"},
    ),
    (
        "Blockchain requirements spine",
        "`direct`",
        "`partial`",
        "capability matrix",
        {"SPEC-REQ-001", "DIR-KAT-001", "DIR-REVIEW-001"},
    ),
    (
        "Conformance and KAT corpus model",
        "`direct`",
        "`partial`",
        "active Path 2 corpus identity",
        {"SPEC-KAT-001", "DIR-KAT-001", "DIR-REJECT-001"},
    ),
    (
        "Consensus verifier model",
        "`direct`",
        "`partial`",
        "public verifier pipeline",
        {"SPEC-VERIFY-001", "DIR-REJECT-001", "DIR-KAT-001"},
    ),
    (
        "Deployment profile model",
        "`direct`",
        "`partial`",
        "target-chain, rollup, bridge, Ethereum",
        {"SPEC-DEPLOY-001", "DIR-LIFE-001", "DIR-REJECT-001"},
    ),
    (
        "Ethereum adoption profile model",
        "`direct`",
        "`partial`",
        "Ethereum account-abstraction",
        {"SPEC-ETH-001", "ADOPT-ETH-001", "SPEC-DEPLOY-001"},
    ),
    (
        "Entropy and stream profile model",
        "`direct`",
        "`partial`",
        "entropy-source classes",
        {"ADOPT-HW-ENT-001", "SPEC-SIGNER-001", "DIR-KAT-001"},
    ),
    (
        "Constant-time and leakage evidence model",
        "`direct`",
        "`partial`",
        "secret/public classification",
        {"SPEC-LEAKAGE-001", "DIR-CT-001", "ADOPT-HW-CT-001"},
    ),
    (
        "Signer public API model",
        "`direct`",
        "`partial`",
        "public command/result classes",
        {"ADOPT-HW-API-001", "SPEC-SIGNER-001", "DIR-REJECT-001"},
    ),
    (
        "Signer state and fault model",
        "`direct`",
        "`partial`",
        "local slot records",
        {"SPEC-STATE-001", "ADOPT-HW-STATE-001", "DIR-LIFE-001"},
    ),
    (
        "Signer and custody profile",
        "`direct`",
        "`partial`",
        "signer, keygen, sampler, wallet, and HSM requirements",
        {"SPEC-SIGNER-001", "DIR-CT-001", "ADOPT-HW-001"},
    ),
    (
        "Lifecycle and state model",
        "`direct`",
        "`partial`",
        "consume-once lifecycle",
        {"SPEC-LIFE-001", "DIR-LIFE-001", "ADOPT-HW-STATE-001"},
    ),
    (
        "Policy model",
        "`policy`",
        "`partial`",
        "policy object boundary",
        {"SPEC-POLICY-001", "POL-REQ-001", "DIR-LIFE-001"},
    ),
    (
        "External review package model",
        "`direct`",
        "`partial`",
        "review lanes, manifest",
        {"SPEC-REVIEW-001", "DIR-REVIEW-001", "DIR-SEC-001"},
    ),
]
DIRECT_IMPLEMENTATION_SURFACE_CONTRACTS = [
    (
        "Normative direct-mode spine",
        "`direct`",
        "`partial`",
        "final direct-mode theorem",
        {"SPEC-DIR-001", "DIR-RISK-001", "DIR-SEC-001"},
    ),
    (
        "Risk-first deployment blocker ledger",
        "`direct`",
        "`partial`",
        "ranked blocker order",
        {"DIR-RISK-001", "DIR-SEC-001", "DIR-PROOF-002"},
    ),
    (
        "Slot leaf and creation evidence frames",
        "`direct`",
        "`partial`",
        "canonical frame bytes",
        {"DIR-LEAF-001", "DIR-EVID-001", "DIR-KAT-001", "DIR-REJECT-001"},
    ),
    (
        "Account-creation STARK API",
        "`direct`",
        "`partial`",
        "account-creation prover/verifier API",
        {"DIR-STARK-001", "DIR-BIND-001", "DIR-KAT-001", "DIR-REJECT-001"},
    ),
    (
        "Spend authorization relation",
        "`direct`",
        "`partial`",
        "spend relation connected to Rust evidence",
        {"DIR-SPEND-001", "DIR-CC-001", "DIR-LIFE-001", "DIR-KAT-001"},
    ),
    (
        "Slot authorization handle",
        "`direct`",
        "`not_ready`",
        "normative wire/state text",
        {"DIR-HANDLE-001", "DIR-CC-001", "DIR-KAT-001"},
    ),
    (
        "Account-creation theorem boundary",
        "`direct`",
        "`partial`",
        "direct creation theorem boundary",
        {
            "DIR-CREATE-001",
            "DIR-SEC-001",
            "DIR-PROOF-001",
            "DIR-PROOF-002",
            "DIR-STARK-001",
            "DIR-BIND-001",
            "DIR-CT-001",
        },
    ),
    (
        "Account-creation negative controls",
        "`direct`",
        "`not_ready`",
        "executable negative KATs",
        {"DIR-CREATE-NEG-001", "DIR-REJECT-001", "DIR-KAT-001"},
    ),
    (
        "Checked-core frame and proof support",
        "`direct`",
        "`partial`",
        "checked-core frame binding",
        {"DIR-CC-001", "DIR-CC-002", "DIR-QROM-002", "DIR-KAT-001"},
    ),
    (
        "Consume-once lifecycle transition",
        "`direct`",
        "`not_ready`",
        "target-chain reorg/finality profile",
        {"DIR-LIFE-001", "SPEC-LIFE-001", "ADOPT-HW-STATE-001"},
    ),
    (
        "Account-creation proof family",
        "`direct`",
        "`not_ready`",
        "selected account-STARK proof family",
        {
            "DIR-PROOF-001",
            "DIR-PROOF-002",
            "DIR-STARK-001",
            "DIR-BIND-001",
            "DIR-REJECT-001",
        },
    ),
    (
        "Creation payload and fiber privacy",
        "`direct`",
        "`not_ready`",
        "concrete payload theorem",
        {"DIR-PAYLOAD-001", "DIR-PROOF-001", "DIR-CT-001"},
    ),
    (
        "Masked-trace relation stack",
        "`direct`",
        "`not_ready`",
        "trace relation",
        {"DIR-TRACE-001", "DIR-PROOF-001", "DIR-KAT-001"},
    ),
    (
        "Descent/SIS proof-route support",
        "`direct`",
        "`not_ready`",
        "descent/SIS support remains research",
        {"DIR-DESCENT-001", "DIR-PROOF-002"},
    ),
    (
        "Native-boundary model",
        "`direct`",
        "`not_ready`",
        "normative object/projection model",
        {"DIR-NB-001", "DIR-CC-002", "DIR-KAT-001"},
    ),
    (
        "Native-boundary knowledge stack",
        "`direct`",
        "`not_ready`",
        "parent-trace extraction",
        {"DIR-NB-002", "DIR-PROOF-001", "DIR-PROOF-002"},
    ),
    (
        "Native-boundary row and range model",
        "`direct`",
        "`not_ready`",
        "row-cost",
        {"DIR-NB-003", "DIR-CT-001"},
    ),
    (
        "Native-boundary implementation cross-check",
        "`direct`",
        "`not_ready`",
        "Rust/Lean assembly checks",
        {"DIR-NB-004", "DIR-KAT-001", "DIR-REJECT-001"},
    ),
    (
        "Authorization-class binding",
        "`direct`",
        "`not_ready`",
        "private authorization-local material cannot change public spend semantics",
        {"DIR-AUTHCLASS-001", "DIR-SEC-001"},
    ),
    (
        "Hash/QROM and action-map budget",
        "`direct`",
        "`partial`",
        "final theorem-linked QROM ledger",
        {"DIR-QROM-001", "DIR-QROM-002", "DIR-ACTION-001", "DIR-REVIEW-001"},
    ),
    (
        "Conformance and KAT corpus",
        "`direct`",
        "`partial`",
        "active Path 2 corpus manifest",
        {"SPEC-KAT-001", "DIR-KAT-001", "DIR-REJECT-001", "DIR-CC-001"},
    ),
    (
        "Active test gates",
        "`direct`",
        "`partial`",
        "fast active Rust gate",
        {"DIR-TEST-001", "DIR-SLOWTEST-001"},
    ),
    (
        "Consensus-safe rejection",
        "`direct`",
        "`partial`",
        "deterministic public errors",
        {"DIR-REJECT-001", "DIR-KAT-001", "SPEC-VERIFY-001"},
    ),
    (
        "Constant-time signer evidence",
        "`direct`",
        "`not_ready`",
        "leakage-harness",
        {"DIR-CT-001", "SPEC-SIGNER-001", "ADOPT-HW-CT-001", "SPEC-LEAKAGE-001"},
    ),
    (
        "External review package",
        "`direct`",
        "`partial`",
        "external cryptanalysis package",
        {"DIR-REVIEW-001", "DIR-SEC-001", "SPEC-REVIEW-001"},
    ),
]
AGGREGATION_SURFACE_CONTRACTS = [
    (
        "Mode B theorem boundary",
        "`mode_b`",
        "`partial`",
        "direct mode does not depend on aggregation",
        {"AGG-MODE-001", "DIR-SEC-001"},
    ),
    (
        "Batch account creation API",
        "`mode_b`",
        "`partial`",
        "targeted tests and aggregate statement binding",
        {"AGG-BATCH-001", "AGG-ORDER-001"},
    ),
    (
        "Batch spend verification",
        "`mode_b`",
        "`not_ready`",
        "duplicate-slot rejection",
        {"AGG-SPEND-001", "DIR-LIFE-001", "AGG-ORDER-001"},
    ),
    (
        "Canonical ordering and duplicates",
        "`mode_b`",
        "`not_ready`",
        "ordering keys",
        {"AGG-ORDER-001", "DIR-REJECT-001"},
    ),
    (
        "Data availability rules",
        "`mode_b`",
        "`not_ready`",
        "consensus state, calldata, DA blobs, or proof public inputs",
        {"AGG-DA-001", "AGG-LC-001"},
    ),
    (
        "Recursive account-proof verifier",
        "`mode_b`",
        "`partial`",
        "concrete recursion proof",
        {"AGG-ROLLUP-001", "DIR-QROM-001"},
    ),
    (
        "Recursion challenger gadget",
        "`mode_b`",
        "`partial`",
        "challenger gadget KATs",
        {"AGG-ROLLUP-002", "DIR-KAT-001"},
    ),
    (
        "Light-client aggregate proof",
        "`mode_b`",
        "`not_ready`",
        "compact proof/state handle",
        {"AGG-LC-001", "AGG-DA-001"},
    ),
    (
        "Bridge or validator committee proof",
        "`mode_b`",
        "`not_ready`",
        "signer set, weights, bitmap/root, quorum rule",
        {"AGG-COMMITTEE-001", "POL-THRESH-001", "POL-REQ-001"},
    ),
]
SECURITY_LEVEL_SURFACE_CONTRACTS = [
    (
        "Hard-128 profile decision",
        "`direct`",
        "`not_ready`",
        "124-bit practical floor",
        {"ADOPT-128-001", "DIR-QROM-001", "DIR-REVIEW-001"},
    ),
]
ETHEREUM_SURFACE_CONTRACTS = [
    (
        "Positioning baseline",
        "`policy`",
        "`partial`",
        "normative Ethereum profile split",
        {"ADOPT-ETH-001", "SPEC-ETH-001"},
    ),
    (
        "ERC-4337 smart-account verifier",
        "`direct`",
        "`partial`",
        "profile ABI/envelope exists",
        {"ADOPT-ETH-AA-001", "SPEC-ETH-001", "DIR-REJECT-001", "DIR-KAT-001"},
    ),
    (
        "EIP-7702 delegated EOA migration",
        "`policy`",
        "`partial`",
        "migration profile exists",
        {"ADOPT-ETH-7702-001", "SPEC-ETH-001", "POL-RECOVERY-001", "ADOPT-HW-STATE-001"},
    ),
    (
        "Secondary-signature/precompile profile",
        "`direct`",
        "`partial`",
        "registry/precompile profile exists",
        {"ADOPT-ETH-7932-001", "SPEC-ETH-001", "DIR-CC-001", "DIR-KAT-001"},
    ),
    (
        "Native frame-transaction validation",
        "`direct`",
        "`partial`",
        "frame-validation profile exists",
        {"ADOPT-ETH-8141-001", "SPEC-ETH-001", "DIR-REJECT-001", "DIR-LIFE-001"},
    ),
    (
        "Rollup aggregate proof handle",
        "`mode_b`",
        "`partial`",
        "rollup handle profile exists",
        {"ADOPT-ETH-ROLLUP-001", "SPEC-ETH-001", "AGG-ROLLUP-001", "AGG-DA-001", "AGG-LC-001"},
    ),
]
HARDWARE_SURFACE_CONTRACTS = [
    (
        "Signer profile baseline",
        "`direct`",
        "`partial`",
        "normative signer/custody profile",
        {"ADOPT-HW-001", "SPEC-SIGNER-001"},
    ),
    (
        "Wallet/HSM API and public errors",
        "`direct`",
        "`partial`",
        "public command/result model exists",
        {"ADOPT-HW-API-001", "DIR-REJECT-001", "SPEC-SIGNER-001"},
    ),
    (
        "Entropy and domain separation",
        "`direct`",
        "`partial`",
        "stream model and mask-PRF drift evidence exist",
        {"ADOPT-HW-ENT-001", "SPEC-SIGNER-001", "DIR-KAT-001"},
    ),
    (
        "Secret-side leakage harness",
        "`direct`",
        "`not_ready`",
        "fixed-shape leakage tests",
        {"ADOPT-HW-CT-001", "DIR-CT-001", "SPEC-SIGNER-001", "SPEC-LEAKAGE-001"},
    ),
    (
        "Rollback and fault state model",
        "`direct`",
        "`partial`",
        "signer state/fault model exists",
        {"ADOPT-HW-STATE-001", "DIR-LIFE-001", "SPEC-LIFE-001", "SPEC-STATE-001"},
    ),
]
POLICY_SURFACE_CONTRACTS = [
    (
        "Policy object baseline",
        "`policy`",
        "`partial`",
        "domain separator, version, and account namespace",
        {"POL-REQ-001", "DIR-CC-001"},
    ),
    (
        "M-of-N account policy",
        "`policy`",
        "`not_ready`",
        "signer set representation",
        {"POL-MSIG-001", "POL-REQ-001", "DIR-LIFE-001"},
    ),
    (
        "Threshold custody route",
        "`policy`",
        "`not_ready`",
        "true threshold signing and proof-compressed M-of-N",
        {"POL-THRESH-001", "AGG-COMMITTEE-001"},
    ),
    (
        "Account recovery policy",
        "`policy`",
        "`not_ready`",
        "guardian, timelock, rotation",
        {"POL-RECOVERY-001", "ADOPT-HW-STATE-001", "DIR-LIFE-001"},
    ),
    (
        "Adaptor-style workflow policy",
        "`policy`",
        "`not_ready`",
        "without claiming native Schnorr-style adaptor algebra",
        {"POL-ADAPTOR-001", "POL-REQ-001", "DIR-LIFE-001"},
    ),
]


def split_table_row(line: str) -> list[str]:
    return [cell.strip() for cell in line.strip().strip("|").split("|")]


def is_separator_row(cells: list[str]) -> bool:
    return bool(cells) and all(set(cell) <= {"-", ":", " "} for cell in cells)


def tracker_statuses() -> dict[str, str]:
    statuses: dict[str, str] = {}
    for line in TRACKER.read_text(encoding="utf-8").splitlines():
        if not line.startswith("|"):
            continue
        cells = split_table_row(line)
        if cells and TRACKER_ITEM_ID.match(cells[0]):
            statuses[cells[0]] = cells[1] if len(cells) > 1 else ""
    return statuses


def section(text: str, heading: str, errors: list[str]) -> str:
    pattern = re.compile(rf"^## {re.escape(heading)}\s*$", re.MULTILINE)
    match = pattern.search(text)
    if match is None:
        errors.append(f"spec/blockchain_requirements.md: missing section ## {heading}")
        return ""
    next_heading = re.search(r"^## ", text[match.end() :], re.MULTILINE)
    if next_heading is None:
        return text[match.end() :]
    return text[match.end() : match.end() + next_heading.start()]


def section_from_source(source: str, text: str, heading: str, errors: list[str]) -> str:
    pattern = re.compile(rf"^## {re.escape(heading)}\s*$", re.MULTILINE)
    match = pattern.search(text)
    if match is None:
        errors.append(f"{source}: missing section ## {heading}")
        return ""
    next_heading = re.search(r"^## ", text[match.end() :], re.MULTILINE)
    if next_heading is None:
        return text[match.end() :]
    return text[match.end() : match.end() + next_heading.start()]


def table_rows(block: str) -> list[list[str]]:
    rows: list[list[str]] = []
    for line in block.splitlines():
        if not line.startswith("|"):
            continue
        cells = split_table_row(line)
        if is_separator_row(cells):
            continue
        rows.append(cells)
    return rows


def bullet_items(block: str) -> list[str]:
    items: list[str] = []
    current: list[str] | None = None
    for line in block.splitlines():
        if line.startswith("- "):
            if current is not None:
                items.append(" ".join(current).rstrip(" ;."))
            current = [line[2:].strip()]
        elif current is not None and line.startswith("  "):
            current.append(line.strip())
        elif current is not None and not line.strip():
            continue
        elif current is not None:
            items.append(" ".join(current).rstrip(" ;."))
            current = None
    if current is not None:
        items.append(" ".join(current).rstrip(" ;."))
    return items


def normalized_text(text: str) -> str:
    return re.sub(r"\s+", " ", text).strip()


def check_refs(
    label: str,
    cell: str,
    tracker_rows: dict[str, str],
    errors: list[str],
) -> list[str]:
    refs = TRACKER_REF.findall(cell)
    if not refs:
        errors.append(f"{label}: missing backticked TRACKER.md row reference")
        return []
    for ref in refs:
        if ref not in tracker_rows:
            errors.append(f"{label}: unknown TRACKER.md row reference {ref}")
    return refs


def check_project_framing(text: str, errors: list[str]) -> None:
    for phrase in OBJECTIVE_PHRASES:
        if phrase not in text:
            errors.append(
                "Project framing: missing chain-native authorization phrase "
                f"{phrase!r}"
            )


def check_requirement_classes(text: str, errors: list[str]) -> None:
    rows = table_rows(section(text, "Requirement Classes", errors))
    if not rows:
        errors.append("Requirement Classes: missing table")
        return
    if rows[0] != ["Class", "Meaning"]:
        errors.append("Requirement Classes: header must be ['Class', 'Meaning']")
        return
    data_rows = rows[1:]
    expected_rows = [list(row) for row in REQUIREMENT_CLASS_CONTRACTS]
    if data_rows != expected_rows:
        errors.append(
            "Requirement Classes: rows must exactly match "
            f"{expected_rows!r}"
        )

    block = section(text, "Requirement Classes", errors)
    normalized_block = normalized_text(block)
    required_phrases = [
        "Direct requirements must not depend on `optional_aggregation_soundness`.",
        "Mode B requirements may depend on a named proof system, accumulator, or recursive wrapper.",
        "Policy requirements may depend on application/account state",
        "bind to the same `slot_authorization_digest`",
        "`TRACKER.md` rows that carry the corresponding work",
        "A requirement without a tracker row is not allowed to remain only prose.",
    ]
    for phrase in required_phrases:
        if phrase not in normalized_block:
            errors.append(f"Requirement Classes: missing policy phrase {phrase!r}")


def check_non_claims(text: str, errors: list[str]) -> None:
    block = section(text, "Non-Claims", errors)
    if "Descender must not imply these are solved by the base signature primitive:" not in block:
        errors.append("Non-Claims: missing base-signature non-claim framing")
    items = bullet_items(block)
    if items != NON_CLAIMS_CONTRACTS:
        errors.append(
            "Non-Claims: bullets must exactly match "
            f"{NON_CLAIMS_CONTRACTS!r}; got {items!r}"
        )


def check_product_bar(text: str, errors: list[str]) -> None:
    block = section(text, "Product Bar", errors)
    normalized_block = normalized_text(block)
    match = re.search(r"```text\n(.*?)\n```", block, re.DOTALL)
    if match is None:
        errors.append("Product Bar: missing text code block")
        return
    lines = match.group(1).splitlines()
    if lines != PRODUCT_BAR_LINES:
        errors.append(
            "Product Bar: target lines must exactly match "
            f"{PRODUCT_BAR_LINES!r}; got {lines!r}"
        )
    required_phrases = [
        "higher than \"a PQ signature verifies.\"",
        "necessary for a serious chain adopter",
        "belongs in `TRACKER.md` with a status below `verified`",
        "not in marketing language",
    ]
    for phrase in required_phrases:
        if phrase not in normalized_block:
            errors.append(f"Product Bar: missing policy phrase {phrase!r}")


def check_readiness_semantics(text: str, errors: list[str]) -> None:
    rows = table_rows(section(text, "Readiness Semantics", errors))
    if not rows:
        errors.append("Readiness Semantics: missing table")
        return

    header = rows[0]
    if header != ["Status", "Meaning"]:
        errors.append("Readiness Semantics: header must be ['Status', 'Meaning']")
        return

    seen: dict[str, str] = {}
    for row in rows[1:]:
        if len(row) != 2:
            errors.append(
                f"Readiness Semantics row {row[0]!r}: expected Status, Meaning"
            )
            continue
        status = row[0].strip("`")
        meaning = row[1]
        if status in seen:
            errors.append(f"Readiness Semantics: duplicate status {row[0]!r}")
        seen[status] = meaning
        if not meaning:
            errors.append(f"Readiness Semantics row {row[0]!r}: empty meaning")

    missing = sorted(READINESS_STATUSES - set(seen))
    extra = sorted(set(seen) - READINESS_STATUSES)
    if missing:
        errors.append(f"Readiness Semantics: missing statuses {missing}")
    if extra:
        errors.append(f"Readiness Semantics: unknown statuses {extra}")

    for status in ("not_ready", "partial"):
        meaning = seen.get(status, "").lower()
        if "not adoption evidence" not in meaning:
            errors.append(
                f"Readiness Semantics row `{status}`: must say it is not adoption evidence"
            )

    ready_meaning = seen.get("ready", "")
    if (
        "cited tracker rows" not in ready_meaning.lower()
        or "`verified`" not in ready_meaning
    ):
        errors.append(
            "Readiness Semantics row `ready`: must require cited tracker rows to be `verified`"
        )


def check_capability_matrix(
    text: str,
    tracker_rows: dict[str, str],
    errors: list[str],
) -> None:
    rows = table_rows(section(text, "Capability Matrix", errors))
    if not rows:
        errors.append("Capability Matrix: missing table")
        return

    header = rows[0]
    expected_header = [
        "Capability",
        "Required class",
        "Descender surface",
        "Current stance",
        "Tracker rows",
    ]
    if header != expected_header:
        errors.append(f"Capability Matrix: header must be {expected_header!r}")
        return

    data_rows = rows[1:]
    if not data_rows:
        errors.append("Capability Matrix: missing data rows")
        return
    if len(data_rows) != len(CAPABILITY_MATRIX_CONTRACTS):
        errors.append(
            "Capability Matrix: "
            f"{len(data_rows)} rows for {len(CAPABILITY_MATRIX_CONTRACTS)} expected capabilities"
        )

    seen_capabilities: list[str] = []
    for index, row in enumerate(data_rows):
        if len(row) != len(header):
            errors.append(
                f"Capability Matrix row {row[0]!r}: "
                f"expected {len(header)} cells, got {len(row)}"
            )
            continue
        capability, required_class, surface, stance, tracker_cell = row
        seen_capabilities.append(capability)
        if index >= len(CAPABILITY_MATRIX_CONTRACTS):
            errors.append(f"Capability Matrix row {capability!r}: unexpected extra row")
            check_refs(f"Capability Matrix row {capability!r}", tracker_cell, tracker_rows, errors)
            continue
        (
            expected_capability,
            expected_class,
            expected_surface_phrase,
            expected_stance_phrase,
            expected_refs,
        ) = CAPABILITY_MATRIX_CONTRACTS[index]
        if capability != expected_capability:
            errors.append(
                "Capability Matrix row order/content mismatch: "
                f"expected {expected_capability!r}, got {capability!r}"
            )
        if required_class != expected_class:
            errors.append(
                f"Capability Matrix row {capability!r}: required class "
                f"{required_class!r} != expected {expected_class!r}"
            )
        if expected_surface_phrase not in surface:
            errors.append(
                f"Capability Matrix row {capability!r}: expected surface phrase "
                f"{expected_surface_phrase!r} not found in {surface!r}"
            )
        if expected_stance_phrase not in stance:
            errors.append(
                f"Capability Matrix row {capability!r}: expected stance phrase "
                f"{expected_stance_phrase!r} not found in {stance!r}"
            )
        refs = set(
            check_refs(
                f"Capability Matrix row {capability!r}",
                tracker_cell,
                tracker_rows,
                errors,
            )
        )
        if refs != expected_refs:
            errors.append(
                f"Capability Matrix row {capability!r}: tracker refs {sorted(refs)} "
                f"!= expected {sorted(expected_refs)}"
            )

    expected_capabilities = [
        capability for capability, *_rest in CAPABILITY_MATRIX_CONTRACTS
    ]
    if seen_capabilities != expected_capabilities:
        missing = [
            capability
            for capability in expected_capabilities
            if capability not in seen_capabilities
        ]
        extra = [
            capability
            for capability in seen_capabilities
            if capability not in expected_capabilities
        ]
        if missing:
            errors.append("Capability Matrix: missing capabilities " + str(missing))
        if extra:
            errors.append("Capability Matrix: unexpected capabilities " + str(extra))


def numbered_gate_texts(block: str) -> dict[int, str]:
    gates: dict[int, list[str]] = {}
    current_gate: int | None = None
    for line in block.splitlines():
        match = re.match(r"^(\d+)\.\s+(.*)$", line)
        if match:
            current_gate = int(match.group(1))
            gates[current_gate] = [match.group(2).strip()]
            continue
        if current_gate is None:
            continue
        if line.startswith("|") or line.startswith("## "):
            current_gate = None
            continue
        if line.startswith("   "):
            gates[current_gate].append(line.strip())
    return {gate: " ".join(parts) for gate, parts in gates.items()}


def check_gate_traceability(
    text: str,
    section_name: str,
    table_label: str,
    tracker_rows: dict[str, str],
    errors: list[str],
    gate_contracts: dict[int, tuple[str, set[str]]] | None = None,
) -> None:
    block = section(text, section_name, errors)
    gate_texts = numbered_gate_texts(block)
    gates = set(gate_texts)
    if not gates:
        errors.append(f"{section_name}: no numbered adoption gates found")
        return
    if gate_contracts is not None:
        missing_contract_gates = sorted(set(gate_contracts) - gates)
        extra_contract_gates = sorted(gates - set(gate_contracts))
        if missing_contract_gates:
            errors.append(
                f"{section_name}: missing expected adoption gates {missing_contract_gates}"
            )
        if extra_contract_gates:
            errors.append(
                f"{section_name}: unexpected adoption gates {extra_contract_gates}"
            )
        for gate, (expected_phrase, _expected_refs) in sorted(gate_contracts.items()):
            text_for_gate = gate_texts.get(gate, "")
            if expected_phrase not in text_for_gate:
                errors.append(
                    f"{section_name} gate {gate}: expected phrase "
                    f"{expected_phrase!r} not found in {text_for_gate!r}"
                )

    rows = table_rows(block)
    headers = [row for row in rows if row and row[0] == "Gate"]
    if not headers:
        errors.append(f"{section_name}: missing {table_label} table header")
        return
    if headers[0] != ["Gate", "Readiness", "Tracker rows"]:
        errors.append(
            f"{table_label}: header must be ['Gate', 'Readiness', 'Tracker rows']"
        )
        return
    trace_rows = [row for row in rows if row and row[0].isdigit()]
    if not trace_rows:
        errors.append(f"{section_name}: missing {table_label} table rows")
        return

    seen: set[int] = set()
    for row in trace_rows:
        if len(row) != 3:
            errors.append(f"{table_label} gate {row[0]}: expected Gate, Readiness, Tracker rows")
            continue
        gate = int(row[0])
        if gate in seen:
            errors.append(f"{table_label}: duplicate gate {gate}")
        seen.add(gate)
        readiness = row[1].strip("`")
        if readiness not in READINESS_STATUSES:
            errors.append(
                f"{table_label} gate {gate}: unknown readiness {row[1]!r}; "
                f"expected one of {sorted(READINESS_STATUSES)}"
            )
        refs = check_refs(f"{table_label} gate {gate}", row[2], tracker_rows, errors)
        if gate_contracts is not None and gate in gate_contracts:
            expected_refs = gate_contracts[gate][1]
            if set(refs) != expected_refs:
                errors.append(
                    f"{table_label} gate {gate}: tracker refs {sorted(set(refs))} "
                    f"!= expected {sorted(expected_refs)}"
                )
        if readiness == "ready":
            not_verified = [ref for ref in refs if tracker_rows.get(ref) != "verified"]
            if not_verified:
                errors.append(
                    f"{table_label} gate {gate}: readiness is ready but tracker rows "
                    f"are not all verified: {not_verified}"
                )

    missing = sorted(gates - seen)
    extra = sorted(seen - gates)
    if missing:
        errors.append(f"{table_label}: missing traceability rows for gates {missing}")
    if extra:
        errors.append(f"{table_label}: traceability rows for non-existent gates {extra}")


def check_policy_traceability(
    text: str,
    tracker_rows: dict[str, str],
    errors: list[str],
) -> None:
    block = section(text, "Policy-Layer Requirements", errors)
    bullets = re.findall(r"^- ", block, re.MULTILINE)
    rows = table_rows(block)
    if not rows:
        errors.append("Policy requirement traceability: missing table")
        return
    if rows[0] != ["Requirement", "Tracker rows"]:
        errors.append(
            "Policy requirement traceability: header must be ['Requirement', 'Tracker rows']"
        )
        return
    trace_rows = [row for row in rows if row and row[0] != "Requirement"]
    if not trace_rows:
        errors.append("Policy requirement traceability: missing table rows")
        return
    if len(trace_rows) != len(bullets):
        errors.append(
            "Policy requirement traceability: "
            f"{len(trace_rows)} traceability rows for {len(bullets)} policy bullets"
        )
    expected_requirements = [requirement for requirement, _refs in POLICY_REQUIREMENT_REFS]
    seen_requirements: list[str] = []
    bullet_text = "\n".join(
        match.group(1).rstrip(" ;.")
        for match in re.finditer(r"^- (.+)$", block, re.MULTILINE)
    )

    for index, row in enumerate(trace_rows):
        if len(row) != 2:
            label = row[0] if row else f"#{index + 1}"
            errors.append(
                f"Policy requirement traceability row {label!r}: "
                "expected Requirement, Tracker rows"
            )
            continue
        requirement, tracker_cell = row
        seen_requirements.append(requirement)
        if index >= len(POLICY_REQUIREMENT_REFS):
            errors.append(
                f"Policy requirement traceability row {requirement!r}: unexpected extra row"
            )
            check_refs(
                f"Policy requirement traceability row {requirement!r}",
                tracker_cell,
                tracker_rows,
                errors,
            )
            continue
        expected_requirement, expected_refs = POLICY_REQUIREMENT_REFS[index]
        if requirement != expected_requirement:
            errors.append(
                "Policy requirement traceability row order/content mismatch: "
                f"expected {expected_requirement!r}, got {requirement!r}"
            )
        if expected_requirement not in bullet_text:
            errors.append(
                f"Policy-Layer Requirements bullets: missing {expected_requirement!r}"
            )
        refs = set(
            check_refs(
                f"Policy requirement traceability row {requirement!r}",
                tracker_cell,
                tracker_rows,
                errors,
            )
        )
        if refs != expected_refs:
            errors.append(
                f"Policy requirement traceability row {requirement!r}: "
                f"tracker refs {sorted(refs)} != expected {sorted(expected_refs)}"
            )

    if seen_requirements != expected_requirements:
        missing = [req for req in expected_requirements if req not in seen_requirements]
        extra = [req for req in seen_requirements if req not in expected_requirements]
        if missing:
            errors.append("Policy requirement traceability: missing requirements " + str(missing))
        if extra:
            errors.append("Policy requirement traceability: unexpected requirements " + str(extra))

    for _requirement, expected_refs in POLICY_REQUIREMENT_REFS:
        for ref in expected_refs:
            if ref not in tracker_rows:
                errors.append(
                    f"Policy requirement traceability: expected tracker row {ref} is missing"
                )


def check_surface_table(
    text: str,
    tracker_rows: dict[str, str],
    errors: list[str],
    section_name: str,
    tracker_prefix: str,
    surface_contracts: list[tuple[str, str, str, str, set[str]]] | None = None,
) -> None:
    required_refs = {ref for ref in tracker_rows if ref.startswith(tracker_prefix)}
    if not required_refs:
        errors.append(f"TRACKER.md: missing {tracker_prefix}* rows")
        return

    rows = table_rows(section(text, section_name, errors))
    if not rows:
        errors.append(f"{section_name}: missing table")
        return

    header = rows[0]
    expected_header = [
        "Surface",
        "Required class",
        "Readiness",
        "Required artifact",
        "Tracker rows",
    ]
    if header != expected_header:
        errors.append(f"{section_name}: header must be {expected_header!r}")
        return

    data_rows = rows[1:]
    if surface_contracts is not None and len(data_rows) != len(surface_contracts):
        errors.append(
            f"{section_name}: {len(data_rows)} rows for "
            f"{len(surface_contracts)} expected surfaces"
        )

    seen_surfaces: set[str] = set()
    seen_refs: set[str] = set()
    for index, row in enumerate(data_rows):
        if len(row) != len(header):
            errors.append(
                f"{section_name} row {row[0]!r}: "
                f"expected {len(header)} cells, got {len(row)}"
            )
            continue
        surface, required_class, readiness_cell, artifact, tracker_cell = row
        if surface in seen_surfaces:
            errors.append(f"{section_name}: duplicate surface {surface!r}")
        seen_surfaces.add(surface)
        if required_class.strip("`") not in {"direct", "mode_b", "policy"}:
            errors.append(
                f"{section_name} row {surface!r}: unknown required class {required_class!r}"
            )
        readiness = readiness_cell.strip("`")
        if readiness not in READINESS_STATUSES:
            errors.append(
                f"{section_name} row {surface!r}: unknown readiness "
                f"{readiness_cell!r}; expected one of {sorted(READINESS_STATUSES)}"
            )
        if not artifact:
            errors.append(f"{section_name} row {surface!r}: empty required artifact")
        refs = check_refs(
            f"{section_name} row {surface!r}",
            tracker_cell,
            tracker_rows,
            errors,
        )
        if surface_contracts is not None:
            if index >= len(surface_contracts):
                errors.append(f"{section_name} row {surface!r}: unexpected extra row")
            else:
                (
                    expected_surface,
                    expected_class,
                    expected_readiness,
                    expected_artifact_phrase,
                    expected_refs,
                ) = surface_contracts[index]
                if surface != expected_surface:
                    errors.append(
                        f"{section_name} row order/content mismatch: "
                        f"expected {expected_surface!r}, got {surface!r}"
                    )
                if required_class != expected_class:
                    errors.append(
                        f"{section_name} row {surface!r}: required class "
                        f"{required_class!r} != expected {expected_class!r}"
                    )
                if readiness_cell != expected_readiness:
                    errors.append(
                        f"{section_name} row {surface!r}: readiness "
                        f"{readiness_cell!r} != expected {expected_readiness!r}"
                    )
                if expected_artifact_phrase not in artifact:
                    errors.append(
                        f"{section_name} row {surface!r}: expected artifact phrase "
                        f"{expected_artifact_phrase!r} not found in {artifact!r}"
                    )
                if set(refs) != expected_refs:
                    errors.append(
                        f"{section_name} row {surface!r}: tracker refs "
                        f"{sorted(set(refs))} != expected {sorted(expected_refs)}"
                    )
        adoption_refs = [ref for ref in refs if ref.startswith(tracker_prefix)]
        if not adoption_refs:
            errors.append(
                f"{section_name} row {surface!r}: missing {tracker_prefix} tracker row"
            )
        seen_refs.update(ref for ref in adoption_refs if ref in required_refs)
        if readiness == "ready":
            not_verified = [ref for ref in refs if tracker_rows.get(ref) != "verified"]
            if not_verified:
                errors.append(
                    f"{section_name} row {surface!r}: readiness is ready "
                    f"but tracker rows are not all verified: {not_verified}"
                )

    missing = sorted(required_refs - seen_refs)
    if missing:
        errors.append(
            f"{section_name}: missing required tracker refs "
            + ", ".join(missing)
        )
    if surface_contracts is not None:
        expected_surfaces = [surface for surface, *_rest in surface_contracts]
        seen_surface_list = [row[0] for row in data_rows if row]
        if seen_surface_list != expected_surfaces:
            missing_surfaces = [
                surface for surface in expected_surfaces if surface not in seen_surface_list
            ]
            extra_surfaces = [
                surface for surface in seen_surface_list if surface not in expected_surfaces
            ]
            if missing_surfaces:
                errors.append(f"{section_name}: missing surfaces {missing_surfaces}")
            if extra_surfaces:
                errors.append(f"{section_name}: unexpected surfaces {extra_surfaces}")


def check_spec_open_work_sections(
    tracker_rows: dict[str, str],
    errors: list[str],
) -> None:
    for rel, sections in SPEC_OPEN_WORK_SECTIONS.items():
        path = ROOT / rel
        if not path.exists():
            errors.append(f"{rel}: missing normative spec file")
            continue
        text = path.read_text(encoding="utf-8")
        for heading, required_refs in sections.items():
            block = section_from_source(rel, text, heading, errors)
            if not block:
                continue
            if "Tracker ownership:" not in block:
                errors.append(f"{rel} ## {heading}: missing Tracker ownership line")
            refs = set(TRACKER_REF.findall(block))
            if not refs:
                errors.append(f"{rel} ## {heading}: missing tracker references")
            for ref in refs:
                if ref not in tracker_rows:
                    errors.append(f"{rel} ## {heading}: unknown tracker reference {ref}")
            missing = sorted(required_refs - refs)
            if missing:
                errors.append(
                    f"{rel} ## {heading}: missing required tracker ownership refs {missing}"
                )


def check_selected_account_stark_residual_matrix(errors: list[str]) -> None:
    rel = "spec/security_model.md"
    text = SECURITY_MODEL.read_text(encoding="utf-8")
    block = section_from_source(rel, text, "Selected Account-STARK Residual Matrix", errors)
    if not block:
        return

    required_markers = [
        "Path2AccountStarkCreationPayloadV1",
        "account_stark_payload_binding_break",
        "AIR relation soundness",
        "account_stark_air_soundness_break",
        "FRI argument-of-knowledge extractor",
        "account_stark_fri_aok_extractor_break",
        "Zero-knowledge simulator and no-export statement",
        "account_stark_simulator_or_no_export_break",
        "FRI soundness margin and recursion ledger",
        "fri_soundness_margin_break",
        "Production admission gate and review disposition",
        "account_stark_production_gate_break",
        "Legacy proof-route disposition",
        "proof_route_authority_break",
    ]
    for marker in required_markers:
        if marker not in block:
            errors.append(f"{rel} ## Selected Account-STARK Residual Matrix: missing {marker!r}")


def check_direct_theorem_preflight_markers(errors: list[str]) -> None:
    rel = "tools/parameter-search/src/production_reference.rs"
    if not RUST_PRODUCTION_REFERENCE.exists():
        errors.append(f"{rel}: missing production reference source")
        return

    text = RUST_PRODUCTION_REFERENCE.read_text(encoding="utf-8")
    for marker in DIRECT_THEOREM_PREFLIGHT_MARKERS:
        if marker not in text:
            errors.append(f"{rel}: missing direct-theorem preflight marker {marker!r}")

    if "blockers.extend_from_slice(production_deployment_direct_theorem_blockers())" not in text:
        errors.append(f"{rel}: direct-theorem blockers are not wired into preflight")


def check_account_stark_residual_preflight_markers(errors: list[str]) -> None:
    rel = "tools/parameter-search/src/production_reference.rs"
    if not RUST_PRODUCTION_REFERENCE.exists():
        errors.append(f"{rel}: missing production reference source")
        return

    text = RUST_PRODUCTION_REFERENCE.read_text(encoding="utf-8")
    for marker in ACCOUNT_STARK_RESIDUAL_PREFLIGHT_MARKERS:
        if marker not in text:
            errors.append(f"{rel}: missing account-STARK residual preflight marker {marker!r}")

    if "blockers.extend_from_slice(production_deployment_account_stark_residual_blockers())" not in text:
        errors.append(f"{rel}: account-STARK residual blockers are not wired into preflight")


def check_ethereum_positioning_note(
    tracker_rows: dict[str, str],
    errors: list[str],
) -> None:
    rel = "research/ethereum_account_abstraction_positioning.md"
    if not ETHEREUM_POSITIONING.exists():
        errors.append(f"{rel}: missing Ethereum positioning note")
        return

    text = ETHEREUM_POSITIONING.read_text(encoding="utf-8")
    if "Status: ACTIVE_SUPPORT" not in "\n".join(text.splitlines()[:8]):
        errors.append(f"{rel}: missing active support status marker")

    source_context = section_from_source(rel, text, "Source Context", errors)
    if source_context and "Primary-source snapshot:" not in source_context:
        errors.append(f"{rel} ## Source Context: missing primary-source snapshot marker")

    non_goals = section_from_source(rel, text, "Non-Goals For Ethereum Positioning", errors)
    if non_goals:
        required_non_claims = [
            "Ethereum has selected Descender",
            "EIP-8141, EIP-7932, or any other draft EIP is guaranteed",
            "smart-account prototype is equivalent to L1 protocol adoption",
        ]
        for phrase in required_non_claims:
            if phrase not in non_goals:
                errors.append(
                    f"{rel} ## Non-Goals For Ethereum Positioning: "
                    f"missing non-claim phrase {phrase!r}"
                )

    consequences = section_from_source(rel, text, "Tracker Consequences", errors)
    if not consequences:
        return

    refs = set(TRACKER_REF.findall(consequences))
    required_refs = {ref for ref in tracker_rows if ref.startswith("ADOPT-ETH-")}
    missing = sorted(required_refs - refs)
    extra = sorted(ref for ref in refs if ref not in tracker_rows)
    if missing:
        errors.append(f"{rel} ## Tracker Consequences: missing tracker refs {missing}")
    if extra:
        errors.append(f"{rel} ## Tracker Consequences: unknown tracker refs {extra}")


def main() -> int:
    errors: list[str] = []
    tracker_rows = tracker_statuses()
    if not tracker_rows:
        errors.append("TRACKER.md: no tracker IDs found")

    text = SPEC.read_text(encoding="utf-8")
    check_project_framing(text, errors)
    check_requirement_classes(text, errors)
    check_readiness_semantics(text, errors)
    check_capability_matrix(text, tracker_rows, errors)
    check_gate_traceability(
        text,
        "Direct-Mode Adoption Gates",
        "Direct-mode gate traceability",
        tracker_rows,
        errors,
        DIRECT_MODE_GATE_CONTRACTS,
    )
    check_gate_traceability(
        text,
        "Mode B Adoption Gates",
        "Mode B gate traceability",
        tracker_rows,
        errors,
        MODE_B_GATE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Specification Baseline Surfaces",
        "SPEC-",
        SPEC_BASELINE_SURFACE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Direct-Mode Implementation And Proof Surfaces",
        "DIR-",
        DIRECT_IMPLEMENTATION_SURFACE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Aggregation And Rollup Surfaces",
        "AGG-",
        AGGREGATION_SURFACE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Ethereum Adoption Surfaces",
        "ADOPT-ETH-",
        ETHEREUM_SURFACE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Security-Level Adoption Surfaces",
        "ADOPT-128-",
        SECURITY_LEVEL_SURFACE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Hardware And Custody Adoption Surfaces",
        "ADOPT-HW-",
        HARDWARE_SURFACE_CONTRACTS,
    )
    check_surface_table(
        text,
        tracker_rows,
        errors,
        "Policy Feature Surfaces",
        "POL-",
        POLICY_SURFACE_CONTRACTS,
    )
    check_policy_traceability(text, tracker_rows, errors)
    check_non_claims(text, errors)
    check_product_bar(text, errors)
    check_spec_open_work_sections(tracker_rows, errors)
    check_selected_account_stark_residual_matrix(errors)
    check_direct_theorem_preflight_markers(errors)
    check_account_stark_residual_preflight_markers(errors)
    check_ethereum_positioning_note(tracker_rows, errors)

    if errors:
        print("requirement traceability check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print("requirement traceability check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
