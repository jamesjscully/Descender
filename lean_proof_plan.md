# Descender Lean 4 Proof Implementation Plan

This document gives a step-by-step plan for formalizing the first Descender proof core in Lean 4. The goal is not to formalize the full cryptographic security theorem immediately. The goal is to build a verified deterministic and combinatorial backbone that later supports the HDX and probabilistic pieces.

The first formal target is:

\[
\textbf{guarded Cauchy-MDS exact-gradient projection}
+
\textbf{HDX wedge-rank tilt}
\Rightarrow
\textbf{finite-dimensional transversality and patch-rank expansion}.
\]

The proof should be developed in layers. Each layer should compile independently and expose clean theorem statements for later stages.

## 0. Project Setup

Create a Lean project with Mathlib.

Suggested layout:

```text
Descender/
  Basic.lean
  FiniteSupport.lean
  LinearCode.lean
  Cauchy.lean
  GuardedTilt.lean
  Hypergraph.lean
  Wedge.lean
  PeelingRank.lean
  MainTheorems.lean
```

Initial imports will likely include:

```lean
import Mathlib.Data.Matrix.Basic
import Mathlib.LinearAlgebra.Matrix.Determinant.Basic
import Mathlib.LinearAlgebra.FiniteDimensional
import Mathlib.LinearAlgebra.Vandermonde
import Mathlib.FieldTheory.Finite.GaloisField
import Mathlib.Data.Finset.Card
import Mathlib.Combinatorics.SimpleGraph.Finite
```

Use a general field parameter first:

```lean
variable {F : Type*} [Field F]
```

Specialize to finite fields only when cardinality or random sampling is needed.

## 1. Finite Vector and Support Infrastructure

File:

```text
FiniteSupport.lean
```

Define Hamming support and Hamming weight for functions over finite index types:

```lean
def support (x : iota -> F) : Finset iota := ...
def weight (x : iota -> F) : Nat := (support x).card
```

Prove basic lemmas:

```lean
mem_support_iff
not_mem_support_iff
weight_eq_zero_iff
weight_le_card
support_add_subset
weight_add_le
support_smul_subset
support_smul_eq_of_ne_zero
```

This file should avoid matrices. It is the shared combinatorial layer.

Success criterion:

```lean
weight (x + y) <= weight x + weight y
```

is available as a theorem.

## 2. Linear Maps and Codes

File:

```text
LinearCode.lean
```

Define linear code image and dual kernel for a linear map:

```lean
def imageCode (A : V ->L[F] M) : Submodule F M := LinearMap.range A
def dualKernel (AT : M ->L[F] V) : Submodule F M := LinearMap.ker AT
```

Here `->L[F]` is ASCII shorthand in this document for Lean's linear-map arrow notation.

For concrete finite-coordinate spaces, define minimum distance as a predicate first, not as an infimum:

```lean
def HasMinWeightAtLeast (C : Submodule F (iota -> F)) (d : Nat) : Prop :=
  forall x, x in C -> x != 0 -> d <= weight x
```

Similarly define:

```lean
def NoKernelVectorBelow (A : M ->L[F] V) (d : Nat) : Prop :=
  forall r, A r = 0 -> r != 0 -> d <= weight r
```

Avoid optimizing definitions too early. Predicates are easier than computable minima.

Success criterion:

```lean
HasMinWeightAtLeast (imageCode A) d
```

can be used to express the guard-block primal distance theorem.

## 3. Cauchy Matrix Formalization

File:

```text
Cauchy.lean
```

Define Cauchy entries over a field:

```lean
def cauchyEntry (x : Row -> F) (alpha : Col -> F) (i : Row) (j : Col) : F :=
  (x i - alpha j)^(-1)
```

Assumptions:

```lean
hDistinctX : Function.Injective x
hDistinctA : Function.Injective alpha
hDisjoint  : forall i j, x i != alpha j
```

Define:

```lean
def CauchyMatrix : Matrix Row Col F := Matrix.of fun i j => (x i - alpha j)^(-1)
```

Primary theorem:

```lean
theorem cauchy_support_lower_bound
  (r : Col -> F)
  (ht : weight r <= Fintype.card Row)
  (hr : r != 0) :
  Fintype.card Row - weight r + 1 <= weight (fun i => sum j, CauchyMatrix x alpha i j * r j)
```

Proof strategy:

1. Let \(S=\operatorname{supp}(r)\).
2. Define the rational function:
   \[
   f(z)=\sum_{j\in S} r_j/(z-\alpha_j).
   \]
3. Clear denominators:
   \[
   f(z)=P(z)/Q(z)
   \]
   where \(\deg P < |S|\).
4. If \(Br\) vanishes at too many \(x_i\), then \(P\) has too many roots.
5. Since \(P\ne 0\), contradiction.

Lean implementation note:

The polynomial/root-count route may be the hardest part of the deterministic proof. If it becomes too slow, prove the equivalent square-minor nonsingularity theorem first:

```lean
theorem cauchy_square_minor_det_ne_zero : ...
```

Then derive support lower bound from total nonsingularity.

Secondary theorem:

```lean
theorem cauchy_guard_primal_distance
  (hRowsLeCols : Fintype.card Row <= Fintype.card GuardCol)
  (s : Row -> F) (hs : s != 0) :
  m0Card - rowCard + 1 <= weight (B0^T *v s)
```

This is the MDS guard-block statement in the dimension range actually needed
for the cryptographic guard layer.  The hypothesis \(n\le m_0\) is necessary:
without at least as many guard columns as row coordinates, a tall Cauchy block
has nonzero left-kernel vectors.  The Lean development records a concrete
\(2\times 1\) Cauchy counterexample to the unrestricted statement as
`cauchy_guard_primal_distance_strong_counterexample_data`.

Success criterion:

The Cauchy block can prove:

\[
\operatorname{wt}(Br)\ge n-\operatorname{wt}(r)+1
\]

for all nonzero \(r\) with \(\operatorname{wt}(r)\le n\).

For the guard block it can prove:

\[
\operatorname{wt}(B_0^\ast s)\ge m_0-\operatorname{wt}(s)+1
\]

for nonzero \(s\) once \(\operatorname{wt}(s)\le m_0\).  Since
\(\operatorname{wt}(s)\le n\), the public cryptographic parameter condition
\(n\le m_0\) yields the advertised minimum distance
\(d_{\mathrm{prim}}\ge m_0-n+1\).

## 4. Guarded Tilted Projection Core

File:

```text
GuardedTilt.lean
```

Use finite coordinate spaces:

```lean
V  := I -> F
M0 := J0 -> F
M1 := J1 -> F
M  := Sum J0 J1 -> F
```

Represent:

```lean
B0 : M0 ->L[F] V
B1 : M1 ->L[F] V
H  : M1 ->L[F] V
```

where:

\[
H=D^\ast C\Delta.
\]

For the deterministic theorem, only require a locality bound on \(H\)'s columns:

```lean
def ColumnSupportBound (H : M1 ->L[F] V) (L : Nat) : Prop := ...
```

Define:

```lean
def AT (r : M0 Prod M1) : V :=
  B0 r.1 + B1 r.2 + H r.2
```

or use `Sum`-indexed vectors if easier.

Main deterministic dual theorem:

```lean
theorem guarded_tilt_no_low_weight_dual
  (hCauchy : forall r, r != 0 -> weight r <= n ->
      n - weight r + 1 <= weight (B r))
  (hH : ColumnSupportBound H L)
  {r : M}
  (hAT : AT r = 0)
  (hr : r != 0)
  (hSmall : weight r <= n) :
  (n + 1) <= (L + 1) * weight r
```

Corollary:

```lean
theorem guarded_tilt_dual_distance :
  NoKernelVectorBelow AT ((n + L) / (L + 1))
```

Exact arithmetic can be adjusted to avoid floor/ceiling pain. The clean Lean statement is often:

```lean
AT r = 0 -> r != 0 -> weight r < (n + 1) / (L + 1) -> False
```

Main primal theorem:

```lean
theorem guarded_tilt_primal_distance
  (hB0 : forall s != 0, mu + 1 <= weight (B0^T s)) :
  forall s != 0, mu + 1 <= weight (A s)
```

Proof:

The guard coordinates of \(A s\) are \(B_0^\ast s\), so the total support is at least the guard support.

When `hB0` is supplied by the Cauchy guard theorem, the caller must also supply
the guard dimension budget:

```lean
hRowsLeGuardCols : rows.length <= j0.length
```

Equivalently, in the mathematical parameter notation, choose \(m_0\ge n\).
This is an architectural requirement, not a proof artifact.

Success criterion:

Formalize:

\[
d_{\mathrm{prim}}\ge m_0-n+1,
\qquad
d_{\mathrm{dual}}> \frac{n+1}{L+1}.
\]

without referencing HDX.

## 5. Hypergraph and Peelability

File:

```text
Hypergraph.lean
```

Define a finite uniform hypergraph abstractly:

```lean
structure Hypergraph (V E : Type*) [Fintype V] [Fintype E] where
  inc : E -> Finset V
```

Define degree into a finite edge set:

```lean
def degreeIn (H : Hypergraph V E) (S : Finset E) (v : V) : Nat :=
  (S.filter fun e => v in H.inc e).card
```

Define peelability:

```lean
def Peelable (H : Hypergraph V E) (tauN k : Nat) : Prop :=
  forall S : Finset E,
    S.Nonempty ->
    S.card <= tauN ->
    exists v, 1 <= degreeIn H S v and degreeIn H S v <= k
```

Define neighbor set:

```lean
def neighbors (H : Hypergraph V E) (S : Finset E) : Finset V :=
  S.biUnion H.inc
```

Prove expansion implies peelability:

```lean
theorem peelable_of_neighbor_expansion
  (hUniform : forall e, (H.inc e).card = 3)
  (hExpand : forall S, S.Nonempty -> S.card <= tauN ->
      (k + 1) * (neighbors H S).card < 3 * S.card) :
  Peelable H tauN k
```

Check inequality direction carefully. In the research text the sufficient condition is:

\[
|\Gamma(S)| > \frac{3}{k+1}|S|.
\]

In natural-number Lean form:

\[
3|S| < (k+1)|\Gamma(S)|.
\]

Then a \((k+1)\)-core contradicts incidence counting.

Success criterion:

Peelability is a reusable combinatorial predicate independent of vector spaces.

## 6. Wedge Data Model

File:

```text
Wedge.lean
```

Start with an abstract wedge system rather than a full simplicial complex:

```lean
structure WedgeSystem (Vertex Wedge : Type*) [Fintype Vertex] [Fintype Wedge] where
  endpoints : Wedge -> Finset Vertex
  endpoints_card : forall w, (endpoints w).card = 3
```

Later refine this into typed 2-faces of an HDX.

For fiber dimension \(k\), use a finite index type `Fiber`:

```lean
variable {Fiber : Type*} [Fintype Fiber]
```

A divergence column is:

```lean
h : Wedge -> Vertex -> Fiber -> F
```

with support contained in endpoints:

```lean
forall w v, v notin endpoints w -> h w v = 0
```

Define local endpoint vectors:

```lean
def localVec (h : ...) (v : Vertex) (w : Wedge) : Fiber -> F := h w v
```

Define local \(k\)-wise independence. Because `k` is a natural number and the fiber type has cardinality `k`, state it as:

```lean
def LocallyIndependentUpTo
  (h : Wedge -> Vertex -> Fiber -> F) (k : Nat) : Prop :=
  forall v (S : Finset Wedge),
    S.card <= k ->
    (forall w in S, v in endpoints w) ->
    LinearIndependent F (fun w : {w // w in S} => localVec h v w)
```

Success criterion:

All wedge-rank lemmas can be stated without simplicial-complex machinery.

## 7. Peeling Implies Rank

File:

```text
PeelingRank.lean
```

This is the most important first Descender-specific theorem.

Given:

```lean
H : Wedge -> (Vertex Prod Fiber -> F)
```

or equivalently:

```lean
Hcol w : Vertex -> Fiber -> F
```

prove:

```lean
theorem linearIndependent_columns_of_peelable
  (hPeel : Peelable wedgeHypergraph tauN k)
  (hLocal : LocallyIndependentUpTo h k)
  (S : Finset Wedge)
  (hS : S.card <= tauN) :
  LinearIndependent F (fun w : {w // w in S} => Hcol w)
```

Proof strategy:

Use induction on `S.card`.

1. If `S` is empty, trivial.
2. Otherwise peelability gives a vertex \(v\) with \(1\le \deg_S(v)\le k\).
3. Restrict any linear relation to the \(v\)-fiber block.
4. Local independence forces coefficients on all wedges incident to \(v\) to be zero.
5. Remove those wedges and apply induction to the remaining set.

Implementation detail:

It may be easier to prove a relation-elimination lemma:

```lean
theorem coefficients_vanish_on_low_degree_vertex
  ...
```

then use well-founded induction on finite-set cardinality.

Success criterion:

Formal proof of:

\[
(\tau,k)\text{-peelability}
+
\text{local }k\text{-wise independence}
\Rightarrow
\operatorname{rank}(H_S)=|S|.
\]

This theorem is the best early milestone because it is novel, finite, and does not depend on probability.

## 8. Connect Wedges to the Tilted Projection

File:

```text
MainTheorems.lean
```

Instantiate the abstract \(H\) in the guarded tilt theorem as:

\[
H=D^\ast C\Delta.
\]

For the first deterministic theorem, use only:

```lean
ColumnSupportBound H 3
```

For the stronger medium-complexity theorem, use:

```lean
LinearIndependent F (fun w in S => Hcol w)
```

from the peeling theorem.

First combined theorem:

```lean
theorem descender_deterministic_core
  (hCauchy : CauchySupportBound B)
  (hWedgeLocality : ColumnSupportBound H 3) :
  FullRank A and
  PrimalDistanceAtLeast A (m0 - n + 1) and
  NoDualVectorBelow AT ((n + 1) / 4)
```

Second combined theorem:

```lean
theorem descender_patch_rank
  (hPeel : Peelable wedgeHypergraph tauN k)
  (hLocal : LocallyIndependentUpTo h k) :
  forall S, S.card <= tauN ->
    LinearIndependent F (fun w : {w // w in S} => Hcol w)
```

Success criterion:

`MainTheorems.lean` exports exactly the deterministic and patch-rank theorem statements from `research_program.md`.

## 9. Probability and Public Thinning

File:

```text
Thinning.lean
```

Do not start here.

Once deterministic and peeling proofs are complete, add finite probability over selected wedge subsets.

Definitions:

```lean
def BernoulliThinning ...
def BadCoreEvent ...
def RegularityEvent ...
```

First theorem target:

```lean
theorem thinning_preserves_large_set_regular
```

Second theorem target:

```lean
theorem thinning_excludes_small_cores
```

Expect to build custom finite probability lemmas. Mathlib has probability infrastructure, but the specific Chernoff/counting arguments are likely new.

Success criterion:

The probabilistic theorem outputs:

```lean
Pr[not  Peelable H tauN k] <= negligible lambda
```

or a finite explicit bound first.

## 10. HDX Regularity Layer

File:

```text
HDX.lean
```

Do not attempt full local spectral expansion first. Start axiomatically.

Define:

```lean
structure TypedFaceSampler where
  Vertex : Type
  Edge : Type
  Face2 : Type
  type : Vertex -> Fin d
  endpoints ...
  regularity_axiom : ...
```

Then prove wedge sampler regularity from the axiom.

Only after the axiom-level proof is stable should we instantiate a concrete HDX family.

Success criterion:

The rest of Descender depends only on a small interface:

```lean
TypedFaceRegularity
BoundedOverlap
PublicThinningGood
```

not on a particular HDX construction.

## 11. q-ary Lattice Layer

File:

```text
QLattice.lean
```

The first dual-only q-ary layer now exists in `Residue.lean` and
`QLattice.lean`.

Define:

\[
\Lambda_A^\perp=\{z\in\mathbb Z^m:A^\ast z\equiv0\pmod q\}.
\]

Lean representation:

```lean
def qaryDualLattice (A : (iota -> ZMod q) ->L[ZMod q] (kappa -> ZMod q)) : Set (kappa -> Int) := ...
```

Initial q-ary bridge targets:

```lean
low_hamming_relation_of_short_l1_relation
no_short_integer_relation_from_modq_transversality
```

The current Lean bridge proves:

```lean
residue_weight_le_intL1Norm
no_short_integer_relation_from_modq_transversality
no_short_qary_dual_relation
```

That is, finite-field no-low-Hamming-weight kernel vectors rule out nontrivial
integer \(q\)-ary dual relations whose integer \(L^1\) norm is below the same
threshold.  This remains useful as a helper, but it is not the KEM-facing
decision-security target.

The centered-noise decision game instead requires:

```lean
AmplitudeVulnerableQaryDualRelation
NoNarrowQaryDualRelations
QaryDualAmplitudePromise
DualIntervalAccepts
CenteredNoisePairingBounded
```

If `z ∈ qaryDualLattice q AT`, then real samples `t = A s + e` satisfy
`<z,t> = <z,e>`.  A relation with `2 * eta * intL1Norm z + 1 < q` gives the
direct centered-interval distinguisher.  Therefore `DescenderKEMCertificate`
now exposes:

```lean
DescenderKEMCertificate.no_narrow_qary_dual_relations
DescenderKEMCertificate.no_narrow_qary_dual_relations_at_noise
```

and the combined q-ary/noise promise carries `noNarrowDual`.  The top-level
interfaces also require `dualAmplitudeEta_le_noiseEta`, so the certified
amplitude exclusion applies at the actual centered-noise radius used by PTND and
the KEM.  Low-Hamming dual search is only acceptable when it proves this
amplitude condition, for example through
`no_narrow_qary_dual_relations_of_l1_lower_bound`.

The search-side certificate should be phrased as a centered/Lee \(L^1\) lower
bound on nonzero q-ary kernel vectors.  Lean exposes this as
`ModqNoLeeKernelVectorBelow`, with bridge theorems
`no_short_integer_relation_from_modq_lee_transversality` and
`no_narrow_qary_dual_relations_of_lee_l1_lower_bound`.  The proof bridge uses
`centeredNormL1_residueVec_le_intL1Norm`: centered residue representatives are
no larger than arbitrary integer representatives, so a Lee lower bound implies
the integer \(L^1\) lower bound consumed by the no-narrow theorem.

For generated matrices with an invertible pivot block, the algebraic lower-bound
target is now separated in `SystematicLee.lean`.  The theorem
`modqNoLeeKernelVectorBelow_of_systematic_lower_bound` reduces the full kernel
certificate to the systematic relation map \(y\mapsto(-P y,y)\), where `P` is
the public matrix obtained after solving the pivot coordinates in terms of the
free coordinates.  The stronger theorem
`modqNoLeeKernelVectorBelow_of_systematic_pivot_expansion` proves the exact
`pivotGain + 1` certificate from a pivot-side Lee expansion lower bound: the
extra \(1\) is the nonzero free coordinate contribution.  The remaining
family-specific task is therefore no longer to reason about arbitrary q-ary
kernel vectors; it is to prove a concrete lower bound for
`centeredNormL1 q (systematicPivotImage q P y)` for every nonzero free vector
`y` produced by the Cauchy-plus-tilt construction.

Lean also has a first algebraic sub-certificate,
`modqNoLeeKernelVectorBelow_of_coordinate_reads`.  If each free coordinate has
at least `pivotGain` pivot rows that read exactly that coordinate with
coefficient \(1\) and all other free coordinates with coefficient \(0\), then
the q-ary Lee certificate follows immediately.  This is useful for
direct-coordinate or sparse-identity certificate routes, but the current
Cauchy-solved pivot matrix should not be assumed to satisfy it without an
explicit generated-setup certificate.

The general Lee-expansion route is now factored through
`SystematicLeeExpansionCertificate`.  There is also a support bridge:
`modqNoLeeKernelVectorBelow_of_systematic_support_expansion` says that ordinary
support expansion of the solved pivot map implies Lee expansion, since every
nonzero centered residue contributes at least \(1\).  This bridge is useful for
proof plumbing, but it is far too weak for the current numerical amplitude
thresholds: it can certify at most a support-sized lower bound, while the
concrete candidates need a large centered \(L^1\) bound.  Therefore the real
remaining certificate must be arithmetic, not just Hamming/support based:
either prove a family-specific lower bound on the centered magnitudes of
`B0^{-1} B1 y`, or materialize a checkable exact certificate for the generated
matrix.

A first family-specific algebraic fact has been formalized for the shifted
Cauchy generator.  The theorem
`modqNoLeeKernelVectorBelow_of_duplicate_solved_block` proves that if a rest
block column is literally a duplicate of a selected pivot-block column, then
solving by a left inverse of the pivot block makes that free coordinate an
exact coordinate read.  This captures the un-tilted part of the current
generator: the Cauchy row/column offsets cancel, so the first rest columns
repeat pivot columns before the sparse tilt is added.  The remaining hard
arithmetic is precisely the tilt contribution after multiplication by
`B0^{-1}`: it must be shown to preserve or amplify the centered Lee mass rather
than creating cancellations.

The current strategy split is:

* `fallback-support`: keep the old support-distance lower bound.  It is
  proof-valid but certifies only \(n+1\), so it is mainly a conservative
  baseline.
* direct coordinate/repetition: construct the solved map so free coordinates
  are read directly by pivot rows.  This is already represented by
  `modqNoLeeKernelVectorBelow_of_coordinate_reads`, but it certifies only
  `lee_repetition + 1` and consumes `lee_repetition * m1` pivot rows, so it is
  usually a poor compact-parameter route.
