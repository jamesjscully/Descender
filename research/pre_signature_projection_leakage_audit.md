# Pre-Signature Projection Leakage Audit

Question: if the 32-byte Fq12 account witness commitment is redefined to pack
the 12 checked-core syndrome residues plus the six selected account-witness
projection coordinates, does revealing those six coordinates in account state
change the leakage model or replay surface?

## Current Model

- Public account state carries a 32-byte witness commitment that canonically
  packs 12 q12289 Fq12 syndrome residues.
- The selected 308-byte projection-carrier path reveals 12 q12289 carrier lanes
  at spend time: six account-witness projection lanes and six pad-source
  projection lanes.
- The carrier is transcript-bound to the slot, public key, message, checked-core
  signature body, statement digest, and opening commitment.
- Accepted verification consumes the slot and is protected by the spent-slot and
  restore-anchor lifecycle rules.

## Proposed 18-Residue Account State

The existing 32-byte witness commitment has enough bit capacity for 18 q12289
residues:

- 18 residues x 14 bits = 252 bits.
- 32 bytes = 256 bits.
- 4 high bits remain as canonical slack.

The proposed layout is:

- residues 0..12: checked-core Fq12 syndrome residues.
- residues 12..18: selected account-witness projection source coordinates.

The checked-core verifier would consume residues 0..12. The hidden-source
projection verifier would consume residues 12..18 instead of accepting a
separate arbitrary account projection-opening frame.

## Leakage Change

This changes the leakage timing and population:

- At spend time, it does not necessarily increase total account-projection
  leakage if the six account lanes are removed from the carrier or treated as
  redundant public state. The same six q12289 values are visible to the verifier
  either way.
- Before spend time, it does increase public leakage: every live account slot
  reveals six additional q12289 linear values before the message, Fiat-Shamir
  challenge, signature body, and pad statement are fixed.
- For unused or never-spent slots, this is strictly more public information than
  the current projection-carrier design.
- For wallets/accounts that prepublish many future slots, this becomes
  cross-slot public leakage. The security theorem must charge 18 public Fq12
  rows per live slot, not 12 rows per live slot plus six account projection rows
  only for spent slots.

This is not automatically fatal. The existing account-state model already treats
the 12 Fq12 syndrome residues as public pre-signature data. The proposed change
is clean if the keygen/sampler proof can tolerate six more public linear
functionals per slot, and if those rows are included in the structural
cryptanalysis and leakage budget.

## Replay Surface

The replay surface does not appear to worsen if the verifier enforces the
following invariants:

- The 18-residue commitment is canonical and profile-tagged, so legacy 12-residue
  commitments cannot be silently interpreted as 18-residue commitments.
- The Fiat-Shamir transcript binds the full 32-byte account witness field, not
  just the first 12 residues.
- The state-assisted opening commitment binds the same 32-byte account witness
  field stored in the slot.
- Carrier transcript binding continues to include slot id, public key digest,
  message digest, signature digest, statement digest, and opening commitment.
- Accepted verification consumes the slot and records the spent slot under the
  existing spent-ledger and restore-anchor policy.

Under those rules, putting the six projection coordinates in account state does
not create a new carrier replay path. A carrier still cannot be moved to another
message, slot, account state, or signature body without breaking transcript
binding or lifecycle checks.

## New Failure Modes To Test

- Legacy 12-residue canonical commitments must be rejected by the 18-residue
  selected profile unless explicitly converted.
- Mutating any of residues 12..18 must change the hidden-source statement and
  reject the carrier unless all dependent transcript bindings are recomputed for
  the same valid slot.
- Reusing a carrier after the slot is consumed must still reject.
- Replaying the same 18-residue account state under a different slot id, message,
  public key, signature body, or restore anchor must reject.
- Publishing many unused slots must be included in leakage probes; the red-team
  input is the matrix with 18 public rows per live slot.

## Current Answer

Revealing the six account projection coordinates pre-signature probably does not
change the replay surface if the selected verifier keeps the existing
transcript-binding and lifecycle rules.

It does change the leakage model. The leakage claim becomes stronger: the system
must be secure with 18 public q12289 rows per live slot, visible before message
selection and even for slots that are never spent. This is the core question to
settle before adopting the 18-residue packed witness-state design.

## Recommended Next Experiment

Build a structural leakage probe for the selected keygen/account-slot sampler
with public rows:

- current baseline: 12 Fq12 syndrome rows per live slot;
- proposed design: 18 rows per live slot, where rows 12..18 are the selected
  account-witness projection source coordinates;
- spent-signature view: proposed account state plus the remaining six pad-source
  carrier lanes and the existing checked-core transcript.

