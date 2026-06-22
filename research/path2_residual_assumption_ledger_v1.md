# Path 2 Residual Assumption Ledger V1

Status: ACTIVE_SUPPORT; selected-route residual authority is now
`../spec/security_model.md` section `Selected Account-STARK Residual Matrix`.

Purpose: separate active native theorem obligations from ordinary hash/domain
assumptions, checked-core downstream assumptions, implementation/KAT gates, and
fallback proof-system assumptions.

This ledger supersedes the earlier version that still treated full lift
nonemptiness and hidden-witness extraction as the active blocker. After the
Poseidon2/Plonky3 account-STARK selection, the native material below is support
history unless mapped into the selected account-STARK matrix in
`spec/security_model.md` or re-selected by ADR.

## Selected Account-STARK Residuals

The active deployable account-creation proof family is
`Path2AccountStarkCreationPayloadV1` plus
`verify_path2_direct_creation_account_stark_payload_boundary`.

Normative residual items live in `spec/security_model.md` and currently are:

| residual item | deployment status |
|---|---|
| payload byte and replay binding | implemented with initial mutation tests; frozen corpus still open |
| AIR relation soundness | internal audit says relation classes are wired; external proof-family review still open |
| FRI argument-of-knowledge extractor | open |
| zero-knowledge simulator and no-export statement | open |
| FRI soundness margin and recursion ledger | open |
| production admission gate and review disposition | fail-closed |
| legacy proof-route disposition | transparent/native-boundary/descent routes are support only |

## Current Proof Shape

The historical native-boundary theorem shape below is support material, not the
selected account-STARK deployment theorem. It remains useful for downstream
checked-core, signer, and native carrier reasoning.

```text
AcceptCreate(c,q)
  =>
obs_c(q)=0 ∧ public_boundary_sound_c(q)
```

plus honest wallet/keygen spendability:

```text
HonestCreate(seed)=Some(q,h)
  =>
LocalValid_c(q,h)
∧ AuthClass_c(h)=p_spend(q)
```

and spend closure:

```text
SpendAccept(c,q,sig,msg,state)
  =>
accepted action is determined by meaning_of_boundary_c(q)
```

Public views must satisfy the active no-export policy.

## Active Native Structural Obligations

| obligation | role | current status | break |
|---|---|---|---|
| projection completeness | every consensus semantic field factors through `q` | interface pinned; concrete `q` bridge started in `research/path2_projection_completeness_bridge_v1.md` as projection bundle plus successful boundary assembly | `projection_factorization_break` |
| context well-formedness | version/parameter/verifier/domain/aggregation fields valid | spec and Lean interface pinned | `context_binding_break` |
| pullback consistency | context/spend/transcript/state faces agree | concrete equality checks pinned | `pullback_consistency_break` |
| `O_gluing` soundness | public faces glue to one semantic boundary | first concrete edge encoding pinned; checked obstruction component bridge started in `research/path2_checked_obstruction_zero_bridge_v1.md` | `gluing_obstruction_break` |
| meaning binding | glued public boundary realizes `meaning_of_boundary(q)` | interface pinned; concrete meaning reconstructed from projection bundle plus assembly in `research/path2_projection_completeness_bridge_v1.md`; private fiber bridge still pending | `meaning_binding_break` |
| spend-state closure | spend state class equals `p_state(q)` transition class | `E_SD` pinned; `p_state` canonicality pinned | `spend_state_closure_break` |
| `p_state` canonicality | state class is full state boundary plus context | spec and Lean interface pinned | `canonicality_break` |
| authorization class binding | compatible auth maps to `p_spend(q)` | typed theorem interface pinned | `authorization_class_binding_break` |
| honest creation spendability | honest creation retains valid private `h` | theorem interface and sampler spec pinned; private fiber bridge in `research/path2_private_fiber_meaning_bridge_v1.md` leaves productized honest constructor open; Rust scope `Path2NativeHonestConstructorScopeV1` pins required artifacts/harnesses; draft constructor `path2_native_honest_constructor_draft_v1` binds private authorization class to public `p_spend`; secret-material constructor `path2_native_honest_constructor_from_secret_material_v1` derives private-fiber digests, generates checked-core proof bytes from the same native frame derivation used by assembly, and rejects proof randomizer/public commitment mismatches; entropy-root bridge derives domain-separated secret material from an owned root and public binding; OS entropy constructor derives the public proof commitment itself and emits verifier-compatible fail-closed output; signer execution report exposes only public digests and sanitized rejection codes; `Path2NativeSignerKeygenProductizationScopeV1` pins remaining whole-signer constant-time and rejection-cleanup criteria | `authorization_local_sampling_break` |
| gauge/randomizer invariance | private choices change no public semantics | interface pinned; private fiber bridge and `Path2NativeHonestConstructorScopeV1` scope this to signer/keygen productization; draft KAT `path2_native_honest_constructor_same_q_gauge_randomizer_invariant` passes; secret-material KATs cover generated proof compatibility, seed-buffer and owned-buffer zeroization, and randomizer mismatch rejection | `gauge_invariance_break` |
| fiber privacy/no-export | public views reveal no private fiber material | allowlist/denylist spec and Lean policy pinned | `fiber_privacy_break` |
| spend closure over `q` | accepted spend action determined by projections | composition theorem interface pinned | `spend_closure_break` |

