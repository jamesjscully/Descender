# Path 2 Core Algorithm Red Team V1

Status: active adversarial review, focused on core design assumptions rather
than deployment hardening.

Timestamp: 2026-05-30T19:15:22Z.

Scope: attack the active native Path 2 algorithm at the theorem and public
boundary level. This pass intentionally excludes already-known productization
blockers such as constant-time implementation, target-chain storage hardening,
and signer/verifier pipeline completion.

## Current Highest-Risk Findings

### RT-CORE-001: private-fiber meaning consistency was tautological

Severity: high theorem-interface risk.

Status: fixed at the Lean interface level.

Anchor:
`Descender/Path2NativeAccountDesign.lean`, `FiberMeaningConsistent`.

Original vulnerable definition:

```text
FiberMeaningConsistent M q h := M.meaningOfBoundary q = M.meaningOfBoundary q
```

The private fiber argument is ignored. As written, `FiberPrivacy` can prove
that gauge/randomizer choices preserve account meaning without relating any
private coordinate, authorization local, lift core, or randomizer to that
meaning. This is not merely incomplete; it can mask exactly the class of bug
where a private representative over the same public boundary changes spend,
lifecycle, transcript, or state semantics.

Attack shape:

```text
choose h_bad whose private authorization or lifecycle semantics differ
FiberMeaningConsistent M q h_bad still holds by reflexivity
FiberPrivacy.gauge_preserves_meaning/randomizer_preserves_meaning can be
instantiated without detecting the semantic change
```

Required repair:

Introduce an explicit private semantic projection, for example:

```text
meaningOfFiber : Boundary -> PrivateFiber -> Meaning
FiberMeaningConsistent M q h :=
  meaningOfFiber q h = M.meaningOfBoundary q
```

Then require local validity, authorization compatibility, gauge validity, and
randomizer validity to imply this non-tautological equality.

Repair applied:
`PrivateFiberPredicates` now includes:

```text
meaningOfFiber : Boundary -> PrivateFiber -> Meaning
```

and `FiberMeaningConsistent` is now:

```text
P.meaningOfFiber q h = M.meaningOfBoundary q
```

`FiberPrivacy.gauge_preserves_meaning` and
`FiberPrivacy.randomizer_preserves_meaning` now prove the non-tautological
predicate. Verified with:

```text
lake env lean Descender/Path2NativeAccountDesign.lean
```

### RT-CORE-002: final public-admission spend theorem did not require projection completeness

Severity: high theorem-composition risk.

Status: fixed at the Lean interface level.

Anchor:
`Descender/Path2NativeAccountDesign.lean`,
`public_admission_spend_action_or_break`.

The theorem composes `PublicAdmissionSoundness` with `SpendClosure`, but it
does not take or use `ProjectionCompleteness`. As a result, the theorem can
return an action satisfying:

```text
action.meaning = meaningOfBoundary(q)
action.authorization = pSpend(q)
action.lifecycle = pLifecycle(q)
action.stateTransition = pState(q)
```

without proving that:

```text
(meaningOfBoundary(q)).spendAuthorizationClass = pSpend(q)
(meaningOfBoundary(q)).lifecycleIdentity = pLifecycle(q)
(meaningOfBoundary(q)).stateTransitionBoundary = pState(q)
```

This leaves a gap between "action has a meaning field" and "the action fields
are the components of that meaning." A malicious or accidentally inconsistent
`MeaningProjection`/`PublicProjections` instantiation could satisfy the wrapper
while the spend authorization class is not actually the meaning's spend class.

Attack shape:

```text
instantiate M.meaningOfBoundary(q).spendAuthorizationClass = A
instantiate Proj.pSpend(q) = B, with A != B
assume SpendClosure over Proj.pSpend
public_admission_spend_action_or_break still proves an action
whose meaning and authorization disagree
```

Required repair:

Make the final theorem consume `ProjectionCompleteness M Proj`, or define
`SpendActionFromBoundary` so each action component is explicitly read from the
corresponding component of `M.meaningOfBoundary(q)` and then prove equality to
`Proj.*` via projection completeness.

