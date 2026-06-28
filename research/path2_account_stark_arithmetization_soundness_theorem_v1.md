# Path2AccountStarkArithmetizationSoundnessTheoremV1

> Classification: `ACTIVE_SUPPORT`. This theorem supports the active
> Poseidon2/Plonky3 account-creation path (`ADR-0004`, tracker row
> `DIR-STARK-001`). It is positive soundness evidence for the spec spine and
> tracker, not normative authority by itself. It upgrades the adversarial
> hole-hunt in `research/path2_account_stark_air_soundness_audit.md` into a
> clause-by-clause positive theorem.

Status: arithmetization-soundness theorem for the `descender-account-stark` AIR.

## Purpose And Risk Position

This document discharges the highest proof-collapse-risk obligation —
**"the account-STARK verifier proves exactly the intended account-creation
statement"** — at the *arithmetization* layer. It proves the positive
direction the prior audit only checked negatively:

```text
any trace T that satisfies every AIR constraint, under a public-input vector pv
that binds the statement, decodes committed columns to a witness that satisfies
the in-circuit account-creation relation R_air — there is no satisfying trace
that encodes a false R_air statement.
```

It is deliberately scoped. It does **not** prove acceptance ⟹ a satisfying
trace exists (that is the FRI argument-of-knowledge extractor, the separate
open obligation owned by `DIR-PROOF-001`/`DIR-PROOF-002`; see
`research/path2_residual_assumption_ledger_v1.md`), and it does **not** prove
the Fiat–Shamir transcript binds `pv` (the account-STARK transcript QROM
obligation, `DIR-QROM-001`). It closes the middle layer between those two.

## Scope: R_air Versus R_create

The full creation relation `CreationHiddenSourceRelationV1`
(`research/creation_hidden_source_relation_v1_spec.md`) has seven clauses.
Not all are arithmetized — by design, the SHA-256 leaf/authorization-digest
bindings and the admission-namespace uniqueness checks are enforced at the
verifier-wiring boundary, not in the constraint polynomial. We therefore
factor `R_create` into:

```text
R_create  =  R_air  ∧  R_wiring  ∧  R_namespace
```

- **R_air** — the in-circuit sub-relation proven here: clauses 2, 3, 4, 5
  (statement recomputation over the active lanes), and 6, plus the
  identity/context-binding portion of clause 1, plus the deployed-anchor
  (role-root + packed-witness) bindings.
- **R_wiring** — clause 1 SHA-256 leaf / `slot_authorization_digest` /
  evidence-duplication equalities and the pad-seed SHA opening. Enforced by the
  deployed verifier (`tools/descender-account-verifier/src/lib.rs`) and owned by
  `DIR-BIND-001` (replay/anchor) and the verifier-boundary checks.
- **R_namespace** — clause 7 admission uniqueness `(account_id, slot_id)` and
  `pad_seed_digest` no-duplicate, and the live state-root transition. Owned by
  `DIR-LIFE-001` and the admission boundary.

This factoring is the honest read of the as-built circuit and avoids the
historical over-reach (claiming the AIR proves digest bindings it does not).
`R_wiring` and `R_namespace` are tracked open elsewhere; this theorem closes
`R_air`.

## Constants And Public-Input Layout

From `tools/descender-account-stark/src/air.rs`:

```text
N_LANES = 6                 (account-projection / masked-projection lanes)
N_COMMIT_LANES = 10         (Poseidon2 commitment digest width)
SYNDROME_LANES = 12         (checked-core Fq12 residues)
ROLE_ROOT_ELEMS = 8 ; RELATION_ID_ELEMS = 4
MODQ_BITS = 14 ; Q12289_CARRIER = Q12289_PACK = 12289
PACKED_WITNESS_COMMITMENT_BYTES = 32 ; PACKED_STREAM_BITS = 18*14 = 252
CONTEXT_ELEMS = ACCOUNT_ID(32) + SLOT_ID(8) + VERIFIER_VERSION(4)
              + STATE_ROOT(32) + LIFECYCLE(32) = 108
```

