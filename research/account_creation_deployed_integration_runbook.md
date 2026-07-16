# Account-Creation Deployed-Integration Runbook (2026-06-18)

> MIGRATION STATUS (2026-06-20): `ACTIVE_RESEARCH`.
> This is an older ordered integration runbook with useful implementation
> history. It is not the current top-level deployment plan; current status lives
> in `../TRACKER.md` and current wire/security claims live in `../spec/`.

Concrete, ordered plan for the GATED deployed-integration phase, written BEFORE
executing it (no deployed changes made by this doc). Each step marked
REVERSIBLE / IRREVERSIBLE. The final gate flip is EXTERNAL-REVIEW-GATED (do not
flip unilaterally, per SECURITY_DOSSIER D3). Prereqs already done: standalone
account-creation proof validated, both criticals fixed, 7/8 relation classes,
conservative-128-bit, ZK (tools/descender-account-stark).

## Step 0 — the 8th relation class: checked-core-residue-compatibility (REVERSIBLE; standalone-buildable)

This is the only remaining relation class and the bridge to the spend. Design:
- The spend consumes a canonical 18-residue `packed_witness` = [12 checked-core
  syndrome lanes (residues 0..11)] ‖ [6 masked_projection lanes (residues
  12..17)], packed canonically (14-bit/lane LE) to `packed_witness_commitment`
  (the deployed object the spend's checked-core frame binds; see
  `pack_reviewed_fq12_packed_witness_state18_from_parts` in production_reference.rs).