## Remaining Native Instantiation Gates

These are not unresolved hidden-witness proof blockers, but they are required
before deployability.

| gate | reason |
|---|---|
| canonical `q` encoder | hybrid 330-byte consensus carrier specified, implemented, and wired into verifier/preflight recomputation |
| projection preimage encoders | implemented for active native projections and wired into carrier preflight |
| `Path2NativeCheckedCoreFrameV1` encoder | implemented with layout, mutation, rejection, no-export KATs, and carrier preflight wiring |
| public-view serializers | implemented for active native account/spend views and validator-facing preflight report |
| mutation/KAT suite | active native projection/frame/view/assembly/carrier mutation KATs plus accepted/rejected verifier-output no-export scans exist |
| domain/schema registry KAT | active native and production-wide domain/schema/profile manifest uniqueness KATs exist |
| active native QROM/hash review | public transcript/domain/carrier hash assumptions are recorded with deployment query caps |
| implementation preflight | broad Rust/Lean/facade checks pass for the active proof-side surface |
| active native deployment preflight | code-generated internal readiness report names the remaining signer/verifier, constant-time, and storage/lifecycle blockers |
| active native verifier pipeline | fail-closed public verifier wrapper emits only public verdict/preflight data and rejects until productization blockers close |
| active native signer pipeline | fail-closed public signer-output wrapper emits carrier/proof digests and no private projection material |
| active native constant-time scope | executable audit scope names secret-dependent operations, public-length non-claims, and required artifacts; signer CT stage report pins the fixed public success/rejection report shape while leaving real CT audit open |
| active native CT evidence ledger | `Path2NativeSignerCtEvidenceLedgerV1` aggregates completed source evidence and pins current backend audit scripts plus the hash-addressed CT evidence bundle runner; private signer hashing now uses an in-tree fixed-workspace SHA-256 path with explicit workspace/schedule/state zeroization, caller-owned digest output before cleanup, no post-cleanup moves in the private SHA callee or digest-wrapper success epilogues, no upstream `Sha256::digest(frame.as_slice())` calls, no private-hash `memset`/`memcpy` calls in release assembly, an active-path CT-shape audit, an explicit register-residual audit that keeps GPR/SIMD register scrubbing open, a target/toolchain register-scrub policy gate, a target-host campaign wrapper that archives environment/timing/register-policy/bundle hashes, and a promotion gate that verifies archived campaign hashes plus timing thresholds while still refusing release CT promotion, while keeping the deployment CT blocker open for private SHA-256 target-CPU timing, target/toolchain register-scrub closure, whole-signer microarchitectural timing, and promoted target-host release evidence |