* direct solved Lee certificate: treat the solved systematic graph
  \(y \mapsto (-P y,y)\) as the primitive object and prove or verify a large
  centered Lee lower bound for that graph.  This is the preferred path for
  compact candidates, because it can use arithmetic expansion of `P` rather
  than spending one pivot row per repeated coordinate read.
* sparse/arithmetic expander certificate: generate `P` from a family with a
  deterministic, checkable centered-Lee expansion certificate.  This is the
  deployable version of the direct solved Lee route; the validator should check
  a compact sidecar certificate rather than solve an arbitrary dense
  optimization problem after the fact.

The first shared-row sidecar is implemented by `shared_lee_cert`.  It certifies
two-coordinate free blocks where every pivot row reads both coordinates.  The
checker enumerates all centered pairs whose free-coordinate \(L^1\) mass is
below the target, so a passing certificate rules out every below-target
relation inside that block.  On the current \(q=32771, m_1=32\) 128-bit
amplitude target, four rows per pair came within one unit of the required Lee
bound, while five rows per pair certified the target with \(n=80\).  This is a
real row-sharing improvement over scalar blocks, which needed \(n=128\) for the
same target.

The more general design principle is now captured by `lee_random_code_bound`.
It treats each shared block as a random linear code in the Lee metric.  For
each nonzero free vector, a random row dot product is uniform over the field, so
the tool multiplies exact Lee-ball volume by the exact lower-tail distribution
of random row Lee sums.  This gives a proof-facing route for parameter design:
either formalize a rejection-sampled random-block theorem, or use its row
budgets to choose group sizes before building deterministic sidecar verifiers.
For \(q=32771, m_1=32\), the 80-bit union-bound design predicts total row
budgets of about 104 for groups of 8, 66 for groups of 16, and 46 for one group
of 32.  These are not yet deterministic Lean certificates, but they identify
where the next proof work should focus.

The explicit-algebraic route should be framed as systematic Lee-code
construction.  For a block map \(P : \mathbb F_q^g \to \mathbb F_q^r\), the
needed statement is precisely that the graph code \(y \mapsto (P y,y)\) has
minimum Lee distance at least the q-ary amplitude target.  A separate research
note compares Lee-BCH/RS codes, Vandermonde/moment graph codes, integer
`l1`/RIP-1 expanders, and character/cyclotomic pseudorandom blocks.  The
immediate conclusion is that the basic Lee-BCH designed bound is much too weak
for the current row budgets, so the first explicit target should be
Vandermonde/moment graph codes and the theorem should look like polynomial
value anti-concentration in the Lee metric.

The first Vandermonde probe narrowed the viable family.  Naive degree-zero
moment columns and midpoint/geometric scalings have concrete one-coordinate
Lee counterexamples.  Shifted moments
\(P_{t,j}=c_j \alpha_t^{1+j}\) with deterministic random nonzero column scales
survive the current bounded attacks at the random-code row budgets; for the
128-bit `q=32771, L=3277, g=32` shape, seed 1 fails at `r=41` and survives at
`r=42`, with a deeper `r=46` run still bottlenecked by a one-coordinate vector.
This suggests the Lean target should be a family-specific polynomial
anti-concentration certificate for shifted, scaled moment graph codes, not a
plain Vandermonde/MDS theorem.

This target now has a Lean module boundary in `Descender.MomentLee`.  The module
defines `shiftedMomentMatrix`, `finShiftedMomentMatrix`,
`MomentLeeAntiConcentration`, and
`modqNoLeeKernelVectorBelow_of_moment_antiConcentration`.  Therefore the
remaining hard proof is exactly the anti-concentration lemma for the selected
public scales and evaluation points; once that lemma is supplied, the q-ary Lee
kernel theorem follows without changing the downstream cryptographic layer.

A direct finite CP-SAT checker now encodes the same target for concrete probe
artifacts.  It verifies the known `g=32,r=40` counterexample, but returns
`UNKNOWN` on the candidate `g=32,r=42` instance after a 120 second run.  This
means the candidate has not collapsed, but it is still not a proof-side
certificate.

The finite search is now split by support.  For `g=32,r=42`, exhaustive
support-1 enumeration certifies the one-column minimum as `3349 >= 3277`.
Anchored coordinate-descent attacks for support budgets 2 through 4 did not
improve that bottleneck.  The remaining proof gap is therefore specifically the
support-at-least-2 anti-cancellation theorem or an exhaustive/certified
replacement for it.  The first replacement is now available: an exact support-2
scanner enumerated `10643068800` two-coordinate assignments for this same block
and found no vector below `3277`.  The concrete finite gap now starts at
support at least 3.

The direct real-sample cancellation is now formalized:

```lean
residuePairing_realSample_eq_noise_of_annihilates_eval
dualIntervalAccepts_realSample_of_pairing_bound
```

So the proof stack does not merely name the new condition.  It proves that any
dual relation annihilating the public evaluation map makes real samples land in
the corresponding noise interval.  The remaining security-side work is the
null/uniform counting theorem and an implementation proof that the selected
public family satisfies `NoNarrowQaryDualRelations`.

Do not block the deterministic finite-field proof on this layer.

## 12. Cryptographic Reduction Layer

Files:

```text
PTND.lean
Security.lean
```

The first abstract reduction layer now exists.  `PTND.lean` defines:

```lean
P_TND_Search
P_TND_Decision
```

and proves that a real decision sample with centered noise bounded by `eta`
becomes a valid planted search instance:

```lean
P_TND_Decision.realSearch_success_of_centered_noise
P_TND_Decision.realSearch_hasSolution_of_centered_noise
P_TND_Search.uniqueSolution_of_evalSeparated
P_TND_Decision.realSearch_planted_unique_of_centered_noise_and_separated
```

The separation predicate `P_TND_Search.EvalSeparated` is the deterministic
decoding/correctness condition: every pair of distinct encoded secrets must be
more than `2 * eta` apart in centered `linf`.  Under that condition, bounded
centered noise has a unique planted solution.  Future KEM correctness should
instantiate this predicate from the proven primal-distance and concrete noise
radius.

`Security.lean` defines the reduction calculus:

```lean
Reduction
Hard
DescenderSecurityProblems.ReductionPipeline
FOTransformProof
CPAToAveragePTNDProof
AveragePTNDToWorstCaseProof
DescenderKEMSecurityCertificate
DescenderCPACore
DescenderCPACore.FOImplicitRejectionProof
DescenderCPACore.AverageToWorstCaseReduction
ProjectedCoboundaryFamily
WorstCaseProjectedCoboundaryProblem
DescenderCPACore.AverageToWorstCaseSearchReduction
DescenderCPACore.SelfRandomizationReduction
DescenderKEMCertificate
```

Nonuniform QPT remains an external assumption interface; Lean should not try to
formalize quantum computation at this stage.  The required cryptographic work is
to instantiate the named certificate fields.  One field is now supplied by the
concrete CPA core:

```lean
DescenderCPACore.cpaToAveragePTNDReduction
DescenderCPACore.cpaToAveragePTNDProof
DescenderCPACore.no_cca_break_of_concrete_cpa
DescenderCPACore.no_cca_break_of_concrete_reductions
```

`DescenderCPACore` takes a `P_TND_Decision` instance as the public sample game.
A CPA adversary is a Boolean distinguisher on ciphertext/sample vectors, and
`BreaksCPA` is exactly `P_TND_Decision.Distinguishes`.  Therefore the
CPA-to-average-P-TND reduction is the identity map on distinguishers, proved in
Lean.

