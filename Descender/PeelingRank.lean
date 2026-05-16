import Descender.Wedge

namespace Descender

/-!
Peeling rank scaffold.

The main theorem is the finite, deterministic bridge from a peelable wedge
hypergraph and local endpoint independence to linear independence of any
small set of wedge columns.
-/

universe u v w f

noncomputable section

namespace WedgeSystem

variable {F : Type f} [Field F]
variable {Vertex : Type u} {Wedge : Type v} {Fiber : Type w}
variable [Fintype Vertex] [Fintype Wedge] [Fintype Fiber]

def peeledBlock (W : WedgeSystem Vertex Wedge) (S : Finset Wedge) (v : Vertex) :
    Finset Wedge := by
  classical
  exact S.filter fun w => v ∈ W.endpoints w

lemma degreeIn_eq_filter_card
    (W : WedgeSystem Vertex Wedge) (S : Finset Wedge) (v : Vertex) :
    W.toHypergraph.degreeIn S v =
      (W.peeledBlock S v).card := by
  classical
  rfl

lemma peeledBlock_card_le
    (W : WedgeSystem Vertex Wedge) {S : Finset Wedge} {v : Vertex} {k : Nat}
    (hv : W.toHypergraph.degreeIn S v ≤ k) :
    (W.peeledBlock S v).card ≤ k := by
  simpa [degreeIn_eq_filter_card] using hv

lemma peeledBlock_mem_base
    (W : WedgeSystem Vertex Wedge) {S : Finset Wedge} {v : Vertex}
    {w : Wedge} (hw : w ∈ W.peeledBlock S v) :
    w ∈ S := by
  classical
  exact (Finset.mem_filter.mp hw).1

lemma peeledBlock_mem_endpoint
    (W : WedgeSystem Vertex Wedge) {S : Finset Wedge} {v : Vertex}
    {w : Wedge} (hw : w ∈ W.peeledBlock S v) :
    v ∈ W.endpoints w := by
  classical
  exact (Finset.mem_filter.mp hw).2

lemma peeledBlock_all_incident
    (W : WedgeSystem Vertex Wedge) {S : Finset Wedge} {v : Vertex} :
    ∀ w, w ∈ W.peeledBlock S v -> v ∈ W.endpoints w := by
  intro w hw
  exact W.peeledBlock_mem_endpoint hw

