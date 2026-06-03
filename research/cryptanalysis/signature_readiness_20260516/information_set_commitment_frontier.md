# Information-Set Commitment Frontier

Date: 2026-05-19

## Purpose

The question is not only which parameter row is best.  The deeper question is
which verifier architectures can bind a Fiat-Shamir challenge while paying the
fewest transcript bytes.

The active-only `69 B` payload is:

```text
17 B challenge || 52 B active responses
```

It is small because it only carries one active block per repetition.  The
missing issue is prechallenge commitment binding: the challenge must be derived
from a commitment state fixed before the challenge is known.

This note frames that obstruction as an information-set problem.

## Abstract Architecture

Let:

```text
W  = allowed prechallenge codeword/mask state
T  = full Fiat-Shamir commitment view
V  = active view carried by the signature for one challenge
C  = challenge space

commit : W -> T
pi_c   : T -> V
rec_c  : V -> T
```

For a challenge `c`, a stateless active-only verifier is soundly bindable only
if:

```text
rec_c(pi_c(commit(w))) = commit(w)
```

for every allowed codeword `w`.

If `commit` is injective on the allowed codeword space, this implies:

```text
w0 != w1  =>  pi_c(commit(w0)) != pi_c(commit(w1)).
```

So the active view must be an information set for the commitment code.

The Lean core is recorded in:

```text
Descender/InformationSetCommitment.lean
```

with the central theorem:

```text
card_codeword_le_activeView_of_reconstructs :
  exact reconstruction + injective commitment
  => |Codeword| <= |ActiveView|
```

This is the architecture-level lower bound.  The same Lean module now also
contains the product-block specialization:

```text
card_codeword_le_productBlockActiveView_of_reconstructs :
  exact reconstruction from ActiveBlock × LocalFree -> ZMod q
  => |Codeword| <= q^(#ActiveBlock * #LocalFree)

card_codeword_le_singleBlock_twoFree_of_reconstructs :
  exact reconstruction from one active two-free block
  => |Codeword| <= q^2
```

The same file now separates the commitment state from the sampled mask:

```text
FiberedArchitecture
card_commitState_le_activeView_of_reconstructs
card_maskState_le_activeView_of_reconstructs_of_stateInjective
card_commitState_le_singleBlock_twoFree_of_fibered_reconstructs
card_maskState_le_singleBlock_twoFree_of_fibered_reconstructs_of_stateInjective
```

This is the key distinction for trying to make the small stateless design work:
the active view may have to be an information set for the commitment state, but
it must not be an information set for the sampled mask itself.

## Consequence For Active-Only Designs

For our product-block shape:

```text
G = 16 blocks
local_free = 2
local_pivot = 3
```

A full per-repetition mask has `G * local_free = 32` q-symbol degrees of
freedom before rejection/box constraints.

A challenge of block weight `w` exposes at most:

```text
w * local_free
```

independent q-symbol degrees through the active systematic block view.  The
local pivot coordinates are algebraically determined by the local free
coordinates, so they do not add independent dimension.

Therefore, if an active-only stateless verifier is supposed to reconstruct the
full prechallenge commitment from only the active view, the allowed commitment
code must have effective dimension at most:

```text
dim(W) <= w * local_free
```

per repetition.

For `w = 1`, that says:

```text
dim(W) <= 2
```

per repetition.  The unrestricted mask has dimension `32`, so the `69 B`
active-only transcript cannot be made into a stateless Fiat-Shamir signature
unless the mask/commitment space is restricted to a very low-dimensional
information-set code, or unless the missing commitment information is supplied
outside the signature.

## What A Near-69B Stateless Design Would Need

A possible but unsafe first route is a low-dimensional mask code:

```text
a          in F_q^k
y_block b  = L_b(a)
t          = A*y
```

where every allowed active block is an information set:

```text
L_b : F_q^k -> F_q^2
```

must be injective for every block `b` that the challenge may select.

For the `w=1` target this forces:

```text
k <= 2.
```

That is a severe restriction.  It would make reconstruction cheap, but it also
means the per-repetition mask entropy and distributional proof are no longer
the same as the full independent-mask sampler.  This is a new cryptographic
construction, not a byte-serialization trick.

More importantly, this transparent mask-code route is not deployable as stated.
If the public active view lets the verifier recover `a`, or otherwise recover
the active mask coordinates `y_b`, then the active response immediately gives:

```text
s_b = chi_b^{-1} * (z_b - y_b)
```

for every signed active block.  This is already formalized by the known-mask
single-view extraction endpoints in:

```text
Descender/BlockChallengeSoundness.lean

SingleViewLinearRound.extractCoordinate_eq_secret
CompactSingleViewLinearRound.extractCoordinate_eq_secret
```

So the viable version is not an information-set mask code.  It is a fibered
information-set commitment code:

```text
Y       = high-entropy mask space
S       = low-dimensional commitment state
T       = public Fiat-Shamir commitment view

state       : Y -> S
commit      : S -> T
pi_c        : T -> V_c
rec_c       : V_c -> T

rec_c(pi_c(commit(state(y)))) = commit(state(y))
```

The active view can determine the commitment state, but it must leave many
possible masks in the fiber:

```text
{ y in Y : state(y) = s }
```

Otherwise `state : Y -> S` is injective, and the Lean theorem
`card_maskState_le_activeView_of_reconstructs_of_stateInjective` pushes the
same active-view cardinality bound onto the mask space itself.

The open questions for the viable fibered route are:

- Can the signer sample from high-entropy fibers over the low-dimensional
  commitment state?
- Can active commitment projections reconstruct `commit(s)` without revealing
  the active mask coordinates?
- Does the fiber sampler preserve the existing rejection/leakage certificate
  surface, or can it be certified with a small new proof object?
- Does the resulting public-view hardness assumption still match the original
  keygen problem?

