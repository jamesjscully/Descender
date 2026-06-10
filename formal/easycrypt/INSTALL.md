# EasyCrypt — install recipe (to run the EC-SPIKE)

**Why this file exists:** `spike_union_bound.ec` is a PAPER-SPIKE. EasyCrypt is
NOT installed on this machine (`opam` is absent; `which easycrypt` fails). This
is the concrete recipe to install it and check the spike. z3 is already present
at `/home/jack/.local/bin/z3` (v4.16.0), so the SMT backend is one prover short
of nothing.

## Measured environment (2026-06-09)

| tool       | status on this box                              |
|------------|-------------------------------------------------|
| `z3`       | PRESENT — `/home/jack/.local/bin/z3` v4.16.0    |
| `opam`     | MISSING                                          |
| `easycrypt`| MISSING                                          |
| `why3`     | MISSING (pulled in by easycrypt)                 |
| `fstar.exe`| PRESENT — `/home/jack/.nix-profile/bin` (unrelated to EC; relevant to the HAX-SPIKE) |

## Recipe (opam — recommended)

```bash
# 1. system deps (Debian/Ubuntu family)
sudo apt-get install -y opam libgmp-dev pkg-config m4

# 2. OCaml toolchain via opam
opam init --bare -y
opam switch create ec ocaml-base-compiler.5.1.1   # EC supports 4.14+/5.x
eval "$(opam env --switch=ec)"

# 3. EasyCrypt + Why3 + the OCaml-side provers
opam pin add -y easycrypt https://github.com/EasyCrypt/easycrypt.git
#    ^ pulls why3 as a dependency

# 4. point Why3 at the EXISTING z3 (do NOT reinstall it)
opam install -y why3
why3 config detect            # discovers /home/jack/.local/bin/z3
#    if z3 is not on PATH for why3, add it explicitly:
#    PATH="/home/jack/.local/bin:$PATH" why3 config detect

# 5. (optional, strengthens SMT) add alt-ergo + cvc5
opam install -y alt-ergo
#    cvc5: download a static release binary, then  why3 config detect

# 6. tell EasyCrypt which provers to use
easycrypt config               # writes ~/.config/easycrypt/easycrypt.conf
```

## Nix alternative

```bash
# EasyCrypt is packaged in nixpkgs; this box already uses nix (fstar.exe lives
# in /home/jack/.nix-profile). z3 from PATH is reused by why3.
nix profile install nixpkgs#easycrypt
why3 config detect
```

## Check the spike

```bash
cd /home/jack/code/Descender/formal/easycrypt
easycrypt -I . spike_union_bound.ec        # WILL surface the admits/axioms
# expect: parses; the `admit.`-ed goals (OB-1..OB-4) report as incomplete;
# the two CLOSURE lemmas should close once their upstream axioms are filled.
```

## Honest expectations for the first run

- The file as written **will not fully verify** — by design it contains
  `admit.` at OB-1..OB-4 and several `axiom`s (the structural-soundness import
  and the single-experiment coupling). Those are the spike's *measured* work.
- The two real-arithmetic closure lemmas (`success_le_target`,
  `success_le_2pow_neg128`) are the parts z3 should close immediately — that is
  the cheap evidence that the Lean `calc`/`linarith` steps port cleanly.
- Library-name drift is expected: `StdBigop.Bigreal.BRA` lemma names
  (`ler_sum`, `ler_sum_const`, `big_split`) may differ across EC versions; fix
  by `pragma` / `print` lookups. Budget a few hours for this, not days.

**This recipe has not been executed here** (no opam). It is a plan, not a log.
