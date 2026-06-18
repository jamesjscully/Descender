# Path 2 Current Core Negative Theorem

Status: negative theorem for the current executable masked-share core; does not
rule out a redesigned transparent IOP of knowledge.

Purpose: push blocker 1 in the negative direction for the proof object that
exists today. The current Merkle/FRI masked-share core cannot be the production
opening proof because its accepted language is strictly larger than the hidden
opening relation.

## Objects

Let `CoreAccept(view, manifest, proof)` be the current debug verifier:

```text
path2_transparent_descent_verify_zk_opening_debug_incomplete_relation_v1
```

It checks:

- proof-byte envelope and query manifest binding;
- Merkle authentication to the trace and constraint roots in `view`;
- opened masked-share rows satisfy, per lane:
  `account_share + mask_share = masked_projection`;
- the same `account_share` satisfies base-128 reconstruction:
  `account_share = lo_share + 128 * hi_share`;
- the committed residual column is zero at sampled rows;
- the FRI-folded constraint column has final all-zero layer.

It does not check:

- C1: `pad_seed` opens to `leaf.pad_seed_digest`;
- C2: masks are derived from `PRF(pad_seed, slot_id)`;
- C4: account projection and pad seed repack to
  `binding.packed_witness_commitment`;
- C6: role commitments bind the account/pad/projection/opening roles.

The production/public verifier is therefore fail-closed through
`PATH2_TRANSPARENT_DESCENT_FULL_HIDDEN_RELATION_DISABLED_ERROR`.

## Negative Theorem

For any public masked projection vector `m in F_q^6` and arbitrary nonzero
public context digests accepted by `Path2TransparentDescentPublicViewV1`, there
exists a proof accepted by `CoreAccept` that is constructible without:

- `pad_seed`;
- a leaf whose `pad_seed_digest` opens to that seed;
- `binding.packed_witness_commitment`;
- PRF-derived masks;
- role commitment openings.

Construction per row `j` and lane `i`:

```text
choose lo_share[j,i], hi_share[j,i] from a public synthetic seed
account_share[j,i] = lo_share[j,i] + 128 * hi_share[j,i] mod q
mask_share[j,i]    = m[i] - account_share[j,i] mod q
masked_share[j,i]  = m[i]
```

Then every opened row satisfies:

```text
account_share + mask_share = masked_share = m[i]
account_share = lo_share + 128 * hi_share
```

The residual column is identically zero, so the FRI layer check accepts the
zero polynomial. Merkle roots are roots of this synthetic trace, not roots of a
trace extracted from any hidden opening.

Therefore:

```text
CoreAccept(view, manifest, proof)
  does not imply
exists pad_seed, account_projection, masks, role_witnesses.
  C1/C2/C4/C6 hold for (leaf, binding, view.masked_projection)
```

No extractor for the current core can prove the production hidden-opening
statement, because there are accepting transcripts whose construction never
mentions the hidden-opening witness or binding data.

## Formal Lean Anchor

`Descender/Path2NativeBoundary.lean` now includes a theorem-side model of the
current sampled row equations:

```text
CurrentMaskedShareCoreRowAccepts(view, row)
```

and proves:

```text
public_only_current_masked_share_core_row_accepts(view)
```

which constructs the public-only satisfying row, plus:

```text
current_masked_share_core_acceptance_does_not_imply_full_hidden_opening(view)
```

which shows there is a model of the missing full-hidden-opening predicate where
the current row equations accept and the full hidden opening is false. This is a
minimal logical model of the current sampled row relation, not a byte-level
Merkle/FRI formalization.

Lean also proves the combined current-route verdict:

```text
current_transparent_route_blocker1_negative_witness
```

which packages this row-core negative with the digest co-location negative from
`research/path2_current_view_binding_negative_theorem.md`.

## Executable Evidence

The Rust regression
`redteam_transparent_zk_core_language_exceeds_hidden_opening_language` builds an
accepted core proof from:

- arbitrary public masked lanes `[1,2,3,4,5,6]`;
- arbitrary leaf/binding/matrix/aggregation digests;
- synthetic Merkle roots generated from the public-only trace above.

The debug core accepts. The public verifier still returns the full-hidden-
relation disabled sentinel, proving the boundary remains fail-closed.

The older regression
`redteam_transparent_zk_core_accepts_public_only_synthetic_trace` shows the same
separation against an honest template view: the synthetic roots differ from the
honest prover roots while the core accepts.

The Rust proof-family metadata now binds this status through:

```text
Path2TransparentCurrentCoreNegativeTargetV1
```

That target hashes the combined route theorem name, the two component Lean
negative theorem names, and the two Rust regression names. It records
`sampled_masked_share_core_only = true`, records
`full_hidden_relation_backend_required = true`, and rejects
`current_core_production_soundness_claim_allowed = true`. Its digest is carried
by the full-hidden-relation proof contract, production obligations, and
proof-family capability.

The redesign target is now explicit too:

```text
Path2TransparentTraceToBindingRelationLayerV1
```

This layer names the exact missing edge. The public view already carries
commitment roots, masked lanes, and the creation-binding digest, but the current
core does not prove that the committed trace opens the same hidden witness that
the creation binding commits to. The layer requires that bridge, including C1,
C2, C4, C6, trace-root role-column binding, the public masked-projection seam,
and ZK witness hiding. It is carried by the proof contract, public-input frame,
production obligations, and capability, with `implemented = false` until the
full-relation backend supplies the proof.

## Consequence For Blocker 1

This is a negative proof for the current executable core, not for all
transparent approaches.

The transparent route remains viable only if the proof object changes from this
sampled masked-share core into a real argument of knowledge for the full hidden
relation, for example by adding a full-relation subproof backend that proves
C1/C2/C4/C6 against the canonical
`Path2TransparentFullHiddenRelationPublicInputsV1` statement, or by replacing
the proof family with another full-witness ZK route.

The positive interface-level target for that replacement is recorded in
`research/path2_full_relation_backend_sufficiency_theorem.md`.

The next positive theorem must prove:

```text
AcceptFull(public_inputs, proof)
  =>
exists hidden opening.
  C1/C2/C4/C5/C6/C7 hold
```

or export a named binding/hash/IOP soundness break. Sampled masked-share
acceptance cannot be reused as that theorem.

## Non-Claims

- This does not break hiding; the issue is soundness/relation knowledge.
- This does not rule out a STARK/IOP of knowledge with a different relation.
- This does not rule out MPC-in-the-head or another full-witness ZK fallback.
- This does not authorize moving to hidden range extraction; blocker 1 remains
  first because the accepted core language is currently too broad.
