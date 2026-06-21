# hax — historical install recipe for the HAX-SPIKE

> PACKAGE CONTEXT (2026-06-20): This is a package-scoped historical install
> recipe, covered by `research/ARCHIVE_INDEX.md`. The measured spike result is
> `SPIKE_VERDICT.md`. Current authority starts at `../../spec/README.md`, ADRs,
> and `../../TRACKER.md`.

**Why this file exists:** `spike_target.md` names a real extraction target. This
file records the original local setup recipe for installing hax and extracting
the target to F\*.

## Measured environment (2026-06-09)

| tool         | status on this box                                          |
|--------------|-------------------------------------------------------------|
| `cargo`      | PRESENT                                                      |
| `cargo hax`  | MISSING — `error: no such command: hax`                     |
| `fstar.exe`  | PRESENT — `/home/jack/.nix-profile/bin`, F\* 2026.03.24~dev |
| `z3`         | PRESENT — `/home/jack/.local/bin/z3` v4.16.0 (F\*'s SMT backend) |

At the time this recipe was written, the spike was gated **only on the hax
frontend**: the F\* checker and its SMT prover were already present.

## Recipe A — nix (matches this box's existing nix usage)

```bash
# hax ships a flake. fstar.exe is already in the nix profile.
nix profile install github:hacspec/hax
cargo hax --version          # verify the cargo subcommand registers
```

## Recipe B — cargo (build hax from source)

```bash
# 1. system deps for the OCaml engine half of hax
sudo apt-get install -y opam libgmp-dev pkg-config
opam init --bare -y
opam switch create hax ocaml-base-compiler.4.14.2
eval "$(opam env --switch=hax)"

# 2. clone + install both halves (Rust frontend + OCaml engine)
git clone https://github.com/hacspec/hax.git
cd hax
./setup.sh                   # builds cargo-hax (frontend) + hax-engine (OCaml)
#    installs the `cargo hax` subcommand into ~/.cargo/bin

cargo hax --version
```

## Extraction command (the spike)

```bash
cd /home/jack/code/Descender/tools/parameter-search

# Extract the named target (+ its pure callees) to F*.
# --include-namespaces keeps the blast radius to the sigma-equation core;
# start with the minimal fixed-size core if the full function chokes (see
# spike_target.md NO-GO mitigation).
cargo hax into fstar \
  -- --include-namespaces 'production_reference::reviewed_fq12_account_carried_expected_response_syndrome'

# output lands in ./proofs/fstar/extraction/*.fst (+ *.fsti)
```

## Check the extracted F\* (prover already present)

```bash
cd proofs/fstar/extraction
# fstar.exe is on PATH from the nix profile; z3 is at /home/jack/.local/bin/z3
fstar.exe --z3version 4.16.0 \
          --smt /home/jack/.local/bin/z3 \
          Production_reference.fst
# then add the obligation lemmas (spec §3 (1)-(4) from spike_target.md) and
# re-check; GO iff fstar.exe discharges them.
```

## Historical expectations

- This is the original install plan, not the measured result. Use
  `SPIKE_VERDICT.md` for the actual extraction evidence.
- The risky constructs are `Result<_,String>` + `?`, `Vec<u16>` in
  `poly_mul_mod`, and `Sha256`/rejection-sampler in
  `reviewed_fq12_challenge_action_vector`. If hax rejects them, fall back to the
  minimal fixed-`[u16;12]` core extraction first (no `Vec`/SHA/`Result`), per
  `spike_target.md`. That core is the most likely clean GO.
- A GO result lets the corresponding source-inspection **marker gate** be
  deleted — the whole point of this layer per `README.md`.
