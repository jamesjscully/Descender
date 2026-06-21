#!/usr/bin/env python3
"""
WS3-c: guard against constant drift between the concrete-security estimator
(scripts/research/cryptanalysis_concrete_security_estimate.py) and the current
Rust reference (tools/parameter-search/src/production_reference.rs).

This is the same bug class that broke the build at the start of the cryptanalysis
engagement (a parameter widened in one place, stale in another). A security
estimate computed over stale constants is worse than none. CI runs this; exit
nonzero on any mismatch.
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RUST = ROOT / "tools/parameter-search/src/production_reference.rs"
EST = ROOT / "scripts/research/cryptanalysis_concrete_security_estimate.py"

# Rust `pub const NAME ... = VALUE;`  ->  estimator Python `NAME = VALUE`
# (estimator var name : rust const name)
PAIRS = {
    "Q": "Q",
    "FIELD_BITS": "FIELD_BITS",
    "SIS_PUBLIC_ROWS": "SIS_FULL_RESPONSE_PUBLIC_ROWS",
    "RESPONSE_BOUND": "RESPONSE_BOUND",
    "REPS": "REPS",
    "ALGEBRAIC_CHALLENGE_BYTES": "ALGEBRAIC_CHALLENGE_BYTES",
    "CHALLENGE_HARDENING_BYTES": "CHALLENGE_HARDENING_BYTES",
}
# estimator var name : rust const name, direct literal equality
RUST_DIRECT = {
    "ACTION_RESIDUES": "REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS",
    "ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK": "ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK",
}

def rust_const(name: str) -> int:
    m = re.search(rf"pub const {name}\b[^=]*=\s*(0x[0-9a-fA-F_]+|[0-9_]+)", RUST.read_text())
    if not m:
        raise SystemExit(f"FAIL: rust const {name} not found in {RUST}")
    return int(m.group(1).replace("_", ""), 0)

def py_const(name: str) -> int:
    m = re.search(rf"^{name}\s*=\s*(0x[0-9a-fA-F_]+|[0-9_]+)", EST.read_text(), re.M)
    if not m:
        raise SystemExit(f"FAIL: estimator const {name} not found in {EST}")
    return int(m.group(1).replace("_", ""), 0)

def main() -> int:
    ok = True
    for py_name, rust_name in PAIRS.items():
        rv, pv = rust_const(rust_name), py_const(py_name)
        status = "OK " if rv == pv else "MISMATCH"
        if rv != pv:
            ok = False
        print(f"[{status}] {py_name}={pv}  vs rust {rust_name}={rv}")
    for py_name, rust_name in RUST_DIRECT.items():
        rv, pv = rust_const(rust_name), py_const(py_name)
        status = "OK " if rv == pv else "MISMATCH"
        if rv != pv:
            ok = False
        print(f"[{status}] {py_name}={pv}  vs rust {rust_name}={rv}")
    # derived invariant: estimator RESPONSE_DIFF_BOUND = 2*RESPONSE_BOUND must equal
    # the rust bounded-kernel bound (the response-difference box).
    derived = 2 * py_const("RESPONSE_BOUND")
    rust_kernel_bound = rust_const("RESPONSE_LINK_BOUNDED_KERNEL_BOUND")
    status = "OK " if derived == rust_kernel_bound else "MISMATCH"
    if derived != rust_kernel_bound:
        ok = False
    print(f"[{status}] 2*RESPONSE_BOUND={derived}  vs rust RESPONSE_LINK_BOUNDED_KERNEL_BOUND={rust_kernel_bound}")
    algebraic_bits = (py_const("ALGEBRAIC_CHALLENGE_BYTES") - 1) * 8 + py_const(
        "ALGEBRAIC_CHALLENGE_LAST_BYTE_MASK"
    ).bit_count()
    py_algebraic_bits = py_const("ALGEBRAIC_CHALLENGE_ENTROPY_BITS")
    status = "OK " if algebraic_bits == py_algebraic_bits else "MISMATCH"
    if algebraic_bits != py_algebraic_bits:
        ok = False
    print(
        f"[{status}] algebraic challenge bits={py_algebraic_bits}  "
        f"vs derived bytes/mask={algebraic_bits}"
    )
    total_bits = py_algebraic_bits + 8 * py_const("CHALLENGE_HARDENING_BYTES")
    py_total_bits = py_const("CHALLENGE_ENTROPY_BITS")
    status = "OK " if total_bits == py_total_bits else "MISMATCH"
    if total_bits != py_total_bits:
        ok = False
    print(f"[{status}] challenge entropy bits={py_total_bits}  vs derived total={total_bits}")
    if not ok:
        print("FAIL: estimator constants drifted from production_reference.rs")
        return 1
    print("OK: estimator constants match the current Rust reference.")
    return 0

if __name__ == "__main__":
    sys.exit(main())
