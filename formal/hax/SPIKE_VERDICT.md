# HAX-SPIKE — VERDICT

**Date:** 2026-06-09
**Tools:** `cargo-hax` 0.3.7 (nix, pins rustc 1.93.0-nightly-2025-11-08),
`fstar.exe` 2026.03.24~dev, `z3` 4.16.0.
**Target:** `reviewed_fq12_account_carried_expected_response_syndrome`
(`tools/parameter-search/src/production_reference.rs:37008`) — the Fq12
sigma-equation RHS `z = a + action(c)·w`.

---

## VERDICT: **GO (conditional)** — hax adopted for the bridge layer, with a
## one-time proof-lib top-up.

hax **extracted the full target function and all its pure callees to idiomatic,
type-faithful F\***. The translation is clean. The only friction was (a) one
real Rust-idiom rewrite (`iter_mut().enumerate()` → index loop) and (b) a set of
**missing leaf definitions in the hax 0.3.7 nix-bundled F\* proof-libraries** —
a packaging/version-skew problem in the shipped support libs, *not* a limitation
of hax's ability to translate the Rust. No construct in `production_reference.rs`
defeated the frontend. That is a GO on the spike's own criteria.

The hedge ("conditional") is honest: a *zero-touch* `fstar.exe` typecheck of the
verbatim extraction is currently blocked by the bundled proof-libs, so the GO is
"adopt hax once the 5 missing proof-lib stubs are supplied," not "it already
typechecks untouched."

---

## What was actually run (commands + real output)

### 1. Direct extraction of the real crate — BLOCKED by a *dependency*, not the target

```
cd tools/parameter-search
cargo hax into -i '-** +descender_parameter_search::production_reference::\
  reviewed_fq12_account_carried_expected_response_syndrome' fstar
```
Output:
```
error[E0658]: use of unstable library feature `cfg_select`
  --> libsqlite3-sys-0.38.0/build.rs:110 ... add #![feature(cfg_select)]
error: could not compile `libsqlite3-sys` (build script)
warning: hax: running `cargo build` was not successful, continuing anyway.
```
No `.fst` produced. **Cause:** the crate depends on `rusqlite` →
`libsqlite3-sys 0.38.0`, whose build script uses `cfg_select!`, stabilized
*after* hax's pinned 2025-11-08 nightly. hax hooks the compiler, so the whole
dependency graph must `cargo check` under hax's toolchain — and this dep won't.
(Plain `cargo check --lib` on the normal stable toolchain succeeds; this is
purely a hax-toolchain-vs-dependency skew.) `rusqlite` is only used by the
persistence-lifecycle code at `production_reference.rs:40068`, unrelated to the
arithmetic target — but it sits in the same crate.

**→ Mitigation (per `spike_target.md`): extract the minimal pure core in a
standalone crate with no `rusqlite`/`serde`/`sha2`.** That crate is
`formal/hax/spike_core/` (target fn + pure callees copied verbatim; the SHA-256
`reviewed_fq12_challenge_action_vector` replaced by an opaque `action_oracle`,
exactly the abstraction the OBLIGATION prescribes).

### 2. Extraction of the minimal core — SUCCEEDED

```
cd formal/hax/spike_core
cargo hax into fstar      # → ./proofs/fstar/extraction/Spike_core.fst (509 lines)
```
After **one** Rust-idiom fixup it extracted with **zero hax errors**. The
generated F\* is faithful and readable:
- `for … in 0..n` → `Rust_primitives.Hax.Folds.fold_range`
- `Result<_,String>` + `?` → `t_Result … Alloc.String.t_String` with
  `fold_range_return` / `ControlFlow`
- `Vec<u16>` + `push`/`pop`/`from_elem`/`len` → `Alloc.Vec.*`
- `assert_eq!` → `Hax_lib.v_assert`; const arrays → `array_of_list`
- the target itself extracted to a clean nested-`match` mirroring
  `Ok(a + action(c)*w)`.