Single-mode public vector `pv` (the verifier-bound statement; `air.rs` indices):

| pv slice | binds | constraint (air.rs) |
|---|---|---|
| `pv[0..10)` | `pad_source_commitment` (C1 squeeze) | `:724` |
| `pv[10..20)` | `opening_tuple_commitment` (C6 squeeze) | `:754` |
| `pv[20..26)` | `masked_projection[6]` | `:904` |
| `pv[26..134)` | context block (account_id‖slot‖ver‖state‖lifecycle) | `:788`,`:841-854` |
| `pv[134..142)` | `role_root` | `:950` |
| `pv[142..146)` | `relation_id` (pinned const) | `:971` |
| `pv[146..158)` | checked-core `syndrome[12]` | `:1017` |
| `pv[158..190)` | `packed_witness_commitment[32]` | `:1068` |

In batch mode the same equalities run against per-row `pubio`/`ctx` columns,
and a Poseidon2 digest chain (`eval_digest_chain`, `air.rs:1129`) binds the
per-row public-IO into one chained digest with an `enable` boolean and a
monotone count, closing reorder/fabricate/inflate (audit headline, batch mode).

## Decoding Map

For a trace `T` define the decoded witness from the committed columns:

```text
pad_seed        := T[c1.lanes + 0 .. +32]            (= T[c2.lanes + 0..32] by SEAM, air.rs:766)
account_proj[j] := eval_c4_residue(T, c4_off + j)    canonical < q   (air.rs:761, :1381)
masks[j]        := reduce_gadget(C2 squeeze)          (air.rs:737)
masked_proj[j]  := T[carry_masked_off + j]                            (air.rs:862)
syndrome[i]     := T[syndrome_off + i*CELLS]          canonical < q   (air.rs:1017)
ctx fields      := T[ctx_off + ..]                                    (air.rs:788)
```

A clause of `R_air` "holds for `T`" means it holds for this decoded witness.

## Main Theorem

> **Theorem (R_air arithmetization soundness).** Let `T` be any assignment to
> the trace columns of the `descender-account-stark` AIR (single or batch mode)
> that satisfies every constraint emitted by `Air::eval` (`air.rs:701`), under a
> public vector `pv` bound as above. Then the decoded witness satisfies
> `R_air(pv)`: clauses (L1)–(L8) below all hold. Equivalently, no constraint-
> satisfying trace decodes to a witness violating any clause of `R_air`.

The proof is the conjunction of the per-clause lemmas. Each lemma is local: it
names the constraint(s) and shows constraint-satisfaction forces the clause.
The Poseidon2 permutation is modeled as a fixed function `P`; `eval_sponge`
(`air.rs:1263`) constrains the state round-by-round so its squeeze equals the
real sponge over the bound input lanes (the audit verified the round wiring is
not declared-only). We take `P` deterministic and collision-free as a modeled
primitive — that modeling assumption is the QROM/CR obligation of `DIR-QROM-001`,
not re-proven here.

### L1 — Commitment binding (clauses 2, 5, 6 commitments)

`pad_source_commitment = P_sponge(pad_seed‖…)` is forced equal to `pv[0..10)`
(`air.rs:712-726`); `opening_tuple_commitment = P_sponge(account_proj‖lifecycle
‖account_id‖…)` equal to `pv[10..20)` (`air.rs:739-756`). The sponge input
lanes are the committed witness columns and the squeeze equals `pv`, so the two
public commitments are exactly the Poseidon2 images of the decoded witness.
A trace cannot present a commitment in `pv` that is not the sponge image of its
own committed preimage. ∎

### L2 — Mask = PRF(pad_seed, slot, domain) (clause 4 masks)

