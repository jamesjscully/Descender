# Path 2 Checked-Core Frame Binding V1

Status: first native-frame binding proof target.

Purpose: prove or specify that downstream checked-core spend verification
consumes only native boundary projections:

```text
checked_core_frame(q,msg,state)
  =
frame(
  p_spend(q),
  p_lifecycle(q),
  p_transcript(q),
  p_state(q),
  msg,
  state,
  p_context(q)
)
```

and no independent semantic handle.

## Why This Matters

Authorization class binding proves:

```text
auth_compatible_c(q,auth) => AuthClass_c(auth)=p_spend(q)
```

But checked-core spend verification can still break the design if its challenge
or verifier relation binds to an older independent handle such as:

```text
slot_authorization_digest
opening_commitment
pad_seed_digest
creation_statement_digest
carrier_transcript_digest
```

whose semantic meaning does not factor through `q`.

The checked-core verifier may keep compact byte handles, but each handle must be
a byte binding for one of the native projections, not an independent source of
spend meaning.

## Accepted Frame Contents

The spend challenge frame may include:

```text
native_boundary_digest(q)
p_context(q)
p_account(q)
p_spend(q)
p_lifecycle(q)
p_transcript(q)
p_state(q)
msg
state
signature/proof commitment
```

It may also include byte encodings:

```text
slot_authorization_digest(q)
checked_core_authorization_handle(q)
account_state_root_before(q)
```

only if their preimages are canonical projections of `q`.

## Disallowed Frame Contents

Disallow as semantic inputs:

```text
independent opening_commitment
independent pad_seed_digest
independent hidden_source_statement
independent carrier relation handle
independent slot authorization digest
old 56-byte carrier transcript
```

These may appear only as:

- private signer/proof data;
- typed restrictions under one native boundary;
- byte/data-availability bindings whose semantic preimage is a projection of
  `q`;
- fallback proof metadata outside the active compact native path.

## Frame Binding Theorem

Target:

```text
checked_core_frame_binding_v1:
  obs_c(q)=0
    =>
  checked_core_frame(q,msg,state,commitment)
    =
  CanonicalEncode(
    domain = p_context(q).spend_verifier_domain,
    verifier_family = p_spend(q).spend_verifier_family,
    authorization_class = p_spend(q).spend_authorization_class,
    authorization_identity = p_spend(q).public_authorization_identity,
    checked_core_handle = p_spend(q).checked_core_or_native_verifier_handle,
    allowed_state_transition_class =
      p_spend(q).allowed_state_transition_class,
    aggregation_mode = p_spend(q).aggregation_mode,
    lifecycle = p_lifecycle(q),
    transcript = p_transcript(q),
    state_boundary = p_state(q),
    message = msg,
    current_state = state,
    proof_commitment = commitment
  )
```

or a named break:

```text
checked_core_frame_binding_break
domain_binding_or_qrom_break
byte_data_availability_break
```

## Relation To Existing 308b Checked-Core

The older 252-byte checked-core signature remains useful as a downstream
spend-time proof. The active interpretation is:

```text
checked_core_authorization = p_spend(q).checked_core_or_native_verifier_handle
```

and:

```text
slot_authorization_digest(q)
```

is allowed only as a digest/encoding of:

```text
p_spend(q), p_lifecycle(q), p_state(q), p_context(q)
```

or the full:

```text
p_transcript(q)
```

The old 56-byte carrier remains out of the active native account-creation path.

## Timing Requirement

The frame must bind prechallenge admitted state:

```text
account_state_root_before = p_state(q).account_state_root_before
slot/lifecycle identity = p_lifecycle(q)
authorization digest = digest(p_spend(q), ...)
```

before the spend challenge is derived.

This preserves the no-postchallenge creation rule:

```text
admitted q first
spend message/challenge second
checked-core response third
consume lifecycle state once
```

If a prover can choose or rewrite the account boundary, slot authorization, or
state root after seeing the challenge, export:

```text
postchallenge_slot_creation_or_rewrite_break
```

or the closest existing break:

```text
checked_core_frame_binding_break
```

## Edge Dependencies

This theorem depends on:

```text
E_AS: account-spend identity binding
E_ST: spend-transcript binding
E_LT: lifecycle-transcript binding
E_LD: lifecycle-state binding
E_AD: account-state binding
E_SD: spend-state binding
```

Especially:

```text
p_spend(q).allowed_state_transition_class
  =
p_state(q).state_transition_class
```

and:

```text
p_spend(q).aggregation_mode
  =
p_transcript(q).aggregation_mode
```

## Current Assessment

The checked-core route is viable if we treat it as a verifier over a frame
constructed from `q`, not as an independent account object.

The most likely failure mode is not algebraic checked-core soundness; it is
frame drift:

```text
checked-core challenge binds old slot_authorization_digest
but native q proves a different p_spend/p_state/p_lifecycle tuple.
```

This is exactly what `E_ST`, `E_LT`, `E_SD`, and the canonical frame theorem
must rule out.

## Required Interface Change

The Lean interface already has:

The Lean interface now models this as an ordered symbolic frame rather than
as `Nat` addition:

```text
frame : Boundary -> Nat -> Nat -> Nat -> Nat
```

where the final `Nat` is the checked-core commitment or signature commitment
bound before challenge derivation, and:

```text
frame q msg state commitment =
  OrderedFrameEncoder.encode [
    Proj.pSpend q,
    Proj.pLifecycle q,
    Proj.pTranscript q,
    Proj.pState q,
    msg,
    state,
    commitment
  ]
```

`OrderedFrameEncoder` carries the binding/injectivity obligation:

```text
encode lhs = encode rhs -> lhs = rhs
```

so formal frame equality is order-sensitive, matching the Rust byte layout
model rather than commutative arithmetic.

## Pass Criteria

Pass if:

- checked-core challenge bytes are a deterministic encoding of native
  projections plus message/state/proof commitment;
- slot authorization digest is a byte binding for native projections;
- checked-core verifier handle is part of `p_spend(q)`;
- state root and lifecycle/nullifier scope are projected from `q`;
- no opening/pad/carrier handle affects acceptance except as private proof data
  or byte binding for `q`.

## Fail Criteria

Fail or redesign if:

- checked-core accepts under a slot authorization digest not determined by `q`;
- checked-core challenge omits state root, lifecycle, or spend authorization
  class;
- checked-core verifier family is selected outside `p_spend(q)`/`p_context(q)`;
- proof commitment can be chosen after challenge;
- old carrier transcript bytes carry semantic authority in the active native
  path.

## Verdict

Checked-core frame binding is a tractable proof obligation. It does not require
reviving the full hidden-witness opening proof.

The required move is to define the checked-core frame as a canonical function of
native projections, then treat any accepting spend under a different frame as a
named checked-core/domain binding break.

## Next Concrete Move

Keep the Lean `CheckedCoreFrameBinding` ordered-frame interface synchronized
with the Rust `Path2NativeCheckedCoreFrameV1` field order.

Then audit the Rust/reference checked-core frame function against the same field
list when implementation work resumes.
