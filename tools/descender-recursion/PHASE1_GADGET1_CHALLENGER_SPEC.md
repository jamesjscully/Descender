# Phase-1 Gadget 1 — In-AIR DuplexChallenger (Fiat–Shamir sponge)

> MIGRATION STATUS (2026-06-20): `ACTIVE_SUPPORT`.
> This is a supporting recursion gadget spec note, not a normative spec. Current
> normative authority starts at `../../spec/README.md`.

Status: SPEC (analysis only — no code, no build).
Scope: gadget 1 ONLY. Gadgets 2–5 are out of scope for this run.
Single source of truth: `descender_account_stark::config` (re-exported by `descender-recursion/src/lib.rs`).
Reference impl this gadget must mirror constant-for-constant:
`~/.cargo/registry/.../p3-challenger-0.6.1/src/duplex_challenger.rs` (`DuplexChallenger`),
`grinding_challenger.rs` (`check_witness`), plus the leaf transcript order in
`p3-uni-stark-0.6.1/src/verifier.rs` and `p3-fri-0.6.1/src/verifier.rs`.

---

## 0. Pinned constants (lifted, do NOT re-declare)

From `descender-account-stark/src/config.rs`:

| Name | Value | Source |
|---|---|---|
| `WIDTH` | 16 | `config::WIDTH`, `Perm = Poseidon2KoalaBear<16>` |
| `RATE` | 8 | `Challenger = DuplexChallenger<Val, Perm, 16, 8>` |
| `CAPACITY` | `WIDTH - RATE = 8` | derived |
| `Val` | `KoalaBear` (`p = 2^31 − 2^24 + 1 = 0x7f000001`) | `config::Val` |
| `Challenge` | `BinomialExtensionField<Val, 4>` (D = 4, ~124-bit) | `config::Challenge` |
| `Perm` | `Poseidon2KoalaBear<16>` (PINNED `default_koalabear_poseidon2_16`) | `config::Perm` |
| `PROD_NUM_QUERIES` | 56 | FRI query count |
| `PROD_QUERY_POW_BITS` | 16 | query-phase grind bits |
| `PROD_COMMIT_POW_BITS` | 0 | commit-phase grind bits (no commit grind) |
| `PROD_LOG_BLOWUP` | 4 | FRI rate exponent |
| `DIGEST_ELEMS` | 8 | Poseidon2-Merkle digest = `RATE` field elems |

KoalaBear sampling constants the bit-draw MUST replicate
(`p3-koala-bear-0.6.1/src/koala_bear.rs`):
`MAX_SINGLE_SAMPLE_BITS = 24`; `SAMPLING_BITS_M[k] = PRIME & !((1<<k)-1)` for `k>0`.
`ORDER_U64 = PRIME = 0x7f000001`; `as_canonical_u64` = the canonical (Montgomery-reduced)
representative in `[0, p)`.

NOTE: the leaf FRI verifier calls **`sample_bits`** for query indices (the biased fast
path, `duplex_challenger.rs:282`), NOT `sample_uniform_bits`. Gadget 1 mirrors `sample_bits`.
The `UniformSamplingField` / `SAMPLING_BITS_M` machinery is documented here only so a future
switch to `sample_uniform_bits` (if the leaf changes) is a localized edit; it is NOT on the
Phase-1 critical path.

---

## 1. Exact reference semantics (the contract to reproduce)

State: `sponge_state: [F;16]`, `input_buffer: Vec<F>` (len ≤ 8), `output_buffer: Vec<F>`.
`new()` ⇒ all-zero state, both buffers empty.

### 1.1 `duplexing()` (`duplex_challenger.rs:86–112`)
Let `n = input_buffer.len()` (assert `n ≤ RATE`).
1. For `i in 0..n`: `sponge_state[i] = input_buffer[i]` (overwrite, NOT add). Drain buffer.
2. If `n > 0` (absorb): `sponge_state[n..RATE].fill(0)`; `sponge_state[RATE] += n` (the
   length tag, `F::from_u8(n)`, added into the FIRST capacity slot, index 8).
   If `n == 0` (squeeze): rate slots are left untouched; no tag added.
