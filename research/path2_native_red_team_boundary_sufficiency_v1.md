# Path 2 Native Red-Team: Boundary Sufficiency V1

Status: first internal cryptoanalytic red-team pass.

Question:

```text
Can two private/local representatives over the same public q produce different
spend, lifecycle, replay, state-transition, or authorization behavior?
```

## Result

No concrete private same-`q` representative attack was found in the current
public API surface: signer-local/private material is not an input to the active
native projection encoders, consensus carrier, verifier preflight report, or
signer/verifier public pipeline wrappers.

However, the red-team pass found a related public-boundary sufficiency issue:
the Rust assembly enforced only these gluing edges:

```text
context.verifier_family_id = spend.spend_verifier_family_id
context.aggregation_mode   = spend.aggregation_mode
account.public_key_identity = spend.public_key_identity
```

It did not enforce three documented semantic edges:

```text
context.spend_verifier_domain_digest
  = spend.spend_domain_separator_digest

spend.allowed_state_transition_class_digest
  = state.consumed_successor_state_class

lifecycle.live_status_marker
  = state.live_state_requirement
```

Without these checks, a malformed public boundary could contain mutually
inconsistent spend-domain, state-transition, or live-status semantics. This is
not a hidden-fiber leakage attack, but it is a boundary-sufficiency bug: one
`q` could carry incoherent public faces unless the verifier treated the
obstruction flags as binding proof evidence and rejected them. The current
carrier/preflight path did not require a zero/valid obstruction flag, so the
safe fix is to make these gluing equalities executable assembly gates.

## Fix Applied

`assemble_path2_native_boundary_v1` now rejects:

```text
Path 2 native assembly spend domain mismatch
Path 2 native assembly spend-state transition mismatch
Path 2 native assembly lifecycle-state live marker mismatch
```

The native projection fixture was updated so the positive case is actually
glued:

```text
spend_domain_separator_digest = context.spend_verifier_domain_digest
allowed_state_transition_class_digest = state.consumed_successor_state_class
live_state_requirement = lifecycle.live_status_marker
```

KAT `path2_native_boundary_assembly_rejects_independent_digest_inputs` now
mutates each edge and requires rejection.

## Security Interpretation

This closes the immediate red-team finding for boundary sufficiency at the
implemented public assembly layer:

```text
same q cannot hide a different spend-domain, state-transition class, or
live-status face through these coordinates without changing a checked public
projection or causing assembly rejection.
```

The remaining red-team items are separate:

- false obstruction acceptance beyond these concrete edges;
- authorization-class confusion inside compatible private auth;
- DA/projection-preimage ambiguity;
- no-export leakage;
- QROM/domain multi-target replay;
- checked-core composition.

## Verification

Commands:

```text
cargo test --manifest-path tools/parameter-search/Cargo.toml path2_native_boundary_assembly --lib
cargo test --manifest-path tools/parameter-search/Cargo.toml path2_native --lib
```

Both pass after the fix.
