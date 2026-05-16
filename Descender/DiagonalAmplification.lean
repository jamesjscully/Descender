import Mathlib.Algebra.Group.Units.Basic
import Mathlib.Algebra.Field.ZMod
import Mathlib.Algebra.Order.BigOperators.GroupWithZero.Finset
import Mathlib.Data.Fintype.BigOperators
import Descender.SampleView

namespace Descender

/-!
Finite-counting interface for the public diagonal amplification step.

`SampleView.DiagonalAmplifiesSupport` is the deterministic good event.  This
file adds the finite seed-space boundary around it: a later probabilistic or
combinatorial proof only has to upper-bound the number of bad diagonal seeds.
If the bad set does not cover the whole seed space, the deterministic
correctness certificate can be instantiated with a good seed.
-/

namespace SampleView

variable {q : Nat}
variable {FullCoord ViewCoord Secret Noise Seed : Type*}

/-- The concrete public diagonal seed space: one field element per viewed
coordinate. -/
abbrev FullDiagonalSeed (q : Nat) [NeZero q] (ViewCoord : Type*) :=
  ViewCoord -> ZMod q

/-- Interpret a full diagonal seed as the public scaling function. -/
def fullDiagonalScale [NeZero q]
    (seed : FullDiagonalSeed q ViewCoord) : ViewCoord -> ZMod q :=
  seed

/-- A seed is good when its public diagonal satisfies the deterministic
support-to-amplitude condition. -/
def GoodDiagonalSeed [NeZero q] [Fintype FullCoord] [Fintype ViewCoord]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) (seed : Seed) : Prop :=
  DiagonalAmplifiesSupport (q := q) select (scale seed) eval minSupport eta

/-- Bad seeds are exactly those whose diagonal fails the deterministic
amplification event. -/
noncomputable def badDiagonalSeeds
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Seed]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) : Finset Seed := by
  classical
  exact Finset.univ.filter fun seed =>
    ¬ GoodDiagonalSeed (q := q) select scale eval minSupport eta seed

/--
Finite bad-seed counting obligation for the probabilistic diagonal layer.

For a uniform seed distribution this corresponds to failure probability at most
`failureBudget / Fintype.card Seed`; the current Lean layer keeps it as an exact
cardinality inequality to avoid committing to a measure API.
-/
noncomputable def DiagonalAmplificationFailureCountAtMost
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Seed]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta failureBudget : Nat) : Prop :=
  (badDiagonalSeeds (q := q) select scale eval minSupport eta).card <=
    failureBudget

/-- One-coordinate bad-scale event for a nonzero pair difference. -/
def CoordinateBadScale [NeZero q] (delta scaleValue : ZMod q)
    (eta : Nat) : Prop :=
  centeredAbs q (scaleValue * delta) <= 2 * eta

/-- The bad scale values for one coordinate difference. -/
noncomputable def coordinateBadScales [NeZero q] [Fintype (ZMod q)]
    (delta : ZMod q) (eta : Nat) : Finset (ZMod q) := by
  classical
  exact Finset.univ.filter fun scaleValue =>
    CoordinateBadScale (q := q) delta scaleValue eta

/-- Centered-small residues at a fixed absolute-value bound. -/
noncomputable def centeredSmallResidues [NeZero q] [Fintype (ZMod q)]
    (bound : Nat) : Finset (ZMod q) := by
  classical
  exact Finset.univ.filter fun residue =>
    centeredAbs q residue <= bound

theorem mem_coordinateBadScales_iff
    [NeZero q] [Fintype (ZMod q)] (delta scaleValue : ZMod q)
    (eta : Nat) :
    scaleValue ∈ coordinateBadScales (q := q) delta eta ↔
      CoordinateBadScale (q := q) delta scaleValue eta := by
  classical
  simp [coordinateBadScales]

theorem mem_centeredSmallResidues_iff
    [NeZero q] [Fintype (ZMod q)] (residue : ZMod q) (bound : Nat) :
    residue ∈ centeredSmallResidues (q := q) bound ↔
      centeredAbs q residue <= bound := by
  classical
  simp [centeredSmallResidues]

/--
Multiplication by a unit carries the one-coordinate bad-scale set exactly onto
the centered-small residue set.

This is the algebraic heart of the diagonal anti-concentration estimate.  In a
prime field the unit hypothesis is discharged by showing the selected
coordinate difference is nonzero.
-/
theorem coordinateBadScales_map_mul_eq_centeredSmallResidues_of_isUnit
    [NeZero q] [Fintype (ZMod q)] [DecidableEq (ZMod q)]
    {delta : ZMod q} (hdelta : IsUnit delta) (eta : Nat) :
    (coordinateBadScales (q := q) delta eta).map
        ⟨fun scaleValue => scaleValue * delta,
          hdelta.mul_left_injective⟩ =
      centeredSmallResidues (q := q) (2 * eta) := by
  classical
  ext residue
  constructor
  · intro hresidue
    rcases Finset.mem_map.mp hresidue with ⟨scaleValue, hscale, hmul⟩
    exact (mem_centeredSmallResidues_iff
      (q := q) residue (2 * eta)).mpr (by
        rw [← hmul]
        exact (mem_coordinateBadScales_iff
          (q := q) delta scaleValue eta).mp hscale)
  · intro hresidue
    let u : (ZMod q)ˣ := hdelta.unit
    let scaleValue : ZMod q := residue * ↑u⁻¹
    refine Finset.mem_map.mpr ⟨scaleValue, ?_, ?_⟩
    · have hsmall :
          centeredAbs q residue <= 2 * eta :=
        (mem_centeredSmallResidues_iff
          (q := q) residue (2 * eta)).mp hresidue
      exact (mem_coordinateBadScales_iff
        (q := q) delta scaleValue eta).mpr (by
          unfold CoordinateBadScale
          simpa [scaleValue, u, hdelta.unit_spec, mul_assoc] using hsmall)
    · simp [scaleValue, u, mul_assoc]

/-- Unit coordinate differences have exactly the centered-small number of bad
scale values. -/
theorem coordinateBadScales_card_eq_centeredSmallResidues_of_isUnit
    [NeZero q] [Fintype (ZMod q)] [DecidableEq (ZMod q)]
    {delta : ZMod q} (hdelta : IsUnit delta) (eta : Nat) :
    (coordinateBadScales (q := q) delta eta).card =
      (centeredSmallResidues (q := q) (2 * eta)).card := by
  classical
  let emb : ZMod q ↪ ZMod q :=
    ⟨fun scaleValue => scaleValue * delta, hdelta.mul_left_injective⟩
  have hmap :
      (coordinateBadScales (q := q) delta eta).map emb =
        centeredSmallResidues (q := q) (2 * eta) :=
    coordinateBadScales_map_mul_eq_centeredSmallResidues_of_isUnit
      (q := q) hdelta eta
  calc
    (coordinateBadScales (q := q) delta eta).card
        = ((coordinateBadScales (q := q) delta eta).map emb).card := by
          simp [emb]
    _ = (centeredSmallResidues (q := q) (2 * eta)).card := by
          rw [hmap]

