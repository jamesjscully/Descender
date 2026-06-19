# Full Spend FS Poseidon2 Re-host Plan

**Date:** 2026-06-18
**Scope:** Extend the Poseidon2 Fiat–Shamir re-host from the (already-done) checked-core
RELATION frame to the FULL spend FS, i.e. the two remaining SHA-256 challenge derivations:

1. the **main LINEAR_SIGMA transaction challenge** `challenge_bytes` (production_reference.rs:60615),
   feeding the deployed 260-byte transaction signature and the frozen KAT corpus
   (`formal/kat/kat_v0.json`); and
2. the **checked-core AUTHORIZATION challenge** `path2_checked_core_authorization_challenge_bytes`
   (production_reference.rs:60668), used by the spend authorization binding.

Both currently call the SHA-256 base `hash_frame_to_challenge_bytes` (production_reference.rs:60586).
A drop-in Poseidon2 variant `hash_frame_to_challenge_bytes_poseidon2` (production_reference.rs:60606)
already exists; it uses `descender_mask_prf::poseidon2_hash_bytes` (Plonky3-free) and preserves the
31-byte layout + last-byte mask, so downstream challenge PARSING is unchanged — only VALUES change.

ANALYSIS ONLY: this document is the plan. No edits or builds were performed.

---

## 1. Recommended approach: re-host the BASE hash, NOT per-call-site

**Recommendation: re-host the base `hash_frame_to_challenge_bytes` (60586) itself to route
through `descender_mask_prf::poseidon2_hash_bytes`, unifying ALL spend FS in one edit.**

Rationale:
- The relation frame already re-hosted via `path2_native_checked_core_frame_v1_challenge`
  (production_reference.rs:3257) calling `hash_frame_to_challenge_bytes_poseidon2` directly.
  The two remaining spend challenges (`challenge_bytes`, `path2_checked_core_authorization_challenge_bytes`)
  are the ONLY other production callers of the SHA base. Re-hosting the base flips both at once
  with a single, auditable change and guarantees prover/verifier consistency (both sides call the
  same base function).
- A per-call-site approach (point `challenge_bytes` and `path2_checked_core_authorization_challenge_bytes`
  each at the `_poseidon2` variant) leaves a live SHA-256 `hash_frame_to_challenge_bytes` lying around
  as a footgun: future callers would silently re-introduce SHA-based FS. After this re-host there is no
  intended remaining SHA-256 FS challenge surface in spend.
- The FS soundness assumption moves SHA-256-as-RO -> Poseidon2-sponge-as-RO uniformly. The
  `A_poseidon2_*` named-standard assumption is ALREADY carried (mask PRF + relation frame), so this
  adds no new assumption class.

**Domain separation caveat (must address during the edit):** `hash_frame_to_challenge_bytes_poseidon2`
hardcodes the relation domain `PATH2_CHECKED_CORE_FS_POSEIDON2_DOMAIN`
("Descender.Path2.checked_core.fiat_shamir.poseidon2.v1", 60595). If the base is re-hosted to use that
single domain, the three FS contexts (relation, transaction, authorization) would share a domain tag.
The frames already embed distinct internal labels/domains (the txn frame embeds `CHALLENGE_HASH_DOMAIN`
+ `proof_commitment`; the authorization frame embeds `PATH2_SLOT_CONSUME_SINGLE_DOMAIN`), so they are
already structurally distinguishable. However, to keep clean FS domain separation, the RECOMMENDED
concrete shape is:

- Introduce TWO new Poseidon2 FS domain constants alongside the existing relation one:
  - `PATH2_TXN_FS_POSEIDON2_DOMAIN`   = "Descender.Path2.transaction.fiat_shamir.poseidon2.v1"
  - `PATH2_AUTH_FS_POSEIDON2_DOMAIN`  = "Descender.Path2.checked_core.authorization.fiat_shamir.poseidon2.v1"
- Generalize the Poseidon2 helper to take a domain arg:
  `hash_frame_to_challenge_bytes_poseidon2_with_domain(domain: &[u8], frame: &[u8])`, and keep the
  existing relation-domain `hash_frame_to_challenge_bytes_poseidon2` as a thin wrapper for back-compat.