Repair applied:
Lean now defines:

```text
SpendActionMatchesMeaning(action)
```

requiring:

```text
action.authorization = action.meaning.spendAuthorizationClass
action.lifecycle = action.meaning.lifecycleIdentity
action.stateTransition = action.meaning.stateTransitionBoundary
```

and proves:

```text
spend_action_matches_meaning_of_projection_complete
```

from `ProjectionCompleteness M Proj` and `SpendActionFromBoundary`.
`public_admission_spend_action_or_break` now takes
`ProjectionCompleteness M Proj` and returns both the boundary action and
`SpendActionMatchesMeaning`. Verified with:

```text
lake env lean Descender/Path2NativeAccountDesign.lean
```

### RT-CORE-003: spend closure was assumed, not derived from checked-core soundness

Severity: high cryptographic-reduction risk.

Status: fixed at the Lean interface level.

Anchor:
`Descender/Path2NativeAccountDesign.lean`, `SpendClosure`,
`CheckedCoreFrameBinding`, `NativeCheckedCoreFrameBinding`.

The active final theorem uses:

```text
V.spendAccept q sig msg state -> exists action, SpendActionFromBoundary ...
```

as an assumption. The checked-core frame binding objects exist, but the final
wrapper does not derive `SpendClosure` from:

- native checked-core frame construction;
- checked-core equation/sigma soundness;
- public-key identity binding;
- authorization class binding;
- state/lifecycle closure;
- replay/consume-once semantics.

This makes the final theorem close by assuming the semantic property it should
reduce to concrete cryptographic checks.

Attack shape:

```text
define V.spendAccept to accept a signature under an independent verifier handle
postulate SpendClosure anyway
final theorem cannot expose the missing checked-core reduction because
SpendClosure already asserts boundary-determined semantics
```

Required repair:

Add a derived theorem target:

```text
checked_core_frame_binding
∧ checked_core_equation_soundness
∧ public_key_identity_binding
∧ authorization_class_binding
∧ state_lifecycle_closure
=> SpendClosure
```

The active final theorem should depend on that derived package, not an
uninterpreted `SpendClosure` assumption.

Repair applied:
Lean now defines:

```text
CheckedCoreSpendEvidence
CheckedCoreSpendReduction
```

where `CheckedCoreSpendReduction` requires every accepted spend to export
checked-core evidence and prove:

```text
checkedCoreAccepts(q,msg,state,evidence)
frame_bound_to_projections
authorization_class_bound
lifecycle_bound
state_transition_bound
replay_scope_bound
```

The theorem:

```text
checked_core_reduction_implies_spend_closure
```

derives `SpendClosure` from that reduction package. The active certificate
fields and `public_admission_spend_action_or_break` now consume
`CheckedCoreSpendReduction` instead of an uninterpreted `SpendClosure`.
Verified with:

```text
lake env lean Descender/Path2NativeAccountDesign.lean
```

### RT-CORE-004: Lean frame/digest models used non-injective `Nat` addition

Severity: medium-high formal-model risk.

Status: fixed at the Lean interface level for frame/digest surfaces.

Anchors:
`CheckedCoreFrameBinding.frame_binds`,
`NativeCheckedCoreFrameBinding.frame_binds`,
`assembledNativeBoundaryDigest`,
`assembledTranscriptDigest`.

Several formal frame/digest equalities model canonical byte encodings by adding
field `Nat`s. Addition is commutative and highly non-injective, while the Rust
implementation relies on ordered, domain-separated, length-delimited bytes.
This is acceptable only as a placeholder equality model; it is unsafe as a
binding model.

Attack shape:

```text
field layout A + B + C = A + C + B
or A + B = A' + B'
Lean frame equality still holds
byte-level frame binding would not
```

Required repair:

Separate the formal model into:

- ordered symbolic frames, not raw `Nat` sums;
- an injectivity or collision-break assumption for each frame encoder;
- theorem wrappers that consume frame injectivity rather than arithmetic
  equality.

Repair applied:
Lean now defines:

```text
OrderedFrameEncoder
ordered_frame_encode_injective
```

and the checked-core frame, native checked-core frame, native boundary digest
assembly, transcript digest assembly, and public-view assembly surfaces all bind
ordered field lists through `OrderedFrameEncoder.encode` rather than `Nat`
addition. The encoder carries an injectivity obligation:

```text
encode lhs = encode rhs -> lhs = rhs
```

This models byte-frame binding as ordered-frame injectivity/collision
resistance instead of commutative arithmetic. Verified with:

```text
lake env lean Descender/Path2NativeAccountDesign.lean
```

### RT-CORE-005: public obstruction admission could become a self-attested flag

Severity: medium-high design risk, depending on final account-creation
verifier.

Status: fixed at the Lean interface level.

Anchors:
`path2_native_no_obstruction_flags_digest`,
`assemble_path2_native_boundary_v1`,
`PublicAdmissionSoundness`.

The Rust public-boundary assembly rejects unless:

```text
obstruction_flags_digest == H(PATH2_NATIVE_NO_OBSTRUCTION_FLAGS_DOMAIN)
```

and checks several concrete pullback/gluing edges. That is good as an
implementation guard, but it is not by itself a computation of every component
of:

```text
obs_c(q)=0
```

listed in the theorem interface:

- context defect;
- pullback defect;
- gluing defect;
- meaning defect;
- spend-state defect;
- canonicality defect.

If a future verifier treats the no-obstruction digest as the obstruction
computation rather than as the result of a checked computation, admission can
degrade into a self-attested "no obstruction" flag.

Attack shape:

```text
construct q with an unmodeled meaning/canonicality defect
set obstruction_flags_digest to the canonical no-obstruction digest
pass the current digest equality and any missing edge checks
```

Required repair:

Maintain an explicit table from each obstruction component to executable checks
or named assumptions. The no-obstruction digest should be the canonical encoding
of checked results, not a standalone substitute for those checks.

Repair applied:
Lean now defines:

```text
ObstructionComponentPredicates
CheckedObstructionZero
```

with explicit component obligations:

```text
contextChecked
pullbackChecked
gluingChecked
meaningChecked
spendStateChecked
canonicalityChecked
```

`PublicAdmissionSoundness` now returns:

```text
CheckedObstructionZero O CObs q ∧ publicBoundarySound q
```

instead of bare `ObstructionZero O q`. The helper theorems:

```text
obstruction_zero_of_checked
checked_obstruction_components
```

let downstream proofs recover ordinary zero obstruction and the full component
check table. This prevents the no-obstruction digest from being modeled as a
self-attested all-clear flag in the proof interface. Verified with:

```text
lake env lean Descender/Path2NativeAccountDesign.lean
```

## Working Attack Order

1. Fix or falsify `FiberMeaningConsistent`; this is the most direct theorem
   hole.
2. Strengthen the final spend theorem to require projection completeness.
3. Replace assumed `SpendClosure` with a derived checked-core/authorization
   composition theorem.
4. Replace additive Lean frame models with ordered symbolic frames plus
   injectivity assumptions.
5. Audit every `obs_c(q)=0` component against executable Rust checks and named
   cryptographic assumptions.

## Patch Closure Summary

The first five serious proof-interface concerns are patched at the Lean
interface level.