/-- The centered-small residue set has the expected interval-size upper bound.

This does not require `q` to be prime: centered lifts inject into the integer
interval `[-bound, bound]`.
-/
theorem centeredSmallResidues_card_le_interval
    [NeZero q] [Fintype (ZMod q)] (bound : Nat) :
    (centeredSmallResidues (q := q) bound).card <= 2 * bound + 1 := by
  classical
  let liftEmb : ZMod q ↪ Int :=
    ⟨centeredLift q, by
      intro a b h
      exact ZMod.valMinAbs_inj.mp h⟩
  have hsubset :
      (centeredSmallResidues (q := q) bound).map liftEmb ⊆
        Finset.Icc (-(bound : Int)) (bound : Int) := by
    intro z hz
    rcases Finset.mem_map.mp hz with ⟨a, ha, rfl⟩
    rw [Finset.mem_Icc]
    have habs : centeredAbs q a <= bound :=
      (mem_centeredSmallResidues_iff (q := q) a bound).mp ha
    change -(bound : Int) <= centeredLift q a ∧ centeredLift q a <= (bound : Int)
    unfold centeredAbs centeredLift at habs
    unfold centeredLift
    omega
  calc
    (centeredSmallResidues (q := q) bound).card
        = ((centeredSmallResidues (q := q) bound).map liftEmb).card := by
          simp [liftEmb]
    _ <= (Finset.Icc (-(bound : Int)) (bound : Int)).card :=
          Finset.card_le_card hsubset
    _ = 2 * bound + 1 := by
          simp
          omega

/-- Seeds whose scale at coordinate `j` is bad for difference `delta`. -/
noncomputable def coordinateBadDiagonalSeeds
    [NeZero q] [Fintype ViewCoord] [Fintype Seed]
    (scale : Seed -> ViewCoord -> ZMod q)
    (j : ViewCoord) (delta : ZMod q) (eta : Nat) : Finset Seed := by
  classical
  exact Finset.univ.filter fun seed =>
    CoordinateBadScale (q := q) delta (scale seed j) eta

/-- Secret-pair-indexed bad event for a fixed diagonal seed. -/
def BadPairDiagonalSeed [NeZero q] [Fintype FullCoord] [Fintype ViewCoord]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) (pair : Secret × Secret) (seed : Seed) : Prop :=
  pair.1 ≠ pair.2 ∧
    minSupport <= weight
      (fun i : FullCoord => eval pair.1 i - eval pair.2 i) ∧
    ¬ 2 * eta < centeredNormLinf q
      (fun j : ViewCoord =>
        scale seed j * eval pair.1 (select j) -
          scale seed j * eval pair.2 (select j))

/-- Bad seeds for one ordered pair of secrets. -/
noncomputable def badPairDiagonalSeeds
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Seed]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) (pair : Secret × Secret) : Finset Seed := by
  classical
  exact Finset.univ.filter fun seed =>
    BadPairDiagonalSeed (q := q) select scale eval minSupport eta pair seed

/-- If a vector has small centered `L∞` norm, every coordinate is small. -/
theorem centeredAbs_le_of_centeredNormLinf_le
    [Fintype ViewCoord] [NeZero q]
    (x : ViewCoord -> ZMod q) (j : ViewCoord) {bound : Nat}
    (hBound : centeredNormLinf q x <= bound) :
    centeredAbs q (x j) <= bound := by
  exact Nat.le_trans
    (Finset.le_sup (s := Finset.univ) (f := fun j => centeredAbs q (x j))
      (Finset.mem_univ j))
    hBound

/--
For a fixed secret pair, a bad diagonal seed is also bad on any selected
coordinate where the pair has the corresponding coordinate difference.

This is the local reduction needed before doing actual scale counting.
-/
theorem badPairDiagonalSeeds_subset_coordinateBadDiagonalSeeds
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Seed]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) (pair : Secret × Secret) (j : ViewCoord) :
    badPairDiagonalSeeds (q := q) select scale eval minSupport eta pair ⊆
      coordinateBadDiagonalSeeds (q := q) scale j
        (eval pair.1 (select j) - eval pair.2 (select j)) eta := by
  classical
  intro seed hseed
  rcases (Finset.mem_filter.mp hseed).2 with ⟨_hst, _hSupport, hFail⟩
  have hNorm :
      centeredNormLinf q
        (fun j : ViewCoord =>
          scale seed j * eval pair.1 (select j) -
            scale seed j * eval pair.2 (select j)) <= 2 * eta :=
    Nat.le_of_not_gt hFail
  have hCoord :
      centeredAbs q
        (scale seed j * eval pair.1 (select j) -
          scale seed j * eval pair.2 (select j)) <= 2 * eta :=
    centeredAbs_le_of_centeredNormLinf_le
      (q := q)
      (fun j : ViewCoord =>
        scale seed j * eval pair.1 (select j) -
          scale seed j * eval pair.2 (select j))
      j hNorm
  have hCoord' :
      CoordinateBadScale (q := q)
        (eval pair.1 (select j) - eval pair.2 (select j))
        (scale seed j) eta := by
    unfold CoordinateBadScale
    simpa [mul_sub] using hCoord
  exact Finset.mem_filter.mpr ⟨Finset.mem_univ seed, hCoord'⟩

/-- Pair bad-seed count is bounded by any selected coordinate bad-seed count. -/
theorem badPairDiagonalSeeds_card_le_coordinateBadDiagonalSeeds_card
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Seed]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) (pair : Secret × Secret) (j : ViewCoord) :
    (badPairDiagonalSeeds (q := q) select scale eval minSupport eta pair).card <=
      (coordinateBadDiagonalSeeds (q := q) scale j
        (eval pair.1 (select j) - eval pair.2 (select j)) eta).card :=
  Finset.card_le_card
    (badPairDiagonalSeeds_subset_coordinateBadDiagonalSeeds
      (q := q) select scale eval minSupport eta pair j)

/--
A coordinate bad-seed estimate gives a pair bad-seed estimate.

