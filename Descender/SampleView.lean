import Descender.Advantage
import Descender.Security

namespace Descender

/-!
Proof-facing interface for publishing only a view of a P-TND sample.

The KEM does not intrinsically need to serialize a sample as a raw vector over
the original measurement coordinate type.  What matters is the public
coordinate system that encapsulation/adversaries can query.  `SampleView`
records an additive map from full measurement samples to that serialized view.

This file deliberately does not claim that any particular compressed view is
secure.  It isolates the exact proof obligations:

* the viewed noise must remain bounded;
* the viewed operator must still separate secrets;
* any distinguisher on the viewed sample pulls back to a distinguisher on the
  full sample.
-/

variable {q : Nat} [NeZero q]
variable {FullCoord ViewCoord Secret Noise : Type*}

/-- A public additive view of full measurement samples. -/
structure SampleView (q : Nat) [NeZero q]
    (FullCoord ViewCoord : Type*) where
  map : (FullCoord -> ZMod q) -> ViewCoord -> ZMod q
  map_add :
    ∀ x y, map (fun i => x i + y i) = fun j => map x j + map y j

namespace SampleView

variable [Fintype FullCoord] [Fintype ViewCoord]

/--
Bookkeeping for a non-free sample-view reduction.

`fullEta` is the original noise radius, `viewEta` is the effective radius after
viewing/rounding/compression, and `failureBudget` is a placeholder for any
future probabilistic loss in the view construction.  The current deterministic
theorems use `viewEta`; later probability layers can charge `failureBudget`.
-/
structure ViewReductionBudget where
  fullEta : Nat
  viewEta : Nat
  failureBudget : Nat
  eta_le_viewEta : fullEta <= viewEta

namespace ViewReductionBudget

def extraNoise (B : ViewReductionBudget) : Nat :=
  B.viewEta - B.fullEta

end ViewReductionBudget

/--
The basic concrete sample view: publish/query only coordinates selected by
`select`.

Mathematically this is the view associated to restricting encapsulation
challenges to the span of the selected coordinate functionals.
-/
def coordinateRestriction (select : ViewCoord -> FullCoord) :
    SampleView q FullCoord ViewCoord where
  map := fun x j => x (select j)
  map_add := by
    intro x y
    rfl

/--
Coordinate restriction followed by public diagonal scaling.

This is the deterministic interface for the support-to-amplitude bridge:
support distance will be supplied by Cauchy/MDS-style arguments, while a public
diagonal is later shown probabilistically to amplify every nonzero support-heavy
difference above the centered `L∞` threshold.
-/
def scaledCoordinateRestriction (select : ViewCoord -> FullCoord)
    (scale : ViewCoord -> ZMod q) :
    SampleView q FullCoord ViewCoord where
  map := fun x j => scale j * x (select j)
  map_add := by
    intro x y
    funext j
    ring

omit [Fintype FullCoord] [Fintype ViewCoord] in
@[simp]
theorem coordinateRestriction_apply (select : ViewCoord -> FullCoord)
    (x : FullCoord -> ZMod q) (j : ViewCoord) :
    (coordinateRestriction (q := q) select).map x j = x (select j) :=
  rfl

omit [Fintype FullCoord] [Fintype ViewCoord] in
@[simp]
theorem scaledCoordinateRestriction_apply
    (select : ViewCoord -> FullCoord) (scale : ViewCoord -> ZMod q)
    (x : FullCoord -> ZMod q) (j : ViewCoord) :
    (scaledCoordinateRestriction (q := q) select scale).map x j =
      scale j * x (select j) :=
  rfl

/-- Restrict a decision P-TND instance to the serialized sample view. -/
def restrictDecision (V : SampleView q FullCoord ViewCoord)
    (P : P_TND_Decision q FullCoord Secret Noise) :
    P_TND_Decision q ViewCoord Secret Noise where
  eval := fun s => V.map (P.eval s)
  noise := fun e => V.map (P.noise e)
  noiseSupport := P.noiseSupport
  nullSample := V.map P.nullSample