| concern | patched invariant | Lean anchor |
|---|---|---|
| RT-CORE-001 | private fiber meaning is non-tautological: `P.meaningOfFiber q h = M.meaningOfBoundary q` | `FiberMeaningConsistent` |
| RT-CORE-002 | public-admission spend result includes `SpendActionMatchesMeaning`, derived from `ProjectionCompleteness` | `spend_action_matches_meaning_of_projection_complete`, `public_admission_spend_action_or_break` |
| RT-CORE-003 | final spend theorem consumes `CheckedCoreSpendReduction`; `SpendClosure` is derived, not assumed directly | `CheckedCoreSpendReduction`, `checked_core_reduction_implies_spend_closure` |
| RT-CORE-004 | frame/digest models use ordered symbolic encoders with injectivity, not `Nat` addition | `OrderedFrameEncoder`, `ordered_frame_encode_injective` |
| RT-CORE-005 | public admission returns `CheckedObstructionZero` with six named component checks, not a bare no-obstruction flag | `ObstructionComponentPredicates`, `CheckedObstructionZero`, `checked_obstruction_components` |

Verification:

```text
lake env lean Descender/Path2NativeAccountDesign.lean
lake build Descender.Path2NativeAccountDesign
lake build Descender
lake env lean Descender.lean
```

Remaining work is no longer to patch these five interfaces, but to instantiate
their predicates with concrete Rust/byte-level checks and named cryptographic
assumptions.

## Assumption Attack Matrix

| assumption surface | attack question | current red-team status |
|---|---|---|
| private fiber meaning | Can a private representative alter semantic spend/lifecycle/state while the proof still says meaning is preserved? | fixed at interface level by adding `meaningOfFiber`; now needs concrete instantiation review |
| projection completeness | Does the final spend theorem force every action component to be the corresponding component of `meaning_of_boundary(q)`? | fixed at interface level: final wrapper consumes `ProjectionCompleteness` and returns `SpendActionMatchesMeaning` |
| spend closure | Is spend semantic closure derived from checked-core verification, or assumed? | fixed at interface level: final wrapper consumes `CheckedCoreSpendReduction`, which derives `SpendClosure` |
| checked-core frame binding | Is the formal model injective/order-sensitive like the byte frame? | fixed at interface level with `OrderedFrameEncoder` and ordered field-list equalities |
| obstruction zero | Is `obs_c(q)=0` computed from all defect components, or can it be represented by a self-attested digest? | fixed at interface level: public admission returns `CheckedObstructionZero` with all six named component checks |
| public-key identity binding | Can account identity and checked-core verifier handle be substituted independently? | current Rust assembly rejects this via `path2_native_public_key_identity_digest_v1`; no new break found in this pass |
| checked-core proof equation | Does accepted native checked-core proof satisfy the expected response syndrome under the admitted verifier handle? | current Rust `path2_native_checked_core_accept_v1` checks the equation; remaining concern is theorem composition into `SpendClosure` |
| message/state binding | Can a spend proof be replayed across message or current state? | native frame includes `message_digest` and `current_state_digest`; no new byte-level break found yet |
| context/domain binding | Can a boundary move across spend domain, chain/app domain, fork/epoch, verifier family, or aggregation mode? | Rust assembly checks verifier family, spend domain, aggregation mode; broader context defect map still needs component-by-component audit |
| lifecycle/replay meaning | Can live/consume-once semantics differ between lifecycle and state projections? | Rust assembly checks live marker equality and state transition class equality; `CheckedObstructionZero.spendStateChecked` now names the remaining component-level proof hook |
| no-export | Can public views export private fiber material? | serializer denylist KATs exist; semantic privacy now has a non-tautological `meaningOfFiber` hook |
| hash/QROM domain separation | Can frame or digest domains collide/replay? | registry KATs exist; formal Lean frame model still needs injective symbolic frames |
| data availability | Can a carrier be admitted while the projection preimage/proof evidence is unavailable or mismatched? | carrier preflight recomputes against a bundle; consensus admission rules must still guarantee availability of that bundle/evidence |

## Non-Findings So Far

- The latest Rust public-key identity binding closes the earlier independent
  checked-core verifier handle substitution attack at the byte/API level.
- The active native checked-core frame now binds message, state, lifecycle,
  transcript, projection digests, and proof commitment before the checked-core
  challenge.
- The active account-creation theorem intentionally permits adversarial
  unspendable public boundaries. That is not itself a forgery, but it makes
  spend-time derivation and public meaning consistency non-negotiable.