For the full diagonal generator, the next theorem should instantiate `hCoord`
by counting functions whose value at `j` lands in `coordinateBadScales`.
-/
theorem badPairDiagonalSeeds_card_le_of_coordinate_bound
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Seed]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) (pair : Secret × Secret) (j : ViewCoord)
    (pairBudget : Nat)
    (hCoord :
      (coordinateBadDiagonalSeeds (q := q) scale j
        (eval pair.1 (select j) - eval pair.2 (select j)) eta).card <=
          pairBudget) :
    (badPairDiagonalSeeds (q := q) select scale eval minSupport eta pair).card <=
      pairBudget :=
  Nat.le_trans
    (badPairDiagonalSeeds_card_le_coordinateBadDiagonalSeeds_card
      (q := q) select scale eval minSupport eta pair j)
    hCoord

/-- The concrete full-diagonal seed family uses all functions
`ViewCoord -> ZMod q`. -/
theorem fullDiagonalScale_apply [NeZero q]
    (seed : FullDiagonalSeed q ViewCoord) (j : ViewCoord) :
    fullDiagonalScale (q := q) seed j = seed j :=
  rfl

/--
Exact coordinate-fiber count for the full diagonal generator.

Choosing the scale at `j` from the bad scale set leaves all other coordinates
free.
-/
theorem coordinateBadDiagonalSeeds_full_card_eq
    [NeZero q] [Fintype ViewCoord] [DecidableEq ViewCoord]
    [DecidableEq (ZMod q)]
    (j : ViewCoord) (delta : ZMod q) (eta : Nat) :
    (coordinateBadDiagonalSeeds
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      (fullDiagonalScale (q := q)) j delta eta).card =
      (coordinateBadScales (q := q) delta eta).card *
        Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1) := by
  classical
  let allScales : ViewCoord -> Finset (ZMod q) :=
    fun _ => Finset.univ
  let badAtJ : ViewCoord -> Finset (ZMod q) :=
    Function.update allScales j (coordinateBadScales (q := q) delta eta)
  have hsubset :
      coordinateBadScales (q := q) delta eta ⊆
        (Finset.univ : Finset (ZMod q)) := by
    intro a _ha
    simp
  have hfilter :
      Fintype.piFinset badAtJ =
        {f ∈ Fintype.piFinset allScales | f j ∈
          coordinateBadScales (q := q) delta eta} := by
    simpa [badAtJ, allScales] using
      (Fintype.piFinset_update_eq_filter_piFinset_mem
        allScales j hsubset)
  have hleft :
      (coordinateBadDiagonalSeeds
        (q := q) (Seed := FullDiagonalSeed q ViewCoord)
        (fullDiagonalScale (q := q)) j delta eta) =
        {f ∈ Fintype.piFinset allScales | f j ∈
          coordinateBadScales (q := q) delta eta} := by
    ext f
    simp [coordinateBadDiagonalSeeds, coordinateBadScales,
      CoordinateBadScale, fullDiagonalScale, allScales]
  calc
    (coordinateBadDiagonalSeeds
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      (fullDiagonalScale (q := q)) j delta eta).card
        = (Fintype.piFinset badAtJ).card := by
          rw [hleft, hfilter]
    _ = ∏ i, (badAtJ i).card := Fintype.card_piFinset badAtJ
    _ = (coordinateBadScales (q := q) delta eta).card *
          ∏ i ∈ (Finset.univ : Finset ViewCoord) \ {j},
            (allScales i).card := by
          have hcardUpdate :
              (fun i => (badAtJ i).card) =
                Function.update (fun i => (allScales i).card) j
                  (coordinateBadScales (q := q) delta eta).card := by
            funext i
            by_cases hi : i = j
            · subst i
              simp [badAtJ]
            · simp [badAtJ, Function.update, hi]
          have hprod :=
            Finset.prod_update_of_mem
              (s := (Finset.univ : Finset ViewCoord)) (i := j)
              (f := fun i => (allScales i).card)
              (b := (coordinateBadScales (q := q) delta eta).card)
              (Finset.mem_univ j)
          rw [hcardUpdate]
          exact hprod
    _ = (coordinateBadScales (q := q) delta eta).card *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1) := by
          simp [allScales, Finset.prod_const,
            Finset.card_sdiff, Finset.card_singleton]

/--
Per-pair bad-seed bound for the full diagonal generator, reduced to counting
bad scale values at one selected coordinate.
-/
theorem badPairDiagonalSeeds_full_card_le_coordinateBadScales_mul
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [DecidableEq ViewCoord]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) (pair : Secret × Secret) (j : ViewCoord) :
    (badPairDiagonalSeeds
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta pair).card <=
      (coordinateBadScales (q := q)
        (eval pair.1 (select j) - eval pair.2 (select j)) eta).card *
        Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1) := by
  calc
    (badPairDiagonalSeeds
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta pair).card
        <= (coordinateBadDiagonalSeeds
          (q := q) (Seed := FullDiagonalSeed q ViewCoord)
          (fullDiagonalScale (q := q)) j
          (eval pair.1 (select j) - eval pair.2 (select j)) eta).card :=
          badPairDiagonalSeeds_card_le_coordinateBadDiagonalSeeds_card
            (q := q) select (fullDiagonalScale (q := q)) eval minSupport eta
            pair j
    _ = (coordinateBadScales (q := q)
          (eval pair.1 (select j) - eval pair.2 (select j)) eta).card *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1) :=
          coordinateBadDiagonalSeeds_full_card_eq
            (q := q) j
            (eval pair.1 (select j) - eval pair.2 (select j)) eta

/--
Per-pair bad-seed bound for the full diagonal generator when the selected
coordinate difference is a unit.

For prime-field coordinates this is the usual nonzero-difference case.  The
remaining combinatorial work is then to upper-bound the size of
`centeredSmallResidues`.
-/
theorem badPairDiagonalSeeds_full_card_le_centeredSmallResidues_mul_of_isUnit
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [DecidableEq ViewCoord]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) (pair : Secret × Secret) (j : ViewCoord)
    (hdelta :
      IsUnit (eval pair.1 (select j) - eval pair.2 (select j))) :
    (badPairDiagonalSeeds
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta pair).card <=
      (centeredSmallResidues (q := q) (2 * eta)).card *
        Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1) := by
  calc
    (badPairDiagonalSeeds
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta pair).card
        <= (coordinateBadScales (q := q)
          (eval pair.1 (select j) - eval pair.2 (select j)) eta).card *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1) :=
          badPairDiagonalSeeds_full_card_le_coordinateBadScales_mul
            (q := q) select eval minSupport eta pair j
    _ = (centeredSmallResidues (q := q) (2 * eta)).card *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1) := by
          rw [coordinateBadScales_card_eq_centeredSmallResidues_of_isUnit
            (q := q) hdelta eta]

