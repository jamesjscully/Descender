# Checked-core Fiat–Shamir → Poseidon2 re-host (2026-06-18, approved)

> Classification: `ACTIVE_SUPPORT`. This note supports ADR-0004 and
> `spec/wire_formats.md`; it is not an ADR and does not supersede the spec
> spine or `decisions/`.

Approved by James to make the {creation + spend} recursion path arithmetization-
friendly (the last expensive in-circuit SHA on the checked-core relation FS). Mirrors
the mask-PRF swap pattern.

## What changed

- New shared primitive: `descender_mask_prf::poseidon2_hash_bytes::<OUT>(domain, input)`
  — a domain-separated Poseidon2 (KoalaBear) sponge byte-hash, reusing the SAME
  capacity-preserving sponge + pinned constants as the C2 mask PRF (Plonky3-free).
  Absorbs `domain ‖ input` one byte/lane, squeezes OUT bytes (low byte of each
  squeezed field element). Property-tested (deterministic, domain-separated,
  input/length-sensitive).
- New deployed fn: `hash_frame_to_challenge_bytes_poseidon2(frame) -> [u8; CHALLENGE_BYTES]`
  — same 31-byte layout + last-byte mask as the SHA-256 `hash_frame_to_challenge_bytes`,
  domain `PATH2_CHECKED_CORE_FS_POSEIDON2_DOMAIN`. Only the challenge VALUES change.
- `path2_native_checked_core_frame_v1_challenge` now routes through the Poseidon2
  variant. Prover/evidence (`path2_native_checked_core_byte_instantiation_evidence_v1`,
  :4253) and verifier (`path2_native_checked_core_accept_v1`, :4218) BOTH call it, so
  the sigma equation stays consistent for honest proofs (verified: 14 checked-core
  tests pass; the sigma-equation check passes).

## Scope (deliberately bounded)

- **In scope (done):** the checked-core RELATION FS frame — the sigma challenge
  verified at :4218, i.e. exactly what the {creation+spend} recursion circuit must
  check in-circuit.
- **Out of scope (left SHA-256, flagged):** (a) the main LINEAR_SIGMA transaction
  challenge `challenge_bytes` (:60622) — the frozen KAT corpus path, intentionally
  untouched; (b) the `path2_checked_core_authorization_challenge` (:60678) — the spend
  AUTHORIZATION challenge, a broader spend-signature change with KAT impact; not part
  of "the checked-core relation frame," flag for a separate explicit decision;
  (c) `path2_native_checked_core_frame_v1_digest` (:3260) — the frame DIGEST (a binding
  commitment, not the FS challenge) stays SHA-256.

## Assumption impact

The checked-core FS soundness moves SHA-256-as-RO → Poseidon2-sponge-as-RO — the same
named-standard `A_poseidon2_*` assumption already carried for the mask PRF (and aligned
with the coded-SIS reframe: cited-literature for the RO idealization, ours to discharge
via an indifferentiability reduction). No new foundational assumption family.

## Validation

- descender-mask-prf: builds + 8 tests pass (incl. the new property test).
- parameter-search default build: 0 `p3-*` (signer stays Plonky3-free — the Poseidon2
  challenge uses the zero-dep descender-mask-prf, not Plonky3).
- checked-core tests: pass after refreshing the two golden assertions that pinned the
  SHA challenge (golden_vector + byte_instantiation evidence) to the Poseidon2 value
  and the `checked_core_challenge_uses_hash_domain_frame` fact computation.

## Recursion payoff

Combined with the leaf being Poseidon2 end-to-end, the {creation + checked-core spend}
co-verification circuit now has ZERO expensive in-circuit SHA on the relation path —
only the export bridge (cheap Poseidon2 re-host, Phase-2 step 0) and (if ever pulled
in) the authorization challenge remain. The recursion's dominant named risk is retired.
