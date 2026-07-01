# Path2AuthorizationClassBindingTheoremV1

> Classification: `ACTIVE_SUPPORT`. Supports the active Path 2 spend path
> (`ADR-0002`, tracker row `DIR-AUTHCLASS-001`, `spec/security_model.md`). It
> closes, at paper level, the authorization-class binding obligation defined in
> `research/path2_authorization_class_binding_proof_v1.md`. Not normative by
> itself.

Status: authorization-class binding theorem (private auth cannot change public
spend semantics).

## Obligation

Prove that signer-held private authorization-local material cannot change public
spend semantics or bypass `p_spend(boundary)`:

```text
auth_compatible_c(q, auth)  ⟹  AuthClass_c(auth) = p_spend(q)
```

and the dangerous failure is impossible:

```text
¬∃ q auth auth'. auth_compatible_c(q,auth) ∧ auth_compatible_c(q,auth')
                 ∧ AuthClass_c(auth) ≠ AuthClass_c(auth').
```

`DIR-AUTHCLASS-001` keeps `authorization_class_binding_theorem_instantiated`
false. Named break: `authorization_class_binding_break`.

## Two-part structure

The obligation factors into a definitional part and a field-completeness part.

### Part A — Local binding is definitional under typed compatibility

Adopt the typed compatibility predicate (the `AuthorizationCompatibleOverClass`
refinement of `research/path2_authorization_class_binding_proof_v1.md` § Required
Lean Refinement):

```text
auth_compatible_c(q, auth) :=
    AuthClass_c(auth) = p_spend(q)
  ∧ auth_local_wellformed_c(q, auth)
  ∧ auth_private_bounds_c(auth)
```

i.e. **compatibility is relative to a public class, not to "can sign somehow."**
Then the local binding is immediate:

> **Lemma A (local binding).** `auth_compatible_c(q,auth) ⟹ AuthClass_c(auth) =
> p_spend(q)`, by the first conjunct of the definition. The no-ambiguity property
> follows: if `auth, auth'` are both compatible with the same `q`, then
> `AuthClass_c(auth) = p_spend(q) = AuthClass_c(auth')`, so they realize the SAME
> class — the dangerous failure is excluded. ∎