The remaining certificate work is:

1. implement the FO simulator map and prove
   `DescenderCPACore.FOImplicitRejectionProof.sound`;
2. implement the average-to-worst-case search builder and prove
   `DescenderCPACore.AverageToWorstCaseSearchReduction.sound`;
3. prove or assume the worst-case projected-coboundary hardness predicate for the declared
   adversary/solver class.

`ProjectedCoboundaryFamily` is the concrete worst-case search side.  It declares
which `P_TND_Search` instances are in the worst-case family, its solvers are
functions that return a secret for every declared instance, and `Solves` requires
the returned secret to satisfy the centered nearest-coboundary success predicate.
The search reduction also carries `W.Complete`, so the theorem cannot be
certified by placing an unsolvable instance in the worst-case family and calling
that vacuous hardness.

Hardness is not stated against all mathematical functions `W.Solver`; that would
contradict completeness by classical choice.  `WorstCaseProjectedCoboundaryProblem`
separates the declared solver algorithm class from the semantic solver function
it computes:

```lean
WorstCaseProjectedCoboundaryProblem.SolverAlg
WorstCaseProjectedCoboundaryProblem.run
WorstCaseProjectedCoboundaryProblem.Solves
```

The average-to-worst theorem is now explicit:

```lean
DescenderCPACore.AverageToWorstCaseSearchReduction.builds_worst_case_solver
DescenderCPACore.AverageToWorstCaseSearchReduction.hard_average_ptnd_of_worst_case_hard
```

The self-randomization proof object now constructs this reduction:

```lean
DescenderCPACore.SelfRandomizationReduction
DescenderCPACore.SelfRandomizationReduction.toAverageToWorstCaseSearchReduction
DescenderCPACore.SelfRandomizationReduction.average_query_is_real
DescenderCPACore.SelfRandomizationReduction.builds_worst_case_solver
DescenderCPACore.SelfRandomizationReduction.hard_average_ptnd_of_worst_case_hard
DescenderCPACore.no_cca_break_of_self_randomization
```

Its fields are the actual proof obligations for Descender's sampler:

```lean
randomize      -- turn a worst-case instance into an average-looking P-TND query
randomize_real -- prove that query lies in the real average sample set
recover        -- extract a worst-case secret from the average distinguisher
recover_sound  -- prove recovery succeeds on every worst-case instance
solverAlg      -- package the recovery algorithm in the declared solver class
run_solverAlg  -- prove the packaged solver computes the recovered secret
```

So the remaining mathematical content is no longer the reduction scaffolding.  It
is proving these self-randomization fields for the actual Descender public
family and noise distribution.

The top-level system certificate is now:

```lean
DescenderKEMCertificate
DescenderKEMCertificate.ind_cca_secure
DescenderKEMCertificate.ind_cca_secure_for_declared_class
```

It bundles the abstract security composition with:

1. transparent public setup and no hidden setup-side state;
2. compact public evaluator/size/cost certificates;
3. nonuniform-QPT adversary-class metadata;
4. q-ary/noise promise interfaces;
5. FO implicit-rejection soundness;
6. self-randomization;
7. worst-case projected-coboundary hardness for the declared solver algorithm
   class.

Once those reductions are supplied, Lean already checks the composition:

```lean
DescenderSecurityProblems.no_cca_break_of_worst_case_hard
DescenderSecurityProblems.no_cpa_break_of_worst_case_hard
DescenderKEMSecurityCertificate.no_cca_break
descender_kem_ind_cca_secure
```

## 13. Concrete Parameter Instantiation Workflow

The proof layer is ready to drive concrete-family search.  The workflow is now:

1. use the Rust engine `tools/parameter-search` to enumerate candidate profiles
   in either exhaustive grid mode or simulated annealing mode, with
   `scripts/parameter_search/search.py` retained as a reference implementation;
2. reject candidates that violate proven side conditions such as `m0 >= n` and
   `q > n + m0 + m1`;
3. rank survivors by size, correctness slack, and explicitly labeled screening
   proxies;
4. run exact finite-field and dual-witness checks for the Pareto frontier;
5. instantiate the surviving public setup/evaluator/noise promises as fields of
   `DescenderKEMCertificate`.

Current generated artifacts:

```text
tools/parameter-search/Cargo.toml
tools/parameter-search/src/main.rs
research/parameter_search/descender-128.json
research/parameter_search/descender-128.md
research/parameter_search/descender-192.json
research/parameter_search/descender-192.md
research/parameter_search/descender-256.json
research/parameter_search/descender-256.md
```

The first concrete key-size optimization is modeled by:

```text
--coefficient-model seeded
```

This corresponds to replacing per-wedge stored fiber coefficients by a
transparent seeded coefficient generator plus verifier-side local-independence
checks.  The abstract Lean hook for this is now `Descender.SeededSetup`, where
`SeededProjectionCertificate` proves that public verification accepts exactly
the deterministic regenerated data and that regenerated data satisfies the
explicit setup predicate consumed by the proof.

The second key-size optimization is modeled by:

```text
--incidence-model seeded
```

This corresponds to replacing explicit stored wedge endpoint and diagonal data
by public thinning/diagonal seeds plus deterministic regeneration.  A future
Lean setup certificate should expose these seeds, define the regenerated wedge
set and diagonal as public functions, and prove that they match the objects
consumed by the deterministic transversality, wedge-rank, and compact evaluator
interfaces.

The combined public proof boundary is:

```lean
SeededPublicKEMLayer
SeededPublicKEMLayer.supplies_public_proof_inputs
```

It packages the existing `TransparentSetupCertificate` and
`CompactOperatorCertificate` with a seeded-regeneration certificate.  This does
not prove that a concrete sampler has the desired combinatorics; it turns the
seeded representation into a theorem-level obligation instead of an informal
key-size optimization.

The next parameter-search mode is:

```text
--objective bandwidth
```

This mode should be used before attempting to relax theorem hypotheses.  It
keeps the current Lean-side proof constraints as hard filters and asks how much
measurement/ciphertext size can be reduced by parameter choice alone.  Only if
the bandwidth Pareto frontier repeatedly hits the same proof-side constraint
should that constraint become a theorem-improvement target.

The current intended public-key representation is:

```text
--public-key-model canonical
```

At the proof boundary this means a concrete candidate should instantiate one
master public seed and deterministic domain-separated expanders for:

- canonical Cauchy guard/base blocks;
- sparse wedge incidence;
- local wedge coefficients;
- diagonal tilt coefficients.

The serialized public key still contains the noisy public sample `t = A s + e`.
The Cauchy description and rule descriptors are no longer per-key data; they
are part of the parameter-set specification and must be reproduced by the
`SeededProjectionRegenerator`.

The companion exact validator is:

```text
validate_frontier
```

It now materializes deterministic public coefficient, incidence, and diagonal
seeds for saved bandwidth-frontier rows.  The generated sparse tilt matrix is
combined with Cauchy guard/base blocks, then checked for full rank, guard-block
rank, three-sparse tilt columns, guard distance, and bounded dual witnesses.
This is the first concrete candidate interface that should later be lifted into
`SeededProjectionRegenerator` and `SeededProjectionCertificate`.

The JSON files are the machine-readable source of truth.  The Markdown files are
review summaries.  The reported `objective_score` and `security_proxy_bits`
values are not Lean theorems and not cryptanalytic claims; they only prioritize
candidates for the next exact-search and estimator passes.