There is also a fork-soundness condition.  If two accepting forked openings use
the same sampled mask, subtraction cancels the mask and extracts the secret.  If
a fibered commitment permits two different masks above the same reconstructed
state, the fork subtraction becomes:

```text
z0 - z1 = (y0 - y1) + (chi0 - chi1) * s
```

The new Lean endpoint
`productBlockLinearResponse_sub_with_mask_delta` records this identity, and
`extract_coordinate_of_deltaExtractable_with_mask_delta` shows that extraction
is recovered once the mask-delta term is known or zero.  Therefore a fibered
design needs a binding alternative:

```text
same reconstructed commitment state + different low-norm masks
=> collision / q-ary lattice witness
```

This is encouraging because it points to the right proof shape.  The fiber
does not have to be information-theoretically singleton; it has to be
computationally binding for the response-sized masks used in accepted
transcripts.

The deterministic fork alternative is now also recorded in Lean:

```text
SameStateMaskCollision
FiberedForkedLinearRound
FiberedForkedLinearRound.extraction_or_collision_of_cover
```

Given compact fork coverage, `extraction_or_collision_of_cover` proves that
the fork either has one mask per repeated commitment and extracts the
product-block secret, or one repeated commitment yields a valid same-state mask
collision.  The remaining construction-specific task is to instantiate
`stateOfMask` and `ValidMask`, then prove that such a collision is a q-ary
lattice/keygen witness.

That bridge is now explicit for bounded masks:

```text
BoundedMaskValid
SameStateMaskCollision.toModqLeeKernelViolation_of_boundedMasks
FiberedMaskStateCollisionCertificate
ModqLeeKernelViolation
modqLeeKernelViolationKeygenBreakGame
noModqLeeKernelViolationBreaker_of_noLeeKernelVectorBelow
```

For a generic subtractive state map, the bounded-mask bridge leaves a precise
construction obligation:

```text
stateOfMask(y0) = stateOfMask(y1)
=> stateOfMask(y0 - y1) = 0

accepted masks valid under BoundedMaskValid eta
Fintype.card Coord * (eta + eta) < leeMin

ModqNoLeeKernelVectorBelow q stateOfMask leeMin
```

Taken literally, this is too strong for a tiny one-symbol state map: a map from
the full product-block mask space to one `ZMod q` symbol has a large kernel.
That is exactly why the deployable version should not certify the kernel of the
public state map alone.

## Locally Recoverable Syndrome Candidate

The constructive version is now formalized in:

```text
Descender.LocallyRecoverableSyndrome
```

The candidate changes the proof target from "the tiny reconstructed state has
no short kernel" to "the block-consistency syndrome has no short valid
same-state collision."  A `LocalLinearSyndrome` gives every block a local linear
state:

```text
localState b view = sum_f coeff b f * view f
stateOfMask y      = localState default (blockView y default)
ValidMask eta y    = BoundedMaskValid eta y and all block local states agree
collisionMap delta b = localState b (blockView delta b)
```

The active reconstruction property is now by construction:

```text
LocalLinearSyndrome.active_reconstructs_state
```

If a sampled mask is consistent, the active block view itself reconstructs the
global state, so a Fiat-Shamir commitment can bind that state without recovering
the full mask.

The hiding boundary is also explicit:

```text
LocalLinearSyndrome.HasLocalKernel
LocalLinearSyndrome.localStateHiding_of_hasLocalKernel
```

This proves only the necessary algebraic condition: each local decoder must be
non-injective, so the active state does not determine the active mask.  A
deployable scheme still needs a concrete sampler/preimage argument showing that
the fiber distribution is secret-independent or leakage-bounded.

The collision branch is now reduced to the right q-ary witness:

```text
LocalLinearSyndrome.collisionMap_maskDelta_eq_zero
LocalLinearSyndrome.collisionToLeeViolation
LocalLinearSyndrome.CollisionCertificate.no_sameStateMaskCollision
ModqBoxKernelViolation
LocalLinearSyndrome.collisionToBoxKernelViolation
LocalLinearSyndrome.BoxCollisionCertificate.no_sameStateMaskCollision
```

For two bounded valid masks with the same reconstructed state, consistency makes
every block-local syndrome of the mask delta vanish.  If the delta is short
enough, this yields a `ModqLeeKernelViolation q S.collisionMap leeMin`.
Therefore a concrete `ModqNoLeeKernelVectorBelow q S.collisionMap leeMin`
certificate rules out the collision branch without requiring the tiny public
state map to be injective.

The sharper certificate is coordinate-wise, not Lee-only.  For the signature
collision branch the delta between two accepted masks is bounded in every
coordinate, so Lean now also records a box-kernel target.  This matters because
a one-equation map from two q-symbols has an unavoidable full q-ary kernel; a
large global Lee minimum is the wrong thing to ask for in this local state
construction.

The concrete test candidate is:

```text
N32Q12289LocalSyndrome
q = 12289
Block = Fin 16
LocalFree = Fin 2
eta = 12
localState_b(x,y) = x + 50*y
delta box = [-24,24]^2 per block
```

This is a formal collision-certificate test for the current `BoundedMaskValid
eta` abstraction.  If the deployed sampler admits a wider mask range than
`[-eta,eta]`, the box certificate must be rerun with that larger fork-delta
bound.

Lean proves:

```text
N32Q12289LocalSyndrome.localBoxKernelCheck_eq_true
N32Q12289LocalSyndrome.noBoxKernel
N32Q12289LocalSyndrome.no_sameStateMaskCollision
N32Q12289LocalSyndrome.localStateHiding
```

The important negative result is also now formalized:

```text
LocalLinearSyndrome.BoxCollisionCertificate.stateOfMask_injectiveOn_validMask
N32Q12289LocalSyndrome.stateOfMask_injectiveOn_validMask
```

For a direct mask-syndrome state, a strong enough bounded-collision certificate
implies injectivity on the accepted valid-mask space.  That closes the binding
branch, but it means the public state would identify the bounded mask if the
verifier learns the state and the mask support.  The theorem
`localStateHiding` is therefore only a global algebraic non-injectivity fact; it
is not a distributional hiding proof for bounded signing masks.