/-- Fully numeric per-pair bound for a unit selected-coordinate difference. -/
theorem badPairDiagonalSeeds_full_card_le_interval_mul_of_isUnit
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [DecidableEq ViewCoord]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) (pair : Secret × Secret) (j : ViewCoord)
    (hdelta :
      IsUnit (eval pair.1 (select j) - eval pair.2 (select j))) :
    (badPairDiagonalSeeds
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta pair).card <=
      (2 * (2 * eta) + 1) *
        Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1) := by
  calc
    (badPairDiagonalSeeds
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta pair).card
        <= (centeredSmallResidues (q := q) (2 * eta)).card *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1) :=
          badPairDiagonalSeeds_full_card_le_centeredSmallResidues_mul_of_isUnit
            (q := q) select eval minSupport eta pair j hdelta
    _ <= (2 * (2 * eta) + 1) *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1) :=
          Nat.mul_le_mul_right
            (Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1))
            (centeredSmallResidues_card_le_interval (q := q) (2 * eta))

/--
Prime-field version of the numeric per-pair bound: nonzero selected-coordinate
differences are units.
-/
theorem badPairDiagonalSeeds_full_card_le_interval_mul_of_prime_nonzero
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [DecidableEq ViewCoord]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) (pair : Secret × Secret) (j : ViewCoord)
    (hdelta :
      eval pair.1 (select j) - eval pair.2 (select j) ≠ 0) :
    (badPairDiagonalSeeds
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta pair).card <=
      (2 * (2 * eta) + 1) *
        Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1) :=
  badPairDiagonalSeeds_full_card_le_interval_mul_of_isUnit
    (q := q) select eval minSupport eta pair j
    (isUnit_iff_ne_zero.mpr hdelta)

/--
Multi-coordinate full-diagonal bad-seed bound for one ordered secret pair.

If a pair difference is a unit on every selected coordinate in `coords`, then a
bad diagonal seed must choose a bad scale value on all those coordinates.  The
result keeps the final expression as a product over the viewed coordinate type;
this is the proof-carrying counterpart of the search diagnostic
`lossy_diagonal_slack_bits`.
-/
theorem badPairDiagonalSeeds_full_card_le_coordinateSet_product_of_units
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [DecidableEq ViewCoord]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) (pair : Secret × Secret)
    (coords : Finset ViewCoord)
    (hdelta :
      ∀ j ∈ coords,
        IsUnit (eval pair.1 (select j) - eval pair.2 (select j))) :
    (badPairDiagonalSeeds
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta pair).card <=
      ∏ j : ViewCoord,
        if j ∈ coords then
          2 * (2 * eta) + 1
        else
          Fintype.card (ZMod q) := by
  classical
  let coordBudget : ViewCoord -> Nat := fun j =>
    if j ∈ coords then
      2 * (2 * eta) + 1
    else
      Fintype.card (ZMod q)
  let badAt : ViewCoord -> Finset (ZMod q) := fun j =>
    if j ∈ coords then
      coordinateBadScales (q := q)
        (eval pair.1 (select j) - eval pair.2 (select j)) eta
    else
      Finset.univ
  have hsubset :
      badPairDiagonalSeeds
        (q := q) (Seed := FullDiagonalSeed q ViewCoord)
        select (fullDiagonalScale (q := q)) eval minSupport eta pair ⊆
        Fintype.piFinset badAt := by
    intro seed hseed
    rw [Fintype.mem_piFinset]
    intro j
    by_cases hj : j ∈ coords
    · have hCoord :
          seed ∈ coordinateBadDiagonalSeeds
            (q := q) (Seed := FullDiagonalSeed q ViewCoord)
            (fullDiagonalScale (q := q)) j
            (eval pair.1 (select j) - eval pair.2 (select j)) eta :=
        badPairDiagonalSeeds_subset_coordinateBadDiagonalSeeds
          (q := q) select (fullDiagonalScale (q := q)) eval minSupport eta pair j
          hseed
      simpa [badAt, hj] using (mem_coordinateBadScales_iff
        (q := q) (eval pair.1 (select j) - eval pair.2 (select j))
        (seed j) eta).mpr (by
          simpa [coordinateBadDiagonalSeeds, fullDiagonalScale] using hCoord)
    · simp [badAt, hj]
  calc
    (badPairDiagonalSeeds
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta pair).card
        <= (Fintype.piFinset badAt).card :=
          Finset.card_le_card hsubset
    _ = ∏ j : ViewCoord, (badAt j).card :=
          Fintype.card_piFinset badAt
    _ <= ∏ j : ViewCoord, coordBudget j := by
          refine Finset.prod_le_prod (fun j _hj => Nat.zero_le _) ?_
          intro j _hj
          by_cases hj : j ∈ coords
          · have hsmall :
                (badAt j).card <= 2 * (2 * eta) + 1 := by
              calc
                (badAt j).card
                    = (coordinateBadScales (q := q)
                        (eval pair.1 (select j) - eval pair.2 (select j))
                        eta).card := by
                      simp [badAt, hj]
                _ = (centeredSmallResidues (q := q) (2 * eta)).card := by
                      rw [coordinateBadScales_card_eq_centeredSmallResidues_of_isUnit
                        (q := q) (hdelta j hj) eta]
                _ <= 2 * (2 * eta) + 1 :=
                      centeredSmallResidues_card_le_interval (q := q) (2 * eta)
            simpa [coordBudget, hj] using hsmall
          · simp [badAt, coordBudget, hj]
    _ = ∏ j : ViewCoord,
        if j ∈ coords then
          2 * (2 * eta) + 1
        else
          Fintype.card (ZMod q) := rfl

/-- Prime-field version of the multi-coordinate pair bound. -/
theorem badPairDiagonalSeeds_full_card_le_coordinateSet_product_of_prime_nonzero
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [DecidableEq ViewCoord]
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) (pair : Secret × Secret)
    (coords : Finset ViewCoord)
    (hdelta :
      ∀ j ∈ coords,
        eval pair.1 (select j) - eval pair.2 (select j) ≠ 0) :
    (badPairDiagonalSeeds
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta pair).card <=
      ∏ j : ViewCoord,
        if j ∈ coords then
          2 * (2 * eta) + 1
        else
          Fintype.card (ZMod q) :=
  badPairDiagonalSeeds_full_card_le_coordinateSet_product_of_units
    (q := q) select eval minSupport eta pair coords
    (by
      intro j hj
      exact isUnit_iff_ne_zero.mpr (hdelta j hj))

