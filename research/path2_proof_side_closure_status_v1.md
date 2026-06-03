# Path 2 Proof-Side Closure Status V1

Status: proof-side closure assessment after native boundary refinement.

Purpose: separate:

- high-risk theorem/design blockers;
- pinned theorem-interface obligations;
- remaining concrete instantiation work;
- implementation-facing serializer/KAT gates.

## Current Verdict

No unresolved hidden-witness extraction blocker remains in the active native
Path 2 proof shape.

The active theorem is no longer:

```text
accepted account creation extracts a full hidden witness
```

It is:

```text
public account admission gives a coherent native boundary q
honest wallet/keygen creates q with retained private h
accepted spends are semantic actions over meaning_of_boundary(q)
public views export no private fiber material
```

The remaining work is concrete instantiation and implementation connection, not
a known proof-design contradiction.

## Closed High-Risk Proof Blockers

### Public/Private Lift Split

Closed at theorem-interface level.

Consensus admission proves:

```text
AcceptCreate(c,q) => obs_c(q)=0 ∧ public_boundary_sound_c(q)
```

Honest creation proves:

```text
HonestCreate(seed)=Some(q,h) => LocalValid_c(q,h)
```

This avoids requiring consensus to prove `obs_c(q)=0 => H_c(q) nonempty` for
adversarially submitted, possibly unspendable accounts.

### Authorization Class Binding

Closed at theorem-interface level.

The active rule is:

```text
auth_compatible_c(q,auth) => AuthClass_c(auth)=p_spend(q)
```

and `auth_compatible_c` is typed over the public class.

### Spend Projection Completeness

Closed after strengthening `p_spend(q)`.

Added or edge-linked:

```text
allowed_state_transition_class
aggregation_mode
```

### `O_gluing` Concrete Edge Encoding

Closed at first-pass concrete-check level.

Pinned public checks:

```text
E_SD
E_ST
context / pullback equality
```

### `p_state(q)` Canonicality

Closed at theorem-interface level.

`state_transition_class` is the canonical class of the full state boundary plus
context, not merely `account_state_root_before`.

### Context-State Domain Binding

Closed at theorem-interface level.

State transition classes bind:

```text
version
parameter set
verifier family
spend domain
chain/app domain
fork/epoch scope
aggregation mode
```

### Checked-Core Frame Binding

Closed at proof-spec level, not implementation-complete.

`Path2NativeCheckedCoreFrameV1` now defines the frame required by the proof.
The old fallback checked-core response transcript frame is classified as proof
evidence, not the active native challenge frame.

### Fiber Privacy / No-Export

Closed at policy/spec interface level.

The active native public output allowlist and private denylist are pinned.

## Remaining Concrete Instantiation Gates

These are not hidden proof-design blockers, but they must be completed before
claiming deployability.

### 1. Concrete Encoder Specs

Need exact implementation specs for:

```text
canonical q bytes
p_context digest/preimage
p_account digest/preimage
p_spend digest/preimage
p_lifecycle digest/preimage
p_transcript digest/preimage
p_state digest/preimage
Path2NativeCheckedCoreFrameV1 encoder
PublicNativeAccountViewV1 serializer
PublicNativeSpendViewV1 serializer
```

### 2. Hash / Domain Binding Assumptions

Need updated assumption ledger for:

```text
native_boundary_digest
projection digests
StateTransitionClass_c(q)
Path2NativeCheckedCoreFrameV1
public output policy id
```

These are ordinary binding/domain assumptions, not hidden-witness proof
assumptions.

### 3. Checked-Core Implementation Connection

Need either:

```text
checked-core challenge = H(Path2NativeCheckedCoreFrameV1)
```

or:

```text
legacy checked-core challenge includes H(Path2NativeCheckedCoreFrameV1)
```

with no legacy semantic handle.

### 4. Public-View No-Export KATs

Need tests that scan accepted and rejected public outputs for the denylist:

```text
authorization local
opening tuple
pad seed / pad lanes
projection lanes
gauge/randomizer
private helper values
raw hidden query openings
```

### 5. Residual Assumption Ledger Refresh

`research/path2_residual_assumption_ledger_v1.md` is stale. It still lists
lift nonemptiness, authorization class binding, and fiber privacy as open in the
old form.

It should be updated to distinguish:

```text
theorem-interface closed
concrete encoding pending
implementation/KAT pending
external review pending
```

## Current Size Status

Preferred active target remains:

```text
account creation public bytes: 244-392
account state bytes:           145-320
signer private state bytes:    128-416
spend signature impact:        0-32
```

The +32-byte stress fallback activates only if:

```text
StateTransitionClass_c(q)
```

cannot be derived from `p_state(q)` plus `p_context(q)`.

Current proof work supports keeping that +32 bytes as fallback only.

## Highest-Risk Remaining Item

The highest-risk remaining internal item is implementation connection for:

```text
Path2NativeCheckedCoreFrameV1
```

Reason:

The proof says checked-core binds native projections, but the visible concrete
helper today is a fallback checked-core response transcript. If implementation
work starts without adding the native frame, checked-core could still accept
under an old independent slot authorization path.

This is implementation-facing but security-critical.

## Recommended Next Step

Refresh:

```text
research/path2_residual_assumption_ledger_v1.md
```

so it no longer reports stale proof blockers and instead names the active
remaining gates.

Then begin implementation-facing specs/KATs in this order:

1. `Path2NativeCheckedCoreFrameV1` encoder and mutation KAT plan.
2. `PublicNativeAccountViewV1` and `PublicNativeSpendViewV1` serializer
   allowlist KAT plan.
3. Projection digest/preimage encoder specs.
4. Residual hash/domain/QROM assumption ledger refresh.

## Do Not Claim Yet

Do not claim deployable security signature yet.

The proof design is now coherent enough to proceed, but deployability still
requires:

- concrete encoders;
- checked-core frame implementation binding;
- no-export serializer tests;
- domain/hash assumption ledger update;
- implementation preflight;
- external review, later.