## Ordinary Hash / Domain Assumptions

These bind public bytes. They must not become hidden-witness proof semantics.

| assumption | role | break |
|---|---|---|
| canonical `q` digest binding | binds expanded native boundary | `byte_data_availability_break`, `domain_binding_or_qrom_break` |
| projection digest binding | binds projection preimages | `projection_factorization_break` |
| state transition class digest binding | binds `p_state(q)` plus full context | `spend_state_closure_break`, `domain_binding_or_qrom_break` |
| native checked-core frame binding | binds challenge preimage | `checked_core_frame_binding_break` |
| no-export policy id binding | binds serializer/policy version | `fiber_privacy_break`, `domain_binding_or_qrom_break` |
| ordered encoder bridge | fixed-width Rust encoders instantiate Lean ordered frames; digest equality separately charges hash/QROM binding | parser-injective surfaces and digest-bound surfaces classified in `research/path2_ordered_encoder_hash_binding_bridge_v1.md`; DA `append_frame` parser/KAT covered by `path2_native_projection_preimage_da_frames_roundtrip` |
| active native domain uniqueness | local and production-wide constant/schema collision checks | covered by `path2_active_native_domains_and_public_schemas_are_unique` and `production_domain_schema_registry_is_globally_disjoint` |
| cryptographic domain separation | disjoint RO/QROM domains under SHA-256 | `domain_binding_or_qrom_break` |

## Downstream Checked-Core Assumptions

Checked-core is allowed only as a downstream spend verifier over:

```text
Path2NativeCheckedCoreFrameV1(q,msg,state,commitment)
```

| assumption | role | status | break |
|---|---|---|---|
| checked-core equation soundness | accepted checked-core proof satisfies relation | downstream cryptographic theorem | `checked_core_frame_binding_break` |
| native frame binding | challenge preimage is native projection frame | spec, implementation, facade exports, KATs, and first Lean/Rust bridge contract present in `research/path2_checked_core_spend_reduction_bridge_v1.md`; concrete `q` representation still needs final naming | `checked_core_frame_binding_break` |
| message binding | challenge binds message digest | implemented in native frame KAT surface | `domain_binding_or_qrom_break` |
| lifecycle/state binding | challenge binds lifecycle and state projections | implemented in native frame/projection assembly KAT surface | `spend_state_closure_break` |
| proof commitment timing | commitment fixed before challenge | proof commitment is included in frame, mutation KATs, and carrier preflight wiring; end-to-end signer/verifier timing remains productization work | `checked_core_frame_binding_break` |

The old fallback checked-core response transcript frame is proof evidence only.
It is not the active native challenge frame.

Bridge obligation still open:

```text
encode_path2_native_checked_core_frame_v1(assembly.checked_core_frame)
  corresponds to
CheckedCoreSpendReduction.frameEncoder.encode [
  Proj.pSpend(q),
  Proj.pLifecycle(q),
  Proj.pTranscript(q),
  Proj.pState(q),
  msg,
  state,
  proofCommitment
]
```

The Rust frame additionally binds context and account digests. This is a
strengthening once the projection-completeness bridge identifies those fields
as projections of the same concrete `q`.

## Authorization-Local Assumptions

Signer-held private authorization is allowed only over the public class:

```text
p_spend(q)
```

| assumption | role | status | break |
|---|---|---|---|
| compatible auth sampling | honest signer creates compatible `auth` | sampler spec pinned; secret-material constructor now checks authorization class and proof-randomizer commitment compatibility; entropy-root derivation covers stream separation and public binding; OS CSPRNG root generation, fixed-attempt q sampling with deterministic fallback, no-runtime-modulo signer q reduction, q-sampler source-shape CT audit, and q-sampler release-assembly audit are wired; signer execution report is digest-only and rejection-sanitized; signer CT stage report pins the required fixed public schedule; whole-signer constant-time and rejection-cleanup audit remains pinned by `Path2NativeSignerKeygenProductizationScopeV1` | `authorization_local_sampling_break` |
| auth class binding | compatible auth has class `p_spend(q)` | theorem interface pinned | `authorization_class_binding_break` |
| auth hiding | public views reveal only public class | active native public-view serializers and accepted/rejected preflight no-export KATs present | `authorization_local_hiding_break` |
| auth lifecycle neutrality | auth cannot alter lifecycle/state semantics | follows from projection/spend closure interfaces | `spend_state_closure_break` |