theorem restrictDecision_realSample
    (V : SampleView q FullCoord ViewCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (s : Secret) (e : Noise) :
    (V.restrictDecision P).realSample s e = V.map (P.realSample s e) := by
  funext j
  exact (congrFun (V.map_add (P.eval s) (P.noise e)) j).symm

/-- A viewed-sample adversary can be run against full samples by precomposing
with the public view map. -/
def pullbackAdversary (V : SampleView q FullCoord ViewCoord)
    (A : (ViewCoord -> ZMod q) -> Bool) :
    (FullCoord -> ZMod q) -> Bool :=
  fun t => A (V.map t)

/--
If an adversary distinguishes the viewed decision problem, then the pulled-back
adversary distinguishes the full decision problem.

Consequently, a viewed public key is not harder to distinguish than the full
public sample.  The converse is intentionally absent: publishing less data may
lose the structure needed for correctness or security unless the viewed
operator is separately certified.
-/
theorem distinguishes_pullback_of_restrict
    (V : SampleView q FullCoord ViewCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    {A : (ViewCoord -> ZMod q) -> Bool}
    (hA : (V.restrictDecision P).Distinguishes A) :
    P.Distinguishes (V.pullbackAdversary A) := by
  constructor
  · intro t ht
    rcases ht with ⟨s, e, he, rfl⟩
    have hReal :
        A ((V.restrictDecision P).realSample s e) = true :=
      hA.1 ((V.restrictDecision P).realSample s e) ⟨s, e, he, rfl⟩
    simpa [pullbackAdversary, restrictDecision_realSample] using hReal
  · intro t ht
    subst t
    have hNull :
        A ((V.restrictDecision P).nullSample) = false :=
      hA.2 ((V.restrictDecision P).nullSample) rfl
    simpa [pullbackAdversary, restrictDecision] using hNull

/--
Advantage-game version of `restrictDecision`.

The viewed experiment is the full real-vs-null experiment followed by the
deterministic public map `V.map`.  This is the formal data-processing boundary
for public serialization/rounding: an adversary on viewed samples is exactly a
pulled-back adversary on full samples.
-/
def restrictExperiment (V : SampleView q FullCoord ViewCoord)
    (E : Advantage.PTNDExperiment q FullCoord Secret Noise) :
    Advantage.PTNDExperiment q ViewCoord Secret Noise where
  problem := V.restrictDecision E.problem
  game := E.game.map V.map

theorem restrictExperiment_advantage_eq_pullback
    (V : SampleView q FullCoord ViewCoord)
    (E : Advantage.PTNDExperiment q FullCoord Secret Noise)
    (A : (ViewCoord -> ZMod q) -> Bool) :
    (V.restrictExperiment E).advantage A =
      E.advantage (V.pullbackAdversary A) :=
  rfl

theorem restrictExperiment_secureAgainst_of_full
    (V : SampleView q FullCoord ViewCoord)
    (E : Advantage.PTNDExperiment q FullCoord Secret Noise)
    {Adversary : Type*}
    (run : Adversary -> (ViewCoord -> ZMod q) -> Bool)
    (allowed : Adversary -> Prop) {ε : ℝ}
    (hSecure :
      E.game.SecureAgainst (fun A t => run A (V.map t)) allowed ε) :
    (V.restrictExperiment E).game.SecureAgainst run allowed ε :=
  E.game.secureAgainst_map_of_source_secure V.map run allowed hSecure

/-- The viewed noise bound required for correctness of a compressed sample. -/
def NoiseSmall (V : SampleView q FullCoord ViewCoord)
    (P : P_TND_Decision q FullCoord Secret Noise) (eta : Nat) : Prop :=
  ∀ e, P.noiseSupport e ->
    centeredResidueNoiseSupport q eta (V.map (P.noise e))

def NoiseSmallWithBudget (V : SampleView q FullCoord ViewCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (B : ViewReductionBudget) : Prop :=
  V.NoiseSmall P B.viewEta

/--
The viewed separation condition required for unique decoding over the
serialized coordinate type.

This is the main proof obligation for any quotient/challenge-space
optimization: the view must still separate distinct secrets by more than twice
the viewed noise radius.
-/
def EvalSeparated (V : SampleView q FullCoord ViewCoord)
    (P : P_TND_Decision q FullCoord Secret Noise) (eta : Nat) : Prop :=
  ∀ s t, s ≠ t ->
    2 * eta < centeredNormLinf q
      (fun j => V.map (P.eval s) j - V.map (P.eval t) j)

def EvalSeparatedWithBudget (V : SampleView q FullCoord ViewCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (B : ViewReductionBudget) : Prop :=
  V.EvalSeparated P B.viewEta

/--
Build the existing correctness certificate for the restricted P-TND instance
from exactly the two obligations a sample view must preserve.
-/
def correctnessCertificate
    (V : SampleView q FullCoord ViewCoord)
    (P : P_TND_Decision q FullCoord Secret Noise) (eta : Nat)
    (hNoise : V.NoiseSmall P eta)
    (hSep : V.EvalSeparated P eta) :
    PTNDCorrectnessCertificate q ViewCoord Secret Noise where
  problem := V.restrictDecision P
  eta := eta
  noiseBound := by
    intro e he
    exact hNoise e he
  separated := by
    intro _s _e _he s t hst
    exact hSep s t hst

/--
Correctness certificate with an explicit non-free view budget.

This is the intended theorem target for quantized or lossy sample views: prove
noise and separation at `B.viewEta`, then record the extra radius and future
probabilistic loss in `B`.
-/
def correctnessCertificateWithBudget
    (V : SampleView q FullCoord ViewCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (B : ViewReductionBudget)
    (hNoise : V.NoiseSmallWithBudget P B)
    (hSep : V.EvalSeparatedWithBudget P B) :
    PTNDCorrectnessCertificate q ViewCoord Secret Noise :=
  V.correctnessCertificate P B.viewEta hNoise hSep

theorem coordinateRestriction_noiseSmallWithBudget_of_full
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (B : ViewReductionBudget)
    (hNoise :
      ∀ e, P.noiseSupport e ->
        centeredResidueNoiseSupport q B.fullEta (P.noise e)) :
    (coordinateRestriction (q := q) select).NoiseSmallWithBudget P B := by
  intro e he j
  exact Nat.le_trans (hNoise e he (select j)) B.eta_le_viewEta

theorem coordinateRestriction_evalSeparatedWithBudget_of_selected_linf
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (B : ViewReductionBudget)
    (hSep :
      ∀ s t, s ≠ t ->
        2 * B.viewEta < centeredNormLinf q
          (fun j : ViewCoord => P.eval s (select j) - P.eval t (select j))) :
    (coordinateRestriction (q := q) select).EvalSeparatedWithBudget P B := by
  intro s t hst
  exact hSep s t hst

def coordinateRestriction_correctnessCertificateWithBudget
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (B : ViewReductionBudget)
    (hNoise :
      ∀ e, P.noiseSupport e ->
        centeredResidueNoiseSupport q B.fullEta (P.noise e))
    (hSep :
      ∀ s t, s ≠ t ->
        2 * B.viewEta < centeredNormLinf q
          (fun j : ViewCoord => P.eval s (select j) - P.eval t (select j))) :
    PTNDCorrectnessCertificate q ViewCoord Secret Noise :=
  (coordinateRestriction (q := q) select).correctnessCertificateWithBudget P B
    (coordinateRestriction_noiseSmallWithBudget_of_full select P B hNoise)
    (coordinateRestriction_evalSeparatedWithBudget_of_selected_linf select P B hSep)

theorem restricted_planted_unique
    (V : SampleView q FullCoord ViewCoord)
    (P : P_TND_Decision q FullCoord Secret Noise) (eta : Nat)
    (hNoise : V.NoiseSmall P eta)
    (hSep : V.EvalSeparated P eta)
    (s : Secret) (e : Noise) (he : P.noiseSupport e) :
    ∀ t, ((V.restrictDecision P).realSearch eta s e).Success t -> t = s :=
  (V.correctnessCertificate P eta hNoise hSep).planted_unique s e he

/--
Deterministic "good diagonal" predicate for a scaled coordinate view.

It says that every distinct pair of secrets whose unscaled difference is large
in support is amplified by the public diagonal into the centered `L∞` gap needed
for dense-noise decoding.
-/
def DiagonalAmplifiesSupport
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord) (scale : ViewCoord -> ZMod q)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport eta : Nat) : Prop :=
  ∀ s t, s ≠ t ->
    minSupport <= weight (fun i : FullCoord => eval s i - eval t i) ->
    2 * eta < centeredNormLinf q
      (fun j : ViewCoord =>
        scale j * eval s (select j) - scale j * eval t (select j))

/--
A selected coordinate view covers the support of a full-coordinate vector when
every nonzero full coordinate is hit by at least one viewed coordinate.

For the uncompressed/full sample view this follows from `Function.Surjective
select`.  For compressed sample views this is the exact combinatorial condition
that must be proved for each relevant secret-pair difference.
-/
def SupportCoveredByView
    [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (x : FullCoord -> ZMod q) : Prop :=
  ∀ i, x i ≠ 0 -> ∃ j : ViewCoord, select j = i

omit [NeZero q] [Fintype FullCoord] [Fintype ViewCoord] in
theorem supportCoveredByView_of_surjective
    [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (hselect : Function.Surjective select)
    (x : FullCoord -> ZMod q) :
    SupportCoveredByView (q := q) select x := by
  intro i _hi
  exact hselect i

omit [NeZero q] [Fintype ViewCoord] in
theorem exists_selected_nonzero_of_supportCoveredByView
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord) (x : FullCoord -> ZMod q)
    (hCover : SupportCoveredByView (q := q) select x)
    (hWeight : 0 < weight x) :
    ∃ j : ViewCoord, x (select j) ≠ 0 := by
  classical
  rw [weight] at hWeight
  rcases Finset.card_pos.mp hWeight with ⟨i, hi⟩
  have hxi : x i ≠ 0 := by
    simpa using (mem_support_iff (x := x) (i := i)).mp hi
  rcases hCover i hxi with ⟨j, hj⟩
  exact ⟨j, by simpa [hj] using hxi⟩

omit [NeZero q] [Fintype ViewCoord] in
theorem exists_selected_pair_nonzero_of_supportCoveredByView
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport : Nat)
    (hMinSupport : 0 < minSupport)
    (hCover :
      ∀ pair : Secret × Secret,
        pair.1 ≠ pair.2 ->
        minSupport <=
          weight (fun i : FullCoord => eval pair.1 i - eval pair.2 i) ->
        SupportCoveredByView (q := q) select
          (fun i : FullCoord => eval pair.1 i - eval pair.2 i)) :
    ∀ pair : Secret × Secret,
      pair.1 ≠ pair.2 ->
      minSupport <=
        weight (fun i : FullCoord => eval pair.1 i - eval pair.2 i) ->
      ∃ j : ViewCoord,
        eval pair.1 (select j) - eval pair.2 (select j) ≠ 0 := by
  intro pair hst hSupport
  exact exists_selected_nonzero_of_supportCoveredByView
    (q := q) select
    (fun i : FullCoord => eval pair.1 i - eval pair.2 i)
    (hCover pair hst hSupport)
    (Nat.lt_of_lt_of_le hMinSupport hSupport)

omit [NeZero q] [Fintype ViewCoord] in
theorem exists_selected_pair_nonzero_of_surjective
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord)
    (eval : Secret -> FullCoord -> ZMod q)
    (minSupport : Nat)
    (hselect : Function.Surjective select)
    (hMinSupport : 0 < minSupport) :
    ∀ pair : Secret × Secret,
      pair.1 ≠ pair.2 ->
      minSupport <=
        weight (fun i : FullCoord => eval pair.1 i - eval pair.2 i) ->
      ∃ j : ViewCoord,
        eval pair.1 (select j) - eval pair.2 (select j) ≠ 0 :=
  exists_selected_pair_nonzero_of_supportCoveredByView
    (q := q) select eval minSupport hMinSupport
    (by
      intro pair _hst _hSupport
      exact supportCoveredByView_of_surjective
        (q := q) select hselect
        (fun i : FullCoord => eval pair.1 i - eval pair.2 i))

theorem scaledCoordinateRestriction_evalSeparated_of_amplifies
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord) (scale : ViewCoord -> ZMod q)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport eta : Nat)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hAmplify :
      DiagonalAmplifiesSupport (q := q) select scale P.eval minSupport eta) :
    (scaledCoordinateRestriction (q := q) select scale).EvalSeparated P eta := by
  intro s t hst
  exact hAmplify s t hst (hSupport s t hst)