3. `permutation.permute_mut(&mut sponge_state)` (one Poseidon2-16 permutation).
4. `output_buffer.clear(); output_buffer.extend(&sponge_state[..RATE])` (8 elems, in order).

### 1.2 `observe(value: F)` (`:166`)
`output_buffer.clear(); input_buffer.push(value); if input_buffer.len()==RATE { duplexing() }`.
- Arrays / `Hash<F,F,N>` / `MerkleCap` / `Vec<Vec<F>>`: observe each element in order (`:178–244`).
  A Poseidon2-Merkle commitment is `Hash<F,F,8>` ⇒ observes its 8 digest elems one by one.
- `observe_slice(public_values)`: observe each elem in order.
- Bytes are NEVER observed in this pipeline: the leaf is Poseidon2 end-to-end; the challenger
  only ever sees `Val` field elements and field-element commitments. The byte path
  (`SerializingHasher`, `into_byte_stream`) is used by Keccak/SHA Merkle stacks, which the
  PINNED config does NOT use. **Gadget 1 has NO byte-absorb path.** (If a future config feeds
  bytes, they would arrive pre-packed as `Val` via the hasher; out of scope.)

### 1.3 `sample() -> EF` over `Challenge` (D=4) (`:253–265`)
`EF::from_basis_coefficients_fn(|_| { ... })` called D=4 times. Each coefficient:
`if !input_buffer.is_empty() || output_buffer.is_empty() { duplexing() }`
then `output_buffer.pop()` (pop from the **BACK / high index**).
So one `Challenge` sample consumes 4 field draws, each popping the back of `output_buffer`;
a duplexing refills the buffer with `sponge_state[0..8]` and pops index 7 first.

### 1.4 `sample_bits(bits) -> usize` (`:282–288`)
`assert!(bits < usize::BITS); assert!((1<<bits) < ORDER_U64);`
`let r: F = self.sample();  // ONE base-field draw (D=1), not a Challenge`
`return r.as_canonical_u64() as usize & ((1<<bits) - 1)`  (low `bits` bits of canonical rep).

### 1.5 `check_witness(bits, w)` (`grinding_challenger.rs:42–47`)
`observe(w); sample_bits(bits) == 0`. Used for FRI PoW: query grind `bits = 16`; commit grind
`bits = 0` (with `PROD_COMMIT_POW_BITS=0`, `sample_bits(0)` ⇒ `(1<<0)-1 = 0` mask ⇒ always 0
⇒ check passes but STILL performs `observe(w)` then a `sample()` — both mutate the transcript and
MUST be modeled). `w` is a single `Val` witness.

---

## 2. Exact transcript order the leaf produces (the script gadget 1 must replay)

From `p3-uni-stark-0.6.1/src/verifier.rs:400–510` then `p3-fri-0.6.1/src/verifier.rs:195–360`.
This ordered list is the canonical "transcript program" the AIR encodes (each step is one or more
observe/sample ops). `O(x)`=observe, `S`=sample Challenge (4 draws), `Sb(k)`=sample_bits(k),
`CW(b)`=check_witness = `O(w);Sb(b)`.

