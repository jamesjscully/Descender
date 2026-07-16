# Path2AccountStarkTranscriptQromTheoremV1

> Classification: `ACTIVE_SUPPORT`. Supports the active Poseidon2/Plonky3
> account-creation path (`ADR-0004`, tracker rows `DIR-QROM-001`,
> `DIR-PROOF-001`). It is not normative by itself. The residual authority is
> `spec/security_model.md` "Selected Account-STARK Residual Matrix", and
> `TRACKER.md` is the top-level status source.

Status: candidate account-STARK Fiat-Shamir/QROM reduction boundary. The local
selected-profile arithmetic is machine-checked under explicit imported bounds;
the transcript theorem, protocol mapping, and full residual pricing are not
instantiated.

## Obligation

The account-creation proof boundary needs one reduction that connects an
accepting non-interactive Plonky3 proof to either:

1. an accepting interactive FRI/account-STARK transcript from which the selected
   extractor obtains a valid witness; or
2. a named FRI, transcript, sponge/QRO, parser, or commitment break.

`Path2AccountStarkFriQromResidualReportV1` deliberately keeps the account-STARK
transcript theorem, residual pricing, external review, deployment-profile
acceptance, and deployment readiness false.

## Pinned Implementation Inputs

- Proof system: Plonky3 `HidingFriPcs` over KoalaBear.
- FRI schedule: `log_blowup = 4`, `num_queries = 120`, query PoW 16.
- Challenge field: degree-8 KoalaBear extension.
- Merkle/MMCS hash: width-16 Poseidon2, rate 7, capacity 9, 13-element digest.
- Fiat-Shamir challenger: `DuplexChallenger<Val, Perm, 16, 7>`, hence rate 7
  and capacity 9.
- Deployment quantum-oracle query bound: `Q = 2^16`.
- Checked-core primitive target horizon: `T = 2^32` (not multiplied into the
  account-STARK adaptive-soundness term under `ADR-0011`).

The 139-bit `poseidon2_sponge_floor_bits` and
`poseidon2_fs_challenger_floor_bits` report values describe distinct
Merkle/MMCS and Fiat-Shamir uses of the same rate-7/capacity-9 profile. Equal
generic floors do not merge their domains or proof obligations into one global
security claim.

## What The Published FRI Result Gives

Block, Garreta, Katz, Thaler, Tiwari, and Zając analyze the BCS transform for
FRI in the random-oracle and quantum-random-oracle models. For the ROM theorem,
their concrete loss has the form

```text
Q * epsilon_rbr + 3 * (Q^2 + 1) / 2^kappa
```

where `Q` is the adversary's oracle-query bound, `epsilon_rbr` is the
round-by-round soundness error, and `kappa` is the random-oracle output length.
Their QROM result gives error `Theta(Q * epsilon_fs)` against bounded-query
quantum adversaries, where `epsilon_fs` is the preceding ROM error. See:

- <https://par.nsf.gov/servlets/purl/10510337>
- <https://eprint.iacr.org/2023/1071>

This is not the same statement as "divide the entire classical exponent by two."
It also proves the result for FRI/batched FRI; extending it to the complete
selected account-STARK transcript, its hiding PCS, all Plonky3 challenge rounds,
and the account-creation extractor remains a project obligation.

## Correction To Prior Pricing

The earlier note treated

```text
120 * (log_blowup / 2) + 16 = 256
```

as a classical marked-search exponent and then halved it to claim a 128-bit
quantum Fiat-Shamir floor. That is a parameter-sizing heuristic, not an
instantiated theorem from the cited source. The current code keeps `256` as a
conservative schedule label, but it must not be reported as composed QROM
security until the following values are supplied to one reduction:

- the selected protocol's exact round-by-round FRI error;
- the Fiat-Shamir/BCS error before the QROM lift;
- a deployment quantum-oracle-query bound `Q`;
- the effective `kappa` for every challenger output used by the proof;
- the Poseidon2 sponge-to-QRO replacement loss for the migrated rate-7 challenger;
- commitment/MMCS collision and hiding terms;
- the selected FRI/AoK extractor and AIR relation errors.

The old 56-query examples remain useful negative-control history, but they are
not the active profile and are not a proof of the current profile.

## Conditional Local Arithmetic

`Descender/Path2HashBudget.lean` now prices the published error shape under the
following explicit candidate inputs:

```text
Q = 2^16
T = 2^32
kappa = 247
epsilon_rbr <= 2^-256
C_qrom <= 2^32
```

Here `C_qrom` is a deliberately generous bound on the unspecified constant in
the paper's `Theta(Q * epsilon_fs)` statement. The normalized multi-target
terms are

```text
C_qrom * T * Q^2 / 2^256
C_qrom * T * Q * 3 * (Q^2 + 1) / 2^247.
```

