import Descender.QLatticeHardness
import Descender.SampleView
import Descender.StrategyComparison
import Descender.SystematicLee

namespace Descender

/-!
Proof-facing interface for efficient public views.

The parameter-search lesson is that low-Hamming or guard-distance certificates
are not the security object for the literal centered-noise public-sample game.
The public view must be efficient as an algorithmic object *and* must expose a
q-ary relation code with large centered Lee minimum.  This file packages that
target as a reusable certificate boundary.

The intended replacement for the Cauchy guard-prefix route is:

* a seeded public view with explicit cost accounting;
* correctness at the viewed noise radius;
* a centered-Lee lower bound for every nonzero public q-ary dual relation;
* the arithmetic wrap inequality turning that Lee bound into
  `NoNarrowQaryDualRelations`.
-/

variable {q : Nat} [NeZero q]
variable {FullCoord ViewCoord Secret Noise DualTarget : Type}

/--
Cost and validation metadata for a public view family.

`seedBits` is the public descriptor size.  `mulWork` accounts for applying the
view/evaluator, while `certificateWork` accounts for checking the proof-carrying
sidecar.  The theorem layer treats these as accounting fields; concrete tools
decide how to estimate them for dense, Toeplitz, block-random, expander, or
arithmetic implementations.
-/
structure PublicViewEfficiency where
  seedBits : Nat
  publicSampleBits : Nat
  mulWork : Nat
  certificateWork : Nat

namespace PublicViewEfficiency

def totalPublicBits (E : PublicViewEfficiency) : Nat :=
  E.seedBits + E.publicSampleBits

def totalWork (E : PublicViewEfficiency) : Nat :=
  E.mulWork + E.certificateWork

def BoundedBy (E B : PublicViewEfficiency) : Prop :=
  E.seedBits <= B.seedBits ∧
    E.publicSampleBits <= B.publicSampleBits ∧
    E.mulWork <= B.mulWork ∧
    E.certificateWork <= B.certificateWork

theorem totalPublicBits_le_of_boundedBy {E B : PublicViewEfficiency}
    (h : E.BoundedBy B) :
    E.totalPublicBits <= B.totalPublicBits := by
  unfold totalPublicBits
  exact Nat.add_le_add h.1 h.2.1

theorem totalWork_le_of_boundedBy {E B : PublicViewEfficiency}
    (h : E.BoundedBy B) :
    E.totalWork <= B.totalWork := by
  unfold totalWork
  exact Nat.add_le_add h.2.2.1 h.2.2.2

end PublicViewEfficiency

/--
Lee-dual certificate for the actually serialized public coordinate system.

`dualMap` is the public adjoint/relation map for the viewed sample.  `leeMin`
is a lower bound on `centeredNormL1` for every nonzero vector in its kernel.
`hWrap` is the no-wrap threshold required by centered-noise P-TND:
`q <= 2 * eta * leeMin + 1`.
-/
structure PublicViewLeeDualCertificate
    [Fintype ViewCoord] [Zero DualTarget]
    (dualMap : (ViewCoord -> ZMod q) -> DualTarget)
    (eta : Nat) where
  leeMin : Nat
  hLee :
    ∀ r : ViewCoord -> ZMod q,
      dualMap r = 0 ->
      r ≠ 0 ->
      leeMin <= centeredNormL1 q r
  hWrap : q <= 2 * eta * leeMin + 1

namespace PublicViewLeeDualCertificate