This narrows the deployable path.  The syndrome idea should be moved from a
direct mask readout to a commitment-domain or trapdoor/preimage relation:

```text
active opening reconstructs a low-dimensional public state
public state has no bounded valid collisions
but recovering the active mask from that state remains computationally hidden
```

In other words, binding by construction is achievable, but direct bounded
mask-syndromes recreate the known-active-mask leakage problem.

## Commitment-Domain Split

The next abstraction is also formalized in Lean:

```text
LocalLinearSyndrome.CommitmentCollision
LocalLinearSyndrome.NoCommitmentCollision
LocalLinearSyndrome.CommittedLocallyRecoverableState
CommittedLocallyRecoverableState.hiddenCollisionOfPublicCollisionOrCommitmentCollision
CommittedLocallyRecoverableState.no_publicCollision_of_no_hiddenCollision_noCommitmentCollision
LocalLinearSyndrome.no_publicStateCollision_of_boxCertificate_noCommitmentCollision
N32Q12289LocalSyndrome.no_publicStateCollision_of_noCommitmentCollision
```

This changes the public state from the direct hidden syndrome to a committed
state:

```text
hiddenState(mask)      = locally recoverable syndrome / preimage state
publicState(mask)      = commit(hiddenState(mask))
active reconstruction  = commit(localHiddenState(active view))
```

A same-public-state fork now has an explicit two-way split:

```text
same public commitment state
=> same hidden state collision
   or commitment collision
```

The existing box certificate closes the same-hidden-state branch.  The remaining
branch is exactly the cryptographic assumption we want to isolate: the public
commitment must be binding/collision-resistant for the hidden preimage domain,
and preimage recovery must be hard enough that the public state does not expose
the active mask.  The Lean theorem is intentionally assumption-parametric here;
it does not claim that the one-symbol `x + 50*y` hidden state is itself a
deployable hiding preimage.  A real instantiation still needs a high-entropy
hidden preimage or trapdoor/opening relation, plus byte accounting for whatever
opening data the active verifier needs.

The active-opening version is now explicit too:

```text
LocalLinearSyndrome.OpenedCommittedLocallyRecoverableState
OpenedCommittedLocallyRecoverableState.public_reconstructsOn_consistent
OpenedCommittedLocallyRecoverableState.hiddenCollisionOfPublicCollisionOrCommitmentCollision
LocalLinearSyndrome.toSaltedOpenedCommittedState
LocalLinearSyndrome.no_saltedPublicStateCollision_of_boxCertificate_noCommitmentCollision
N32Q12289LocalSyndrome.saltedOpening_signatureBytes_eq
```

For the compact `n=32`, `w=1`, `26`-repetition structural payload, the byte
formula is:

```text
17 B challenge + 52 B active responses + 26 * opening_bytes_per_rep
= 69 + 26 * opening_bytes_per_rep
```

So a per-repetition salt costs:

```text
128-bit opening salt: 69 + 26 * 16 = 485 B
256-bit opening salt: 69 + 26 * 32 = 901 B
```

Lean records these as:

```text
N32Q12289LocalSyndrome.salted128Opening_signatureBytes_eq
N32Q12289LocalSyndrome.salted256Opening_signatureBytes_eq
```

This is promising only as an accounting target.  The direct salted preimage
still reveals too much if the opening lets the verifier recover the hidden
state:

```text
LocalLinearSyndrome.saltedHiddenState_injectiveOn_validMask_of_boxCertificate
N32Q12289LocalSyndrome.saltedHiddenState_injectiveOn_validMask
```

Under the box certificate, equal hidden salted state implies equal bounded valid
mask.  Therefore a deployable construction cannot simply reveal the hidden
preimage `(local syndrome, salt)` as its active opening.  It needs one of:

- a trapdoor commitment/opening that lets the verifier reconstruct the public
  state without learning the injective hidden state;
- a zero-knowledge active opening for the committed hidden state;
- a different hidden domain whose active-recoverable component is not injective
  on the accepted bounded mask distribution.

## Trying Both Hiding Routes

Both candidate escape routes are now represented in Lean.

### Route A: Public-Only Trapdoor/ZK Opening

The public-only opening route is:

```text
LocalLinearSyndrome.PublicOpenedCommittedLocallyRecoverableState
PublicOpenedCommittedLocallyRecoverableState.public_reconstructsOn_consistent
PublicOpenedCommittedLocallyRecoverableState.NoPerfectHiddenExtractor
LocalLinearSyndrome.publicOpened_public_reconstructs_state
LocalLinearSyndrome.no_publicOpenedStateCollision_of_boxCertificate_noCommitmentCollision
N32Q12289LocalSyndrome.no_publicOpenedStateCollision_of_noCommitmentCollision
```

Here the active opening reconstructs only the public committed state:

```text
openPublicState(active view, opening) = commit(hiddenState(mask))
```

It does not have to reconstruct or reveal `hiddenState(mask)`.  The soundness
split remains the same: public-state collisions imply either hidden-state
collisions, closed by the box certificate, or commitment collisions, closed by
the commitment assumption.  This is the cleanest proof surface for a trapdoor
or ZK-style construction.  The remaining missing piece is concrete: specify
`openPublicState`, prove its correctness/zero-knowledge or preimage-hiding
property, and charge its opening bytes.

### Route B: Non-Injective Accepted-Mask Hidden Domain

The direct non-injective hidden-domain route is constrained by:

```text
LocalLinearSyndrome.ValidStateNonInjective
LocalLinearSyndrome.no_validStateNonInjective_of_boxCertificate
N32Q12289LocalSyndrome.no_validStateNonInjective
```

Under the current box-collision proof, accepted-mask non-injectivity is exactly
a same-state mask collision.  So it cannot coexist with the no-collision
certificate that closes the fibered fork branch.  In that proof shape, this
route fails.