`selectedAccountStarkFriQromRbrTerm_le_q130_v1` and
`selectedAccountStarkFriQromBcsOracleTerm_le_q130_v1` prove each term is at
most `2^-130`; `selectedAccountStarkFriQromNormalizedBudget_le_q128_v1`
proves their sum is at most `2^-128`. The generated residual report and
seven-case domain corpus pin those dimensions and theorem names.

This is conditional arithmetic, not transcript closure. In particular,
`kappa = 247` is only a candidate effective-output mapping for the degree-8
challenge. The account-STARK now uses a rate-7/capacity-9 duplex, so one
extension challenge spans two output blocks; that source shape alone does not
prove that every BCS oracle call exposes 247 effective bits. The report therefore keeps
`bcs_challenger_output_mapping_instantiated`,
`bcs_rbr_error_theorem_instantiated`, and
`bcs_qrom_lift_constant_reviewed` false.

## Published-Bound Single-Leaf Instantiation

The conditional `2^-256` input is stronger than the explicit bound obtained by
substituting the current single-leaf proof dimensions into Corollary 4. The
implementation pins two trace rows; hiding doubles the committed degree, so
`degree_bits = 2`, and blowup 16 gives `|L0| = 2^6 = 64`. With
`|F| = 2130706433^8`, `rho = 1/16`, `m = 3`, `delta = 191/256`, and 120
queries, `Descender/Path2HashBudget.lean` proves both published RBR terms are at
most `2^-218`.

That candidate bound exposes a policy distinction:

- in the theorem-native adaptive-one-forgery game, the selected QROM allowance
  gives RBR and oracle terms bounded by `2^-154` and `2^-165`, and their sum is
  below `2^-128`;
- multiplying the same RBR field term by an additional `T = 2^32` lifetime
  union factor does not establish `2^-128`.

The latter is a failure of that stronger lifetime profile, not a lower bound on
the protocol's actual error. `ADR-0011` selects the theorem-native adaptive
game: one adversary may choose statements and attempts adaptively, and `Q` is
global across all of its random-oracle queries. The checked-core account target
count is therefore not multiplied into this term. A stronger wall-clock
lifetime failure profile remains separately parameterized and out of scope for
the selected deployment theorem.

## Candidate Closure Statement

For a fixed deployment profile with explicit oracle-query bound `Q`, prove that
an adversary producing an accepted account-creation proof with probability
`epsilon` yields either an extracted witness satisfying the account-creation
relation or one named break, with a composed bound no larger than the profile's
accepted target:

```text
epsilon
  <= C_qrom * Q * (Q * epsilon_rbr
                   + 3 * (Q^2 + 1) / 2^kappa)
     + epsilon_sponge_qro
     + epsilon_mmcs
     + epsilon_extractor
     + epsilon_air
```

The exact decomposition may change when the selected Plonky3 transcript is
mapped to the paper's game. What matters is that every term is tied to the
actual verifier and an explicit query/profile parameter, rather than inferred
from an informal exponent conversion.

## Required Closure Work

1. Map every Plonky3 challenger observe/sample operation to the BCS/FRI game and
   identify which theorem hypotheses hold.
2. Complete the Plonky3-to-RBR hypothesis mapping for the machine-checked
   single-leaf `2^-218` candidate bound.
3. Prove that every relevant rate-7 challenger observe/sample operation,
   including the second squeeze for a degree-8 extension challenge, maps to a
   BCS oracle output with effective `kappa >= 247`.
4. Complete production-window assembly for the rate-7/capacity-9 recursive
   transcript: the monolithic AIR arithmetic covers full-plus-partial pre-alpha and
   quotient observes, alpha/zeta refills, OOD, batch-combination alpha, betas, and
   query continuation; the real pre-alpha/quotient stream is constraint-checked.
   Actual inner-proof columns for the complete OOD/opening stream still need one
   production settlement member window, pricing, and a KAT chain.
5. Obtain a reviewed explicit constant for the QROM lift no larger than the
   pinned `2^32` allowance.
6. Compose the result with the selected FRI/AoK extractor, AIR soundness, hiding,
   and commitment terms.
7. Obtain external FRI/QROM review and explicit deployment-profile acceptance.

## Honest Residual

- Poseidon2-as-ideal-permutation/QRO replacement is an explicit assumption.
- The `2^-256` RBR error remains only a conditional stress input. The explicit
  single-leaf candidate bound is `2^-218`; its Plonky3 hypothesis mapping and
  the `kappa = 247` challenger mapping remain uninstantiated.
- The reviewed QROM lift constant is not instantiated.
- The full account-STARK transcript theorem is not machine checked.
- Recursion has a separate outer transcript and is outside direct-mode closure.
- No current artifact may flip
  `account_stark_transcript_qrom_theorem_instantiated`,
  `residual_pricing_complete`, or `deployment_ready` based on this note alone.
