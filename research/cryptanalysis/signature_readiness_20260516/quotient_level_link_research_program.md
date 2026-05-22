# Quotient-Level Account Link Research Program

This is a sidecar research program.  It must not block, replace, or mutate the
current q12289 account-carried proof and implementation path.  The deployable
track should continue using the reviewed bounded-kernel/account-link design
while this program investigates whether a cleaner quotient-level link can
combine representative validity, account binding, and kernel exclusion from
first principles.

## Research Thesis

The current proof path separates several jobs:

```text
quotient digest
  + account-carried hidden-link evidence
  + representative validity bridge
  + bounded response-link kernel certificate
```

The descent-interface framing suggests these may be shadows of one semantic
object: a quotient-level account link.

The target primitive is a map

```text
link : R -> L
```

from hidden representatives to an account/link quotient, together with a
semantic projection

```text
semantic : R -> Q
```

such that `L` is the coarsest public/account object that still makes descent
effective: accepted local data glues to account-backed hidden state, or yields
a named break.

## Firebreaks

- Do not change the current q12289 verifier, proof path, byte target, or
  implementation while this research is open.
- Do not replace the `[0,3]` bounded-kernel account-link certificate without an
  independently replayed certificate for a concrete alternative.
- Do not move any result from this file into a deployable claim until it has a
  machine-checkable artifact, a Lean-facing theorem shape, and independent
  cryptanalytic review.
- Research outputs should be sidecar docs, scripts, and generated reports under
  `research/` unless a later reviewed migration plan says otherwise.

## Program State

- active milestone: complete, no replacement found
- last updated: 2026-05-22
- current sidecar artifacts:
  `quotient_level_link_kernel_search.{md,json}`,
  `quotient_level_link_coordinate_pair_sweep.{md,json}`,
  `quotient_level_link_hiding_review.{md,json}`,
  `quotient_level_link_account_semantics.{md,json}`,
  `quotient_level_link_lean_interface_comparison.md`,
  `quotient_level_link_hidden_commitment_byte_state_review.{md,json}`,
  `quotient_level_link_existing_commitment_reuse_review.{md,json}`,
  `quotient_level_link_promotion_gate.{md,json}`,
  `quotient_level_link_implementation_handoff.md`,
  `quotient_level_link_firebreak_audit.{md,json}`,
  `scripts/research/verify_quotient_level_link_closure.py`,
  `quotient_level_link_sampler_replay_*_b96.json`,
  `quotient_level_link_exact_enum_coordinate-link-1-2_b96.json`, and
  `quotient_level_link_exact_enum_sparse-difference-link-0m3-1m2_b96.json`,
  plus exact coordinate-pair artifacts for `coordinate-link-1-14`,
  `coordinate-link-8-20`, and `coordinate-link-14-15`
- completed so far:
  reusable tool inventory, read-only baseline runner, candidate row families,
  candidate result schema, rank/fiber checks, LLL fallback probes, exact
  enumeration support, first control-row artifact, first hand-picked sparse
  linear link probes, coordinate-pair sweep, exact passes for one lower-margin
  coordinate alternative, one sparse alternative, and the top three sweep
  candidates