If the 18-row pre-signature account-state view has no practical distinguisher,
short relation, or key-recovery signal under the existing attack probes, the
18-residue design is the most efficient way to close the full packed-witness
opening gap without expanding the 308-byte signature.

## Probe Result

`scripts/research/pre_signature_projection_leakage_probe.py` adds the first
conservative public-row probe for this redesign. It models the checked-core
syndrome rows and account projection rows in separate namespaces, because no
bridge theorem currently identifies those coordinates.

Artifact:
`research/cryptanalysis/signature_readiness_20260516/pre_signature_projection_leakage_probe_n32_q12289.md`

Result:

- baseline live slot: 12 rows over 54 modeled coordinates, rank 12, kernel
  dimension 42.
- proposed live slot: 18 rows over the same modeled coordinates, rank 18,
  kernel dimension 36.
- additional rank per live slot: 6.
- small public-row dependency probe: none found up to support 3 with
  coefficients in `{-2, -1, 1, 2}`.
- sparse public-view column-kernel probe: the baseline model has support <= 3
  zero relations involving selected account projection columns, which is the
  executable shape of the old projection rebinding gap. The proposed 18-row
  model has no zero relation involving checked-core columns or selected account
  projection columns in the same probe; remaining zero relations involve only
  nonselected hidden projection coordinates.
- multi-slot scaling is linear: 1, 2, 4, and 8 live slots add 6, 12, 24, and 48
  public rows/rank respectively, with no cross-slot dependency modeled.
- signature byte increase required by this layout: 0.
- carrier account-lane bytes that can become redundant if the carrier derives
  them from account state: 12.

Interpretation: the immediate cost is exactly six independent public q12289
rows per live slot. The probe does not show a new algebraic row-dependency or
carrier replay vector, but it also does not prove keygen/public-view
pseudorandomness under the stronger 18-row live-slot leakage model.

`scripts/research/pre_signature_projection_statistical_probe.py` adds the
nonlinear/statistical gate. It separates two cases:

- uniform-looking projection residues: no immediate marginal distinguisher in
  the bounded probe;
- small centered witness-coordinate residues: severe distinguisher. Across
  20,000 sampled accounts with eight live slots each, all projection residues
  are centered within `[-12, 12]`, while the uniform model has only about
  `0.00198` of residues in that range and no sampled account with all projection
  values small.

This is a hard promotion gate. The 18-residue design should not replace the old
model in Lean or account-state validation unless the selected account projection
residues are proved uniform-looking/one-time-masked, or the security theorem is
explicitly changed to tolerate six public small witness coordinates per live
slot and key-recovery probes are rerun under that stronger leakage assumption.

## Masked 18-Residue Prototype

Option A is now prototyped as a Rust-only masked relation. Instead of packing
raw account projection residues, the prototype packs
`raw_projection_i + mask_i mod q`, where `mask_i` is derived from the pad seed,
slot id, and a dedicated domain separator:

`Descender.Signature.q12289.local_delta.reviewed_fq12_packed_witness_state18_projection_mask.v1`

Added helpers:

- `reviewed_fq12_packed_witness_state18_projection_masks`
- `reviewed_fq12_pack_masked_witness_state18_from_parts`
- `reviewed_fq12_masked_packed_witness_state18_projection_opening`
- `reviewed_fq12_masked_state18_packed_witness_opening`
- `verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_opening`
- `verify_reviewed_fq12_state_assisted_masked_witness_state18_packed_witness_opening`
- `verify_reviewed_fq12_state_assisted_masked_witness_state18_projection_carrier_tuple`
- `verify_reviewed_fq12_state_assisted_masked_projection_soundness_certificate`

The regression `masked_witness_state18_unmasks_projection_with_pad_seed`
confirms:

- packed public residues differ from the raw small projection coordinates;
- the correct pad seed unmasks the projection and verifies the carrier;
- the full packed witness opening extracts the 12 checked-core residues, six
  masked projection residues, six masks, six unmasked projection residues, and
  a repacked commitment equal to the committed 32-byte witness field;
- candidate acceptance packages a local soundness certificate containing the
  checked-core challenge/response syndrome, packed opening, pad-seed digest,
  carrier lanes, carrier check bytes, and transcript binding;
- a wrong pad seed rejects before acceptance;
- carrier rebinding to a different embedded projection still rejects through
  the hidden-material break-export path.

The statistical probe now includes the pad-seed-masked centered model. Its
public projection residues match the uniform marginal test:

- uniform small-centered fraction: about `0.001977`;
- raw centered small-centered fraction: `1.0`;
- pad-seed-masked centered small-centered fraction: about `0.001973`.

`scripts/research/pre_signature_projection_mask_independence_probe.py` adds a
bounded check that the new projection-mask stream is not accidentally reusing
the existing pad PRF projection lanes. Across 50,000 pad seeds, it found:

- distinct domains for pad lanes and projection masks;
- maximum centered Pearson correlation: about `0.004880`;
- total mask-pad equality fraction: about `0.000097`, close to `1/q`;
- adjacent slot six-lane mask vector equality fraction: `0.0`.

This is evidence against an implementation-level stream collision, not a PRF
proof. It leaves the cryptographic domain-separation theorem and lifecycle
prechallenge-fixation theorem as proof obligations.

`scripts/research/pre_signature_projection_mask_frame_disjointness.py` adds a
byte-frame disjointness certificate for the two SHA-256 input layouts. The pad
PRF input is `pad_seed || 46-byte pad_frame || counter` and is 82 bytes. The
projection-mask input is
`mask_domain || pad_seed || slot_id || slot_id || lane || counter` and is 154
bytes. The certificate checks 1,536 bounded block/lane/counter pairs and finds
zero byte-equal collisions. The Rust regression
`projection_mask_prf_frame_is_disjoint_from_pad_prf_frame` checks the same
frame lengths, domain offsets, bounded disjointness, and range rejections.
The hidden-source acceptance contract now also carries this as an explicit
required obligation, `mask_prf_domain_separation`, and leaves it missing until
the SHA-256 PRF/random-oracle theorem is supplied. The executable frame
certificate is evidence for the implementation layout; it is not credited as a
cryptographic independence proof.

`scripts/research/pre_signature_projection_mask_prf_assumption_api.py` pins the
remaining assumption boundary. It defines the adversary view for the masked
18-residue public state, the forbidden view, the named break event, and the
theorem obligations that remain before Lean/account-state promotion:

- SHA-256 PRF/random-oracle indistinguishability for the projection-mask domain;
- multi-domain separation from pad PRF lanes;
- lifecycle prechallenge fixation of `pad_seed`, `slot_id`, and packed witness
  bytes;
- composition with revealed pad-opening rows and carrier lanes;
- single-use pad-seed policy under crash/reorg recovery.

`scripts/research/pre_signature_projection_mask_prf_leakage_simulator.py`
turns that boundary into a real/ideal simulator contract. The live-slot ideal
world replaces only the six masked projection residues inside the packed witness
field with independent uniform `Z_q` residues while preserving checked-core
residues and every public field not deterministically derived from those six
packed residue bytes. A separate valid-spend world records that opening
the slot intentionally discloses the selected projection opening through the
certificate interface; this is not counted as live-slot pre-signature leakage.
The artifact remains fail-closed and now consumes four child contracts:

- `pre_signature_projection_mask_prf_ro_reduction_n32_q12289` specifies the
  ROM/SHA-256 assumption boundary, mask input frame, query symbols, hybrids, and
  named breaks;
- `pre_signature_projection_lifecycle_same_game_lift_n32_q12289` places
  account-state reads, restore/reorg attempts, spend attempts, and verifier
  queries into one lifecycle experiment;
- `pre_signature_projection_spend_time_leakage_budget_n32_q12289` enumerates
  the 21 accepting-certificate disclosures and marks the six selected
  projection residues as intentional post-spend disclosure;
- `pre_signature_projection_reject_equivalence_n32_q12289` enumerates the seven
  real/simulated failure classes that must collapse to the same public reject
  surface.

Those contracts make the proof graph concrete, but they do not yet close the
cryptographic gap. The next step is to compose them into one mask-PRF leakage
theorem or explicitly carry their remaining fail-closed assumptions into the
final advantage bound.

`scripts/research/pre_signature_projection_mask_prf_leakage_theorem_composition.py`
now performs that composition as a fail-closed theorem target,
`pre_signature_projection_mask_prf_leakage_or_named_break`. It carries five
assumption groups into the final bound: SHA-256/RO treatment, global domain
registry, lifecycle same-game controls, spend-time projection budget, and
reject equivalence. The artifact records 13 named breaks and 20 open items, so
it is usable by global soundness as a single dependency but still does not make
the masked projection route deployable.

`scripts/research/pre_signature_projection_pad_seed_reuse_surface.py` makes the
single-use pad-seed item concrete. With the same pad seed in slots 0 and 1, the
projection masks differ because `slot_id` enters the mask stream, but the
pad-seed digest repeats and the public pad-opening statement repeats when the
observed block set is the same. This does not by itself break the masked
projection, but it shows that single-use pad-seed policy must be enforced or
proved harmless; it is not implied by slot-id-separated masks.

