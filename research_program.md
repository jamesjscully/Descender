# Descender Research Program

Descender is a proof-first research program for a post-quantum key encapsulation mechanism based on noisy measured descent over transparent sheaf complexes. The final design reached in `chat.md` is a guarded tilted exact-gradient projection built from three pieces:

1. a transparent descent operator on an expanding sheaf complex,
2. a balanced exact-gradient guard code, and
3. an HDX-local wedge sampler that injects controlled descent geometry.

The project goal is not to propose another heuristic structured LWE variant. The goal is to construct a public family for which breaking the KEM reduces to a canonical worst-case nearest projected coboundary problem, including against nonuniform quantum polynomial-time adversaries.

## 1. Core Thesis

ECC and related pre-quantum systems hide a coordinate inside an exact low-dimensional algebraic action. Shor-type quantum algorithms break that pattern by turning the hidden coordinate into a Fourier-readable period, kernel, stabilizer, or hidden subgroup.

Descender follows a different design rule:

\[
\text{Do not hide a coordinate as an exact Fourier-readable symmetry.}
\]

Instead, the secret is a local trivialization:

\[
s \in C^0(X,\mathcal F)/H^0(X,\mathcal F),
\]

and the public key is noisy projected descent data:

\[
t = A s + e.
\]

Here \(A\) is not an arbitrary matrix. It factors through a public sheaf coboundary:

\[
C^0(X,\mathcal F)/H^0
\xrightarrow{D=d^0}
C^1(X,\mathcal F)
\xrightarrow{\Pi}
M.
\]

The hard problem is therefore:

\[
\textbf{Projected Transparent Noisy Descent:}
\quad
\text{recover or distinguish } s \text{ from } t=\Pi D s+e.
\]

The intended security statement is:

\[
\text{IND-CCA break}
\Rightarrow
\text{P-TND break}
\Rightarrow
\text{worst-case nearest projected coboundary solver.}
\]

## 2. Mathematical Setting

For a security parameter \(\lambda\), public setup chooses:

\[
(X_\lambda,\mathcal F_\lambda,D_\lambda,\Pi_\lambda).
\]

The base object is a finite typed high-dimensional expander \(X\), preferably a local-spectral and cosystolic HDX. The coefficient sheaf \(\mathcal F\) is initially the constant sheaf with fiber \(\mathbb F_q^k\), though the proof framework should allow expanding coefficient sheaves later.

Let:

\[
V = C^0(X,\mathcal F)/H^0(X,\mathcal F),
\]

\[
W = C^1(X,\mathcal F),
\]

\[
D=d^0:V\to W.
\]

The setup must ensure \(D\) is injective on \(V\), with controlled conditioning or an explicit finite-field substitute. The public map used by the KEM is:

\[
A=\Pi D:V\to M.
\]

The canonical worst-case problem is nearest projected coboundary:

\[
\text{given } y\in M,\quad \text{find } b\in A(V) \text{ minimizing } \|y-b\|.
\]

Equivalently, decode near the projected coboundary code \(A(V)\), or study the associated \(q\)-ary sheaf lattices.

## 3. TND-Admissible Families

A public family \((X_\lambda,\mathcal F_\lambda,\Pi_\lambda)\) should be called TND-admissible if it satisfies the proof obligations below.

**Controlled kernel.** After quotienting by global sections, \(D:V\to W\) is injective and well-conditioned.

**Cosystolic distance.** Nonzero local-coordinate differences produce globally visible descent data. Nearest projected coboundary has a meaningful decoding radius.

**No low-weight dual relations.** The dual map

\[
A^\ast=D^\ast\Pi^\ast
\]

has no small nonzero kernel vectors and no short \(q\)-ary lattice relations.

**Local spectral expansion.** The HDX supplies mixing, concentration, and local-to-global control.

**Projection as lossless sampler.** \(\Pi\) must behave like a lossless sampler or condenser on \(\operatorname{im}(D)\), while its adjoint is transverse to \(\ker D^\ast\).

**Self-randomization.** The family should support randomizing hard instances into average-looking P-TND instances.

**Primal-dual symmetry.** The KEM uses both \(A\) and \(A^\ast\), so hardness and expansion must hold on both sides.

## 4. Final Projection Design

The final projection design should be built through the adjoint:

\[
J=\Pi^\ast:M^\ast\to W.
\]

Rather than choosing \(\Pi\) directly, Descender designs \(J\) as a guarded tilted exact-gradient embedding.

Choose a public left-inverse lift:

\[
R:W\to V,
\qquad
RD=I_V.
\]

Equivalently, define:

\[
\widetilde D=R^\ast,
\qquad
D^\ast\widetilde D=I_V.
\]

Split the measurement space:

\[
M=M_0\oplus M_1,
\qquad
\dim M_0=m_0,\quad \dim M_1=m_1.
\]

The guard block is exact-gradient. The tilted block is exact-gradient plus local HDX descent sampling.

Let:

\[
B_0:M_0^\ast\to V,
\qquad
B_1:M_1^\ast\to V,
\]

\[
C:M_1^\ast\to W,
\qquad
\Delta=\operatorname{diag}(\theta_1,\ldots,\theta_{m_1}).
\]

Define:

\[
J(r_0,r_1)
=
\widetilde D B_0 r_0
+
\widetilde D B_1 r_1
+
C\Delta r_1.
\]

Then the actual public map is:

\[
A=J^\ast D
=
\begin{bmatrix}
B_0^\ast\\
B_1^\ast+\Delta^\ast C^\ast D
\end{bmatrix}.
\]

So:

\[
A(s)
=
\left(
B_0^\ast s,\;
B_1^\ast s+\Delta^\ast C^\ast D s
\right),
\]

and:

\[
A^\ast(r_0,r_1)
=
B_0r_0+B_1r_1+D^\ast C\Delta r_1.
\]

This is the central Descender formula.

## 5. Balanced Guard Code

The conservative first design chooses \(B=[B_0\ B_1]\) as a Cauchy MDS disperser.

Choose distinct field elements:

\[
x_1,\ldots,x_n,\alpha_1,\ldots,\alpha_m\in\mathbb F_q,
\qquad
q>n+m.
\]

Define:

\[
B_{j,i}=\frac{1}{x_j-\alpha_i}.
\]

This gives strong deterministic transversality. If \(r\in\mathbb F_q^m\) has Hamming weight \(t\le n\), then:

\[
\operatorname{wt}(Br)\ge n-t+1.
\]

If \(B_0\) has \(m_0\) columns and \(m_0\ge n\), then the guard block gives:

\[
\operatorname{wt}(B_0^\ast s)\ge m_0-n+1
\]

for every nonzero \(s\in V\).