def scaledCoordinateRestriction_correctnessCertificate_of_amplifies
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord) (scale : ViewCoord -> ZMod q)
    (P : P_TND_Decision q FullCoord Secret Noise) (minSupport eta : Nat)
    (hNoise :
      (scaledCoordinateRestriction (q := q) select scale).NoiseSmall P eta)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hAmplify :
      DiagonalAmplifiesSupport (q := q) select scale P.eval minSupport eta) :
    PTNDCorrectnessCertificate q ViewCoord Secret Noise :=
  (scaledCoordinateRestriction (q := q) select scale).correctnessCertificate
    P eta hNoise
    (scaledCoordinateRestriction_evalSeparated_of_amplifies select scale P
      minSupport eta hSupport hAmplify)

theorem scaledCoordinateRestriction_evalSeparatedWithBudget_of_amplifies
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord) (scale : ViewCoord -> ZMod q)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport : Nat) (B : ViewReductionBudget)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hAmplify :
      DiagonalAmplifiesSupport
        (q := q) select scale P.eval minSupport B.viewEta) :
    (scaledCoordinateRestriction (q := q) select scale).EvalSeparatedWithBudget
      P B := by
  exact scaledCoordinateRestriction_evalSeparated_of_amplifies
    (q := q) select scale P minSupport B.viewEta hSupport hAmplify

