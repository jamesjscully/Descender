require import AllCore List.
require import Spike_union_bound.
require import Spine_break_events.

clone import Spine with
  type break_t    <- break_event,
  op   break_univ <- break_event_univ
proof break_univ_uniq, break_univ_total, break_card9.
realize break_univ_uniq.  proof. exact break_event_univ_uniq.  qed.
realize break_univ_total. proof. exact break_event_univ_total. qed.
realize break_card9.      proof. exact break_event_card9.      qed.