There is one viable variant: allow same-state fibers, but make the fork mask
delta recoverable from active openings:

```text
FiberDeltaOpening
FiberDeltaOpening.correctedExtractCoordinate_eq_secret
```

This connects to the existing corrected-extraction theorem.  If two openings
for the same hidden state let the extractor recover
`mask0 - mask1`, then the fork can still extract the secret despite
non-injective fibers.  The cost is that the opening may reveal mask-delta
information, so the next concrete design would need both byte accounting and a
leakage argument for the delta-opening data.

### Constructive Variant: One-Time-Padded Local Fiber Openings

The useful higher-level object is not "reveal the mask" and not merely "hide
the mask".  It is:

```text
single opening hides local mask view
two same-fiber openings subtract to the local mask delta
```

This is now represented by:

```text
LocalFiberDeltaOpening
LocalFiberDeltaOpening.correctedExtractCoordinate_eq_secret
LocalFiberDeltaOpening.PerfectLocalOpeningHiding
LocalFiberDeltaOpening.additiveMaskedLocalOpening_perfectHiding
LocalFiberDeltaOpening.additiveOneTimePad
LocalFiberDeltaOpening.additiveCommittedOneTimePad
LocalFiberDeltaOpening.StateDigestBinding
LocalFiberDeltaOpening.HashBoundLocalDeltaForkedRound.correctedExtractCoordinate_eq_secret
```

The construction is the standard additive one-time-pad pattern.  For a fixed
hidden committed state, sample or derive a hidden local pad `p_b`.  The active
opening for block `b` is:

```text
opening_b = localMaskView_b + p_b
```

A single opening is perfectly hiding when `p_b` is uniform: for any two local
views, there is a bijection on pads that produces the identical opening.  This
is formalized without probabilities as
`additiveMaskedLocalOpening_perfectHiding`.

The important refinement is that the pad must be bound by the public
commitment, not publicly derived from it.  The committed construction treats the
hidden commitment preimage as containing the pad source.  Public commitment
equality plus the no-commitment-collision assumption forces equality of the
hidden preimage, hence equality of the pad source, while still leaving the pad
hidden in one transcript.  This is the role of
`LocalFiberDeltaOpening.additiveCommittedOneTimePad`.

Across a fork, the prechallenge digest binds the same public fiber/state via
`LocalFiberDeltaOpening.StateDigestBinding`.  Once two accepting local openings
refer to the same state digest, `HashBoundLocalDeltaForkedRound` proves that the
public states agree and the pad cancels:

```text
opening0_b - opening1_b = localMaskView0_b - localMaskView1_b
```

So the extractor obtains exactly the mask-delta term needed by corrected
extraction, while no single transcript has to reveal the mask.  This is the
algebraic structure we were looking for.  It turns the non-injective-fiber
route from "hope the collision is harmless" into an explicit fork-delta
opening relation.

For the current `n=32`, `q=12289`, two-free-coordinate local view, the naive
byte account is one 2-residue local opening per repetition.  Charging those as
four bytes gives:

```text
N32Q12289LocalSyndrome.localDeltaOpening_signatureBytes_eq  -- 173 B
```

Tighter bit packing can be evaluated separately; the theorem records the simple
byte-aligned target.

The remaining concrete cryptographic obligations are:

- instantiate the commitment/hash assumption used by
  `additiveCommittedOneTimePad` and `StateDigestBinding` with concrete bytes;
- instantiate the pad source from commitment randomness or a PRF/RO seed inside
  the hidden committed preimage;
- account for the active opening bytes, ideally only the active block-local
  masked view rather than a global object.

### Byte Commitment Instantiation

The byte-facing version is now isolated in:

```text
Descender.LocalDeltaByteCommitment
LocalDeltaByteCommitment.ByteCommittedPadPreimage
LocalDeltaByteCommitment.ByteCommitmentConfig
LocalDeltaByteCommitment.byteCommittedLocalDeltaOpening
LocalDeltaByteCommitment.ByteCommittedLocalDeltaForkedRound
LocalDeltaByteCommitment.byteCommittedLocalDeltaFork_correctedExtractCoordinate_eq_secret
```

The hidden preimage is serialized as:

```text
fiber-state bytes || pad-seed bytes
```

and committed with a domain-separated byte hash.  The exact assumption is
`ByteCommitmentConfig.noCollision`: no two different hidden preimages produce
the same public digest.  The byte digest itself is the public fiber state, and
`byteDigestBinding` is identity binding on those digest bytes.

The concrete N32/q12289 byte layout is now fixed:

```text
LocalDeltaByteCommitment.N32Q12289.commitmentDomainTag
  "Descender.Signature.q12289.local_delta.v1"

LocalDeltaByteCommitment.N32Q12289.encodeHiddenPreimage
  fiber-state[2] || pad-seed[32] = 34 B

LocalDeltaByteCommitment.N32Q12289.padDomainTag
  "Descender.Signature.q12289.local_delta.pad.v1"

LocalDeltaByteCommitment.N32Q12289.padFrame
  pad-domain[45] || block-index[1] = 46 B

LocalDeltaByteCommitment.N32Q12289.localViewOfPadBytes
  four PRF bytes -> two little-endian q-residues
```

The hash assumption has been moved to the byte boundary:

```text
LocalDeltaByteCommitment.ByteHashBinding
LocalDeltaByteCommitment.N32Q12289.CommitmentHashBinding
LocalDeltaByteCommitment.N32Q12289.CommitmentHashBinding.toCommitmentConfig
```

and the pad-source assumption has a concrete interface:

```text
LocalDeltaByteCommitment.N32Q12289.PadPRFBinding
LocalDeltaByteCommitment.N32Q12289.PadPRFBinding.expandPad
```