## Fallback Proof-System Assumptions

Fallback assumptions are inactive unless a named native gate fails.

| fallback assumption | route | active? |
|---|---|---|
| transparent full-trace extraction | masked trace/STARK-like | no |
| transparent relation knowledge | masked trace/STARK-like | no |
| no-export/ZK simulator for trace openings | masked trace/STARK-like | no |
| hybrid base-128 range proof rows | masked trace/STARK-like | no |
| lattice/SIS sigma repetition soundness | parked lattice-sigma | no |
| MPC-in-the-head full witness soundness | fallback full-witness proof | no |

Fallback activation rule:

```text
activate fallback only after naming which native gate failed
and why no native public obstruction/encoding can replace it.
```

## Retired Or Demoted Paths

| old path/object | disposition |
|---|---|
| full hidden witness extraction at account creation | retired from active native theorem |
| public opening of pad seed/opening tuple | dead unless switching to public-opening model |
| independent opening/pad/carrier commitments plus equality proof | dead for compact native path |
| old 56-byte carrier as account creation proof | fallback/negative-control only |
| old checked-core response transcript as native challenge frame | not sufficient; proof evidence only |
| old 64-variable generalized projection relation | dead |
| transparent 8-16 KB proof envelope | fallback only, excluded from active byte target |

## Native Route Closure Criteria

Proof-side theorem interfaces are now coherent enough to proceed, and the
private-fiber bridge has moved from a pure scope relation to an executable
secret-material constructor/proof-generation harness. The remaining
signer/keygen production blocker is now executable scope data rather than only
prose.

Proof-side closure status:

1. internal proof-side preflight is complete for the active native public boundary;
2. remaining internal work is implementation/productization outside this proof-side slice:
   production signer/keygen entropy ownership, full-secret zeroization,
   storage/lifecycle hardening, and completion of the constant-time artifacts
   named by `Path2NativeConstantTimeAuditScopeV1`;
3. external review remains later and is excluded from the internal tracker.

## Current Decision

Keep masked-trace/STARK-like and lattice-sigma routes as fallback.

Do not include their proof envelopes in active native byte estimates.

Preferred active byte target remains:

```text
account creation public bytes: 330 active native carrier body
account state bytes:           145-320
signer private state bytes:    128-416
spend signature impact:        0-32
```

Stress fallback:

```text
account creation public bytes: 244-424
account state bytes:           145-352
```

only if `StateTransitionClass_c(q)` cannot be derived from existing `p_state`
and `p_context` fields.

## Next Active Move

