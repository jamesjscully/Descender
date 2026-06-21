# HIGH-3 fix — challenge widening to ≥246 bits (implementation plan)

> MIGRATION STATUS (2026-06-20): `ACTIVE_RESEARCH`.
> This records an older hard-128 remediation plan for the q12289 byte-layout
> lineage. Current security-level policy is ADR-0006 and `spec/security_model.md`.

Status: **DESIGN CAPTURED, partial edits REVERTED to green** (a worker hit a session
limit mid-cascade, leaving KAT red + fixtures/EC untouched — reverted to avoid a
green-while-broken tree). This doc lets the next pass finish in one shot.

## The bug (red-team HIGH-3)
The checked-core message-binding break is MULTI-TARGET: advantage `T·q/2^n` over `T`
signing queries (the spine unions over the 9 break *events*, not over `T`). At
`n=214` (current effective width), `T=2^32`, `q=2^64` → `2^-118`, breaching `2^-128`.
Maintainer decision: WIDEN to ≥246 effective bits (no usage cap). At `n≥246`,
`T·q/2^n = 2^-150` at `T=2^32,q=2^64` ≤ the `2^-133` per-event budget.

## The design (CORRECTED — widen the existing hardening field, NOT the algebraic part)
HEAD already has `CHALLENGE_HARDENING_BYTES = 4` and `CHALLENGE_BYTES =
ALGEBRAIC_CHALLENGE_BYTES(23) + CHALLENGE_HARDENING_BYTES(4) = 27` (~214 effective bits
after the 0x3f mask on the algebraic part). The reverted worker widened the *algebraic*
part (23→27) — but that changes the ACTION-vector input (action derives from the
algebraic challenge). **Prefer instead: widen the HARDENING field** so the action input
is untouched:
- `CHALLENGE_HARDENING_BYTES: 4 → 8` ⇒ `CHALLENGE_BYTES: 27 → 31` (**+4 B**)
- effective message-binding bits = `algebraic_bits_after_0x3f_mask (~214) + 8*8 (64)`… —
  NOTE recompute exactly: with ALGEBRAIC=23 (184 b − 2 masked = 182) + HARDENING 8*8=64 →
  **246** ✓ (or set HARDENING to whatever hits ≥246 exactly; 8 B is the clean choice).
- the hardening bytes are extra SHA-256 output bytes in `hash_frame_to_challenge_bytes`
  (truncate to 31 instead of 27), bound by the same FS frame; they do NOT feed the action
  vector (action still derives from the algebraic 23 B only — UNCHANGED, minimal ripple).

## Remaining cascade (what the worker did NOT reach — finish these)
1. **Byte layouts both paths** (+4 B each): linear-sigma 260→**264 B**; Fq12 envelope
   314→**318 B** (checked_core 258→262 + carrier 56). Update all layout constants +
   the `signature_layout` offsets in the KAT JSON.
2. **Byte-name fixtures + marker gates** (LOCKSTEP, like the 308→314 relabel):
   `selected_projection_valid_314_byte_fixture_accepts` → `_318_`; the `.find()` marker
   + marker string + fn def (3 sites in `production_reference.rs`); any `314`/`260`
   byte-label strings → `318`/`264`. Run the preflight gate to confirm `.find()` resolves.
3. **KAT regen**: `--kat-generate` then `--kat-check` byte-for-byte; bump the
   `producing_version_digest`.
4. **EC multi-target reduction** — THE CORE OF THE FIX, not just the width:
   `break_checked_core_message_binding.ec` must replace the single-target `q/2^n` with
   the **multi-target `T·eps_spr(n)`** term and prove it ≤ `2^-133` at `T≤2^32, q≤2^64,
   n≥246`. Add `T` as a parameter the spine plugs (currently `spike_union_bound.ec` /
   `gap_c_remodel_spine_wiring.ec` plug a single-target term — fix the plug). Re-typecheck
   all `formal/easycrypt/*.ec` exit 0.
5. **Lean** `Descender/Path2HashBudget.lean` (worker partially edited it) — finish + `lake build`.
6. **Dossier**: flip the §0 HIGH-3 banner item + §2/§3 + `per_event_epsilon_budget_closure.md`
   to RESOLVED (widened to ≥246 b; multi-target `T·eps_spr` closes); update byte sizes 264/318.
7. **Gates**: cargo build + q12289 bin suite + fail-closed preflight + renamed fixtures +
   KAT + `lake build`. Confirm new sizes pk=172 (unchanged), sig 264 (lin-sigma) / 318 (Fq12).

## Guardrail check (maintainer-approved): +4 sig bytes, no usage cap, SIS box untouched
(RESPONSE_BOUND=48, 96<114 unchanged). If the cascade exceeds ~+8 B or needs a deeper
redesign, stop and re-confirm.