- Re-point `challenge_bytes` (60622) and `path2_checked_core_authorization_challenge_bytes` (60678) at
  the new helper with their respective domains, and KEEP the now-dead SHA `hash_frame_to_challenge_bytes`
  ONLY if some non-FS/legacy use remains; otherwise delete it. (Net: this is "base re-host" in spirit —
  one unified Poseidon2 sponge — with explicit per-context domain tags, the best of both options.)

If a maximally minimal change is preferred over domain hygiene, the literal base re-host (edit the body
of 60586 to call `poseidon2_hash_bytes` with a single FS domain) is acceptable because the frames are
already internally domain-separated; but the two-domain variant above is the recommended end-state.

---

## 2. Exact edit list (file:line)

All in `tools/parameter-search/src/production_reference.rs`:

- **60594–60613** — add `PATH2_TXN_FS_POSEIDON2_DOMAIN` and `PATH2_AUTH_FS_POSEIDON2_DOMAIN` consts;
  add `hash_frame_to_challenge_bytes_poseidon2_with_domain(domain, frame)`; keep
  `hash_frame_to_challenge_bytes_poseidon2` as the relation-domain wrapper (used at 3257, 4358, 67575, 79802).
- **60615–60629** `challenge_bytes` — change body line 60622 from
  `hash_frame_to_challenge_bytes(&challenge_hash_frame(...))` to
  `hash_frame_to_challenge_bytes_poseidon2_with_domain(PATH2_TXN_FS_POSEIDON2_DOMAIN.as_bytes(), &challenge_hash_frame(...))`.
- **60668–60688** `path2_checked_core_authorization_challenge_bytes` — change body line 60678 to use
  `..._with_domain(PATH2_AUTH_FS_POSEIDON2_DOMAIN.as_bytes(), &path2_checked_core_authorization_challenge_frame(...))`.
- **60586–60592** `hash_frame_to_challenge_bytes` (SHA base) — after the two re-points there is no intended
  FS caller left except the in-file test at 102369 and the doc-string-scan tests. Decide: delete it, or
  retain + rename to `..._sha256_legacy` and update the test at 102369. RECOMMENDED: retain renamed so the
  archived-SHA KAT and the equivalence test stay meaningful, but mark `#[deprecated]`/comment "not FS".

No signature struct / layout / size constant changes (see §4 — bytes UNCHANGED).

### Binaries with their OWN local `hash_frame_to_challenge_bytes`
- `src/bin/signature_q12289_local_delta_stateful.rs:3536` delegates to `prodref::hash_frame_to_challenge_bytes`.
  This is the **DEPLOYED / KAT-producing** binary (`production_keygen`/`production_sign`/`production_verify`).
  It must follow the base: change its delegate (3537) to call the new Poseidon2 txn path, OR — cleaner —
  have it call `prodref::challenge_bytes` indirectly. Its ~30 call sites (2678, 2741, 2830, 2928, 3672,
  3865, 4367, 4714, 4792, 5903/5904, 5953/5954, 6108/6109, 6427, 9860, ...) all route through the local
  delegate, so a SINGLE edit at 3537 flips the whole deployed signer. **This is the load-bearing edit for
  KAT regeneration.**
- `src/bin/signature_q12289_compact_format.rs:501` and `signature_q12289_implicit_verifier.rs:503` define
  INDEPENDENT local `Sha256::digest` copies. These are experimental/non-deployed format probes (NOT the
  KAT path, NOT the deployed signer). They do NOT need to change for spend-FS correctness, but for
  consistency they should either be re-hosted too or explicitly documented as "legacy SHA experiment, not
  the deployed FS." RECOMMENDED: leave with a comment, out of scope; flag in the test checklist.

### Plonky3-free confirmation
- `descender_mask_prf::poseidon2_hash_bytes` is in `tools/descender-mask-prf/src/mask.rs:156`; that crate's
  Cargo.toml line 8 is DELIBERATELY EMPTY of any p3-* dep. `tools/parameter-search/Cargo.toml` (lines 9–14)
  documents that the default wallet/signer build links ZERO p3-* crates; Plonky3 only enters behind a
  non-default validator feature. **The deployed signer stays Plonky3-free: confirmed.** No new dependency
  is added by this re-host.