### 3. `fstar.exe` typecheck of the extraction — names resolve, blocked only by missing proof-lib stubs

Ran with the hax F\* proof-libs found on the box
(`/nix/store/hha4…-source/hax-lib/proof-libs/fstar/{core,rust_primitives}` +
`…/hax-lib/proofs/fstar/extraction`):
```
fstar.exe --include <core> --include <rust_primitives> --include <hax-lib> Spike_core.fst
```
F\* loaded every library, and resolved **all** of the heavy machinery
(`fold_range`, `fold_range_return`, `fold_enumerated_slice`, `Vec::*`,
`Result`, `Option::is_some_and`). It failed *only* on a short list of leaf
identifiers that the hax **frontend emits but the bundled proof-libs do not
define** — each surfaced as `Error 72: Identifier … not found`:

| emitted symbol (Rust source)                  | from              | in bundled lib? |
|-----------------------------------------------|-------------------|-----------------|
| `Alloc.Vec.Into_iter` (`.into_iter()`)        | `into_iter()`     | present but **internally inconsistent** (`t_IntoIter` allocator param typed `unit` yet applied to `t_Global`) → `Error 189` |
| `Iterator.f_any` (`.any()`)                   | `.any(..)`        | **missing** (`f_all` present, `f_any` absent) |
| `Core_models.Slice.impl__last` (`.last()`)    | `.last()`         | **missing** (`get`/`len`/`is_empty` present) |
| `Core_models.Option.impl_2__copied`           | `Option::copied`  | **missing** |
| `Rust_primitives.Hax.while_loop_cf`           | `while`+`break`   | **missing** (`while_loop`, `while_loop_return` present) |