```
1.  O(degree_bits as Val)
2.  O(base_degree_bits as Val)
3.  O(preprocessed_width as Val)
4.  O(commitments.trace)            // Hash<Val,8> -> 8 observes
5.  [if preprocessed_width>0] O(preprocessed_commit)  // leaf: 0, skipped
6.  O(public_values)                // observe_slice, in order (~the leaf IO vector)
7.  alpha = S                       // constraint-batching challenge (Challenge, 4 draws)
8.  O(commitments.quotient_chunks)  // Hash<Val,8> -> 8 observes
9.  [if ZK] O(commitments.random)   // HidingFriPcs ON -> present, 8 observes
10. zeta = S                        // OOD point (Challenge, 4 draws)
    --- enter PCS / FRI verify(coms, opening_proof, challenger) ---
11. (per opened matrix round) O(commitment); S (=opening point); then per opened point
    O(point as Challenge slice)     // p3-fri verifier.rs:917-946 batch-open transcript
12. alpha_fri = S                   // FRI batching alpha (fri/verifier.rs:195)
13. for each commit-phase layer i in 0..L:
        O(commit_phase_commits[i])  // Hash<Val,8> -> 8 observes
        CW(PROD_COMMIT_POW_BITS=0)  // O(commit_pow_witnesses[i]); Sb(0)
        beta_i = S                  // per-round fold challenge (fri/verifier.rs:306-310)
14. O(final_poly)                   // observe_algebra_slice: each Challenge -> 4 observes
15. for &log_arity in log_arities: O(Val::from_usize(log_arity))
16. CW(PROD_QUERY_POW_BITS=16)      // O(query_pow_witness); Sb(16)
17. for q in 0..PROD_NUM_QUERIES(56):
        index_q = Sb(log_global_max_height + extra_query_index_bits())
```

`log_global_max_height = sum(log_arities) + log_blowup + log_final_poly_len`
(`fri/verifier.rs:256`). The recursion AIR treats the per-query bit width as a CONSTANT computed
from the (public, shape-checked) inner proof header — see §6 for why it must be a fixed circuit
parameter, not witness-chosen.

The AIR does NOT need to know the semantics of alpha/zeta/beta/index — only that it reproduces the
challenger op-sequence in this exact order so the sampled values equal the leaf prover's.

---

## 3. AIR column layout

The challenger is a **sequential state machine unrolled over rows**: one row per challenger
"event". An event is the smallest atomic transition: a single `observe`, or a single field-draw
inside a `sample`/`sample_bits`, or a `duplexing`. This keeps each transition low-degree and lets
the Poseidon2 permutation (gadget-2/Poseidon2-air) be invoked exactly when a row's `is_permute`
flag is set.

### 3.1 Per-row columns

State carried across rows (the "registers"):

```
sponge[0..16]        : [F;16]   full sponge state at START of row
in_buf[0..8]         : [F;8]    input buffer contents (slot j valid iff j < in_len)
in_len               : F        input buffer length, 0..=8
out_buf[0..8]        : [F;8]    output buffer contents (squeeze register)
out_len              : F        output buffer length, 0..=8
```

Per-row operation selector (mutually exclusive booleans; exactly one set, enforced by
`sum = 1` and each `b*(b-1)=0`):

```
is_observe           : F   absorb one field elem this row
is_sample_draw       : F   pop one field draw this row (component of sample / sample_bits)
is_permute           : F   this row performs a duplexing-permute (paired with the Poseidon2 sub-AIR)
is_pad               : F   inactive tail row (no-op, state frozen)
```

Per-row I/O / payload:

```
observed_val         : F   the field element absorbed (valid when is_observe)
absorb_len_tag       : F   = in_len at the moment a permute fires from a full/absorb buffer
                            (n in §1.1 step 2); 0 on a squeeze-permute
draw_out             : F   the field element popped this row (valid when is_sample_draw)
is_squeeze_permute   : F   1 if this permute had n==0 (no rate overwrite, no tag) else 0
bit_mask_k           : F   the (1<<k)-1 mask for a sample_bits row (only on the row whose draw
                            feeds a sample_bits; constant per query, see §5)
bits_out             : F   = draw_out.canonical & bit_mask_k  (the query index / grind check)
```

A `transcript_step_id` (public-table index) column ties each active row to the §2 program so the
fixed selectors (which op, with what `k`) are PREPROCESSED, not witness-chosen (§6, security).

### 3.2 Poseidon2 coupling

