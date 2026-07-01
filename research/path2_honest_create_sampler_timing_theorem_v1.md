# Path2HonestCreateSamplerTimingTheoremV1

> Classification: `ACTIVE_SUPPORT`. Supports the active Path 2 creation path
> (`ADR-0002`, tracker row `DIR-CREATE-001`, `spec/security_model.md`). It closes,
> at paper level, the honest-create sampler and no-postchallenge timing
> obligations specified in `research/path2_honest_create_sampler_v1.md` and
> `research/path2_no_postchallenge_creation_timing_game.md`. Not normative by
> itself.

Status: honest-create sampler soundness + no-postchallenge timing theorem.

## Obligation

Two coupled obligations under `DIR-CREATE-001`
(`Path2AccountCreationSamplerTimingReportV1` imports the no-postchallenge
timing slice and keeps the honest-sampler flag false):

1. **No-postchallenge timing** — account creation cannot be influenced after the
   spend challenge; otherwise the prevalidated-slot model is circular. Break:
   `postchallenge_slot_creation_or_rewrite_break`.
2. **Honest-create sampler** — honest creation yields a spendable private witness
   realizing the public spend class. Break: `authorization_local_sampling_break`.

## Lemma T — No-postchallenge timing (anti-circularity)

> **Lemma T.** If a Path 2 spend accepts for slot `s` with
> `account_state_root_before = R`, then the `PrevalidatedSlotLeafV1` for `s` was
> admitted under namespace `(account_id, R, slot_authorization_digest)` *before*
> the spend challenge was derived, or the adversary exports one of
> `{checked_core_message_binding_break, direct registry/state-namespace break,
> slot_authorization_digest hash/domain collision, lifecycle rollback/reorg
> break}`.

**Proof.** Two facts compose:

1. **Creation statement is spend-independent (structural).** By the
   arithmetization-soundness theorem (#1, L5) and
   `Path2CreationHiddenSourceStatementV1`, the creation-time statement contains
   NO `message_digest`, `signature_digest`, spend proof commitment, or
   `slot_authorization_digest` — there is no column or field through which a
   spend-time value can enter creation. So step 2 of the timing order
   (`research/path2_no_postchallenge_creation_timing_game.md`) is computable
   before any spend-time value exists.
2. **Admission precedes challenge (ordering enforced).** The deployed acceptance
   boundary (`spec/security_model.md`, check 4) requires the creation verifier
   accepted the evidence *before the spend challenge was derived*, and the
   checked-core challenge frame (`path2_checked_core_authorization_challenge_frame`)
   binds `R` and `slot_authorization_digest`; direct authorization
   (`validate_path2_direct_slot_authorization_state_namespace`) accepts ONLY an
   already-admitted `(account_id, R, slot_authorization_digest)` tuple. Hence a
   spend accepted under `R` forces a leaf admitted under that exact namespace
   before challenge derivation.

A spend that needed a leaf rewritten/admitted *after* seeing the challenge would
therefore need to break the frame binding (fact 2), forge the namespace tuple, or
collide `slot_authorization_digest` — each a named break. The State-Root Timing
Lemma gives the same for a mismatched root. ∎

This is exactly the anti-circularity property: because creation cannot see the
spend challenge (fact 1) and admission is recorded before the challenge (fact 2),
the prevalidated slot is fixed independently of the spend it later authorizes.

## Lemma S — Honest-create sampler soundness

> **Lemma S.** `HonestCreate_c(seed, context, account_params) = Some(q,h)` ⟹
> `obs_c(q)=0 ∧ LocalValid_c(q,h) ∧ AuthClass_c(h.authorizationLocal) = p_spend(q)`.

**Proof (by the private-to-public construction order).** The sampler builds
`h = (liftCore, authorizationLocal, gauge, randomizer)` and `q` in the order
`seed → authorizationLocal → AuthClass → (lifecycle/account/state) → q →
liftCore → gauge → randomizer`:

- **`obs_c(q)=0` by construction:** every face of `q` is a restriction of one
  semantic source `M_c = AccountNamespace × SpendAuthorizationClass ×
  LifecycleIdentity × TranscriptBindingSurface × StateTransitionBoundary ×
  ParameterContext`, so the obstruction (face-consistency) check vanishes by
  construction — the faces are projections of a single coherent object.
- **`AuthClass = p_spend(q)` by construction:** the sampler derives
  `AuthClass_c(authorizationLocal)` and then *sets* `p_spend(q) = AuthClass`. This
  is the typed `AuthorizationCompatibleOverClass` predicate, so the authorization-
  class binding theorem (#6, Lemma A) holds by construction — `q` exposes exactly
  the auth's public class rather than being guessed independently.
- **`lift/gauge/randomizer_valid`:** `liftCore = canonical_lift_core_c(q)` (valid
  by public-boundary soundness + canonicality); `gauge = identity_gauge_c(q)` (so
  `boundary(gauge.h)=q`, `meaning(gauge.h)=meaning_of_boundary(q)`); `randomizer`
  validity by sampler well-formedness. Conjunction = `LocalValid_c(q,h)`. ∎

## Theorem (composed)

> Combining Lemma T and Lemma S: honest creation produces a slot whose public
> boundary is obstruction-free, whose retained private witness `h` realizes
> exactly `p_spend(q)`, and which is fixed before — and independent of — any spend
> challenge. Therefore the prevalidated-slot model is sound and non-circular: a
> later accepted spend consumes a prechallenge-admitted authorization once
> (couples to #9 consume-once), or exports a named break.

This connects to the direct theorem's "admitted before the spend challenge"
hypothesis (#1 chain) and the authorization-class binding (#6): #6 gives "private
auth can't change the public class"; #7 gives "honest auth *realizes* the class,
and creation can't be steered by the spend."

## Adversarial Self-Check

- **Grind creation against a favorable challenge.** Impossible by Lemma T fact 1:
  the creation statement has no spend-time field, so there is nothing to grind
  against; and fact 2 records admission before the challenge.
- **Rewrite the leaf post-challenge.** Forces a frame-binding / namespace / hash
  break (Lemma T).
- **Sample `q` first, hope auth exists.** Rejected design (dead condition): the
  sampler MUST derive `p_spend(q)` from `authorizationLocal` (Lemma S). An
  adversary submitting a coherent `q` with no compatible auth gets an *unspendable*
  account — not a forgery (a state-policy/pricing concern, not a proof failure).
- **Honest residual.** (1) The lifecycle rollback/reorg term in Lemma T is charged
  to the lifecycle model (#9, `DIR-LIFE-001`) — named, not proven here. (2) The
  selected-Rust Lean support hooks for `HonestCreate` returning
  `AuthorizationCompatibleOverClass` and the five-row sampler integration
  checklist are now digest-pinned by the sampler/timing report; the accepted
  boundary row names
  `selectedRustAcceptedBoundaryHonestSamplerSound_of_boundaryAccepted_honestCreate`,
  which machine-checks the accepted-boundary import, authorization-class
  obligation import, no-postchallenge timing import, selected-Rust honest-create
  authorization bridge, and the binding of `q,h` to the accepted spend's own
  creation boundary and concrete leaf authorization fiber as a support package.
  The full honest-create sampler evidence path is still not closed. (3) Signer state-loss is
  key-management engineering, not a consensus proof. (4) External review. Only
  the no-postchallenge timing slice is imported as closed by the sampler/timing
  report; the honest-sampler theorem remains open.

## What This Closes And Hands Off

- **Closes:** no-postchallenge timing (anti-circularity) grounded in L5
  spend-independence + the admission-before-challenge ordering hooks, now
  machine-checked by `noPostchallengeTiming_createdBeforeChallenge` and
  `noPostchallengeTiming_lateAdmission_excluded`.
- **Paper-level support, still open as a report flag:** honest-create sampler
  soundness by the private-to-public construction order, reusing #6's typed
  compatibility. The selected-Rust support hooks are pinned by
  `PATH2_ACCOUNT_CREATION_SAMPLER_AUTHORIZATION_BRIDGE_HOOKS`, and the remaining
  integration path is row-pinned by
  `PATH2_ACCOUNT_CREATION_SAMPLER_INTEGRATION_ROWS`, with the accepted-boundary
  row now requiring
  `selectedRustAcceptedBoundaryHonestSamplerSound_of_boundaryAccepted_honestCreate`.
- **Hands off:** the lifecycle reorg/rollback term (#9); full honest-create
  sampler evidence integration that consumes the checklist; external review.
  `Path2AccountCreationSamplerTimingReportV1` keeps its honest-sampler theorem
  flag false pending that integration and review, while the no-postchallenge
  timing flag is true.