The PRF assumption is intentionally separate from collision resistance:
collision resistance binds hidden preimages to public digests, while pad
pseudorandomness makes the one-time-padded local openings hide the local mask
view.  The blockchain state model is represented by
`LocalDeltaByteCommitment.N32Q12289.BlockchainAccountState`, with a slot digest
and an unused-slot predicate.

The executable byte fixture is:

```text
tools/parameter-search/src/bin/signature_q12289_local_delta_stateful.rs
```

It implements the same layout with SHA-256:

```text
commitment_digest = SHA256(commitment-domain || hidden-preimage)
pad_bytes         = rejection-sampled q-residues from SHA256(pad-seed || pad-frame || counter)
opening_bytes     = encode_q(local-mask-view + pad-residue-view)
```

The challenge frame is deliberately prechallenge-only:

```text
challenge = SHA256(challenge-domain || profile || parameters ||
                   slot-id || public-key || message || slot-digest)
```

The active responses and local openings are checked against the derived
challenge after the fact.  They are not hashed into the challenge frame, because
including post-challenge openings would turn signing into a circular fixed-point
search.

The tests pin the domain lengths, preimage layout, pad frame layout, 177/181/1009
byte counts, and a sample digest/pad/opening vector.

For the N32/q12289 row the named byte constants are:

```text
local fiber state: 2 B
pad seed:          32 B
hidden preimage:  34 B
local opening:     4 B per repetition
SHA-256 digest:   32 B per repetition, if stateless
```

The key deployment distinction is now explicit:

```text
LocalDeltaByteCommitment.N32Q12289.statefulSpend_signatureBytes_eq
  -- 177 B when commitment digests are precommitted/stored externally

LocalDeltaByteCommitment.N32Q12289.statefulTransaction_signatureBytes_eq
  -- 181 B when the signature also carries the 4-byte slot id

LocalDeltaByteCommitment.N32Q12289.statelessSha256_signatureBytes_eq
  -- 1009 B when every repetition carries its 32-byte digest
```

So the 177-byte target is a stateful/precommitted spend payload.  It includes
the 17-byte algebraic challenge plus the 4-byte transcript-hardening suffix.  A
reusable stateless signature must either pay the digest bytes,
compress/aggregate them, or find a different way to make the verifier recover
the same prechallenge fiber commitments.

At this point the executable verifier is an ideal relation checker, not yet a
deployable public verifier: the test account stores the hidden preimage so the
program can check the local opening equation directly.  The remaining
commitment-representation task is to replace that ideal check with a compact
public proof or public opening representation without losing the 177-byte target
class.

The public-verifier boundary is now represented directly in Lean:

```text
LocalDeltaByteCommitment.N32Q12289.PublicOpeningProofSystem
LocalDeltaByteCommitment.N32Q12289.StatefulPublicOpeningVerifierRelation
LocalDeltaByteCommitment.N32Q12289.StatefulPublicOpeningVerifierRelation.openingWitness_of_accepts
LocalDeltaByteCommitment.N32Q12289.StatefulPublicOpeningVerifierRelation.algebraicVerifierObligations
LocalDeltaByteCommitment.N32Q12289.statefulTransactionWithProofBytes_eq
```

The theorem says the remaining cryptographic object has exactly one job:
public verification of the hidden opening relation must imply existence of the
same hidden preimage that the ideal verifier used.  Byte accounting is
`181 + proof_bytes`.

As a public-checkable control point, the executable fixture now supports a
witness-revealing proof mode:

```text
LocalDeltaByteCommitment.N32Q12289.revealedPreimageProofSystem
LocalDeltaByteCommitment.N32Q12289.revealedPreimageTransactionSignatureBytes_eq
revealed hidden preimage = 34 B
public-checkable transaction = 215 B
```

This closes the public-checkability sanity test but not the deployment target,
because it reveals the witness instead of proving it in a hiding way.  The next
construction has to replace those 34 witness bytes with a compact witness-hiding
opening/proof, or show a different commitment representation that avoids this
relation entirely.

The byte budgets now pinned in Lean and the executable report are:

```text
proof <= 75 B   -> total <= 256 B
proof <= 87 B   -> total < 269 B completed-response verifier
proof <= 88 B   -> total <= 269 B completed-response verifier
proof <= 827 B  -> total < 1009 B stateless SHA-digest mode
```

So the practical north-star is no longer exactly 177 B.  A deployable
stateful construction in the 200-260 B range would still be extremely
interesting; a construction above 269 B has to justify itself against the
already implemented completed-response route; and a construction near 1009 B is
probably only useful as a proof fallback.  Since a generic zero-knowledge proof
of SHA-256 plus PRF expansion is unlikely to fit below the 90-byte competitive
budget, the highest-value design move is to replace the hash-preimage opening
with an algebraic, witness-hiding opening relation.

The current concrete algebraic target is:

```text
LocalDeltaByteCommitment.N32Q12289.LinearSigmaOpeningTarget

LocalDeltaByteCommitment.N32Q12289.linearSigmaTargetProofBytes_eq
  -- 66 B = 32 B proof commitment + 34 B masked witness response

LocalDeltaByteCommitment.N32Q12289.linearSigmaTargetTransactionSignatureBytes_eq
  -- 247 B total
```

This is a design target, not a completed construction.  It says what kind of
primitive would be small enough: an algebraic account commitment where the
opening proof can be compressed to roughly one commitment element plus one
masked response over the 34-byte hidden opening witness.  If a concrete
post-quantum instantiation needs substantially more than that, it should be
compared directly against the 269-byte completed-response route.

The target is now a formal byte-shape, not just prose:

```text
LocalDeltaByteCommitment.N32Q12289.AlgebraicLinearSigmaOpeningConfig
LocalDeltaByteCommitment.N32Q12289.AlgebraicLinearSigmaOpeningConfig.toLinearSigmaOpeningTarget
LocalDeltaByteCommitment.N32Q12289.LinearSigmaProofFields
```

The executable fixture has the matching parser/encoder for:

```text
proof_commitment[32] || masked_witness_response[34]
```

The selected primitive shape is:

```text
LocalDeltaByteCommitment.N32Q12289.CompressedLinearSigmaAlgebra

verify: L_statement(z) = a + challenge * target(statement)
```

Here `L_statement` is the concrete compressed linear relation map to be chosen,
`a` is the proof commitment, and `z` is the masked witness response.  This is
the smallest plausible algebraic path because the proof commitment is the same
size as the account commitment, while the response is exactly the hidden
opening witness width.

The current instantiation route restricts the witness domain:

```text
LocalDeltaByteCommitment.N32Q12289.SeedRestrictedCompressedLinearSigmaAlgebra

hidden_preimage = derive_fiber_state(seed)[2] || seed[32]
```

This removes the information-theoretic 34-to-32 binding obstruction by making
the committed entropy 32 bytes.  The 34-byte proof response is retained for the
opening relation, but the two fiber-state bytes are deterministic account
structure, not extra independent preimage entropy.  The remaining concrete
assumption is computational binding/knowledge soundness for the seed relation
map `L_statement(seed)`.

The SIS-shaped version is now explicit in Lean:

```text
LocalDeltaByteCommitment.N32Q12289.SISStyleSeedOpeningMap

target(statement) = statement.digest
verify: L_statement(z) = a + challenge * statement.digest
```

This pins the deployable 247-byte target to two assumptions instead of an
opaque "small ZK proof" placeholder.  `relationMapSound` says that hitting the
digest through `L_statement(seed)` implies the byte-level opening relation, and
`sisKnowledgeSoundness` says that an accepting sigma equation extracts a seed
hitting that digest.  The next concrete step is to choose the matrix/domain
parameters that make these two assumptions credible for 32-byte seeds.

The first byte-level q-ary target is fixed as:

```text
sisSeedResidues = 18
sisSyndromeResidues = 18
sisSeedPackedBits = 252
sisResidueBits = 14
sisSyndromePackedBits = 252
sisSeedPackedBytes = 32
sisSyndromePackedBytes = 32
sisSeedSlackBits = 4
sisSyndromeSlackBits = 4
```

So a 32-byte masked seed response can carry 18 q-residues, and a 32-byte account
digest/proof commitment can carry an 18-coordinate q-ary syndrome for
`q = 12289`.

The reduction object is:

```text
LocalDeltaByteCommitment.N32Q12289.PackedSISSeedOpeningReduction
```

It compiles the concrete gate for a matrix/domain choice.  A candidate must
provide canonical packed seed validity, canonical digest validity, byte-level
linearity for the actual signature fields, relation-map soundness, and an
`extracts_or_break` theorem whose break is the selected SIS/algebraic binding
assumption.  With `noAlgebraicBindingBreak`, Lean converts the object to
`SISStyleSeedOpeningMap` and therefore to the 247-byte
`LinearSigmaOpeningTarget`.  The digest-validity condition matters because
18 q-residues inject into 32 bytes but do not cover all 32-byte strings.
The Rust fixture pins the first matrix-generation domain as
`Descender.Signature.q12289.local_delta.sis_matrix.v1` and samples matrix
entries by SHA-256 counter expansion with rejection modulo `12289`.
The generated `18 x 18` matrix is full rank (`det = 11243` mod `12289`), so the
bare public linear map is invertible.  This means the square packed-SIS
instantiation has the wrong security shape: it gives extraction, but public
linear algebra also recovers the seed.  Lean records the abstract obstruction as
`NontrivialSeedFiber`, `noNontrivialSeedFiber_of_injective`, and
`PublicSeedRecoverer`.

The natural rectangular repair is also bounded now.  If `r` public q-syndrome
rows are carried in the same 32-byte account field, then the public side has
`14r` packed bits and the hidden fiber has at most `14(18-r)+4` packed bits.
Lean proves `noBalanced128BitFiberSplit`, so this family cannot simultaneously
reach `128` public bits and `128` hidden-fiber bits.  The balanced `r=9` point
is `126/130`; the `r=10` point is `140/116`.  This rules out a pure rectangular
linear fiber as the final 247-byte construction.

The Pedersen-style linear blinding repair is also ruled out at this byte size.
`noPedersenStyle128BitSplit` says that if the response's 18 q-residues are split
between message seed and blinding, then `>=128` message bits plus `>=128` public
bits plus enough blinding dimension for perfect hiding cannot all fit.

The viable no-extra-byte variant is to reclaim the two fiber-state bytes in the
masked response.  The full `34` bytes carry `19` q-residues with `6` slack bits.
At `10` public rows, this gives `140` public bits and `132` hidden-fiber bits,
and the deterministic `10 x 19` matrix prefix has rank `10`.  Lean records the
capacity theorem as `fullResponseFiber_publicRows10_meets128`.  This keeps the
same `66` proof bytes but requires a full-response relation, not the earlier
seed-only relation.  The corresponding proof interface is now
`FullResponseFiberedLinearSigmaAlgebra`, whose extractor returns a full 34-byte
response vector and whose soundness obligation requires every accepted extracted
fiber vector to imply the byte opening relation.

The relation bridge is now pinned down by `FullResponseByteOpeningAlgebra`.
It converts the accepted extracted response with
`hiddenPreimageFromLinearSigmaWitnessBytes` and sets the public opening target
to the byte-level `idealRelation` for that decoded witness.  Thus the next
obligation is not to reinterpret the proof interface; it is to instantiate the
actual q-ary response map and prove that its accepting fiber implies the byte
opening relation.

The q-ary response-map gate now exists as
`PackedSISFullResponseOpeningReduction`.  It fixes the candidate at `19`
response residues, `10` public syndrome rows, a canonical `10`-residue syndrome
inside the 32-byte digest/commitment field, and the same 66-byte sigma proof
layout.  The remaining question is now entirely concrete: whether the selected
matrix/domain and packing can satisfy the relation-map soundness and
extraction-or-binding-break fields without losing witness hiding.