`is_permute` rows are the ONLY rows that change the sponge nonlinearly. The 16 input cells to the
permutation are this row's POST-absorb state `sponge_in'` (see §4.2); the 16 output cells are next
row's `sponge`. The permutation itself is gadget 2's Poseidon2-air instance over the SAME PINNED
`Poseidon2KoalaBear<16>` round constants. Gadget 1 only exposes the 16-in / 16-out wire bundle and
asserts `permute(sponge_in') == sponge_next` via the shared Poseidon2 sub-AIR (cross-AIR bus or
in-lined columns — interface decision deferred to gadget 2; gadget 1 fixes the wire contract:
`(sponge_in'[0..16]) -> (sponge_next[0..16])`).

---

## 4. Transition constraints

All constraints are over consecutive rows `(local, next)`. `'` denotes the next row.
Booleanity + `selector-sum = 1` enforced on every non-first, non-final-pad row.

### 4.1 `is_observe` row
Mirrors §1.2 (NO permute on this row unless the push fills the buffer — see 4.1b).
- `out_len' = 0` and `out_buf' = 0` (observe clears output buffer).
- `in_buf'[in_len] = observed_val`; `in_buf'[j] = in_buf[j]` for `j < in_len`;
  `in_buf'[j]` unconstrained-but-zeroed for `j > in_len` (use `j*` masking by `in_len` selector).
- `in_len' = in_len + 1`.
- `sponge' = sponge` (state unchanged by a plain observe).
- Constraint that "buffer not yet full": enforce `in_len < RATE` on a pure observe row.

### 4.1b Auto-duplex on full buffer
The reference does `if input_buffer.len()==RATE { duplexing() }` INSIDE observe. Model this as the
observe row being immediately followed by an `is_permute` row whose `absorb_len_tag = RATE = 8`.
Equivalently: when `in_len + 1 == RATE`, the NEXT row MUST be `is_permute` with `n = 8`. Enforce
with a selector-transition rule: `(is_observe AND in_len'==RATE) ⇒ next.is_permute`.

### 4.2 `is_permute` row (the duplexing, §1.1)
Let `n = in_len` (absorb length). Define POST-absorb pre-permute state `sponge_in'`:
- If `is_squeeze_permute = 1` (n==0): `sponge_in'[i] = sponge[i]` for all 16 (rate untouched).
- Else (absorb): for `i < n`: `sponge_in'[i] = in_buf[i]`; for `n ≤ i < RATE`: `sponge_in'[i] = 0`;
  `sponge_in'[RATE] = sponge[RATE] + n` (length tag into capacity slot 8);
  `sponge_in'[i] = sponge[i]` for `RATE < i < 16` (rest of capacity carried).
  Enforce `absorb_len_tag = n` and `is_squeeze_permute = 0` here; `n = 0 ⇒ is_squeeze_permute = 1`
  (so the n==0 absorb branch can never be taken — matches reference: empty buffer ⇒ squeeze).
- Poseidon2: `sponge' = permute(sponge_in')` (gadget-2 bus, §3.2).
- `in_buf' = 0`, `in_len' = 0` (buffer drained).
- `out_buf'[i] = sponge'[i]` for `i in 0..RATE`; `out_len' = RATE = 8` (refill).

Selector trigger rules for permute (so it fires EXACTLY when the reference duplexes):
1. Auto-duplex from full observe buffer (§4.1b): `n = 8`.
2. Sample/sample_bits with `in_buf` nonempty OR `out_buf` empty (§1.3 / §4.3): permute precedes
   the draw. `n = in_len` (could be 1..8 — a pending-input squeeze-or-absorb, matches
   `!input_buffer.is_empty() ⇒ duplexing()`).
3. NEVER otherwise.

