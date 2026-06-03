# Path 2 Native Red-Team: False Obstruction Acceptance V1

Status: second internal cryptoanalytic red-team pass.

Question:

```text
Can a public q pass active native carrier/preflight checks with obs_c(q) not
actually equal to zero?
```

## Result

The red-team pass found a concrete implementation gap: the active native public
surface carried an `obstruction_flags_digest`, but assembly/preflight did not
require it to be the canonical no-obstruction value.

That meant a malformed public boundary could carry arbitrary obstruction flags
while still passing carrier preflight, provided the rest of the projection
digests were self-consistent. This did not by itself show a spend forgery, but
it violated the intended theorem premise:

```text
AcceptCreate(c,q) => obs_c(q)=0
```

because the executable accept path did not force the `obs_c(q)=0` marker.

## Fix Applied

The implementation now defines:

```text
PATH2_NATIVE_NO_OBSTRUCTION_FLAGS_DOMAIN
path2_native_no_obstruction_flags_digest()
```

and `assemble_path2_native_boundary_v1` rejects unless:

```text
bundle.obstruction_flags_digest = path2_native_no_obstruction_flags_digest()
```

with the fail-closed reason:

```text
Path 2 native assembly obstruction flags are not cleared
```

The no-obstruction domain is also registered in the production-wide
domain/schema registry and exported through the production facade.

## Security Interpretation

The executable accept path now matches the active theorem premise at the public
boundary layer:

```text
carrier/preflight acceptance requires the canonical no-obstruction digest
```

Future nonzero obstruction certificates must not be accepted by this path until
they have their own explicit verifier and theorem. The current path is
zero-obstruction only.

## Verification

Commands:

```text
cargo test --manifest-path tools/parameter-search/Cargo.toml path2_native_boundary_assembly --lib
cargo test --manifest-path tools/parameter-search/Cargo.toml domain --lib
```

Both pass after the fix.