`scripts/research/pre_signature_projection_pad_seed_reuse_closure_options.py`
compares the two closure paths. The preferred implementation path is to add a
public `pad_seed_digest` to each state-assisted account slot record/snapshot
and reject duplicate digests at account-state construction and restore. This
costs 32 public bytes per live slot and requires a schema migration, but it
turns same-seed reuse into a public deterministic invariant and catches duplicate
live slots before any spend choice. The fallback implementation path is a
durable spend-time `pad_seed_digest` ledger; this avoids adding the digest to
every unspent slot, but it only catches the second accepted spend and still
requires consensus/storage schema changes. The proof-only path is possible in
principle, but it must prove that repeated pad-seed digests, repeated
pad-opening statements, shared-seed mask streams, carrier lanes, and crash/reorg
choice do not compose into leakage or replay. That expands the already-open
PRF/leakage theorem, so it should be avoided unless both storage approaches are
rejected.

The preferred path is now executable in the Rust state-assisted account schema.
`ProductionReviewedFq12StateAssistedAccountSlot` and
`ProductionReviewedFq12StateAssistedAccountSlotRecord` carry `pad_seed_digest`;
account-state construction, checked record export, and snapshot restore reject
zero or duplicate pad-seed digests. Slot opening now checks the stored
`pad_seed_digest` before accepting the hidden-source opening commitment, so a
postcommit pad-digest mutation fails at the lifecycle boundary rather than being
treated as only a generic opening-commitment mismatch.

The target-chain/spent-ledger boundary now persists the same digest for
selected-projection spends. `ProductionSpentSlotRecord` carries an optional
`pad_seed_digest`; selected-projection acceptance records it, consensus spend
records expose it, and target-chain commit validation rejects zero, mismatching,
or duplicate selected-projection pad-seed digests. This makes single-use
pad-seed enforcement durable across in-memory, transactional KV, file-backed KV,
and SQLite-backed canonical-node store conformance tests.

The executable lifecycle boundary now has a slot-bound masked verifier helper:
`verify_reviewed_fq12_state_assisted_masked_witness_state18_slot_projection_carrier_tuple`.
It first derives the opening commitment from the prechallenge account slot and
only then runs the masked projection-carrier tuple check. This required a
state-assisted Fq12-specific commitment validator, because the generic account
commitment validator still describes the older account-carried witness shape
and rejects 18-residue packed witnesses. The regression now checks:

- slot-bound masked tuple acceptance for the precommitted opening;
- postcommit pad-digest mutation rejection at the slot-opening boundary;
- duplicate account-state pad-seed digest rejection during construction and
  persisted-record restore;
- durable spent-ledger `pad_seed_digest` persistence and duplicate-digest
  target-chain commit rejection;
- alternate 18-residue witness replay rejection against the original slot;
- wrong pad seed and carrier rebinding rejection at the masked tuple boundary.

This makes the masked branch the current preferred path. The remaining hard
proof obligation is to show that the pad-seed-derived masks are independent of
the raw account projection values under the final state lifecycle, are
domain-separated from pad PRF projection lanes, and cannot be selected
post-challenge. The single-use lifecycle side is now closed at the executable
account-state and spent-ledger schema level; remaining lifecycle work is final
deployment database review, broader crash/reorg adversary coverage, and theorem
integration.

## Rust Prototype Result

The Rust-only prototype now has explicit 18-residue helpers:

- `pack_reviewed_fq12_packed_witness_state18`
- `pack_reviewed_fq12_packed_witness_state18_from_parts`
- `unpack_reviewed_fq12_packed_witness_state18`
- `reviewed_fq12_packed_witness_state18_projection_opening`
- `verify_reviewed_fq12_state_assisted_packed_witness_state18_projection_carrier_tuple`

The production-reference regression
`packed_witness_state18_projection_carrier_closes_projection_rebinding`
checks three things:

- the 18-residue packed state round-trips the 12 checked-core syndrome residues
  and six account projection residues;
- legacy 12-residue Fq12 unpacking rejects an 18-residue witness state when the
  projection residues are nonzero, so the prototype is profile-separated from
  the old packed-syndrome meaning;
- a carrier generated from packed witness state A cannot be rebound to packed
  witness state B with different embedded account projection coordinates. The
  verifier rejects through the hidden-material break-export path.

This closes the specific projection-opening rebinding counterexample at the
Rust relation level. It does not yet promote the account-state schema or the
deployment verifier to the 18-residue model.