### 4.3 `is_sample_draw` row (one field draw, §1.3 inner closure)
Precondition handled by an immediately-preceding `is_permute` row when needed (rule 4.2-trigger 2);
on a draw row the output buffer is guaranteed nonempty. Pop from the BACK:
- `draw_out = out_buf[out_len - 1]`.
- `out_len' = out_len - 1`; `out_buf'[j] = out_buf[j]` for `j < out_len-1`; tail zeroed.
- `sponge' = sponge`, `in_buf' = in_buf`, `in_len' = in_len` (a draw does NOT touch state/input).
- A `Challenge` sample = 4 consecutive `is_sample_draw` rows (basis coeff order 0..4, popping
  index out_len-1 downward); the recursion's higher gadgets read the 4 popped `draw_out`s as the
  Challenge limbs in coefficient order.

### 4.4 `sample_bits` decomposition (§1.4, §5)
A `sample_bits(k)` is ONE `is_sample_draw` row (D=1, a single base draw) with the bit-extraction
constraints of §5 attached (`bits_out = canon(draw_out) & ((1<<k)-1)`).

### 4.5 `check_witness(b, w)` (§1.5)
Encoded as: one `is_observe` row with `observed_val = w`, immediately followed by the
`sample_bits(b)` decomposition. For the query grind (`b=16`) the verifier checks
`bits_out == 0` (boundary/public constraint on that row). For commit grind (`b=0`,
`PROD_COMMIT_POW_BITS=0`) the mask is 0 so `bits_out == 0` is automatic — but the `observe(w)` row
and the draw row still exist and still mutate the transcript (do NOT optimize them away; dropping
them desyncs every later challenge).

### 4.6 Boundary constraints
- First row: `sponge = 0` (16 zeros), `in_len = 0`, `out_len = 0`, all buffers 0 (`new()`).
- Public-input binding: the §2 transcript program (which op on which row, and the bytes/elems
  observed at steps 1–17) is bound to the inner proof's public values + commitments via gadget 5
  (`public_io`) and gadgets 2–4 (which supply `commitments.*`, `final_poly`, `log_arities`,
  PoW witnesses as their own witness/IO). Gadget 1 asserts only the SPONGE arithmetic; it
  CONSUMES the observed stream as constrained inputs from those gadgets.

### 4.7 Degree note
Every transition is degree ≤ 2 in the challenger registers (overwrites, adds, equality, single
multiply for masked-buffer selects), EXCEPT the Poseidon2 permute which is the sub-AIR's own degree
(handled by gadget 2). `sample_bits` bit-decomposition (§5) adds the only multi-row arithmetic
gadget purely inside gadget 1.

---

## 5. `sample_bits` — drawing query indices in-AIR (exact)

Reference (`duplex_challenger.rs:282–288`): `r = sample(); r.as_canonical_u64() as usize & ((1<<bits)-1)`.

In-AIR realization on the draw row:
1. `r = draw_out` (a `Val`; one base-field draw from the sponge).
2. Canonical decomposition: introduce 31 boolean bit columns `c_0..c_30` with
   `sum_{j=0}^{30} c_j * 2^j = canon(r)` and each `c_j*(c_j-1)=0`. `canon(r)` is the canonical
   representative in `[0, p)`; since `r` is already a field element equal to its canonical value as
   an integer < p, the constraint is `sum c_j 2^j == r` (as field elements) PLUS a range proof that
   the 31-bit value is `< p = 0x7f000001` (standard "is-canonical" lt-check against the prime; this
   is exactly what `as_canonical_u64` guarantees and what `sample_bits`'s
   `assert!((1<<bits)<ORDER_U64)` relies on).
3. `bits_out = sum_{j=0}^{k-1} c_j * 2^j` (the low `k` bits) — `k` is the FIXED, preprocessed bit
   width for that transcript step (constant per call site; for the 56 query rows it is the SAME
   constant `K = log_global_max_height + extra_query_index_bits()`, derived from the shape-checked
   inner-proof header, NOT witness-chosen — see §6).
4. The query index `index_q = bits_out` is exported to gadget 2/3 (Merkle/FRI open at that index).
   For grind rows, the constraint is `bits_out == 0`.

