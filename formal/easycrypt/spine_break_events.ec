(* -----------------------------------------------------------------------------
   GAP-B instantiation, step 1 (datatype foundation).

   Concretizes the abstract `break_t` of spike_union_bound.ec (declared there as
   `type break_t` with `card 9`, plus axioms break_univ_uniq / break_univ_total)
   into the 9 NAMED break events from SECURITY_DOSSIER section 2, and proves the three
   finiteness facts the parametric spine carries as axioms. This is the EC-verified
   datatype the full SUF-CMA clone (spine_endpoint.ec, future) plugs in via
   `clone Spike_union_bound with type break_t = break_event, op break_univ = ...`,
   discharging those axioms with the lemmas below.

   This file introduces NO `admit`/`sorry`/`axiom` — every fact is machine-checked.
   It does NOT yet model the experiment / structural_split (those need the faithful
   verify model — the high-stakes multi-week core; see GAPB_SPINE_INSTANTIATION_SPEC.md).
   --------------------------------------------------------------------------- *)
require import AllCore List.

(* The 9 named break events (dossier section 2 rows 1-9). *)
type break_event = [
  | CreationTimeHiddenSource           (* 1 break_creation_time_hidden_source.ec       *)
  | PrevalidationLeafBinding           (* 2 break_prevalidation_leaf_binding.ec         *)
  | PrevalidationRegistryBinding       (* 3 break_prevalidation_registry_binding.ec     *)
  | PrevalidationDataAvailability      (* 4 NO EC reduction (held fail-closed)          *)
  | PostchallengeSlotCreationOrRewrite (* 5 break_postchallenge_slot_creation...ec      *)
  | CheckedCoreMessageBinding          (* 6 break_checked_core_message_binding.ec       *)
  | SlotReplayOrRollback               (* 7 break_slot_replay_or_rollback.ec            *)
  | QromOrDomainSeparation             (* 8 classifier False                            *)
  | LiveSlotLeakageOrKeygen            (* 9 classifier False                            *)
].

(* The finite enumeration (Lean `Finset.univ`; spike_union_bound `break_univ`). *)
op break_event_univ : break_event list =
  [ CreationTimeHiddenSource
  ; PrevalidationLeafBinding
  ; PrevalidationRegistryBinding
  ; PrevalidationDataAvailability
  ; PostchallengeSlotCreationOrRewrite
  ; CheckedCoreMessageBinding
  ; SlotReplayOrRollback
  ; QromOrDomainSeparation
  ; LiveSlotLeakageOrKeygen ].

(* Discharges spike_union_bound `break_univ_uniq`. *)
lemma break_event_univ_uniq : uniq break_event_univ.
proof. by rewrite /break_event_univ. qed.

(* Discharges spike_union_bound `break_univ_total`. *)
lemma break_event_univ_total : forall (b : break_event), b \in break_event_univ.
proof. by case. qed.

(* Discharges spike_union_bound `break_card9 : size break_univ = 9`. *)
lemma break_event_card9 : size break_event_univ = 9.
proof. by rewrite /break_event_univ. qed.
