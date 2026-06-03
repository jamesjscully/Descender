#!/usr/bin/env python3
"""Audit the response-link bounded-kernel manifest.

This script is intentionally separate from
``verify_response_link_bounded_kernel_certificate.py`` so the existing verifier
hash pinned in the audit manifest does not change.  By default it performs a
fast static audit of the manifest and certificate hashes/fields.  With
``--replay`` it also invokes the standalone verifier and checks that the replay
returns ``verified`` with no mismatches.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_MANIFEST = (
    ROOT
    / "research/cryptanalysis/signature_readiness_20260516/"
    "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json"
)
DEFAULT_VERIFIER = ROOT / "scripts/research/verify_response_link_bounded_kernel_certificate.py"


def file_sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def canonical_json_sha256(payload: object) -> str:
    encoded = json.dumps(payload, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(encoded).hexdigest()


def load_json(path: Path) -> Any:
    return json.loads(path.read_text())


def localize_manifest_path(path_text: str) -> Path:
    path = Path(path_text)
    if path.exists():
        return path
    candidate = ROOT / path_text
    if candidate.exists():
        return candidate
    return path


def append_check(checks: list[dict[str, object]], name: str, ok: bool, **details: object) -> None:
    checks.append({"name": name, "ok": ok, **details})


def audit_manifest(manifest_path: Path, verifier_path: Path) -> tuple[dict[str, object], list[dict[str, object]], Path]:
    manifest = load_json(manifest_path)
    checks: list[dict[str, object]] = []

    manifest_without_hash = {
        key: value for key, value in manifest.items() if key != "manifest_payload_sha256"
    }
    actual_manifest_hash = canonical_json_sha256(manifest_without_hash)
    append_check(
        checks,
        "manifest_payload_sha256",
        manifest.get("manifest_payload_sha256") == actual_manifest_hash,
        expected=manifest.get("manifest_payload_sha256"),
        actual=actual_manifest_hash,
    )

    artifact_info = manifest["artifacts"]
    certificate_path = localize_manifest_path(str(artifact_info["certificate_path"]))
    certificate = load_json(certificate_path)

    append_check(
        checks,
        "certificate_file_sha256",
        artifact_info.get("certificate_file_sha256") == file_sha256(certificate_path),
        expected=artifact_info.get("certificate_file_sha256"),
        actual=file_sha256(certificate_path),
        path=str(certificate_path),
    )
    append_check(
        checks,
        "certificate_canonical_json_sha256",
        artifact_info.get("certificate_canonical_json_sha256") == canonical_json_sha256(certificate),
        expected=artifact_info.get("certificate_canonical_json_sha256"),
        actual=canonical_json_sha256(certificate),
    )
    append_check(
        checks,
        "verifier_file_sha256",
        artifact_info.get("verifier_file_sha256") == file_sha256(verifier_path),
        expected=artifact_info.get("verifier_file_sha256"),
        actual=file_sha256(verifier_path),
        path=str(verifier_path),
    )

    statement = manifest["statement"]
    box = statement["box"]
    append_check(checks, "statement_q", statement.get("q") == certificate.get("q"))
    append_check(checks, "statement_matrix_rows", statement.get("matrix_rows") == 12)
    append_check(checks, "statement_matrix_cols", statement.get("matrix_cols") == 23)
    append_check(
        checks,
        "statement_matrix_domain",
        statement.get("matrix_domain") == certificate.get("matrix_domain"),
    )
    append_check(
        checks,
        "statement_coords",
        statement.get("coords_fixed_mod_q_to_zero") == certificate.get("coords"),
    )
    append_check(checks, "statement_box_type", box.get("type") == "l_infinity")
    append_check(checks, "statement_box_lower", box.get("lower") == -certificate.get("bound"))
    append_check(checks, "statement_box_upper", box.get("upper") == certificate.get("bound"))
    append_check(checks, "statement_box_dimension", box.get("dimension") == 23)
    append_check(
        checks,
        "statement_cover_radius",
        statement.get("euclidean_cover_radius_squared")
        == certificate.get("euclidean_cover_radius_squared"),
    )

    replay = manifest["replay"]
    basis_validation = replay["basis_validation"]
    certificate_validation = certificate["reduced_basis_lattice_validation"]
    append_check(checks, "replay_status", replay.get("status") == "verified")
    append_check(checks, "replay_mismatches_empty", replay.get("mismatches") == [])
    append_check(
        checks,
        "canonical_basis_sha256",
        basis_validation.get("canonical_basis_sha256") == certificate.get("canonical_basis_sha256"),
    )
    append_check(
        checks,
        "reduced_basis_sha256",
        basis_validation.get("reduced_basis_sha256") == certificate.get("reduced_basis_sha256"),
    )
    append_check(
        checks,
        "canonical_det_abs",
        basis_validation.get("canonical_det_abs")
        == certificate_validation.get("canonical_det_abs"),
    )
    append_check(
        checks,
        "reduced_det_abs",
        basis_validation.get("reduced_det_abs")
        == certificate_validation.get("reduced_det_abs"),
    )
    append_check(
        checks,
        "visited_nodes",
        replay.get("visited_nodes") == certificate.get("visited_nodes"),
    )
    append_check(
        checks,
        "visited_leaves",
        replay.get("visited_leaves") == certificate.get("visited_leaves"),
    )
    append_check(
        checks,
        "nonzero_vectors_in_cover_ball",
        replay.get("nonzero_vectors_in_cover_ball")
        == certificate.get("nonzero_vectors_in_cover_ball"),
    )
    append_check(
        checks,
        "cover_ball_vector_sha256",
        replay.get("cover_ball_vector_sha256")
        == certificate.get("cover_ball_vector_sha256"),
    )

    return manifest, checks, certificate_path


def run_replay(verifier_path: Path, certificate_path: Path, timeout: float) -> dict[str, object]:
    completed = subprocess.run(
        [
            sys.executable,
            str(verifier_path),
            str(certificate_path),
            "--timeout",
            str(timeout),
        ],
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    try:
        payload = json.loads(completed.stdout)
    except json.JSONDecodeError:
        payload = {"stdout": completed.stdout, "stderr": completed.stderr}
    return {
        "returncode": completed.returncode,
        "result": payload,
        "stderr": completed.stderr,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--verifier", type=Path, default=DEFAULT_VERIFIER)
    parser.add_argument("--replay", action="store_true")
    parser.add_argument("--timeout", type=float, default=240.0)
    args = parser.parse_args()

    manifest, checks, certificate_path = audit_manifest(args.manifest, args.verifier)
    replay_result = None
    if args.replay:
        replay_result = run_replay(args.verifier, certificate_path, args.timeout)
        replay_payload = replay_result["result"]
        append_check(
            checks,
            "standalone_replay_status",
            replay_result["returncode"] == 0
            and isinstance(replay_payload, dict)
            and replay_payload.get("status") == "verified"
            and replay_payload.get("mismatches") == [],
            returncode=replay_result["returncode"],
            status=replay_payload.get("status") if isinstance(replay_payload, dict) else None,
            mismatches=replay_payload.get("mismatches") if isinstance(replay_payload, dict) else None,
        )

    failed = [check for check in checks if not check["ok"]]
    output = {
        "schema": "Descender.response_link.bounded_kernel.manifest_audit.v1",
        "manifest_path": str(args.manifest),
        "manifest_payload_sha256": manifest.get("manifest_payload_sha256"),
        "certificate_path": str(certificate_path),
        "checks": checks,
        "failed_checks": failed,
        "status": "verified" if not failed else "mismatch",
    }
    if replay_result is not None:
        output["replay"] = replay_result
    print(json.dumps(output, indent=2, sort_keys=True))
    if failed:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