Every one of these is a *gap in the shipped support library*, not a Rust
construct hax can't translate. Rewriting each source idiom to an equivalent that
*does* have a library model (`.last()`→`len()`/index, `.into_iter()`→index,
`.any()`→accumulator loop) removed them one-by-one and pushed F\* strictly
further each time. After 5 such rewrites the **only** remaining unresolved
symbol is `while_loop_cf` (the `poly_mul_mod` reduction loop's `while`+`break`);
that one needs either the missing lib stub or a `for`/fold rewrite of the loop.

---

## Which Rust constructs blocked extraction?

**None blocked the hax *frontend*.** The constructs the NO-GO criteria flagged as
risky all extracted successfully:
- `Result<_,String>` + `?` ✅ (→ `fold_range_return`/`ControlFlow`)
- `Vec<u16>` growth + `pop`/`push`/`from_elem` in `poly_mul_mod` ✅
- `format!`/`String` error messages ✅
- `assert_eq!`/`assert!` ✅, `let-else` ✅, nested `for…enumerate` ✅

What required hand-edits were **idiom rewrites to dodge missing library stubs**
(see `spike_core/src/lib.rs`, 5 comments tagged `hax fixup`), plus the one
genuine hax-frontend limitation:
- `iter_mut().enumerate()` → `Error [HAX0003] mutation of this &mut is not
  allowed here` (hax issue #420). This is a real, documented hax restriction —
  but the fix (plain index loop) is trivial and semantically identical.

SHA-256 / the rejection-sampler / the unbounded `for counter in 0_u32..` loop in
`uniform_mod_from_hash` were **not tested**: per the OBLIGATION, `action(c)` is a
random oracle and was abstracted to an opaque total map. If hax were ever pointed
at the real `uniform_mod_from_hash`, the unbounded counter loop and `Sha256`
trait machinery are the likeliest *genuine* frontend blockers — but they are out
of scope for the obligation, which only needs `action : challenge → Fq12` total.

---

## Is the F\* obligation tractable? — YES (structurally)

The extracted code is exactly the shape the OBLIGATION wants: small, first-order,
fixed-width integer arithmetic over `[u16;12]` + a `Vec<u16>` poly. Obligations
(1)–(4) from `spike_target.md` map onto standard F\* lemmas:
- (1) `add_mod`/`sub_mod` = field ±, result `<q`: the extraction keeps the
  `u32` intermediates explicit (`(cast u32 …) %! (cast Q)`), so the `<q` and
  no-overflow facts are direct `mod`-bound lemmas — Z3-trivial.
- (2) `poly_mul_mod` = mult in `F_q[X]/(X^12+X+25)`: the convolution + monic
  reduction extracted faithfully; this is the one lemma needing real work
  (loop invariant on `product`), but it is a standard schoolbook-mul
  correctness proof, well within F\*+Z3.
- (3) `unpack_…_digest` lane-canonicality (`<q`, `Err` on non-canonical): the
  `decoded >= Q` guard extracted verbatim → straightforward refinement.
- (4) Composition `a + act*w`: the target's body is literally
  `fq12_syndrome_add randomizer (fq12_syndrome_mul (action_oracle c) witness)`.

No higher-order, no trait-object, no unbounded-state obstacle to the proof. The
obligation is tractable; the gating risk is library plumbing, not proof
difficulty.

---

## Recommendation for the bridge layer: **hax**, over Cryptol/SAW and Lean-exec+KAT

1. **Adopt hax** for the sigma-equation/arithmetic bridge. It extracts the real
   Rust verbatim (no re-implementation drift — the killer advantage over a
   hand-written Cryptol or Lean model, which would re-encode the algorithm and
   then need its *own* equivalence argument to the Rust). The F\* obligation is
   tractable.

2. **Two concrete prerequisites before declaring a hard GO** (both one-time, low
   effort):
   - **(a) Isolate the verifier core from `rusqlite`.** Either move the pure
     arithmetic (`add_mod`…`poly_mul_mod`…the syndrome fns) into a dependency-free
     sub-crate/module that hax targets, or downgrade/patch `libsqlite3-sys` so
     it builds on hax's pinned nightly. Without this, hax can't even `check` the
     real crate. The `spike_core` crate is the template.
   - **(b) Top up the hax F\* proof-libs** with the 5 missing stubs
     (`impl__last`, `impl_2__copied`, `f_any`, the `Into_iter` allocator-type
     fix, `while_loop_cf`) — or pin a hax release whose bundled libs match
     `fstar.exe` 2026.03.24. Alternatively, keep the source-side idiom rewrites
     (already proven to work) as the cheaper path.

3. **Keep `../kat/` as the independent cross-check** regardless. hax proves
   *Rust ≡ F\* spec*; KAT vectors prove *both ≡ the reference implementation's
   runtime behavior*. They are complementary, not redundant. Do **not** retire
   the KAT layer.

4. **Marker gates:** once (a)+(b) land and the four lemmas discharge, the
   source-inspection `verify_reviewed_fq12_account_carried_sigma_equation`
   marker gate (`production_reference.rs:8023`) can be deleted, per the README's
   stated Go outcome. Until then, keep it.

**Cryptol/SAW or Lean-exec+KAT** remain the fallback *only if* prerequisite (a)
proves intractable (e.g. the verifier genuinely cannot be separated from
`rusqlite`) — but nothing observed suggests that. hax is the recommended primary.

---

## Artifacts (for the record)

- `formal/hax/spike_core/` — standalone minimal-core crate (compiles on stable;
  `src/lib.rs` carries 5 `// hax fixup` comments documenting each rewrite).
- `formal/hax/extracted/Spike_core.fst` — the generated F\* (saved snapshot).
- hax proof-libs used: `/nix/store/hha4hknvhm2rrs7qy7vjp3sh183cnaaq-source/hax-lib/proof-libs/fstar/`.

**Honesty note:** No F\* lemma was *proved* — the spike got as far as a near-clean
`fstar.exe` name-resolution/typecheck of the extracted target, blocked on one
missing library combinator (`while_loop_cf`). The obligation lemmas (1)–(4) are
characterized as tractable but were **not** discharged. "hax extracted cleanly
(1 frontend idiom fixup) but the bundled F\* proof-libs are out of sync (5
missing/!inconsistent stubs)" is the precise state.