This is not circular: it is the intended *boundary of what "compatible" means*.
The honest constructor returns this stronger predicate (the proof chain
`HonestCreateSoundness ⟹ LocalValid ⟹ authorizationCompatible ⟹
AuthorizationClassMatches`), which couples to the honest-create sampler (#7).

### Part B — Field-completeness (the load-bearing part)

Lemma A is only *meaningful* if `p_spend(q)` contains every spend semantic the
deployed verifier can observe — otherwise an out-of-class field could vary spend
behavior while `AuthClass_c` stays fixed. So the real theorem is:

> **Lemma B (field-completeness).** Every consensus-visible field that the
> deployed checked-core spend verifier reads when deciding acceptance is a
> projection of `q` contained in `p_spend(q)` (or in `p_lifecycle/p_state(q)` with
> an `O_gluing` edge).

**Discharge, grounded in the deployed acceptance boundary.** The direct-theorem
acceptance hypotheses (`spec/security_model.md` § Direct Theorem Acceptance
Boundary, checks 6–7) state the checked-core challenge frame binds exactly:

```text
{ public_key_identity, message_digest, account_id, account_state_root,
  slot_id, proof_commitment, slot_authorization_digest }
```

and acceptance is checked-core verification under the active Poseidon2 challenge
path plus the separated Fq12 action-map term. Map these to the `p_spend(q)`
fields (`research/path2_authorization_class_binding_proof_v1.md` § Public Spend
Projection):

| `p_spend(q)` field | bound by | acceptance hypothesis |
|---|---|---|
| `verifier_family` | `verifier_version` / pinned `relation_id` (#1 L6/L7) | versioned frame + in-AIR relation pin |
| `public_authorization_identity` | `public_key_identity` | check 6 (pk in frame) |
| `spend_domain` | domain separators in the frame / `slot_authorization_digest` | checks 3,6 |
| `checked_core_handle` | `proof_commitment` | check 6 |
| `allowed_state_transition_class` | `account_state_root` (before) + consume-once successor | checks 6,8 |
| `aggregation_mode` | direct mode = `none` (Mode A consumes no aggregation) | `spec/README.md` Mode A |

Every field the frame binds is in `p_spend(q)`; conversely every `p_spend(q)`
field is bound by the frame. The residual is the **negative** direction — that no
acceptance branch reads a field *outside* this set — which is a bounded audit of
the deployed checked-core verifier (below), not an open algebraic gap. ∎

## Theorem

> **Theorem (authorization-class binding + spend closure).** Under typed
> compatibility (Part A) and field-completeness (Part B):
>
> ```text
>   auth_compatible_c(q,auth) ⟹ AuthClass_c(auth) = p_spend(q)            (local)
>   obs_c(q)=0 ∧ auth_compatible_c(q,auth)
>       ⟹ AuthClass_c(auth) = meaning_of_boundary_c(q).spendAuthorizationClass   (boundary)
>   obs_c(q)=0 ∧ auth_compatible_c(q,auth) ∧ SpendProduced(...) ∧ SpendAccept(...)
>       ⟹ accepted spend authorization = p_spend(q)                       (closure)
> ```
>
> So private authorization-local material (representative coords, signing witness,
> opening randomness, gauge, one-time masks, simulator randomness) cannot change
> the public spend class; only `p_spend(q)` determines accepted spend semantics,
> or the adversary exports `authorization_class_binding_break`.

The boundary step uses projection completeness
(`meaning_of_boundary_c(q).spendAuthorizationClass = p_spend(q)`,
`research/path2_projection_completeness_bridge_v1.md`) and the `O_gluing` edges
`E_AS` (`p_account.authorization_identity = p_spend.public_authorization_identity`)
and `E_SD` (`p_spend.allowed_state_transition_class = p_state.state_transition_class`).

## Adversarial Self-Check

- **Two compatible auths, different class.** Excluded by Lemma A under typed
  compatibility — both equal `p_spend(q)`.
- **Out-of-class field varies spend acceptance.** This is the only real risk
  (Part B negative direction); it is a bounded verifier audit, charged as the
  residual, not an algebraic hole. If found, the field MUST move into `p_spend(q)`
  (the doc's "What Must Move Into q" list).
- **"Compatible" weakened to can-sign-somehow.** Then Lemma A is an assumption,
  not definitional — explicitly rejected; the typed predicate is mandatory.
- **Honest residual.** (1) Field-completeness negative audit of the deployed
  checked-core spend verifier (no acceptance branch reads a field outside
  `p_spend(q)`). (2) The Lean `AuthorizationCompatibleOverClass` instantiation +
  honest-create wrapper (couples to #7). (3) The projection-completeness bridge
  `p_spend(q) = meaning_of_boundary`. (4) External review. No flag flipped.

## What This Closes And Hands Off

- **Closes (paper level):** local authorization-class binding (definitional under
  typed compatibility) + the positive field-completeness mapping to the deployed
  acceptance boundary; the no-ambiguity property; the spend-closure consequence.
- **Machine-checked boundary hooks:** `Descender/Path2FullSpendConcrete.lean`
  proves `authorizationClass_obligationDigest_nonzero_of_obligationTracked`,
  `authorizationClass_boundary_eq_leafBoundary_of_obligationTracked`, and
  `authorizationClass_matches_pSpend_of_obligationTracked`, exposing the carried
  proof-boundary facts that the obligation digest is nonzero, creation evidence
  and leaf share the same boundary, and the admitted leaf's public
  authorization class equals `p_spend(boundary)`. These are boundary hooks, not
  theorem closure.
- **Hands off:** the field-completeness *negative* audit of the spend verifier;
  the Lean predicate instantiation (with #7 honest-create); projection-completeness
  bridge; external review. `DIR-AUTHCLASS-001` keeps
  `authorization_class_binding_theorem_instantiated` false pending the Lean
  instantiation + audit.
