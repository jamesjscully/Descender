# Quotient-Level Link Promotion Gate

Status: no replacement.  Keep the current reviewed split design.

This gate does not change the q12289 verifier, proof path, byte target, or implementation.

## Decision

| item | value |
|---|---|
| classification | `no_replacement` |
| decision | `keep_current_path` |
| migration plan required | `False` |
| current spend bytes | `250` |
| current account bytes | `68` |
| current online plus account bytes | `318` |

Public quotient links fail sampler hiding under public-link conditioning.  Hidden-commitment links preserve the desired sampler surface only by adding proof obligations or assumptions that are not cheaper or simpler than the current 68-byte split account object.  Existing account commitments cannot supply the hidden linear link equation for free.

Implementation instruction: Do not migrate the deployable q12289 verifier, proof path, byte target, or account-link implementation based on this research program.  Continue using the current reviewed split account-carried bounded-kernel design unless a later promotion gate with independent review supersedes this one.

## Promotion Matrix

| candidate | evidence complete | bytes saved | state changed | assumptions changed | Lean support | review status | recommendation |
|---|---:|---:|---|---|---|---|---|
| current-reviewed-split-account-link | `True` | `0` | `False` | none | CommittedSigma reviewed account-link path | current control | keep |
| public-two-row-quotient-link-shortlist | `True` | `64` | `True` | adds public link conditioning; hiding target fails | not migrated | rejected before review | reject: sampler support floor falls to 106-108 bits |
| hidden-commitment-quotient-link | `True` | `None` | `depends_on_commitment_choice` | adds commitment hiding, binding/extraction, and non-public link-equation proof obligations | Descender.HiddenCommittedQuotientLink sidecar only | not ready | reject for deployment: no surviving byte or proof simplification |
| reuse-existing-account-commitments-for-link | `True` | `0` | `False` | would require new ZK/hash-circuit proof if link remains hidden | source hooks checked; no free homomorphic equation | not needed for rejection | reject for deployment |
| pedersen-style-homomorphic-hidden-link | `False` | `at_most_4_before_proof_bytes` | `True` | adds new algebraic commitment assumption and likely proof/opening bytes | interface-compatible in principle only | would require independent cryptanalytic review | research-only, not promotion-ready |

## Assumption Deltas

- No cryptanalytic assumption is removed from the deployable path.
- Public quotient links add a public-link conditioning surface and fail the 128-bit hiding target.
- Hidden commitments add commitment hiding and binding/extraction obligations.
- A hidden link equation must be non-public; a public projection of the sigma equation adds no security constraint.
- Reuse of byte-hash account commitments would require a new ZK/hash-circuit soundness argument to avoid opening link values.

## Replay Commands

- `python3 scripts/research/quotient_level_link_kernel_search.py`
- `python3 scripts/research/quotient_level_link_coordinate_pair_sweep.py --top-k 20 --exact-margin-floor 43 --bound 96`
- `python3 scripts/research/quotient_level_link_exact_enum.py coordinate-link-1-2 --bound 96`
- `python3 scripts/research/quotient_level_link_exact_enum.py sparse-difference-link-0m3-1m2 --bound 96`
- `python3 scripts/research/quotient_level_link_exact_enum.py coordinate-link-1-14 --bound 96`
- `python3 scripts/research/quotient_level_link_exact_enum.py coordinate-link-8-20 --bound 96`
- `python3 scripts/research/quotient_level_link_exact_enum.py coordinate-link-14-15 --bound 96`
- `python3 scripts/research/quotient_level_link_sampler_replay.py current-coordinate-link-0-3 --bound 96`
- `python3 scripts/research/quotient_level_link_sampler_replay.py coordinate-link-1-2 --bound 96`
- `python3 scripts/research/quotient_level_link_sampler_replay.py sparse-difference-link-0m3-1m2 --bound 96`
- `python3 scripts/research/quotient_level_link_sampler_replay.py coordinate-link-1-14 --bound 96`
- `python3 scripts/research/quotient_level_link_sampler_replay.py coordinate-link-8-20 --bound 96`
- `python3 scripts/research/quotient_level_link_sampler_replay.py coordinate-link-14-15 --bound 96`
- `python3 scripts/research/quotient_level_link_hiding_review.py`
- `python3 scripts/research/quotient_level_link_account_semantics.py`
- `python3 scripts/research/quotient_level_link_hidden_commitment_byte_state_review.py`
- `python3 scripts/research/quotient_level_link_existing_commitment_reuse_review.py`
- `python3 scripts/research/quotient_level_link_firebreak_audit.py`
- `python3 scripts/research/quotient_level_link_promotion_gate.py`
- `python3 scripts/research/verify_quotient_level_link_closure.py`
- `python3 scripts/research/review_gate_replay.py --dry-run --skip-timing --json /tmp/quotient_review_gate_dry_run.json --md /tmp/quotient_review_gate_dry_run.md`