def scaledCoordinateRestriction_correctnessCertificateWithBudget_of_amplifies
    [DecidableEq FullCoord] [DecidableEq (ZMod q)]
    (select : ViewCoord -> FullCoord) (scale : ViewCoord -> ZMod q)
    (P : P_TND_Decision q FullCoord Secret Noise)
    (minSupport : Nat) (B : ViewReductionBudget)
    (hNoise :
      (scaledCoordinateRestriction (q := q) select scale).NoiseSmallWithBudget
        P B)
    (hSupport :
      ∀ s t, s ≠ t ->
        minSupport <= weight (fun i : FullCoord => P.eval s i - P.eval t i))
    (hAmplify :
      DiagonalAmplifiesSupport
        (q := q) select scale P.eval minSupport B.viewEta) :
    PTNDCorrectnessCertificate q ViewCoord Secret Noise :=
  (scaledCoordinateRestriction (q := q) select scale).correctnessCertificateWithBudget
    P B hNoise
    (scaledCoordinateRestriction_evalSeparatedWithBudget_of_amplifies
      select scale P minSupport B hSupport hAmplify)

/--
Coordinate restriction preserves a full-coordinate centered noise bound.

This is the easy half of a coordinate-view proof: projecting to fewer
coordinates cannot make a coordinatewise `L∞` noise promise larger.
-/
theorem coordinateRestriction_noiseSmall_of_full
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise) (eta : Nat)
    (hNoise :
      ∀ e, P.noiseSupport e ->
        centeredResidueNoiseSupport q eta (P.noise e)) :
    (coordinateRestriction (q := q) select).NoiseSmall P eta := by
  intro e he j
  exact hNoise e he (select j)