/-- Every globally bad seed is bad for at least one ordered secret pair. -/
theorem badDiagonalSeeds_subset_pairUnion
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [Fintype Seed] [DecidableEq Seed] [DecidableEq FullCoord]
    [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) :
    badDiagonalSeeds (q := q) select scale eval minSupport eta ⊆
      (Finset.univ.product (Finset.univ : Finset Secret)).biUnion
        (badPairDiagonalSeeds (q := q) select scale eval minSupport eta) := by
  classical
  intro seed hseed
  have hbad :
      ¬ DiagonalAmplifiesSupport (q := q) select (scale seed) eval
        minSupport eta := by
    simpa [badDiagonalSeeds, GoodDiagonalSeed] using hseed
  simp [DiagonalAmplifiesSupport] at hbad
  rcases hbad with ⟨s, t, hst, hSupport, hFail⟩
  refine Finset.mem_biUnion.mpr ?_
  refine ⟨(s, t), ?_, ?_⟩
  · simp
  · simp [badPairDiagonalSeeds, BadPairDiagonalSeed, hst, hSupport, hFail]

/-- Union bound over ordered secret pairs for the diagonal bad event. -/
theorem badDiagonalSeeds_card_le_sum_badPairDiagonalSeeds
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [Fintype Seed] [DecidableEq Seed] [DecidableEq FullCoord]
    [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) :
    (badDiagonalSeeds (q := q) select scale eval minSupport eta).card <=
      ∑ pair ∈ Finset.univ.product (Finset.univ : Finset Secret),
        (badPairDiagonalSeeds (q := q) select scale eval minSupport eta pair).card := by
  classical
  calc
    (badDiagonalSeeds (q := q) select scale eval minSupport eta).card
        <= ((Finset.univ.product (Finset.univ : Finset Secret)).biUnion
              (badPairDiagonalSeeds (q := q) select scale eval minSupport eta)).card :=
          Finset.card_le_card
            (badDiagonalSeeds_subset_pairUnion
              (q := q) select scale eval minSupport eta)
    _ <= ∑ pair ∈ Finset.univ.product (Finset.univ : Finset Secret),
          (badPairDiagonalSeeds (q := q) select scale eval minSupport eta pair).card :=
          Finset.card_biUnion_le

/--
Pairwise bad-event estimates imply the global finite bad-seed bound.

This is the Lean-facing union-bound target for the full diagonal generator:
prove a concrete anti-concentration estimate for each ordered secret pair, then
sum those estimates below the seed-space size.
-/
theorem failureCountAtMost_of_pairwise_badPair_bounds
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [Fintype Seed] [DecidableEq Seed] [DecidableEq FullCoord]
    [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta failureBudget : Nat)
    (pairBudget : Secret × Secret -> Nat)
    (hPair :
      ∀ pair,
        (badPairDiagonalSeeds (q := q) select scale eval minSupport eta pair).card <=
          pairBudget pair)
    (hSum :
      (∑ pair ∈ Finset.univ.product (Finset.univ : Finset Secret),
        pairBudget pair) <= failureBudget) :
    DiagonalAmplificationFailureCountAtMost (q := q) select scale eval
      minSupport eta failureBudget := by
  calc
    (badDiagonalSeeds (q := q) select scale eval minSupport eta).card
        <= ∑ pair ∈ Finset.univ.product (Finset.univ : Finset Secret),
          (badPairDiagonalSeeds (q := q) select scale eval minSupport eta pair).card :=
          badDiagonalSeeds_card_le_sum_badPairDiagonalSeeds
            (q := q) select scale eval minSupport eta
    _ <= ∑ pair ∈ Finset.univ.product (Finset.univ : Finset Secret),
          pairBudget pair := by
          exact Finset.sum_le_sum (by
            intro pair _hpair
            exact hPair pair)
    _ <= failureBudget := hSum

/--
Global full-diagonal failure bound from per-pair multi-coordinate witnesses.

For each ordered secret pair, `pairCoords pair` names coordinates where the
pair difference is nonzero.  The budget hypothesis is exactly the summed
multi-coordinate bad-scale product.  This theorem is the Lean-facing target for
turning the validator's positive `lossy_diagonal_slack_bits` rows into actual
proof-carrying diagonal evidence.
-/
theorem failureCountAtMost_full_of_pairwise_coordinateSets_prime
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta failureBudget : Nat)
    (pairCoords : Secret × Secret -> Finset ViewCoord)
    (hdelta :
      ∀ pair j,
        j ∈ pairCoords pair ->
          eval pair.1 (select j) - eval pair.2 (select j) ≠ 0)
    (hBudget :
      (∑ pair ∈ Finset.univ.product (Finset.univ : Finset Secret),
        ∏ j : ViewCoord,
          if j ∈ pairCoords pair then
            2 * (2 * eta) + 1
          else
            Fintype.card (ZMod q)) <= failureBudget) :
    DiagonalAmplificationFailureCountAtMost
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta
      failureBudget := by
  refine failureCountAtMost_of_pairwise_badPair_bounds
    (q := q) (Seed := FullDiagonalSeed q ViewCoord)
    select (fullDiagonalScale (q := q)) eval minSupport eta failureBudget
    (fun pair =>
      ∏ j : ViewCoord,
        if j ∈ pairCoords pair then
          2 * (2 * eta) + 1
        else
          Fintype.card (ZMod q))
    ?_ hBudget
  intro pair
  exact badPairDiagonalSeeds_full_card_le_coordinateSet_product_of_prime_nonzero
    (q := q) select eval minSupport eta pair (pairCoords pair)
    (by
      intro j hj
      exact hdelta pair j hj)

/--
Uniform-budget version of the multi-coordinate diagonal bound.