`masks` are not free columns: C2 is a Poseidon2 sponge over `pad_seed‖slot_id‖…`
(`air.rs:728-738`), squeezed to 30 lanes and `reduce_gadget`-reduced
(`reduce_gadget.rs`) to six `mask[j] ∈ [0,q)`. The SEAM `pad_seed C1==C2`
(`air.rs:765-772`, 32 byte equalities) forces the C2 PRF and the C1 commitment
to use the *same* pad seed, and context-bind (a) (`air.rs:806-813`) forces the
absorbed `slot_id` to equal `pv` slot. Hence each `mask[j]` is the deterministic
sponge image of `(pad_seed, slot_id, domain)` — there is no freedom to choose
masks independent of the committed pad seed and public slot. ∎

### L3 — Canonical account projection, range-exact (clause 4 residues)

`eval_c4_residue` (`air.rs:1381-1426`) decomposes each lane into base-128 cells,
recomposes `d_lo,d_hi`, and enforces canonicity `< q` via the `is96` inverse
gadget: `is96 = [d_hi==96]` (forced by `(d_hi-96)·inv = 1-is96`, `air.rs:1407`),
then `is96·d_lo = 0` (`air.rs:1420`) rejects the single non-canonical corner
`d_hi=96 ∧ d_lo≠0`. With each cell range-bounded, the lane value is the unique
canonical residue `< q`; a lane `≥ q` has no satisfying decomposition. The
`d_hi=96,d_lo≠0` corner has an explicit regression guard
(`c4_is96_inverse_corner_violates_constraints`, audit MED-3). The recomposed
value is bound into `carry_acc` (`air.rs:762`). ∎

### L4 — Carrier mod-q exactness: masked = (acc+mask) mod q (clauses 4, 6)

For each lane (`air.rs:856-906`): `masked` is range-checked canonical via its
14-bit recomposition plus `masked + slack = q-1, slack∈[0,q)`
(`air.rs:881-889`); `wrap` is boolean (`air.rs:892`); and
`acc + mask = masked + wrap·q` (`air.rs:893-896`). Since `acc,mask ∈ [0,q)`
(L2, L3) their sum is `< 2q`, so `wrap ∈ {0,1}` makes the reduction *exact*:
`masked = (acc+mask) mod q`, uniquely. `masked` is forced equal to `pv[20+j]`
(`air.rs:904`). Therefore the public `masked_projection` is the exact canonical
masked carrier of the committed account projection under the L2 mask —
unmasking is well defined and recovers `account_proj`. ∎

### L5 — Statement consistency seams (clause 5 active-lane statement)

The active-lane statement is recomputed canonically from the bound parts. Two
seams force internal consistency: `account_projection C4==C6`
(`air.rs:773-779`) makes the range-checked projection (L3) equal the projection
committed in the opening tuple (L1, C6); the pad-seed seam (L2) makes the
commitment and the PRF share a seed. Combined with the context binding (L6),
every input to the recomputed statement is pinned to either a committed witness
column or a public-statement field — no spend `message_digest`/`signature_digest`
column exists in the layout, so the in-circuit statement is structurally
spend-independent (the non-circularity requirement of the spec, clause 5). ∎

### L6 — Context / identity binding (clause 1 identity, kills universal token)

Context-bind (a)–(d) (`air.rs:806-854`): the `slot_id` absorbed into C2, the
`lifecycle_anchor` and `account_id` folded into the C6 opening-tuple preimage,
and (single-mode, MED-1 hardening) the `verifier_version` and
`account_state_root` ctx columns are each forced equal to the corresponding
`pv` context field. So one trace is bound to exactly one
`(account_id, slot_id, state_root, verifier_version, lifecycle)` tuple; the
context-free universal-admission token is impossible. In batch mode the same is
achieved by `ctx==pubio` (`air.rs:797-803`) plus the digest chain. ∎

### L7 — Deployed-anchor: role-root export binding (clause 6 / DIR-BIND)