We deliberately mirror `sample_bits` (the biased low-bits path), NOT `sample_uniform_bits`. The
KoalaBear `SAMPLING_BITS_M` rejection thresholds and the two-half slow path
(`duplex_challenger.rs:405–440`) are NOT used by the leaf FRI verifier, so they are NOT constrained.
(Documented in §0 for forward-compatibility only.)

Bias note (carry as soundness obligation, not a bug): `sample_bits` is `1/p`-close to uniform per
the reference proof (`:274–281`); the recursion inherits exactly this bias since it reproduces the
same draw. No additional loss introduced by the AIR.

---

## 6. Security / soundness obligations specific to gadget 1

1. **Op-program is PREPROCESSED, not witness-chosen.** Which row is observe vs draw vs permute,
   and each `sample_bits` width `k`, MUST be fixed columns derived from the (shape-validated) inner
   proof header. If a malicious prover could relabel an observe as a draw, or shrink `k`, it could
   resample challenges off a forged transcript. The selector layout is bound to a public
   `transcript_step_id` table. (Mirror of: the leaf verifier's control flow is not adversary-chosen.)
2. **Length-tag binding (`sponge[RATE] += n`).** The +n tag (§4.2) is what makes the absorb
   prefix-free; dropping it lets `[5,7]` and `[5,7,0,0]` collide (reference test
   `test_absorb_is_length_binding`). The AIR MUST add `n` into capacity slot 8 on every absorb
   permute. Negative test 7.3.
3. **Overwrite, not add, on rate absorb.** §1.1 step 1 OVERWRITES `sponge[i]` with the input
   (capacity carries; rate is replaced). A buggy "add" gadget would still be a sponge but a
   DIFFERENT one ⇒ wrong challenges. Negative test 7.2.
4. **Pop from the back.** `output_buffer.pop()` is LIFO (index `out_len-1`). Popping the front
   would reorder challenge limbs. Negative test 7.4.
5. **No byte path.** Asserting the leaf never byte-absorbs (§1.2) is a config invariant; if a
   future config introduces SHA/Keccak Merkle, this gadget is INCOMPLETE and must be revisited
   (flag, do not silently accept).
6. **Challenge field margin.** Each `S` is a degree-4 (~124-bit) Challenge; the recursion unions
   over many sampled challenges, eroding the 128-bit margin (lib.rs open obligation). Gadget 1 does
   not fix this; it faithfully reproduces the draws so the budget is accountable downstream.

---

## 7. Negative tests (forge ⇒ different sampled challenge)

Discipline (MEMORY lesson): test hiding/soundness by ATTEMPTED RECOVERY / divergence, never by
substring scan. Each test runs the in-AIR challenger AND the native
`config::Challenger` (`DuplexChallenger<Val,Perm,16,8>`) on the same op-stream and asserts EQUALITY
on the honest path, then DIVERGENCE on each forgery.

7.1 **Golden equivalence (positive).** Replay the full §2 transcript (real inner proof) through
both the native `Challenger` and the AIR trace; assert every sampled `alpha, zeta, beta_i,
index_q` and every grind `bits_out` matches the native challenger element-for-element. This is the
constant-for-constant anchor.

7.2 **Wrong absorb (add vs overwrite).** Mutate one observed commitment limb; assert the AIR's
sampled `zeta` (and all later challenges) differ from the unmutated run AND match a native
`Challenger` fed the SAME mutation (i.e. the AIR tracks the real sponge, not a degenerate one).

7.3 **Length-tag stripped.** Build two transcripts differing only by trailing-zero padding of one
absorb (`[5,7]` vs `[5,7,0,0]`, distinct `n`); assert the AIR produces DIFFERENT challenges
(reproduces `test_absorb_is_length_binding`). A gadget that omits the +n tag would make them equal —
that variant MUST fail the test.

7.4 **Buffer pop order flipped.** Force a configuration where front-pop vs back-pop differ; assert
the AIR's Challenge limbs match native LIFO order and that a front-pop reference diverges.

7.5 **Wrong public IO (forged statement).** Flip one element of `public_values` (step 6); assert
`alpha` and every downstream challenge change. (This is the core Fiat–Shamir forgery: a wrong
transcript MUST change the sampled challenge — exactly the prompt's attempted-forgery requirement.)

7.6 **Wrong query-index width `k`.** Attempt a trace where a `sample_bits` row uses `k-1`; assert
the preprocessed-width constraint (§5 step 3 / §6 #1) rejects it (the index would otherwise land in
a smaller range, letting the prover dodge a query). Should be UNSATISFIABLE, not "produces a
different but accepted index".

7.7 **Grind check.** Feed a `query_pow_witness` whose `sample_bits(16) != 0`; assert the
`bits_out == 0` boundary constraint (§4.5) is violated ⇒ trace rejected. Also: assert the commit
grind (`b=0`) rows are PRESENT (dropping them desyncs later betas — diff test).

7.8 **Permute-trigger fidelity.** Assert a permute fires iff (buffer full) OR (sample with
`in_buf` nonempty or `out_buf` empty); construct a trace with a spurious/missing permute and assert
it diverges from native. (Guards §4.2 trigger rules — the subtle `sample()` auto-duplex condition.)

---

## 8. How gadget 1 plugs into `recursive_air`

- `recursive_air` (lib.rs `mod recursive_air`) assembles gadgets 1–5 as column blocks of ONE
  verifier-AIR. Gadget 1 owns the challenger register columns (§3.1) + the Poseidon2 permute bus to
  gadget 2.
- **Producers of the observed stream:** gadget 5 (`public_io`) supplies steps 1–3,6;
  gadgets 2–4 supply the commitments (step 4,8,9,11,13), `final_poly` (14), `log_arities` (15),
  and PoW witnesses (13,16). Gadget 1 CONSUMES these as constrained inputs (cross-block equality),
  it does not re-derive them.
- **Consumers of the sampled stream:** gadget 4 (`constraint_eval`) consumes `alpha`, `zeta`;
  gadget 3 (`fri_fold`) consumes `alpha_fri`, each `beta_i`; gadgets 2/3 consume each `index_q`
  from §5. The wire contract is: gadget 1 EXPORTS `(alpha, zeta, alpha_fri, [beta_i], [index_q;56])`
  as named columns at the rows where they are finalized, and the consuming gadgets reference those
  exact cells.
- **Ordering invariant:** the verifier-AIR is correct iff gadget 1's op-program (§2) is replayed in
  the precise order the leaf verifier uses; `transcript_step_id` is the shared index that aligns
  gadget 1's rows with the data-supplying gadgets' rows. Any reorder is caught by 7.1/7.5.
- **Single source of truth:** all constants come from the re-exported `config` (§0); gadget 1
  re-declares NONE of WIDTH/RATE/field/Poseidon2 — a leaf config drift breaks the build at the
  re-export (lib.rs `phase1_target_present`) and the golden test 7.1.
- Phase-2/3: the same challenger gadget is reused for the {creation+spend} co-verification and the
  block-fold IVC; no gadget-1 change anticipated unless the spend FS frame is re-hosted to
  Poseidon2 (then a second challenger instance, same gadget, different op-program).

---

## 9. Out of scope (explicit)

Gadgets 2 (`merkle_verify`), 3 (`fri_fold`), 4 (`constraint_eval`), 5 (`public_io`), the
`recursive_air` assembly, the recursive-FRI knowledge extractor, the outer-STARK Johnson params,
and the QROM/FS soundness seam are NOT specified here (see `lib.rs` §"Open soundness obligations"
and `research/path2_recursion_circuit_concrete_design.md`). The Poseidon2 permutation internals are
gadget 2; gadget 1 fixes only the 16-in/16-out wire contract (§3.2).