This is often the most convenient certificate shape for generated parameter
rows: a checker proves every ordered secret pair has a product bound at most
`pairBudget`, then verifies `|Secret|^2 * pairBudget <= failureBudget`.
-/
theorem failureCountAtMost_full_of_pairwise_coordinateSets_uniform_prime
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta failureBudget pairBudget : Nat)
    (pairCoords : Secret × Secret -> Finset ViewCoord)
    (hdelta :
      ∀ pair j,
        j ∈ pairCoords pair ->
          eval pair.1 (select j) - eval pair.2 (select j) ≠ 0)
    (hPairBudget :
      ∀ pair,
        (∏ j : ViewCoord,
          if j ∈ pairCoords pair then
            2 * (2 * eta) + 1
          else
            Fintype.card (ZMod q)) <= pairBudget)
    (hBudget :
      ((Finset.univ : Finset Secret).product
        (Finset.univ : Finset Secret)).card * pairBudget <= failureBudget) :
    DiagonalAmplificationFailureCountAtMost
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta
      failureBudget := by
  refine failureCountAtMost_full_of_pairwise_coordinateSets_prime
    (q := q) select eval minSupport eta failureBudget pairCoords hdelta ?_
  calc
    (∑ pair ∈ Finset.univ.product (Finset.univ : Finset Secret),
      ∏ j : ViewCoord,
        if j ∈ pairCoords pair then
          2 * (2 * eta) + 1
        else
          Fintype.card (ZMod q))
        <= ∑ _pair ∈ Finset.univ.product (Finset.univ : Finset Secret),
          pairBudget := by
          exact Finset.sum_le_sum (by
            intro pair _hpair
            exact hPairBudget pair)
    _ = ((Finset.univ : Finset Secret).product
          (Finset.univ : Finset Secret)).card * pairBudget := by
          simp
    _ <= failureBudget := hBudget

/--
Global full-diagonal failure bound from the selected-coordinate unit witness
hypothesis.

This is the first end-to-end probabilistic counting theorem in this file.  The
remaining project-specific work is to prove the hypothesis from the sample-view
geometry: every secret pair above the support threshold must be separated on at
least one viewed coordinate by a unit field element.
-/
theorem failureCountAtMost_full_of_pairwise_selected_unit_differences
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta failureBudget : Nat)
    (hSelectedUnit :
      ∀ pair : Secret × Secret,
        pair.1 ≠ pair.2 ->
        minSupport <=
          weight (fun i : FullCoord => eval pair.1 i - eval pair.2 i) ->
        ∃ j : ViewCoord,
          IsUnit (eval pair.1 (select j) - eval pair.2 (select j)))
    (hBudget :
      ((Finset.univ : Finset Secret).product
        (Finset.univ : Finset Secret)).card *
        ((2 * (2 * eta) + 1) *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)) <=
        failureBudget) :
    DiagonalAmplificationFailureCountAtMost
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta
      failureBudget := by
  classical
  let scalarBudget : Nat :=
    (2 * (2 * eta) + 1) *
      Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)
  refine failureCountAtMost_of_pairwise_badPair_bounds
    (q := q) (Seed := FullDiagonalSeed q ViewCoord)
    select (fullDiagonalScale (q := q)) eval minSupport eta failureBudget
    (fun _pair => scalarBudget) ?_ ?_
  · intro pair
    by_cases hst : pair.1 ≠ pair.2
    · by_cases hSupport :
        minSupport <=
          weight (fun i : FullCoord => eval pair.1 i - eval pair.2 i)
      · rcases hSelectedUnit pair hst hSupport with ⟨j, hdelta⟩
        exact badPairDiagonalSeeds_full_card_le_interval_mul_of_isUnit
          (q := q) select eval minSupport eta pair j hdelta
      · have hempty :
            badPairDiagonalSeeds
              (q := q) (Seed := FullDiagonalSeed q ViewCoord)
              select (fullDiagonalScale (q := q)) eval minSupport eta pair =
              ∅ := by
          ext seed
          simp [badPairDiagonalSeeds, BadPairDiagonalSeed, hSupport]
        rw [hempty]
        simp [scalarBudget]
    · have hempty :
          badPairDiagonalSeeds
            (q := q) (Seed := FullDiagonalSeed q ViewCoord)
            select (fullDiagonalScale (q := q)) eval minSupport eta pair =
            ∅ := by
        ext seed
        simp [badPairDiagonalSeeds, BadPairDiagonalSeed, hst]
      rw [hempty]
      simp [scalarBudget]
  · simpa [scalarBudget, Finset.card_product] using hBudget

/-- Prime-field global version: selected nonzero coordinate differences are
units. -/
theorem failureCountAtMost_full_of_pairwise_selected_nonzero_differences_prime
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta failureBudget : Nat)
    (hSelectedNonzero :
      ∀ pair : Secret × Secret,
        pair.1 ≠ pair.2 ->
        minSupport <=
          weight (fun i : FullCoord => eval pair.1 i - eval pair.2 i) ->
        ∃ j : ViewCoord,
          eval pair.1 (select j) - eval pair.2 (select j) ≠ 0)
    (hBudget :
      ((Finset.univ : Finset Secret).product
        (Finset.univ : Finset Secret)).card *
        ((2 * (2 * eta) + 1) *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)) <=
        failureBudget) :
    DiagonalAmplificationFailureCountAtMost
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta
      failureBudget :=
  failureCountAtMost_full_of_pairwise_selected_unit_differences
    (q := q) select eval minSupport eta failureBudget
    (by
      intro pair hst hSupport
      rcases hSelectedNonzero pair hst hSupport with ⟨j, hdelta⟩
      exact ⟨j, isUnit_iff_ne_zero.mpr hdelta⟩)
    hBudget

/--
Prime-field full-diagonal failure bound for a surjective selected-coordinate
view.

This discharges the remaining selected-nonzero hypothesis in the uncompressed
or support-covering case: if `select` hits every full coordinate and
`minSupport > 0`, then every support-heavy pair has a viewed coordinate with a
nonzero difference.
-/
theorem failureCountAtMost_full_of_surjective_select_prime
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta failureBudget : Nat)
    (hselect : Function.Surjective select)
    (hMinSupport : 0 < minSupport)
    (hBudget :
      ((Finset.univ : Finset Secret).product
        (Finset.univ : Finset Secret)).card *
        ((2 * (2 * eta) + 1) *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)) <=
        failureBudget) :
    DiagonalAmplificationFailureCountAtMost
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta
      failureBudget :=
  failureCountAtMost_full_of_pairwise_selected_nonzero_differences_prime
    (q := q) select eval minSupport eta failureBudget
    (exists_selected_pair_nonzero_of_surjective
      (q := q) select eval minSupport hselect hMinSupport)
    hBudget

/--
Prime-field full-diagonal failure bound for a compressed selected-coordinate
view whose selected coordinates cover every relevant pair-support.

