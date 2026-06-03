# Path 2 Native Red-Team: QROM / Domain Multi-Target Replay V1

Status: closed for the current active-native hash ledger.

## Attack Question

Do active native domains, digest fanout, or deployment query bounds leave a
multi-target, cross-context, or replay surface that is not charged in the
current QROM/hash review?

## Finding

The active-native domain registry was globally disjoint after the recent
red-team fixes, and the DA digest was already present in
`Path2NativeHashQromReviewV1`.

The pass found an accounting gap: the two hash surfaces introduced by the prior
red-team fixes were registered as production domains but not named in the active
native query ledger:

- authorization-class digest
- no-obstruction digest

That was not an immediate codec collision, because the domains were unique and
registered. It was a proof-accounting gap: a deployed theorem should charge
these queries explicitly instead of relying on an implicit "miscellaneous hash"
bucket.

## Blue-Team Fix

Updated `Path2NativeHashQromReviewV1`:

- active query symbols increased from 11 to 13;
- added `q_native_authorization_class`;
- added `q_native_no_obstruction_flags`;
- ordinary hash assumptions increased from 6 to 8;
- added assumptions for authorization-class tuple binding and unique
  no-obstruction flag binding.

## Tests

Covered by:

- `path2_native_hash_qrom_review_tracks_public_transcript_assumptions`
- `path2_active_native_domains_and_public_schemas_are_unique`
- `production_domain_schema_registry_is_globally_disjoint`

Commands run:

- `cargo test --manifest-path tools/parameter-search/Cargo.toml path2_native_hash_qrom_review --lib`
- `cargo test --manifest-path tools/parameter-search/Cargo.toml domain --lib`

## Residual Risk

This pass closes the obvious active-native hash-ledger omission. It does not
prove a full QROM reduction for the checked-core proof system; that remains in
the checked-core composition review.