The condition \(m_0\ge n\) is not cosmetic.  It is the dimension-budget
condition that prevents the guard block from being a tall \(n\times m_0\)
matrix with a nontrivial left kernel.  The Lean development contains a
\(2\times 1\) Cauchy counterexample showing that the stronger statement without
this hypothesis is false: a nonzero row combination can annihilate all guard
columns.  Therefore any cryptographic parameter set that wants the guard block
alone to certify primal distance must allocate at least as many guard
coordinates as checked row coordinates.

The guard block protects primal distance even if the tilted block has accidental cancellations.

## 5.1 Cryptosystem Architecture After the Guard-Budget Fix

The corrected architecture is a layered public linear code with a dimension
budget visible at setup time.

1. **Descent layer.** Choose the public sheaf complex and quotient:
   \[
   V=C^0(X,\mathcal F)/H^0,\qquad D=d^0:V\to W.
   \]
   The setup must make \(D\) injective on the quotient and provide sparse
   multiplication routines for \(D\) and \(D^\ast\).

2. **Guard layer.** Choose a Cauchy or balanced MDS block
   \[
   B_0:M_0^\ast\to V
   \]
   with
   \[
   m_0=\dim M_0\ge n=\dim V.
   \]
   This layer is deliberately redundant.  Its job is not compression; its job is
   to give a formally certified primal distance
   \(d_{\mathrm{prim}}\ge m_0-n+1\).

3. **Tilt/sampler layer.** Choose the HDX-local wedge sampler
   \(C:M_1^\ast\to W\) and public diagonal \(\Delta\).  This layer supplies
   locality, mixing, and anti-cancellation evidence.  It may be thinner than the
   guard layer, but it must not be asked to replace the guard-budget condition.

4. **Public projection.** Publish multiplication by
   \[
   A(s)=\left(B_0^\ast s,\;B_1^\ast s+\Delta^\ast C^\ast D s\right).
   \]
   Encapsulation samples a secret \(s\) and centered small noise \(e\), then
   releases \(t=As+e\).  Decapsulation/validation works by bounded nearest
   projected-coboundary decoding against \(A(V)\).

5. **Security proof split.** The deterministic Lean core proves the guard and
   low-weight dual transversality statements.  The current cryptographic Lean
   layer now packages the remaining security claim as an explicit certificate:
   an FO/CCA-to-CPA reduction, a CPA-to-average-P-TND reduction, an
   average-to-worst-case projected-coboundary reduction, and a worst-case
   hardness assumption.  The CPA-to-average-P-TND step is now formalized for the
   direct Descender CPA sample game as the identity reduction on decision
   distinguishers.  The remaining proof fields are now named concrete maps:
   `FOImplicitRejectionProof` must simulate CCA adversaries as CPA/P-TND
   distinguishers, and `AverageToWorstCaseSearchReduction` must turn such
   distinguishers into solvers for a complete worst-case
   `ProjectedCoboundaryFamily`.  The worst-case hardness predicate is over a
   declared algorithm class `WorstCaseProjectedCoboundaryProblem.SolverAlg`, not
   over all mathematical choice functions.  The average-to-worst step now has a
   concrete self-randomization proof object: prove `randomize_real` and
   `recover_sound` for the chosen sampler, and Lean constructs the reduction.
   These ingredients are bundled by `DescenderKEMCertificate`, whose theorem
   `ind_cca_secure_for_declared_class` is the current top-level no-break
   statement for the declared nonuniform-QPT adversary class.

This means the first proof-first parameter regime should accept extra guard
redundancy.  Later optimization can replace the Cauchy guard by a different
balanced code or add another independently certified primal-distance mechanism,
but it cannot simply set \(m_0<n\) while keeping the same theorem.

## 6. HDX Wedge Sampler

The local tilted component \(C\) is an HDX wedge sampler.

Use a typed \(d\)-dimensional simplicial complex \(X\) with type map:

\[
\tau:X(0)\to\{0,\ldots,d\}.
\]

For an oriented type triple \(\theta=(a;b,c)\), every typed 2-face

\[
\{v_a,v_b,v_c\}
\]

gives one wedge:

\[
w=(v_a;v_b,v_c).
\]

The wedge patch consists of two edges:

\[
P_w=\{(v_a,v_b),(v_a,v_c)\}.
\]

Use one anchor per typed triangle to avoid immediate local dependencies. If more type balance is needed, use a carefully chosen set of oriented type triples \(\Theta\).

With constant fiber \(\mathbb F_q^k\), choose public vectors:

\[
x_w,y_w\in\mathbb F_q^k.
\]

Define \(c_w\in C^1(X,\mathcal F)\) by:

\[
c_w(a,b)=x_w,
\qquad
c_w(a,c)=y_w,
\]

and zero elsewhere.

Then:

\[
h_w=D^\ast c_w
\]

has vertex-block values:

\[
h_w(a)=-(x_w+y_w),
\qquad
h_w(b)=x_w,
\qquad
h_w(c)=y_w.
\]

Thus each column has bounded locality:

\[
|\operatorname{supp}_{\mathrm{blocks}}(c_w)|\le 2,
\]

\[
|\operatorname{supp}_{\mathrm{blocks}}(D^\ast c_w)|\le 3.
\]

## 7. Public Thinning

Do not use all wedges. Select a linear-size public subset.

Let:

\[
N=|X(0)|,
\qquad
m_1=\rho N.
\]

For every wedge \(w\) in the typed universe \(\Omega\), include \(w\) if:

\[
\operatorname{Hash}(\text{seed},w)<p,
\qquad
p\approx \frac{\rho N}{|\Omega|}.
\]

Equivalently, sort accepted wedges by hash and take the first \(m_1\). The result is a transparent public sampler:

\[
\mathcal W\subseteq\Omega.
\]

The theorem target is that, with overwhelming probability over the public seed, \(\mathcal W\) satisfies the wedge-sampler axioms below.

## 8. Local Fiber Condition

For every vertex \(v\), let:

\[
\mathcal W(v)=\{w\in\mathcal W:v\in w\}.
\]

The endpoint vectors:

\[
g_{v,w}=h_w(v)\in\mathbb F_q^k
\]

must be locally \(k\)-wise independent:

\[
\text{every subset of at most } k \text{ incident vectors at } v
\text{ is linearly independent.}
\]

This can be enforced publicly. For example, use local Vandermonde or MDS vectors, then verify the condition at every vertex. If verification fails, increment a public nonce and regenerate coefficients. This is part of setup, not a secret diagnostic.

## 9. Patch-Rank Expansion by Peeling

Let \(\mathcal H=(X(0),\mathcal W)\) be the selected wedge hypergraph, where every wedge touches three vertices.

For \(S\subseteq\mathcal W\), define:

\[
\deg_S(v)=|\{w\in S:v\in w\}|.
\]

The hypergraph is \((\tau,k)\)-peelable if every nonempty

\[
S\subseteq\mathcal W,
\qquad
|S|\le \tau N
\]

contains a vertex \(v\) with:

\[
1\le \deg_S(v)\le k.
\]

Equivalently, \(\mathcal H\) has no small \((k+1)\)-core.