---

## 3. Downstream that CHANGES VALUE (every KAT / golden / fixture)

Anything whose bytes depend on a transaction or authorization challenge changes value (the challenge feeds
the response and openings via Fiat–Shamir, so the WHOLE signature body downstream of the challenge moves):

### A. Frozen KAT corpus — `formal/kat/kat_v0.json` (PRIMARY)
Every field derived from sign/verify changes: per-vector `challenge`, `response`, `openings`,
`linear_sigma_proof`, `signature_hex`, `signature_bytes`, the positive vectors' downstream bytes, and the
top-level `producing_version_digest` (computed in
`signature_q12289_local_delta_stateful.rs:6974` over PROFILE/Q/PK_BYTES/KAT_SIGNATURE_BYTES/CHALLENGE_BYTES
+ a freshly recomputed positive signature — the fresh signature changes -> digest changes). Reject-reason
strings and lengths are UNCHANGED.
- Current value to be superseded: `producing_version_digest = 444055fe...44cf51`.
- `formal/kat/kat_v0.md` (human index) — the embedded producing-version digest line (kat_v0.md:26) and
  any quoted bytes must be re-synced; it says "Do not hand-edit — regenerate."
- `formal/kat/kat_v0_sha256_archived.json` — currently BYTE-IDENTICAL to kat_v0.json (the SHA snapshot was
  pre-staged but the re-host has not happened yet). This file is the historical SHA-256 anchor; LEAVE IT
  unchanged so the SHA-era corpus is preserved for the archived equivalence test / audit trail.

### B. Tests / fixtures that recompute (no hardcoded hex — they recompute, so they PASS after regen)
- `production_reference.rs:102369` `hash_frame_to_challenge_bytes(&frame)` + assertions at 102374–102378:
  these compare `challenge_bytes(...)` to the base hash of the frame and assert `!= legacy`. If the base is
  renamed/retained-as-SHA, this test must be repointed to the Poseidon2 helper (it currently proves
  "challenge_bytes == base hash of frame", which after re-host means base = Poseidon2 helper).
- `production_reference.rs:4358` and `:3257` (relation) and `:67575`, `:79802` — already Poseidon2; UNCHANGED.
- The KAT in-file tests `kat_v0_corpus_is_internally_consistent` (7579) and
  `kat_v0_corpus_reproduces_frozen_json` (7609) recompute the corpus and compare to disk — they will FAIL
  until `kat_v0.json` is regenerated, then PASS. (These are the forcing function; that's intended.)
- The deployed binary's many `hash_frame_to_challenge_bytes(&frame)` round-trip asserts (2678/2741/2830/
  2928/3672/3865/4367/4714/4792/5903-6109/6427/9860 in stateful.rs) RECOMPUTE both sides; they stay green
  after the delegate edit because prover and verifier shift together. No hardcoded SHA challenge constant
  was found in these (verified: only structural asserts like `algebraic_challenge_last_byte_mask == 0x3f`).

### C. Confirmed NOT golden vectors (no change)
- `path2_native_checked_core_frame_v1_golden_vector_and_roundtrip` (67517 / asserted via doc-scan at 47432,
  47475) — this is the relation frame golden, ALREADY on Poseidon2; UNCHANGED.
- `research/*.json` files — design/decision artifacts, not KAT/golden vectors; UNCHANGED.
- No Lean file references `hash_frame_to_challenge_bytes`/`challenge_bytes`/poseidon FS — the Lean boundary
  files in the working tree are unrelated to this change.

### D. Byte-size impact on the deployed signature: NONE
The Poseidon2 helper preserves `CHALLENGE_BYTES` / the 31-byte algebraic+hardening layout and the
last-byte mask. Deployed transaction signature stays **260 bytes** (and the reviewed Fq12 account-carried
transaction signature stays `REVIEWED_FQ12_ACCOUNT_CARRIED_TRANSACTION_SIGNATURE_BYTES`). PK stays 172 B.
Only challenge/response/opening/carrier VALUES move. No size constant edits.