## Lean Support

Build commands:
- `lake build Descender.DescentInterface`
- `lake build Descender.HiddenCommittedQuotientLink`

Theorem/interface names:
- `Descender.DescentInterface.EffectiveDescentCertificate.accepts_glues_or_breaks`
- `Descender.CommittedSigma.AccountLinkProofSystem.accepts_accountBacked_of_noBreak`
- `Descender.CommittedSigma.HomomorphicAccountLinkInterface.linkProofAccepts_of_publicProjection`
- `Descender.CommittedSigma.reviewedAccountHiddenLinkBytes_eq`
- `Descender.HiddenCommittedQuotientLink.Interface.toAccountLinkProofSystem`
- `Descender.HiddenCommittedQuotientLink.Interface.accepts_accountBacked_of_noBreaks`
- `Descender.HiddenCommittedQuotientLink.Interface.accepts_glues_or_breaks_of_effectiveDescent`

## Future Reopening Requirements

- Concrete commitment/proof system with byte and verifier-cost model.
- Sampler replay showing no public-link conditioning below the hiding target.
- Exact bounded-kernel certificate for the concrete selected link.
- Lean q12289-shaped instantiation of the sidecar theorem shape.
- Independent cryptanalytic review of any new commitment or ZK/hash-circuit assumption.
- Passing import firebreak audit before any deployable import change.
- Review-gate replay dry run includes quotient_level_link_closure_verify.

## Artifact Hashes

