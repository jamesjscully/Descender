# Overnight Parameter Search Summary

Run started: 2026-05-15T07:59:05Z  
Initial run finished: 2026-05-15T09:30:59Z  
Certified validation rerun finished after wiring the validator certificate route.  
Threads: 12

## Scope

Two search lanes were run for `descender-128`, `descender-192`, and `descender-256`.

- `compressed_*`: broad lossy/view/diagonal search over compressed public samples.
- `exact_*`: full-view exact lane that should validate under the current proof boundary.
- `diag_targeted/*`: narrower lossy-budget guard-only search forced near the diagonal certificate support-distance regime.
- `diag_targeted/*_validation_certified`: rerun after wiring the guard-block lossy diagonal certificate into the validator.

All searches and validators exited with status `0`.

## Fully Validated Current-Proof Rows

These rows use the full-view proof route and pass the current validator.

| profile | q | n | m0 | m1 | eta | pk bytes | ct bytes | security proxy | guard distance | route |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| descender-128 | 131071 | 24 | 40 | 24 | 1 | 200 | 168 | 129.49 | 17 | full-view |
| descender-192 | 131071 | 36 | 56 | 36 | 1 | 260 | 228 | 194.23 | 21 | full-view |
| descender-256 | 131071 | 48 | 80 | 48 | 1 | 336 | 304 | 258.97 | 33 | full-view |

Validator pass counts:

- `exact_descender-128_validation`: 328 / 328 passed.
- `exact_descender-192_validation`: 331 / 331 passed.
- `exact_descender-256_validation`: 313 / 313 passed.

## Broad Compressed Lane

The broad compressed search found smaller raw rows, but none validated under the current proof boundary.

| profile | best raw pk | best raw ct | route | validator blocker |
|---|---:|---:|---|---|
| descender-128 | 142 | 110 | lossy-budget-candidate | view certificate missing; combined certificate missing |
| descender-192 | 176 | 144 | support-covered-needs-hitting-set | view certificate missing; combined certificate missing |
| descender-256 | 232 | 200 | support-covered-needs-hitting-set | view certificate missing; combined certificate missing |

The broad objective preferred very small uncertified rows. None of the retained broad rows had positive diagonal slack.

## Targeted Diagonal-Certificate Lane

The targeted run recovered rows with positive diagonal certificate slack. After the validator was updated to accept the guard-block lossy diagonal certificate route, these rows validate.

| profile | q | n | m0 | m1 | eta | sample bits | view | pk bytes | ct bytes | security proxy | diag req | diag slack | route |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| descender-128 | 65537 | 32 | 80 | 32 | 1 | 12 | 80 | 184 | 152 | 130.01 | 49 | 9.48 | guard-block-lossy-diagonal-certified |
| descender-192 | 131071 | 40 | 96 | 40 | 2 | 15 | 96 | 244 | 212 | 193.62 | 53 | 56.01 | guard-block-lossy-diagonal-certified |
| descender-256 | 524309 | 48 | 112 | 48 | 1 | 16 | 112 | 288 | 256 | 257.03 | 64 | 21.15 | guard-block-lossy-diagonal-certified |

Targeted validator route counts:

- `descender-128_validation_certified`: 41 / 308 rows passed; min pk/ct `184 / 152`.
- `descender-192_validation_certified`: 19 / 306 rows passed; min pk/ct `244 / 212`.
- `descender-256_validation_certified`: 42 / 306 rows passed; min pk/ct `288 / 256`.

The remaining failed rows still lack the view-reduction certificate: they are either below the diagonal support-distance threshold or do not satisfy the guard-block lossy route preconditions.

## Interpretation

The old full-view boundary gives `200 / 260 / 336` byte public keys for the three levels. The certified diagonal route improves those to `184 / 244 / 288` bytes.

The proof/validator gap that blocked these rows has been closed at the validator boundary:

- The numeric security and distance screens pass for the targeted diagonal rows.
- The lossy diagonal support-distance proxy passes.
- The validator now accepts the guard-block Cauchy distance as the pair-coordinate support certificate when the selected view contains the full guard block, the field is prime, and the diagonal bad-seed inequality has positive slack.

Next work should focus on deeper estimator validation, higher bounded-dual search, and deciding whether the 128-bit row can be pushed below `184` bytes without dropping below the diagonal support-distance threshold.