- current finding:
  the first exact replayed coordinate-pair shortlist has three candidates
  (`coordinate-link-1-14`, `coordinate-link-8-20`, `coordinate-link-14-15`)
  with shortest coefficient bound `190`, exact margin `94`, rank `14`, fiber
  dimension `9`, and packed link size `4` bytes.  This is only a sidecar
  bounded-kernel result; it is not a deployment recommendation until semantic
  factorization, hiding, sampler, account-state, and independent review gates
  pass.
  The semantic-factorization screen now shows that no two-coordinate
  projection candidate standalone determines the 12-row semantic quotient:
  these rows are auxiliary account/bounded-kernel rows for the product quotient
  `semantic x link`, not standalone semantic-digest replacements.
  The initial hiding ledger shows that the top candidates match the current
  control on rank, packed link bytes, and raw product-fiber dimension, but raw
  q-residue capacity is only `122` bits.  The current control reaches the
  reviewed `133`-bit hiding floor through the packed low-bit support formula
  `12289^9 * 3073`; non-current candidates need candidate-specific sampler
  replay before receiving any support-size or zero-TV claim.
  The first sampler replays now reject the current exact-passed public-link
  shortlist under public link conditioning: `coordinate-link-1-14` has support
  floor `108` bits, while `coordinate-link-8-20` and
  `coordinate-link-14-15` have support floor `106` bits.  Even the current
  `[0,3]` row would have only `120` bits if its link coordinates were treated
  as public quotient data, which clarifies why the existing path keeps account
  binding separate from the reviewed hidden-representative sampler surface.
  Workstream D now selects `hidden-commitment-quotient-link` as the only
  remaining research branch: link values must stay hidden behind binding/hiding
  commitments, with a non-public proof of the link equation, or the sampler
  support target fails.
  The sidecar Lean module `Descender.HiddenCommittedQuotientLink` builds and
  shows that the hidden-commitment interface refines the existing
  `AccountLinkProofSystem`; it sharpens the break buckets but does not yet
  remove a deployable proof obligation.
  The hidden-commitment byte/state review found no deployable byte win:
  public links fail hiding, hash-only hidden links lose pre-response backing,
  adding hidden link commitments to the current commitments is dominated, and
  Pedersen-style commitments introduce new assumptions/proof bytes.  The only
  remaining research-only branch is reusing existing account commitments to
  derive link binding without revealing link values.
  The existing-commitment reuse review closes that branch for deployment:
  current byte-hash account commitments bind full hidden preimages only after
  opening, and do not expose either a hidden quotient-link commitment or a
  homomorphic equation for `link(z) = link(r) + c * link(w)`.  Opening the link
  values reintroduces the public-link hiding failure, while keeping them hidden
  requires a new zero-knowledge/hash-circuit proof with unknown nonzero bytes
  and new assumptions.  No byte-saving or proof-simplification candidate has
  survived the sidecar screens.
  The promotion gate classifies the program outcome as `no_replacement` and
  `keep_current_path`: do not migrate the deployable q12289 verifier, proof
  path, byte target, or account-link implementation based on this research.
  Future work may reopen the branch only with a concrete commitment/proof
  system, replayed sampler evidence, a q12289-shaped Lean instantiation, and
  independent cryptanalytic review.
  The import firebreak audit passes: `Descender.HiddenCommittedQuotientLink`
  is not imported by the deployable Lean aggregate, and
  `Descender.DescentInterface` is only imported by that sidecar prototype.
- next concrete step:
  no implementation migration; keep the current reviewed split design.  If this
  research is reopened, start from the future reopening requirements in
  `quotient_level_link_promotion_gate.md`, then rerun
  `scripts/research/quotient_level_link_firebreak_audit.py` before touching
  deployable imports.  Use
  `scripts/research/verify_quotient_level_link_closure.py` as the single
  closure-bundle consistency check.  The broader review-gate replay dry run now
  includes `quotient_level_link_closure_verify`, so changes to this closure
  path should be visible in the review replay checklist as well.

## Core Definitions To Investigate

Let:

- `R`: hidden representative module, e.g. packed full-response vectors.
- `Q`: semantic public quotient, e.g. q-syndrome digest/opening target.
- `L`: account/link quotient stored or committed before challenge use.
- `B`: accepted bounded response-difference box.
- `BadBound`: bounded differences that preserve public acceptance but allow a
  post-challenge representative attack.
- `link : R -> L`: candidate quotient-level account link.
- `semantic : R -> Q`: current semantic quotient map.

A candidate `link` is admissible only if it satisfies the following.

### 1. Semantic Factorization

The semantic quotient must factor through the link, or the link must refine the
semantic quotient:

```text
link(r0) = link(r1) -> semantic(r0) = semantic(r1)
```