- ADD to the re-host AIR: take the 6 `masked_projection` lanes already in the
  relation (canonical mod-q from fix #1) PLUS 12 `syndrome` lanes as public
  inputs (the checked-core residues from the spend frame); constrain the 18
  residues canonically pack to a PUBLIC `packed_witness_commitment` anchor (the
  same 14-bit packing the spend uses), exposed + transcript-bound. This binds the
  creation proof's masked_projection to exactly the packed witness the spend
  verifies → closes checked-core-residue-compat (8/8 classes).
- It is standalone-buildable (the packing is pure arithmetic); it only needs the
  syndrome lanes + the packed_witness_commitment as public inputs, which the
  deployed leaf already carries. Build + test in descender-account-stark first
  (small config), THEN wire. This step alone is REVERSIBLE (additive crate work).

## Step 1 — pin the mask-PRF decision in ONE place (REVERSIBLE)
Retract the stale `path2_stark_route_af_hash_selection.md` q12289-native R_P /
small-field analysis; record the single authoritative line: mask PRF = native
KoalaBear Poseidon2 (vetted constants) with q12289 as application modulus +
wide-then-reduce (6 lanes per mask). The explicit `MASK` domain lane and six
digits are now deployed in both the C2 AIR and standalone signer derivation;
the loose aggregate reduction term remains below 2^-137 after charging six
masks and the selected 2^32-account horizon.

## Step 2 — global pad-seed uniqueness (REVERSIBLE design choice, then code)
Choose ONE: (a) derive masks from `PRF(pad_seed ‖ account_id ‖ slot)` so reuse is
structurally harmless (preferred — no global registry needed; changes the C2 PRF
input → must match the AIR + spend); or (b) a persistent admitted-pad-seed-digest
registry with the same status as consume-once. Recommend (a). If (a), fold
account_id into the C2 PRF input in the AIR (it's already a public context field)
+ the deployed mask fn. Downgrade tracker §C duplicate-prevention row [x]→[~]
until a cross-batch test exists.

## Step 3 — deployed spend mask-PRF swap (IRREVERSIBLE — changes deployed signature values)
Replace `reviewed_fq12_packed_witness_state18_projection_masks` (SHA-256
`uniform_mod_from_hash`) in production_reference.rs with the KoalaBear-Poseidon2 +
wide-then-reduce derivation, MATCHING the C2 AIR exactly (same constants, same
wide-then-reduce, same PRF input incl. account_id if Step 2a). Bytes unchanged
(262/318 B; still 6 q12289 masked lanes); VALUES change. This requires a vetted
KoalaBear-Poseidon2 available to parameter-search WITHOUT pulling Plonky3 into the
deployed crate — extract a minimal pinned-constant Poseidon2-KoalaBear + the
wide-then-reduce into a small no-Plonky3 module (or a tiny shared crate the AIR
and signer both use, so they CANNOT diverge). IRREVERSIBLE once shipped.

## Step 4 — regenerate the KAT corpus (IRREVERSIBLE — golden vectors change)
All masked_projection / masked_projection_digest / dependent KATs change value
(masks change). Regenerate formal/kat/kat_v0.json + fixtures; re-run --kat-check.
Keep the OLD corpus archived. IRREVERSIBLE (the deployed scheme's vectors change).

## Step 5 — wire the proof into the signer/verifier (REVERSIBLE behind the gate)
DONE 2026-06-18 (verify side) via **Option A (separate verifier crate), James's
choice.** See `account_creation_step5_verifier_wiring_status.md`. New crate
`tools/descender-account-verifier` owns Plonky3 (re-export discipline; bundle
pack/unpack; `verify_deployed_account_creation` = STARK verify + statement↔anchor
binding + role-root export bridge; 2/2 isolation tests pass). parameter-search got
a feature-gated (`account-creation-stark-verifier`, OFF by default) optional dep +
`verify_path2_direct_creation_account_stark_payload_boundary`. Verified: default
build 0 `p3-*` (signer stays Plonky3-free), feature build 96 `p3-*` (Plonky3 only
via the verifier crate). RESIDUAL = producer-side migration (emit `pack_bundle`
output as `proof_bytes`; set `native_role_root_commitment` via
`role_root_export_digest`; carry raw `verifier_version`) — which ALSO retires the
transparent-descent full-hidden-relation backend = **the HIGH-2 fix**. Stays behind
the fail-closed gate. REVERSIBLE.

## Step 6 — EC re-charge (REVERSIBLE; formal, do AFTER Step 3 so EC matches deployed)
Re-charge `frame_prf_ro_reduction.ec`: re-point the abstract RO/dmask from
SHA-256 frame to the KoalaBear-Poseidon2 sponge (proof body unchanged — it's
abstract; uniformity now by wide-then-reduce, not rejection). Dossier §3(b)4:
relabel `A_sha256_projection_mask_frame_prf` → `A_poseidon2_projection_mask_prf`,
tiered NAMED-STANDARD (KoalaBear-Poseidon2 vetted), and downgrade the
"SUF-CMA preserved" claim to "preserved modulo this clean re-charge" (the
mask is hiding-only, break #9 = 0 mass, but the lemma isn't composed into the
spine — state honestly). Typecheck all .ec via the store-path easycrypt.

## Step 7 — full-config soundness + ZK + integration tests (REVERSIBLE; HEAVY → run serially to disk)
At the conservative-128-bit prod config: end-to-end admission→spend integration
test (create slot via proof → spend consumes it), the 8th-class binding, the
mask-swap consistency (deployed mask == AIR mask), KAT byte-stability after
regen. RAM-conscious: serial, small N where possible, big runs one-at-a-time to
disk.

## Step 8 — fail-closed gate flip (IRREVERSIBLE + EXTERNAL-REVIEW-GATED — NOT autonomous)
Do NOT flip unilaterally (SECURITY_DOSSIER D3). Requires external review +
James's sign-off. Also gated on the irreducibly-external coded-SIS lattice
cryptanalysis (spend unforgeability assumption) — independent of this phase.

## Reversibility summary
- REVERSIBLE / safe to do on a branch first: Steps 0,1,2(design),5,6,7.
- IRREVERSIBLE (deployed scheme changes): Steps 3,4 (do together, on a branch,
  with the old corpus archived; "irreversible" = the deployed signature's mask
  values change, so any already-issued artifacts under the old PRF are invalid).
- FORBIDDEN autonomously: Step 8 (external-review-gated).

## What this readies
When authorized, the phase executes in this order with Step 0 (8th class) buildable
immediately and reversibly; the irreversible Steps 3/4 isolated and branch-first;
the final flip held for external review. Nothing here is executed by this doc.