The first concrete Lean-instantiation milestone should not try to prove a new
security theorem.  It should build one conservative candidate object whose
fields match the existing certificate:

```lean
DescenderKEMCertificate
```

and then replace abstract assumptions one by one with concrete transparent
setup, compact evaluator, q-ary/noise, FO, and self-randomization witnesses.

`Security.lean` also defines `PTNDCorrectnessCertificate`, which packages the
centered-noise bound and separation condition needed for planted search
correctness:

```lean
PTNDCorrectnessCertificate.planted_success
PTNDCorrectnessCertificate.planted_has_solution
PTNDCorrectnessCertificate.planted_unique
```

This is the current practical KEM boundary.  A candidate construction can claim
formal IND-CCA security only by producing a `DescenderKEMSecurityCertificate`.
It can claim formal correctness of the planted noisy target only by producing a
`PTNDCorrectnessCertificate` from the deterministic primal-distance theorem and
the concrete noise radius.

`SampleView.lean` now adds the proof boundary for publishing a compressed or
quotient view of a P-TND sample instead of the full measurement vector.  A
`SampleView q FullCoord ViewCoord` is an additive public map

```lean
(FullCoord -> ZMod q) -> (ViewCoord -> ZMod q)
```

and it defines the restricted decision instance consumed by the compressed
KEM surface.  The key proved facts are:

```lean
SampleView.distinguishes_pullback_of_restrict
SampleView.correctnessCertificate
SampleView.restricted_planted_unique
```

This is the formal version of the challenge-space idea: an adversary against a
viewed sample pulls back to an adversary against the full sample, but the viewed
scheme is correct only after proving two explicit obligations:

```lean
SampleView.NoiseSmall
SampleView.EvalSeparated
```

`NoiseSmall` says the viewed noise remains bounded in centered norm.
`EvalSeparated` says the viewed operator still separates distinct secrets by
more than twice the viewed noise radius.  Any quotient/syndrome optimization
must now target these two predicates; simply compressing away coordinates is
not a valid proof step unless the restricted operator satisfies them.

Non-free viewed-sample reductions are represented by:

```lean
SampleView.ViewReductionBudget
SampleView.correctnessCertificateWithBudget
```

`ViewReductionBudget.viewEta` is the effective viewed noise radius after
rounding, quantization, or other lossy publication.  `failureBudget` is reserved
for the probabilistic layer when the view construction preserves separation
only outside an explicit bad event.

The support-to-amplitude bridge has a deterministic Lean boundary:

```lean
SampleView.scaledCoordinateRestriction
SampleView.DiagonalAmplifiesSupport
SampleView.SupportCoveredByView
SampleView.exists_selected_pair_nonzero_of_supportCoveredByView
SampleView.exists_selected_pair_nonzero_of_surjective
SampleView.scaledCoordinateRestriction_evalSeparated_of_amplifies
SampleView.scaledCoordinateRestriction_correctnessCertificate_of_amplifies
```

`DiagonalAmplifiesSupport` is the exact "good public diagonal" event: every
pair of distinct secrets whose unscaled difference has enough Hamming support is
separated in centered `L∞` after selecting coordinates and applying the public
diagonal.  This lets the Cauchy/support theorem feed the dense-noise
correctness certificate once a later probabilistic lemma proves that a sampled
diagonal satisfies the event with high probability.

The noise side is intentionally not hidden.  Arbitrary diagonal scaling does not
preserve a pre-existing small centered noise vector.  A concrete protocol must
either sample the error in the scaled view coordinates, restrict to scalings
that preserve the chosen noise law, or pay an explicit `ViewReductionBudget`.
The certificate theorem therefore still requires `SampleView.NoiseSmall` for
the scaled view.

`DiagonalAmplification.lean` adds the finite-counting layer that the randomized
diagonal proof must discharge:

```lean
SampleView.FullDiagonalSeed
SampleView.fullDiagonalScale
SampleView.GoodDiagonalSeed
SampleView.BadPairDiagonalSeed
SampleView.badPairDiagonalSeeds
SampleView.CoordinateBadScale
SampleView.coordinateBadScales
SampleView.centeredSmallResidues
SampleView.coordinateBadScales_card_eq_centeredSmallResidues_of_isUnit
SampleView.centeredSmallResidues_card_le_interval
SampleView.coordinateBadDiagonalSeeds_full_card_eq
SampleView.badPairDiagonalSeeds_full_card_le_coordinateBadScales_mul
SampleView.badPairDiagonalSeeds_full_card_le_interval_mul_of_isUnit
SampleView.badPairDiagonalSeeds_full_card_le_interval_mul_of_prime_nonzero
SampleView.badDiagonalSeeds
SampleView.DiagonalAmplificationFailureCountAtMost
SampleView.badDiagonalSeeds_card_le_sum_badPairDiagonalSeeds
SampleView.failureCountAtMost_of_pairwise_badPair_bounds
SampleView.failureCountAtMost_full_of_pairwise_selected_unit_differences
SampleView.failureCountAtMost_full_of_pairwise_selected_nonzero_differences_prime
SampleView.failureCountAtMost_full_of_supportCovered_select_prime
SampleView.failureCountAtMost_full_of_surjective_select_prime
SampleView.exists_good_diagonal_of_failureCountAtMost
SampleView.exists_scaledCoordinateRestriction_correctnessCertificate_of_failureCount
SampleView.exists_scaledCoordinateRestriction_correctnessCertificate_full_of_supportCovered_select_prime
SampleView.exists_scaledCoordinateRestriction_correctnessCertificate_full_of_surjective_select_prime
```

The probabilistic theorem can now be stated as a finite bad-seed bound.  If the
number of bad seeds is less than the seed-space size, Lean constructs a good
diagonal seed and the corresponding scaled-view correctness certificate.
The concrete generator is `FullDiagonalSeed q ViewCoord = ViewCoord -> ZMod q`,
i.e. one public scale per viewed coordinate.  The global bad event is decomposed
by an ordered-secret-pair union bound, so the remaining theorem can focus on a
single pair's bad-scale count.  For a chosen viewed coordinate `j`, Lean proves
that the pair bad-seed count is at most:

```text
#(coordinateBadScales delta eta) * q^(#ViewCoord - 1)
```

where `delta` is the selected coordinate difference for that pair.  Thus the
scalar arithmetic layer is now closed: if `delta` is a unit, this bad-scale set
has the same cardinality as centered residues with absolute value at most
`2*eta`, and that set has size at most `2*(2*eta)+1`.  Over a prime field this
applies to every nonzero selected coordinate difference.  The current remaining
sample-view hypothesis is therefore geometric: every support-heavy distinct
secret pair must have at least one viewed coordinate on which the two
evaluations differ nontrivially.  Under that hypothesis Lean now proves the
global full-diagonal bad-seed bound by a union bound over ordered secret pairs.
For compressed views this geometric hypothesis is now exposed as
`SupportCoveredByView`: the selected coordinates must hit the support of every
support-heavy secret-pair difference.  The theorem
`failureCountAtMost_full_of_supportCovered_select_prime` turns this hitting-set
condition into the same full-diagonal bad-seed bound.  For the uncompressed/full
selected-coordinate view, `exists_selected_pair_nonzero_of_surjective` proves
the support-covering condition from `Function.Surjective select` plus
`0 < minSupport`.