/--
The exact separation obligation for a coordinate-restricted sample view.

This theorem is intentionally close to the definition: unlike noise boundedness,
separation is not automatic under coordinate restriction.  A lower-dimensional
view is valid only when the selected coordinates still give a centered `L∞`
gap larger than twice the noise radius for every pair of distinct secrets.
-/
theorem coordinateRestriction_evalSeparated_of_selected_linf
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise) (eta : Nat)
    (hSep :
      ∀ s t, s ≠ t ->
        2 * eta < centeredNormLinf q
          (fun j : ViewCoord => P.eval s (select j) - P.eval t (select j))) :
    (coordinateRestriction (q := q) select).EvalSeparated P eta := by
  intro s t hst
  exact hSep s t hst

/--
Concrete correctness certificate for a coordinate-restricted public sample.

The theorem isolates the real proof task for shrinking the public sample:
choose `select` so that the selected coordinates still satisfy the centered
separation condition.
-/
def coordinateRestriction_correctnessCertificate
    (select : ViewCoord -> FullCoord)
    (P : P_TND_Decision q FullCoord Secret Noise) (eta : Nat)
    (hNoise :
      ∀ e, P.noiseSupport e ->
        centeredResidueNoiseSupport q eta (P.noise e))
    (hSep :
      ∀ s t, s ≠ t ->
        2 * eta < centeredNormLinf q
          (fun j : ViewCoord => P.eval s (select j) - P.eval t (select j))) :
    PTNDCorrectnessCertificate q ViewCoord Secret Noise :=
  (coordinateRestriction (q := q) select).correctnessCertificate P eta
    (coordinateRestriction_noiseSmall_of_full select P eta hNoise)
    (coordinateRestriction_evalSeparated_of_selected_linf select P eta hSep)

end SampleView

end Descender
