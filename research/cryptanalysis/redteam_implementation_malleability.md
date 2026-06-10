# RED-TEAM — Implementation / Malleability (VECTOR 2)

**Date:** 2026-06-10 · **Target:** `tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs`
(+ shared `production_reference.rs` encoders). · **Verdict:** **BREAK FOUND — signature
malleability / strong-unforgeability (SUF-CMA) violation in the deployed verifier code.**

---

## TL;DR (the break)

Given ONE honest signature on a message, an attacker with **no secret and no witness
knowledge** can produce a **byte-distinct second signature that the verifier accepts
for the same `(message, slot)`** — and in fact up to **2^26 ≈ 6.7×10^7** distinct
accepting signatures. This contradicts the dossier's **SUF-CMA** claim (`SECURITY_DOSSIER.md
§1`). It survives on BOTH the simple quotient verifier and the **deployable
proof-carrying path** (`reference_verify_with_proof` → `verify_public_quotient_proof_signature_with_mode`).

Reproduced by two passing Rust tests added to the binary's `tests` module:
- `tests::redteam_response_opening_malleability_distinct_accepting_signature` (quotient path)
- `tests::redteam_response_opening_malleability_deployable_proof_path` (deployable proof path)

```
$ cd tools/parameter-search && cargo test --bin signature_q12289_local_delta_stateful redteam_response_opening
running 2 tests
test tests::redteam_response_opening_malleability_distinct_accepting_signature ... ok
test tests::redteam_response_opening_malleability_deployable_proof_path ... ok
```
Full suite stays green (30 passed) — the break does not disturb the existing harness.

---

## Root cause

The per-rep relation check in
`verify_stateful_signature_with_hidden_preimage_relation` (line ~3960) and the
identical `verify_local_delta_relation_with_masked_witness` (line ~4053, used by the
deployable proof path) reconstruct, **per repetition**:

```
mask_view[c] = response[c] - sign * public_view.free[block][c]   (mod q)   // response ∈ [-48,48]
assert local_state(mask_view) == fiber_state                               // a + 50*b ≡ fiber
assert openings[c] == mask_view[c] + pad[c]                                (mod q)
```

where `local_state(mask_view) = mask_view[0] + 50*mask_view[1] (mod q)` (`LOCAL_SYNDROME_SLOPE = 50`).

**The verifier accepts ANY `mask_view` satisfying `local_state == fiber_state` with the
response in the box [-48,48].** It does NOT pin `mask_view` to the signer's canonical
choice. The signer always uses `mask_view_for_fiber_state(...)` = the FIRST lattice
point found in the double loop (`build_local_delta_..` / `reference_sign`), but the
verifier does not require canonicity.

The local-delta relation `a + 50·b ≡ fiber (mod 12289)` has a **coded kernel vector
`(50, −1)` with L∞ coeff = 50 ≤ 96 — well INSIDE the box**. So for the reference
fiber_state there are exactly **two** in-box mask_views per rep:

```
fiber_state = local_state([12, q-12]) = 12 + 50*(q-12) ≡ -588 ≡ 11701 (mod 12289)
in-box mask_views hitting it: (12, -12)  [signer's canonical]  and  (-38, -11)  [the malleated one]
                              (-38) + 50*(-11) = -588  ✓   both coords in [-48,48]  ✓
```

The challenge is Fiat–Shamir-bound to `(pk, message, slot_id, slot.digest[, proof_commitment])`
— but it binds **neither the response bytes nor the opening bytes**. `fiber_state` comes
from the (unchanged) `masked_witness_response`. So swapping `(12,-12)→(-38,-11)` and
recomputing the openings (`mask_view + pad`) yields a fresh signature the verifier
accepts. With the all-zero reference public key the `free` coordinates vanish, so the
response equals the mask_view directly and the malleated response trivially stays in box.

### Amplification: ~2^26 distinct signatures per message
The verifier loop is **independent per repetition** (26 reps; each reconstructs its own
mask_view, checks the same `fiber_state`, checks its own opening). Each rep independently
admits either of the 2 in-box mask_views ⇒ **2^26 distinct accepting signatures** for a
single message. The PoC flips all 26 reps at once; per-rep mixing also verifies.

---

## Why the 114 > 96 defense does NOT cover this

`SECURITY_DOSSIER.md §3(a).4` / `beyond_box_VERDICT.md` claim "min coded kernel vector
coeff 114 > 96, so no forgeable short response difference." That certificate is about the
**full-response CODED-SIS** lattice (the 12×23 `reviewed_response_link_code` instance) —
NOT about the **local-delta per-rep opening relation** `a + 50·b ≡ fiber` that the
stateful verifier actually executes. The local relation's kernel vector `(50,−1)` has
coeff 50, sits inside [−96,96], and the response box is [−48,48]. The 114>96 enumeration
provides **no protection** for the relation the deployed verifier checks. This is a
relation-mismatch between the proven object and the deployed check.

---

## Severity

**HIGH for the stated SUF-CMA goal; impact-bounded by the consume-once lifecycle.**

- **SUF-CMA is broken outright:** the definition forbids producing ANY new accepting
  signature on a queried message; we produce ~2^26 of them with no key/witness. The
  dossier's headline claim (`§1`: "SUF-CMA in the QROM") is contradicted by the code.
- **Replay/dedup risk:** any layer that dedups, caches, or pins replay-protection on the
  *signature bytes* (or that processes a tx in a mempool before slot consumption is
  finalized) can be made to see two "different" valid signatures for one intended
  spend — classic transaction-malleability (cf. the BTC/segwit class of bugs).
- **Mitigant:** the consume-once slot lifecycle (`slot.unused`) means the SAME slot
  cannot be re-spent stateful-side once consumed, so this is not by itself a
  double-spend of value. That's why severity is HIGH, not critical. But uniqueness /
  non-malleability is a stated security property here and it is false.

---

## Reproduction (exact)

Tests are in the `tests` module of
`tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs`.

Core of the forgery (no secret used):
```rust
let canonical = [12u16, Q - 12];
let fiber_state = local_state(canonical);
let alt = [mod_q(-38), mod_q(-11)];       // second in-box mask_view, same fiber_state
assert_eq!(local_state(alt), fiber_state);
// rebuild response = alt + sign*free (free==0 here), opening = alt + pad, per rep
// splice into signature[response_start..opening_start] and the opening region
// -> verify_quotient_stateful_signature / reference_verify_with_proof both ACCEPT
```

Run:
```
cd /home/jack/code/Descender/tools/parameter-search
cargo test --bin signature_q12289_local_delta_stateful redteam_response_opening
```

Count check (2 in-box solutions per rep ⇒ 2^26 signatures):
```
python3 -c "Q=12289;S=50;B=48;f=(12+S*(Q-12))%Q;print([(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a+S*b)%Q==f])"
# -> [(-38, -11), (12, -12)]
```

---

## Fix recommendations

The verifier must enforce a **canonical** mask_view, e.g. one of:
1. Recompute the canonical `mask_view = mask_view_for_fiber_state(fiber_state)` and
   require the reconstructed mask_view to equal it (reject any other in-box solution).
2. Bind the response AND opening bytes into the Fiat–Shamir challenge frame so they are
   no longer free to re-solve.
3. Reduce the box / change the slope so the local relation has no in-box kernel vector
   (i.e. lift the local-delta relation to the same coeff>box regime the 114-certificate
   proves), and prove the local relation — not just the full-response code — has min
   coded-kernel coeff > 2·RESPONSE_BOUND.

Any of these closes the malleability; (1) is the minimal code change.

---

## What I also checked (no break on these)

- **Response bound parse (line 1258, `raw > (2*RESPONSE_BOUND)`):** raw∈[0,96]→[-48,48],
  a bijection; symmetric; no off-by-one giving an out-of-range residue. Clean.
- **Opening residue check (line 1284, `value >= Q`):** strict; no `>=` vs `>` slack;
  residues forced canonical. Clean.
- **Challenge canonical-high-bits check (line 1198) + active_block range (1206) + sign
  bit:** active_block is `read_bits_le(...,4)` ∈[0,15] always < GROUPS=16 (range check is
  dead but not exploitable); sign maps 0/1 → +1/−1 bijectively. No 0x3f-mask / sign-flip
  malleability *in the challenge itself* (it is FS-recomputed and compared byte-exact at
  line 3819 / 4295). The malleability is in response/opening, which are NOT FS-bound.
- **Length confusion:** all entry points check `signature.len() == expected` and
  `public_key.len() == PK_BYTES` before slicing; slot_id is range-checked via
  `slots.get(...)`. No overflow/length-confusion path found.
- **masked_state18 14-bit packing slack bits:** packed-tail-zero is enforced by the
  sampler/`packed_tail_zero` discipline in tests; the deployed `unpack_q_*` reject
  out-of-range residues. The malleability above does not require touching these.

The decisive gap is the **non-canonical mask_view acceptance** in the local-delta
opening relation — the verifier checks a *linear constraint with an in-box kernel
vector* instead of pinning the unique signer representative.