Best case:

```text
semantic = project ∘ link
```

This prevents account state from binding to irrelevant metadata.

### 2. Challenge Equivariance

For sigma responses `z = r + c*w`, the link should be compatible with the
challenge action:

```text
link(z) = link(r) + c * link(w)
```

or a precisely stated variant over the actual challenge algebra.

If this fails, the verifier will probably need adapter bytes or extra proof
obligations.

### 3. Effective Descent

The verifier should prove existence of a representative over the linked
quotient, not validity of a globally exposed representative:

```text
accepted(public, proof)
  -> exists r,
       link(r) matches account state
       and r satisfies the local equations
```

or:

```text
accepted(public, proof) -> named_break
```

### 4. Bounded-Kernel Faithfulness

The link must kill dangerous bounded kernel directions without collapsing the
entire hidden fiber:

```text
ker(link) ∩ BadBound = {0}
```

This is the first-principles replacement target for the separate response-link
kernel certificate.

### 5. Hiding Capacity

The link must preserve enough harmless fiber entropy:

```text
H(R | link(R), public data, accepted local equations) >= target_margin
```

An injective link is not acceptable unless the proof explicitly moves hiding to
some other source.  The goal is a maximal safe quotient, not a singleton fiber.

## Workstreams

### Workstream A: Formal Model

Deliverable: `quotient_level_link_model.md`.

Tasks:

- Define `R`, `Q`, `L`, `B`, `BadBound`, `semantic`, and `link` for the current
  q12289 full-response setting.
- Map each existing artifact to the model:
  quotient digest, account link, response-link code, local-delta representative
  predicate, and bounded-kernel certificate.
- State the admissible-link conditions in Lean-facing terms compatible with
  `Descender.DescentInterface`.
- Identify which conditions are already implied by current artifacts and which
  require new search/certification.

Exit criteria:

- The model can explain the current `[0,3]` account link as one admissible or
  partially admissible point.
- The model explicitly names the gap between current artifacts and a unified
  quotient-level link.

### Workstream B: Kernel Geometry

Deliverable: `quotient_level_link_kernel_search.{md,json}`.

Tasks:

- Reuse the existing quotient/SIS and bounded-kernel search tooling as a
  read-only baseline.
- Enumerate candidate linear maps `link : R -> L` by row count and row family:
  coordinate projections, sparse linear combinations, block-local rows, and
  semantic-quotient refinements.
- For each candidate, test:
  - rank and fiber dimension;
  - whether `semantic` factors through `link`;
  - `ker(link) ∩ BadBound`;
  - shortest bounded bad vector found by exact enumeration or lattice search;
  - packed byte cost.
- Treat the current `[0,3]` link as a control row.

Exit criteria:

- A Pareto table of candidate links by bytes, fiber dimension, and bounded
  kernel margin.
- At least one negative-control candidate that fails for a clear reason.

### Workstream C: Hiding And Sampler Compatibility

Deliverable: `quotient_level_link_hiding_review.{md,json}`.

Tasks:

- For promising links from Workstream B, compute conditioned fiber capacity.
- Check whether the existing conditioned-fiber sampler can sample the new link
  fibers without rejection loops or extra timing surface.
- Separate raw fiber dimension from usable hiding after local fiber-state,
  pad, canonical packing, and challenge constraints.
- Record total variation or exact-support claims only where the sampler
  certificate actually proves them.

Exit criteria:

- No candidate is promoted unless it has a concrete hiding-capacity ledger.
- The report distinguishes byte savings from hiding loss.

### Workstream D: Account Binding Semantics

Deliverable: `quotient_level_link_account_semantics.md`.

Tasks:

- Specify what account state stores for a unified link: digest, linear syndrome,
  commitment, or packed quotient element.
- State the prechallenge binding game for `link`.
- Determine whether witness commitment and randomizer commitment can be merged,
  partially merged, or must remain distinct.
