# Path 2 MITH Full-Witness Probe

Status: fallback proof-family probe; not the active selected endpoint.

Purpose: decide whether an MPC-in-the-head or related full-witness hash-ZK
proof could replace the transparent committed-trace theorem if transparent
knowledge soundness or no-export cannot be made deployable.

## Target

The fallback must prove the same active Path 2 native-boundary account-creation
target:

```text
accepted account-creation boundary b
  =>
exists h.
  InFiber(B_c, b, h)
  LocalValid_c(h)
  DescentConsistent_c(h)
```

It must not export the hidden representative, pad source, opening tuple,
projection relation, range witnesses, masks, blinding/noise, or raw role
columns.

## Theorem Shape

A MITH proof-family theorem should be stated as:

```text
MithFullWitnessAccepts(b, public_view, proof)
  =>
    exists trace h.
      transparentExtractsTrace(b, public_view, trace)
      and traceOpensBoundary(b, selected_profile, trace)
      and maskedOpeningsConsistent(b, public_view, selected_profile, trace)
      and traceRelationValid(b, selected_profile, trace)
      and traceAssemblesRepresentative(trace, h)
      and extractedRolesComplete(b, selected_profile, h)
      and InFiber(B_c, b, h)
      and LocalValid_c(h)
      and DescentConsistent_c(h)
      and NormRangeValid(b, h)
      and BlindingNoiseBounded(b, h)
  or
    boundaryBreakFor(b.context, b, breakEvent)
```

with separate side theorems for:

```text
MITH special soundness / extractor rewind loss
MITH simulator indistinguishability / no-export
Fiat-Shamir QROM schedule and multi-target loss
binding from MITH witness lanes to the selected Path 2 public boundary
```

The cleanest Lean integration would be a parallel endpoint or selected-profile
mode that consumes `fullWitnessExtractable` and `fullWitnessRelationKnowledge`,
then maps the extracted witness back into the existing `ExtractedTrace` and
`HiddenRepresentative` surfaces so the trace-assembly and native validity
lemmas can be reused.

## Byte Floor

Current active planning inputs:

```text
NativeBoundaryTraceRelationV1 working rows = 176
range/account witness lane floor = 194
range/account witness bytes if opened = 340
parked_lattice_sigma_floor = 16416 bytes
```

The existing lattice-sigma toy model used an 80-lane / 66-row floor:

```text
first_msg_per_round = 116 bytes
response_per_round = 140 bytes
per_round = 256 bytes
64 repetitions = 16416 bytes
```

Scaling only the response to the active 194-lane witness floor gives an
optimistic MITH floor:

```text
first_msg_per_round = 116 bytes
response_per_round = 340 bytes
per_round = 456 bytes
```

| repetitions | optimistic proof floor | read |
|---:|---:|---|
| 16 | 7296 | attractive only with wide challenges and tight QROM accounting |
| 24 | 10944 | plausible if challenge soundness is strong |
| 32 | 14592 | near the 16416-byte comparator |
| 40 | 18240 | above the comparator |
| 64 | 29184 | byte-heavy fallback |

This is not a concrete MITH proof-size estimate. It ignores view commitments,
seed trees, challenge encoding, relation-evaluation messages, repetition
aggregation, and any cost from SHA-heavy witness logic. It is only a lower
bound for deciding whether the fallback is worth specifying.

Scaling the existing 64-round sigma skeleton by both active row and lane floors
is more conservative:

```text
old q-ops per round = 66 rows + 80 lanes = 146
working q-ops per round = 176 rows + 194 lanes = 370
old 64-round verifier floor = 9344 q-ops
working 64-round verifier floor = 23680 q-ops
```

Using the existing 194-lane response floor and keeping the old first message as
an optimistic lower bound gives:

| relation | rows | lanes | 64-round byte floor | verifier q-ops floor | read |
|---|---:|---:|---:|---:|---|
| old parked sigma | 66 | 80 | 16416 | 9344 | comparator |
| optimistic active MITH | 160 | 194 | 35232 | 22656 | too large unless theorem quality dominates |
| working active MITH | 176 | 194 | 36256 | 23680 | about 2.2x bytes and 2.5x q-ops |
| stress active MITH | 220 | 194 | 39072 | 26496 | not byte-competitive |

This pessimistic table is probably the right default comparison unless a
concrete MITH design justifies fewer repetitions through wider challenge
action with a quantified QROM/forking loss.

## Go / No-Go

Go if:

- soundness can use a wider challenge or lower repetition count so the proof
  remains near or below `16416` bytes;
- opened MPC views are simulatable and do not reveal raw hidden Path 2 roles;
- witness lanes stay algebraic and avoid hidden SHA preimage circuits;
- extractor output maps cleanly into the active `ExtractedTrace` and
  `HiddenRepresentative` contracts;
- verifier work is acceptable for account creation.

No-go if:

- the active relation requires roughly 40 or more repetitions at the 194-lane
  response floor;
- the 64-round row/lane extrapolation is representative of the concrete proof;
- opened views expose enough linear information to reconstruct hidden roles;
- role restrictions require SHA-heavy circuits inside the MITH relation;
- the proof is byte-heavier than the transparent stress row without a clear
  theorem-quality or implementation advantage.

## Current Read

MITH is theorem-cleaner than the transparent route because full-witness
extraction and no-export are native to the construction. It is probably not
smaller under a 64-round sigma-like skeleton: the active working extrapolation
is about `36256` bytes, above the `16416`-byte comparator. It remains valuable
as a theorem-quality escape hatch if transparent extraction or no-export fails.
It becomes byte-competitive only if a concrete wide-challenge construction
cuts repetitions while preserving QROM soundness and simulator privacy.

The next MITH-specific task is a concrete challenge-width model: repetitions,
challenge bits per repetition, QROM/forking loss, and opened-view bytes for
the exact `NativeBoundaryTraceRelationV1` witness layout.
