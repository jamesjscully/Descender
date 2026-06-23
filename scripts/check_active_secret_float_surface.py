#!/usr/bin/env python3
"""Reject floating-point use in active first-party secret-bearing source.

This is a narrow deployment-safety guard. It does not prove constant-time
behavior, but it prevents a Falcon-style float-signing class from entering the
active Path 2 signer/prover implementation surface unnoticed.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

SCAN_ROOTS = (
    "tools/descender-mask-prf/src",
    "tools/descender-account-stark/src",
    "tools/descender-account-verifier/src",
    "tools/descender-recursion/src",
)
SCAN_FILES = (
    "tools/parameter-search/src/production_reference.rs",
)

# serde::Serializer requires these method signatures. The implementation must
# reject floats so a future public byte-model object cannot silently admit them.
SERDE_FLOAT_REJECTION_SNIPPETS = {
    "tools/descender-account-stark/src/counting_serializer.rs": (
        "fn serialize_f32(self, _: f32) -> Result<(), CountErr> {\n"
        "        Err(CountErr)\n"
        "    }",
        "fn serialize_f64(self, _: f64) -> Result<(), CountErr> {\n"
        "        Err(CountErr)\n"
        "    }",
    ),
}

ALLOWED_FLOAT_SIGNATURE_LINES = {
    (
        "tools/descender-account-stark/src/counting_serializer.rs",
        "fn serialize_f32(self, _: f32) -> Result<(), CountErr> {",
    ),
    (
        "tools/descender-account-stark/src/counting_serializer.rs",
        "fn serialize_f64(self, _: f64) -> Result<(), CountErr> {",
    ),
}

# This block is non-deployment descent/SIS estimator support. It is retained as
# research/parameter evidence and is not a signer, prover, wallet, or verifier
# deployment path. Keep the block explicit so any new production_reference.rs
# float use outside this support range fails the gate.
ALLOWED_FLOAT_BLOCKS = {
    "tools/parameter-search/src/production_reference.rs": (
        (
            "pub struct Path2DescentSisHardnessSanity",
            "pub fn path2_descent_sis_matrix_entry",
        ),
    ),
}

FORBIDDEN_PATTERNS = (
    ("float type", re.compile(r"\bf(?:32|64)\b")),
    ("float cast", re.compile(r"\bas\s+f(?:32|64)\b")),
    ("float module", re.compile(r"\b(?:std|core)::f(?:32|64)::")),
    (
        "float math call",
        re.compile(
            r"\.\s*(?:sqrt|cbrt|ln|log(?:2|10)?|exp|powf|ceil|floor|round|trunc|fract)\s*\("
        ),
    ),
    (
        "float literal",
        re.compile(r"(?<![A-Za-z0-9_])\d+\.\d+(?:[eE][+-]?\d+)?(?:f32|f64)?\b"),
    ),
)


def rust_files() -> list[Path]:
    files: list[Path] = []
    for root in SCAN_ROOTS:
        files.extend(sorted((ROOT / root).rglob("*.rs")))
    files.extend(ROOT / file for file in SCAN_FILES)
    return files


def strip_comments_and_strings(line: str) -> str:
    out: list[str] = []
    i = 0
    in_string = False
    in_char = False
    escaped = False
    while i < len(line):
        ch = line[i]
        nxt = line[i + 1] if i + 1 < len(line) else ""
        if in_string:
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == '"':
                in_string = False
            out.append(" ")
            i += 1
            continue
        if in_char:
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == "'":
                in_char = False
            out.append(" ")
            i += 1
            continue
        if ch == "/" and nxt == "/":
            break
        if ch == "/" and nxt == "*":
            end = line.find("*/", i + 2)
            if end == -1:
                break
            out.extend(" " for _ in range(end + 2 - i))
            i = end + 2
            continue
        if ch == '"':
            in_string = True
            out.append(" ")
            i += 1
            continue
        if ch == "'":
            in_char = True
            out.append(" ")
            i += 1
            continue
        out.append(ch)
        i += 1
    return "".join(out)


def strip_cfg_test_items(lines: list[str]) -> set[int]:
    skipped: set[int] = set()
    i = 0
    while i < len(lines):
        if lines[i].strip() != "#[cfg(test)]":
            i += 1
            continue

        start = i
        j = i + 1
        while j < len(lines) and (
            not lines[j].strip() or lines[j].lstrip().startswith("#[")
        ):
            j += 1
        if j >= len(lines):
            skipped.add(start)
            break

        depth = 0
        found_open = False
        k = start
        while k < len(lines):
            skipped.add(k)
            sanitized = strip_comments_and_strings(lines[k])
            if "{" in sanitized:
                found_open = True
            if found_open:
                depth += sanitized.count("{")
                depth -= sanitized.count("}")
                if depth <= 0:
                    break
            elif ";" in sanitized:
                break
            k += 1
        i = k + 1
    return skipped


def allowed_float_block_lines(rel: str, lines: list[str]) -> set[int]:
    allowed: set[int] = set()
    for start_marker, end_marker in ALLOWED_FLOAT_BLOCKS.get(rel, ()):
        start = next((i for i, line in enumerate(lines) if start_marker in line), None)
        end = next(
            (i for i, line in enumerate(lines[start + 1 :], start + 1) if end_marker in line),
            None,
        ) if start is not None else None
        if start is None or end is None:
            continue
        allowed.update(range(start, end))
    return allowed


def check_required_rejections(errors: list[str]) -> None:
    for rel, snippets in SERDE_FLOAT_REJECTION_SNIPPETS.items():
        text = (ROOT / rel).read_text(encoding="utf-8")
        for snippet in snippets:
            if snippet not in text:
                errors.append(f"{rel}: missing required float-rejection serializer snippet")


def scan_file(path: Path, errors: list[str]) -> None:
    rel = path.relative_to(ROOT).as_posix()
    lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    skipped = strip_cfg_test_items(lines)
    allowed_blocks = allowed_float_block_lines(rel, lines)

    for index, raw_line in enumerate(lines):
        stripped = raw_line.strip()
        if index in skipped or index in allowed_blocks:
            continue
        if (rel, stripped) in ALLOWED_FLOAT_SIGNATURE_LINES:
            continue
        sanitized = strip_comments_and_strings(raw_line)
        for label, pattern in FORBIDDEN_PATTERNS:
            match = pattern.search(sanitized)
            if match:
                errors.append(
                    f"{rel}:{index + 1}: {label} in active secret surface: "
                    f"{match.group(0)!r}"
                )
                break


def main() -> int:
    errors: list[str] = []
    check_required_rejections(errors)
    files = rust_files()
    for path in files:
        if not path.exists():
            errors.append(f"{path.relative_to(ROOT).as_posix()}: missing scan target")
            continue
        scan_file(path, errors)

    if errors:
        print("active secret float surface check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    print(f"active secret float surface check passed ({len(files)} files)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