This property turns local fiber independence into rank expansion:

\[
(\tau,k)\text{-peelability}
+
\text{local }k\text{-wise independence}
\Rightarrow
\operatorname{rank}(D^\ast C)_S=|S|
\]

for every \(S\subseteq\mathcal W\) with \(|S|\le\tau N\).

The proof is by peeling. Any linear dependence among columns \(h_w\) restricts at a low-degree vertex to a dependence among at most \(k\) locally independent vectors, forcing those coefficients to vanish. Iterating removes all coefficients.

A sufficient expansion condition is:

\[
|\Gamma(S)|>\frac{3}{k+1}|S|
\]

for every small \(S\), where \(\Gamma(S)\) is the set of vertices touched by \(S\).

Fiber dimension \(k\) makes this condition substantially easier. For \(k=4\), the required ratio is \(0.6|S|\); for \(k=8\), it is approximately \(0.33|S|\).

## 10. Sampling Regularity

The HDX supplies distributional regularity; the fibers and peeling supply rank.

The typed wedge universe should satisfy double-sampler behavior between:

\[
X(2),\quad X(1),\quad X(0),
\]

and between typed wedges, edges, and vertices.

For large vertex and edge sets, selected wedges should hit them at approximately the expected rate. After public thinning, the same should hold with a controlled concentration loss.

The intended proof route is:

1. local spectral HDX implies typed-face incidence regularity;
2. public thinning preserves large-set regularity by concentration;
3. bounded overlap plus thinning excludes small \((k+1)\)-cores;
4. local MDS fibers turn peelability into rank.

## 11. Deterministic Transversality Theorem

For the guarded tilted map:

\[
A=
\begin{bmatrix}
B_0^\ast\\
B_1^\ast+\Delta C^\ast D
\end{bmatrix},
\]

assume \(B\) is Cauchy and every column of \(H=D^\ast C\) has support at most \(L\).

If \(m_0\ge n\), then:

\[
\operatorname{rank}(A)=n.
\]

The primal code \(A(V)\) has minimum Hamming distance:

\[
d_{\mathrm{prim}}\ge m_0-n+1.
\]

This primal-distance claim relies on the guard-budget condition \(m_0\ge n\).
Without it, the guard block can have nonzero left-kernel vectors, and the
claimed positive lower bound is false.  The Lean theorem
`cauchy_guard_primal_distance_strong` therefore takes the corresponding
dimension hypothesis explicitly.

The dual code \(\ker A^\ast\) has no nonzero vector of Hamming weight:

\[
t<\frac{n+1}{L+1}.
\]

For wedge columns, \(L=3\), hence:

\[
d_{\mathrm{dual}}>\frac{n+1}{4}.
\]

This theorem is the first concrete payoff of the design. It is not heuristic: the guard block supplies primal distance and the Cauchy support bound prevents low-weight dual cancellation against the bounded-locality wedge term.

## 12. Random Diagonal and Medium-Complexity Cancellations

The deterministic theorem handles very low-complexity relations. The random diagonal \(\Delta\) is for stronger anti-cancellation statements.

Let:

\[
H=D^\ast C.
\]

Then:

\[
A^\ast(r_0,r_1)
=
B_0r_0+B_1r_1+H\Delta r_1.
\]

For fixed support \(S=\operatorname{supp}(r_1)\):

\[
H\Delta r_1
=
\sum_{i\in S}\theta_i r_{1,i}h_i.
\]

If:

\[
\operatorname{rank}\{h_i:i\in S\}=|S|,
\]

then the tilted term has full local rank over that patch set. Public random diagonal scalars make accidental cancellation against \(B_0r_0+B_1r_1\) unlikely, with probability roughly \(q^{-|S|}\) for fixed targets.

The next theorem must turn this into a union bound over all relevant medium-complexity vectors.

## 13. Candidate Parameters

Let:

\[
N=|X(0)|,
\qquad
k=4 \text{ or } 8,
\qquad
n=k(N-\dim H^0).
\]

Choose:

\[
m_1=\rho N,
\qquad
0.25\le \rho\le 1.
\]

The selected wedge average vertex degree is about \(3\rho\), independent of \(k\).

For the guard block choose:

\[
m_0=n+\mu,
\qquad
\mu\approx 0.1n\text{ to }0.25n.
\]

Then:

\[
m=m_0+m_1.
\]

For \(k=4,\rho=0.5,\mu=0.2n\):

\[
m\approx 5.3N.
\]

This is a proof-first regime. Parameters should be optimized only after the theorem chain is stable.

## 14. KEM Skeleton

The KEM uses the usual Regev/Kyber-style cancellation pattern with the Descender public map \(A\).

### Setup

Public setup generates:

\[
q,X,\mathcal F,D,B_0,B_1,C,\Delta.
\]

It exposes multiplication oracles:

```text
A_mul(s) =
    (B0^T s, B1^T s + Delta C^T D s)

AT_mul(r0,r1) =
    B0 r0 + B1 r1 + D^T C Delta r1
```

No dense matrix \(A\) needs to be stored.

### KeyGen

Sample small:

\[
s\leftarrow\chi_s^V,
\qquad
e\leftarrow\chi_e^M.
\]

Compute:

\[
t=A s+e.
\]

Return:

\[
pk=t,
\qquad
sk=s.
\]

### Encaps

Sample or derive:

\[
r=(r_0,r_1),
\qquad
e_1,
\qquad
e_2,
\qquad
\mu.
\]

Compute:

\[
u=A^\ast r+e_1,
\]

\[
v=\langle r,pk\rangle+e_2+\operatorname{Encode}(\mu).
\]

Return:

\[
ct=(u,v),
\qquad
ss=\operatorname{KDF}(\mu,pk,ct).
\]

### Decaps

Compute:

\[
w=v-\langle u,s\rangle.
\]

Since:

\[
pk=As+e,
\qquad
u=A^\ast r+e_1,
\]

we get:

\[
w
=
\operatorname{Encode}(\mu)
+
e_2+\langle r,e\rangle-\langle e_1,s\rangle.
\]

