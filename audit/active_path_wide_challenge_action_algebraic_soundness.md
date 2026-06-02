# Active Path Wide Challenge-Action Algebraic Soundness

- generated at: `2026-06-02T20:24:29.398097+00:00`
- blocks: `16`
- repetitions: `26`
- multifork draws per commitment: `6`
- current response bound: `[-48, 48]`

## Finding

There are two different challenge-action surfaces in the active Path 2/Fq12 stack.
The compact local surface decodes each repetition as a 4-bit block index and a 1-bit sign.
The checked-core Fq12 surface hashes the whole canonical challenge byte string into a 12-residue field action.

The realistic widening route is to add auxiliary canonical challenge bits consumed by the Fq12 action, while leaving the compact local block/sign semantics unchanged.
Changing the compact local coefficient alphabet is a much larger redesign.

## Route Assessment

### auxiliary_fq12_action_bits

- status: `highest probability of proof closure`
- description: Append canonical auxiliary challenge bits after the existing 130 compact-local bits. The Fq12 checked-core action consumes the whole longer challenge byte string while the compact local block/sign decoder keeps its current bit offsets.
- native checked-core acceptance already computes challenge = H(frame)
- Fq12 action hashes the whole challenge byte string into 12 q-residues
- appending bits after the compact-local prefix preserves the 26 existing 5-bit block/sign fields
- compact local response/opening semantics can remain 4-bit block plus 1-bit sign
- does not multiply active response coordinates by larger local coefficients

New obligations:
- update Lean/Rust challenge-byte constants and canonical high-bit mask
- prove the Fq12 action-collision and QROM budget for the longer byte string
- prove extra bits are bound into the selected checked-core frame and are not ignored by the Fq12 sigma equation
- keep any older account-carried compact-local verifier either updated or explicitly non-active

### wide_compact_coefficient_alphabet

- status: `possible but high redesign risk`
- description: Decode 7 bits as 4-bit block index plus an 8-value nonzero coefficient alphabet for the compact local equation.
- product-block verifier equation supports arbitrary q scalars
- current noise-preservation theorem only covers {-1,0,1}
- Rust challenge_mul rejects every coefficient except -1, 0, 1
- larger coefficients stress the [-48,48] one-byte response bound
- fixed-repetition compact multifork coverage remains weak for weight-one active blocks

### larger_block_universe

- status: `not a local patch`
- description: Decode 6 or 7 index bits as 64 or 128 possible active blocks.
- current public key, block-diagonal view, pad openings, and Lean block type are 16-block objects
- changing the block count rewrites public-key size, coverage, local state, and generated certificates
- does not preserve the fixed active response/opening byte accounting in the current theorem

## Coefficient-Alphabet Probe

This measures only the compact local multifork event for a weight-one active block design.
A block is extractable in one repetition if two same-commitment draws hit that block with different coefficients.

| coefficient choices | any-block miss probability | miss bits |
|---:|---:|---:|
| 2 | 0.999997109334 | 0.000004 |
| 4 | 0.999751992344 | 0.000358 |
| 8 | 0.998917090839 | 0.001563 |
| 16 | 0.997997476783 | 0.002892 |

The result is bad for coefficient widening as a compact-extraction strategy: even 8 coefficients at 26 repetitions and 6 draws gives only about 0.0016 bits against any missed block.

## Response-Bound Pressure

| max coefficient | shift bound if secret eta=12 | exact-box mask bound for same response bound |
|---:|---:|---:|
| 1 | 12 | 60 |
| 2 | 24 | 72 |
| 4 | 48 | 96 |

The one-byte response bound can remain at 48 only if the signer samples from a larger mask interval and proves exact rejection or bounded leakage for every coefficient-induced shift.

## Recommendation

- primary route: `auxiliary_fq12_action_bits`
- reason: It addresses the QROM/multi-target horizon with the smallest algebraic blast radius: the active native checked-core verifier already consumes whole challenge bytes through the Fq12 action, while compact local block/sign semantics and response bounds can stay unchanged.
- do not commit to: `wide_compact_coefficient_alphabet`
- blocker: It needs new coefficient-noise theorems, Rust verifier semantics, signer/rejection-sampling proofs, and still does not obviously buy useful compact multifork coverage at fixed 26 reps.
