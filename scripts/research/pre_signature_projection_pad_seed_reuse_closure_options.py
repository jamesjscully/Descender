#!/usr/bin/env python3
"""Compare enforcement-vs-proof paths for pad-seed reuse closure."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
DEFAULT_JSON = READINESS_DIR / "pre_signature_projection_pad_seed_reuse_closure_options_n32_q12289.json"
DEFAULT_MD = READINESS_DIR / "pre_signature_projection_pad_seed_reuse_closure_options_n32_q12289.md"
REUSE_SURFACE_JSON = READINESS_DIR / "pre_signature_projection_pad_seed_reuse_surface_n32_q12289.json"
MASK_PRF_API_JSON = READINESS_DIR / "pre_signature_projection_mask_prf_assumption_api_n32_q12289.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_payload() -> dict[str, Any]:
    reuse = load_json(REUSE_SURFACE_JSON)
    mask_api = load_json(MASK_PRF_API_JSON)
    implementation_paths = [
        {
            "id": "account_state_precommit_unique_pad_seed_digest",
            "summary": "Add pad_seed_digest to each state-assisted account slot record/snapshot and reject duplicate digests at account-state construction and restore.",
            "closes": [
                "duplicate live-slot pad seeds before any spend",
                "backup/restore snapshots containing repeated pad seed digests",
                "same-seed repeated public pad-opening statements across live account state",
            ],
            "code_surface": [
                "ProductionReviewedFq12StateAssistedAccountSlot",
                "ProductionReviewedFq12StateAssistedAccountSlotRecord",
                "ProductionReviewedFq12StateAssistedLifecycleSnapshot",
                "state-assisted lifecycle file store JSON",
                "target-chain lifecycle KV/SQLite persistence",
                "state-assisted slot-opening validation tests",
            ],
            "pros": [
                "catches reuse before signing or verification",
                "does not require revealing the pad seed itself",
                "turns the theorem obligation into an ordinary public uniqueness invariant",
            ],
            "cons": [
                "schema-changing migration across lifecycle snapshots and chain-state stores",
                "adds one public 32-byte digest per live slot in account state",
                "requires deciding whether old snapshots without the field are rejected or migrated",
            ],
            "assessment": "strongest engineering closure; moderate schema cost; preferred if the final theorem does not explicitly allow pad-seed reuse",
        },
        {
            "id": "spend_time_pad_seed_digest_ledger",
            "summary": "Expose pad_seed_digest at selected-projection spend verification and persist a per-account digest ledger that rejects repeats.",
            "closes": [
                "second accepted spend that reveals a previously used pad seed digest",
                "replay/reorg paths after the first accepted spend if the digest ledger is durable",
            ],
            "code_surface": [
                "selected-projection transition API must accept pad_seed_digest or pad_seed",
                "new pad-seed-digest ledger or extended spent-slot record schema",
                "ProductionSelectedProjectionTargetChainDb trait and KV backends",
                "mock blockchain/conformance tests",
            ],
            "pros": [
                "does not add a digest to every unspent slot record",
                "can be localized to consensus acceptance if only spent reuse matters",
            ],
            "cons": [
                "does not catch duplicate live slots before the first spend",
                "existing ProductionSpentSlotRecord has no hidden-source digest field",
                "transition currently receives only account_witness_opening_frame, not pad_seed_digest",
                "still requires storage schema/API changes for durability",
            ],
            "assessment": "useful if account-state schema must remain small, but weaker than precommit uniqueness and still not a trivial patch",
        },
    ]
    proof_path = {
        "id": "prove_same_seed_reuse_harmless",
        "summary": "Do not enforce uniqueness; prove that same pad seed across slots is harmless under the final leakage theorem.",
        "must_prove": [
            "repeated pad_seed_digest does not help correlate or predict raw selected projection residues",
            "repeated pad-opening statement for identical observed block sets does not condition the projection-mask stream beyond the modeled adversary view",
            "slot-id-separated projection masks remain pseudorandom even when all mask queries share the same pad_seed prefix",
            "multi-slot same-seed pad PRF outputs do not create replay or malleability in the carrier relation",
            "crash/reorg recovery cannot let an adversary choose which same-seed slot is spent after observing challenge-dependent data",
        ],
        "evidence_against_easy_closure": [
            "reuse surface probe shows pad_seed_digest_repeats is true",
            "reuse surface probe shows pad_opening_statement_repeats is true for matching observed blocks",
            "current assumption API still lists single_use_pad_seed_policy as open",
        ],
        "pros": [
            "no schema migration",
            "keeps account-state and spent-ledger byte formats unchanged",
            "may be acceptable if SHA-256 PRF/RO theorem is already strong enough for repeated seeds",
        ],
        "cons": [
            "harder theorem: must compose PRF, pad-opening leakage, carrier lanes, and lifecycle adversary",
            "leaves repeated public pad statements as intended behavior rather than rejected behavior",
            "harder to audit than a public uniqueness invariant",
        ],
        "assessment": "cryptographically possible in principle, but it expands the theorem and is less attractive than enforcing uniqueness unless schema cost is unacceptable",
    }
    return {
        "schema": "Descender.pre_signature_projection_pad_seed_reuse_closure_options.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "status": "pad_seed_reuse_closure_options_fail_closed",
        "deployment_ready": False,
        "source_artifacts": {
            "reuse_surface": rel(REUSE_SURFACE_JSON),
            "mask_prf_assumption_api": rel(MASK_PRF_API_JSON),
            "reuse_surface_status": reuse.get("status"),
            "mask_prf_assumption_status": mask_api.get("status"),
        },
        "observed_reuse_surface": {
            "masks_equal_across_slots": reuse["masks_equal_across_slots"],
            "pad_seed_digest_repeats": reuse["pad_seed_digest_repeats"],
            "pad_opening_statement_repeats": reuse["pad_opening_statement_repeats"],
            "current_public_spend_record_fields": reuse["current_public_spend_record_fields"],
        },
        "implementation_paths": implementation_paths,
        "proof_path": proof_path,
        "recommendation": {
            "preferred": "account_state_precommit_unique_pad_seed_digest",
            "reason": "It closes reuse before any adversarial spend choice, converts the issue into a public deterministic invariant, and avoids expanding the already-open PRF/leakage theorem.",
            "fallback": "spend_time_pad_seed_digest_ledger if account-state schema expansion is unacceptable",
            "avoid_unless_necessary": "prove_same_seed_reuse_harmless",
        },
        "next_steps": [
            "Decide whether a 32-byte public pad_seed_digest per live slot is acceptable account-state overhead.",
            "If acceptable, prototype account-state precommit uniqueness and migration-fail-closed tests.",
            "If not acceptable, prototype a separate durable pad_seed_digest ledger at selected-projection spend acceptance.",
            "Only choose the proof path if both storage approaches are rejected.",
        ],
        "artifacts": {"json": rel(DEFAULT_JSON), "markdown": rel(DEFAULT_MD)},
    }


def markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# Pre-Signature Projection Pad-Seed Reuse Closure Options",
        "",
        "This artifact compares the two closure paths for same-pad-seed reuse in",
        "the masked 18-residue design: enforce public uniqueness, or prove reuse",
        "harmless.",
        "",
        "## Summary",
        "",
        f"- status: `{payload['status']}`",
        f"- deployment ready: `{payload['deployment_ready']}`",
        f"- preferred path: `{payload['recommendation']['preferred']}`",
        f"- fallback: `{payload['recommendation']['fallback']}`",
        "",
        "## Observed Reuse Surface",
        "",
    ]
    observed = payload["observed_reuse_surface"]
    for key, value in observed.items():
        lines.append(f"- {key}: `{value}`")
    lines.extend(["", "## Implementation Paths", ""])
    for row in payload["implementation_paths"]:
        lines.extend(
            [
                f"### {row['id']}",
                "",
                row["summary"],
                "",
                f"Assessment: {row['assessment']}",
                "",
                "Closes:",
            ]
        )
        lines.extend(f"- {item}" for item in row["closes"])
        lines.extend(["", "Code surface:"])
        lines.extend(f"- `{item}`" for item in row["code_surface"])
        lines.extend(["", "Pros:"])
        lines.extend(f"- {item}" for item in row["pros"])
        lines.extend(["", "Cons:"])
        lines.extend(f"- {item}" for item in row["cons"])
        lines.append("")
    proof = payload["proof_path"]
    lines.extend(["## Proof Path", "", f"### {proof['id']}", "", proof["summary"], ""])
    lines.extend(["Must prove:"])
    lines.extend(f"- {item}" for item in proof["must_prove"])
    lines.extend(["", "Evidence against easy closure:"])
    lines.extend(f"- {item}" for item in proof["evidence_against_easy_closure"])
    lines.extend(["", "Pros:"])
    lines.extend(f"- {item}" for item in proof["pros"])
    lines.extend(["", "Cons:"])
    lines.extend(f"- {item}" for item in proof["cons"])
    lines.extend(["", f"Assessment: {proof['assessment']}", ""])
    rec = payload["recommendation"]
    lines.extend(
        [
            "## Recommendation",
            "",
            f"Preferred: `{rec['preferred']}`",
            "",
            rec["reason"],
            "",
            f"Fallback: {rec['fallback']}",
            "",
            f"Avoid unless necessary: {rec['avoid_unless_necessary']}",
            "",
            "## Next Steps",
            "",
        ]
    )
    lines.extend(f"- {item}" for item in payload["next_steps"])
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
                "preferred": payload["recommendation"]["preferred"],
                "deployment_ready": payload["deployment_ready"],
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