This is the compression-facing version of the diagonal counting theorem.  It
does not require `select` to be surjective onto all full coordinates; it only
requires support coverage for the support-heavy secret-pair differences that
enter the correctness proof.
-/
theorem failureCountAtMost_full_of_supportCovered_select_prime
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta failureBudget : Nat)
    (hMinSupport : 0 < minSupport)
    (hCover :
      ∀ pair : Secret × Secret,
        pair.1 ≠ pair.2 ->
        minSupport <=
          weight (fun i : FullCoord => eval pair.1 i - eval pair.2 i) ->
        SupportCoveredByView (q := q) select
          (fun i : FullCoord => eval pair.1 i - eval pair.2 i))
    (hBudget :
      ((Finset.univ : Finset Secret).product
        (Finset.univ : Finset Secret)).card *
        ((2 * (2 * eta) + 1) *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)) <=
        failureBudget) :
    DiagonalAmplificationFailureCountAtMost
      (q := q) (Seed := FullDiagonalSeed q ViewCoord)
      select (fullDiagonalScale (q := q)) eval minSupport eta
      failureBudget :=
  failureCountAtMost_full_of_pairwise_selected_nonzero_differences_prime
    (q := q) select eval minSupport eta failureBudget
    (exists_selected_pair_nonzero_of_supportCoveredByView
      (q := q) select eval minSupport hMinSupport hCover)
    hBudget

/-- If the bad diagonal seeds are fewer than all seeds, some seed is good. -/
theorem exists_good_diagonal_of_badDiagonalSeeds_card_lt
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Seed]
    [DecidableEq Seed] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat)
    (hbad :
      (badDiagonalSeeds (q := q) select scale eval minSupport eta).card <
        Fintype.card Seed) :
    ∃ seed,
      GoodDiagonalSeed (q := q) select scale eval minSupport eta seed := by
  classical
  by_contra hnoneExists
  have hnone :
      ∀ seed,
        ¬ GoodDiagonalSeed (q := q) select scale eval minSupport eta seed := by
    intro seed hgood
    exact hnoneExists ⟨seed, hgood⟩
  have hAll :
      badDiagonalSeeds (q := q) select scale eval minSupport eta =
        (Finset.univ : Finset Seed) := by
    ext seed
    simp [badDiagonalSeeds, hnone seed]
  have hcard :
      (badDiagonalSeeds (q := q) select scale eval minSupport eta).card =
        Fintype.card Seed := by
    simp [hAll]
  have hlt : Fintype.card Seed < Fintype.card Seed := by
    rw [hcard] at hbad
    exact hbad
  exact (Nat.lt_irrefl _) hlt

/--
A finite bad-seed bound below the seed-space size yields a good diagonal.

This is the direct target for a later randomized diagonal theorem.
-/
theorem exists_good_diagonal_of_failureCountAtMost
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Seed]
    [DecidableEq Seed] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta failureBudget : Nat)
    (hBound :
      DiagonalAmplificationFailureCountAtMost (q := q) select scale eval
        minSupport eta failureBudget)
    (hBudget : failureBudget < Fintype.card Seed) :
    ∃ seed,
      GoodDiagonalSeed (q := q) select scale eval minSupport eta seed := by
  exact exists_good_diagonal_of_badDiagonalSeeds_card_lt
    (q := q) select scale eval minSupport eta
    (Nat.lt_of_le_of_lt hBound hBudget)

/--
Finite-counting bridge from a sampled diagonal family to an actual dense-noise
P-TND correctness certificate.

The theorem deliberately keeps the noise side explicit: the caller must prove
that every admissible seed, or at least the selected good seed by strengthening
this hypothesis later, gives a small viewed noise distribution.
-/
theorem exists_scaledCoordinateRestriction_correctnessCertificate_of_failureCount
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Seed]
    [DecidableEq Seed] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport eta failureBudget : Nat)
    (hNoise :
      ∀ seed,
        (scaledCoordinateRestriction (q := q) select (scale seed)).NoiseSmall
          P eta)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hBound :
      DiagonalAmplificationFailureCountAtMost (q := q) select scale P.eval
        minSupport eta failureBudget)
    (hBudget : failureBudget < Fintype.card Seed) :
    Nonempty (Σ _seed : Seed,
      PTNDCorrectnessCertificate q ViewCoord Secret Noise) := by
  rcases exists_good_diagonal_of_failureCountAtMost
      (q := q) select scale P.eval minSupport eta failureBudget hBound hBudget with
    ⟨seed, hGood⟩
  exact ⟨⟨seed,
      scaledCoordinateRestriction_correctnessCertificate_of_amplifies
        (q := q) select (scale seed) P minSupport eta (hNoise seed)
        hSupport hGood⟩⟩

/--
Finite-counting bridge for lossy public views.

This is the budgeted version of
`exists_scaledCoordinateRestriction_correctnessCertificate_of_failureCount`.
The bad-diagonal event is tested against the viewed radius `B.viewEta`, while
the amount of bad seed space is charged to `B.failureBudget`.
-/
theorem exists_scaledCoordinateRestriction_correctnessCertificateWithBudget_of_failureCount
    [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] [Fintype Seed]
    [DecidableEq Seed] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (scale : Seed -> ViewCoord -> ZMod q)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport : Nat) (B : ViewReductionBudget)
    (hNoise :
      ∀ seed,
        (scaledCoordinateRestriction (q := q) select (scale seed)).NoiseSmallWithBudget
          P B)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hBound :
      DiagonalAmplificationFailureCountAtMost (q := q) select scale P.eval
        minSupport B.viewEta B.failureBudget)
    (hBudget : B.failureBudget < Fintype.card Seed) :
    Nonempty (Σ _seed : Seed,
      PTNDCorrectnessCertificate q ViewCoord Secret Noise) := by
  rcases exists_good_diagonal_of_failureCountAtMost
      (q := q) select scale P.eval minSupport B.viewEta B.failureBudget
      hBound hBudget with
    ⟨seed, hGood⟩
  exact ⟨⟨seed,
      scaledCoordinateRestriction_correctnessCertificateWithBudget_of_amplifies
        (q := q) select (scale seed) P minSupport B (hNoise seed)
        hSupport hGood⟩⟩

/--
End-to-end full-diagonal correctness existence theorem for a prime field and a
surjective selected-coordinate view.

This packages the current proof stack:
support separation + surjective view + scalar anti-concentration + union bound
below the seed-space size gives an actual public diagonal seed and a
scaled-view P-TND correctness certificate.
-/
theorem exists_scaledCoordinateRestriction_correctnessCertificate_full_of_surjective_select_prime
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport eta failureBudget : Nat)
    (hselect : Function.Surjective select)
    (hMinSupport : 0 < minSupport)
    (hNoise :
      ∀ seed : FullDiagonalSeed q ViewCoord,
        (scaledCoordinateRestriction (q := q) select (fullDiagonalScale (q := q) seed)).NoiseSmall
          P eta)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hFailureBudget :
      ((Finset.univ : Finset Secret).product
        (Finset.univ : Finset Secret)).card *
        ((2 * (2 * eta) + 1) *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)) <=
        failureBudget)
    (hBudget : failureBudget < Fintype.card (FullDiagonalSeed q ViewCoord)) :
    Nonempty (Σ _seed : FullDiagonalSeed q ViewCoord,
      PTNDCorrectnessCertificate q ViewCoord Secret Noise) := by
  refine exists_scaledCoordinateRestriction_correctnessCertificate_of_failureCount
    (q := q) (Seed := FullDiagonalSeed q ViewCoord)
    select (fullDiagonalScale (q := q)) P minSupport eta failureBudget
    hNoise hSupport ?_ hBudget
  exact failureCountAtMost_full_of_surjective_select_prime
    (q := q) select P.eval minSupport eta failureBudget
    hselect hMinSupport hFailureBudget