---

## 4. Legitimate KAT/golden/fixture regeneration (NEVER hand-fake values)

The corpus is generated from the REAL sign/verify pipeline; that is the only sanctioned source. Steps:

1. Apply the §2 source edits (base/helper re-host + the deployed binary delegate at stateful.rs:3537).
2. Build: `cargo build -p descender-parameter-search` (default features — confirms Plonky3-free links).
3. Regenerate the corpus FROM THE REAL PIPELINE (this re-runs production_keygen/sign/verify and writes the
   bytes the verifier actually produces — never edit kat_v0.json by hand):
   ```
   cd tools/parameter-search
   cargo run --bin signature_q12289_local_delta_stateful -- --kat-generate
   ```
   This overwrites `formal/kat/kat_v0.json` with re-derived `signature_hex`/`challenge`/`response`/
   `openings`/`linear_sigma_proof`/`producing_version_digest`.
4. Re-sync the human index `formal/kat/kat_v0.md` (update the producing-version digest line + any quoted
   bytes; the file itself says regenerate, do not hand-edit semantics). The 260-byte layout table and
   reject-reason table are unchanged.
5. Verify reproduction (the CI gate):
   ```
   cargo run --bin signature_q12289_local_delta_stateful -- --kat-check
   ```
   Must print "reproduces byte-for-byte."
6. Review the diff: confirm ONLY value fields changed (challenge/response/openings/carrier/signature_hex/
   producing_version_digest), and that lengths / reject reasons / layout / slot semantics are byte-identical
   to the prior corpus. Confirm `kat_v0_sha256_archived.json` is UNCHANGED (it stays the SHA-era anchor).
7. Commit kat_v0.json + kat_v0.md together with the source edits in one atomic change so CI never sees a
   half-migrated state.

Provenance integrity: the producing-version digest is recomputed inside the binary from a fresh signature,
so it self-attests the new Poseidon2 logic. There is NO step where a value is typed by hand.

---

## 5. Test checklist

- [ ] `cargo build -p descender-parameter-search` (default features) succeeds; `cargo tree` shows ZERO p3-*
      crates -> deployed signer Plonky3-free.
- [ ] `--kat-generate` then `--kat-check` -> "reproduces byte-for-byte."
- [ ] `kat_v0_corpus_reproduces_frozen_json` (stateful.rs:7609) PASSES post-regen.
- [ ] `kat_v0_corpus_is_internally_consistent` (stateful.rs:7579) PASSES.
- [ ] `cargo test -p descender-parameter-search` full suite green (all the recompute-both-sides challenge
      round-trip tests in production_reference.rs + stateful.rs).
- [ ] production_reference.rs:102369 test repointed to the Poseidon2 helper and PASSES (challenge_bytes ==
      Poseidon2 helper of frame; still `!= legacy` frame).
- [ ] Relation-frame goldens (67517 golden_vector_and_roundtrip; byte-instantiation evidence at 4358) still
      PASS unchanged (no value drift — relation was already Poseidon2).
- [ ] Spend authorization binding round-trip:
      `require_path2_checked_core_authorization_challenge_binding` (60690) +
      `verify_path2_checked_core_authorization` accept a freshly-signed authorization and reject a mutated
      challenge (recompute test — passes after re-host since both sides moved).
- [ ] Diff audit: signature byte SIZES unchanged (260 B txn, 172 B pk); only values moved.
- [ ] `kat_v0_sha256_archived.json` unchanged; `kat_v0.md` digest line re-synced.
- [ ] Decide + document fate of the experimental local SHA copies in
      signature_q12289_compact_format.rs:501 and signature_q12289_implicit_verifier.rs:503 (out of deployed
      scope; comment as legacy SHA experiment or re-host for consistency).
- [ ] Dossier/assumption note: record FS assumption move SHA-256-RO -> Poseidon2-sponge-RO now covers ALL
      spend FS (txn + authorization + relation), under the already-carried `A_poseidon2_*` named-standard.
