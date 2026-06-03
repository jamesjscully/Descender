# Path 2 Native Red-Team: Authorization-Class Confusion V1

Status: closed for the current active-native public boundary.

Question:

```text
Can authorization-local material select a spend meaning not fixed by p_spend(q)?
```

## First Finding

The concrete spend projection includes:

```text
allowed_authorization_local_mode
```

but the encoder only validated verifier family and aggregation mode. A public
boundary with an unknown authorization-local mode could therefore assemble as a
valid active native boundary. That would not be a same-`q` private collision,
because changing the mode changes `p_spend(q)`, but it is still an
authorization-class confusion surface: the system would accept a public spend
class whose private authorization semantics are not defined.

## Fix Applied

The implementation now defines:

```text
PATH2_NATIVE_AUTHORIZATION_LOCAL_MODE_DIRECT = 1
```

and `path2_native_spend_projection_v1_bytes` rejects any other mode with:

```text
Path 2 native spend projection authorization-local mode mismatch
```

KAT `path2_native_boundary_assembly_rejects_independent_digest_inputs` mutates
the mode and requires assembly rejection.

## Second Finding

This closes the unsupported-mode confusion surface. A second blue-team push
closed the arbitrary-class-digest surface: `spend_authorization_class_digest`
is now derived from the complete public spend class rather than accepted as an
independent label.

The implementation now defines:

```text
PATH2_NATIVE_AUTHORIZATION_CLASS_DOMAIN
path2_native_spend_authorization_class_digest_v1(...)
```

over:

```text
spend_verifier_family_id
checked_core_verifier_handle_digest
public_key_identity
allowed_authorization_local_mode
allowed_state_transition_class_digest
aggregation_mode
spend_domain_separator_digest
```

and `path2_native_spend_projection_v1_bytes` rejects unless:

```text
spend.spend_authorization_class_digest
  =
path2_native_spend_authorization_class_digest_v1(spend semantic fields)
```

KAT `path2_native_boundary_assembly_rejects_independent_digest_inputs` now
mutates both the digest and one semantic input to the digest and requires
rejection.

## Third Finding

After checked-core equation verification was wired in, the remaining
authorization-class confusion surface became sharper: checked-core acceptance
used `checked_core_verifier_handle_digest`, while account ownership was named by
an independent `public_key_identity`. That allowed a mix-and-match public
boundary shape where the proof equation was verified against one checked-core
handle while account/spend identity named a separate digest.

## Final Fix Applied

The implementation now defines:

```text
PATH2_NATIVE_PUBLIC_KEY_IDENTITY_DOMAIN
path2_native_public_key_identity_digest_v1(verifier_family, checked_core_handle)
```

and assembly rejects unless:

```text
account.public_key_identity
  =
spend.public_key_identity
  =
path2_native_public_key_identity_digest_v1(
  spend.spend_verifier_family_id,
  spend.checked_core_verifier_handle_digest
)
```

This makes the checked-core verifier handle the source of the public key
identity used by account/spend authorization. The authorization-class digest
then derives over that identity, so the spend class, account identity, and
checked-core proof equation all factor through the same verifier handle.

This closes the current executable boundary version of:

```text
auth_compatible_c(q,auth) => AuthClass_c(auth)=p_spend(q)
```

The future signer/keygen implementation still needs ordinary productization and
constant-time review, but no remaining consensus-visible authorization
coordinate is independent of `p_spend(q)` in the active native public boundary.

## Verification

Commands:

```text
cargo test --manifest-path tools/parameter-search/Cargo.toml path2_native_boundary_assembly --lib
cargo test --manifest-path tools/parameter-search/Cargo.toml path2_native --lib
cargo test --manifest-path tools/parameter-search/Cargo.toml domain --lib
cargo test --manifest-path tools/production-reference/Cargo.toml --lib
```

All pass after the fixes.