The first attempt to close that concrete gate gives a negative but useful
answer for the current byte relation.  Lean proves the necessary saturation
condition:

```text
FullResponseByteOpeningAlgebra.acceptedFiber_relationSaturated
FullResponseByteOpeningAlgebra.acceptedFiber_subsingleton_of_commitmentBinding
```

So every response in an accepted public fiber must be a valid decoded opening;
but if the opening relation binds all 34 hidden-preimage bytes with an
injective commitment, accepted fibers must be singletons.  The Rust report
confirms that the selected `10 x 19` matrix is not singleton-binding: it has
rank `10`, kernel dimension `9`, and a sampled nonzero kernel vector with zero
public syndrome.  This means the no-extra-byte full-response path cannot keep
the current “commit every hidden-preimage byte by SHA” relation.  It must use a
fiber-saturated quotient witness, or spend more public bytes/rows.

That quotient witness path is now formalized.  `FullResponseQuotientRelation`
defines the semantic opening relation over the public syndrome quotient, not
over the individual 34-byte representative.  `PackedSISFullResponseQuotientOpeningReduction`
then packages the same 10x19 q-ary map, extraction-or-break assumption, and
witness-hiding assumption into the existing 247-byte target.  The remaining
design problem is therefore sharply stated: find a public
`quotientOpensStatement` that captures the needed local-delta verifier
semantics using only the 10-residue quotient, while the 9-residue kernel remains
independent hiding randomness.

The bridge back to the old local-opening predicate is
`FullResponseExistentialQuotientRelation`: a quotient is valid when there exists
at least one representative in that fiber satisfying the representative-level
opening relation.  This is the mathematically right weakening for the 10x19
map: extraction lands in the quotient class, while the hidden kernel coordinate
is not itself a semantic object that must be committed.

The representative predicate is no longer abstract.  Lean now has
`ByteLocalDeltaRepresentativeSemantics`, which fixes the concrete N32/q12289
local-delta byte equations: active responses decode from the 52-byte response
field, local openings decode from the 26 four-byte opening fields, the active
mask view is reconstructed from the response, challenge coefficient, and public
key view, and the representative must satisfy both the local state equation and
the padded-opening equation.  The concrete instantiation is
`concreteByteLocalDeltaRepresentativeSemantics`: it uses the packed 17-byte
algebraic challenge plus 4 transcript-hardening bytes, the 172-byte public-key
packed14 free-coordinate view, and the
`PadPRFBinding.expandPad` pad source.  `ByteLocalDeltaRustLayout` records the
matching Rust byte offsets for response centering, public-key free-coordinate
packing, packed-q canonicality, and the pad frame.

The quotient target now has the honest existence bridge it needs:
`HonestLocalDeltaQuotientStatement` packages a representative response together
with its digest-syndrome equality and local-delta opening proof, and
`targetRepresentativeExists_of_honestStatements` turns a family of those objects
into the `targetRepresentativeExists` field required by
`FullResponseExistentialQuotientRelation`.  The next proof obligation is no
longer to invent the relation; it is to prove that the actual stateful account
generator emits these honest statement witnesses for its concrete SHA-256 pad
source and q-syndrome digest encoding.

This is now partly closed.  Lean adds
`ConcreteHonestLocalDeltaSigningTrace`, proving that the serialized local
equations imply the concrete representative predicate, and
`ConcreteHonestFullResponseQuotientSigningTrace`, proving that a packed
q-syndrome digest equality yields `HonestLocalDeltaQuotientStatement` and
`targetRepresentativeExists_of_traces`.  The Rust reference now contains the
matching q-syndrome digest builder: decode the 34-byte hidden preimage as 19
canonical 14-bit q-residues, apply the 10x19 full-response matrix, and pack the
10-residue syndrome into the 32-byte digest field.  Its quotient fixture test
checks the state-local equations against that digest path.

The implementation cleanup is now done for the reference path.  The verifier
has a production quotient wrapper,
`verify_quotient_stateful_signature`, that checks the packed q-syndrome digest
against the hidden preimage and then runs the same local byte equations.  The
legacy SHA-commitment verifier remains only as a comparison artifact, with a
negative test showing it rejects quotient accounts.  Lean mirrors this bridge
with `statefulVerifier_quotientIdealWitness_of_accepts` and
`statefulVerifier_targetQuotientOpens_of_accepts`, so an accepting stateful
public-opening verifier using the quotient linear-sigma target is connected
directly to the fiber-saturated quotient relation.

The concrete proof surface is now packaged as reviewable objects.  Lean's
`FullResponseCodec` names the deployed full-response q-syndrome decoder and
digest-match predicate, while `FullResponseMatrixSamplerAgreement` isolates the
byte-for-byte SHA-256 rejection-sampling agreement for the 10x19 matrix.
`ConcreteFullResponseQuotientReductionAssumptions` then fixes the exact
quotient target used by the executable verifier: matrix agreement, canonical
packed digest checks, honest representative existence, extraction-or-break,
witness hiding, and the 247-byte total.  The pad source has the analogous
wrapper `SHA256RejectionPadPRFAssumption`, which fixes the concrete pad frame
and sampler agreement while leaving SHA-256 PRF security as an explicit
assumption.  The additional `FullResponseQuotientAlgebraCertificate` records
the proof-relevant quotient facts: the 10x19 map is surjective onto the
10-residue public quotient, and the advertised hidden-fiber vectors are kernel
vectors.  `FullResponseDeploymentReviewCertificate` bundles these pieces with
the Fiat-Shamir frame agreement into one object.  Lean now also names the
packed-sigma extraction failure event explicitly as
`ConcreteFullResponseQuotientReductionAssumptions.extractionFailure` and proves
`no_extractionFailure` from `extracts_or_break` plus
`noAlgebraicBindingBreak`.