Decode \(\mu'\), rederive the coins, recompute the ciphertext, and use implicit rejection for CCA security.

## 15. Main Theorem Chain

The research program should stabilize the following theorem chain.

**Theorem 1: Wedge Sampler Theorem.** For the chosen typed local-spectral/cosystolic HDX and public thinning parameters, the selected wedge sampler is admissible with overwhelming probability.

This requires four lemmas:

1. typed-face HDX regularity;
2. thinning preserves large-set regularity;
3. thinning excludes small \((k+1)\)-cores;
4. local MDS fibers plus peelability imply patch-rank expansion.

**Theorem 2: Guarded Tilted Transversality.** The guarded Cauchy-MDS exact-gradient projection with HDX wedge tilt has full rank, protected primal distance, no low-weight dual relations, and medium-complexity anti-cancellation from patch-rank expansion plus \(\Delta\).

**Theorem 3: TND-Admissibility.** Arithmetic HDX towers with suitable coefficient sheaves and the Descender projection code satisfy the TND-admissibility axioms.

**Theorem 4: Average-to-Worst-Case Reduction.** Any nonuniform QPT adversary solving average-case P-TND over the public family yields a solver for worst-case nearest projected coboundary in the same family.

**Theorem 5: KEM Security.** Assuming primal and dual P-TND hardness, the CPA core is secure; the FO-style implicit rejection transform gives IND-CCA security in the quantum random oracle model.  In Lean this is represented by `DescenderKEMCertificate`: it bundles transparent setup, compact evaluators, nonuniform-QPT class metadata, q-ary/noise promises, FO soundness, self-randomization, and worst-case hardness.  Its theorem `DescenderKEMCertificate.ind_cca_secure_for_declared_class` proves that no declared-class CCA adversary breaks the instantiated family.

## 16. Concrete Parameter Search Phase

The abstract KEM certificate is now wired together, so the next research phase is
to instantiate it with concrete public families and parameters.  This should be
done by constraint-driven search, not by hand-picking a parameter set.

The first harness is:

```text
tools/parameter-search/Cargo.toml
tools/parameter-search/src/main.rs
scripts/parameter_search/search.py
scripts/parameter_search/default_profiles.json
scripts/parameter_search/README.md
research/parameter_search/descender-128.{json,md}
research/parameter_search/descender-192.{json,md}
research/parameter_search/descender-256.{json,md}
```

The Rust engine under `tools/parameter-search` is the primary path for larger
enumerations and future exact finite-field diagnostics.  The Python script is a
readable reference implementation and remains useful for quick comparisons and
reporting glue.  The Rust engine supports exhaustive grid mode and simulated
annealing mode.  Candidate rows are now ranked by an `objective_score` that caps
excess proxy-security credit and penalizes public size, ciphertext size,
secret-key size, large modulus arithmetic, tiny noise, and excessive guard
redundancy.

The harness currently enumerates coarse candidates over:

```text
q, n, m0, m1, fiber dimension k, centered-noise radius eta
```

and applies hard proof-side filters:

1. `q > n + m0 + m1`, so the Cauchy guard/tilt description has enough field
   elements;
2. `m0 >= n`, the dimensional hypothesis required by the proven guard primal
   distance theorem;
3. centered-noise headroom `eta < q/2`;
4. guard distance `m0 - n + 1` exceeds the correctness radius;
5. profile-specific size and reduction-slack budgets.

The reported `primal_proxy_bits`, `dual_proxy_bits`,
`self_randomization_proxy_bits`, `security_proxy_bits`, and `objective_score`
are screening quantities only.  They are not cryptanalytic security claims.
Their purpose is to identify a Pareto frontier of candidates worth subjecting to
exact finite-field rank checks, projection-distance searches, dual-witness
searches, and eventually a real lattice/coding estimator.

The initial profiles are:

```text
descender-128
descender-192
descender-256
```

The initial coarse run found nonempty feasible regions for all three profiles
under the current filters.  This is encouraging only as a search signal: it
means the guard-budget theorem and compact-public-description budget do not
immediately make the design impossible.  It does not yet establish concrete
security.

The next parameter-research tasks are:

1. replace the screening proxies with sharper attack-cost estimators;
2. add exact finite-field rank and projected-distance checks to the Rust engine;
3. run scaled dual-witness searches preserving the candidate ratios, also in
   Rust or another exact-arithmetic backend;
4. instantiate the transparent setup and compact evaluator certificate for one
   conservative candidate;
5. feed the surviving candidate back into Lean as the first concrete
   `DescenderKEMCertificate` instantiation target.

### 16.1 Key-Size Levers

The main public-key driver in the first explicit model was the per-wedge fiber
coefficient term:

```text
m1 * (2 * fiber_dim * elem_bytes)
```

This term came from storing both local fiber vectors for every sampled wedge.
It is not an inherent mathematical requirement if those coefficients are
generated by a transparent public rule and verified during setup.  The Rust
search engine now exposes this distinction:

```text
--coefficient-model explicit
--coefficient-model seeded
```

The seeded model stores a coefficient-generation seed plus a small rule
descriptor and fiber rule data, rather than per-wedge coefficients.  This keeps
the setup transparent: the verifier recomputes the same coefficients and checks
the required local independence conditions.

A controlled same-candidate comparison shows this is the largest immediate
public-key lever:

```text
descender-128 explicit top: 19,968 B -> seeded same candidate: 1,824 B
descender-192 explicit top: 34,784 B -> seeded same candidate: 2,368 B
descender-256 explicit top: 128,992 B -> seeded same candidate: 6,464 B
```

After rerunning annealing with the seeded coefficient model and a compact
objective, the current best screening rows are:

```text
descender-128: q=32771, n=80,  m0=104, m1=80,  k=24, eta=4
  pk=1456 B, ct=400 B

descender-192: q=32771, n=112, m0=144, m1=112, k=32, eta=4
  pk=1952 B, ct=544 B

descender-256: q=32771, n=176, m0=224, m1=176, k=48, eta=4
  pk=2944 B, ct=832 B
```

These are still screening-model rows, not security claims.  But they change the
engineering picture: the public-key issue is no longer obviously fatal if
coefficient generation can be made transparent and verifiable.  The next
dominant representation costs are endpoint/incidence metadata, diagonal data,
and the guard/ciphertext dimensions.  Therefore the next low-hanging key-size
lever is to apply the same seed-and-verify design to wedge incidence and public
thinning, while keeping the deterministic verifier inside the eventual
`TransparentSetupCertificate`.

The second lever is now modeled by:

```text
--incidence-model seeded
```

In this model the public key stores a thinning seed, nonce/counter data, a
deterministic endpoint-selection rule descriptor, and a diagonal-generation
seed, rather than storing selected wedge endpoints and diagonal entries per
wedge.  The verifier reconstructs the wedge set and diagonal, then checks the
same public sampler, peelability, local independence, and transversality
conditions required by the proof.

With both low-hanging representation levers enabled:

```text
--coefficient-model seeded
--incidence-model seeded
```

the current compact annealing screen reports:

```text
descender-128: q=32771, n=96,  m0=120, m1=96,  k=24, eta=4
  pk=1104 B, ct=464 B

descender-192: q=32771, n=144, m0=176, m1=144, k=32, eta=4
  pk=1440 B, ct=672 B

descender-256: q=32771, n=208, m0=256, m1=208, k=48, eta=4
  pk=1920 B, ct=960 B
```

These sizes are now in the same rough bandwidth regime as ML-KEM in the
screening model.  The price is a sharper setup/certificate obligation:
transparent regeneration must be proved equivalent to the explicit objects used
by the algebraic proof, and the verifier must reject seeds that fail local
fiber-independence, sampler regularity, or small-core checks.

The proof layer now has an abstract Lean boundary for this obligation in
`Descender.SeededSetup`.  The key theorem is:

```lean
SeededPublicKEMLayer.supplies_public_proof_inputs
```

It says that a seeded public layer supplies transparent setup, compact public
evaluators within budget, and valid explicit regenerated setup data.  Concrete
parameter work still has to instantiate the abstract `ValidExplicitData`
predicate with actual local-independence, sampler, transversality, and
small-core certificates.

Deferred proof note: the seeded representation is therefore a formal theorem
boundary, not a completed concrete setup proof.  We should come back to it when
selecting a final candidate and instantiate `ValidExplicitData` for that
candidate's coefficient generator, thinning/incidence generator, and diagonal
generator.

### 16.2 Next Optimization Lever: Measurement Dimension

After seeded coefficients and seeded incidence, the next dominant bandwidth
term is no longer stored setup metadata.  It is the size of the transmitted
measurement/ciphertext, which scales with the measurement dimension and modulus
encoding.  In the compact seeded screen the leading ciphertext sizes are:

```text
descender-128: ct=464 B at n=96,  m0=120, m1=96
descender-192: ct=672 B at n=144, m0=176, m1=144
descender-256: ct=960 B at n=208, m0=256, m1=208
```

The next optimization pass should therefore target dimension compression and
slack removal:

1. minimize `n`, `m0`, and `m1` subject to the hard proof side conditions;
2. reduce guard slack rather than treating extra guard rows as free security;
3. explore whether the dimensional hypothesis can be satisfied with smaller
   `m0 - n` margins;
4. test smaller moduli only when correctness and dual-witness margins remain
   credible;
5. replace screening proxy bits with exact finite-field rank, projection
   distance, and dual-witness checks on the Pareto frontier.

This is a different lever from seeded setup.  Seeded setup makes the public
description compact; measurement-dimension optimization makes the actual
ciphertext and runtime smaller.

The Rust search engine now has a dedicated mode for this:

```text
--objective bandwidth
```

This mode keeps the current proof-side conditions as hard filters, caps extra
security-proxy credit, and ranks first by ciphertext pressure.  The first broad
seeded bandwidth pass is saved under:

```text
research/parameter_search/bandwidth_seeded/
```

Its top screening rows are:

```text
descender-128: q=32771, n=32, m0=48, m1=32, k=24, eta=4
  pk=704 B, ct=192 B

descender-192: q=32771, n=48, m0=72, m1=48, k=32, eta=3
  pk=848 B, ct=272 B

descender-256: q=32771, n=64, m0=96, m1=64, k=48, eta=3
  pk=1024 B, ct=352 B
```

These rows are deliberately aggressive screening candidates, not final
parameters.  Their small dimensions make the next validation step urgent:
replace the proxy terms by exact rank, projected-distance, dual-witness, and
estimator checks before weakening or changing any theorem hypothesis.

### 16.3 Canonical Public-Key Accounting

The next structural public-key optimization is now modeled by:

```text
--public-key-model canonical
```

The canonical model makes three accounting changes:

1. the Cauchy matrix is generated canonically from the parameter set and master
   seed, so no Cauchy row/column description is serialized per public key;
2. coefficient, incidence, and diagonal seeds are derived from one master seed
   by domain separation;
3. coefficient/incidence/diagonal rule descriptors are fixed by the parameter
   set and live in the specification, not in each public key.

Unlike an unrealistically tiny "seed-only" public key, this model still charges
for the noisy public sample:

```text
t = A s + e.
```

Running the bandwidth search with canonical public-key accounting produced:

```text
research/parameter_search/bandwidth_canonical/

descender-128: q=32771, n=32, m0=48, m1=32, k=24, eta=4
  seed=64 B, sample=160 B, pk=224 B, ct=192 B

descender-192: q=32771, n=48, m0=72, m1=48, k=32, eta=3
  seed=64 B, sample=240 B, pk=304 B, ct=272 B

descender-256: q=32771, n=64, m0=96, m1=64, k=48, eta=3
  seed=64 B, sample=320 B, pk=384 B, ct=352 B
```

Expanded seeded validation for these canonical rows is saved under:

```text
research/parameter_search/bandwidth_canonical_expanded_validation/
research/parameter_search/bandwidth_canonical_expanded_validation_w3_top1/
```

All checked rows passed the generated sparse-tilt rank/support checks through
bounded dual weight `2`, and the top row for each profile passed through
bounded dual weight `3`.

The first expanded seeded-frontier validator is now implemented as:

```text
tools/parameter-search/target/release/validate_frontier
```

Running it on the bandwidth frontier with bounded dual search through weight
`2` produced:

```text
research/parameter_search/bandwidth_seeded_expanded_validation/validation.json
research/parameter_search/bandwidth_seeded_expanded_validation/validation.md
```

It derives deterministic public coefficient, incidence, and diagonal seeds for
each candidate, expands a concrete sparse tilt matrix with exactly three
nonzero row entries per tilt column, combines it with Cauchy guard/base blocks,
and checks exact finite-field rank, guard-block rank, column support, guard
distance, and bounded dual witnesses.  All checked rows passed:

```text
descender-128: min passing pk=672 B,  min passing ct=192 B
descender-192: min passing pk=816 B,  min passing ct=272 B
descender-256: min passing pk=960 B,  min passing ct=352 B
```

A focused top-row run with bounded dual search through weight `3` also passed:

```text
research/parameter_search/bandwidth_seeded_expanded_validation_w3_top1/

descender-128: pk=704 B,  ct=192 B, no dual witness through weight 3
descender-192: pk=848 B,  ct=272 B, no dual witness through weight 3
descender-256: pk=1024 B, ct=352 B, no dual witness through weight 3
```

This is still not a final cryptographic validation.  It says the tiny rows are
not immediately killed by a concrete deterministic sparse-tilt generator.  The
next blocker is deeper dual-witness search and a real attack-estimator pass.

## 17. Concrete Work Plan

### Phase 1: Formalize the algebra

Write precise definitions for:

```text
TND-admissible sheaf complex
projected transparent noisy descent
nearest projected coboundary
guarded tilted exact-gradient projection
wedge-sampler admissibility
```

Prove the deterministic guarded transversality theorem first. This theorem is the cleanest immediate result and does not require the full HDX machinery.

### Phase 2: Prove the wedge rank lemma

Prove formally:

\[
(\tau,k)\text{-peelability}
+
\text{local }k\text{-wise independence}
\Rightarrow
\operatorname{rank}(D^\ast C)_S=|S|.
\]

This should be the first wedge-sampler lemma written in full.

### Phase 3: Prove probabilistic thinning lemmas

Show that public thinning of the bounded-overlap typed wedge universe:

1. preserves endpoint and patch regularity for large sets;
2. excludes small \((k+1)\)-cores with high probability.

The small-core exclusion lemma is the central new combinatorial lemma.

### Phase 4: Select the first HDX family

Candidate families include arithmetic Ramanujan complex towers and explicit Chevalley/coset HDXs. The choice should be proof-driven:

```text
bounded degree
typed face regularity
local spectral expansion
cosystolic expansion
enough symmetry for self-randomization
efficient public enumeration
```

If a simpler synthetic HDX model is enough for the first theorem, use it before specializing to arithmetic constructions.

### Phase 5: Extend from Hamming to q-ary amplitude security

The deterministic theorem gives Hamming transversality. Cryptographic distinguishers may use short integer vectors \(z\in\mathbb Z^m\) with:

\[
A^\ast z\equiv 0\pmod q.
\]

For centered coordinate noise of radius \(\eta\), the salient public-sample
condition is not Hamming weight.  If:

\[
2\eta \|z\|_1+1 < q,
\]

then the statistic \(\langle z,t\rangle\) directly distinguishes
\(t=As+e\) from uniform: real samples always land in the centered interval of
radius \(\eta\|z\|_1\), while uniform samples land there with probability
\((2\eta\|z\|_1+1)/q\).

The Lean target is therefore `NoNarrowQaryDualRelations`: no nontrivial public
q-ary dual relation may have a narrow centered-noise inner-product interval.
Low-Hamming transversality remains a useful deterministic helper, but it is not
the KEM-facing security condition unless it implies this amplitude bound.
The KEM and average-to-worst-case certificates now require the amplitude
parameter to be no larger than the actual noise radius, so the no-narrow
condition is certified for the real deployed noise, not for an unrelated proof
knob.

For parameter search, the least constraining deterministic certificate is an
exact lower bound on the centered/Lee \(L_1\) norm of every nontrivial q-ary
dual relation.  Hamming support distance only gives the fallback bound
\(\|z\|_1\ge n+1\), which is proof-valid but usually too weak for compact
public keys.  The search/validation layer should therefore treat the amplitude
requirement as a separate certificate:

\[
\min_{z\in\Lambda_q^\perp(A),\ z\not\equiv 0\bmod q}\|z\|_{1,\mathrm{centered}}
\;\ge\;
\left\lceil \frac{q-1}{2\eta_{\mathrm{eff}}}\right\rceil.
\]

The proof layer now also formalizes the direct real-sample half of the attack:
if a dual relation annihilates the public evaluation map, then
\(\langle z, As+e\rangle=\langle z,e\rangle\).  Thus any bounded pairing
condition forces real samples into the interval tested by the distinguisher.
The next theorem obligation is not another Hamming search; it is proving that
the concrete public family has no such narrow q-ary dual relations at the
selected effective noise radius, or redesigning the sample game so this
statistic is unavailable.

The parameter search now treats this as a strategy axis rather than a single
fixed theorem knob.  `fallback-support` keeps the conservative \(n+1\) Lee
bound; `solved-repetition` prices a construction where each free coordinate is
read directly by repeated pivot rows; and `direct-solved-lee` models the
preferred proof-carrying route where the solved systematic map
\(y\mapsto(-P y,y)\) has a large centered Lee distance.  The search records the
certificate rows and estimated key-generation multiply-adds for these routes,
so a smaller key is not silently bought by an impractical setup computation.
The deployable target is a generated sparse/arithmetic solved map with a compact
deterministic Lee-expansion certificate.

The first shared-row construction-side certificate is now a two-free-coordinate
block checker.  Unlike scalar repetition, each certified pivot row reads both
coordinates in the pair.  For the current \(q=32771, m_1=32\) 128-bit amplitude
target, the scalar-block family needed \(n=128\) pivot rows; the shared-pair
certificate passed at \(n=80\), with four rows per pair missing by only one
Lee unit.  This makes shared-row arithmetic expanders the next concrete
optimization direction.

The principled design model is a Lee-metric random linear code bound.  For a
fixed nonzero free vector, one random shared row has a uniform dot product over
\(\mathbb F_q\).  The row-budget tool therefore computes exact Lee-ball
volumes and exact lower-tail probabilities for sums of random centered residues,
then applies a union bound over all below-target free vectors and all groups.
At the same \(q=32771, m_1=32\) target, this suggests that larger shared groups
could reach the amplitude condition with far fewer total pivot rows:
approximately 104 rows for groups of 8, 66 rows for groups of 16, and 46 rows
for one group of 32 at an 80-bit expected-bad budget.  The remaining issue is
not whether such blocks should exist, but how to make their certification
transparent: exact deterministic sidecars are easy for pairs and hard for large
groups, so the next proof route must either support rejection-sampled random
blocks or add a scalable deterministic verifier.

The explicit-algebraic path should be framed as systematic Lee-code
construction: choose a block map \(P : \mathbb F_q^g \to \mathbb F_q^r\) so
the graph code \(y \mapsto (P y,y)\) has large minimum Lee distance.  Basic
Lee-BCH designed-distance bounds are far too weak for the compact targets, but
Vandermonde/moment graph codes remain a plausible explicit route if one can
prove polynomial value anti-concentration in the Lee metric.  Integer `l1`
RIP-1 expanders are the sparse fallback, provided modular wrap can be ruled out.

The first concrete algebraic probe makes this sharper.  A plain degree-zero
Vandermonde column, or a shifted moment matrix with midpoint/geometric column
scales, has small one-coordinate Lee relations and should not be pursued.  The
viable explicit candidate is instead a shifted, randomly scaled moment graph
code:

\[
P_{t,j}=c_j\alpha_t^{1+j}.
\]

For the 128-bit `q=32771, L=3277, g=32` target, the seed-1 probe finds
one-coordinate counterexamples at `r=40,41`, but no below-target vector at
`r=42` or `r=46` under exact unit search, bounded pair search, sparse random
search, and annealing.  This is still experimental evidence, not a security
claim; it identifies the right certificate theorem to try.

The proof interface for this route is `Descender.MomentLee`.  Parameter search
can now label this route as `moment-lee`, while Lean reduces the deployable
claim to `MomentLeeAntiConcentration` for the concrete shifted/scaled moment
matrix.  This keeps the construction explicit and public, without treating the
probe output as a proof.

### Phase 6: Average-to-worst-case reduction

Develop the analog of the LWE proof architecture for projected descent complexes:

\[
\text{average-case P-TND}
\Rightarrow
\text{worst-case nearest projected coboundary}.
\]

This is the central cryptographic theorem. If it fails, Descender may still be an interesting code-based/noisy-descent scheme, but it will not meet the original no-backdoor standard.  In the Lean architecture, this theorem is exactly the `AveragePTNDToWorstCaseProof` field of the KEM security certificate.

### Phase 7: Prototype after theorem targets stabilize

Only after the first formal theorem statements are stable, implement:

```text
finite field arithmetic
Cauchy B and fast transforms
HDX or toy-complex wedge sampler
D_mul / DT_mul
C_mul / CT_mul
A_mul / AT_mul
KeyGen / Encaps / Decaps
correctness tests
rank and low-weight-dual diagnostics
```

Diagnostics are sanity checks, not the source of security.

## 18. Concrete Parameter-Search Status

The current implementation track has moved from proof artifacts into a concrete
parameter-search loop in `tools/parameter-search`.  The Rust search engine now
supports:

```text
canonical public-key accounting
single public master seed
seeded coefficient/incidence/diagonal generation
public sample and ciphertext bit-packing
conservative compression-as-extra-noise correctness filtering
expanded finite-field validation of top rows
bounded dual-witness checks for small weights
```

The best currently validated compressed canonical rows are:

| target | q | n | m0 | m1 | k | eta | encoded sample bits | rounding error | effective eta | public key | ciphertext |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 128 | 32771 | 32 | 52 | 32 | 24 | 2 | 13 | 3 | 5 | 201 B | 169 B |
| 192 | 16411 | 56 | 88 | 56 | 32 | 2 | 11 | 5 | 7 | 262 B | 230 B |
| 256 | 65537 | 64 | 104 | 64 | 48 | 2 | 12 | 9 | 11 | 316 B | 284 B |

These rows passed the expanded validation harness on the searched frontier:
exact finite-field rank, guard-block rank, tilt support, guard distance with
effective noise, and bounded dual search up to weight 2; the first row per
profile also passed the weight-3 dual check.

This does not yet make a security claim.  The remaining work is to replace the
screening proxies with concrete attack estimates and to connect the compressed
sample semantics to the final KEM proof statement.  The current proof-side
interpretation is simple: compression is not placed in the master seed, because
`t` carries the instance-specific noisy syndrome; instead, the serialized `t` is
bit-packed and the lost precision is charged as additional centered noise.

### 18.1 Sample Views and Challenge-Space Compression

A more structural optimization is to publish only the part of the sample that
encapsulation can actually query.  If challenge vectors \(r\) range over all of
the measurement dual \(M^\ast\), the functional

\[
r\mapsto \langle r,t\rangle
\]

is equivalent to the full sample \(t\).  In that case there is no meaningful
lossless compression beyond bit packing and quantization.

If challenges are restricted to a public subspace \(R\subseteq M^\ast\), then
the public key only needs the induced view of \(t\) on \(R\), i.e. \(t\) modulo
\(R^\perp\).  The proof cannot treat this as a serialization detail.  It must
replace the decision/search problem by the restricted operator and prove that
the restricted operator is still sound.

The Lean boundary for this is now `Descender.SampleView`.  A `SampleView` is an
additive public map from full measurement samples to serialized view samples.
It proves:

```lean
SampleView.distinguishes_pullback_of_restrict
SampleView.correctnessCertificate
SampleView.restricted_planted_unique
```

The important obligations for any concrete quotient/challenge-space proposal
are:

```lean
SampleView.NoiseSmall
SampleView.EvalSeparated
```

`NoiseSmall` requires the viewed noise to remain bounded.  `EvalSeparated`
requires the viewed operator to separate distinct secrets by more than twice the
viewed noise radius.  Therefore the next proof-first compression question is
not "can we store fewer coordinates?" but:

```text
Can we choose a lower-dimensional challenge space R such that P_R A remains
TND-admissible and the viewed noise remains inside the correctness radius?
```

This gives a clean proof target for future public-key reduction.  A pure
syndrome map \(H\) with \(H A = 0\) is not a valid replacement for the current
KEM sample, because it erases the secret signal and no longer supports the
Regev/Kyber-style cancellation identity.

The first concrete Lean specialization is coordinate restriction:

```lean
SampleView.coordinateRestriction_correctnessCertificate
```

It proves that publishing selected coordinates is correct if the selected
coordinates satisfy the viewed separation predicate.  The noise half is easy:
coordinate restriction preserves coordinatewise centered-noise bounds.  The
hard half is separation.  The existing Cauchy guard result gives many nonzero
coordinates, but P-TND correctness currently asks for a centered \(L^\infty\)
gap.  A Hamming support lower bound alone does not prove such a gap.  This is
the precise proof bottleneck for structural public-sample compression.

The next bridge is now explicit in Lean:

```lean
SampleView.scaledCoordinateRestriction
SampleView.DiagonalAmplifiesSupport
SampleView.SupportCoveredByView
SampleView.exists_selected_pair_nonzero_of_supportCoveredByView
SampleView.exists_selected_pair_nonzero_of_surjective
SampleView.scaledCoordinateRestriction_correctnessCertificate_of_amplifies
```

This represents a selected coordinate view followed by a public diagonal.  The
new deterministic hypothesis says that support-heavy secret differences become
large in centered \(L^\infty\) after the public diagonal.  In other words, the
Cauchy guard theorem can remain a support-distance theorem, while the missing
probabilistic layer proves that the public diagonal is good with high
probability.

One caveat is now part of the proof interface rather than an implicit promise:
public diagonal scaling also scales noise.  A concrete KEM must sample the
noise in the scaled view, choose a scaling family that preserves the noise
promise, or account for the larger effective radius through
`SampleView.ViewReductionBudget`.  The Lean certificate still requires
`SampleView.NoiseSmall` for the scaled view.

The randomized diagonal step is now reduced to finite counting in
`Descender.DiagonalAmplification`:

```lean
SampleView.FullDiagonalSeed
SampleView.fullDiagonalScale
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

So the next concrete theorem is no longer "prove correctness from randomness"
in one jump.  It is: for the chosen seed space and public diagonal generator,
bound the cardinality of bad seeds.  Once that count is below the seed-space
size, Lean already produces a good diagonal and a correctness certificate.
The actual seed space is now the full public diagonal
`ViewCoord -> ZMod q`.  The proof has a standard union-bound shape: estimate
the bad scales for each ordered pair of distinct secrets, then sum those
pairwise estimates below the full seed-space size.
For one selected coordinate with difference `delta`, Lean has reduced the
pairwise seed count to:

```text
#(coordinateBadScales delta eta) * q^(#ViewCoord - 1)
```

The scalar estimate is now formalized.  If `delta` is a unit, multiplication by
`delta` bijects bad scales with centered residues of absolute value at most
`2*eta`, and that centered interval has size at most `2*(2*eta)+1`.  In a prime
field, nonzero `delta` is a unit.  Therefore the remaining proof obligation for
the full-diagonal counting theorem is not scalar arithmetic; it is the
sample-view separation hypothesis: every support-heavy distinct secret pair
must expose at least one viewed coordinate with a nonzero difference.  Once that
holds, Lean proves a global bad-seed bound by summing the per-pair
`(2*(2*eta)+1) * q^(#ViewCoord-1)` estimates.

For compressed selected-coordinate views, this support-covering condition is now
formalized directly.  `SupportCoveredByView` states that the selected view hits
every nonzero coordinate of a relevant full difference, and
`failureCountAtMost_full_of_supportCovered_select_prime` proves the resulting
finite bad-seed bound.  The theorem
`exists_scaledCoordinateRestriction_correctnessCertificate_full_of_supportCovered_select_prime`
is therefore the current compression-facing correctness theorem: a selected view
that covers all relevant pair supports, together with support distance,
viewed-noise boundedness, and the finite diagonal bad-budget inequality,
produces a concrete good public diagonal seed and a P-TND correctness
certificate.

For the full or uncompressed selected-coordinate view,
`exists_selected_pair_nonzero_of_surjective` proves support coverage from
surjectivity of `select` and `0 < minSupport`.  The theorem
`exists_scaledCoordinateRestriction_correctnessCertificate_full_of_surjective_select_prime`
is the corresponding full-view special case.

The compression question is now sharper.  A smaller public sample view must
now be designed by proving `SupportCoveredByView` for a much smaller selected
coordinate family, or by proving a lossy view theorem with an explicit
`ViewReductionBudget`.

The project should therefore keep a small proof-strategy portfolio instead of
discarding older routes as soon as a smaller public-key route appears.  This is
now reflected in Lean by `Descender.StrategyComparison`:

```lean
SampleView.StrategyKind
SampleView.StrategyCostProfile
SampleView.StrategyCertificate
SampleView.fullSurjectiveStrategyCertificate
SampleView.supportCoveredStrategyCertificate
SampleView.lossyBudgetStrategyCertificate
```

The comparison axes are public key bytes, ciphertext bytes, key-generation and
setup-validation work, encapsulation/decapsulation work, and the additional
proof obligations imposed by the route:

* full/surjective view: largest baseline, lowest proof risk;
* support-covered view: exact structural compression, but requires a concrete
  selected-coordinate hitting argument;
* lossy-budget view: smallest sample encoding candidate, but pays for rounding
  or quantization through a larger effective noise radius.

This makes parameter search a strategy comparison problem.  The Rust search
engine records the selected proof route with `--proof-strategies`; exact routes
reject bit-packed samples, while the lossy-budget route is allowed to trade
sample bytes for correctness slack.
The search also has an explicit viewed-coordinate parameter:

```text
--view-coords 0,48,64,80,96,128,...
```

`0` is the old full publication of all \(m_0+m_1\) coordinates.  Smaller
values price a real public-sample coordinate view.  The full/surjective proof
route rejects such rows; the support-covered and lossy-budget routes keep them
as candidates whose follow-up validation must prove either the
selected-coordinate support-covering condition or the explicit viewed-noise
budget.

`Descender.KEMProfile` now packages the four certificate layers needed before a
row can graduate from search candidate to KEM profile:

```lean
SampleView.SupportCoveredStrategyObligations
SampleView.LossyBudgetStrategyObligations
ExactValidationCertificate
ValidatedStrategyCertificate
CertifiedKEMProfile
```

This is the current project gate.  Parameter search may rank smaller viewed
samples, but exact validation intentionally rejects non-full views with
`view_reduction_certificate_missing` until the corresponding support-covered or
lossy-budget certificate is supplied.

There is one more independent optimization axis: the primal-distance model.
The current proof uses guard-only distance and therefore requires \(m_0 \ge n\).
The search can also run:

```text
--primal-distance-models guard-only,combined-proxy
```

`combined-proxy` lets the viewed combined measurement matrix contribute to the
correctness distance budget.  It is useful for comparing whether lowering
`m0` is worth a new proof, but validation must keep it non-deployable until the
combined primal-distance certificate is proved.
The search has `--m0-values` for this mode, since the old `--m0-overheads`
parameterization can only generate \(m_0 \ge n\).

The combined-distance theorem surface is now present in Lean through
`GuardedCombinedDeterministicExportBundle` and
`descender_deterministic_core_combined`.  This means the architectural proof no
longer forces the primal distance to come from the guard block alone.  The
remaining job is concrete: for a selected parameter row, provide the
matrix-specific combined primal-distance certificate consumed by that bundle.

The proof layer now separates three regimes:

```text
dense small-amplitude noise:
  centered L-infinity correctness via PTNDCorrectnessCertificate

sparse support noise:
  Hamming/support correctness via PTNDSupportCorrectnessCertificate

lossy viewed samples:
  SampleView.ViewReductionBudget plus viewed noise/separation obligations
```

This clarifies how to revisit the Cauchy guard theorem.  The theorem remains
the right deterministic statement for support-distance and sparse-noise
decoding.  It is not, by itself, an amplitude-separation theorem for dense
small noise.  To use coordinate/sample compression in the current dense-noise
KEM, the next mathematical target is an amplitude lemma for the selected view,
possibly with a tunable bad-event probability and effective noise radius.

## 19. Open Risks

**Cauchy structure may be exploitable.** Cauchy \(B\) gives clean theorems, but algebraic structure can create attacks. If needed, replace it with a pseudorandom balanced code, rank condenser, or subspace-design-based construction.

**Hamming distance is not enough.** The scheme needs q-ary amplitude safety:
for every nontrivial public dual relation \(z\), centered noise must wrap enough
modulo \(q\), or the public view must avoid such relations.  The formal target
is `NoNarrowQaryDualRelations`, not merely absence of low-Hamming duals.

**HDX regularity may not imply small-core exclusion at useful density.** The wedge sampler theorem depends on proving that public thinning avoids small \((k+1)\)-cores.

**The average-to-worst-case theorem may fail.** This is the biggest cryptographic risk. The project should be judged by whether this theorem can be made real.

**Coefficient sheaves may need to be nonconstant.** Constant fibers are the right first object, but expanding coefficient sheaves may be required to remove invariant structure.

## 20. Compact Statement

Descender is the project of constructing a post-quantum KEM from transparent noisy descent:

\[
t=As+e,
\]

where:

\[
A=
\begin{bmatrix}
B_0^\ast\\
B_1^\ast+\Delta C^\ast D
\end{bmatrix}.
\]

Here:

```text
D      = sheaf coboundary on a typed local-spectral/cosystolic HDX
B      = Cauchy or balanced exact-gradient guard code
C      = fiber-boosted HDX wedge sampler
Delta  = public random diagonal tilt
```

The wedge sampler is:

```text
one oriented wedge per selected typed 2-face
public Bernoulli/hash thinning to m1 = rho |X(0)|
constant F_q^k fibers
c_w(a,b) = x_w
c_w(a,c) = y_w
h_w(a) = -(x_w + y_w)
h_w(b) = x_w
h_w(c) = y_w
local k-wise endpoint independence
no small (k+1)-core in the selected wedge hypergraph
```

The immediate mathematical target is:

\[
\textbf{guarded Cauchy-MDS exact-gradient projection}
+
\textbf{HDX wedge-rank tilt}
\Rightarrow
\textbf{TND-admissible public family}.
\]

The final cryptographic target is:

\[
\textbf{breaking Descender}
\Rightarrow
\textbf{solving worst-case nearest projected coboundary}
\]

for transparent sheaf-complex families, even against nonuniform quantum adversaries.
