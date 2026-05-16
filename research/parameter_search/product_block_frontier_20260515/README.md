# Product-Block Frontier Sweep, 2026-05-15

This run exercises the first exact product-block Lee route in the parameter
search and validator.  The grid is intentionally restricted to the sidecar that
has a global certificate:

- `q = 32771`
- `m1 = 32`
- `lee_repetition = 3277`
- `lee_rows_per_group = 5`
- sidecar:
  `research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.json`

The product-block route is:

```text
productBlockComposableSystematicLeeCertificate -> composableSystematicLeeDualCertificate
```

The validator checked the sidecar for every passing row.  All checked rows used
`80` product-block pivot rows and the sidecar had checked `21470904` centered
free-coordinate pairs.

## Sweep

Each profile used:

```text
mode: grid
q-values: 32771
n-values: 80,96,112,128,160,192,224,256,320,384,448,512
m0-values: 80,88,96,104,112,128,160,192,224,256,320,384,448,512,544
m1-values: 32
fiber-dims: 8,16,24,32,48,64,96,128,192,256,384,512
eta-values: 2,3,4,5,6,8,10
sample-bits: 0
view-coords: 0
proof-strategies: full-surjective
primal-distance-models: guard-only
qary-amplitude-strategies: product-block-lee
coefficient-model: seeded
incidence-model: seeded
public-key-model: component
top: 200
```

## Validated Frontier Summary

| profile | rows checked | rows passed | min passing pk | min passing ct | representative min-pk row |
|---|---:|---:|---:|---:|---|
| descender-128 | 200 | 200 | 1280 B | 304 B | `q=32771, n=80, m0=104, m1=32, k=48, eta=5` |
| descender-192 | 200 | 200 | 1440 B | 320 B | `q=32771, n=80, m0=128, m1=32, k=64, eta=5` |
| descender-256 | 200 | 200 | 1568 B | 352 B | `q=32771, n=80, m0=128, m1=32, k=96, eta=5` |

Artifacts:

- `search_descender-128/descender-128.json`
- `search_descender-192/descender-192.json`
- `search_descender-256/descender-256.json`
- `validation_descender-128/validation.json`
- `validation_descender-192/validation.json`
- `validation_descender-256/validation.json`
- `validation_descender-128_no_sidecar_check/validation.json`

The no-sidecar negative check validates that product-block rows are not allowed
through as heuristics: the sampled rows fail with
`shared_pair_global_certificate_missing`.

## Interpretation

This sweep confirms that the product-block proof route is now visible to the
frontier machinery as an exact, sidecar-checked construction rather than a
filter tweak.  The structural change shows up directly in the accounting:
`m1 = 32` is covered by `16` two-coordinate product blocks, each with `5` rows,
so the amplitude certificate consumes `80` rows and `160` modeled q-ary
key-generation multiplications.

The next structural bottleneck is no longer the global proof interface.  It is
the construction of better local two-coordinate blocks, and then larger
certified product-block sidecars for other `q`, `target_l1`, and
`rows_per_group` choices.