omit [Fintype Fiber] in
lemma local_coefficients_vanish
    (W : WedgeSystem Vertex Wedge)
    (h : Wedge -> Vertex -> Fiber -> F)
    {k : Nat}
    (hLocal : W.LocallyIndependentUpTo h k)
    {S : Finset Wedge} {v : Vertex}
    (hS : S.card ≤ k)
    (hIncident : ∀ w, w ∈ S -> v ∈ W.endpoints w)
    (c : {w // w ∈ S} -> F)
    (hrel : (∑ x : {w // w ∈ S}, c x • localVec h v x.1) = 0) :
    ∀ x : {w // w ∈ S}, c x = 0 := by
  classical
  have hLI := hLocal v S hS hIncident
  have hcoeff :=
    (Fintype.linearIndependent_iffₛ.mp hLI)
      c
      (fun _ : {w // w ∈ S} => (0 : F))
      ?_
  · intro x
    simpa using hcoeff x
  · simpa [hrel]

omit [Fintype Fiber] in
lemma local_coefficients_vanish_on_peeledBlock
    (W : WedgeSystem Vertex Wedge)
    (h : Wedge -> Vertex -> Fiber -> F)
    {k : Nat}
    (hLocal : W.LocallyIndependentUpTo h k)
    {S : Finset Wedge} {v : Vertex}
    (hv : W.toHypergraph.degreeIn S v ≤ k)
    (c : {w // w ∈ W.peeledBlock S v} -> F)
    (hrel :
      (∑ x : {w // w ∈ W.peeledBlock S v},
        c x • localVec h v x.1) = 0) :
    ∀ x : {w // w ∈ W.peeledBlock S v}, c x = 0 := by
  classical
  exact W.local_coefficients_vanish h hLocal
    (W.peeledBlock_card_le hv)
    (W.peeledBlock_all_incident)
    c hrel

omit [Fintype Fiber] in
lemma global_relation_restricts_to_peeledBlock
    (W : WedgeSystem Vertex Wedge)
    (h : Wedge -> Vertex -> Fiber -> F)
    (hSupport : W.SupportedOnEndpoints h)
    {S : Finset Wedge} {v : Vertex}
    (c : {w // w ∈ S} -> F)
    (hrel : (∑ x : {w // w ∈ S}, c x • column h x.1) = 0) :
    (∑ y : {w // w ∈ W.peeledBlock S v},
      c ⟨y.1, W.peeledBlock_mem_base y.2⟩ • localVec h v y.1) = 0 := by
  classical
  ext a
  let coeff : Wedge -> F := fun w => if hw : w ∈ S then c ⟨w, hw⟩ else 0
  have hcoord :
      (∑ x : {w // w ∈ S}, c x • column h x.1) (v, a) = 0 := by
    simpa using congrFun hrel (v, a)
  calc
    (∑ y : {w // w ∈ W.peeledBlock S v},
        c ⟨y.1, W.peeledBlock_mem_base y.2⟩ • localVec h v y.1) a
        = ∑ y : {w // w ∈ W.peeledBlock S v},
            coeff y.1 * h y.1 v a := by
          simp only [Finset.sum_apply, Pi.smul_apply, localVec, smul_eq_mul]
          refine Finset.sum_congr rfl ?_
          intro y hy
          have hyS : y.1 ∈ S := W.peeledBlock_mem_base y.2
          simp [coeff, hyS]
    _ = ∑ w ∈ W.peeledBlock S v, coeff w * h w v a := by
          simpa using
            (Finset.sum_subtype (W.peeledBlock S v)
              (by intro w; rfl)
              (fun w => coeff w * h w v a)).symm
    _ = ∑ w ∈ S, coeff w * h w v a := by
          exact Finset.sum_subset
            (by
              intro w hw
              exact W.peeledBlock_mem_base hw)
            (by
              intro w hwS hwT
              have hv_not : v ∉ W.endpoints w := by
                intro hv
                exact hwT (Finset.mem_filter.mpr ⟨hwS, hv⟩)
              simp [hSupport w v hv_not])
    _ = ∑ x : {w // w ∈ S}, coeff x.1 * h x.1 v a := by
          exact
            (Finset.sum_subtype S
              (by intro w; rfl)
              (fun w => coeff w * h w v a))
    _ = ∑ x : {w // w ∈ S}, c x * h x.1 v a := by
          refine Finset.sum_congr rfl ?_
          intro x hx
          simp [coeff, x.2]
    _ = (∑ x : {w // w ∈ S}, c x • column h x.1) (v, a) := by
          simp [column]
    _ = 0 := by simpa using hcoord

omit [Fintype Vertex] [Fintype Wedge] [Fintype Fiber] in
lemma global_relation_restricts_to_erase_of_zero
    (h : Wedge -> Vertex -> Fiber -> F)
    [DecidableEq Wedge]
    {S : Finset Wedge} {e : Wedge}
    (heS : e ∈ S)
    (c : {w // w ∈ S} -> F)
    (hzero : c ⟨e, heS⟩ = 0)
    (hrel : (∑ x : {w // w ∈ S}, c x • column h x.1) = 0) :
    (∑ y : {w // w ∈ S.erase e},
      c ⟨y.1, Finset.mem_of_mem_erase y.2⟩ • column h y.1) = 0 := by
  classical
  let coeff : Wedge -> F := fun w => if hw : w ∈ S then c ⟨w, hw⟩ else 0
  have hsubS :
      (∑ w ∈ S, coeff w • column h w) =
        ∑ x : {w // w ∈ S}, coeff x.1 • column h x.1 :=
    Finset.sum_subtype S (by intro w; rfl)
      (fun w => coeff w • column h w)
  have hcoeffS :
      (∑ x : {w // w ∈ S}, coeff x.1 • column h x.1) =
        ∑ x : {w // w ∈ S}, c x • column h x.1 := by
    refine Finset.sum_congr rfl ?_
    intro x hx
    simp [coeff, x.2]
  have hfinS : (∑ w ∈ S, coeff w • column h w) = 0 := by
    calc
      (∑ w ∈ S, coeff w • column h w)
          = ∑ x : {w // w ∈ S}, coeff x.1 • column h x.1 := hsubS
      _ = ∑ x : {w // w ∈ S}, c x • column h x.1 := hcoeffS
      _ = 0 := hrel
  have herase_fin :
      (∑ w ∈ S.erase e, coeff w • column h w) =
        ∑ w ∈ S, coeff w • column h w := by
    exact Finset.sum_subset
      (by
        intro w hw
        exact Finset.mem_of_mem_erase hw)
      (by
        intro w hwS hwErase
        by_cases hwe : w = e
        · subst hwe
          simp [coeff, heS, hzero]
        · exact False.elim (hwErase (Finset.mem_erase.mpr ⟨hwe, hwS⟩)))
  have hfinErase : (∑ w ∈ S.erase e, coeff w • column h w) = 0 := by
    calc
      (∑ w ∈ S.erase e, coeff w • column h w)
          = ∑ w ∈ S, coeff w • column h w := herase_fin
      _ = 0 := hfinS
  have hsubErase :
      (∑ w ∈ S.erase e, coeff w • column h w) =
        ∑ y : {w // w ∈ S.erase e}, coeff y.1 • column h y.1 :=
    Finset.sum_subtype (S.erase e) (by intro w; rfl)
      (fun w => coeff w • column h w)
  calc
    (∑ y : {w // w ∈ S.erase e},
      c ⟨y.1, Finset.mem_of_mem_erase y.2⟩ • column h y.1)
        = ∑ y : {w // w ∈ S.erase e}, coeff y.1 • column h y.1 := by
          refine Finset.sum_congr rfl ?_
          intro y hy
          have hyS : y.1 ∈ S := Finset.mem_of_mem_erase y.2
          simp [coeff, hyS]
    _ = ∑ w ∈ S.erase e, coeff w • column h w := hsubErase.symm
    _ = 0 := hfinErase

omit [Fintype Fiber] in
lemma coefficient_vanishes_at_peeled_vertex
    (W : WedgeSystem Vertex Wedge)
    (h : Wedge -> Vertex -> Fiber -> F)
    {k : Nat}
    (hSupport : W.SupportedOnEndpoints h)
    (hLocal : W.LocallyIndependentUpTo h k)
    {S : Finset Wedge} {v : Vertex}
    (hv : W.toHypergraph.degreeIn S v ≤ k)
    (c : {w // w ∈ S} -> F)
    (hrel : (∑ x : {w // w ∈ S}, c x • column h x.1) = 0)
    (x : {w // w ∈ S})
    (hxv : v ∈ W.endpoints x.1) :
    c x = 0 := by
  classical
  let T : Finset Wedge := W.peeledBlock S v
  let cT : {w // w ∈ T} -> F :=
    fun y => c ⟨y.1, W.peeledBlock_mem_base y.2⟩
  have hlocalRel :
      (∑ y : {w // w ∈ T}, cT y • localVec h v y.1) = 0 := by
    simpa [T, cT] using
      W.global_relation_restricts_to_peeledBlock h hSupport c hrel
  have hzero :=
    W.local_coefficients_vanish_on_peeledBlock h hLocal hv cT hlocalRel
  have hxT : x.1 ∈ T := by
    exact Finset.mem_filter.mpr ⟨x.2, hxv⟩
  simpa [cT] using hzero ⟨x.1, hxT⟩

omit [Fintype Fiber] in
theorem linearIndependent_columns_of_peelable
    (W : WedgeSystem Vertex Wedge)
    (h : Wedge -> Vertex -> Fiber -> F)
    {tauN k : Nat}
    (hPeel : (W.toHypergraph).Peelable tauN k)
    (hSupport : W.SupportedOnEndpoints h)
    (hLocal : W.LocallyIndependentUpTo h k)
    (S : Finset Wedge)
    (hS : S.card ≤ tauN) :
    LinearIndependent F
      (fun x : {w // w ∈ S} => column h x.1) := by
  classical
  /-
  The endpoint support hypothesis is essential: without it, non-incident
  wedges can contribute at the peeled vertex and local independence cannot
  isolate the peeled block.
  -/
  have hmain :
      ∀ n,
        ∀ S : Finset Wedge,
          S.card = n ->
          S.card ≤ tauN ->
          LinearIndependent F
            (fun x : {w // w ∈ S} => column h x.1) := by
    intro n
    induction n using Nat.strong_induction_on with
    | h n ih =>
      intro S hcard hSsmall
      by_cases hSnonempty : S.Nonempty
      · rcases hPeel S hSnonempty hSsmall with ⟨v, hv_pos, hv_le⟩
        have hblock_nonempty : (W.peeledBlock S v).Nonempty := by
          rw [← Finset.card_pos]
          have hdeg_pos : 0 < W.toHypergraph.degreeIn S v :=
            Nat.lt_of_lt_of_le Nat.zero_lt_one hv_pos
          simpa [degreeIn_eq_filter_card] using hdeg_pos
        rcases hblock_nonempty with ⟨e, heT⟩
        have heS : e ∈ S := W.peeledBlock_mem_base heT
        have hev : v ∈ W.endpoints e := W.peeledBlock_mem_endpoint heT
        refine (Fintype.linearIndependent_iff).mpr ?_
        intro c hrel x
        by_cases hxv : v ∈ W.endpoints x.1
        · exact W.coefficient_vanishes_at_peeled_vertex
            h hSupport hLocal hv_le c hrel x hxv
        · have he_zero : c ⟨e, heS⟩ = 0 :=
            W.coefficient_vanishes_at_peeled_vertex
              h hSupport hLocal hv_le c hrel ⟨e, heS⟩ hev
          have hlt : (S.erase e).card < n := by
            calc
              (S.erase e).card < S.card := Finset.card_erase_lt_of_mem heS
              _ = n := hcard
          have hEraseSmall : (S.erase e).card ≤ tauN :=
            le_trans (Finset.card_erase_le (s := S) (a := e)) hSsmall
          have hLIerase :=
            ih (S.erase e).card hlt (S.erase e) rfl hEraseSmall
          let cErase : {w // w ∈ S.erase e} -> F :=
            fun y => c ⟨y.1, Finset.mem_of_mem_erase y.2⟩
          have hrelErase :
              (∑ y : {w // w ∈ S.erase e},
                cErase y • column h y.1) = 0 := by
            simpa [cErase] using
              global_relation_restricts_to_erase_of_zero
                h heS c he_zero hrel
          have hxe : x.1 ≠ e := by
            intro hxe
            exact hxv (by simpa [hxe] using hev)
          have hxErase : x.1 ∈ S.erase e :=
            Finset.mem_erase.mpr ⟨hxe, x.2⟩
          have hx_zero :=
            (Fintype.linearIndependent_iff.mp hLIerase)
              cErase hrelErase ⟨x.1, hxErase⟩
          simpa [cErase] using hx_zero
      · have hEmpty : S = ∅ := Finset.not_nonempty_iff_eq_empty.mp hSnonempty
        refine (Fintype.linearIndependent_iff).mpr ?_
        intro c hrel x
        exfalso
        simpa [hEmpty] using x.2
  exact hmain S.card S rfl hS

omit [Fintype Fiber] in
theorem peelability_implies_linear_independence
    (W : WedgeSystem Vertex Wedge)
    (h : Wedge -> Vertex -> Fiber -> F)
    {tauN k : Nat}
    (hPeel : (W.toHypergraph).Peelable tauN k)
    (hSupport : W.SupportedOnEndpoints h)
    (hLocal : W.LocallyIndependentUpTo h k)
    (S : Finset Wedge)
    (hS : S.card ≤ tauN) :
    LinearIndependent F
      (fun x : {w // w ∈ S} => column h x.1) :=
  W.linearIndependent_columns_of_peelable h hPeel hSupport hLocal S hS

end WedgeSystem

end

end Descender