The Rust reference emits those same boundaries in machine-checkable report
fields.  `assumption_review_targets` lists the five remaining deployment gates,
and `assumption_closure_report` separates executable closure artifacts from
residual cryptographic assumptions.  The emitted artifacts now include a matrix
sampler certificate, quotient algebra certificate, pad sampler certificate, and
Fiat-Shamir frame certificate.  `blockchain_signature_size_comparison` records
the leading blockchain target as `172 B` public keys and `247 B` transaction
signatures including slot id and the 66-byte quotient proof.  Comparator rows
include source URLs for FIPS 204, FIPS 205, the Falcon specification, and RFC
8032.  This is the practical Pareto point we now have to either cryptanalyze
into a deployable assumption set or abandon in favor of a larger but
easier-to-review verifier architecture.

The current proof-composition endpoint is
`FullResponseDeploymentReviewCertificate.statefulVerifier_conditionalSoundness`.
It is conditional, but it is no longer an informal checklist: an accepting
stateful verifier under the review certificate yields the target quotient
opening, active equation, slot non-reuse, 247-byte accounting, and the exact
named assumption facts consumed by the reduction.

The named facts are now game-shaped:

```text
PackedFullResponseExtractionGame.noExtractionBreak
QuotientWitnessHidingGame.noWitnessHidingBreak
SHA256PadPRFGame.noPadPRFBreak
FiatShamirFrameAgreementGame.noFrameMismatchBreak
```

The executable report mirrors these names in `assumption_closure_report`, so
the next cryptanalysis pass can attach concrete adversary resources and
advantage bounds to each game without changing the verifier-composition
theorem.

The 128-bit budget gate is now part of the formal surface as well:
`GameSecurityBudget` records adversary query count, classical security bits,
and quantum security bits.  Each game must provide `budget_meets128`, and the
top-level conditional theorem returns those budget facts alongside the no-break
facts.  The executable report exposes the same target and records where each
query-bound analysis still needs to be filled in.

The first concrete generic-query budget table is now emitted as
`concrete_security_budgets`.  It makes the tradeoff visible:

```text
extraction quotient target: 140 raw bits -> 2^6 quantum-query margin
hidden quotient fiber:      132 raw bits -> 2^2 quantum-query margin
SHA-256 pad source:         256 raw bits -> 2^64 quantum-query margin
hardened FS challenge:      162 transcript-bound bits -> 2^17 quantum / 2^34 classical margin
```

The previous 17-byte challenge was the dominant review bottleneck for
multi-query security margins.  The current target spends 4 bytes on a
transcript-hardening suffix, keeping the 130-bit algebraic challenge unchanged
while making the Fiat-Shamir budget much easier to sell.

The important caveat is also formal:

```text
LocalDeltaByteCommitment.N32Q12289.noInformationTheoreticBinding_34_to_32
```

A 32-byte account commitment cannot be injective over all 34-byte hidden
preimages.  Therefore the algebraic sigma branch must be computationally
binding, restricted-domain, or backed by additional account-state bytes.  This
rules out any claim that the 247-byte target is obtained merely by replacing
SHA-256 with a transparent linear map.

## Pareto Frontier By Verifier Architecture

The current theoretical frontier looks like this:

| Architecture | Commitment binding source | Size shape | Status |
|---|---|---:|---|
| Active-only stateless | none / circular | `69 B` | not Fiat-Shamir signable |
| Explicit commitments | carried full commitment | `3709 B` | implemented baseline |
| Completed response, `w=1,r=26` | reconstruct full commitment | `745 B` | implemented default |
| Completed response, high weight | reconstruct full commitment | about `269 B` | implemented in current verifier family |
| Stateful precommitment | commitment stored in state | spend can approach `69 B` | architecture open |
| Stateful quotient opening | q-syndrome account digest + 66 B proof | `247 B` tx | leading candidate, assumption-parametric |
| Transparent information-set mask code | active view recovers mask/codeword | could approach `69 B` | rejected: leaks active secret |
| Fibered information-set commitment code | active view reconstructs commitment state only | could approach `69 B` | viable target, new construction required |

## Interpretation

There are three plausible directions:

1. **Best stateless conservative path.** Use completed-response implicit
   reconstruction.  This gives complete Fiat-Shamir binding without a new
   commitment primitive.  The high-weight version is likely the byte-size
   leader here.

2. **Best stateful blockchain path.** Publish or amortize commitments in
   account state, then spend with the active-only core.  This can approach
   `69 B` per spend, but it changes wallet/state semantics and must prevent
   commitment reuse.

3. **Best speculative stateless path.** Build a fibered information-set
   commitment code so every active challenge projection determines the full
   prechallenge commitment state, while the sampled mask remains hidden inside
   a high-entropy fiber.  The dimension lower bound says the reconstructed
   per-round commitment state has at most two independent q-symbols at `w=1`,
   not that the signer must use only two q-symbols of mask entropy.

## Next Theoretical Tests

Before implementing more variants, the useful mathematical tests are:

1. **Done.** Prove a product-block specialization of the Lean cardinality theorem:

```text
active-only reconstruction for weight w
=> allowed mask-code cardinality <= q^(w * local_free)
```

2. **Done.** Instantiate the `w=1` consequence:

```text
allowed per-round code cardinality <= q^2
```

3. **Done.** Separate the commitment-state lower bound from the mask-state
   lower bound:

```text
exact reconstruction bounds CommitState;
it bounds MaskState only if stateOfMask is injective
```

4. Analyze whether a `k<=2` commitment-state code can be implemented with
   high-entropy mask fibers and a certified sampler.

5. Separately model a stateful precommitment architecture where the active
   spend signature is `69 B` but the account state carries the commitment root
   or commitment queue.

The important conclusion is that the `69 B` stateless target is not blocked by
an implementation detail.  It requires one of two structural changes:
externalized commitment state, or a low-dimensional reconstructed commitment
state with high-entropy mask fibers.