- Check lifecycle constraints: one-shot slots, rolling slots, precommitted
  pools, rollback/reorg behavior, and restore-anchor placement.

Exit criteria:

- A concrete account-state shape for each viable candidate.
- Explicit reasons when a candidate saves proof bytes but increases state or
  lifecycle risk.

### Workstream E: Lean Interface Prototype

Deliverable: a sidecar Lean module, not imported by deployment proofs unless
reviewed later.

Tasks:

- Extend `Descender.DescentInterface` only if the abstraction is clearly
  reusable.
- Define an `AdmissibleQuotientLink` structure with fields for semantic
  factorization, challenge equivariance, bounded-kernel faithfulness, and
  hiding capacity as abstract propositions.
- Prove a generic theorem:

```text
accepted + admissible_link + no_breaks -> account-backed quotient witness
```

- Instantiate only against abstract types first; do not touch the current
  q12289 theorem endpoint until Workstreams B-D identify a concrete candidate.

Exit criteria:

- Lean prototype compiles independently.
- The theorem shape can be compared directly against the existing
  `EffectiveDescentCertificate.accepts_glues_or_breaks` wrapper.

### Workstream F: Promotion Gate

Deliverable: `quotient_level_link_promotion_gate.md`.

Tasks:

- Define the minimum evidence required before any candidate affects the
  deployable path.
- Include:
  - exact replay command;
  - generated artifacts and hashes;
  - Lean theorem names;
  - cryptanalytic assumptions changed or removed;
  - byte/state/timing deltas;
  - independent review checklist.

Exit criteria:

- The current implementation agent can ignore this research until a promotion
  gate passes.
- If no candidate passes, the research still documents why the current
  separated artifacts are justified.

## Milestones

### Milestone 0: Baseline Map

Produce Workstream A.  No search yet.

Question answered:

```text
What exact quotient/link/fiber problem is the current design solving?
```

Progress checklist:

- [ ] Define the concrete current objects `R`, `Q`, `L`, `B`, and `BadBound`.
- [ ] Identify the current `semantic : R -> Q` map from existing q12289
  quotient artifacts.
- [ ] Identify the current account/link map, including the `[0,3]`
  response-link code and 68-byte account state.
- [ ] Map every current proof artifact to one of:
  representative validity, semantic quotient, account binding, kernel
  exclusion, hiding/sampler, or Fiat-Shamir frame binding.
- [ ] Record which current conditions already fit
  `Descender.DescentInterface`.
- [ ] Record which conditions are still ad hoc or artifact-specific.
- [ ] Write `quotient_level_link_model.md`.

Detailed outcomes:

- A diagram of the current proof path:

  ```text
  hidden representative -> semantic quotient -> account/link evidence
  ```

- A table with one row per current artifact and columns:
  `artifact`, `layer`, `predicate enforced`, `break event`, `byte/state cost`,
  `Lean theorem/source`.
- A precise statement of the current gap between separated artifacts and a
  unified quotient-level link.
- A baseline conclusion: either the current design already has a quotient-level
  link in disguise, or the missing fields are named explicitly.

### Milestone 1: Search Harness

Produce Workstream B with the current `[0,3]` target as control.

Question answered:

```text
Is the current response-link map close to Pareto-optimal among small links?
```

Progress checklist:

- [x] Inventory reusable scripts for quotient/SIS and bounded-kernel search.
- [x] Add a read-only baseline runner for the current `[0,3]` link.
- [x] Define candidate row families:
  coordinate projections, sparse linear rows, block-local rows,
  semantic-quotient refinements, and random control rows.
- [x] Define the exact candidate schema for JSON output.
- [x] Implement rank/fiber-dimension checks.
- [x] Implement semantic-factorization checks.
- [x] Implement exact bounded-kernel enumeration where feasible.
- [x] Implement lattice/LLL/BKZ fallback probes where exact enumeration is too
  large.