`role_root = P_sponge(pad_source_commitment ‖ opening_tuple_commitment ‖
masked_projection)` with the sponge *input lanes constrained equal to the actual
commitment columns* (`air.rs:918-931`) and the squeeze forced equal to public
`role_root` (`air.rs:940-953`). The root is therefore over exactly the proven
role handles; a proof cannot present a `role_root` for a different handle set.
`relation_id` is pinned in-AIR to the constant `RELATION_ID` (`air.rs:962-977`),
so a proof for this relation/version cannot be replayed as a proof for another
relation. ∎

### L8 — Deployed-anchor: packed-witness binding (clause 4 / MED-2 closure)

CLASS-C (`air.rs:979-1069`): the 12 checked-core `syndrome` residues are public
and range-checked `< q` (`air.rs:1008-1041`); the 6 `masked_projection` residues
reuse the carrier's already-validated 14 bits (`air.rs:1043-1053`); and the
32-byte `packed_witness_commitment` is forced equal to the 14-bit-LE packing of
the 252-bit `syndrome‖masked` stream (`air.rs:1056-1069`), matching the deployed
`pack_reviewed_fq12_packed_witness_state18_from_parts`. Hence pinning those 32
public bytes pins *both* the syndrome and the masked projection to the exact
packed witness the spend's checked-core frame consumes — the creation↔spend tie
(audit MED-2 closure). ∎

### Composition

L1–L8 each hold for any constraint-satisfying `T`. Their conjunction is exactly
`R_air(pv)`. Therefore every constraint-satisfying trace decodes to a witness
satisfying `R_air`, proving the Theorem. ∎

## Adversarial Self-Check (red-team of this theorem)

- **Two-identical-rows / single instance.** Both rows are bound to the *same*
  `pv` commitments, so the relation is pinned to one statement; a differing
  second row needs a Poseidon2 collision (modeled out by `DIR-QROM-001`), not an
  under-constraint. Not a hole.
- **Empty / mismatched public values downstream.** L6 binds context *in-circuit*
  in single mode (MED-1), so the binding does not rest on the verifier supplying
  `pv`; a downstream verifier that mis-supplies `pv` still cannot satisfy the
  constraints with a different context. Batch mode binds via the digest chain.
- **Free masks.** Refuted by L2 (sponge-determined + SEAM + slot bind).
- **Non-canonical lane sneaking `≥ q`.** Refuted by L3 (no satisfying
  decomposition) and L4 (sum `< 2q`, wrap boolean ⇒ exact). The one inverse-
  gadget corner has a regression guard.
- **Attacker-chosen syndrome decoupled from spend.** Refuted by L8: syndrome is
  packed into the public `packed_witness_commitment`, which the deployed binding
  pins to the spend's packed witness (negative-tested, `tests/deployed_binding.rs`).
- **Honest residual — NOT closed here.** Acceptance ⟹ trace exists (FRI AoK
  extractor) and `pv` transcript binding (FS/QROM) are *assumed* as the inputs
  to this theorem; they remain open (#2, #4). `R_wiring` (SHA leaf/auth digests)
  and `R_namespace` (admission uniqueness) are enforced outside the AIR and are
  tracked under `DIR-BIND-001`/`DIR-LIFE-001` (#5, #9). This theorem makes those
  the *only* remaining gaps in the statement-correctness story — it does not
  shrink them.

## What This Closes And Hands Off

- **Closes (R_air):** the positive arithmetization-soundness theorem for the
  in-circuit account-creation relation, clause-by-clause against `air.rs`. The
  "selected statement / AIR" obligation's *arithmetic* sub-claim now has a
  positive theorem, not just a hole-hunt.
- **Hands off:** extractor (#2, `DIR-PROOF-001/002`); transcript QROM (#4,
  `DIR-QROM-001`); SHA leaf/anchor `R_wiring` theorem-level binding (#5,
  `DIR-BIND-001`); admission `R_namespace` (#9, `DIR-LIFE-001`).
- **Production posture unchanged:** the deployment gate stays fail-closed; no
  report flag is flipped by this document. It is internal positive evidence; the
  external-review lane and the extractor remain the gating residuals before any
  `verified` status.
