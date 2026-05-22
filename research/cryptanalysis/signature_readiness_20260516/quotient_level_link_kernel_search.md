# Quotient-Level Link Kernel Search

Status: generalized link-row LLL probes generated.

This is a Milestone 1 sidecar artifact for the quotient-level account
link research program.  It does not change the current q12289
account-carried proof path, verifier, byte target, or implementation.

## Scope

Milestone 1 asks whether the current response-link map is close to
Pareto-optimal among small links.  This pass normalizes the existing
reviewed artifacts into a reusable candidate schema and adds a first
hand-picked set of generalized linear-link LLL probes.  LLL-positive
margins are evidence only; exact enumeration is still required before
any pass claim.

## Base Surface

| item | value |
|---|---:|
| modulus `q` | `12289` |
| representative residues | `23` |
| semantic quotient rows | `12` |
| bare quotient rank | `12` |
| bare quotient kernel dimension | `11` |
| response-difference bound | `96` |

Matrix domain:

```text
Descender.Signature.q12289.local_delta.sis_full_response_matrix.v1
```

Matrix transcript SHA-256:

```text
b21703e6381ec794407acf6313af5e347d591b65251ad1f8e7a24d06b0bb71a6
```

## Tool Inventory

| tool | current role | Milestone 1 reuse | gap |
|---|---|---|---|
| `scripts/research/response_link_bounded_kernel_check.py` | Reconstructs the 12x23 q12289 full-response matrix and checks coordinate-projection response-link kernels with LLL, lattice enumeration, exact enumeration, CP-SAT, or SCIP modes. | yes | Currently centered on coordinate projections; sparse linear row families need a generalized link-row input format. |
| `scripts/research/verify_response_link_bounded_kernel_certificate.py` | Replays an exact bounded-kernel certificate and validates basis/hash consistency. | yes | Needs generalized certificate schema if link rows are not coordinate projections. |
| `scripts/research/quotient_sis_lattice_attack_review.py` | Builds the bare 12x23 quotient/SIS q-kernel lattice and runs LLL/BKZ plus sparse signed relation probes. | yes | Useful as the bare-quotient negative control; does not enumerate link candidates. |
| `scripts/research/response_link_lattice_attack_review.py` | Runs lattice attack review for the selected response-link target. | yes | Needs candidate-parameterized invocation and result normalization. |
| `tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs` | Rust reference/report source for full-response quotient parameters, response-link code frontier, sampler and account-link surfaces. | yes | Useful for source-of-parameters and controls, but not a sidecar search harness by itself. |

## Candidate Families

| family | description |
|---|---|
| `coordinate_projection` | Expose or commit selected representative coordinates as the link. This includes the current [0,3] response-link code. |
| `sparse_linear_rows` | Use sparse linear combinations of representative residues as link rows. Search should prefer low Hamming weight rows before dense random rows. |
| `block_local_rows` | Use rows aligned to local block structure. These are candidates for cleaner verifier/account semantics if they preserve challenge equivariance. |
| `semantic_quotient_refinement` | Refine the existing 12-row semantic quotient with a small number of extra quotient rows, aiming to absorb response-link soundness into the quotient itself. |
| `random_control` | Random low-row links used only as negative/scale controls; not deployment candidates without interpretability and replayable certificates. |
| `full_kernel_control` | Expose enough coordinates to kill the entire public quotient kernel. This should be sound but not hiding-friendly, so it is a control rather than a deployment target. |

## Candidate Schema

Each search result should emit:

| field | meaning |
|---|---|
| `candidate` | stable string id |
| `family` | coordinate_projection \| sparse_linear_rows \| block_local_rows \| semantic_quotient_refinement \| random_control \| full_kernel_control |
| `link_rows` | number of link rows/residues exposed or committed |
| `packed_bits` | estimated packed bit cost for link rows |
| `packed_bytes` | estimated packed byte cost for link rows |
| `rank` | rank of semantic quotient plus link constraints when known |
| `fiber_dimension` | dimension remaining after semantic quotient plus link constraints when known |
| `semantic_factors` | true if semantic quotient provably factors through link; false if not; null if not checked |
| `semantic_factorization` | rank-based rowspace check distinguishing standalone semantic factorization from product-quotient auxiliary links |
| `bounded_kernel_status` | pass \| fail \| unknown \| control |
| `shortest_bad_vector_bound` | coefficient bound of shortest detected vector in ker(semantic) cap ker(link), if known |
| `search_method` | source of evidence |
| `status` | control \| candidate \| rejected \| needs_search \| needs_exact_enum \| needs_probe \| needs_hiding_review |

## Control Rows

| candidate | rows | bytes | rank | fiber dim | bounded-kernel status | shortest bound | status |
|---|---:|---:|---:|---:|---|---:|---|
| bare-quotient-negative-control | 0 | 0 | 12 | 11 | fail | 68 | rejected |
| current-coordinate-link-0-3 | 2 | 4 | 14 | 9 | pass | 139 | control |
| full-kernel-coordinate-code-control | 11 | 20 | 23 | 0 | pass | - | control |

## First Generalized Probes

These rows use the generalized linear-link evaluator.  They are not
certificates.  A `needs_exact_enum` row only means LLL did not find a
bounded bad vector.  `auxiliary_product_quotient_only` means the
candidate rows can accompany the semantic quotient but do not
standalone determine it.

| candidate | family | rows | bytes | rank | fiber dim | semantic class | shortest bound | margin | evidence/status |
|---|---|---:|---:|---:|---:|---|---:|---:|---|
| coordinate-link-0-1 | coordinate_projection | 2 | 4 | 14 | 9 | auxiliary_product_quotient_only | 115 | 19 | needs_exact_enum |
| coordinate-link-0-2 | coordinate_projection | 2 | 4 | 14 | 9 | auxiliary_product_quotient_only | 117 | 21 | needs_exact_enum |
| coordinate-link-1-2 | coordinate_projection | 2 | 4 | 14 | 9 | auxiliary_product_quotient_only | 115 | 19 | needs_hiding_review |
| sparse-sum-link-0p3-1p2 | sparse_linear_rows | 2 | 4 | 14 | 9 | auxiliary_product_quotient_only | 121 | 25 | needs_exact_enum |
| sparse-difference-link-0m3-1m2 | sparse_linear_rows | 2 | 4 | 14 | 9 | auxiliary_product_quotient_only | 117 | 21 | needs_hiding_review |
| sparse-cross-link-0p7-3p11 | sparse_linear_rows | 2 | 4 | 14 | 9 | auxiliary_product_quotient_only | 107 | 11 | needs_exact_enum |

The current `[0,3]` control has response-difference bound `96`; exact enumeration reports shortest intersection-vector coefficient bound `139`, giving margin `43`.  Its replay manifest payload hash is:

```text
da10b823ad8f4142dfeccdbf7bbcddbb5e7148f420d8ff36687bae412d09fbf0
```

The bare quotient negative control has a shortest reduced full-kernel
row with coefficient bound `68`, inside the response-difference bound.  That row does not vanish on
coordinates `[0,3]`, which is why the current response-link code
catches it.

## Current Interpretation

The existing evidence supports the current `[0,3]` link as a working
control, not as an optimality claim.  The next harness must search
alternatives under the same result schema and make the comparison
explicit.

## Next Required Work

- Run the first broader coordinate-pair sweep with current [0,3] as control.
- Promote any coordinate-pair sweep candidates near or above the current [0,3] margin into exact-enum certificates.
- Add sparse-row candidate generation rather than the current hand-picked sparse probes.
- Start the hiding ledger for exact-pass non-current candidates if their byte cost remains competitive.