The compression-facing end-to-end theorem is
`exists_scaledCoordinateRestriction_correctnessCertificate_full_of_supportCovered_select_prime`.
It says that over a prime field, a selected view satisfying `SupportCoveredByView`
for all relevant pair differences, positive support threshold, support-distance
theorem, viewed-noise theorem, and the finite bad-budget inequality together
produce an actual full diagonal seed and a scaled-view P-TND correctness
certificate.  The full-view special case is
`exists_scaledCoordinateRestriction_correctnessCertificate_full_of_surjective_select_prime`.

`StrategyComparison.lean` keeps these alternatives as first-class certificate
routes rather than treating proof development as a single mutable theorem:

```lean
SampleView.StrategyKind
SampleView.StrategyCostProfile
SampleView.StrategyCertificate
SampleView.fullSurjectiveStrategyCertificate
SampleView.supportCoveredStrategyCertificate
SampleView.lossyBudgetStrategyCertificate
```

The intended portfolio is:

* `fullSurjective`: conservative baseline, closest to the original proof route;
* `supportCovered`: exact structural compression by publishing a selected
  coordinate family that hits every relevant support;
* `lossyBudget`: non-free compression where rounding/quantization is paid for
  by an explicit viewed-noise budget.

This is deliberate.  Key size is not the only objective: setup validation,
key-generation work, encapsulation/decapsulation work, and proof assumptions
must be compared side by side.  A later KEM profile can keep the conservative
proof as a validation baseline while selecting a smaller strategy only when its
extra obligations are instantiated.

The concrete parameter search now mirrors this distinction.  It records both
the proof strategy and the viewed-coordinate count:

```text
--proof-strategies full-surjective,support-covered,lossy-budget
--view-coords 0,48,64,80,96,128,...
```

`view-coords = 0` means the old full public sample of all \(m_0+m_1\)
coordinates.  Nonzero values model a smaller public sample and ciphertext
coordinate vector.  The full/surjective route is filtered out unless the view
is full; support-covered and lossy-budget rows are retained as proof obligations
for exact hitting-set validation or explicit viewed-noise-budget validation.

The concrete view model used by the search selects a prefix of the combined
coordinate list, with guard coordinates first.  A compressed row may still use
the guard-only primal-distance theorem when `view_coords >= m0`, since the
whole guard block is still published.  The validator rejects only rows that use
guard-only correctness after hiding some guard coordinates, reporting
`guard_only_requires_guard_view`.

The validator now reports which reduced-view route is closest to being
certifiable.  `support-covered-needs-hitting-set` means the row must supply the
strong `SupportCoveredByView` hypothesis for every relevant secret-pair
difference.  `guard-block-lossy-candidate` means the selected view contains all
guard coordinates and the lossy viewed-noise budget is absorbed into `viewEta`;
it still needs the support-distance-to-amplitude-separation bridge before it can
prove `SampleView.EvalSeparatedWithBudget` for centered P-TND.
`guard-block-lossy-diagonal-candidate` means the same guard-prefix route also
passes the current multi-coordinate diagonal-amplification screening proxy but
has not yet met every exact certificate precondition.  A row is upgraded to
`guard-block-lossy-diagonal-certified` when the validator can instantiate the
Lean-facing pair-coordinate theorem: the selected prefix contains the full
Cauchy guard block, \(q\) is prime, the guard Cauchy distance supplies the
pair-coordinate support lower bound, the lossy viewed-noise budget passes, and
the diagonal bad-seed inequality has nonnegative slack.  This is the first
implemented reduced-view validation route.
`lossy-budget-candidate` means the row passes the cheap checks for the two Lean
obligations in `LossyBudgetStrategyObligations`, but does not yet have a
specific guard-prefix or combined-distance certificate.

The end-to-end certificate boundary is now `KEMProfile.lean`:

```lean
SampleView.SupportCoveredStrategyObligations
SampleView.LossyBudgetStrategyObligations
SampleView.LossyBudgetDiagonalStrategyObligations
SampleView.LossyBudgetDiagonalGenericObligations
SampleView.LossyBudgetPairCoordinateStrategyObligations
SampleView.LossyBudgetUniformPairCoordinateStrategyObligations
ExactValidationCertificate
ValidatedStrategyCertificate
CertifiedKEMProfile
```

A deployed profile must provide all four layers: a correctness strategy
certificate, exact validation evidence for the concrete row, the q-ary/noise and
self-randomization security certificate, and the final KEM certificate proving
IND-CCA security.  Smaller viewed-coordinate rows remain search candidates, but
the validator marks them as missing a view-reduction certificate until the
support-covered or lossy-budget proof is supplied.

The guard-shape optimization is tracked as a separate primal-distance model:

```text
--primal-distance-models guard-only,combined-proxy
```

`guard-only` is the current theorem path and requires \(m_0 \ge n\).
`combined-proxy` prices correctness distance from the viewed combined
measurement matrix instead of the guard block alone.  It is intentionally marked
as a missing certificate by validation until a theorem replaces the guard-only
Cauchy hypothesis with a combined-matrix primal-distance certificate.
Use direct `--m0-values` for this search branch; `--m0-overheads` always keeps
\(m_0 \ge n\) and therefore cannot test the intended dimension reduction.

The deterministic adapter theorem is now formalized:

```lean
guarded_tilt_primal_distance_combined_operator
guarded_tilt_deterministic_transversality_combined
GuardedCombinedDeterministicExportBundle
descender_deterministic_core_primal_combined
descender_deterministic_core_combined
```

This closes the Lean plumbing for the combined-distance route.  What remains for
a concrete `combined-proxy` parameter row is the actual matrix-specific
certificate proving the full primal output has the required distance.

The parameter search now keeps this proxy close to that certificate obligation
without running expensive minimum-distance certification inside the scan.  Each
row records:

- `combined_rank_deficit = max(0, n - m0)`;
- `combined_distance_gap = max(0, required_distance - guard_distance)`;
- `combined_certification_margin = correctness_slack - combined_distance_gap`;
- `combined_certification_pressure = combined_rank_deficit +
  combined_distance_gap`.

The objective subtracts
`--combined-certification-weight * combined_certification_pressure`, with a
default weight of `0.25`.  This biases search toward candidates where the later
combined certificate only has to repair a small guard deficit.  It does not make
the row accepted: validation still reports
`combined_primal_distance_certificate_missing` until the concrete matrix proof
or exact certificate is supplied.

Success criterion:

Lean checks the security skeleton under explicit reductions and a worst-case
hardness assumption:

```lean
DescenderKEMSecurityCertificate -> NoBreak BreaksCCA
```

## 14. Recommended Milestones

### Milestone A: Lean project compiles

Create a Lake project with Mathlib and empty module skeletons.

### Milestone B: support and weight library

All Hamming support lemmas compile.

### Milestone C: guarded tilt theorem without Cauchy

Assume `CauchySupportBound B` abstractly and prove the guarded transversality theorem.

This avoids the hardest algebra at first.

### Milestone D: peeling rank theorem

Prove peelability plus local independence implies column independence.

This is the first genuinely Descender-specific verified result.

### Milestone E: Cauchy support theorem

Prove the Cauchy/MDS support lower bound and remove the abstract assumption from Milestone C.  Also prove the guard-block primal-distance theorem with the explicit dimension hypothesis \(rows.length\le cols.length\), and keep the unrestricted false statement out of public theorem surfaces.

### Milestone F: combined deterministic core

