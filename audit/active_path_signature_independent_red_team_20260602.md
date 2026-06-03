# Active Path Signature Independent Red-Team Audit

Date: 2026-06-02

Scope: independent Lean-only audit of the active Path 2 native / reviewed-Fq12
signature proof surface. This pass looked directly at theorem statements,
definitions, and proof assumptions, not at tracker status.

## Verdict

I do not see a closed EUF-CMA proof for the active path signature scheme. The
Lean modules compile, but the active security claim is currently a boundary
composition theorem whose decisive cryptographic and implementation facts are
supplied as external propositions.

No direct algebraic forgery was found in this pass. The strongest red-team
conclusion is that the proof is not cryptanalytically closed: a successful
attack should focus on the unpriced full-game bridge, byte-frame/domain
injectivity, and the reviewed-Fq12 role-root/full-opening binding assumptions.
The original QROM/multi-target margin finding below was real at the time of the
audit, but has since been superseded by the widened auxiliary-Fq12 challenge
route.

## High-Risk Findings

### APS-RT-001: Final Full-Signature Theorem Is a Boundary Composition

`Descender/Path2NativeAccountDesign.lean` defines
`NativeFullSignatureSecurityBoundary` with arbitrary real-valued success
probabilities plus proposition fields for keygen, signing oracle, completeness,
implementation matching, and the full-forgery-to-accepted-spend reduction.

The final theorem:

- `NativeFullSignatureSecurityBoundary.fullSignatureSuccessProb_le_activePathForgeryBound`

only chains:

- `signatureForgerySuccess_le_acceptedSpend`
- `acceptedSpendSoundness.acceptedSpendForgery_le_bound`

Reference:

- `Descender/Path2NativeAccountDesign.lean:1547`
- `Descender/Path2NativeAccountDesign.lean:1565`
- `Descender/Path2NativeAccountDesign.lean:1607`

Red-team impact: the theorem is only as strong as the supplied boundary record.
It does not independently model EUF-CMA, key generation, signing oracle behavior,
or the real verifier byte relation.

### APS-RT-002: Concrete Full-Forgery Lift Is Tautological Without a Byte Bridge

In `Descender/Path2FullSpendConcrete.lean`,
`ConcreteSpendVerificationAccepts` already includes:

- bound prevalidated slot facts;
- admitted prechallenge leaf facts;
- checked-core tuple binding;
- consume-once accepted transition.

Then `ConcreteFullSignatureForgerySucceeds` is defined as exactly
`ConcreteSpendVerificationAccepts C run.spend`.

References:

- `Descender/Path2FullSpendConcrete.lean:322`
- `Descender/Path2FullSpendConcrete.lean:350`
- `Descender/Path2FullSpendConcrete.lean:395`
- `Descender/Path2FullSpendConcrete.lean:417`

Red-team impact: the concrete full-game lift is a useful carrier theorem, but it
does not prove that an arbitrary valid production signature implies those facts.
That bridge remains the actual security work.

### APS-RT-003: QROM and Multi-Target Margin Was Initially Extremely Tight

Status update: superseded by the active widened auxiliary-Fq12 challenge route.
The active Lean/Rust constants now carry 23 algebraic challenge bytes plus
four hardening bytes, 26 repetitions, 182 transcript bits after QROM loss, and
53 deployment-margin bits.  The active multi-target certificate now prices a
32-bit aggregate horizon and keeps a 54-bit aggregate horizon as the
fail-closed negative control.

The active direct budget pins:

- target security: 128 bits;
- deployment quantum hash query bound log2: 16;
- QROM quadratic loss: 32 bits;
- transcript term after QROM loss: 130 bits;
- minimum deployment margin: 1 bit.

The active multi-target certificate spends that one bit on:

- `accountHorizonLog2 := 1`
- `spendHorizonLog2 := 0`
- `messageHorizonLog2 := 0`
- `aggregateTargetCountLog2 := 1`

References:

- `Descender/Path2HashBudget.lean:36`
- `Descender/Path2HashBudget.lean:78`
- `Descender/Path2HashDomainAndMultiTarget.lean:111`
- `Descender/Path2HashDomainAndMultiTarget.lean:139`
- `Descender/Path2HashBudget.lean:41`
- `Descender/Path2HashDomainAndMultiTarget.lean:202`

Residual red-team impact: the old one-bit horizon no longer blocks the active
path. The remaining risk is closure quality: the final theorem must still
connect the widened challenge bytes and multi-target ledger to the exact
executable verifier, stateful slot model, and deployment query budget.

### APS-RT-004: Domain Separation Is Numeric Tag Inequality Only

The active hash-domain proof checks that the numeric tags 0, 1, 2, and 3 are
distinct. It does not prove production byte-frame injectivity, prefix-freeness,
length canonicality, parser agreement, or collision resistance of the underlying
SHA/QROM input encoding.

References:

- `Descender/Path2HashDomainAndMultiTarget.lean:24`
- `Descender/Path2HashDomainAndMultiTarget.lean:45`
- `Descender/Path2HashDomainAndMultiTarget.lean:75`