theorem noNarrowDual
    [Fintype ViewCoord] [Zero DualTarget]
    {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
    {eta : Nat}
    (C : PublicViewLeeDualCertificate (q := q) dualMap eta) :
    NoNarrowQaryDualRelations q eta dualMap :=
  no_narrow_qary_dual_relations_of_lee_l1_lower_bound
    (q := q) (eta := eta) (d := C.leeMin)
    (AT := dualMap) C.hLee C.hWrap

def amplitudePromise
    [Fintype ViewCoord] [Zero DualTarget]
    {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
    {eta : Nat}
    (C : PublicViewLeeDualCertificate (q := q) dualMap eta) :
    QaryDualAmplitudePromise q ViewCoord DualTarget dualMap where
  eta := eta
  noNarrowDual := C.noNarrowDual

end PublicViewLeeDualCertificate

/--
Concrete public-view witness for the dual-relation failure that the Lee
certificate is meant to exclude.

This is a deliberately small extracted object: an integer relation `z` plus the
proof that its residue is a nontrivial public dual relation whose centered
noise interval does not wrap modulo `q`.
-/
structure PublicViewNarrowDualWitness
    [Fintype ViewCoord] [Zero DualTarget]
    (dualMap : (ViewCoord -> ZMod q) -> DualTarget)
    (eta : Nat) where
  z : ViewCoord -> Int
  vulnerable : AmplitudeVulnerableQaryDualRelation q eta dualMap z

namespace PublicViewNarrowDualWitness

theorem impossible_of_noNarrowDual
    [Fintype ViewCoord] [Zero DualTarget]
    {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
    {eta : Nat}
    (C : NoNarrowQaryDualRelations q eta dualMap)
    (W : PublicViewNarrowDualWitness (q := q) dualMap eta) :
    False :=
  C ⟨W.z, W.vulnerable⟩

theorem impossible_of_certificate
    [Fintype ViewCoord] [Zero DualTarget]
    {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
    {eta : Nat}
    (C : PublicViewLeeDualCertificate (q := q) dualMap eta)
    (W : PublicViewNarrowDualWitness (q := q) dualMap eta) :
    False :=
  impossible_of_noNarrowDual C.noNarrowDual W

end PublicViewNarrowDualWitness

/--
Systematic Lee witness for the more structured certificate failure:
a nonzero free vector whose public systematic relation `(-P y, y)` is shorter
than the advertised Lee lower bound.
-/
structure SystematicLeeViolation
    [Fintype Pivot] [Fintype Free]
    (P : Pivot -> Free -> ZMod q) (leeMin : Nat) where
  y : Free -> ZMod q
  nonzero : y ≠ 0
  short : centeredNormL1 q (systematicRelation q P y) < leeMin

namespace SystematicLeeViolation

theorem relation_kernel
    [Fintype Pivot] [Fintype Free]
    {P : Pivot -> Free -> ZMod q} {leeMin : Nat}
    (W : SystematicLeeViolation (q := q) P leeMin) :
    systematicDualMap q P (systematicRelation q P W.y) = 0 :=
  systematicDualMap_relation_eq_zero q P W.y

theorem relation_nonzero
    [Fintype Pivot] [Fintype Free]
    {P : Pivot -> Free -> ZMod q} {leeMin : Nat}
    (W : SystematicLeeViolation (q := q) P leeMin) :
    systematicRelation q P W.y ≠ 0 :=
  (systematicRelation_nonzero_iff q P).mpr W.nonzero

theorem impossible_of_distanceAtLeast
    [Fintype Pivot] [Fintype Free]
    {P : Pivot -> Free -> ZMod q} {leeMin : Nat}
    (hLee : SystematicLeeDistanceAtLeast q P leeMin)
    (W : SystematicLeeViolation (q := q) P leeMin) :
    False := by
  exact Nat.not_lt_of_ge (hLee W.y W.nonzero) W.short

theorem impossible_of_systematicCertificate
    [Fintype Pivot] [Fintype Free]
    {P : Pivot -> Free -> ZMod q}
    (C : SystematicLeeCertificate q Pivot Free P)
    (W : SystematicLeeViolation (q := q) P C.lowerBound) :
    False :=
  impossible_of_distanceAtLeast C.lowerBoundProof W

theorem impossible_of_publicViewLeeDualCertificate
    [Fintype Pivot] [Fintype Free]
    {P : Pivot -> Free -> ZMod q} {eta : Nat}
    (C : PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta)
    (W : SystematicLeeViolation (q := q) P C.leeMin) :
    False := by
  have hLower := C.hLee
    (systematicRelation q P W.y)
    W.relation_kernel
    W.relation_nonzero
  exact Nat.not_lt_of_ge hLower W.short

end SystematicLeeViolation

/--
Certificate that a serialized coordinate view has exactly the q-ary dual
relations obtained by extending viewed relations into a larger certified
coordinate system.

This is the proof boundary needed for genuine public-sample view reduction.
It is not enough to know that the full public key has no narrow dual relation:
after coordinates are removed, the viewed adjoint has a different kernel.  The
view must provide an `extend` map into the full coordinate system, prove that
the viewed dual map is the full dual map after extension, and prove that the
extension preserves centered Lee `L1`.
-/
structure LeeDualViewRestrictionCertificate
    [Fintype FullCoord] [Fintype ViewCoord] [Zero DualTarget]
    (fullDualMap : (FullCoord -> ZMod q) -> DualTarget)
    (viewDualMap : (ViewCoord -> ZMod q) -> DualTarget) where
  extend : (ViewCoord -> ZMod q) -> FullCoord -> ZMod q
  map_eq : ∀ r, viewDualMap r = fullDualMap (extend r)
  leeNorm_eq : ∀ r, centeredNormL1 q (extend r) = centeredNormL1 q r
  nonzero_extend : ∀ r, r ≠ 0 -> extend r ≠ 0

namespace LeeDualViewRestrictionCertificate

def restrictLeeDualCertificate
    [Fintype FullCoord] [Fintype ViewCoord] [Zero DualTarget]
    {fullDualMap : (FullCoord -> ZMod q) -> DualTarget}
    {viewDualMap : (ViewCoord -> ZMod q) -> DualTarget}
    {eta : Nat}
    (R : LeeDualViewRestrictionCertificate
      (q := q) fullDualMap viewDualMap)
    (Cfull : PublicViewLeeDualCertificate (q := q) fullDualMap eta) :
    PublicViewLeeDualCertificate (q := q) viewDualMap eta where
  leeMin := Cfull.leeMin
  hLee := by
    intro r hr hrne
    have hfullKernel : fullDualMap (R.extend r) = 0 := by
      rw [← R.map_eq r]
      exact hr
    have hfullNonzero : R.extend r ≠ 0 := R.nonzero_extend r hrne
    calc
      Cfull.leeMin <= centeredNormL1 q (R.extend r) :=
        Cfull.hLee (R.extend r) hfullKernel hfullNonzero
      _ = centeredNormL1 q r := R.leeNorm_eq r
  hWrap := Cfull.hWrap

theorem noNarrowDual
    [Fintype FullCoord] [Fintype ViewCoord] [Zero DualTarget]
    {fullDualMap : (FullCoord -> ZMod q) -> DualTarget}
    {viewDualMap : (ViewCoord -> ZMod q) -> DualTarget}
    {eta : Nat}
    (R : LeeDualViewRestrictionCertificate
      (q := q) fullDualMap viewDualMap)
    (Cfull : PublicViewLeeDualCertificate (q := q) fullDualMap eta) :
    NoNarrowQaryDualRelations q eta viewDualMap :=
  (R.restrictLeeDualCertificate Cfull).noNarrowDual

end LeeDualViewRestrictionCertificate

/--
The core construction target: an efficient public view with correctness and
Lee-dual security certified for the same serialized coordinate system.

This is intentionally independent of the old Cauchy/guard construction.  A
concrete algorithm should instantiate this structure directly, then optionally
wrap it in a `StrategyCertificate`.
-/
structure EfficientPublicViewCertificate
    [Fintype FullCoord] [Fintype ViewCoord] [Zero DualTarget]
    (P : P_TND_Decision q FullCoord Secret Noise)
    (dualMap : (ViewCoord -> ZMod q) -> DualTarget)
    (B : SampleView.ViewReductionBudget) where
  view : SampleView q FullCoord ViewCoord
  efficiency : PublicViewEfficiency
  hNoise : view.NoiseSmallWithBudget P B
  hSeparated : view.EvalSeparatedWithBudget P B
  leeDual : PublicViewLeeDualCertificate (q := q) dualMap B.viewEta

namespace EfficientPublicViewCertificate

def correctnessCertificate
    [Fintype FullCoord] [Fintype ViewCoord] [Zero DualTarget]
    {P : P_TND_Decision q FullCoord Secret Noise}
    {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
    {B : SampleView.ViewReductionBudget}
    (C : EfficientPublicViewCertificate (q := q) P dualMap B) :
    PTNDCorrectnessCertificate q ViewCoord Secret Noise :=
  C.view.correctnessCertificateWithBudget P B C.hNoise C.hSeparated

theorem noNarrowDual
    [Fintype FullCoord] [Fintype ViewCoord] [Zero DualTarget]
    {P : P_TND_Decision q FullCoord Secret Noise}
    {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
    {B : SampleView.ViewReductionBudget}
    (C : EfficientPublicViewCertificate (q := q) P dualMap B) :
    NoNarrowQaryDualRelations q B.viewEta dualMap :=
  C.leeDual.noNarrowDual

theorem noPublicViewNarrowDualWitness
    [Fintype FullCoord] [Fintype ViewCoord] [Zero DualTarget]
    {P : P_TND_Decision q FullCoord Secret Noise}
    {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
    {B : SampleView.ViewReductionBudget}
    (C : EfficientPublicViewCertificate (q := q) P dualMap B)
    (W : PublicViewNarrowDualWitness (q := q) dualMap B.viewEta) :
    False :=
  PublicViewNarrowDualWitness.impossible_of_noNarrowDual C.noNarrowDual W

def dualAmplitudePromise
    [Fintype FullCoord] [Fintype ViewCoord] [Zero DualTarget]
    {P : P_TND_Decision q FullCoord Secret Noise}
    {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
    {B : SampleView.ViewReductionBudget}
    (C : EfficientPublicViewCertificate (q := q) P dualMap B) :
    QaryDualAmplitudePromise q ViewCoord DualTarget dualMap :=
  C.leeDual.amplitudePromise

def strategyCertificate
    [Fintype FullCoord] [Fintype ViewCoord] [Zero DualTarget]
    {P : P_TND_Decision q FullCoord Secret Noise}
    {dualMap : (ViewCoord -> ZMod q) -> DualTarget}
    {B : SampleView.ViewReductionBudget}
    (C : EfficientPublicViewCertificate (q := q) P dualMap B)
    (costs : SampleView.StrategyCostProfile) :
    SampleView.StrategyCertificate q ViewCoord Secret Noise PUnit where
  kind := SampleView.StrategyKind.lossyBudget
  costs := costs
  correctness := ⟨⟨PUnit.unit, C.correctnessCertificate⟩⟩

end EfficientPublicViewCertificate

/--
Systematic specialization for the main construction search:
`A = [I | P]`, so public dual relations are
`(-P y, y)`.  The only hard mathematical certificate is the Lee lower bound
for those systematic relations.
-/
def systematicLeeDualCertificate
    [Fintype Pivot] [Fintype Free]
    (P : Pivot -> Free -> ZMod q) {eta leeMin : Nat}
    (hLee : SystematicLeeDistanceAtLeast q P leeMin)
    (hWrap : q <= 2 * eta * leeMin + 1) :
    PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q P) eta where
  leeMin := leeMin
  hLee := by
    intro r hr hrne
    have hrSys := eq_systematicRelation_of_systematicDualMap_eq_zero q P hr
    let y : Free -> ZMod q := fun f => r (Sum.inr f)
    have hyne : y ≠ 0 := by
      intro hy
      apply hrne
      rw [hrSys]
      exact (systematicRelation_eq_zero_iff q P).mpr hy
    rw [hrSys]
    exact hLee y hyne
  hWrap := hWrap

/--
Composable systematic specialization for proof-carrying efficient views.

This is the global-proof entry point for block/local constructions: local
Lee-distance certificates are composed by
`ComposableSystematicLeeCertificate.toSystematicLeeCertificate`, then used to
build the public-view q-ary Lee-dual certificate.
-/
def composableSystematicLeeDualCertificate
    {Block LocalPivot LocalFree Pivot Free : Type}
    [Fintype Block] [Fintype LocalPivot] [Fintype LocalFree]
    [Fintype Pivot] [Fintype Free]
    (Pglobal : Pivot -> Free -> ZMod q)
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (freeSlice : Block -> (Free -> ZMod q) -> LocalFree -> ZMod q)
    {eta : Nat}
    (C : ComposableSystematicLeeCertificate q
      Block LocalPivot LocalFree Pivot Free Pglobal Plocal freeSlice)
    (hWrap : q <= 2 * eta * C.lowerBound + 1) :
    PublicViewLeeDualCertificate
      (q := q) (ViewCoord := Sum Pivot Free)
      (DualTarget := Pivot -> ZMod q) (systematicDualMap q Pglobal) eta :=
  systematicLeeDualCertificate Pglobal
    (eta := eta) (leeMin := C.lowerBound)
    C.toSystematicLeeCertificate.lowerBoundProof hWrap

/--
Concrete product-block public-view Lee certificate.

This is the proof-facing endpoint for the `product-block-lee` search route:
each local block supplies an exact bounded-free-vector Lee check, Lean proves
the block-diagonal local-to-global step, and the result is the public-view
no-narrow-dual certificate for `A = [I | P]`.
-/
def blockDiagonalCheckedLeeDualCertificate
    {Block LocalPivot LocalFree : Type}
    [DecidableEq Block]
    [Fintype Block] [Fintype LocalPivot] [Fintype LocalFree]
    (Plocal : Block -> LocalPivot -> LocalFree -> ZMod q)
    (Clocal :
      ∀ b, BoundedFreeLeeCheckCertificate q LocalPivot LocalFree (Plocal b))
    {eta lowerBound : Nat}
    (hLower : ∀ b, lowerBound <= (Clocal b).lowerBound)
    (hWrap : q <= 2 * eta * lowerBound + 1) :
    PublicViewLeeDualCertificate
      (q := q)
      (ViewCoord := Sum (Block × LocalPivot) (Block × LocalFree))
      (DualTarget := Block × LocalPivot -> ZMod q)
      (systematicDualMap q (blockDiagonalSystematicMatrix Plocal)) eta :=
  composableSystematicLeeDualCertificate
    (blockDiagonalSystematicMatrix Plocal) Plocal
    (productFreeSlice (q := q))
    (blockDiagonalProductComposableCertificateOfCheckedLocal q Plocal
      Clocal hLower)
    hWrap

end Descender