/--
End-to-end compressed-view correctness existence theorem for a prime field.

The selected coordinate view may be genuinely smaller than the full coordinate
set.  The caller must prove that it covers the support of every relevant
secret-pair difference, plus the usual support-distance, viewed-noise, and
finite bad-budget hypotheses.
-/
theorem exists_scaledCoordinateRestriction_correctnessCertificate_full_of_supportCovered_select_prime
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport eta failureBudget : Nat)
    (hMinSupport : 0 < minSupport)
    (hCover :
      ∀ pair : Secret × Secret,
        pair.1 ≠ pair.2 ->
        minSupport <=
          weight (fun i : FullCoord => P.eval pair.1 i - P.eval pair.2 i) ->
        SupportCoveredByView (q := q) select
          (fun i : FullCoord => P.eval pair.1 i - P.eval pair.2 i))
    (hNoise :
      ∀ seed : FullDiagonalSeed q ViewCoord,
        (scaledCoordinateRestriction (q := q) select (fullDiagonalScale (q := q) seed)).NoiseSmall
          P eta)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hFailureBudget :
      ((Finset.univ : Finset Secret).product
        (Finset.univ : Finset Secret)).card *
        ((2 * (2 * eta) + 1) *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)) <=
        failureBudget)
    (hBudget : failureBudget < Fintype.card (FullDiagonalSeed q ViewCoord)) :
    Nonempty (Σ _seed : FullDiagonalSeed q ViewCoord,
      PTNDCorrectnessCertificate q ViewCoord Secret Noise) := by
  refine exists_scaledCoordinateRestriction_correctnessCertificate_of_failureCount
    (q := q) (Seed := FullDiagonalSeed q ViewCoord)
    select (fullDiagonalScale (q := q)) P minSupport eta failureBudget
    hNoise hSupport ?_ hBudget
  exact failureCountAtMost_full_of_supportCovered_select_prime
    (q := q) select P.eval minSupport eta failureBudget
    hMinSupport hCover hFailureBudget

/--
End-to-end compressed-view correctness theorem with an explicit lossy view
budget.

This packages the support-covered diagonal route for the case used by the
parameter search: sample quantization or view compression raises the effective
noise radius to `B.viewEta`, and the diagonal bad-seed union bound is charged to
`B.failureBudget`.
-/
theorem exists_scaledCoordinateRestriction_correctnessCertificateWithBudget_full_of_supportCovered_select_prime
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport : Nat) (B : ViewReductionBudget)
    (hMinSupport : 0 < minSupport)
    (hCover :
      ∀ pair : Secret × Secret,
        pair.1 ≠ pair.2 ->
        minSupport <=
          weight (fun i : FullCoord => P.eval pair.1 i - P.eval pair.2 i) ->
        SupportCoveredByView (q := q) select
          (fun i : FullCoord => P.eval pair.1 i - P.eval pair.2 i))
    (hNoise :
      ∀ seed : FullDiagonalSeed q ViewCoord,
        (scaledCoordinateRestriction (q := q) select (fullDiagonalScale (q := q) seed)).NoiseSmallWithBudget
          P B)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hFailureBudget :
      ((Finset.univ : Finset Secret).product
        (Finset.univ : Finset Secret)).card *
        ((2 * (2 * B.viewEta) + 1) *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)) <=
        B.failureBudget)
    (hBudget : B.failureBudget < Fintype.card (FullDiagonalSeed q ViewCoord)) :
    Nonempty (Σ _seed : FullDiagonalSeed q ViewCoord,
      PTNDCorrectnessCertificate q ViewCoord Secret Noise) := by
  refine exists_scaledCoordinateRestriction_correctnessCertificateWithBudget_of_failureCount
    (q := q) (Seed := FullDiagonalSeed q ViewCoord)
    select (fullDiagonalScale (q := q)) P minSupport B hNoise hSupport ?_ hBudget
  exact failureCountAtMost_full_of_supportCovered_select_prime
    (q := q) select P.eval minSupport B.viewEta B.failureBudget
    hMinSupport hCover hFailureBudget

/-- Budgeted full-view/surjective specialization of the lossy diagonal route. -/
theorem exists_scaledCoordinateRestriction_correctnessCertificateWithBudget_full_of_surjective_select_prime
    [NeZero q] [Fact q.Prime]
    [Fintype FullCoord] [Fintype ViewCoord] [Fintype Secret]
    [DecidableEq ViewCoord] [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport : Nat) (B : ViewReductionBudget)
    (hselect : Function.Surjective select)
    (hMinSupport : 0 < minSupport)
    (hNoise :
      ∀ seed : FullDiagonalSeed q ViewCoord,
        (scaledCoordinateRestriction (q := q) select (fullDiagonalScale (q := q) seed)).NoiseSmallWithBudget
          P B)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hFailureBudget :
      ((Finset.univ : Finset Secret).product
        (Finset.univ : Finset Secret)).card *
        ((2 * (2 * B.viewEta) + 1) *
          Fintype.card (ZMod q) ^ (Fintype.card ViewCoord - 1)) <=
        B.failureBudget)
    (hBudget : B.failureBudget < Fintype.card (FullDiagonalSeed q ViewCoord)) :
    Nonempty (Σ _seed : FullDiagonalSeed q ViewCoord,
      PTNDCorrectnessCertificate q ViewCoord Secret Noise) := by
  refine exists_scaledCoordinateRestriction_correctnessCertificateWithBudget_of_failureCount
    (q := q) (Seed := FullDiagonalSeed q ViewCoord)
    select (fullDiagonalScale (q := q)) P minSupport B hNoise hSupport ?_ hBudget
  exact failureCountAtMost_full_of_surjective_select_prime
    (q := q) select P.eval minSupport B.viewEta B.failureBudget
    hselect hMinSupport hFailureBudget

end SampleView

end Descender