Red-team impact: a byte-level ambiguity or parser/frame mismatch could bypass
the numeric tag proof. The active proof needs a byte-frame theorem tying the
tag registry to the actual hashed inputs.

### APS-RT-005: Hidden-Source Reviewed-Fq12 Extraction Remains an Assumption

The central P2FS-001 hidden-source statement is still a supplied proposition:

- `ReviewedFq12HiddenSourcePayloadSoundness`

The proof decomposes the assumption into payload projection, extraction
certificate availability, payload binding, certificate soundness, selected
certificate role lift, role-root binding, assembly, native validity, and
compatibility. That decomposition is valuable, but the main extractor facts are
still provided by certificate fields.

References:

- `Descender/Path2NativeBoundary.lean:1456`
- `Descender/Path2NativeBoundary.lean:1490`
- `Descender/Path2NativeBoundary.lean:1511`
- `Descender/Path2NativeBoundary.lean:1607`
- `Descender/Path2FullOpeningSourceBridge.lean:157`
- `Descender/Path2CompatibilitySoundness.lean:49`

Red-team impact: the likely attack target is malleating a valid reviewed-Fq12
payload/certificate chain so that selected active-lane evidence remains valid
while unselected native roles, full-opening coordinates, or representative
compatibility are swapped.

### APS-RT-006: Role-Root Binding Is Isolated but Not Proven Cryptographically

`Path2RoleRootBinding.lean` cleanly isolates two residual claims:

- accepted creation/payload binding pins one native role root;
- that native role root pins every full-native-opening role coordinate.

`Path2RoleRootConcrete.lean` lowers these to pairwise field-equality
certificates, but those field-equality sources are still assumptions about the
production encoding/commitment relation.

References:

- `Descender/Path2RoleRootBinding.lean:77`
- `Descender/Path2RoleRootBinding.lean:110`
- `Descender/Path2RoleRootBinding.lean:149`
- `Descender/Path2RoleRootConcrete.lean:38`
- `Descender/Path2RoleRootConcrete.lean:141`

Red-team impact: unless the production role root is proven binding for all
roles, an attacker should try alternate full-opening tuples with the same
selected payload/extraction surface.

### APS-RT-007: Byte Parser and Checked-Core Bridge Are Certificate Hooks

`Path2CheckedCoreByteBridge.lean` states parser/canonicality/frame/hash/syndrome
certificates that imply the native checked-core accept relation. The parser and
hash functions themselves remain abstract hooks.

References:

- `Descender/Path2CheckedCoreByteBridge.lean:28`
- `Descender/Path2CheckedCoreByteBridge.lean:127`
- `Descender/Path2CheckedCoreByteBridge.lean:183`
- `Descender/Path2CheckedCoreByteBridge.lean:217`

Red-team impact: malformed proof bytes, alternate encodings, noncanonical
lengths, decoded commitment mismatches, or frame/hash disagreement remain
production-critical until the Rust parser/hash implementation is proven to
instantiate the Lean certificate.

### APS-RT-008: Named Break Events Are Not Yet Priced in the Final Bound

Many active theorems are `or break` statements. The final accepted-spend layer
contains an arbitrary `acceptedSpendForgeryProb`, arbitrary
`acceptedSpendForgeryBound`, and a supplied inequality between them.

References:

- `Descender/Path2NativeAccountDesign.lean:1547`
- `Descender/Path2FullSpendGame.lean:155`
- `Descender/Path2FullSpendGame.lean:182`
- `Descender/Path2FullSpendGame.lean:211`

Red-team impact: the Lean proof names the failure modes but does not yet give a
global probability game, union bound, or reduction that makes each break
negligible under deployment parameters.

## Most Plausible Attack Directions

1. Byte-frame/domain confusion: find two production byte encodings that produce
   equal hashed frames while Lean only knows distinct numeric tags.
2. Reviewed-Fq12 role swap: keep selected active-lane payload evidence valid
   while changing unselected native role digests or full-opening coordinates.
3. Multi-target budget overflow: instantiate a realistic deployment horizon
   whose aggregate target count exceeds the one-bit active budget.
4. EUF-CMA bridge failure: produce a verifier-accepted signature that does not
   map to `ConcreteSpendVerificationAccepts` as currently defined.
5. Break-event laundering: force a named break event that is accepted in the
   local theorem but not priced in the final probability statement.

## Verification Performed

The following active proof files compiled during this audit:

- `lake env lean Descender/Path2NativeAccountDesign.lean`
- `lake env lean Descender/Path2FullSpendGame.lean`
- `lake env lean Descender/Path2FullSpendConcrete.lean`
- `lake env lean Descender/Path2HashDomainAndMultiTarget.lean`
- `lake env lean Descender/Path2NativeBoundary.lean`
- `lake env lean Descender/Path2RoleRootBinding.lean`
- `lake env lean Descender/Path2FullOpeningSourceBridge.lean`
- `lake env lean Descender/Path2CheckedCoreByteBridge.lean`

Compilation confirms the boundary framework is internally consistent. It does
not close the cryptanalytic assumptions listed above.