| path | sha256 | bytes |
|---|---|---:|
| `Descender.lean` | `c89c91427ef5ae5df7cc89b3918fb4307a00842b5cd1cc5d20913e12ab14873b` | `1200` |
| `Descender/DescentInterface.lean` | `599c70497e229234390d1d435f082856235b5b2ba646c2ef9582dc30eca0650d` | `8617` |
| `Descender/HiddenCommittedQuotientLink.lean` | `0c806fb87763a9b5870f05d8b63c95116aa810b7d4891234f11c8b5468740bef` | `9950` |
| `research/cryptanalysis/signature_readiness_20260516/README.md` | `a19f9c063db4baec5d57bcdf37cd3eb1304b41681e7b934c4d7cef7b2fbc6935` | `122159` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_research_program.md` | `cb27fb78e31f1c692f18ac5648c30277e298ee62f52c0d8efaa9329f93c8e106` | `25410` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_kernel_search.json` | `e8a76758996be24e16068eb601e50571364a165f4c8502442e7f9424187725a4` | `52407` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_kernel_search.md` | `0070affcc68d91978176304fa78e73d25d65d2080fc706c133664966bb8fd005` | `7770` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_coordinate_pair_sweep.json` | `108bc13855a1d58fd6837d6ac1491dc8aee15ab9a8a5b76f5189d4a3fcdbf4ee` | `361001` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_coordinate_pair_sweep.md` | `5cf1946290d198c30714df4676a1c5fe408726d0666cd63f0e51ca21013b2263` | `7940` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_coordinate-link-1-2_b96.json` | `9464e0bcdf862af4d5545d4df2d8a124610d28c87d8a0200892ccf2a88732c73` | `8959` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_sparse-difference-link-0m3-1m2_b96.json` | `84f921c1771dfbf73d3cfe20b14fd23eb133c1903826f944c50ecfc1eddc3bb9` | `9114` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_coordinate-link-1-14_b96.json` | `16b9c1f60c073e694368bd9ab76ca483c2f952b6f3953e3536a15ede890e587f` | `8876` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_coordinate-link-8-20_b96.json` | `9b6dea9d99be80556084bfafaf60108dd43f771d76cd8e0dc716480e24213470` | `8879` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_exact_enum_coordinate-link-14-15_b96.json` | `3bec577d117c6d0fb3f2da889d0e6169b0f398f345f3182118cada62617f8230` | `8863` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_sampler_replay_current-coordinate-link-0-3_b96.json` | `f81df9b69d9b50f62351597e1ca0113047b062ae4522cd66bb9d66bd7ed252e5` | `3461` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_sampler_replay_coordinate-link-1-2_b96.json` | `3bf3b0d0853314279058cdcba77d2865be1c973ffc68ddd9aca285a19e33b611` | `3450` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_sampler_replay_sparse-difference-link-0m3-1m2_b96.json` | `888cc12a9be94b9401703e69e247bed508fc0eaa6bddca4406d20a1678f7e26a` | `3444` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_sampler_replay_coordinate-link-1-14_b96.json` | `2998eeecd07e2e98efa1d00941ee907f4027f860daa8c6df020f4065fb6833bf` | `3442` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_sampler_replay_coordinate-link-8-20_b96.json` | `339ccd7570c4b0b0320a88e644ce98b36f001565feb802d71ae2a84b364fcc99` | `3424` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_sampler_replay_coordinate-link-14-15_b96.json` | `5c31e688bb7ffec0feb6030fe9c8d7e95535612050ef4338c8e30956604b6ba5` | `3427` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_hiding_review.json` | `a4e08732fae2422b243c89bab4ba9a938da61e84384e57313a3c4a6e75c8ab0d` | `15075` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_hiding_review.md` | `7a131bf9ba6812ccc430e18dfa149ce0205a40f41da503f7262bcf0c462244bd` | `3287` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_account_semantics.json` | `f36c98b36f3b31c99413c3aab56cba08be200d149b72395855d3f4fcf306273d` | `5853` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_account_semantics.md` | `914f14c9505845d1ba7dba731fa3637179e1305f2d632973d260c54b5aeb3ddf` | `3407` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_lean_interface_comparison.md` | `af7fac7b19cad875bda64d235ecb82c52579ee798521f52f987c2b3d69fc1559` | `3463` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_hidden_commitment_byte_state_review.json` | `4b0d5358f606ddd3987d75f7a5e62f4e72f97e2d53179526480133e749af1043` | `10172` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_hidden_commitment_byte_state_review.md` | `4c7eb2019266df63c0019610c63b8742f8bd6f6edc77d90d41295ca6249f6b33` | `3221` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_existing_commitment_reuse_review.json` | `7a30218a08c0fdbec4a4534320020cdffae23e249b567422f70cc1cf6fa8c3c5` | `7186` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_existing_commitment_reuse_review.md` | `11648055702e63e11b9dfac6b7551fc2beab4bdb563ae8515062ef706b3d7160` | `4322` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_implementation_handoff.md` | `64908999a63be759e1d45b084b2a35490cb2f7cda3ab811098cd8e5bb39b6f85` | `3202` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_firebreak_audit.json` | `a396931c27b64ba6b92a1efebf7ada02bbe3ffff3cf07f7e63aea4b8c3ecbe12` | `1113` |
| `research/cryptanalysis/signature_readiness_20260516/quotient_level_link_firebreak_audit.md` | `7ba5f32bb4abe76e7a63b84e44d3655f31bf8aee154622c5432733d8cda91796` | `841` |

## Script Hashes

| path | sha256 | bytes |
|---|---|---:|
| `scripts/research/quotient_level_link_kernel_search.py` | `a5e030510fb2cbc7940cea248609e75a62930d2c5836c6371dacee307c0e9e84` | `31611` |
| `scripts/research/quotient_level_link_coordinate_pair_sweep.py` | `2d0e9f9455f721bc2afd6938d1435f70534aafa27a09b5bd8d912440c3ef4937` | `12765` |
| `scripts/research/quotient_level_link_exact_enum.py` | `b7fdaa4ec8192d8761ef40fdba405792038e16ed55724126769d065c4643a4a8` | `13913` |
| `scripts/research/quotient_level_link_sampler_replay.py` | `21c62d9c7621cadb0ed25b0f478bc303489fa45e7dbb46e0a8f831334e0ba61c` | `9766` |
| `scripts/research/quotient_level_link_hiding_review.py` | `1a31af84e97887b327e7c00d1ed40584c2cc78bea2288d3590b7dae7e99abccf` | `17299` |
| `scripts/research/quotient_level_link_account_semantics.py` | `e5309d848ea08bdea23927fea6b0a4c5fe2e64c70e5c282b2892b67c76d0cb5a` | `11884` |
| `scripts/research/quotient_level_link_hidden_commitment_byte_state_review.py` | `11cd2eb15a6e4c0849da4907cfc81fa71a0aa1963415f2cd975b8c0442098e37` | `14229` |
| `scripts/research/quotient_level_link_existing_commitment_reuse_review.py` | `b373a8cd3bd1b2eb6b9cd64cd2fe02cebcb5acc787393766feb6b5e10a963963` | `13692` |
| `scripts/research/quotient_level_link_promotion_gate.py` | `129ffd311add8a4d364b971c0e586ae8f7535ee67d93a5b95f33fcafdae89d28` | `19108` |
| `scripts/research/quotient_level_link_firebreak_audit.py` | `6250b762fb02bc43ddaed74371643a67708bdcc650e27938277902dd041bcdba` | `5823` |
| `scripts/research/verify_quotient_level_link_closure.py` | `f3c239d970dad3454d6e0fc7d8b73d870e44623a2c49ede07c1727ea2d15d607` | `3735` |
| `scripts/research/review_gate_replay.py` | `eaceb75a67b6a2af0576062377c4b48f4143f87f625c0060175b134e0da9d5a9` | `16137` |
