# Account-creation + spend recursion circuit — concrete as-built design (2026-06-18)

Design-first pass (analysis only) grounding the strategic
`path2_proof_family_aggregation_recursion_analysis.md` (2026-06-17: Family-A STARK
chosen; highest-leverage = recursive {creation + checked-core spend} → one O(1)
chain-state proof; dominant named risk = SHA-256 unfriendly on the recursion path)
in the crates that now exist.

## Headline finding — the Poseidon2 mask swap retired most of the SHA recursion risk

The creation leaf (`descender-account-stark`) is **Poseidon2 end-to-end** — AIR round
constraints, Merkle/MMCS, Fiat-Shamir (DuplexChallenger), and the C2 mask-PRF — with
**zero SHA in `descender-account-stark/src` and `descender-mask-prf/src`** (verified).
So an in-circuit verifier of one creation proof is **~400–600 Poseidon2 permutations**
(FS public-IO absorb ~24 + 56 queries × short salted Merkle paths ~340 + transcript
misc), all arithmetization-friendly — a SMALL recursion AIR by Plonky3 standards. The
swap paid off exactly here.

Remaining in-circuit SHA-256 on the {creation+spend} critical path is now ONLY:
- (b) the spend checked-core FS frame (`hash_frame_to_challenge_bytes`,
  production_reference.rs ~3260) — EXPENSIVE in-circuit (each SHA block > the whole
  inner-creation verify); and
- (c) the role-root export bridge `SHA256(DS‖role_root)` (descender-account-verifier).

(c) is a cheap, low-assumption re-host to Poseidon2 (already trusting Poseidon2 as the
leaf hash) — **recommended first, Phase-2 prep.** (b) touches the DEPLOYED 262/318-B
spend signature + its SUF-CMA/QROM argument (SHA-256 SPR/CR + frame-PRF→RO seam) —
**hard-to-reverse, assumption-touching; gate behind James's redesign guardrail; NOT a
prerequisite for first recursion** (bind via public-input digest equality meanwhile).

## creation↔spend binding already exists in the leaf statement (MED-2 dischargeable here)

The leaf already exposes, transcript-bound and range-checked: `syndrome[12]`,
`masked_projection[6]`, `packed_witness_commitment[32]` (14-bit-LE, IDENTICAL layout to
the deployed `pack_reviewed_fq12_packed_witness_state18_from_parts`), and `role_root[8]`.
So the recursion AIR discharges the audit's MED-2 ("syndrome not tied to a spend frame
in-AIR") by an **in-AIR field-equality** on the 32 packed bytes between the inner
proof's `packed_witness_commitment` and the spend frame's packed witness — NO new
hashing. role↔spend binding via the export bridge (re-hosted to Poseidon2).

## O(1) chain-state public inputs

A fixed-size tuple independent of N: `chain_state_root` (reuse the leaf's in-AIR
`chain_digest`/`BatchDigest` Poseidon2 accumulator over
`(role_root, packed_witness_commitment, account_id, slot_id)`), `relation_id`,
`verifier_version`, epoch/fork scope, count N. Attests: "N valid creation proofs AND N
valid checked-core spend frames, pairwise bound, accumulated into chain_state_root."

## Plonky3 0.6.1 recursion readiness — HAND-WRITTEN required

NO recursion / verifier-as-AIR primitive in the installed p3-* 0.6.1 crates (only
NATIVE out-of-circuit `verify`/`verify_fri`). "Plonky3 recurses natively" is true of
downstream zkVMs (SP1/Valida), NOT the installed crates. The recursive verifier AIR
must be hand-written, porting `verify_fri`+`verify` into constraints, reusing
`p3-poseidon2-air` for the hash sub-circuits. Multi-month build, not a library call.

## Phased build plan
- **Phase 0 (DONE):** Poseidon2 leaf STARK + ZK-FRI + batch API + creation↔spend
  binding columns + chain digest; mask-PRF Poseidon2; export bridge present.
- **Phase 1 — single-proof recursion:** hand-written verifier-AIR re-verifies ONE
  inner creation proof (~400–600 Poseidon2 perms, zero in-circuit SHA; defer bridge/
  frame to public-input equality). Validates FS-in-circuit + FRI-in-circuit.
- **Phase 2 — {creation+spend} co-verify:** add §-bindings (packed-witness equality =
  MED-2 discharge; role→bridge). Re-host bridge (c) to Poseidon2 first. Decision point
  (behind James) on re-hosting spend FS (b).
- **Phase 3 — batch/IVC over a block:** fold per-pair proofs into the Poseidon2
  chain_state_root accumulator → one O(1) proof per block. Terminate in a STARK (no
  Groth16 wrap) to stay PQ.

## Ranked open risks / soundness obligations
1. **Plonky3 recursion immaturity (HIGHEST)** — hand-written, unaudited; recursive-FRI
   knowledge extractor argued from scratch.
2. **In-circuit Fiat-Shamir soundness / QROM seam** — faithful in-AIR sponge; inner
   Poseidon2-FS and spend SHA-FS coexist in one transcript; FRI QROM loss un-quantified.
3. **Conservative Johnson params for the OUTER STARK** (recursion doesn't inherit inner
   soundness).
4. **Challenge field ~124-bit** (ext degree 4 over KoalaBear, config.rs:104) — union
   over recursion rounds erodes the 128-bit margin; consider ext degree 5 for deep
   recursion.
5. **Spend-side SHA re-host hazard** — (b) touches deployed signature + SUF-CMA;
   hard-to-reverse.
6. **PQ "terminate-in-STARK"** — no Groth16 wrap.
7. **Canonicity/packing exact-equality preserved** under recursion (no Johnson slack on
   the MED-2 binding).

## Status
Design-first complete. The actual recursive-verifier AIR (Phase 1+) is a multi-month
build requiring James's go-ahead. The one in-box prep step it greenlights is re-hosting
the export bridge (c) to Poseidon2; the spend-FS (b) re-host stays behind the redesign
guardrail.