Implementation-facing specs/KATs for the public proof-side slice are closed.
The private-fiber bridge now has an executable secret-material constructor and
checked-core proof generator, and `Path2NativeSignerKeygenProductizationScopeV1`
pins the remaining signer/keygen production criteria. The entropy-root bridge
now covers domain-separated stream derivation, public-binding sensitivity, OS
CSPRNG root generation, fixed-attempt q sampling with deterministic fallback,
no-runtime-modulo signer q reduction, and proof-commitment derivation inside
the constructor. The signer execution report now gives a digest-only,
rejection-sanitized public harness for this constructor surface. The signer CT
stage report now pins the fixed public success/rejection schedule, and the q
sampler no longer exports a sampler-exhaustion rejection, but it does not claim
branch, memory-access, comparison/mask lowering, SHA-256, timing, or cleanup
constant-time behavior. The q-sampler source-shape audit closes the current
source-level sampler comparison/mask/memory-access shape but leaves
backend/assembly and timing evidence open. The q-sampler release-assembly audit
now closes the current optimized x86_64 sampler body shape, but whole-signer
timing, SHA-256 behavior, OS entropy boundary, and rejection cleanup remain
open. The rejection-cleanup public-surface audit now closes the public
serialization part of that cleanup gap by proving constructor and entropy
failures export only coarse public rejection codes and no digest/proof material;
allocator/runtime cleanup evidence remains open. The SHA-256 boundary audit now
closes the source-level secret-dependent input-shape concern for signer entropy
derivation, and the SHA-256 call-schedule audit pins the valid private path to
fixed signer-owned call counts: 52 entropy-derivation digest calls plus 4
secret-component digest calls. The SHA-256 backend dependency inventory pins
the current review target to `sha2 0.10.9` with `cfg-if`, `cpufeatures`, and
`digest`, no `sha2-asm` dependency in the lockfile, and the active
`sha2/force-soft` feature. The SHA-256 backend/library constant-time claim
remains open.
The SHA-256 selected-backend source audit now pins the locked `sha2 0.10.9`
source surface: no active `sha2-asm` dependency, `force-soft` enabled, runtime
x86/SHA-NI dispatch not selected, soft backend selected, and the soft backend's
public block loop present. The selected-backend release assembly audit now pins
the compiled portable `compress256` object, absence of x86 `digest_blocks`,
absence of `shani_cpuid` storage, absence of public CPU-feature dispatch, zero
allocator/memory-routine calls in the audited compression body, zero SHA-NI
instruction counts, and a public block-count guard. A dedicated soft-backend
CT-shape audit additionally pins no source-level `if`/`match`/`while`, only the
public block-count loop branches in release assembly, zero calls/cmovs/setcc,
zero variable shifts, and zero allocator/memory/SHA-NI/CPU-dispatch markers.
The internal-state lifetime audit pins the remaining concrete gap: upstream
soft `sha2` owns local `block_u32` schedule state and `state_cpy`, exposes no
zeroization markers, and the compiled `compress256` stack frame reserves
`0xa8` bytes with zero explicit zero stores before return. This removes the
runtime dispatch/hardware-backend selection surface and materially narrows the
portable backend review, but does not close portable backend timing or SHA
internal compression-state lifetime review.
The OS entropy boundary audit now closes signer-boundary leakage around entropy
acquisition by removing raw backend error formatting and pinning zeroization on
either acquisition failure; OS RNG health and platform support remain backend
evidence. The runtime-cleanup source audit now closes source-level owned-buffer
cleanup shape for success and early-error paths, including explicit scrub before
private-fiber constructor errors return. Signer entropy/constructor digest
frames are now fixed stack-backed frames rather than heap `Vec` frames, and are
explicitly scrubbed after hashing; this removes the previous allocator-residue
concern for private SHA input frames by construction. The signer-owned
zeroization helpers now use volatile writes and compiler fences, and the
release-assembly zeroize audit closes compiler dead-store-resistance for the
fixed-size seed/root/syndrome helpers and the fixed 448-byte signer digest-frame
helper. The private digest-frame constructor backend audit now confirms the
entropy and secret-component digest constructors reserve fixed stack frames,
make zero allocator calls, and call the fixed-frame zeroizer after hashing;
whole-execution register-spill/lifetime review and SHA-256 compression-internal
state lifetime remain backend evidence. The
release whole-signer timing probe exposed a public-input
constructor-rejection short path in the legacy direct signer-report path. The
active implementation now has a prevalidated public-input wrapper that rejects
malformed public projection/account input before entropy acquisition and before
private signing. The production-style OS entropy report entrypoint now routes
through that wrapper, and a source-level production-routing audit pins that
shape while retaining the legacy direct path only as deterministic
test/negative-control evidence. This narrows the fixed-stage timing requirement
to valid/prevalidated private signing cases, but the active native signer/keygen
still remains non-deployable until whole-signer constant-time evidence is
supplied. In parallel, the active native storage/lifecycle hardening scope
remains an internal productization gate.