- [x] Run a full two-coordinate projection sweep with `[0,3]` as the control.
- [x] Exact-enumerate the top tied sweep candidates:
  `coordinate-link-1-14`, `coordinate-link-8-20`, and
  `coordinate-link-14-15`.
- [x] Classify exact-passed coordinate candidates by semantic-factorization
  compatibility before sending them to hiding review.
- [x] Emit `quotient_level_link_kernel_search.json`.
- [x] Emit `quotient_level_link_kernel_search.md`.

Detailed outcomes:

- A Pareto table with columns:
  `candidate`, `rows`, `packed bits/bytes`, `rank`, `fiber dimension`,
  `semantic factors`, `bounded-kernel status`, `shortest bad vector bound`,
  `search method`, `status`.
- Negative controls that fail for known reasons, including at least one
  candidate with a short bounded kernel vector.
- A direct comparison against `[0,3]`: better, equal, worse, or incomparable.
- A candidate shortlist for hiding review, with no deployment recommendation.

### Milestone 2: Hiding Ledger

Produce Workstream C for every candidate that beats or matches the current byte
cost.

Question answered:

```text
Do byte improvements destroy the hidden fiber needed for witness hiding?
```

Progress checklist:

- [x] Emit the initial hiding ledger for the current control, exact-passed top
  coordinate shortlist, and lower-margin exact comparison rows.
- [x] Compute raw product-fiber capacity for the exact-passed candidate ledger.
- [x] Replay candidate-specific public-link-conditioned support for the current
  exact-passed ledger.
- [x] Subtract known constraints:
  local fiber-state equality, pad constraints, canonical packing constraints,
  and public link-row conditioning.  Challenge-action and accepted-response
  bound refinements remain separate follow-up checks.
- [x] Check whether the current conditioned-fiber sampler applies unchanged to
  the public-link shortlist.
- [x] Record that the current exact-passed public-link shortlist fails the
  128-bit support target under public link conditioning.
- [x] Identify candidates requiring a hidden-link model or higher-dimensional
  public quotient surface.
- [x] Compute exact support size where possible.
- [x] Record no total-variation claim for rejected non-current candidates; any
  future TV claim must be backed by an executable or
  Lean-facing certificate.
- [x] Emit `quotient_level_link_hiding_review.json`.
- [x] Emit `quotient_level_link_hiding_review.md`.

Detailed outcomes:

- A hiding-capacity ledger with columns:
  `candidate`, `raw fiber bits`, `lost bits by constraint`, `usable hiding
  bits`, `sampler compatibility`, `TV/support claim`, `timing risk`,
  `status`.
- A clear rejection reason for every candidate whose byte savings consume too
  much hiding margin.
- A list of candidates that preserve or improve the current hiding posture.
- A statement of whether any candidate can reuse the existing sampler without
  changing production timing assumptions.

### Milestone 3: Account Object

Produce Workstream D for surviving candidates.

Question answered:

```text
Can one quotient-level account object replace separate representative,
account-binding, and kernel-exclusion artifacts?
```

Progress checklist:

- [x] For each surviving candidate, define the account object stored on-chain or
  committed prechallenge.
- [x] Determine whether the account object is a digest, linear syndrome,
  commitment, packed quotient element, or compound object.
- [x] State the binding game for the account object at the requirement level.
- [x] Determine whether witness commitment and randomizer commitment can merge
  for the screened hidden-commitment options: no deployable merge is currently
  justified; reuse remains research-only.
- [x] Determine whether the bounded response-link certificate is absorbed,
  weakened, or still required.
- [x] Estimate account-state bytes, spend bytes, live-read bytes, and touched
  state per spend.
- [x] Review lifecycle implications at the slot-byte level by comparing live
  reads and writebacks against the current rolling-slot account object.
- [x] Emit `quotient_level_link_account_semantics.md`.
- [x] Emit `quotient_level_link_hidden_commitment_byte_state_review.md`.

Detailed outcomes:

- One account-state proposal per surviving candidate.
- A byte/state table with columns:
  `candidate`, `account bytes`, `spend bytes`, `live read`, `writeback`,
  `proof bytes`, `total online+state`, `lifecycle risk`.
- A proof-obligation table showing which current assumptions are removed,
  merged, preserved, or made stronger.
- A yes/no answer for each candidate:

  ```text
  Can this replace the current 68-byte account-link object?
  ```

- If no candidate replaces it, a documented reason the current split is
  necessary.

### Milestone 4: Formal Prototype

Produce Workstream E only after at least one candidate has credible search and
hiding evidence.

Question answered:

```text
Does the abstraction reduce proof obligations, or merely rename them?
```

Progress checklist:

- [x] Define a sidecar hidden-commitment quotient-link interface in Lean.
- [x] Include fields or break buckets for commitment binding, public-projection
  exclusion, sampler-surface preservation, account binding, and bounded
  link-equation soundness.
- [x] Prove the generic theorem shape:

  ```text
  accepted + admissible_link + no_breaks -> account-backed quotient witness
  ```

- [x] Compare the theorem directly against
  `EffectiveDescentCertificate.accepts_glues_or_breaks`.
- [x] Compare the theorem directly against the existing
  `AccountLinkProofSystem`.
- [x] Defer abstract q12289-shaped instantiation until a commitment option
  survives; the existing-commitment reuse review found no deployable byte or
  proof-simplification candidate to instantiate.
- [x] Do not import the sidecar prototype into deployment theorem files.
- [x] Build the Lean sidecar module independently.

Detailed outcomes:

- A compiling Lean file with theorem names and comments that map back to the
  research model.
- A theorem-comparison note:
  `new obligations`, `removed obligations`, `renamed obligations`,
  `unchanged assumptions`.
- A decision on whether the abstraction genuinely simplifies proof
  composition.
- A list of exact fields needed for a future concrete q12289 instantiation.
- A stop condition for concrete instantiation: do not instantiate q12289-shaped
  hidden links until a new non-public equation proof or commitment algebra is
  costed below the current reviewed split design.

### Milestone 5: Promotion Decision

Produce Workstream F.

Question answered:

```text
Should a future agent migrate the deployable proof path, or keep the current
separated design?
```

Progress checklist:

- [x] Create `quotient_level_link_promotion_gate.md`.
- [x] Record all candidate artifact paths and hashes.
- [x] Record replay commands.
- [x] Record Lean theorem names and build commands.
- [x] Record cryptanalytic assumptions removed, added, or changed.
- [x] Record byte/state/timing deltas against the current reviewed target.
- [x] Record independent review requirements.
- [x] Classify the result:
  no replacement, compression candidate, proof-simplification candidate, or
  migration-ready candidate.
- [x] If migration is recommended, write a separate migration plan with staged
  changes and rollback criteria.  Migration is not recommended, so no migration
  plan is produced.

Detailed outcomes:

- A promotion matrix with columns:
  `candidate`, `evidence complete`, `bytes saved`, `state changed`,
  `assumptions changed`, `Lean support`, `external review status`,
  `recommendation`.
- A final decision:

  ```text
  keep current path
  ```

  or:

  ```text
  prepare reviewed migration plan
  ```

- Explicit instruction for the implementation agent: ignore this research
  unless and until the promotion gate says otherwise.
- If promoted, a risk ledger listing what can regress relative to the current
  account-carried bounded-kernel path.

## Expected Outcomes

There are three acceptable endings.

1. **No replacement found.** The current separated account-link plus
   bounded-kernel design is justified as a locally optimal engineering point.
2. **Compression found.** A quotient-level link absorbs the response-link
   kernel certificate or account-link bytes without reducing hiding margin.
3. **Proof simplification found.** Byte cost stays similar, but the unified
   link removes adapter lemmas or makes the verifier-composition theorem
   cleaner.

Any of these is useful.  The research fails only if it produces attractive
terminology without executable evidence.