Export:

\[
d_{\mathrm{prim}}\ge m_0-n+1,
\qquad
d_{\mathrm{dual}}> (n+1)/(L+1),
\qquad
\operatorname{rank}(H_S)=|S|.
\]

### Milestone G: finite random thinning

Formalize explicit finite probability bounds for small-core exclusion.

### Milestone H: HDX regularity interface

Formalize typed-face regularity axiomatically and connect it to wedge-sampler regularity.

### Milestone I: q-ary lattice strengthening

Done for the first dual-only bridge: Hamming no-kernel transversality now rules
out nontrivial short integer residue relations in \(L^1\).  Remaining work:
smoothing-style primal/dual lattice conditions and quantitative noise
anti-concentration.

### Milestone J: P-TND reduction skeleton

Done as an honest certificate interface: `Security.lean` formalizes reduction
composition, exposes `descender_kem_ind_cca_secure`, and packages P-TND planted
correctness through `PTNDCorrectnessCertificate`.

The sample-view boundary is also in place: `SampleView.lean` proves that
compressed/quotient sample publication is proof-compatible exactly when the
viewed operator preserves noise boundedness and separation.  This lets the
parameter work explore restricted challenge spaces without silently weakening
the KEM proof.

The first concrete viewed-sample theorem is now the coordinate-restriction
view:

```lean
SampleView.coordinateRestriction
SampleView.coordinateRestriction_noiseSmall_of_full
SampleView.coordinateRestriction_evalSeparated_of_selected_linf
SampleView.coordinateRestriction_correctnessCertificate
```

This captures the architecture "publish only the coordinates selected by
`select : ViewCoord -> FullCoord`."  The result is deliberately asymmetric:
coordinate restriction automatically preserves a coordinatewise centered-noise
bound, but it does not automatically preserve secret separation.  The selected
coordinates must satisfy a centered `L∞` separation condition:

```lean
∀ s t, s ≠ t ->
  2 * eta < centeredNormLinf q
    (fun j : ViewCoord => P.eval s (select j) - P.eval t (select j))
```

This is stronger/different than the current deterministic Cauchy guard theorem,
which is a Hamming support-distance statement.  Therefore the next proof target
for public-sample dimension reduction is a genuine amplitude/separation lemma
for the selected challenge coordinates, not merely another support lower bound.

The next concrete bridge is also present:

```lean
SampleView.scaledCoordinateRestriction
SampleView.DiagonalAmplifiesSupport
SampleView.scaledCoordinateRestriction_correctnessCertificate_of_amplifies
```

This converts support distance into the dense `L∞` separation certificate under
an explicit deterministic good-diagonal hypothesis.  The remaining mathematical
work is probabilistic: prove that the proposed public diagonal/challenge
sampling distribution satisfies `DiagonalAmplifiesSupport` with the claimed
failure probability and handle the corresponding viewed-noise distribution.
The first part is now reduced to the finite-counting predicate
`SampleView.DiagonalAmplificationFailureCountAtMost`.  The current proof path is
to prove pairwise estimates for `SampleView.badPairDiagonalSeeds`, sum them with
`SampleView.failureCountAtMost_of_pairwise_badPair_bounds`, and then apply
`SampleView.exists_scaledCoordinateRestriction_correctnessCertificate_of_failureCount`.
The pairwise scalar estimate is complete for unit coordinate differences:
`SampleView.badPairDiagonalSeeds_full_card_le_interval_mul_of_isUnit` bounds the
per-pair bad set by `(2*(2*eta)+1) * q^(#ViewCoord-1)`, and the prime-field
corollary replaces the unit condition by nonzero difference.  The remaining
proof obligation is the selected-view separation condition recorded by
`SampleView.failureCountAtMost_full_of_pairwise_selected_unit_differences`: each
support-heavy distinct pair needs a viewed coordinate with a unit difference.
For support-covered selected views this is now proved in Lean by
`SampleView.failureCountAtMost_full_of_supportCovered_select_prime`; for
surjective selected views it follows from
`SampleView.failureCountAtMost_full_of_surjective_select_prime`.  The remaining
research issue for compression is now concrete: produce explicit small selected
coordinate families satisfying `SupportCoveredByView`, or replace it with an
analogous lossy separation theorem carrying a `ViewReductionBudget`.

The support/Hamming correctness path is also formalized separately:

```lean
P_TND_Search.SupportSuccess
P_TND_Search.SupportEvalSeparated
P_TND_Search.supportUniqueSolution_of_supportEvalSeparated
P_TND_Decision.realSearch_supportSuccess_of_support_noise
PTNDSupportCorrectnessCertificate
```

This is the correct home for the existing Cauchy guard theorem.  The Cauchy
guard theorem proves many nonzero output coordinates, so it naturally supports
a sparse-noise/code-style P-TND correctness statement.  It does not by itself
prove the dense centered-amplitude `L∞` statement used by the current
`PTNDCorrectnessCertificate`.

Decision rule:

* dense small-amplitude noise uses `PTNDCorrectnessCertificate` and needs
  centered `L∞` separation;
* sparse support noise uses `PTNDSupportCorrectnessCertificate` and can consume
  Hamming/Cauchy support distance directly;
* lossy sample-view compression uses
  `SampleView.correctnessCertificateWithBudget` once the viewed operator
  satisfies the dense-amplitude obligations at the effective radius.

Remaining work is not another skeleton.  It is the actual proof content for the
certificate fields:

1. define the concrete CCA KEM game and random-oracle transform interface as an
   implementation of `FOImplicitRejectionProof`;
2. prove the average-to-worst-case projected-coboundary reduction by filling a
   `SelfRandomizationReduction`;
3. instantiate q-ary/noise promises for the selected family, including
   `NoNarrowQaryDualRelations` at the concrete effective noise radius;
4. instantiate worst-case hardness assumptions for the selected family and
   adversary class;
5. for any reduced public-sample view, prove `SampleView.NoiseSmall` and
   `SampleView.EvalSeparated` for the selected challenge space.

## 15. Engineering Rules for the Lean Development

Keep early definitions finite and concrete. Avoid general category-theoretic sheaves until the finite combinatorial version is stable.

Prefer theorem predicates over computable optimization definitions. For example, use `HasMinWeightAtLeast` before defining exact minimum distance.

Separate assumptions from constructions. Prove guarded tilt from a `CauchySupportBound` assumption first, then prove Cauchy matrices satisfy it.

Avoid probability until the deterministic core compiles.

Use abstract interfaces for HDX regularity first. Instantiating Ramanujan or Chevalley HDXs is a separate research project.

Keep theorem statements close to the math:

```lean
weight r < threshold -> AT r != 0
```

is usually easier than encoding exact minimum-distance objects.

## 16. First Theorem to Formalize

The first nontrivial theorem should be:

\[
\boxed{
(\tau,k)\text{-peelability}
+
\text{local }k\text{-wise independence}
\Rightarrow
\operatorname{rank}(D^\ast C)_S=|S|
}
\]

Reasons:

1. It is central to the wedge sampler.
2. It uses only finite sets and linear independence.
3. It is independent of probability, HDX spectral theory, Cauchy matrices, and cryptographic games.
4. It validates the most novel part of the Descender construction.

The second theorem should be guarded tilted transversality with abstract Cauchy support and abstract column-locality assumptions.

Together, these two results form the first serious computer-assisted proof core for Descender.
