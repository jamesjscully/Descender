# ECC vs 1D Maps

Source: https://chatgpt.com/share/6a049699-d57c-83ea-b16e-77b96cee7da3
Extracted: 2026-05-13 15:24:06 UTC
Visible messages: 63

## 1. User

Is elastic curve cryptography basically a 1d maps problem? If not what is different?

## 2. Assistant

I’ll assume you mean **elliptic curve cryptography** rather than “elastic.” I’ll separate the **dynamical-systems intuition** from the actual algebraic structure, because that distinction is the core issue.

## 3. Assistant

Assuming **elliptic curve cryptography**: **not really**. There is a superficial “1D map” view, but it misses the essence.

ECC is based on an **elliptic curve over a finite field**, with a group law on its points. A private key is a scalar \(k\), a public base point is \(G\), and the public key is

\[
Q = kG
\]

where \(kG\) means “add \(G\) to itself \(k\) times.” Standards describe ECC in exactly this language: arithmetic on elliptic-curve groups over finite fields, with security resting on the elliptic-curve discrete logarithm problem. citeturn190002search2turn190002search3

So there **is** a one-dimensional-looking map:

\[
k \mapsto kG
\]

from the cyclic scalar group \(\mathbb{Z}/n\mathbb{Z}\) into a subgroup of curve points. In that limited sense, ECC is a **1D parameterization problem**: the secret is one scalar, and public-key generation maps that scalar into a finite group.

But it is **not** a “1D maps problem” in the dynamical-systems sense, like iterating

\[
x_{t+1} = f(x_t)
\]

on an interval. The important differences are:

**First, the space is not the real line.** The curve is usually something like

\[
y^2 = x^3 + ax + b \pmod p
\]

so the points live over a **finite field**, not \(\mathbb{R}\). Geometry still matters, but it is algebraic geometry over finite fields, not smooth real dynamics.

**Second, the operation is a group law, not generic function iteration.** You can define a translation map

\[
T_G(P) = P + G
\]

and then \(kG = T_G^k(0)\). But \(T_G\) is just a finite cyclic rotation on a group. It is not chaotic, contractive, expanding, mixing, or anything like that. It is structurally closer to **modular exponentiation** than to a logistic map.

**Third, the hard problem is not predicting the orbit.** Forward iteration is easy: compute \(kG\) efficiently by double-and-add. The hard problem is inversion: given \(G\) and \(Q = kG\), recover \(k\). That is the **elliptic-curve discrete logarithm problem**. Menezes’ ECDLP report states the security condition directly: given \(E\), finite field data, \(P\), and \(Q = \ell P\), one must find \(\ell\); ECC security depends on that being infeasible. citeturn190002search12

**Fourth, “elliptic curve” means algebraic dimension one, but not 1D state dynamics.** An elliptic curve is a one-dimensional algebraic variety, yes. But its cryptographic utility comes from the finite abelian group structure on its rational points, especially large prime-order subgroups. The curve is “1D” as an algebraic object, while the cryptographic map is a **group homomorphism** from scalars to points.

The best compression is:

**ECC is not a 1D maps problem. It is a discrete-log problem in a finite cyclic subgroup of the rational points of a genus-one algebraic curve.**

The closest “1D map” analogy is this:

\[
\mathbb{Z}/n\mathbb{Z} \to \langle G\rangle,\qquad k \mapsto kG
\]

That map is easy to evaluate and hard to invert. But calling it “1D maps” suggests the wrong machinery. The right machinery is **finite fields, abelian groups, algebraic curves, scalar multiplication algorithms, and discrete-log hardness**, not phase portraits or stability of iterated real maps.

## 4. User

Why does the structure of the rational points make ecdlp harder than the 1d maps equivalent?

## 5. Assistant

The important correction is: **the cyclic subgroup used by ECC is abstractly one-dimensional**. The hardness does not come from there being “more dimensions”; it comes from the fact that the public representation hides the coordinate along that one-dimensional cycle.

## 6. Assistant

**It does not, in the abstract.** A prime-order ECC subgroup is literally a **cyclic 1D object**:

\[
\langle G\rangle \cong \mathbb Z/n\mathbb Z
\]

So if you had the right coordinate system, every point would just be labeled by its scalar:

\[
0,\;G,\;2G,\;3G,\ldots,(n-1)G
\]

and ECDLP would be trivial. Given \(Q=kG\), just read off the coordinate \(k\).

The entire cryptographic point is that the curve gives you an **efficient forward representation** of this cyclic group without giving you the **linear coordinate along the cycle**.

The public representation of \(Q\) is not \(k\). It is something like

\[
Q = (x_Q,y_Q)\in E(\mathbb F_p)
\]

where

\[
y^2 = x^3 + ax + b \pmod p.
\]

Those coordinates are **ambient algebraic coordinates**, not the **group-coordinate** \(k\). The map

\[
k \mapsto kG = (x(kG),y(kG))
\]

looks pseudorandom with respect to \(k\). Nearby scalars do not produce nearby points in any useful metric, and the \(x\)-coordinate does not monotonically encode progress around the cycle.

So the difference is not “ECC is more than 1D.” It is:

**ECC hides a 1D cyclic coordinate inside a nonlinear finite-field algebraic representation.**

A trivial 1D map would be something like

\[
k \mapsto k \bmod n
\]

or

\[
k \mapsto kG
\]

where points are explicitly stored as their index. Then inversion is immediate.

A more relevant comparison is finite-field discrete log:

\[
k \mapsto g^k \pmod p.
\]

That is also a 1D cyclic orbit. But finite fields have **extra multiplicative/additive structure** that supports **index calculus**. You can exploit factorization:

\[
g^e \equiv \prod_i \ell_i^{a_i} \pmod p
\]

for small primes \(\ell_i\), gather many such relations, and solve a large linear system for logarithms. That gives subexponential algorithms for many finite-field DLPs.

Elliptic curves were attractive because, for well-chosen ordinary curves, the point group does **not** seem to admit an analogous cheap relation-generation mechanism. Points do not factor into small points the way integers or polynomials factor into small primes. The group law is efficiently computable, but it does not expose a rich arithmetic decomposition that turns logs into linear algebra.

So for a good elliptic curve, known attacks are essentially **generic cyclic-group attacks**, mainly:

\[
O(\sqrt n)
\]

methods such as Pollard rho or baby-step/giant-step. That is roughly optimal in the generic group model. This is why a 256-bit elliptic-curve group gives about 128-bit security: the best generic attack costs about \(2^{128}\) group operations.

The subtle point is that the rational points have **just enough structure** to make forward scalar multiplication efficient:

\[
(k,G)\mapsto kG
\]

but, for ordinary secure curves, apparently **not enough extra exploitable structure** to recover \(k\) much faster than square-root search.

Bad curves break this. For example, **anomalous curves**, certain **supersingular curves**, or curves with small embedding degree can admit reductions to easier problems, often finite-field DLP via pairings or \(p\)-adic methods. Those cases show exactly the principle: when the rational-point structure leaks an additional computable representation, ECDLP becomes easier.

So the clean answer is:

**ECDLP is not harder because the group is “less 1D.” Abstractly it is 1D. It is harder because the curve representation withholds the scalar coordinate while also avoiding the factorization/index-calculus structure that makes many other 1D cyclic discrete-log problems easier.**

## 7. User

For any ecc alg, is there one coordinate system that would "unlock" that curve and break the algorithm?

## 8. Assistant

This is a good place to be very literal about “coordinate system.” There is a **tautological coordinate** that would break it, but the claim “find coordinates” can mean several very different things algebraically.

## 9. Assistant

**Yes, tautologically. No, not constructively.**

For any ECC system using a prime-order subgroup \(\langle G\rangle\), there exists a coordinate

\[
\phi_G : \langle G\rangle \to \mathbb Z/n\mathbb Z
\]

such that

\[
\phi_G(kG)=k.
\]

That coordinate would completely “unlock” the curve. It would turn every public key \(Q\) into its private scalar \(k\). But computing that coordinate **is exactly the elliptic-curve discrete logarithm problem**. So the magical coordinate exists for stupid reasons: every finite cyclic group is isomorphic to \(\mathbb Z/n\mathbb Z\). The cryptographic question is whether that isomorphism can be computed efficiently from the public curve representation. For well-chosen curves, no classical method is known. ECDLP is precisely: given \(P\) and \(Q=\ell P\), recover \(\ell\); ECC security requires that to be infeasible. citeturn363595search17

The distinction is:

\[
\textbf{existence of a coordinate} \neq \textbf{efficient computability of the coordinate}.
\]

Every secure ECC group has a hidden linear coordinate. The public coordinates \((x,y)\) are not that coordinate. They are **ambient finite-field coordinates** satisfying the curve equation. A coordinate change like Weierstrass \(\leftrightarrow\) Montgomery \(\leftrightarrow\) Edwards can make arithmetic faster or safer, but it does not reveal \(\phi_G\). These model changes are efficiently invertible descriptions of the same underlying group, not logs.

There **are** special curves where something like your “unlocking coordinate” exists. The classic cases are exactly the ones excluded from normal ECC parameter choices.

For **anomalous curves**, where \(\#E(\mathbb F_p)=p\), Smart showed a linear-time method for solving the discrete log; the paper explicitly says such curves must be eliminated for cryptographic use. citeturn363595search2 That is a real example of extra arithmetic structure creating a hidden coordinate-like attack.

For curves with small **embedding degree**, pairings can reduce ECDLP on the curve to DLP in a finite-field multiplicative group. The MOV/Frey–Rück family of reductions does not literally give the scalar coordinate directly, but it transports the problem into a representation where finite-field index calculus may apply; Galbraith summarizes this as reducing divisor-class-group/curve DLP to DLP in an extension field when the embedding degree is small. citeturn775933search4

For arbitrary cyclic groups with only black-box group operations, Shoup’s generic lower bound says you cannot beat roughly square-root complexity in the subgroup size; that matches Pollard rho/baby-step giant-step scale. This matters because good ECC curves are designed so known attacks are essentially generic. citeturn294753search0

So the clean answer is:

**For every ECC group, there exists a scalar-coordinate system that would break it. But for secure curves, finding that coordinate appears computationally equivalent to solving ECDLP itself.**

A stronger version:

**A curve is broken precisely when its public algebraic representation admits an efficiently computable bridge to a representation where the discrete-log coordinate is easier to recover.**

That bridge might be a pairing, a \(p\)-adic lift, a Weil descent, an exploitable endomorphism structure, or—in a different computational model—Shor’s quantum algorithm. Shor’s algorithm gives polynomial-time algorithms for discrete logarithms, including elliptic-curve discrete logarithms, so in that sense quantum computation supplies a nonclassical way to extract the hidden coordinate. citeturn294753search3

But classically, for standard curves like P-256, Curve25519, secp256k1, etc., no known “coordinate system” unlocks the curve. If one were found, it would not just be an implementation trick. It would be a major cryptanalytic break: ECDH, ECDSA, EdDSA, and related schemes using that subgroup would lose their discrete-log security.

## 10. User

How does this scheme break with quantum?

## 11. Assistant

I’ll frame this as **what the quantum computer does algebraically**, not as “it tries keys faster.” That misconception is the source of most bad explanations of the ECC break.

## 12. Assistant

ECC breaks under quantum because **Shor’s algorithm solves the hidden linear coordinate problem directly**.

Suppose the public key is

\[
Q = kG
\]

where \(G\) is the public base point, \(Q\) is the public key, and \(k\) is the private scalar. Classically, the attacker sees \(G\) and \(Q\), but not the hidden scalar coordinate \(k\). The problem is to recover \(k\).

A quantum attacker defines a function over two scalar variables:

\[
f(a,b)=aG+bQ.
\]

Since \(Q=kG\), this is really

\[
f(a,b)=aG+b(kG)=(a+bk)G.
\]

So \(f(a,b)\) only depends on the single hidden linear combination

\[
a+bk \pmod n.
\]

That means many different pairs \((a,b)\) produce the same curve point. Specifically,

\[
f(a,b)=f(a',b')
\]

whenever

\[
(a-a') + k(b-b') \equiv 0 \pmod n.
\]

So the function has a hidden period/subgroup:

\[
H=\{(a,b):a+bk\equiv 0\pmod n\}.
\]

Finding that hidden subgroup gives you \(k\). For example, one generator is roughly

\[
(k,-1),
\]

because

\[
k + k(-1) \equiv 0 \pmod n.
\]

The quantum computer does not “try private keys faster.” That is the wrong mental model. It prepares a superposition over many \((a,b)\), evaluates \(f(a,b)\) reversibly, and then uses the **quantum Fourier transform** to make the hidden periodicity interfere constructively. The measurement gives a linear constraint from which \(k\) can be recovered with high probability after a few samples. Proos and Zalka explicitly describe Shor’s discrete-log algorithm for elliptic-curve groups; NIST likewise states that elliptic-curve discrete log can be efficiently solved by Shor’s algorithm on a quantum computer. citeturn231294search0turn991618search0

In compressed form, the quantum attack is:

\[
Q=kG
\]

Define:

\[
f(a,b)=aG+bQ.
\]

Because \(Q=kG\):

\[
f(a,b)=(a+bk)G.
\]

So \(f\) hides the kernel:

\[
a+bk=0 \pmod n.
\]

Quantum Fourier sampling reveals that kernel. The kernel reveals \(k\). Once \(k\) is known, the private key is known.

This is why ECC does **not** merely lose “half its bits” under quantum attack. Symmetric cryptography mostly gets hit by Grover-style square-root speedups, so larger symmetric keys help. ECC gets hit by Shor-style structure extraction: the underlying discrete-log problem moves from “best known classical attack is roughly \(\sqrt n\)” to **polynomial time** on a sufficiently large fault-tolerant quantum computer. NIST’s post-quantum materials describe widely used public-key systems as vulnerable to Shor’s algorithm and motivate migration because practical large-scale quantum computers would break much of modern public-key cryptography. citeturn991618search4turn991618search6

For protocols:

**ECDH:** Alice publishes \(A=aG\), Bob publishes \(B=bG\). A quantum attacker recovers \(a\) from \(A=aG\) or \(b\) from \(B=bG\), then computes the shared secret \(abG\).

**ECDSA / EdDSA:** The public signing key is \(Q=dG\). A quantum attacker recovers \(d\), the signing secret, and can forge signatures.

So the deep answer is:

**Classically, the scalar coordinate \(k\) is hidden inside the elliptic-curve group representation. Quantumly, Shor’s algorithm turns “find the hidden scalar coordinate” into an abelian hidden-subgroup problem, then uses Fourier interference to recover the subgroup slope. That slope is the private key.**

## 13. User

How do we extend these general principles about hidden coordinates to quantum?

## 14. Assistant

The right abstraction is: **classically hidden coordinate = an isomorphism you know exists but cannot compute; quantumly breakable hidden coordinate = an isomorphism whose level sets form a Fourier-visible subgroup.**

## 15. Assistant

The extension is:

**Classical hidden-coordinate problem:**  
You know a coordinate exists, but the public representation does not expose it.

**Quantum hidden-coordinate problem:**  
The hidden coordinate becomes recoverable when it appears as a **period, kernel, stabilizer, or hidden subgroup** of an efficiently computable function. Quantum Fourier sampling can then recover the **dual constraints** defining that hidden structure.

That is the real general principle.

Classically, for ECC, the point

\[
Q = kG
\]

has a hidden scalar coordinate \(k\). But the public coordinates \((x_Q,y_Q)\) do not tell you \(k\). The curve representation lets you compute forward scalar multiplication, but not invert it.

Quantumly, you do not ask: “Can I read \(k\) from \(Q\)?” You instead build a function whose **symmetry depends on \(k\)**:

\[
F_k(a,b)=aG+bQ.
\]

Since \(Q=kG\),

\[
F_k(a,b)=aG+b(kG)=(a+bk)G.
\]

So \(F_k\) has a hidden kernel:

\[
H_k=\{(a,b)\in \mathbb Z_n^2 : a+bk\equiv 0 \pmod n\}.
\]

That kernel is a line in \(\mathbb Z_n^2\). Its slope is the private key.

So the quantum move is not “try many \(k\)’s in parallel.” That is the cartoon version. The real move is:

**turn the hidden coordinate into a hidden subgroup, then Fourier-sample the annihilator of that subgroup.**

Kitaev framed factoring and discrete logarithm under the broader **Abelian stabilizer** problem, giving polynomial quantum algorithms based on eigenvalue/phase measurement and Fourier transforms over finite abelian groups. citeturn698816view0 Proos and Zalka spell this out specifically for elliptic-curve discrete logarithms. citeturn698816view1

The mechanism is this. A quantum computer prepares a superposition over \((a,b)\):

\[
\frac{1}{n}\sum_{a,b}|a,b\rangle.
\]

It evaluates \(F_k\) coherently:

\[
\frac{1}{n}\sum_{a,b}|a,b\rangle|F_k(a,b)\rangle.
\]

If you measure or discard the output register, the input register collapses to a **coset state**:

\[
|s+H_k\rangle.
\]

That state is uniform over one affine copy of the hidden line. Now apply the quantum Fourier transform over \(\mathbb Z_n^2\). The Fourier transform of a coset state is supported only on the **orthogonal annihilator**:

\[
H_k^\perp.
\]

If the character is indexed by \((r,s)\), the sample satisfies, up to convention,

\[
s \equiv rk \pmod n.
\]

For prime \(n\), if \(r\neq 0\), then

\[
k \equiv s r^{-1} \pmod n.
\]

So the scalar coordinate is recovered from a **dual linear equation**.

That is the deep pattern:

\[
\textbf{hidden coordinate in primal space}
\quad\longrightarrow\quad
\textbf{visible constraint in Fourier/dual space}.
\]

Classical computation sees labels. Quantum computation can create interference among all labels in a coset, and the destructive interference cancels every frequency except those compatible with the hidden subgroup. That is why quantum algorithms are so strong for **abelian periodicity** problems.

This gives a clean general schema.

You start with a hard-looking map

\[
x \mapsto f_\theta(x)
\]

where \(\theta\) is the hidden coordinate. If the equality pattern of \(f_\theta\) is exactly cosets of some subgroup \(H_\theta\),

\[
f_\theta(x)=f_\theta(y)
\iff
x-y\in H_\theta,
\]

then \(H_\theta\) is the real hidden object. If the ambient domain is a finite abelian group and the quantum Fourier transform over that group is efficient, then quantum sampling gives elements of \(H_\theta^\perp\). From enough such samples, you reconstruct \(H_\theta\), and from \(H_\theta\), you recover \(\theta\).

ECC fits perfectly because the hidden coordinate \(k\) is the slope of a hidden line in \(\mathbb Z_n^2\).

Factoring fits too. The function

\[
x\mapsto a^x \bmod N
\]

has a hidden period \(r\), where \(a^r\equiv 1\pmod N\). Quantum Fourier sampling recovers that period. Then classical number theory turns \(r\) into factors of \(N\). Shor’s algorithm is the canonical example of this broader algebraic phenomenon; Childs and van Dam summarize Shor’s algorithm as the best-known dramatic quantum speedup and review the broader class of algebraic quantum algorithms built around this kind of structure. citeturn698816view2

But there is an important limit.

**Quantum computers do not automatically reveal all hidden coordinates.** They are especially good when the coordinate is hidden as **abelian symmetry**: periods, kernels, stabilizers, homomorphism fibers, cosets. If the hidden coordinate is not encoded as exact group symmetry, or if the group is nonabelian in a hard way, or if the oracle cannot be evaluated coherently and efficiently, the Shor/Kitaev machinery does not simply apply.

That is why “post-quantum” cryptography is not just “use larger ECC keys.” ECC and RSA fall to Shor-type algebraic algorithms, but lattice-, code-, hash-, and other constructions are chosen because no analogous efficient quantum hidden-subgroup extraction is known. NIST’s post-quantum project explicitly frames migration around replacing widely used systems that future quantum computers could break, and its first standards include ML-KEM, ML-DSA, and SLH-DSA rather than larger elliptic curves. citeturn698816view3

So the compact principle is:

**Classically, a cryptosystem is safe when the scalar coordinate exists but is not computable from the public representation. Quantumly, it is unsafe when that hidden coordinate can be re-expressed as the slope, period, kernel, or stabilizer of an efficiently computable abelian action. The quantum Fourier transform does not read the coordinate directly; it reads the dual constraints whose intersection determines the coordinate.**

## 16. User

From first principles, how do we select a way to hide the coordinates in a way that is not quantum exploitable but is efficient to forward compute and results in maximally small keys?

## 17. Assistant

The first-principles answer is: **do not hide a coordinate inside an exact low-dimensional group action. Hide it inside a high-dimensional noisy relation whose legitimate use only requires coarse recovery, but whose full inversion appears to require solving a hard decoding/lattice problem.**

ECC fails quantumly because the secret is a **slope of an exact abelian symmetry**:

\[
Q=kG
\]

and the attacker can build

\[
F(a,b)=aG+bQ=(a+bk)G.
\]

That function has exact coset structure. Quantum Fourier sampling sees the hidden line. So, from first principles, the post-quantum design rule is:

**Avoid exact periodicity. Avoid exact homomorphism fibers. Avoid hidden subgroup structure over efficiently Fourier-transformable groups.**

The replacement pattern is **noise plus dimension**.

In lattice cryptography, the public key has the shape

\[
b = A s + e \pmod q
\]

where \(s\) is the secret coordinate and \(e\) is small noise. If \(e=0\), the scheme is dead: solve linear equations. With \(e\neq 0\), you no longer have exact cosets. A quantum Fourier transform does not reveal a sharp annihilator line, because there is no exact subgroup to annihilate. Recovering \(s\) becomes a **high-dimensional approximate decoding problem**, not a hidden-period problem.

This is the conceptual move from ECC to LWE/MLWE:

**ECC:** exact low-dimensional algebraic coordinate.  
**LWE/MLWE:** noisy high-dimensional linear coordinate.

That is why the current practical answer is **module-lattice cryptography**, especially **ML-KEM** for key establishment. NIST’s FIPS 203 specifies ML-KEM and says its security is related to the difficulty of **Module Learning With Errors**; NIST describes it as presently believed secure even against adversaries with quantum computers. citeturn341111view0

The basic ML-KEM/Kyber intuition is:

\[
t = A s + e
\]

Public key contains \(A,t\). Encapsulation creates noisy correlated values using fresh randomness. Decapsulation uses the secret \(s\) to cancel the main term and recover a message bit/string by rounding. The public attacker sees only noisy correlations. The honest party never needs to solve the full inverse problem; they only need noise small enough that rounding succeeds.

So the selection criterion is not “find the most elegant hidden coordinate.” It is closer to:

**Choose the smallest public representation that still forces the attacker into the best-known high-dimensional decoding/lattice attacks, after accounting for quantum speedups.**

That immediately creates a tradeoff. Small keys require **structure**. But structure is exactly what attackers exploit. So the design space is a spectrum:

| Family | Coordinate-hiding mechanism | Key-size behavior | Main issue |
|---|---:|---:|---|
| ECC/RSA/DLP | Exact algebraic coordinate | Tiny | Broken by Shor |
| Plain LWE | Noisy high-dimensional linear equations | Large | Safe-ish but bulky |
| Ring-LWE/NTRU | Noisy structured polynomial equations | Small | More algebraic structure |
| Module-LWE | Middle ground: structured, but less collapsed than pure ring | Small/practical | Current best general tradeoff |
| Code-based | Decoding random/structured codes | Often larger | Good diversity, bulkier |
| Hash-based signatures | Hash preimages/Merkle trees | Small public keys, large signatures | Conservative but big signatures |

The “maximally small key” goal is dangerous if taken literally. The smallest schemes tend to be those with the most exploitable algebraic compression. Isogeny systems were attractive partly because they looked ECC-like and compact, but SIKE was broken by classical cryptanalysis; the SIKE team noted that no known SIDH variant retained roughly the same performance while remaining secure. citeturn239148search3 That episode is a useful warning: **compactness purchased with delicate algebra is often cryptanalytic debt.**

For standardized key establishment today, the practical Pareto point is ML-KEM. The sizes are much larger than ECC but still operationally small: ML-KEM-512 has an 800-byte public/encapsulation key and 768-byte ciphertext; ML-KEM-768 has a 1184-byte public key and 1088-byte ciphertext; ML-KEM-1024 has a 1568-byte public key and 1568-byte ciphertext. NIST recommends **ML-KEM-768 as the default parameter set** because it gives a larger security margin at reasonable performance cost. citeturn655932view0

So, if the question is “what would I pick under current knowledge?”:

**For KEM/key exchange:** use **ML-KEM-768** by default. Use ML-KEM-512 only when bandwidth/latency constraints really matter and the security category is acceptable. Use ML-KEM-1024 when high assurance matters more than size.

**For signatures:** use **ML-DSA** as the main standardized lattice signature, and **SLH-DSA** as a conservative hash-based backup if you can tolerate larger signatures. NIST finalized FIPS 204 for ML-DSA and FIPS 205 for SLH-DSA; NIST describes SLH-DSA as based on different math and intended as a backup if ML-DSA proves vulnerable. citeturn469113view0

**For algorithmic diversity:** watch **HQC**, which NIST selected as a backup KEM based on different mathematics from ML-KEM. NIST explicitly says HQC is not intended to replace ML-KEM, but to provide a fallback if ML-KEM were weakened. citeturn763325search0

The deeper design law is this:

**A post-quantum public-key scheme wants just enough algebra to compute forward efficiently and compress the public object, but not so much algebra that the hidden coordinate becomes an exact symmetry.**

Lattices hit that balance by using:

**high dimension** to make search exponential-ish,  
**noise** to destroy exact Fourier-visible periods,  
**modular arithmetic** for compact computation,  
**polynomial/module structure** for key compression and fast multiplication,  
**parameter inflation** to cover known classical and quantum attacks.

That is the reason ML-KEM keys are kilobytes, not 32 bytes. The extra bytes are not sloppy engineering; they are the price of abandoning ECC’s exact abelian coordinate, which is precisely what Shor exploits.

## 18. User

noise and dimension, etc seems kind of wacky. It is not a principled solution. We need to think more clearly about what it means to be able to hide the coordinate system in a suitable way and produce some candidate solutions. We are going beyond the known frontier now. We might find it helpful to use category theory language or some other framework that helps us generalize and explore solutions in that more abstract space.

## 19. Assistant

You are right: **“add noise and dimension” is not the principle**. It is one engineering move that happens to block the most obvious Fourier attack. The deeper principle is closer to this:

**A public-key scheme hides a coordinate system by publishing an object after forgetting a trivialization. A quantum attack succeeds when the forgotten coordinate is the kernel, period, shift, or stabilizer of an efficiently Fourier-decomposable action.**

So the design target is not “noise.” It is:

\[
\boxed{\text{make coordinate recovery a hard lifting/section problem whose kernel groupoid is not Fourier-readable.}}
\]

## 1. Coordinate hiding as a forgetful functor

Let \(\mathcal E\) be a category of **coordinatized objects**. An object might be a vector space with a secret basis, a curve with a secret isogeny path, a code with a secret permutation, a module with a secret decomposition, a bundle with a secret trivialization, etc.

Let \(\mathcal P\) be the category of **public objects**, and let

\[
U:\mathcal E\to\mathcal P
\]

be a forgetful functor.

A private key is some coordinate/trivialization/path/lift \(\tau\). The public key is

\[
p = U(X,\tau).
\]

Breaking the scheme means constructing an efficient **section**, lift, or morphism in the fiber of \(U\):

\[
\text{given } U(X,\tau),\quad \text{recover something equivalent to } \tau.
\]

In ECC, this forgetful map is too clean. The secret coordinate is \(k\), the public object is

\[
Q=kG,
\]

and the hidden coordinate lives in the cyclic group \(\mathbb Z/n\mathbb Z\). The fiber is an **abelian torsor**. That is exactly the structure quantum Fourier methods love.

Kitaev’s Abelian Stabilizer Problem already includes factoring and discrete logarithm, and gives polynomial-time quantum algorithms for finite abelian stabilizer/period problems. citeturn183104search0 Proos and Zalka spell this out specifically for elliptic-curve discrete logarithms. citeturn183104search1

So ECC fails because its coordinate-forgetting functor has a kernel pair like

\[
E\times_{\mathcal P}E \simeq E\times \mathbb Z/n\mathbb Z,
\]

and \(\mathbb Z/n\mathbb Z\) has completely transparent character theory.

## 2. The quantum exploitability criterion

A hidden-coordinate scheme is quantum-fragile when the public map factors through something like:

\[
A \curvearrowright X,\qquad a\mapsto a\cdot x,
\]

where \(A\) is a finite abelian group, or close enough to one, and equality of public outputs gives exact cosets:

\[
f(x)=f(y)\iff x-y\in H.
\]

Then quantum Fourier sampling gives samples from \(H^\perp\), the annihilator of the hidden subgroup. The hidden coordinate is recovered from dual linear constraints.

This generalizes from “abelian subgroup” to “hidden shift.” Even when the underlying group action is commutative but not quite ECC-like, quantum algorithms can still do substantially better than classical search. Kuperberg’s algorithm gives subexponential quantum time for the dihedral hidden subgroup problem and applies to hidden shift over finitely generated abelian groups. citeturn208897search0 This is why commutative group-action systems such as CSIDH are not “ECC-broken by Shor,” but they still live uncomfortably close to known quantum hidden-shift machinery; CSIDH itself is explicitly a commutative group-action construction. citeturn953298search0

So a sharper rule is:

\[
\boxed{
\text{Do not let the secret coordinate be recoverable from an efficiently diagonalizable group algebra.}
}
\]

For a finite group \(G\), the quantum Fourier transform is essentially block-diagonalization of \(\mathbb C[G]\). For abelian \(G\), every irreducible representation is one-dimensional, so the hidden coordinate becomes a set of scalar phases. For nonabelian \(G\), the Fourier transform lands in matrix blocks, and extracting the hidden object is much less automatic. There is no general efficient quantum solution for the nonabelian HSP; efficient solutions in broad cases would imply progress on problems such as graph isomorphism. citeturn208897search8

That does **not** mean “use any nonabelian group and you are safe.” It means the relevant invariant is representation-theoretic:

**Does the coordinate groupoid have a large accessible abelianization, many low-dimensional irreps, an efficiently computable character table, or a reduction to hidden shift?**

If yes, danger.

## 3. The categorical principle

The clean abstraction is:

\[
U:\mathcal E\to\mathcal P
\]

where \(\mathcal E\) is the category of objects with hidden coordinates, and \(\mathcal P\) is the quotient/forgotten public category.

The **coordinate ambiguity** over a public object \(p\) is not just a set. It is a fiber groupoid:

\[
\mathcal K_p = \mathcal E\times_{\mathcal P}\{p\}.
\]

An attacker wants to choose a point, morphism, or trivialization in \(\mathcal K_p\). A quantum attacker is powerful when \(\mathcal K_p\) is equivalent to a torsor under a Fourier-friendly group.

So the candidate post-quantum design principle is:

\[
\boxed{
\text{Use a forgetful functor whose fibers are hard groupoids, not abelian torsors.}
}
\]

Or more compactly:

\[
\boxed{
\text{hide coordinates in quotient stacks, not cyclic groups.}
}
\]

ECC hides the coordinate in a cyclic torsor. LWE hides it in a noisy high-dimensional affine structure. A more geometric frontier would hide it in a **nonabelian moduli groupoid**.

## 4. What “suitable hiding” should mean

A coordinate-hiding scheme should satisfy several structural conditions.

First, **forward evaluation must be cheap**. The honest user should apply a short morphism, correspondence, basis change, tensor operation, isogeny, or functor:

\[
s:X_0\to X_s.
\]

Second, **inversion should be a lifting problem**, not arithmetic in a known group:

\[
U(X_0),U(X_s)\quad\mapsto\quad s?
\]

Third, the ambiguity groupoid should have **small abelian shadow**. If the relevant groupoid has a large computable abelianization,

\[
\mathcal G \to \mathcal G_{\mathrm{ab}},
\]

and the secret mostly lives there, quantum hidden-shift methods may attack the abelian quotient.

Fourth, there should be **no efficient canonicalization**. If there is a canonical public representative

\[
\operatorname{can}(U(X)),
\]

then the hidden coordinate may become readable by comparing canonical forms. In category language, an efficient canonicalizer is an efficient section/skeleton functor. That is exactly what we do not want.

Fifth, the public encoding must avoid auxiliary data that linearizes the problem. The SIKE failure is a warning here: extra torsion/endomorphism information made a very compact isogeny construction collapse under classical cryptanalysis; the SIKE team’s postscript notes that known modifications avoiding the attack severely worsened performance and key size. citeturn953298search9

## 5. Candidate direction A: nonabelian groupoid-action cryptography

The most direct replacement for ECC is not “lattice with noise.” It is:

\[
\textbf{ECC: group action by } \mathbb Z/n\mathbb Z
\]

replaced by

\[
\textbf{post-quantum candidate: groupoid action by a nonabelian correspondence groupoid.}
\]

Let \(\mathcal G\) be a finite or effectively finite groupoid acting on a class of compact algebraic objects \(\mathcal X\). A secret key is a short morphism/path

\[
g:X_0\to X_1.
\]

The public key is only the public moduli representative

\[
U(X_1).
\]

Breaking means recovering a path \(g\), or an equivalent morphism in \(\mathcal G\).

This is categorically clean: the secret is a **morphism**, the public key is an **object up to isomorphism**, and the attack is **path recovery in a quotient stack**.

The reason isogeny cryptography remains intellectually attractive, despite SIKE’s failure, is that elliptic-curve or abelian-variety moduli give very compact public objects. CSIDH explicitly follows the hard-homogeneous-space/group-action line, but its action is commutative. citeturn953298search0 The next conceptual move would be to use **noncommutative isogeny groupoids**, not abelian class-group actions.

A plausible candidate shape:

\[
\text{objects: supersingular curves / abelian varieties / oriented varieties}
\]

\[
\text{morphisms: ideals, isogenies, correspondences, Hecke modifications}
\]

\[
\text{secret: short noncommuting path}
\]

\[
\text{public key: compact moduli invariant}
\]

Existing work around group and groupoid actions already frames graph/code/lattice/isogeny isomorphism problems in this language; recent lecture material explicitly lists cryptographic group actions, groupoids, isogenies, code isomorphism, lattice isomorphism, and graph isomorphism under a common groupoid-action umbrella. citeturn488717search5

This is probably the cleanest “beyond frontier” direction if the goal is **small keys**.

## 6. Candidate direction B: bimodule/interchange-law key exchange

A major obstruction: Diffie–Hellman wants two parties’ operations to commute:

\[
a(bX)=b(aX).
\]

If we require secrets to come from one commutative group, we are back near abelian hidden shift. So instead, use **two noncommutative actions that commute only by functoriality**.

Let \(\mathcal L\) and \(\mathcal R\) be two nonabelian monoidal categories acting from the left and right on a space of objects \(\mathcal X\):

\[
L\triangleright X,\qquad X\triangleleft R.
\]

Require an interchange law:

\[
L\triangleright (X\triangleleft R)
\cong
(L\triangleright X)\triangleleft R.
\]

Then Alice samples \(L\), publishes

\[
A=L\triangleright X_0.
\]

Bob samples \(R\), publishes

\[
B=X_0\triangleleft R.
\]

Both derive

\[
K=L\triangleright X_0\triangleleft R.
\]

This has the **shape** of Diffie–Hellman but does not require a single abelian secret group. The secrets live in a bicategory of bimodules/correspondences. The shared-key identity comes from associativity/interchange, not commutativity.

Candidate instantiations could involve:

**noncommutative orders and ideal bimodules**,  
**module categories over finite tensor categories**,  
**Hecke modifications of bundles**,  
**correspondences between algebraic varieties**,  
**quiver-representation moduli with left/right tensor actions**.

This is speculative, but structurally it is much cleaner than “take a commutative group action and hope Kuperberg is expensive enough.”

The central security question would be:

\[
\text{given } X_0,\;L\triangleright X_0,\quad \text{recover }L?
\]

and similarly on the right. The attack should be a **nonabelian double-coset/path problem**, not a hidden shift over an abelian group.

## 7. Candidate direction C: hard canonicalization / quotient by relabeling

Another principled way to hide coordinates is to publish an object modulo change of coordinates:

\[
X \sim gX,\qquad g\in G.
\]

The secret is a labeling, basis, permutation, or isomorphism. The public object is the quotient object.

This includes graph isomorphism, code equivalence, lattice isomorphism, module isomorphism, and related “hidden basis” problems. Quantumly, these correspond more naturally to **nonabelian** hidden subgroup/isomorphism problems, not abelian periods. The nonabelian HSP remains fundamentally less understood than the abelian case; Hallgren–Russell–Ta-Shma summarize the contrast between efficient abelian HSP algorithms and the open nonabelian setting, including the graph-isomorphism connection. citeturn208897search8

The problem is key size. Random graphs, random codes, and random modules are usually too large to describe near the ECC scale. So the interesting frontier is not raw graph/code isomorphism. It is:

\[
\boxed{
\text{compactly described moduli objects whose coordinate isomorphism problem remains hard.}
}
\]

That suggests algebraic graph families, structured codes, module categories, sheaf moduli, or arithmetic varieties—but every structure added for compression is also potential attack surface.

## 8. Candidate direction D: noninvertible categorical factorization

Shor-like attacks exploit **invertible group symmetries**. Another route is to abandon groups entirely and hide coordinates in a category with many noninvertible morphisms.

Let \(\mathcal C\) be a monoidal category or semigroup-like category. A secret is a short factorization

\[
f = f_\ell\circ\cdots\circ f_2\circ f_1.
\]

The public key is the resulting object or composite invariant:

\[
U(fX_0).
\]

Breaking means finding a short factorization or equivalent path. There is no global group of coordinates, no inverse operation, and no immediate group algebra to Fourier-diagonalize.

Possible domains:

**quiver mutations**,  
**nonsemisimple module categories**,  
**matrix factorizations**,  
**sheaf modifications**,  
**correspondence categories**,  
**rewriting systems with hard normal-form inversion**.

The danger is linearization. Many noncommutative or semigroup proposals die because the public operation secretly embeds into matrices over a finite field, after which ordinary linear algebra recovers the secret. So this direction needs a brutal **linearization audit**.

## 9. Candidate direction E: approximate/enriched coordinate hiding

This is where lattices belong, but stated more cleanly.

Instead of a strict functor

\[
U:\mathcal E\to\mathcal P,
\]

use a stochastic/enriched functor, or Markov kernel,

\[
K:\mathcal E\rightsquigarrow\mathcal P.
\]

Then the public object is not a point in an exact quotient. It is a sample from a distribution around the forgotten coordinate.

In LWE-like form:

\[
b=As+e \pmod q.
\]

The exact affine fiber \(b=As\) would be linear algebra. The error term makes the fiber fuzzy; Fourier sampling no longer gives exact annihilators. This is not philosophically wacky if phrased as:

\[
\boxed{
\text{replace exact quotient fibers by smoothed fibers with bounded coordinate information.}
}
\]

Regev’s LWE work gives reductions from worst-case lattice problems to LWE-type learning problems, which is why this family has unusually strong theoretical support compared with many algebraic alternatives. citeturn183104search14 NIST’s ML-KEM standard is based on Module-LWE and states that ML-KEM is presently believed secure even against adversaries with quantum computers. citeturn197741search0

So noise is not the principle, but **smoothing a quotient so there is no exact kernel pair** is a principle.

## 10. The small-key constraint

For truly small keys, raw high-dimensional randomness is probably the wrong object. You want a compact moduli space.

Information-theoretically, for \(\lambda\)-bit quantum security against generic search, the hidden coordinate space should have size around

\[
2^{2\lambda}
\]

because Grover gives square-root search. That means the public moduli space needs at least roughly \(2\lambda\) bits of entropy. For 128-bit quantum security, a fantasy lower bound is on the order of **256 bits** of public-object entropy. But achieving that with an actually hard, efficiently computable, non-Fourier-readable structure is the whole problem.

This points toward:

\[
\boxed{
\text{compact algebraic moduli + hard nonabelian path inversion}
}
\]

rather than:

\[
\text{huge random matrices/codes/graphs}.
\]

That is why isogeny signatures such as SQISign remain interesting: they target very compact keys/signatures using quaternionic and isogeny structure. The current SQISign NIST Round 2 specification is explicitly in the compact post-quantum signature line based on quaternions and isogenies. citeturn893146search8 Whether analogous compact KEMs can be made robust is a much harder question.

## 11. The research program I would actually pursue

The most coherent frontier direction is:

\[
\boxed{
\textbf{nonabelian quotient-stack / groupoid-action cryptography over compact moduli spaces.}
}
\]

I would formalize a candidate as follows.

Choose a compactly representable moduli problem \(\mathcal M\), with objects over finite fields and short public invariants:

\[
\operatorname{inv}(X)\in\{0,1\}^m.
\]

Choose a nonabelian correspondence groupoid \(\mathcal G\) acting on \(\mathcal M\). Secret keys are short paths

\[
g:X_0\to X_g.
\]

Public keys are

\[
\operatorname{inv}(X_g).
\]

The central hardness assumption is:

\[
\textbf{Groupoid Action Inversion:}
\quad
(X_0,X_g)\mapsto g
\]

is hard on average, even quantumly.

Then audit the construction with five tests:

**Abelianization test:** does the secret project to a large computable abelian quotient?

**Representation-leakage test:** do low-dimensional irreducible representations distinguish the hidden path?

**Linearization test:** can the action be embedded into linear algebra over a known ring/field?

**Canonicalization test:** is there an efficient normal form for public objects?

**Auxiliary-data test:** does validation or key exchange reveal extra structure that turns the groupoid problem into an easier module/endomorphism problem?

The slogan version:

\[
\boxed{
\text{A good post-quantum hidden coordinate is not a number. It is a morphism in a hard groupoid.}
}
\]

Or, even more compressed:

\[
\boxed{
\text{ECC hides a scalar in an abelian torsor. A frontier PQ scheme should hide a trivialization in a nonabelian quotient stack.}
}
\]

The hard part is making that object **small** without adding algebraic structure that reintroduces a hidden torus, class group, linear representation, or canonical form. That is the design tension. But it is a much clearer target than “noise and dimension.”

## 20. User

Ok let's go down that path that you suggest. We need to identify that small but algebraically suitable object. We can construct it if needed

## 21. Assistant

Good. Then the key move is to stop looking for “a new curve” and instead look for a **compact moduli object whose forgotten coordinate category is not a group**.

The best candidate class I see is:

\[
\boxed{
\textbf{supersingular elliptic / superspecial abelian-variety moduli points, acted on by a monoidal module category}
}
\]

More concretely:

\[
\boxed{
\textbf{public object: a tiny moduli invariant, e.g. } j(E)\in \mathbb F_{p^2};
\qquad
\textbf{secret coordinate: a module/isogeny presentation hidden behind that invariant.}
}
\]

This is not merely a poetic answer. It is very close to the emerging **module isogeny** direction: Robert’s 2024 “module action for isogeny based cryptography” proposes replacing a commutative group action by a module action; its advertised \(\otimes\)-MIKE construction has users send only the \(j\)-invariant of a curve, requires no torsion information, and derives a common secret as a dimension-4 principally polarized abelian variety. The same abstract explicitly says Kuperberg’s algorithm presumably does not apply, while warning that some variants are insecure if Alice and Bob do not use coprime isogeny degrees. citeturn141841search1

That is exactly the kind of object we were trying to invent.

## 1. What we are actually selecting

A “small but suitable object” is not just an object \(X\). It is a triple:

\[
\mathcal C
\xrightarrow{\;\Phi\;}
\mathcal X
\xrightarrow{\;\operatorname{coarse}\;}
\{0,1\}^m.
\]

Here:

**\(\mathcal C\)** is the hidden coordinate category: modules, lattices, ideals, correspondences, isogeny paths, polarizations.

**\(\Phi\)** is the efficient forward construction: turn the hidden coordinate into an algebraic object.

**\(\operatorname{coarse}\)** is the public forgetting map: publish only a compact invariant such as a \(j\)-invariant, Igusa invariants, or theta constants.

ECC has

\[
\mathcal C \simeq \mathbb Z/n\mathbb Z,
\]

which is fatal: the coordinate category is an abelian group. CSIDH improves the public object but still uses a commutative group action; the original CSIDH paper advertises an efficient post-quantum commutative group action and 64-byte public keys at conjectured NIST category-1 security. citeturn276603search0 But commutative group actions remain close to hidden-shift structure: quantum analyses of CSIDH explicitly use Kuperberg-style hidden-shift algorithms. citeturn276603search2 De Feo’s 2025 groupoid-action slides summarize the same dividing line: abelian group-action inversion leads to hidden shift and Kuperberg; for nonabelian settings, no comparable general quantum speedup is known. citeturn455887view3

So the object we want should **not** be a torsor under a group. It should be a **small moduli point whose hidden coordinates form a monoidal category, groupoid, or correspondence category with weak abelian shadow**.

## 2. The primary candidate: module-isogeny coordinates over a supersingular curve

Start with a supersingular elliptic curve

\[
E_0/\mathbb F_{p^2}.
\]

The public representation of an elliptic curve is tiny:

\[
j(E)\in \mathbb F_{p^2}.
\]

That is one field element. If \(p\) is about 256 bits, this is about 512 bits, i.e. 64 bytes. This is the same compactness regime that made isogeny systems so attractive.

But we do **not** use the elliptic-curve group \(E(\mathbb F_q)\) as the cryptographic group. We use \(E_0\) as a **base object** for a module/action construction.

Let \(R\) be a suitable order acting on \(E_0\), morally a suborder of \(\operatorname{End}(E_0)\). Let \(\mathcal C_R\) be a category of finite-rank \(R\)-modules or \(R\)-lattices, with polarization/Hermitian data when needed. There is a Serre-type tensor construction

\[
M \longmapsto M\otimes_R E_0.
\]

This sends a hidden module \(M\) to an abelian variety. The public key is only a coarse moduli invariant of the result.

The key exchange shape is:

\[
M_A,\;M_B \in \mathcal C_R.
\]

Alice publishes something like

\[
P_A = \operatorname{coarse}(M_A\otimes_R E_0),
\]

Bob publishes

\[
P_B = \operatorname{coarse}(M_B\otimes_R E_0).
\]

The shared object is

\[
K_{AB}
=
\operatorname{coarse}\bigl((M_A\otimes_R M_B)\otimes_R E_0\bigr).
\]

The reason both parties agree is not group commutativity in the ECC/CSIDH sense. It is **monoidal functoriality**:

\[
M_A\star(M_B\star E_0)
\cong
(M_A\otimes_R M_B)\star E_0
\cong
(M_B\otimes_R M_A)\star E_0
\cong
M_B\star(M_A\star E_0),
\]

assuming the chosen module category has the necessary symmetric or braided monoidal structure.

This is the categorical replacement for Diffie–Hellman.

The hidden coordinate is no longer a scalar \(k\), nor a class-group element \(g\), but a **module presentation / isogeny path / local modification pattern**. The public object forgets that presentation.

So the central hardness assumption becomes:

\[
\boxed{
\textbf{given } E_0 \textbf{ and } \operatorname{coarse}(M\otimes_R E_0),
\textbf{ recover a usable } M.
}
\]

And for key exchange:

\[
\boxed{
\textbf{given } E_0,\;M_A\star E_0,\;M_B\star E_0,
\textbf{ compute } (M_A\otimes_R M_B)\star E_0.
}
\]

This is a **monoidal action parallelization problem**, not a discrete log problem.

## 3. Why this is the right abstraction

The public object remains very small because elliptic curves have one-dimensional coarse moduli:

\[
E \longmapsto j(E).
\]

But the hidden coordinate space can be much richer than the curve itself. The hidden algebra lives in modules, isogenies, polarizations, and higher-dimensional abelian varieties.

That is the trick:

\[
\boxed{
\textbf{use dimension 1 for public compression, but higher-dimensional/module structure for security.}
}
\]

This is also why genus-2 and dimension-4 objects keep appearing in the frontier. Superspecial abelian-variety isogeny graphs are already known to have rich quaternionic structure: Jordan and Zaytman describe superspecial \([\ell]\)-isogeny graphs as generalized Brandt graphs, connected in their setting, constructible via definite quaternion algebras, and related to quotients of Bruhat–Tits buildings by quaternionic unitary groups. citeturn455887view0 Richelot isogeny graphs of principally polarized abelian surfaces are also studied as cryptographic objects in genus 2. citeturn455887view1turn455887view4

But if we publish genus-2 objects directly, the public key usually becomes several field elements. If we can publish only \(j(E)\) while pushing the common secret into dimension 4 internally, that is the compactness sweet spot.

## 4. The design rule: avoid the invertible rank-1 collapse

There is a sharp failure mode.

If the secret modules are **rank-1 invertible projective modules**, then their isomorphism classes form the Picard/class group:

\[
\operatorname{Pic}(R).
\]

That collapses back to a commutative group action. Then we are back near CSIDH, hidden shift, and Kuperberg.

So the candidate must avoid living primarily in

\[
K_0,\quad \operatorname{Pic}(R),\quad \det(M),\quad \text{or any other easily computable abelian quotient.}
\]

The dangerous map is something like

\[
M \longmapsto \det(M)\in \operatorname{Pic}(R).
\]

If the public key or protocol behavior leaks \(\det(M)\), Fitting ideals, local elementary divisors, or an orientation class, then the attacker may recover a large abelian projection of the secret. That projection may be enough.

So the construction should enforce something like:

\[
\det(M) = D_0
\]

for a fixed public determinant, or otherwise mask/fix the Picard component, while leaving entropy in non-invertible, higher-rank, or polarization data.

A crude slogan:

\[
\boxed{
\textbf{rank-1 invertible modules are too group-like; use higher-rank or noninvertible module coordinates.}
}
\]

This is why the module-action direction is more interesting than simply “another class group action.”

## 5. A concrete candidate construction

Here is the clean prototype I would put on the whiteboard.

### Setup

Choose a large prime \(p\) and a supersingular elliptic curve

\[
E_0/\mathbb F_{p^2}.
\]

Let \(R\) be a chosen order acting on \(E_0\), with enough explicit structure to compute the forward action but not enough public structure to canonicalize arbitrary endpoints.

Define a hidden coordinate category \(\mathcal C_R\) of polarized rank-\(r\) \(R\)-modules. For the smallest practical candidate, take \(r=2\), because \(r=1\) risks collapsing to \(\operatorname{Pic}(R)\).

Define the forward functor:

\[
\Phi(M)=M\otimes_R E_0.
\]

Depending on the exact module class, \(\Phi(M)\) may be an elliptic curve, an abelian surface, or an intermediate object from which an elliptic endpoint is extracted. The public encoding is:

\[
\operatorname{pub}(M)=\operatorname{coarse}(\Phi(M)).
\]

In the most aggressive small-key version:

\[
\operatorname{pub}(M)=j(E_M)\in \mathbb F_{p^2}.
\]

### Alice

Alice samples a short coordinate presentation

\[
M_A = T_{a_L}\cdots T_{a_2}T_{a_1}(M_0),
\]

where the \(T_i\) are elementary local modifications / small-prime isogeny steps / module transformations.

She computes

\[
P_A=\operatorname{pub}(M_A)
\]

and publishes \(P_A\).

### Bob

Bob samples

\[
M_B = T_{b_L}\cdots T_{b_2}T_{b_1}(M_0)
\]

from a compatible but preferably coprime degree family, computes

\[
P_B=\operatorname{pub}(M_B),
\]

and publishes \(P_B\).

### Shared secret

Alice computes

\[
K_A
=
\operatorname{coarse}\bigl((M_A\otimes_R M_B)\otimes_R E_0\bigr)
\]

using her coordinate \(M_A\) and Bob’s public object.

Bob computes

\[
K_B
=
\operatorname{coarse}\bigl((M_B\otimes_R M_A)\otimes_R E_0\bigr).
\]

The monoidal symmetry gives

\[
K_A = K_B
\]

up to canonical isomorphism, after which both hash a canonical invariant.

This gives a public-key primitive whose public keys can plausibly be as small as \(j\)-invariants, while the shared secret lives in a higher-dimensional moduli space.

This is essentially the principle behind the \(\otimes\)-MIKE line, but I am phrasing it as a general design pattern rather than a finished standardized protocol. The current literature treats this as active research, not settled engineering. citeturn141841search1

## 6. What must be audited immediately

This construction lives or dies on whether the hidden coordinate category has a dangerous abelian shadow. I would audit in this order.

**First: Picard leakage.**  
Does the public key determine \(\det(M)\), a class-group element, or a rank-1 ideal component? If yes, the problem may reduce to a hidden shift.

**Second: canonicalization.**  
Can one compute a canonical representative of \(\Phi(M)\) or its endomorphism ring? If yes, the hidden coordinate may become recoverable as a normal-form comparison.

**Third: auxiliary data.**  
Do not publish torsion images, bases, orientation certificates, or endomorphism hints unless there is a proof they do not linearize the problem. SIDH/SIKE is the canonical warning: the extra torsion information needed for key exchange became the attack surface.

**Fourth: decomposable-product leakage.**  
In higher-dimensional abelian varieties, product vertices such as \(E_1\times E_2\) can be special. Genus-2 path-finding algorithms exploit routes through lower-dimensional/product structure, so parameters and sampling must avoid making decomposition too easy. Superspecial genus-2 and abelian-surface isogeny graphs have enough studied local structure that this is not a hypothetical concern. citeturn455887view1turn455887view4

**Fifth: quantum claw/path search.**  
Even if Kuperberg does not apply, generic quantum search, claw finding, and quantum walks still apply. The security estimate must be based on the actual endpoint space, branching, multiplicity of equivalent paths, and whether any path or only the original coordinate suffices.

## 7. The second candidate: superspecial genus-2 public objects

If the module-action construction is too delicate, the next object is:

\[
\boxed{
\textbf{a principally polarized superspecial abelian surface, usually represented as } J(C)
}
\]

where \(C\) is a genus-2 curve.

The public key is then the tuple of absolute Igusa invariants:

\[
(j_1,j_2,j_3).
\]

This is still small: three field elements rather than one. The hidden coordinate is a path in a Richelot or \((\ell,\ell)\)-isogeny graph.

The forward computation is clean: repeatedly apply small-degree isogenies, especially Richelot \((2,2)\)-isogenies. The public object forgets the path.

The security problem is:

\[
\text{given } A_0,A_1,\quad \text{find an isogeny path } A_0\to A_1.
\]

This is more naturally a **nonabelian groupoid path problem** than a hidden shift problem. The literature around superspecial abelian surfaces, Richelot graphs, quaternionic hermitian forms, and Brandt graphs supports exactly this geometry. citeturn455887view0turn455887view1

But for key exchange, this object alone is not enough. A generic noncommutative path graph does not give

\[
a(bX)=b(aX).
\]

So genus-2 superspecial objects are immediately useful for **hashes, identification schemes, and signatures**, but key exchange requires an additional monoidal/interchange mechanism. That brings us back to the module-action idea.

## 8. My current ranking

| Candidate | Public size | Hidden coordinate | Main virtue | Main risk |
|---|---:|---|---|---|
| Supersingular \(j(E)\) with raw isogeny path | **1 field element** | noncommutative path | smallest possible object | no natural DH/NIKE |
| CSIDH-style oriented curve | **1 field element** | class-group element | elegant NIKE | abelian hidden shift / Kuperberg |
| Superspecial genus-2 surface | **3 field elements** | Richelot/isogeny path | genuinely groupoid-like | key exchange not automatic |
| Module-isogeny / \(\otimes\)-MIKE-like object | **possibly 1 field element** | higher-rank module/action coordinate | small keys plus NIKE shape | young, needs brutal audit |
| Random code/lattice/graph isomorphism | large | hidden basis/isomorphism | conceptually nonabelian | not compact enough |

So the best research target is:

\[
\boxed{
\textbf{module-isogeny key exchange over supersingular elliptic curves, with public } j\textbf{-invariants and hidden higher-rank module coordinates.}
}
\]

The fallback object for a more conservative one-way/signature primitive is:

\[
\boxed{
\textbf{superspecial principally polarized abelian surfaces, public Igusa invariants, secret Richelot/isogeny paths.}
}
\]

## 9. The clean hardness assumption to formulate

I would not formulate the assumption as “isogeny is hard.” Too vague.

I would define:

\[
\textbf{Monoidal Action Inversion:}
\]

Given

\[
X_0,\quad X_M=\operatorname{coarse}(M\star X_0),
\]

recover a module-coordinate \(M'\) such that

\[
M'\star X_0 \cong X_M.
\]

And:

\[
\textbf{Monoidal Action Parallelization:}
\]

Given

\[
X_0,\quad X_M,\quad X_N,
\]

compute

\[
X_{M\otimes N}
\]

without knowing either \(M\) or \(N\).

The second assumption is the real key-exchange assumption. It is the categorical analogue of computational Diffie–Hellman, but without an underlying group exponent.

That is the object-level replacement for ECDH.

## 10. The construction target

The concrete target I would pursue is:

\[
\boxed{
\begin{aligned}
&\textbf{Base object: } E_0/\mathbb F_{p^2}\text{ supersingular.}\\
&\textbf{Public key: } j(E_M)\in \mathbb F_{p^2}.\\
&\textbf{Secret: } \text{short presentation of a rank-2 module / isogeny coordinate }M.\\
&\textbf{Agreement: } \operatorname{Hash}\bigl(\operatorname{inv}((M_A\otimes M_B)\star E_0)\bigr).\\
&\textbf{Security goal: } \text{no reduction to Picard/class-group hidden shift; no torsion leakage.}
\end{aligned}
}
\]

The conceptual point is:

\[
\boxed{
\textbf{we keep the public object one-dimensional, but make the hidden coordinate categorical.}
}
\]

That is the cleanest path I see toward **ECC-sized public keys without ECC’s quantum-fatal hidden scalar coordinate**.

## 22. User

Make this concrete and actually propose an algorithm that can be implemented

## 23. Assistant

I’ll make this concrete as a **research prototype**, not a secure deployable cryptosystem. The goal will be an algorithm you could implement and experimentally attack, while keeping the categorical “hidden coordinate” idea intact.

## 24. Assistant

Yes. I would propose a concrete **research KEM/NIKE** whose public object is still essentially a single elliptic-curve \(j\)-invariant, but whose hidden coordinate is a **rank-2 module/isogeny coordinate**, not a scalar and not a class-group element.

Call the prototype:

**CTI-KEM: Coprime Tensor-Isogeny KEM**

It is **not deployable crypto**. It is an implementable candidate primitive for experiments and cryptanalysis.

The construction is directly inspired by Damien Robert’s module-action framework. His paper extends the usual ideal action on oriented elliptic curves to a Hermitian module action on oriented polarized abelian varieties, gives algorithms for computing that action, and proposes \(\otimes\)-MIKE, where parties publish only \(j\)-invariants and the common secret is a dimension-4 principally polarized abelian variety. Crucially, unlike SIDH, the public key does **not** include torsion images. citeturn248681view0

## Core object

We use:

\[
E_0/\mathbb F_{p^2}
\]

a fixed supersingular elliptic curve with known endomorphism structure.

A secret key is not a scalar \(k\). It is a smooth-degree isogeny/module coordinate:

\[
M_A
\]

derived from a secret isogeny path

\[
E_0 \to E_A.
\]

The public key is only:

\[
\operatorname{pk}_A = j(E_A)\in \mathbb F_{p^2}.
\]

So the public key can be roughly **one \(\mathbb F_{p^2}\) element**. If \(p\) is about 256 bits, that is about **64 bytes**, the same compactness regime targeted by recent isogeny work. Robert’s module-action proposal explicitly claims 64-byte NIST-level-1 public keys for its NIKE variant. citeturn248681view0

The shared secret is not an elliptic curve. It is a compact invariant of a dimension-4 principally polarized abelian variety:

\[
A_{AB}.
\]

The whole trick is:

\[
\textbf{public compression happens in dimension 1;}
\qquad
\textbf{security lives in the hidden module/action structure.}
\]

## Parameters

Choose:

\[
p = cN_A N_B - 1
\]

prime, with

\[
N_A=\ell_A^{e_A},\qquad N_B=\ell_B^{e_B},\qquad \gcd(N_A,N_B)=1.
\]

For a first implementation, take small toy values:

\[
\ell_A=2,\qquad \ell_B=3.
\]

Use real security parameters only after the construction survives toy attacks. The **coprime-degree** condition is deliberate. Robert’s ePrint page includes an update warning about insecurity if Alice and Bob do not use coprime isogeny degrees. citeturn248681view0

Pick a fixed supersingular base curve, for example a \(j=1728\)-type curve such as:

\[
E_0: y^2=x^3-x
\]

over a suitable field. Robert’s slides use this curve form in the concrete module-action setting and describe the relation between supersingular ideal action and oriented rank-2 module action. citeturn248681view2

Public parameters:

```text
params = {
    p,
    Fp2,
    E0,
    R = Z[π_p] or selected orientation order,
    N_A = ℓ_A^e_A,
    N_B = ℓ_B^e_B,
    hash functions H, KDF,
    ppav invariant encoding inv4
}
```

The implementation must maintain two internal representations of a secret:

```text
secret path:    E0 --φ--> E_secret
module form:    (M, H_M)
```

The path gives an easy way to compute the public endpoint. The module form is what we use to compute the shared action.

## Key generation

For role \(A\), use degree \(N_A\). For role \(B\), use degree \(N_B\).

```text
KeyGen(params, role):

    if role == A:
        N = N_A
        ℓ = ℓ_A
        e = e_A
    else:
        N = N_B
        ℓ = ℓ_B
        e = e_B

    E = E0
    path = []

    for i in 1..e:
        choose a random cyclic subgroup K_i ⊂ E[ℓ]
        avoid immediate backtracking
        compute the ℓ-isogeny φ_i : E → E/K_i
        append φ_i to path
        E = E/K_i

    E_secret = E

    M = ModuleFromPath(path, R)
    pk = CanonicalJ(E_secret)

    sk = {
        role,
        path,
        module = M,
        endpoint = E_secret
    }

    return pk, sk
```

The subroutine `ModuleFromPath` is the important algebraic bridge. In the ideal-theoretic implementation, one samples or reconstructs a smooth left ideal \(I\) in the relevant quaternionic order, then converts it to a rank-2 Hermitian \(R\)-module \((M_I,H_I)\). Robert’s slides state the key relation: if \(I\subset \mathcal O_0\) and \(E_I=I\cdot E_0\), then the associated rank-2 module action gives the Weil restriction of \(E_I\), and rank-2 module inversion is at least as hard as the supersingular isogeny path problem over \(\mathbb F_{p^2}\). citeturn239469view0

For a toy implementation, you can initially store both:

```text
path representation
quaternion/module representation
```

and verify that they produce the same public endpoint.

## Shared-secret derivation

Given my secret module \(M_A\) and your public key \(j(E_B)\), I reconstruct a curve model \(E_B\), form its oriented Weil-restriction object, apply my module action, then hash a canonical dimension-4 invariant.

```text
Derive(params, sk_self, pk_other):

    E_other = CurveFromJ(pk_other)

    W_other = OrientedWeilRestriction(E_other, R)

    A_shared = ModuleAction(sk_self.module, W_other)

    z = Inv4(A_shared)

    return H("CTI shared invariant" || z)
```

The expected correctness identity is:

\[
M_A\star W(E_B)
\cong
M_A\star (M_B\star E_0)
\cong
(M_A\otimes_R M_B)\star E_0
\cong
(M_B\otimes_R M_A)\star E_0
\cong
M_B\star W(E_A).
\]

So Alice computes:

\[
K_A = \operatorname{Hash}(\operatorname{Inv}_4(M_A\star W(E_B))),
\]

and Bob computes:

\[
K_B = \operatorname{Hash}(\operatorname{Inv}_4(M_B\star W(E_A))).
\]

If the module action and invariant are implemented correctly:

\[
K_A = K_B.
\]

That gives a NIKE-like primitive. A KEM is then immediate: one side uses an ephemeral role-\(A\) keypair and sends only the ephemeral \(j\)-invariant.

## KEM wrapper

```text
Encaps(params, pk_recipient):

    pk_eph, sk_eph = KeyGen(params, role=A)

    raw = Derive(params, sk_eph, pk_recipient)

    ct = pk_eph

    ss = KDF("CTI-KEM" || pk_recipient || ct || raw)

    return ct, ss
```

```text
Decaps(params, sk_recipient, ct):

    pk_eph = ct

    raw = Derive(params, sk_recipient, pk_eph)

    ss = KDF("CTI-KEM" || PublicKey(sk_recipient) || ct || raw)

    return ss
```

The ciphertext is one \(j\)-invariant. The recipient public key is one \(j\)-invariant. So this targets the same “tiny public object” regime as isogeny systems.

## The three required hard subroutines

The algorithm is implementable if you implement these three pieces.

### 1. Smooth isogeny path evaluation

This is standard.

For small \(\ell\), compute chains of \(\ell\)-isogenies using Vélu-style formulas or existing Sage/Magma isogeny routines.

```text
EvalPath(E0, path):
    E = E0
    for kernel description K_i in path:
        φ_i = IsogenyFromKernel(E, K_i)
        E = Codomain(φ_i)
    return E
```

This part is not the research bottleneck.

### 2. Module action on the oriented Weil restriction

This is the real algebraic engine.

Input:

```text
(M, H_M)      # secret Hermitian R-module
(A, λ, ι_R)   # oriented principally polarized abelian variety
```

Output:

```text
(M, H_M) ⋆ (A, λ, ι_R)
```

Robert’s slides give the computational shape: if

\[
\psi:(M_2,H_2)\hookrightarrow(M_1,H_1)
\]

is an \(N\)-similitude, then one obtains an \(N\)-isogeny of principally polarized abelian varieties whose kernel is

\[
\ker\phi
=
M_1/M_2\cdot A
=
A_1[M_2]
=
\{P\in A_1(k):m(P)=0\ \forall m\in M_2\}.
\]

That is concrete: represent the module inclusion, compute the corresponding torsion kernel inside the abelian variety, quotient by it, and transport the polarization. citeturn239469view1

Implementation route:

```text
ModuleAction(M, A):

    decompose M via one or two N-similitudes into a standard module R^g
    compute the kernel subgroup K = A[M_submodule]
    verify K is isotropic for the polarization pairing
    compute quotient abelian variety A' = A / K
    transport polarization and R-orientation
    return A'
```

For a first prototype, represent abelian varieties using **theta coordinates**. This is where existing high-dimensional isogeny code is more relevant than ordinary elliptic-curve code.

### 3. Dimension-4 invariant

This is the least settled part.

You need:

```text
Inv4(A_shared) → bytes
```

such that Alice and Bob get the same bytes from isomorphic shared ppavs.

For a prototype:

```text
Inv4(A):

    compute level-2 or level-4 theta null coordinates Θ(A)
    normalize projectively
    canonicalize sign/permutation ambiguities introduced by the implementation
    return hash_or_encoding(Θ_canonical)
```

For a serious candidate, this needs real modular invariant theory for dimension-4 ppavs. Robert’s slides explicitly flag this as an open practical need: “Need good dimension 4 modular invariants to represent \(A_{12}\),” e.g. symmetric polynomials in theta constants. citeturn248681view2

So this algorithm is implementable as a **research prototype** if we accept a deterministic theta-structure encoding. It is not yet a polished cryptosystem until `Inv4` is made robust, canonical, and audited.

## Why this is not SIDH

SIDH/SIKE published auxiliary torsion-point images. Those became the attack surface. The SIKE team’s own postscript says SIKE/SIDH are insecure, describes the practical classical key-recovery attacks, and states that no known SIDH variant retains roughly the same performance while remaining secure. citeturn330179view2

CTI-KEM publishes only:

```text
j(E_A)
j(E_B)
```

No torsion images. No basis images. No hidden-subgroup oracle handed to the attacker.

That does not prove security. It merely avoids the exact SIDH failure mode.

## Why this is not CSIDH either

CSIDH uses a commutative class-group action. That gives elegant tiny public keys, but the secret coordinate still lives in an abelian group-action world. Quantum analyses of CSIDH explicitly use Kuperberg-style hidden-shift attacks. citeturn187090search1

CTI-KEM tries to move from:

\[
\text{rank-1 invertible ideal/class-group action}
\]

to:

\[
\text{rank-2 Hermitian module action / higher-dimensional ppav action}.
\]

That is the whole point. If the construction collapses back to Picard-group data, it fails conceptually.

## Required audits

Before taking this seriously, attack these first:

**1. Picard collapse.**  
Check whether \(M\) is secretly determined by \(\det M\), a class-group element, or another abelian invariant. If yes, the scheme degenerates toward CSIDH-like hidden shift.

**2. Canonicalization attack.**  
Try to compute a canonical representative of the public endpoint or shared ppav. Any efficient canonical form is a coordinate-recovery machine.

**3. Module-inversion attack.**  
Given \(j(E_0)\) and \(j(E_A)\), try to recover a usable \((M_A,H_A)\). This is the analogue of private-key recovery.

**4. Parallelization attack.**  
Given only

\[
j(E_0),\quad j(E_A),\quad j(E_B),
\]

try to compute

\[
\operatorname{Inv}_4((M_A\otimes M_B)\star E_0)
\]

without either module. This is the CTI analogue of CDH.

**5. Invariant leakage.**  
Make sure `Inv4` does not expose enough theta-structure to reverse-engineer the module action.

## Minimal implementation plan

A practical prototype should be built in four stages.

**Stage 1: toy endpoint system.**  
Implement supersingular curve setup, random smooth isogeny paths, endpoint \(j\)-invariants, and path verification. Use tiny \(p\), e.g. 30–60 bits.

**Stage 2: ideal/module bookkeeping.**  
Maintain the quaternionic ideal or Hermitian module associated to each generated path. Verify Robert’s relation experimentally:

\[
(M_I,H_I)\star E_0 \simeq W(E_I).
\]

**Stage 3: module action on Weil restrictions.**  
Represent \(W(E)\) initially as \(E\times E^\sigma\) with descent/orientation metadata. Implement the kernel computation

\[
A[M_2]=\{P:m(P)=0\ \forall m\in M_2\}
\]

and quotient by it in theta coordinates.

**Stage 4: shared invariant.**  
Implement a deterministic theta-null encoding for dimension-4 ppavs. First require honest parties to match. Then harden toward a true isomorphism invariant.

## The algorithm in one block

```text
CTI-KEM / CTI-NIKE

Public parameters:
    p, Fp2, E0, R, N_A, N_B, Inv4, H, KDF

Secret key:
    smooth isogeny path E0 → E
    associated Hermitian rank-2 R-module M

Public key:
    j(E)

KeyGen(role):
    choose N_role-smooth random isogeny path φ: E0 → E
    compute module M_φ
    return pk = j(E), sk = (φ, M_φ)

Derive(sk_self, pk_other):
    E_other = CurveFromJ(pk_other)
    W_other = OrientedWeilRestriction(E_other)
    A = ModuleAction(sk_self.M, W_other)
    z = Inv4(A)
    return H(z)

Encaps(pk_R):
    pk_e, sk_e = KeyGen(role=A)
    raw = Derive(sk_e, pk_R)
    return ct = pk_e, ss = KDF(pk_R || ct || raw)

Decaps(sk_R, ct):
    raw = Derive(sk_R, ct)
    return ss = KDF(PublicKey(sk_R) || ct || raw)
```

The clean mathematical assumption is:

\[
\boxed{
\textbf{Module Action Parallelization is hard:}
}
\]

Given

\[
j(E_0),\quad j(M_A\star E_0),\quad j(M_B\star E_0),
\]

compute

\[
\operatorname{Inv}_4((M_A\otimes_R M_B)\star E_0).
\]

That is the actual analogue of CDH.

The candidate is attractive because the public object is tiny, the secret coordinate is not a scalar, and the public equality structure is not obviously an abelian hidden subgroup. The weak point is equally clear: **if the rank-2 Hermitian module data collapses to an abelian invariant, or if dimension-4 ppavs admit an efficient canonicalization/inversion route, the construction dies.**

## 25. User

What is needed to settle the dim 4 plans issue? Give it a try

## 26. Assistant

I’ll treat “dim 4 plans issue” as the **dimension-4 principally polarized abelian variety invariant problem** from the proposed CTI/module-isogeny construction: how Alice and Bob turn the shared fourfold into the same bytes without leaking a coordinate system.

## 27. Assistant

Yes. The **dimension-4 issue** is not “how do we compute the fourfold?” That is hard but increasingly concrete: dimension-4 isogeny algorithms now live in **theta models**, and recent work explicitly says that in dimension \(4\) and higher, one cannot rely on Jacobian models the way one can in dimensions \(2\) and \(3\). The open problem for the module-action scheme is narrower and sharper: **given the shared principally polarized abelian fourfold \(A_{12}\), how do both parties extract the same canonical bytes without publishing or relying on a hidden level structure?** Robert’s slides state the missing piece directly: “Need good dimension 4 modular invariants to represent \(A_{12}\),” suggesting symmetric polynomials in theta constants. citeturn370818view0turn370818view1

Here is the concrete attempt.

Use **level-one theta-orbit invariants** of a dimension-4 principally polarized abelian variety. In genus \(4\), there are

\[
2^{g-1}(2^g+1)=136
\]

even theta characteristics. For each even characteristic \(m\), define

\[
t_m=\theta_m(0,A)^8.
\]

The eighth power is important. Under a change of symplectic level structure, the \(t_m\)’s transform by a **common scalar** and a **permutation** of the 136 even characteristics:

\[
t_m \mapsto \lambda\, t_{\gamma m}.
\]

This transformation law is exactly why invariant polynomials in the \(t_m\)’s can descend from the level cover to honest modular invariants on \(\mathcal A_4\). Atyam summarizes the theta transformation law and notes that polynomials invariant under the affine symplectic action on characteristics define modular forms for the full modular group. citeturn370818view2turn336493view1

So define the shared-secret invariant as follows.

Let

\[
E_4=\{m\in(\mathbb F_2)^8:q(m)=0\}
\]

be the set of even characteristics. Compute the vector

\[
T(A)=(t_m)_{m\in E_4}.
\]

Then do **not** hash \(T(A)\) directly. Raw theta coordinates still depend on the hidden level structure. Instead compute invariant polynomial sums.

The simplest family is:

\[
P_d(A)=\sum_{m\in E_4} t_m^d.
\]

Because \(t_m\mapsto \lambda t_{\gamma m}\), we get

\[
P_d\mapsto \lambda^d P_d.
\]

Therefore the ratios

\[
I_d(A)=\frac{P_d(A)}{P_1(A)^d}
\]

are invariant under both scaling and symplectic relabeling, whenever \(P_1\neq 0\). This already gives a crude absolute invariant.

But I would not trust only power sums. Power sums treat the 136 theta constants as an unordered multiset, which quotients by too much symmetry. The correct strengthening is to use **orbit sums over tuples of theta characteristics**.

For each orbit

\[
O\subset E_4^r
\]

under the affine action of

\[
\operatorname{Sp}(8,\mathbb F_2),
\]

define

\[
F_O(A)=\sum_{(m_1,\ldots,m_r)\in O} t_{m_1}\cdots t_{m_r}.
\]

Again,

\[
F_O\mapsto \lambda^r F_O.
\]

So if \(F_B\) is a nonzero base invariant of degree \(b\), define absolute ratios

\[
J_O(A)=\frac{F_O(A)^b}{F_B(A)^r}.
\]

Those are true level-one invariants of the ppav. The orbit classification is not ad hoc: Igusa/Salvati Manni classify tuple orbits of even theta characteristics under \(\operatorname{Sp}(2g,\mathbb Z/2\mathbb Z)\), and Atyam uses exactly this orbit machinery to construct modular forms on \(\mathcal A_4\). citeturn336493view0turn336493view2

So my proposed `Inv4` is:

```text
Inv4(A_theta):

    Input:
        A dimension-4 ppav represented in a theta model.

    1. Compute the 136 values
           t_m = theta_m(0,A)^8
       for all even characteristics m.

    2. Compute a fixed list of invariant polynomial forms:
           P_d = sum_m t_m^d
           F_O = sum_{(m1,...,mr) in O} t_m1 ... t_mr
       where O ranges over precomputed Sp(8,F2)-orbits of pairs,
       triples, and selected quadruples of even theta characteristics.

    3. Choose the first nonzero base form F_B from a fixed public list.

    4. Normalize all forms by weighted projective ratios:
           J_i = F_i^{deg(F_B)} / F_B^{deg(F_i)}

    5. Canonically encode the resulting finite-field vector.
       If necessary, identify Frobenius conjugates by taking the
       lexicographically smaller encoding of v and v^p.

    6. Return Hash("CTI-Inv4-v0" || encoded_vector).
```

This is implementable. The main precomputation is the finite orbit table for \(\operatorname{Sp}(8,\mathbb F_2)\) acting on tuples of even theta characteristics. That table is public and fixed for genus \(4\). The online computation is just finite-field arithmetic and summation.

For a first version, I would use the following invariant list:

```text
Degree 1:
    P1 = Σ_m t_m

Degree 2:
    P2 = Σ_m t_m^2
    Q0 = Σ_{m<n, q(m+n)=0} t_m t_n
    Q1 = Σ_{m<n, q(m+n)=1} t_m t_n

Degree 3:
    all Sp(8,F2)-orbit sums on unordered triples of distinct even characteristics

Degree 4:
    selected orbit sums on quadruples, especially those detecting
    syzygetic/azygetic configurations and decomposable-locus behavior
```

I would also include the genus-4 Schottky form as a diagnostic:

\[
F_T
=
16\sum_{m\in E_4}\theta_m^{16}
-
\sum_{m\in E_4}(\theta_m^8)^2.
\]

Atyam cites Igusa’s result that the zero locus of this Schottky form is the Jacobian locus \(J_4\subset\mathcal A_4\). That is useful because if the shared \(A_{12}\) lands systematically in a special locus—Jacobian, decomposable, hyperelliptic, product locus—then the security analysis must use that smaller moduli problem, not generic \(\mathcal A_4\). citeturn336493view0

What would be needed to **settle** this properly?

First, define the actual shared-secret locus

\[
S_{\otimes}\subset \mathcal A_4
\]

reached by the module-action construction. Do not solve all of \(\mathcal A_4\) if \(A_{12}\) lives in a much smaller special subvariety. The right invariant only has to separate generic points of \(S_{\otimes}\), not arbitrary ppavs.

Second, prove or experimentally validate that the chosen invariant map

\[
A\mapsto (J_1(A),\ldots,J_N(A))
\]

is **generically separating** on \(S_{\otimes}\). In algebraic terms, the selected modular functions should generate, or at least separate a large subfield of, the function field

\[
k(S_{\otimes}).
\]

Third, test the bad divisors: \(P_1=0\), theta-null vanishing, decomposable locus, Jacobian locus, Frobenius/twist ambiguity. The invariant algorithm must have deterministic fallback bases so it never randomly fails on honest shared secrets.

Fourth, verify non-leakage. The invariant must not reveal a theta structure, torsion basis, module coordinate, or decomposition of \(A_{12}\). This is why hashing raw theta nulls is conceptually wrong: a theta null point is a modular invariant **with level structure**, not automatically a level-one moduli invariant.

My guess: **this theta-orbit invariant approach is the right first solution**. It may not be minimal, and it may not be a clean closed-form “Igusa invariant for genus 4,” but it is principled, computable, and directly aligned with the symmetry that must be quotiented out.

The concise version:

**Represent \(A_{12}\) internally in a level-2 or level-4 theta model, compute the 136 eighth powers of even theta constants, throw away the level structure by evaluating \(\operatorname{Sp}(8,\mathbb F_2)\)-orbit-sum modular forms, normalize by weighted ratios, and hash the resulting level-one invariant vector.**

That is the concrete `Inv4` I would try first.

## 28. User

it it possible that no backdoor exists to this system?

## 29. Assistant

**Yes, it is possible that no backdoor exists.** But that statement is much weaker than we want. In frontier cryptography, “no backdoor exists” is usually not something you can prove absolutely. What you can hope to prove is narrower:

\[
\textbf{any efficient backdoor would imply an efficient algorithm for a known hard problem.}
\]

For the module-isogeny direction, the right formalization is this.

Let

\[
U:\mathcal C \to \mathcal M
\]

be the public-forgetting map from hidden coordinate objects to public moduli objects. A secret is a coordinate/morphism/module \(M\in\mathcal C\). The public key is

\[
U(M)=j(E_M).
\]

A **backdoor** is an efficient functorial way to recover enough hidden structure from \(U(M)\) to either invert the action or compute the shared object:

\[
j(E_0),j(E_A),j(E_B)
\mapsto
\operatorname{Inv}_4(A_{AB}).
\]

So the scary object is not necessarily a private-key recovery algorithm. It could be a weaker **parallelization backdoor**: an algorithm that computes the shared invariant without ever recovering Alice’s or Bob’s secret coordinate.

That matters because for Diffie–Hellman-style systems, the analogue of CDH can be easier than full discrete log in principle.

For the specific module-action proposal we discussed, Damien Robert’s paper claims the framework extends rank-1 ideal actions to Hermitian module actions, gives algorithms to compute the action, and says rank-2 module action inversion is at least as hard as the supersingular isogeny path problem. It also explicitly says the proposed \(\otimes\)-MIKE protocol publishes only \(j\)-invariants, no torsion information, and produces a dimension-4 principally polarized abelian variety as common secret. But the same abstract says Kuperberg’s algorithm **presumably** does not apply, and flags an insecurity update when Alice and Bob do not use coprime isogeny degrees. That is promising, but it is not a no-backdoor theorem. citeturn309869view0

The reason “no backdoor” is hard to prove is that a backdoor may be a completely unexpected functor:

\[
F:\mathcal M \to \mathcal A
\]

from the public moduli problem into some easier algebraic category \(\mathcal A\). Maybe \(\mathcal A\) is a class group, a lattice, a representation ring, a theta-coordinate normal form, an endomorphism algebra, a low-dimensional cohomology theory, or a hidden product decomposition. Once such an \(F\) exists, the public object may leak a coordinate that the original designers were not thinking about.

That is basically what happened culturally with SIDH/SIKE. It was not quantumly broken; it was broken by a classical key-recovery attack exploiting extra structure. The SIKE team’s note says SIKE/SIDH are insecure and should not be used, and describes the Castryck–Decru attack as a practical classical key-recovery attack; attempted modifications avoiding the attack severely worsened performance and key size. citeturn107727search0

So the right answer is:

**Yes, there may be no backdoor. But the default research posture should be that some hidden functor, descent, canonicalization, abelianization, or auxiliary-structure attack probably exists until the scheme survives deliberate attempts to find one.**

For our candidate, the **good news** is that the obvious Shor/Kuperberg-style attack surface is weaker than ECC/CSIDH. ECC exposes an abelian hidden subgroup problem. CSIDH exposes a commutative group-action / hidden-shift-like structure. In contrast, the module-action scheme is trying to make the hidden coordinate categorical rather than a rank-1 abelian class-group coordinate. The nonabelian hidden-subgroup landscape is much less solved: recent HSP surveys still describe the abelian case as efficiently solved while the nonabelian case has no comparable general efficient quantum solution. citeturn472053search3

But the **bad news** is that “nonabelian” is not a magic shield. Many apparently nonabelian problems have large abelian shadows. The real test is not whether the construction uses noncommutative algebra somewhere. The test is whether the public object leaks through one of these maps:

\[
M \mapsto \det M,
\]

\[
M \mapsto [M]\in K_0(R),
\]

\[
M \mapsto \operatorname{Pic}(R),
\]

\[
M \mapsto \operatorname{End}(A_M),
\]

\[
M \mapsto \text{theta normal form},
\]

\[
M \mapsto \text{decomposition type of }A_M,
\]

\[
M \mapsto \text{low-dimensional representation character}.
\]

If any of those is efficiently computable and sufficiently informative, the system may have a backdoor even though the original coordinate category looked hard.

The deepest version is this:

\[
\boxed{
\textbf{A backdoor is an efficient section of a quotient, or an efficient descent to an easier quotient.}
}
\]

So to make “no backdoor” credible, we need to show that the public forgetful map

\[
U:\mathcal C\to\mathcal M
\]

has no efficient section and no dangerous low-complexity quotient.

For the CTI/module-isogeny system, I would divide the no-backdoor program into five claims.

**Claim 1: no abelian collapse.**  
The public \(j(E_M)\) and the shared invariant must not be determined by \(\det(M)\), a Picard-class component, or any other class-group-like invariant. If this fails, the system degenerates back toward CSIDH-style hidden shift.

**Claim 2: no canonicalization.**  
There should be no efficient canonical form for the public endpoint or the dimension-4 shared ppav that reconstructs the hidden module coordinate. A canonicalizer is a coordinate system. If you can compute a unique normal representative, you have probably found the missing coordinate.

**Claim 3: no auxiliary-level leakage.**  
The dimension-4 invariant must throw away theta level structure. Hashing raw theta nulls would be dangerous unless the level structure is fixed identically and harmless. The invariant should be an orbit-sum / modular invariant, not a coordinate chart.

**Claim 4: no product-locus shortcut.**  
The shared fourfold must not land in an easily decomposable locus where

\[
A_{AB}\cong E_1\times E_2\times E_3\times E_4
\]

or some structured product whose factors reveal the path. Dimension four is attractive, but it also gives attackers more subvarieties to exploit.

**Claim 5: no parallelization oracle.**  
Even if recovering \(M_A\) is hard, there must be no direct algorithm

\[
(j(E_A),j(E_B))\mapsto \operatorname{Inv}_4(A_{AB}).
\]

This is the real key-exchange assumption.

So, is it possible no backdoor exists? **Yes.** The strongest reason for optimism is that the scheme avoids the three known fatal patterns: it is not an exact scalar discrete log, it is not obviously a commutative hidden-shift action, and it does not publish SIDH-style torsion images. The strongest reason for pessimism is that compact algebraic moduli objects are dense with accidental structure. Small keys are purchased with structure, and structure is exactly where backdoors live.

My current belief would be:

**A no-backdoor version is plausible only if the secret entropy genuinely lives in the non-invertible/rank-2 module coordinate, while all rank-1/Picard/determinant shadows are fixed, masked, or useless.**

If the rank-2 coordinate is merely decorative and the protocol’s actual behavior factors through an abelian invariant, it is dead.

The research target should therefore be a theorem of the following shape:

\[
\boxed{
\text{Any algorithm computing } \operatorname{Inv}_4(A_{AB})
\text{ from } j(E_A),j(E_B)
\text{ induces an algorithm for supersingular isogeny path / endomorphism-ring recovery.}
}
\]

That would not prove absolute security, but it would be the first serious “no hidden coordinate shortcut” result.

Without a reduction like that, the honest answer is:

**It may have no backdoor. But we should assume it does until we fail, very hard, to construct one.**

## 30. User

I am not comfortable with this approach. A black hat could find an exploit that no white had found. We should climb back up our abstraction ladder and formulate what we need to construct with provably no backdoors as a hard constraint. We don't need to give the proof immediately but we need to formulate the problem correctly

## 31. Assistant

Yes. The discomfort is correct. “We don’t know an attack” is the wrong standard. We need distinguish **cryptographic hardness** from **backdoor absence**, then build the latter into the problem statement instead of hoping audits catch it.

## 32. Assistant

Yes. We should climb back up. The previous direction was **cryptanalytic exploration**. What you are asking for now is a **specification discipline**: define the object so that “no backdoor” is not an empirical hope but a theorem target.

The first correction is severe:

**“No known attack” is not a security property.**  
**“No backdoor” must mean: every efficient way to exploit the public representation, including one using hidden auxiliary knowledge, reduces to solving a public, explicitly stated hard problem.**

This is the right abstraction.

## 1. Separate three meanings of “backdoor”

There are three different failures people conflate.

**Planted setup backdoor.** The designer chooses public parameters with hidden trapdoor information. Dual EC DRBG is the canonical cautionary example: the suspicious part was not elliptic curves per se, but unexplained constants that could encode a secret relation. Reports at the time emphasized that knowledge of a hidden relation between the standard’s public points could allow prediction of the generator output. citeturn414564news34turn414564news36

**Kleptographic implementation backdoor.** The algorithm implementation is maliciously modified to leak secrets while appearing normal. This is the domain Young and Yung called **kleptography**; later work on algorithm-substitution attacks formalizes the same general concern: an adversary secretly replaces a cryptosystem’s implementation with a malicious one designed to leak key material. citeturn602151search0turn414564search0

**Structural mathematical backdoor.** There is an efficient transformation, functor, invariant, descent, canonicalization, or hidden coordinate system that breaks the scheme. Nobody planted it; it was simply there.

Your concern is mostly the third one, but a sane construction must handle all three.

## 2. The right object is not “a crypto scheme”; it is a transparent quotient problem

We should formulate a **Transparent Coordinate-Hiding Problem**.

For each security parameter \(\lambda\), construct:

\[
\mathcal C_\lambda
\xrightarrow{\;U_\lambda\;}
\mathcal P_\lambda
\]

where:

\[
\mathcal C_\lambda = \text{coordinatized/secret objects}
\]

\[
\mathcal P_\lambda = \text{public uncoordinatized objects}
\]

\[
U_\lambda = \text{forgetful/publication functor}.
\]

A secret key is a coordinate, morphism, path, basis, module, factorization, or trivialization:

\[
c\in \mathcal C_\lambda.
\]

The public key is:

\[
p=U_\lambda(c)\in \mathcal P_\lambda.
\]

The intended security claim is not “this looks hard.” It is:

\[
\boxed{
\text{computing a useful section of }U_\lambda
\text{ is equivalent to solving a public hard problem.}
}
\]

A **backdoor** is precisely an efficient section, partial section, or shortcut descent.

More formally, a backdoor is an auxiliary string \(\tau_\lambda\) and an efficient algorithm \(B\) such that:

\[
B(\tau_\lambda, U_\lambda(c)) \mapsto \text{useful hidden coordinate information}
\]

or, for key exchange,

\[
B(\tau_\lambda, U_\lambda(c_A), U_\lambda(c_B))
\mapsto K(c_A,c_B),
\]

with non-negligible probability.

So the no-backdoor target is:

\[
\boxed{
\forall\ \text{poly-size auxiliary advice }\tau_\lambda,\ 
\forall\ \text{QPT algorithms }B,\ 
B \text{ succeeds only negligibly unless it solves }H_\lambda.
}
\]

Here \(H_\lambda\) is a **publicly specified hardness problem**, not a designer-controlled mystery.

This is the key move: **a black-hat exploit is just a nonuniform adversary with advice**. If your theorem quantifies over nonuniform quantum polynomial-time adversaries, it already includes “someone found a trick and wrote it down.”

## 3. The public setup must be trapdoorless by construction

The setup cannot be:

\[
\text{designer chooses clever object }X.
\]

It must be:

\[
pp_\lambda = \operatorname{Setup}(1^\lambda, \rho)
\]

where \(\rho\) is public randomness, and `Setup` is deterministic, audited, minimal, and reproducible.

Even better:

\[
pp_\lambda = \operatorname{Setup}(1^\lambda)
\]

with no parameter randomness at all.

If public randomness is unavoidable, then the parameter-generation transcript must be part of the public object:

\[
(pp_\lambda,\rho,\pi)
\]

where \(\pi\) proves:

\[
pp_\lambda = \operatorname{Setup}(1^\lambda,\rho).
\]

But that only prevents a planted secret. It does **not** prove structural safety. For structural safety, we need a reduction.

The correct parameter condition is:

\[
\boxed{
\text{For every accepted }pp,\text{ or for }pp\text{ generated from public randomness except negligible bad set,}
\text{ security reduces to }H_\lambda.
}
\]

A construction that is secure only because “we hope the chosen instance is not special” is not acceptable under your standard.

## 4. The theorem target

The problem should be formulated around two reductions.

### Coordinate inversion reduction

Define the inversion game:

\[
p = U_\lambda(c).
\]

Given \(p\), recover any \(c'\) such that:

\[
U_\lambda(c') = p
\]

or such that \(c'\) is operationally equivalent to \(c\).

The theorem target:

\[
\boxed{
\textbf{If an efficient quantum algorithm inverts }U_\lambda,
\textbf{ then it solves }H_\lambda.
}
\]

This handles private-key recovery.

### Parallelization reduction

For key exchange, inversion is not enough. The attacker may compute the shared secret without recovering either private key.

So define:

\[
K(c_A,c_B)
\]

as the shared invariant.

The attacker sees:

\[
U(c_A),\quad U(c_B)
\]

and tries to compute:

\[
K(c_A,c_B).
\]

The theorem target:

\[
\boxed{
\textbf{If an efficient quantum algorithm computes }K(c_A,c_B)
\textbf{ from }U(c_A),U(c_B),
\textbf{ then it solves }H_\lambda.
}
\]

This is the true analogue of CDH, but abstracted away from groups.

## 5. What the hard problem \(H_\lambda\) must look like

The public hard problem cannot itself contain hidden setup. It should be **canonical**, **random-self-reducible if possible**, and **not instance-fragile**.

Bad:

\[
H_\lambda = \text{“break this particular curve/object we chose.”}
\]

Better:

\[
H_\lambda = \text{“solve a worst-case or average-case problem over a publicly sampled family.”}
\]

Best:

\[
\boxed{
\text{average-case security of the scheme reduces to worst-case hardness of a natural problem.}
}
\]

This is one reason LWE-style schemes are philosophically strong despite looking ugly: Regev’s LWE result connects average-case LWE hardness to worst-case lattice problems, giving a reductional story rather than merely an aesthetic one. citeturn602151search4

For our abstraction, the target is:

\[
\boxed{
\text{Construct }U_\lambda:\mathcal C_\lambda\to\mathcal P_\lambda
\text{ such that breaking coordinate hiding is reducible from a canonical hard problem.}
}
\]

Not:

\[
\text{Pick an exotic moduli object and hope.}
\]

## 6. Category-theoretic formulation

Let:

\[
U:\mathcal C\to \mathcal P
\]

be a functor.

The fiber over a public object \(p\) is:

\[
\mathcal C_p = U^{-1}(p).
\]

A hidden coordinate system is a choice of object in this fiber.

A backdoor is an efficient procedure that constructs one of the following:

### A section

\[
S:\mathcal P\to \mathcal C
\]

with

\[
U\circ S \cong \operatorname{id}_{\mathcal P}.
\]

This is a full coordinate-recovery backdoor.

### A partial section on challenge distribution

\[
S:\operatorname{Supp}(\mathcal D_\lambda)\subseteq \mathcal P\to \mathcal C
\]

This is enough to break the real scheme.

### A descent to an easier quotient

\[
F:\mathcal P\to \mathcal Q
\]

where \(\mathcal Q\) has tractable coordinates, and \(F(U(c))\) preserves enough of \(c\) to compute secrets.

This is subtler and more dangerous. Most real cryptanalytic breakthroughs look like this: not “recover the whole secret,” but “map the public object into a domain where the relevant secret shadow is easy.”

So the no-backdoor property should say:

\[
\boxed{
\text{Every efficient section or useful descent from }\mathcal P
\text{ yields an efficient solver for }H_\lambda.
}
\]

That is the categorical version of the theorem.

## 7. The hard constraint: no hidden setup, no special instance, no unexplained constants

The object we need to construct must satisfy:

\[
\boxed{
\textbf{Transparent generation.}
}
\]

No secret parameter selection. No unexplained constants. No “we sampled until it looked good.” No designer-chosen curve. No private ceremony.

\[
\boxed{
\textbf{Public verifiability.}
}
\]

Anyone can verify that parameters came from the specified generator.

\[
\boxed{
\textbf{Instance robustness.}
}
\]

Security cannot depend on a secret fact about the chosen instance.

\[
\boxed{
\textbf{Nonuniform quantum security.}
}
\]

The adversary may have arbitrary polynomial-size advice. That advice represents the black-hat’s unpublished exploit.

\[
\boxed{
\textbf{Reductional backdoor exclusion.}
}
\]

Any successful shortcut implies a solver for the canonical hard problem.

These five together are the right “no backdoor” formulation.

## 8. Candidate formal definition

Call the object a **Trapdoorless Coordinate-Hiding KEM**.

A family

\[
\Pi_\lambda=(\operatorname{Setup},\operatorname{KeyGen},\operatorname{Pub},\operatorname{Combine},\operatorname{Encaps},\operatorname{Decaps})
\]

is trapdoorless relative to a hard problem family \(H_\lambda\) if:

### Transparent setup

\[
pp_\lambda = \operatorname{Setup}(1^\lambda,\rho)
\]

is deterministic from public \(\rho\), and there is a polynomial-time verifier:

\[
\operatorname{VerifySetup}(1^\lambda,\rho,pp_\lambda)=1.
\]

### Coordinate hiding

For every nonuniform quantum polynomial-time adversary \(A=\{A_\lambda,z_\lambda\}\),

\[
\Pr[A_\lambda(z_\lambda,pp,U(c))\to c']\leq \operatorname{negl}(\lambda),
\]

unless \(A\) implies a solver for \(H_\lambda\).

### Parallelization hiding

For every nonuniform QPT adversary \(A\),

\[
\Pr[
A(z_\lambda,pp,U(c_A),U(c_B))=K(c_A,c_B)
]
\leq \operatorname{negl}(\lambda),
\]

unless \(A\) implies a solver for \(H_\lambda\).

### Parameter-subversion resistance

For every QPT adversary \(G\) that outputs:

\[
(pp,\tau)
\]

such that:

\[
\operatorname{VerifySetup}(pp)=1,
\]

the advantage of \(\tau\) in the above games is negligible, unless \(G\) produces a solver for \(H_\lambda\).

This last condition is the planted-backdoor killer. It says even if someone gives you accepted parameters plus “secret sauce,” the sauce should not help.

### Implementation subversion boundary

This cannot be solved by math alone. But the primitive should be **deterministic wherever possible**, use **explicit randomness derivation**, and support **test-vector equivalence** so implementation deviations are detectable. That addresses kleptographic substitutions; algorithm-substitution attacks are a known formal threat model, so implementation determinism and reproducibility are not optional engineering details. citeturn414564search0

## 9. What this rules out

This formulation rejects many seductive constructions.

It rejects:

\[
\text{“Pick a special curve/object and hope no one knows its trapdoor.”}
\]

It rejects:

\[
\text{“Use a random-looking algebraic object with unexplained constants.”}
\]

It rejects:

\[
\text{“Security follows from the fact that no attack is published.”}
\]

It rejects:

\[
\text{“The object is nonabelian, therefore safe.”}
\]

It also rejects our earlier module-isogeny candidate **unless** we can supply the reduction:

\[
\text{parallelization attack}
\Rightarrow
\text{canonical hard-problem solver}.
\]

Without that theorem, it remains an interesting construction, not a backdoor-free construction.

## 10. The problem statement we should now work on

Here is the clean research problem.

\[
\boxed{
\begin{aligned}
\textbf{Problem: Transparent Trapdoorless Coordinate Hiding.}\\
\text{Construct a public-coin family of forgetful functors }
U_\lambda:\mathcal C_\lambda\to\mathcal P_\lambda
\text{ with compact encodings,}\\
\text{efficient forward evaluation, and a combination operation }K(c_A,c_B),\\
\text{such that any nonuniform quantum polynomial-time algorithm that}\\
\text{either inverts }U_\lambda\text{ on sampled instances or computes }K(c_A,c_B)\\
\text{from }U(c_A),U(c_B)\text{ yields a solver for a canonical public hard problem }H_\lambda.\\
\text{The setup must be deterministic from public randomness and parameter-subversion resistant.}
\end{aligned}
}
\]

This is the formulation.

It moves us from:

\[
\text{“Find a clever object.”}
\]

to:

\[
\text{“Find a quotient whose useful sections are provably equivalent to a canonical hard problem.”}
\]

That is the right abstraction if **provably no backdoors** is a hard constraint.

The next design question becomes: **what canonical hard problem \(H_\lambda\) is suitable?** Not “which algebraic toy is beautiful,” but which \(H_\lambda\) can plausibly support compact encodings, transparent setup, nonuniform quantum security, and a reduction from every useful coordinate shortcut.

## 33. User

perfect. I agree that identifying the canonical hard problem is the right way to go. Lets approach this from first principles

## 34. Assistant

We should now separate **requirements forced by public-key functionality** from **assumptions we merely like**. I’ll derive the candidate hard-problem shape first, then rank existing and new mathematical families against that shape.

## 35. Assistant

Good. Start from the functional constraints.

A public-key system needs an **easy forward map** and a **hard reverse/parallelization problem**. A backdoor-free public-key system needs something stronger: the forward map must be sampled from a **transparent public distribution**, and any efficient shortcut must imply an efficient algorithm for a **public canonical hard problem**. That pushes us away from “special algebraic object with mysterious structure” and toward **average-case hardness with a worst-case reduction**. This is exactly why LWE is philosophically strong: Regev’s LWE result gives a reduction from worst-case lattice problems such as GapSVP/SIVP to a random learning/decoding problem, so breaking the average-case public instances implies solving worst-case lattice problems. citeturn868141search2turn940200search1

The first-principles shape of the hard problem should be this:

\[
\boxed{
\textbf{recover a hidden coordinate from a public random measurement corrupted by bounded uncertainty.}
}
\]

Not “add noise because noise is fashionable.” More precisely:

\[
y = A s + e.
\]

Here \(A\) is a **public random morphism**, \(s\) is the hidden coordinate, and \(e\) is a bounded perturbation. If \(e=0\), this is just linear algebra. If \(A\) is replaced by an exact group action, quantum Fourier/HSP machinery may see the symmetry. The perturbation is doing one specific conceptual job:

\[
\boxed{
\textbf{it destroys exact quotient fibers.}
}
\]

The public object is no longer an exact coset, orbit, kernel, or stabilizer. It is an **approximate section problem**.

## The canonical abstraction

Let \(\mathcal A_\lambda\) be a finite, publicly described **additive/normed category**. Think finite modules, codes, lattices modulo \(q\), chain complexes, sheaves over finite spaces, etc. We need addition so the forward map is efficient; we need a norm/weight so “small error” is meaningful.

Choose objects \(S,T\in\mathcal A_\lambda\). Sample a public morphism

\[
A:S\to T
\]

from a transparent distribution. Sample a secret coordinate

\[
s\leftarrow S
\]

and a small error

\[
e\leftarrow \chi_T.
\]

Publish

\[
(A,y),\qquad y=A s+e.
\]

Define the hard problem:

\[
\boxed{
\textbf{Noisy Section Problem}_{\mathcal A}:
\text{ given }(A,y),\text{ recover }s\text{ or distinguish }y=A s+e\text{ from uniform.}
}
\]

This is the clean generalization. It includes LWE, module-LWE, syndrome decoding, rank-syndrome decoding, and potentially sheaf/cochain variants.

The associated **no-backdoor theorem target** is:

\[
\boxed{
\begin{aligned}
&\text{For every nonuniform quantum polynomial-time adversary }B,\\
&\text{if }B\text{ solves random Noisy Section instances over }\mathcal A_\lambda,\\
&\text{then }B\text{ yields an algorithm for a canonical worst-case problem }H_{\mathcal A,\lambda}.
\end{aligned}
}
\]

That \(H_{\mathcal A,\lambda}\) might be shortest vector, nearest vector, bounded-distance decoding, minimum distance, rank decoding, nearest cocycle, or some analogous “short representative” problem in the category.

This is the real object we need to construct.

## Why average-case is mandatory

Worst-case hardness alone is not enough. Cryptography samples public keys. So the adversary attacks **typical instances**, not adversarially chosen hard instances. The hard problem therefore has to be distributional:

\[
\text{random instance hard on average}
\]

and ideally supported by:

\[
\text{average-case solver} \Rightarrow \text{worst-case solver}.
\]

That is the anti-backdoor shape. A hidden exploit is just a nonuniform average-case algorithm with advice. If the theorem quantifies over nonuniform quantum algorithms, then “a black hat found a clever trick” is already included in the adversary model.

This is also why plain “NP-hard” is the wrong target. Modern cryptography rests on average-case hardness; Goldreich puts one-way functions at the foundation and emphasizes that cryptographic hardness is average-case, not merely worst-case. citeturn868141search5

## Existing instances of the abstraction

The abstraction immediately yields a hierarchy.

| Hard problem family | Abstraction | No-backdoor quality | Compactness |
|---|---|---:|---:|
| **Plain LWE** | noisy section over \(\mathbb Z_q^n\) | strongest reductional story | bulky unless public matrix is seed-generated |
| **Module-LWE** | noisy section over modules \(R_q^k\) | strong but structured | practical |
| **Code decoding / syndrome decoding** | noisy section with Hamming weight | transparent, algebraically diverse | usually larger keys |
| **Rank-metric decoding** | noisy section with rank weight | compact-ish, but subtle | moderate |
| **Isogeny/groupoid path problems** | hidden coordinate in moduli | compact | currently weak no-backdoor proof story |

Plain LWE is the cleanest theoretical answer. Module-LWE is the practical compromise. NIST’s ML-KEM standard is explicitly based on Module-LWE and is currently the primary standardized KEM family; NIST says ML-KEM is believed secure even against adversaries with large-scale quantum computers. citeturn940200search5 Langlois and Stehlé’s module-lattice reductions are exactly the kind of proof architecture one wants when compressing from plain LWE toward structured modules. citeturn868141search3

Code-based cryptography gives useful diversity. NIST selected HQC as a backup KEM specifically because it is based on different mathematics from ML-KEM, which is valuable if lattice assumptions are weakened. citeturn451013search0

So, from first principles, the current best canonical hard problem is not “elliptic curves but weirder.” It is:

\[
\boxed{
\textbf{Noisy random section over a transparent additive category, with worst-case-to-average-case reduction.}
}
\]

LWE is the best-known instantiation. The research frontier is to find a category \(\mathcal A\) that gives **more compact encodings** without losing the reductional/no-backdoor property.

## Why exact categorical coordinate hiding is dangerous

If we use an exact forgetful functor

\[
U:\mathcal C\to\mathcal P,
\]

then the fiber over \(p\) is an exact groupoid:

\[
\mathcal C_p=U^{-1}(p).
\]

If that fiber is a torsor under an abelian group, Shor/Kitaev-style methods are a direct threat. If it is a commutative group action, hidden-shift algorithms are a threat. If it is nonabelian, we may be safer, but we usually lack the reductional theorem. That is not acceptable under the “provably no backdoors” constraint.

So the abstraction should not be an exact quotient stack unless we can prove a worst-case reduction. Instead, use a **stochastic/approximate quotient**:

\[
K:\mathcal C \rightsquigarrow \mathcal P,
\]

where \(K(c)\) samples a public object near the image of \(c\). In concrete algebra:

\[
K(s)=A s+e.
\]

That is the categorical meaning of noise: **replace exact fibers by smeared fibers** so there is no exact symmetry object for Fourier sampling or canonicalization to recover.

## The candidate hard problem we should define

I would name it:

\[
\boxed{
\textbf{Transparent Noisy Section Problem, TNSP}_{\mathcal A}.
}
\]

A family \(\mathrm{TNSP}_{\mathcal A,\lambda}\) consists of:

\[
\mathcal A_\lambda:
\text{ publicly specified finite additive/normed category}
\]

\[
S_\lambda,T_\lambda\in\mathcal A_\lambda
\]

\[
A\leftarrow \operatorname{Hom}_{\mathcal A}(S_\lambda,T_\lambda)
\]

\[
s\leftarrow S_\lambda,\qquad e\leftarrow \chi_{T_\lambda}
\]

\[
y=A s+e.
\]

Search version:

\[
(A,y)\mapsto s'
\quad\text{such that}\quad
y-A s'\text{ is small.}
\]

Decision version:

\[
(A,A s+e)\quad\text{vs.}\quad(A,u)
\]

where \(u\leftarrow T_\lambda\) is uniform.

The theorem target:

\[
\boxed{
\text{Decision/Search TNSP}_{\mathcal A}
\text{ on average}
\;\Longrightarrow\;
\text{worst-case }H_{\mathcal A}.
}
\]

The KEM theorem target:

\[
\boxed{
\text{IND-CCA break of the KEM}
\;\Longrightarrow\;
\text{Decision/Search TNSP}_{\mathcal A}
\;\Longrightarrow\;
H_{\mathcal A}.
}
\]

And the no-backdoor theorem target:

\[
\boxed{
\text{Any nonuniform QPT shortcut, even with advice, gives a solver for }H_{\mathcal A}.
}
\]

That is the formal problem.

## The compactness frontier

Now the whole project becomes sharper:

\[
\boxed{
\textbf{Find }\mathcal A\textbf{ with compact morphism encodings and a worst-case-to-average-case theorem.}
}
\]

Plain random modules give good security but large public data. Structured modules give compactness but increase structural risk. This is the fundamental tension.

The known path is:

\[
\mathbb Z_q^n
\quad\to\quad
R_q^k
\]

where \(R_q=\mathbb Z_q[x]/(f(x))\). That is LWE \(\to\) Ring/Module-LWE. It compresses \(A\), accelerates multiplication, and gives practical keys. But it weakens the universality of the hardness claim: now the worst-case problem lives in **structured module lattices**, not arbitrary lattices.

A new path could be:

\[
\text{vector spaces/modules}
\quad\to\quad
\text{chain complexes / sheaves / cohomological modules}
\]

Define a **Noisy Coboundary Section Problem**:

\[
y=d s+e
\]

where \(d:C^0\to C^1\) is a public coboundary map in a random or explicit high-dimensional complex, \(s\) is a hidden 0-cochain, and \(e\) is a small 1-cochain error.

Search:

\[
\text{recover }s\text{ modulo }H^0
\]

or recover a nearby coboundary representative.

Worst-case target:

\[
\text{nearest cochain / cosystolic expansion / short representative in cohomology class.}
\]

That is an honest frontier candidate. But it only becomes cryptographically serious if we can prove something like:

\[
\text{average-case noisy coboundary solving}
\Rightarrow
\text{worst-case cohomological nearest-representative problem.}
\]

Without that theorem, it is just another beautiful object.

## The first-principles conclusion

The canonical hard problem should not be stated as “lattice problem” specifically. It should be stated one abstraction level higher:

\[
\boxed{
\textbf{Transparent Noisy Section over a compactly representable normed additive category.}
}
\]

Then known cryptography appears as instances:

\[
\text{LWE}=\text{TNSP over }\mathbb Z_q\text{-modules with Euclidean/smoothing noise}.
\]

\[
\text{Module-LWE}=\text{TNSP over structured }R_q\text{-modules}.
\]

\[
\text{Syndrome decoding}=\text{TNSP over }\mathbb F_2\text{-vector spaces with Hamming weight}.
\]

\[
\text{Possible sheaf/cochain crypto}=\text{TNSP over chain complexes with sparse cochain error}.
\]

So the research problem becomes:

\[
\boxed{
\begin{aligned}
&\textbf{Construct a compact category }\mathcal A_\lambda\\
&\textbf{with transparent public sampling, efficient forward maps, efficient reconciliation,}\\
&\textbf{and a reduction from average-case TNSP}_{\mathcal A}
\textbf{ to worst-case }H_{\mathcal A}. 
\end{aligned}
}
\]

That is the right first-principles formulation.

The hard part is no longer vague. It is exactly this:

**Can we find a category more compact than plain LWE, less structurally fragile than aggressive algebraic moduli, and still equipped with a worst-case-to-average-case theorem?**

That is the problem worth attacking.

## 36. User

yes. See https://stacks.math.columbia.edu/browse for inspiriation

## 37. Assistant

The Stacks table of contents points to exactly the abstraction layers we want: **sites/sheaves, descent, groupoids/quotients, algebraic stacks, moduli, deformation theory**. I’m going to turn that into a cryptographic problem statement rather than another exotic isogeny proposal.

## 38. Assistant

Yes. The Stacks Project is the right inspiration because its organizing spine is exactly the one we need: **categories → sheaves/sites → descent → groupoids/quotients → stacks → moduli/deformation theory**. The browse page explicitly separates these layers: categories, sheaves on spaces, sites and sheaves, and stacks appear in the preliminaries; descent and groupoid schemes appear in scheme theory; quotient groupoids, algebraic stacks, and moduli stacks appear later as the more global framework. citeturn390946view0

The cryptographic analogue should be:

\[
\boxed{
\textbf{public object} = \text{imperfect descent datum}
}
\]

\[
\boxed{
\textbf{secret coordinate} = \text{a trivialization/gluing witness}
}
\]

\[
\boxed{
\textbf{hard problem} = \text{recover or exploit the trivialization from noisy local-to-global data}
}
\]

That gives us a better canonical hard problem than “find a weird curve.”

## The canonical hard problem: Transparent Noisy Descent

Define a public finite site, or more concretely a finite cover/nerve:

\[
\mathcal S_\lambda
\]

with a sheaf of finite modules

\[
\mathcal F_\lambda.
\]

Start abelian. Nonabelian stacks are tempting, but the proof path should begin in the abelian/sheaf-of-modules case.

The site/sheaf gives cochain groups:

\[
C^0(\mathcal S,\mathcal F),\qquad C^1(\mathcal S,\mathcal F),\qquad C^2(\mathcal S,\mathcal F),\ldots
\]

and a coboundary map

\[
d^0:C^0(\mathcal S,\mathcal F)\to C^1(\mathcal S,\mathcal F).
\]

A secret coordinate is a local trivialization:

\[
s\in C^0(\mathcal S,\mathcal F).
\]

The public object is a noisy descent boundary:

\[
y=d^0s+e.
\]

Here \(e\) is small in a public norm: Hamming weight, Lee weight, rank weight, local sheaf weight, or a discrete Gaussian-like norm.

So the basic problem is:

\[
\boxed{
\textbf{TND}_{\mathcal S,\mathcal F}:
\text{ given }(\mathcal S,\mathcal F,d^0,y=d^0s+e),
\text{ recover }s\text{ modulo global sections, or distinguish }y\text{ from random.}
}
\]

This is the **Transparent Noisy Descent Problem**.

The exact version,

\[
y=d^0s,
\]

is not cryptographically interesting. It is just descent with no error; in the abelian finite case, it collapses to linear algebra. The noise is not decoration. It is what turns exact descent into **nearest descent**.

The canonical worst-case problem is:

\[
\boxed{
\textbf{Nearest Coboundary / Bounded-Distance Coboundary Decoding.}
}
\]

Given

\[
y\in C^1(\mathcal S,\mathcal F),
\]

find

\[
b\in B^1=\operatorname{im}(d^0)
\]

such that

\[
\|y-b\|
\]

is minimal, or at most some promised radius.

This is the stacky/sheaf-theoretic analogue of nearest codeword, syndrome decoding, and lattice bounded-distance decoding.

## Why this is the right abstraction

A descent datum is precisely local data plus compatibility on overlaps. A trivial descent datum comes from local choices \(s_i\), with transition data

\[
s_j-s_i
\]

on overlaps. In cochain language, that is \(d^0s\).

So cryptographically:

\[
\textbf{secret} = \text{local coordinates}
\]

\[
\textbf{public} = \text{overlap data}
\]

\[
\textbf{attack} = \text{reconstruct the hidden local coordinates}
\]

\[
\textbf{noise} = \text{corrupted descent datum}
\]

This is much more principled than “noise and dimension.” It says:

\[
\boxed{
\textbf{public keys are noisy gluing data. private keys are gluing witnesses.}
}
\]

The backdoor-free theorem target becomes:

\[
\boxed{
\text{average-case noisy descent}
\;\Longrightarrow\;
\text{worst-case nearest-coboundary hardness}.
}
\]

That is the form we want. Regev’s LWE work is important because it gives exactly this type of reductional story for ordinary noisy linear equations: LWE’s major theoretical strength is being tied to worst-case lattice hardness. citeturn197427search0 ML-KEM is the standardized practical descendant of this family, with NIST tying its security to Module-LWE and saying it is currently believed secure even against quantum adversaries. citeturn197427search1

Our question is whether we can generalize that proof pattern from vector-space/module equations to **descent complexes**.

## The abelian construction skeleton

Let

\[
D=d^0:C^0\to C^1.
\]

Think of \(D\) as a sparse structured matrix generated from a public site/sheaf.

### Key generation

Sample

\[
s\leftarrow C^0,
\qquad
e\leftarrow \chi_{C^1}.
\]

Publish

\[
t=Ds+e.
\]

The public key is:

\[
pk=(\text{seed for }D,\; t).
\]

The secret key is:

\[
sk=s.
\]

### Encapsulation

Sample a small dual cochain

\[
r\in (C^1)^\vee.
\]

Compute

\[
u=D^\ast r
\]

and

\[
v=\langle r,t\rangle+\operatorname{Encode}(K).
\]

Send

\[
ct=(u,v).
\]

### Decapsulation

The owner computes

\[
v-\langle u,s\rangle.
\]

Since

\[
u=D^\ast r,
\]

we have

\[
\langle u,s\rangle
=
\langle D^\ast r,s\rangle
=
\langle r,Ds\rangle.
\]

Therefore

\[
v-\langle u,s\rangle
=
\langle r,Ds+e\rangle+\operatorname{Encode}(K)-\langle r,Ds\rangle
\]

so

\[
v-\langle u,s\rangle
=
\operatorname{Encode}(K)+\langle r,e\rangle.
\]

If the noise term is small enough, round/reconcile to recover \(K\).

This is an LWE/Regev-style encryption pattern, but the public morphism is not an arbitrary matrix. It is a **descent differential** from a transparent site/sheaf construction.

The security assumption is the decision version:

\[
(D,Ds+e)\approx_c(D,u)
\]

where \(u\) is uniform in \(C^1\).

Then the KEM proof can follow the known pattern: PKE from noisy descent, then a Fujisaki–Okamoto-style transform for CCA security, analogous in spirit to the FO transform used in ML-KEM’s standardized construction. citeturn197427search3

## What Stacks adds beyond ordinary LWE

We have to be honest: a finite abelian sheaf complex is still linear algebra. If we instantiate it naively, we have just invented “LWE with a funny matrix.”

The Stacks-inspired contribution is not vocabulary. It is the design space:

\[
\boxed{
\textbf{local-to-global structure with controlled cohomology and transparent generation.}
}
\]

We want a family where:

\[
C^0 \xrightarrow{d^0} C^1 \xrightarrow{d^1} C^2
\]

has useful expansion/cosystolic properties, compact generation, and no hidden parameter choices.

The hard problem is not merely solving

\[
Ds+e.
\]

It is solving noisy descent in a family where \(D\) is generated by a site/sheaf with strong local-to-global guarantees.

That suggests we should look for public families with:

\[
\textbf{sparse local incidence}
\]

\[
\textbf{large cosystolic distance}
\]

\[
\textbf{small cohomology or controlled cohomology}
\]

\[
\textbf{efficient multiplication by }d^0,d^{0\ast}
\]

\[
\textbf{compact seedable description}
\]

\[
\textbf{worst-case-to-average-case reduction}
\]

The last condition is the hard one.

## The nonabelian stack version

Once the abelian case is clean, the stacky version is natural.

Let \(\mathcal G\) be a sheaf of finite groups. A secret is a family of local labels

\[
h_i\in \mathcal G(U_i).
\]

The exact descent transitions are

\[
g_{ij}=h_i^{-1}h_j.
\]

They satisfy the cocycle condition on triple overlaps:

\[
g_{ij}g_{jk}g_{ki}=1.
\]

The noisy public object is

\[
\tilde g_{ij}=h_i^{-1}h_j\cdot \epsilon_{ij}.
\]

The hard problem is:

\[
\boxed{
\textbf{Noisy Torsor Trivialization:}
\text{ recover }h_i\text{, or compute a shared function of them, from corrupted transition data.}
}
\]

This is the stack analogue of noisy descent. It is closer to “objects up to isomorphism” and quotient stacks. But the proof story is weaker. Many group synchronization problems have spectral, SDP, or representation-theoretic attacks. Nonabelian structure is not enough.

So I would treat nonabelian noisy torsors as **stage two**, not the canonical starting assumption.

## The derived/deformation version

The Stacks Project’s later layers also point toward deformation theory and cotangent complexes. The cryptographic analogue would be:

\[
\textbf{secret} = \text{gauge transformation / deformation witness}
\]

\[
\textbf{public} = \text{noisy Maurer–Cartan or obstruction datum}
\]

For a dg Lie algebra \(L^\bullet\), one could imagine publishing a perturbed Maurer–Cartan element

\[
\alpha = g\cdot \alpha_0 + e
\]

and asking the attacker to recover the gauge coordinate \(g\), or find a nearby object in the same deformation class.

That is beautiful, but too unconstrained right now. I would not use it as the canonical hard problem until we know how to prove average-case hardness.

## The problem statement we should write down

Here is the clean formulation.

\[
\boxed{
\begin{aligned}
\textbf{Transparent Noisy Descent Problem.}\\
\text{For each security parameter }\lambda,\text{ generate from public randomness}\\
\text{a finite site }\mathcal S_\lambda\text{ and a finite normed sheaf of modules }\mathcal F_\lambda.\\
\text{Let }D_\lambda=d^0:C^0(\mathcal S_\lambda,\mathcal F_\lambda)\to C^1(\mathcal S_\lambda,\mathcal F_\lambda).\\
\text{Given }(D_\lambda,y),\ y=D_\lambda s+e,\text{ with }s\text{ random and }e\text{ small,}\\
\text{distinguish }y\text{ from uniform or recover }s\text{ modulo }H^0(\mathcal S_\lambda,\mathcal F_\lambda).
\end{aligned}
}
\]

The canonical worst-case problem:

\[
\boxed{
\begin{aligned}
\textbf{Nearest Coboundary Problem.}\\
\text{Given }D_\lambda\text{ and }y\in C^1,\text{ find }b\in \operatorname{im}(D_\lambda)\\
\text{minimizing }\|y-b\|,\text{ or find }b\text{ within a promised decoding radius.}
\end{aligned}
}
\]

The no-backdoor theorem target:

\[
\boxed{
\text{Any nonuniform quantum polynomial-time adversary solving average-case TND}
\Rightarrow
\text{a solver for worst-case Nearest Coboundary in the same public family.}
}
\]

This quantifies over black-hat “secret tricks” as nonuniform advice. That is what we want.

## What would count as progress

The first real deliverable is not a polished KEM. It is a theorem candidate:

\[
\boxed{
\textbf{Average-case noisy descent over a transparent sheaf-complex family is as hard as worst-case nearest coboundary.}
}
\]

To attempt that, we need to choose the family.

My first candidate family would be:

\[
\boxed{
\textbf{bounded-degree high-dimensional expander complexes with finite-module coefficient sheaves.}
}
\]

Why? Because they are local, sparse, seedable, and naturally cohomological. The norm is local weight. The public map is a coboundary. The hard problem is nearest coboundary/cosystolic decoding. The cryptographic object is no longer “a matrix”; it is a **local-to-global obstruction system**.

But the bar is high: unless we can prove the average-case-to-worst-case bridge, this is only an elegant code-based system.

## The concise synthesis

The Stacks-inspired hard problem should be:

\[
\boxed{
\textbf{Noisy descent, not hidden scalar multiplication.}
}
\]

ECC hides a scalar in an exact abelian torsor.  
LWE hides a vector in noisy linear measurements.  
A Stacks-inspired primitive should hide a **trivialization** in noisy descent data.

The canonical hard problem is therefore:

\[
\boxed{
\textbf{Nearest Coboundary / Noisy Descent over transparent finite sites and sheaves.}
}
\]

And the research question becomes:

\[
\boxed{
\text{Can we construct compact, transparent sheaf-complex families for which average-case noisy descent reduces to worst-case nearest-coboundary hardness, even against nonuniform quantum adversaries?}
}
\]

That is the right formulation. It keeps the backdoor constraint explicit, it uses the Stacks hierarchy for real structural reasons, and it gives us a concrete path to an implementable KEM only after the hard problem is properly specified.

## 39. User

Lets make this concrete and turn it into a real algorithm/ research proposal that will, if successful, produce a provably hard to break quantum cryptography system.

## 40. Assistant

I’ll make this operational as a research-grade KEM proposal: concrete algorithms, explicit noisy-descent hardness assumptions, reduction targets, and implementation/proof milestones while separating prototype from security theorem.

## 41. Assistant

The first version should stay abelian and cochain-based, with a reducible LWE-like baseline before adding compact sheaf/site structure where genuine novelty enters.

## 42. Assistant

Yes. Let’s turn it into a **real research proposal plus implementable candidate algorithm**. I’ll phrase it as a post-quantum public-key/KEM project, not QKD.

The core design will be:

\[
\boxed{
\textbf{P-TND-KEM: Projected Transparent Noisy Descent Key Encapsulation}
}
\]

The “projected” part is important. A naïve public key of the form

\[
y=d^0s+e
\]

is too directly structured: if the higher coboundary \(d^1\) is public, then

\[
d^1y=d^1e
\]

may be an immediate distinguisher, because \(d^1e\) remains small/local. So the public object should not be raw noisy descent data. It should be a **public randomized observation** of descent data:

\[
\boxed{
t=\Pi d^0s+e
}
\]

where \(d^0\) is a descent/coboundary operator and \(\Pi\) is a transparent public projection/measurement operator. This keeps the Stacks-inspired meaning—**secret = local trivialization, public = noisy measured gluing data**—but avoids publishing the obvious cocycle tests directly.

The Stacks Project is structurally relevant because it organizes the exact hierarchy we are borrowing from: categories, sheaves on spaces, sites and sheaves, stacks, sheaves of modules, cohomology, descent, groupoids, algebraic stacks, and moduli stacks. citeturn288869view0

---

## 1. Mathematical object

For each security parameter \(\lambda\), public setup constructs:

\[
X_\lambda
\]

a finite combinatorial site, concretely a bounded-degree simplicial or cubical complex.

Then choose a finite sheaf of modules

\[
\mathcal F_\lambda
\]

over \(X_\lambda\), usually with fibers

\[
\mathcal F(v)\cong \mathbb F_q^k.
\]

The cochains are

\[
C^0=C^0(X,\mathcal F),
\qquad
C^1=C^1(X,\mathcal F).
\]

The descent coboundary is

\[
D=d^0:C^0\to C^1.
\]

A secret key is a local trivialization:

\[
s\in C^0.
\]

The exact descent/gluing datum would be

\[
Ds\in C^1.
\]

But instead of publishing \(Ds+e\), we publish a **measured descent datum**:

\[
t=As+e,
\qquad
A=\Pi D,
\]

where

\[
\Pi:C^1\to M
\]

is a public observation map into a measurement module \(M\cong \mathbb F_q^m\).

So the public linear map is

\[
A:C^0\to M.
\]

The cryptographic hard problem is not “solve a random matrix equation.” It is:

\[
\boxed{
\textbf{recover a hidden local trivialization from noisy projected descent data.}
}
\]

This is the first concrete hard problem.

---

## 2. Canonical hard problem

Define **Projected Transparent Noisy Descent**, or **P-TND**.

### Search P-TND

Given public data

\[
(X,\mathcal F,\Pi,q)
\]

and

\[
t=\Pi d^0s+e,
\]

where \(s\leftarrow \chi_s^{C^0}\) and \(e\leftarrow \chi_e^M\) are small, recover \(s\), or any \(s'\) such that

\[
\Pi d^0s'\approx t.
\]

Equivalently, find a nearby projected coboundary.

### Decision P-TND

Distinguish

\[
(X,\mathcal F,\Pi,\Pi d^0s+e)
\]

from

\[
(X,\mathcal F,\Pi,u),
\qquad
u\leftarrow M \text{ uniform}.
\]

This is the direct analogue of LWE, but with the public map \(A\) constrained to factor through a transparent descent operator:

\[
A=\Pi d^0.
\]

The analogy to LWE is deliberate. LWE’s philosophical strength is not merely “noise”; it is the worst-case-to-average-case reduction from hard lattice problems to noisy average-case instances. Regev’s work is the canonical model for this proof shape. citeturn144800search19

For practical comparison, NIST’s ML-KEM is built on Module-LWE and is currently standardized as a post-quantum KEM; NIST says ML-KEM’s security is related to Module-LWE and is presently believed secure even against adversaries with quantum computers. citeturn144800search2turn144800search5 Module-LWE itself has worst-case-to-average-case reductions for module lattices, which is the kind of reductional story we want to reproduce in this sheaf/descent setting. citeturn144800search22

---

## 3. Worst-case problem to reduce from

The canonical worst-case problem should be a sheaf/coboundary analogue of lattice decoding.

Given

\[
A=\Pi d^0:C^0\to M,
\]

define the projected coboundary code/lattice

\[
B_A=\operatorname{im}(A)\subseteq M.
\]

The worst-case problem is:

\[
\boxed{
\textbf{Nearest Projected Coboundary Problem}
}
\]

Given

\[
y\in M,
\]

find

\[
b\in B_A
\]

minimizing

\[
\|y-b\|.
\]

Or, in bounded-distance form:

\[
\text{given } y=b+e,\quad b\in B_A,\quad \|e\|\le \rho,
\]

recover \(b\).

Equivalently, define the \(q\)-ary sheaf lattice

\[
\Lambda_A^\perp
=
\{z\in\mathbb Z^m: A^\top z\equiv 0\pmod q\}.
\]

Then the problem becomes a structured \(q\)-ary lattice decoding problem attached to the sheaf complex.

The central theorem target is:

\[
\boxed{
\text{average-case P-TND solver}
\Rightarrow
\text{worst-case nearest projected coboundary / sheaf-lattice solver.}
}
\]

And the **no-backdoor** target is stronger:

\[
\boxed{
\text{any nonuniform QPT adversary breaking the scheme}
\Rightarrow
\text{a solver for the canonical worst-case problem.}
}
\]

That includes a black-hat exploit as polynomial-size advice.

---

## 4. The object family

We need \(X_\lambda\) and \(\mathcal F_\lambda\) to be transparent, compact, efficiently computable, and expansion-rich.

The natural candidate is:

\[
\boxed{
\textbf{bounded-degree high-dimensional expanders with coefficient sheaves.}
}
\]

Why this family?

High-dimensional expanders give local-to-global structure, cosystolic/coboundary expansion, and code-like local testability properties. Evra and Kaufman constructed bounded-degree cosystolic expanders in every dimension using a local-to-global criterion. citeturn752702search0 Work by Dinur and Kaufman connects high-dimensional expansion to agreement expansion, which is exactly the kind of local-to-global constraint system that makes this family natural for noisy descent. citeturn752702search9 There is also active work on sheaves over high-dimensional expanders as a source of locally testable codes, which is directly adjacent to the coefficient-sheaf construction we need. citeturn752702search6

Concretely:

\[
X_\lambda = \text{bounded-degree HDX}
\]

\[
\mathcal F_\lambda(v)=\mathbb F_q^k
\]

\[
D=d^0
\]

\[
\Pi = \text{public randomized local observation operator.}
\]

The operator \(\Pi\) should be generated from public randomness and should destroy obvious cocycle tests while preserving efficient multiplication by \(A\) and \(A^\top\).

---

## 5. Concrete algorithm: P-TND-KEM

This is a Kyber/Regev-shaped KEM, but the matrix \(A\) is not arbitrary and not secretly generated:

\[
A=\Pi d^0.
\]

### Public parameters

```text
q              prime modulus
X              finite site / bounded-degree complex
F              finite coefficient sheaf over X
D = d^0        descent coboundary C^0 -> C^1
Π              public projection C^1 -> M
A = ΠD         public map C^0 -> M, represented only by algorithms
χ_s, χ_e       small secret/noise distributions
Encode, Decode message encoding and reconciliation
H, G, KDF      hash/expansion functions
```

The public map \(A\) is not stored densely. It is represented by two algorithms:

```text
A_mul(s)    = Π(D(s))
AT_mul(r)   = D^T(Π^T(r))
```

This is important for compactness.

---

## 6. Setup

```text
Setup(1^λ):

    Choose q, k, n0, m, noise parameters.

    Generate X_λ from public seed seed_X.
        X is a bounded-degree simplicial/cubical complex.

    Generate a coefficient sheaf F over X from public seed seed_F.
        Fibers are F_q^k.
        Restriction maps are public sparse/block matrices.

    Construct D = d^0 : C^0(X,F) -> C^1(X,F).

    Generate public projection Π : C^1(X,F) -> F_q^m
        from public seed seed_Π.
        Each row observes a small pseudorandom patch of C^1.

    Define A = ΠD.

    Publish pp = (q, X description, seed_F, seed_Π, parameters).
```

There is no hidden setup. The seeds are public. A verifier can recompute everything.

The projection \(\Pi\) has two jobs:

1. **Avoid raw cocycle leakage** such as \(d^1(Ds+e)=d^1e\).
2. **Retain enough structure** that \(A\) can be multiplied quickly and stored compactly.

---

## 7. Key generation

Let

\[
n=\dim C^0,
\qquad
m=\dim M.
\]

Secret and error are small:

\[
s\leftarrow \chi_s^n,
\qquad
e\leftarrow \chi_e^m.
\]

Compute

\[
t=As+e\pmod q.
\]

```text
KeyGen(pp):

    s ← χ_s^n
    e ← χ_e^m

    t = A_mul(s) + e mod q

    pk = t
    sk = (s, z)
        where z is a random fallback secret for implicit rejection

    return pk, sk
```

The public key is the noisy projected descent datum:

\[
pk=t.
\]

The private key is the local trivialization:

\[
sk=s.
\]

---

## 8. CPA encryption core

For a message seed \(\mu\), derive randomness deterministically:

\[
(r,e_1,e_2)=G(\mu,pk).
\]

Sample dimensions:

\[
r\in \mathbb F_q^m,
\qquad
e_1\in C^0,
\qquad
e_2\in \mathbb F_q^\ell.
\]

Usually \(r,e_1,e_2\) are small.

Compute:

\[
u=A^\top r+e_1
\]

and

\[
v=\langle r,t\rangle+e_2+\operatorname{Encode}(\mu).
\]

Ciphertext:

\[
ct=(u,v).
\]

Decapsulation computes:

\[
v-\langle u,s\rangle.
\]

Since

\[
u=A^\top r+e_1
\]

and

\[
t=As+e,
\]

we have

\[
\langle r,t\rangle
=
\langle r,As+e\rangle
=
\langle A^\top r,s\rangle+\langle r,e\rangle.
\]

Therefore

\[
v-\langle u,s\rangle
=
\operatorname{Encode}(\mu)
+
e_2
+
\langle r,e\rangle
-
\langle e_1,s\rangle.
\]

If the final noise term is below the reconciliation threshold, decoding recovers \(\mu\).

This is the same algebraic cancellation pattern as lattice KEMs, but with

\[
A=\Pi d^0.
\]

---

## 9. Full KEM with implicit rejection

```text
Encaps(pp, pk):

    μ ← {0,1}^256

    (r, e1, e2) = G("coins" || μ || Hash(pk))

    u = AT_mul(r) + e1 mod q

    v = <r, pk> + e2 + Encode(μ) mod q

    ct = (u, v)

    ss = KDF("P-TND-KEM" || μ || Hash(pk) || Hash(ct))

    return ct, ss
```

```text
Decaps(pp, sk, pk, ct):

    parse ct = (u, v)

    w = v - <u, s> mod q

    μ' = Decode(w)

    (r', e1', e2') = G("coins" || μ' || Hash(pk))

    u' = AT_mul(r') + e1' mod q

    v' = <r', pk> + e2' + Encode(μ') mod q

    ct' = (u', v')

    if ct' == ct:
        ss = KDF("P-TND-KEM" || μ' || Hash(pk) || Hash(ct))
    else:
        ss = KDF("P-TND-KEM" || z || Hash(pk) || Hash(ct))

    return ss
```

This follows the usual KEM discipline: build a passively secure PKE-like core, then use a Fujisaki–Okamoto-style transform/implicit rejection to get chosen-ciphertext security. Kyber/ML-KEM follows this broad pattern; the Kyber specification says Kyber.CCAKEM is obtained from Kyber.CPAPKE via a tweaked Fujisaki–Okamoto transform, and modular analyses of FO cover quantum-random-oracle settings. citeturn300899search5turn300899search16

---

## 10. Correctness condition

The decryption noise is

\[
\eta
=
e_2+\langle r,e\rangle-\langle e_1,s\rangle.
\]

Correctness requires

\[
\|\eta\|_\infty < q/4
\]

or whatever threshold the chosen reconciliation uses.

So parameter selection must enforce a tail bound:

\[
\Pr[\|\eta\|_\infty\ge q/4]\le 2^{-\lambda_{\mathrm{fail}}}.
\]

This part is straightforward. It is the same engineering problem as in LWE/MLWE systems: choose \(q\), dimensions, and noise widths so decryption failure is negligible.

---

## 11. Why this is actually Stacks-inspired

In ordinary LWE, the public map is just a matrix:

\[
A:s\mapsto As.
\]

Here, \(A\) factors as

\[
C^0(X,\mathcal F)
\xrightarrow{d^0}
C^1(X,\mathcal F)
\xrightarrow{\Pi}
M.
\]

Interpretation:

\[
s\in C^0
\]

is a system of local coordinates/trivializations.

\[
d^0s\in C^1
\]

is the induced overlap/gluing datum.

\[
\Pi d^0s+e
\]

is noisy public measurement of the gluing datum.

So:

\[
\boxed{
\textbf{private key} = local trivialization
}
\]

\[
\boxed{
\textbf{public key} = noisy projected descent datum
}
\]

\[
\boxed{
\textbf{hard problem} = noisy projected descent inversion
}
\]

That is a genuine categorical/descent generalization of noisy linear coordinate hiding.

---

## 12. The no-backdoor theorem target

The proposal succeeds only if we prove something like the following.

### Theorem A: expansion/no-short-dual tests

For the public family

\[
A=\Pi d^0,
\]

the associated \(q\)-ary sheaf lattices

\[
\Lambda_A^\perp
=
\{z\in\mathbb Z^m:A^\top z\equiv0\pmod q\}
\]

have no efficiently exploitable short dual vectors, except with negligible probability over public setup.

This rules out immediate distinguishers.

### Theorem B: worst-case to average-case reduction

For every nonuniform quantum polynomial-time algorithm \(\mathcal A\) that solves average-case P-TND with non-negligible advantage, there exists a quantum polynomial-time algorithm \(\mathcal B^\mathcal A\) solving worst-case nearest projected coboundary / sheaf-lattice decoding in the same public family.

Symbolically:

\[
\boxed{
\mathsf{P\text{-}TND}_{\mathrm{avg}}
\leq_{\mathrm{QPT}}
\mathsf{NPCoboundary}_{\mathrm{worst}}.
}
\]

This is the central research theorem.

### Theorem C: KEM security

Assuming decision P-TND and its dual/transposed variant, the CPA core is IND-CPA secure.

Then the FO/implicit-rejection wrapper gives IND-CCA security in the quantum random oracle model.

### Corollary: no structural backdoor

If any nonuniform QPT adversary, with arbitrary polynomial-size advice \(\tau_\lambda\), breaks P-TND-KEM, then that adversary yields a solver for the canonical worst-case projected coboundary problem.

That is the formal way to capture “a black hat found a secret exploit.”

---

## 13. Concrete instantiation plan

### Prototype family

Start with:

\[
q\in\{3329,4093,7681\}
\]

for implementation convenience.

Let \(X\) be a bounded-degree complex with

\[
|X_0|=N_0,
\qquad
|X_1|=N_1.
\]

Fibers:

\[
\mathcal F(v)=\mathbb F_q^k
\]

with \(k=1,2,4\) initially.

Then

\[
n=k|X_0|,
\qquad
N_1'=k|X_1|.
\]

Projection:

\[
\Pi:\mathbb F_q^{N_1'}\to\mathbb F_q^m.
\]

Each row of \(\Pi\) observes \(w\) pseudorandom edge-fiber coordinates, with random signs/weights.

Example toy parameters:

```text
q       = 3329
k       = 2
|X0|    = 512
n       = 1024
m       = 1536
row wt Π = 32
χ        = centered binomial η=2 or η=3
```

This is not a claimed secure set. It is enough to implement, test correctness, and start cryptanalysis.

Target parameters might sit around:

```text
n       ≈ 768–2048
m/n     ≈ 1.3–2.0
q       ≈ 2^12 to 2^15
row wt Π ≈ 32–96
noise   tuned by estimator and proof
```

Public key size:

\[
m\log_2 q
\]

plus small seeds.

Ciphertext size:

\[
n\log_2 q+\ell\log_2 q
\]

plus small overhead.

This will not be ECC-sized. The point is **proof discipline first**. Compactness comes from seedable \(D,\Pi\), not from tiny moduli invariants.

---

## 14. Implementation architecture

The implementation should expose only four linear-algebra oracles.

```text
D_mul(s):      C^0 -> C^1
DT_mul(y):     C^1 -> C^0

Pi_mul(y):     C^1 -> M
PiT_mul(r):    M -> C^1
```

Then:

```text
A_mul(s):
    return Pi_mul(D_mul(s))

AT_mul(r):
    return DT_mul(PiT_mul(r))
```

Everything else is ordinary KEM plumbing.

Sparse implementation:

```text
D_mul:
    for each oriented edge e=(v,w):
        out[e] = ρ_{w,e} s[w] - ρ_{v,e} s[v]

Pi_mul:
    for each measurement row i:
        out[i] = Σ_j coeff[i,j] * y[index[i,j]]
```

Transpose implementation:

```text
PiT_mul:
    initialize y = 0
    for each measurement row i:
        for each selected coordinate j:
            y[j] += coeff[i,j] * r[i]

DT_mul:
    for each oriented edge e=(v,w):
        accumulate restriction-transpose contributions to v,w
```

This is implementable in C/Rust/Python immediately.

---

## 15. Research milestones

### Milestone 1: working toy KEM

Deliver:

```text
setup.py
keygen.py
encaps.py
decaps.py
tests_correctness.py
```

Use random regular graphs or small complexes first. Do not claim security.

Goal:

\[
\Pr[\text{decapsulation failure}] < 2^{-40}
\]

for toy settings.

### Milestone 2: attack harness

Implement obvious attacks:

\[
\text{linear distinguishers}
\]

\[
\text{low-weight dual search}
\]

\[
\text{primal lattice embedding}
\]

\[
\text{dual lattice attacks}
\]

\[
\text{BKW-style attacks}
\]

\[
\text{information-set decoding analogues}
\]

\[
\text{distinguishers using }X,\mathcal F,\Pi\text{ structure}
\]

The first thing to try to break is \(\Pi\). If \(\Pi\) leaves short constraints, the system dies.

### Milestone 3: prove expansion properties

Prove that the chosen \(X,\mathcal F,\Pi\) family has:

\[
\text{large primal distance}
\]

\[
\text{large dual distance}
\]

\[
\text{no sparse annihilators}
\]

\[
\text{smoothing-like behavior of }\Lambda_A^\perp
\]

This is where high-dimensional expander theory enters.

### Milestone 4: average-case to worst-case theorem

This is the main mathematical project.

We need an analogue of:

\[
\text{LWE average-case hardness}
\Leftarrow
\text{worst-case lattice hardness}.
\]

For P-TND:

\[
\text{average-case noisy projected descent hardness}
\Leftarrow
\text{worst-case nearest projected coboundary hardness}.
\]

If this theorem fails, the project is not backdoor-free by our standard.

### Milestone 5: QPT/nonuniform formulation

State the reduction against nonuniform quantum polynomial-time adversaries.

That is the part that turns “no known black-hat trick” into:

\[
\text{black-hat trick} \Rightarrow \text{solver for canonical hard problem}.
\]

### Milestone 6: final KEM proof

Once P-TND decision/search hardness is established:

\[
\text{P-TND hardness}
\Rightarrow
\text{CPA security}
\Rightarrow
\text{CCA security via FO/implicit rejection}.
\]

---

## 16. What could go wrong

The main failure modes are clear.

**First**, if \(A=\Pi d^0\) has low-weight dual relations, then decision P-TND is distinguishable.

**Second**, if \(\Pi\) is too random/dense, the scheme collapses back to ordinary LWE with a fancy description. That is safe but not new.

**Third**, if \(\Pi\) is too structured/sparse, attacks may exploit the geometry of \(X\) or the coefficient sheaf.

**Fourth**, if the worst-case-to-average-case reduction cannot be proved, the scheme may still be interesting, but it fails the “provably no backdoors” requirement.

**Fifth**, if the public family admits hidden canonicalization, spectral leakage, or a small representation-theoretic quotient, that quotient may be a structural backdoor.

So the first serious theorem must be about **dual constraints and smoothing**. Without that, do not trust the scheme.

---

## 17. The actual proposal statement

Here is the research proposal in compact form.

\[
\boxed{
\begin{aligned}
&\textbf{Project: P-TND-KEM}\\[2mm]
&\textbf{Goal: Construct a post-quantum KEM whose security reduces to}\\
&\textbf{worst-case nearest projected coboundary over transparent sheaf complexes.}
\end{aligned}
}
\]

For each \(\lambda\), public setup generates:

\[
X_\lambda,\mathcal F_\lambda,D_\lambda=d^0,\Pi_\lambda
\]

from public randomness.

Define:

\[
A_\lambda=\Pi_\lambda D_\lambda.
\]

The public key is:

\[
t=A_\lambda s+e.
\]

Encapsulation uses:

\[
u=A_\lambda^\top r+e_1,
\qquad
v=\langle r,t\rangle+e_2+\operatorname{Encode}(\mu).
\]

Decapsulation computes:

\[
v-\langle u,s\rangle
=
\operatorname{Encode}(\mu)
+
e_2+\langle r,e\rangle-\langle e_1,s\rangle.
\]

Security target:

\[
\boxed{
\text{IND-CCA break}
\Rightarrow
\text{decision/search P-TND break}
\Rightarrow
\text{worst-case nearest projected coboundary solver}.
}
\]

No-backdoor target:

\[
\boxed{
\text{the above reduction holds for nonuniform QPT adversaries with arbitrary advice.}
}
\]

That is the correct formulation.

---

## 18. The first implementation should be deliberately modest

I would implement this in three layers.

**Layer 1: algebraic kernel**

```text
finite field arithmetic
sparse block matrices
D_mul / DT_mul
Pi_mul / PiT_mul
A_mul / AT_mul
```

**Layer 2: KEM core**

```text
KeyGen
Encaps
Decaps
reconciliation
failure-rate testing
```

**Layer 3: research diagnostics**

```text
rank(A)
dual low-weight search
empirical distribution tests
lattice-attack estimator
BKW-style distinguisher
projection-structure attacks
```

Do not begin with the fanciest HDX. Begin with a simple graph/sheaf/projection system, break it, then increase structure until the diagnostics stop finding trivial attacks. Only then attempt the theorem.

---

## 19. Why this is the right next step

This proposal satisfies the abstraction we wanted:

\[
\textbf{coordinates are hidden as local trivializations}
\]

\[
\textbf{public data is noisy measured descent}
\]

\[
\textbf{setup is transparent}
\]

\[
\textbf{the hard problem is canonical}
\]

\[
\textbf{backdoors are quantified as nonuniform adversarial advice}
\]

\[
\textbf{success requires a reduction, not vibes}
\]

It also explains exactly how the project could fail.

The final deliverable, if the proof works, would be a real KEM whose security statement has the form:

\[
\boxed{
\text{Breaking P-TND-KEM with a quantum computer is at least as hard as solving worst-case nearest projected coboundary in the public sheaf-complex family.}
}
\]

That is the sort of theorem that would make the system meaningfully “provably hard to break” under a stated, transparent, canonical assumption.

## 43. User

this seems reasonable, except for one thing: increasing  structure until diagnostics stop finding trivial attacks, then attempting to prove a theorem is extremely speculative. We need to be more principled in finding the right HDX. To me, the way to approach this is to start actually sketching the proof, and as we iterate on the proof, take note of the mathematical properties that are needed. Then we work from those properties as our next principles.

## 44. Assistant

I agree. The right workflow is proof-first: sketch the desired reduction, locate its failure points, then turn each into an axiom for the HDX/sheaf family.

## 45. Assistant

The correction is to let the reduction template dictate lemmas like sampling, smoothing, dual distance, expansion, and random self-reducibility. If no known HDX satisfies them, that defines the object to construct.

## 46. Assistant

Exactly. The proof should drive the object. Diagnostics are only sanity checks after the theorem has told us what must be true.

So let’s reverse the workflow:

\[
\textbf{desired reduction}
\quad\Longrightarrow\quad
\textbf{required axioms}
\quad\Longrightarrow\quad
\textbf{candidate HDX/sheaf family}
\quad\Longrightarrow\quad
\textbf{algorithm}.
\]

The Stacks-inspired layer remains: **private key = local trivialization**, **public key = noisy measured descent datum**, **security = hardness of noisy descent**. The Stacks Project’s table of contents is structurally relevant here because it goes from categories, sheaves, sites, modules on sites, cohomology, descent, groupoids, stacks, and moduli—exactly the abstraction ladder we are borrowing from. citeturn900874view0

---

## 1. Start with the proof we want

Let

\[
C^0 \xrightarrow{D=d^0} C^1 \xrightarrow{d^1} C^2
\]

be the cochain complex of a finite site/sheaf pair \((X,\mathcal F)\) over \(\mathbb F_q\). We quotient out global sections:

\[
V := C^0/H^0(X,\mathcal F),
\]

so that

\[
D:V\to C^1
\]

is injective.

We do **not** publish raw noisy descent data \(Ds+e\), because then \(d^1(Ds+e)=d^1e\) may expose a small-error distinguisher. Instead choose a public observation map

\[
\Pi:C^1\to M
\]

and define

\[
A=\Pi D:V\to M.
\]

The public key is

\[
t=As+e.
\]

The cryptographic problem is:

\[
\boxed{
\textbf{P-TND: given } A=\Pi d^0,\ t=As+e,\ \textbf{distinguish from uniform or recover }s.
}
\]

The proof goal is:

\[
\boxed{
\textbf{Breaking P-TND-KEM}
\Rightarrow
\textbf{solving worst-case Nearest Projected Coboundary}
}
\]

against **nonuniform quantum polynomial-time adversaries**. That last phrase is the formal way to include “a black hat found a hidden exploit and wrote it down as advice.”

The worst-case problem is:

\[
\boxed{
\textbf{Nearest Projected Coboundary: given }y\in M,\textbf{ find }b\in A(V)
\textbf{ minimizing }\|y-b\|.
}
\]

Equivalently, decode near the projected coboundary code/lattice

\[
A(V)\subseteq M.
\]

This is the exact analogue of LWE’s reductional posture. Regev/Micciancio–Regev reductions use Gaussian/smoothing machinery to connect average noisy linear equations to worst-case lattice problems; that is the proof architecture we should try to generalize, not merely imitate syntactically. citeturn670437view0

---

## 2. What the proof immediately forces

To prove anything meaningful, the public family

\[
(X,\mathcal F,\Pi)
\]

must satisfy a set of axioms. These are not implementation desiderata. They are **proof obligations**.

### Axiom 1: controlled kernel

We need

\[
\ker(D)=H^0(X,\mathcal F)
\]

and after quotienting,

\[
D:V\to C^1
\]

must be injective with controlled conditioning.

Over the reals, after choosing normalized lifts,

\[
\sigma_{\min}(D^\ast D)\ge c,
\qquad
\sigma_{\max}(D^\ast D)\le C
\]

for constants or at worst polylogarithmic factors.

Cryptographic meaning: **different local trivializations must produce macroscopically different descent data.** Otherwise decoding is ill-conditioned and hidden short directions become backdoors.

This is a Poincaré/spectral-gap condition.

### Axiom 2: cosystolic distance

For every nonzero class of local coordinate differences,

\[
x\in V,\quad x\ne 0,
\]

we need

\[
\|Dx\|\ge \alpha \|x\|
\]

in the relevant local norm.

More generally, in degree \(1\), any cochain close to a cocycle/coboundary should be testably close. This is where **cosystolic expansion** enters.

Evra and Kaufman’s bounded-degree cosystolic expander work is relevant because it gives a local-to-global criterion for cosystolic expansion and applies it to Ramanujan complexes. citeturn129037search1

Cryptographic meaning: **nearest projected coboundary should have a unique decoding radius.**

### Axiom 3: no low-weight dual relations

A decision-LWE-like scheme dies if there is a small public vector \(r\) such that

\[
A^\ast r=0.
\]

Then

\[
\langle r,t\rangle=\langle r,e\rangle
\]

is small for real keys but uniform for random \(t\). That is an immediate distinguisher.

Here

\[
A^\ast=D^\ast\Pi^\ast.
\]

So low-weight dual attacks correspond to

\[
\Pi^\ast r\in \ker(D^\ast),
\]

i.e. the image of \(r\) lands in a cycle-like subspace.

Thus the projection \(\Pi\) is not an afterthought. It must be chosen so that

\[
\boxed{
\Pi^\ast(M^\ast)\cap \ker(D^\ast)
\]

has no small nonzero vectors, and preferably is trivial.}

This is the first serious design constraint. A raw boundary/coboundary matrix of a complex has many local relations. The projection must be **transverse to cycles**.

### Axiom 4: local spectral expansion

We need mixing and concentration strong enough to support smoothing arguments. In proof language, we want Poincaré/log-Sobolev-type inequalities on the cochain spaces and their links.

Oppenheim’s local spectral expansion program is relevant because local spectral expansion implies spectral gaps in links and global Laplacians. citeturn670437view2

Cryptographic meaning: **random local perturbations should smooth globally**, not remain trapped in small geometric defects.

### Axiom 5: projection as a lossless sampler/condenser

The projection

\[
\Pi:C^1\to M
\]

must satisfy two-sided randomness properties.

For every nonzero \(x\in V\),

\[
\Pr_{\Pi}[\Pi D x=0]\le q^{-m}+\varepsilon.
\]

For every small nonzero \(r\in M^\ast\),

\[
D^\ast \Pi^\ast r
\]

must be nonzero and large.

So \(\Pi\) should behave like a **universal hash / lossless condenser on the coboundary space**, while also being efficiently computable from a short public seed.

This is where the HDX alone is not enough. We need an HDX **plus a projection code**.

### Axiom 6: self-randomization

A worst-case-to-average-case theorem needs a way to randomize a hard instance into the average distribution while preserving its hard core.

For ordinary LWE, the row vectors \(a\in \mathbb Z_q^n\) are uniform, and the group structure gives self-randomization. For P-TND, we need a replacement:

\[
\text{worst-case projected coboundary instance}
\quad\longrightarrow\quad
\text{average-looking noisy descent instance}.
\]

This suggests using complexes in **highly symmetric towers**, ideally with large automorphism/deck-transformation groups. Arithmetic quotient complexes are natural for this reason.

### Axiom 7: primal-dual symmetry

The KEM uses both

\[
t=As+e
\]

and ciphertext terms of the form

\[
u=A^\ast r+e_1.
\]

So decision hardness must hold for both \(A\) and \(A^\ast\). The object must be **bi-admissible**:

\[
\mathsf{P\text{-}TND}(A)
\quad\text{and}\quad
\mathsf{P\text{-}TND}(A^\ast)
\]

should both reduce to canonical worst-case problems.

This rules out many “one-sided expander” constructions.

---

## 3. Definition: a TND-admissible sheaf complex

We should now define the mathematical object first.

A public family

\[
\{(X_\lambda,\mathcal F_\lambda,\Pi_\lambda)\}_{\lambda}
\]

is **TND-admissible** if it satisfies:

\[
C^0 \xrightarrow{D=d^0} C^1 \xrightarrow{d^1} C^2
\]

with:

\[
\ker D=H^0
\]

and, after quotienting by \(H^0\),

\[
D:V\to C^1
\]

is injective and well-conditioned;

the pair \((X,\mathcal F)\) has coefficient cosystolic expansion in degrees \(0\) and \(1\);

the projection \(\Pi\) is a lossless sampler on \(\operatorname{im}(D)\);

the dual embedding

\[
\Pi^\ast:M^\ast\to (C^1)^\ast
\]

is transverse to \(\ker D^\ast\);

the family has enough symmetry or randomized covering structure for self-randomization;

all parameters are generated from public deterministic rules or public randomness with verifiable transcripts.

This is now a theorem target, not a vibe.

---

## 4. The proof skeleton

The desired paper would have the following theorem chain.

### Theorem 1: P-TND average-case hardness

Assume \((X_\lambda,\mathcal F_\lambda,\Pi_\lambda)\) is TND-admissible. Then any nonuniform QPT algorithm that distinguishes

\[
(A,As+e)
\]

from

\[
(A,u)
\]

with non-negligible advantage yields an algorithm for worst-case Nearest Projected Coboundary in the same family.

Symbolically:

\[
\boxed{
\mathsf{Decision\ P\text{-}TND}_{\mathrm{avg}}
\;\Longrightarrow\;
\mathsf{NPCoboundary}_{\mathrm{worst}}.
}
\]

This is the central theorem. It is the analogue of LWE’s average-to-worst-case theorem, but over projected descent complexes.

### Theorem 2: search/decision equivalence

For TND-admissible families, decision P-TND and search P-TND are equivalent up to polynomial loss.

This requires a self-randomization lemma:

\[
t=As+e
\quad\mapsto\quad
t+Ah= A(s+h)+e
\]

with \(h\) sampled so that \(s+h\) is randomized in \(V\).

### Theorem 3: CPA security

The Regev/Kyber-shaped encryption core is IND-CPA secure assuming primal and dual decision P-TND.

The algebra is:

\[
t=As+e,
\]

\[
u=A^\ast r+e_1,
\]

\[
v=\langle r,t\rangle+e_2+\operatorname{Encode}(\mu).
\]

The owner computes:

\[
v-\langle u,s\rangle
=
\operatorname{Encode}(\mu)
+
e_2+\langle r,e\rangle-\langle e_1,s\rangle.
\]

If this noise is below the decoding threshold, correctness follows.

### Theorem 4: CCA security

Apply an FO-style transform / implicit rejection to the CPA core.

This part is not the novel mathematics. It is standard KEM hygiene.

---

## 5. What HDX properties the proof demands

Now we can ask: **what kind of HDX could plausibly satisfy the TND-admissibility axioms?**

The proof demands at least four structural ingredients.

First, **local spectral expansion**, because we need global Laplacian gaps, concentration, and mixing.

Second, **cosystolic expansion**, because nearest-coboundary decoding needs large distance and local-to-global testability.

Third, **coefficient sheaf expansion**, because constant coefficients may leave too much low-dimensional invariant structure. Sheaves on HDX are already studied in connection with locally testable codes; First and Kaufman explicitly frame sheaves on high-dimensional expanders as a route to good 2-query LTCs via expanding sheaves and towers of coverings. citeturn670437view3

Fourth, **arithmetic or combinatorial symmetry**, because the reduction needs self-randomization, not just expansion.

This points to a specific first candidate.

---

## 6. First principled candidate: arithmetic Ramanujan complex tower

Use finite quotients of affine Bruhat–Tits buildings:

\[
X_N=\Gamma(I_N)\backslash \mathcal B,
\]

where \(\mathcal B\) is a building of type \(\widetilde A_d\), for example associated with \(\mathrm{PGL}_{d+1}(F)\).

Lubotzky–Samuels–Vishne give explicit constructions of Ramanujan complexes as quotients of Bruhat–Tits buildings associated with \(\mathrm{PGL}_d(F)\) over positive-characteristic local fields. citeturn359449search3 These are natural candidates because they are explicit, arithmetic, highly structured, and tied to strong spectral behavior.

Use dimension \(d\ge 2\), so we have at least:

\[
C^0 \xrightarrow{d^0} C^1 \xrightarrow{d^1} C^2.
\]

For proof convenience, \(d=2\) is the minimal nontrivial case. For stronger duality and coefficient-sheaf control, \(d=3\) may be better.

The coefficient sheaf should initially be written abstractly:

\[
\mathcal F_N.
\]

But the proof tells us what it must do:

\[
H^0(X_N,\mathcal F_N)\ \text{controlled},
\]

\[
H^1(X_N,\mathcal F_N)\ \text{controlled or zero},
\]

\[
\text{coefficient cosystolic expansion in degrees }0,1.
\]

So the right coefficient sheaf is not chosen aesthetically. It is chosen to satisfy the spectral/cosystolic axioms.

A sane first paper would prove results for an **axiomatic expanding sheaf** over such a complex, then instantiate later.

---

## 7. The projection is the most important missing object

The boundary operator alone is not enough.

If

\[
A=D,
\]

then low-dimensional topology may give low-weight dual relations. For example, local faces create local chain relations. That is a distinguisher.

So define the projection through its adjoint:

\[
J:=\Pi^\ast:M^\ast\hookrightarrow (C^1)^\ast.
\]

Think of \(J\) as a public code embedding. Then

\[
A^\ast=D^\ast J.
\]

The key requirement is:

\[
\boxed{
J(M^\ast)\cap \ker D^\ast
\text{ has no small nonzero vectors.}
}
\]

More strongly, for all nonzero \(r\),

\[
\|D^\ast J r\|\ge \beta \|r\|.
\]

This says \(J\) embeds the measurement dual space into 1-cochains **transversely to cycles**.

So the actual object is not merely an HDX. It is:

\[
\boxed{
(X,\mathcal F,J)
}
\]

where \(J\) is a **transverse Tanner/sheaf code embedding** into degree-1 cochains.

This is probably the right mathematical novelty.

---

## 8. Refined construction target

The proof-led object is:

\[
\boxed{
\textbf{Arithmetic HDX + expanding coefficient sheaf + transverse projection code.}
}
\]

More explicitly:

\[
X_N=\Gamma(I_N)\backslash \mathcal B_{\widetilde A_d}
\]

\[
\mathcal F_N=\text{public expanding coefficient sheaf}
\]

\[
D_N=d^0:C^0(X_N,\mathcal F_N)/H^0\to C^1(X_N,\mathcal F_N)
\]

\[
J_N:M_N^\ast\hookrightarrow (C^1(X_N,\mathcal F_N))^\ast
\]

\[
A_N^\ast=D_N^\ast J_N,
\qquad
A_N=J_N^\ast D_N.
\]

Then:

\[
\boxed{
t=A_Ns+e
}
\]

is the public key.

This is much more principled than “choose a random projection.”

---

## 9. Concrete algorithm after the proof axioms

The algorithm remains simple.

### Setup

\[
pp=(q,X_N,\mathcal F_N,J_N,\chi_s,\chi_e).
\]

Everything is generated from public seeds or deterministic arithmetic rules.

Define:

\[
A=J^\ast D.
\]

Do not store \(A\) densely. Store algorithms for:

\[
D,\ D^\ast,\ J,\ J^\ast.
\]

### KeyGen

Sample:

\[
s\leftarrow \chi_s^V,
\qquad
e\leftarrow \chi_e^M.
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

Sample \(\mu\), derive small randomness:

\[
r,e_1,e_2=G(\mu,pk).
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
\]

\[
ss=\operatorname{KDF}(\mu,pk,ct).
\]

### Decaps

Compute:

\[
w=v-\langle u,s\rangle.
\]

Decode:

\[
\mu'=\operatorname{Decode}(w).
\]

Recompute ciphertext deterministically from \(\mu'\). If it matches, output the real shared secret; otherwise output pseudorandom fallback.

This algorithm is implementable once \(D,D^\ast,J,J^\ast\) are implemented.

---

## 10. Proof-led HDX selection

Now the question “which HDX?” becomes precise.

We need a family satisfying these proof properties:

\[
\boxed{
\begin{array}{ll}
\textbf{P1} & \text{bounded degree, explicit public construction;}\\
\textbf{P2} & \text{local spectral expansion;}\\
\textbf{P3} & \text{cosystolic expansion in degrees }0,1;\\
\textbf{P4} & \text{coefficient-sheaf expansion;}\\
\textbf{P5} & \text{large symmetry / covering tower for self-randomization;}\\
\textbf{P6} & \text{existence of transverse projection code }J;\\
\textbf{P7} & \text{dual expansion for }A^\ast;\\
\textbf{P8} & \text{smoothing bounds for the associated sheaf lattices.}
\end{array}
}
\]

Ramanujan complexes are the natural first HDX candidate because they are explicit arithmetic quotients with strong spectral/cosystolic expansion literature. Evra–Kaufman’s cosystolic-expander result applies a local-to-global criterion to Ramanujan complexes, while Oppenheim’s local spectral framework supplies the spectral-gap side. citeturn129037search1turn670437view2

But the proof makes clear that **Ramanujan alone is not the full answer**. The missing proof object is \(J\), the transverse projection code.

So the right research target is:

\[
\boxed{
\textbf{Construct transverse projection codes over arithmetic HDX sheaf cochains.}
}
\]

That is much sharper than “try HDXs until one survives attacks.”

---

## 11. The first theorem to prove

The first nontrivial theorem should not mention KEMs.

It should be purely mathematical:

\[
\boxed{
\textbf{Transverse Projection Theorem.}
}
\]

Let

\[
X_N
\]

be an arithmetic HDX tower and \(\mathcal F_N\) an expanding coefficient sheaf. Then there exists an explicit public family of embeddings

\[
J_N:M_N^\ast\hookrightarrow (C^1(X_N,\mathcal F_N))^\ast
\]

such that:

\[
\|D_N^\ast J_N r\|\ge \beta\|r\|
\]

for every \(r\neq 0\), and

\[
J_N(M_N^\ast)
\]

is a lossless sampler/condenser for \(\operatorname{im}(D_N)\).

This theorem would eliminate the most obvious dual distinguishers and give the projection the right proof behavior.

Then the second theorem is the cryptographic one:

\[
\boxed{
\textbf{Average P-TND over }(X_N,\mathcal F_N,J_N)
\Rightarrow
\textbf{worst-case nearest projected coboundary.}
}
\]

Only after these two theorem statements are stable should implementation begin.

---

## 12. Why this is not speculative in the bad way

The speculative version is:

\[
\text{build object}\to\text{attack}\to\text{patch}\to\text{hope}.
\]

The proof-led version is:

\[
\text{state reduction}\to\text{derive axioms}\to\text{construct object satisfying axioms}.
\]

The object we now need is not “some HDX.” It is a **TND-admissible sheaf complex**.

The next mathematical work is therefore:

1. Prove the abstract P-TND security theorem assuming TND-admissibility.
2. Prove that arithmetic Ramanujan complex towers with suitable coefficient sheaves and transverse projection codes are TND-admissible.
3. Only then instantiate a KEM.

---

## 13. The refined research proposal

The project becomes:

\[
\boxed{
\begin{aligned}
&\textbf{Title: Transparent Noisy Descent from Expanding Sheaves on Arithmetic HDX}\\[1mm]
&\textbf{Goal: Build a KEM whose security reduces to worst-case nearest projected coboundary.}\\
\end{aligned}
}
\]

The core mathematical deliverable is:

\[
\boxed{
(X_N,\mathcal F_N,J_N)
\text{ is TND-admissible.}
}
\]

The core cryptographic deliverable is:

\[
\boxed{
\text{IND-CCA security of P-TND-KEM}
\Leftarrow
\text{worst-case Nearest Projected Coboundary}
}
\]

against nonuniform QPT adversaries.

The concrete candidate family is:

\[
\boxed{
\text{arithmetic Ramanujan complex towers}
}
\]

with:

\[
\boxed{
\text{expanding coefficient sheaves}
}
\]

and:

\[
\boxed{
\text{explicit transverse projection codes.}
}
\]

That is the principled version. The proof tells us exactly what the HDX must be: not merely a high-dimensional expander, but an arithmetic, coefficient-expanding, self-randomizing sheaf complex equipped with a projection code transverse to coboundary cycles.

## 47. User

I love this. Now lets put a bit more work into exploring the transverse projection code more explicitly . How do we design this embedding to the greatest effect?

## 48. Assistant

I’ll treat the transverse projection code as the central cochain-level object, deriving its proof obligations before proposing explicit constructions and failure tests.

## 49. Assistant

The refinement is to design \(J=\Pi^\ast\) as a sparse dual code inside \(C^1\), chosen via the desired distribution of \(D^\ast Jr\), not as a generic projection.

## 50. Assistant

Yes. The projection code is the **hinge**. The right way to design it is not to pick a random \(\Pi:C^1\to M\), but to design its adjoint

\[
J=\Pi^\ast:M^\ast\to C^1
\]

as a **code embedded into 1-cochains**.

Let

\[
V=C^0(X,\mathcal F)/H^0(X,\mathcal F),
\qquad
W=C^1(X,\mathcal F),
\qquad
D=d^0:V\to W.
\]

The public map is

\[
A=\Pi D=J^\ast D:V\to M.
\]

The dual map is

\[
A^\ast=D^\ast J:M^\ast\to V.
\]

So every dangerous dual relation is a vector

\[
r\in M^\ast
\]

such that

\[
A^\ast r=D^\ast Jr=0.
\]

Equivalently, \(Jr\) has landed in the **co-closed/cycle-like subspace**

\[
\ker D^\ast\subseteq C^1.
\]

Thus the projection-code problem is:

\[
\boxed{
\textbf{Embed }M^\ast\textbf{ into }C^1\textbf{ so that low-complexity vectors never land near }\ker D^\ast.
}
\]

That is the transverse projection problem.

---

## 1. The Hodge picture

The dangerous subspace is

\[
Z_{\mathrm{co}}:=\ker D^\ast.
\]

It contains the co-closed 1-cochains: cycles, harmonic components, and local relations depending on the complex. If \(J(M^\ast)\) intersects this subspace in low-weight vectors, the scheme is dead, because then for public keys

\[
t=As+e,
\]

a low-weight \(r\in\ker A^\ast\) gives

\[
\langle r,t\rangle=\langle r,e\rangle,
\]

which is small/noisy rather than uniform. That is an immediate distinguisher.

So we want \(J(M^\ast)\) to have **large angle** from \(\ker D^\ast\). Over finite fields, “large angle” becomes:

\[
\boxed{
\ker(D^\ast J)
\textbf{ has large minimum distance and no short }q\textbf{-ary lattice vectors.}
}
\]

This is stronger than ordinary rank. Rank only says no exact algebraic collapse. Cryptography needs no **small** collapse.

---

## 2. The optimal first design: exact-gradient projection

The cleanest transverse embedding is:

\[
\boxed{
J = D B
}
\]

where

\[
B:M^\ast\to V
\]

is an outer projection code.

Then

\[
A^\ast
=
D^\ast J
=
D^\ast D B
=
L_0B,
\]

where

\[
L_0=D^\ast D
\]

is the degree-0 sheaf Laplacian on \(V\).

If \(L_0\) is invertible on \(V=C^0/H^0\), then

\[
\ker A^\ast=\ker B.
\]

Also,

\[
A=J^\ast D=B^\ast D^\ast D=B^\ast L_0,
\]

so because \(L_0\) is invertible,

\[
A(V)=B^\ast(V).
\]

This is a very strong conclusion:

\[
\boxed{
\textbf{The exact-gradient embedding removes all cycle/cocycle leakage.}
}
\]

All primal and dual code properties of \(A\) become the corresponding code properties of the outer code \(B^\ast\). The HDX/sheaf complex supplies a **canonical exact subspace** \(\operatorname{im}D\subset C^1\), and we place the projection code entirely inside it.

This is the most effective transversality move.

The measurement vectors are

\[
J e_i = D b_i,
\]

where \(b_i\in V\) is the \(i\)-th column of \(B\). Public measurements are

\[
(\Pi y)_i
=
\langle D b_i,y\rangle.
\]

For honest descent data \(y=Ds\),

\[
(\Pi Ds)_i
=
\langle Db_i,Ds\rangle
=
\langle b_i,L_0s\rangle.
\]

So we are not measuring arbitrary edges. We are measuring **gradients against gradients**, equivalently Laplacian-filtered local trivializations.

This is the right baseline.

---

## 3. What this reveals

The exact-gradient construction proves something important:

\[
\boxed{
\textbf{The projection-code problem reduces to constructing a balanced outer code }B.
}
\]

The HDX alone does **not** save a bad projection. If \(B\) has low-weight kernel vectors, then \(A^\ast\) has low-weight kernel vectors. If the row space of \(B\) has low-weight codewords, then \(A(V)\) has low-weight codewords. The sheaf Laplacian cannot fix that, because it is invertible on \(V\).

So the projection code must be a **balanced code**, meaning both the code and its dual are good.

Let

\[
G:=B^\ast:V\to M.
\]

We want:

\[
\boxed{
C_{\mathrm{prim}}=G(V)\subseteq M
}
\]

to have large distance / good lattice smoothing, and

\[
\boxed{
C_{\mathrm{dual}}=\ker G^\ast=\ker B
}
\]

also to have large distance / good lattice smoothing.

A random linear code has this with high probability. A sparse LDPC-style parity matrix usually does **not** automatically have this, because its row space contains low-weight rows. That is a key warning.

---

## 4. The right object: a balanced transverse code

Define a **balanced transverse projection code** as a map

\[
B:M^\ast\to V
\]

such that \(G=B^\ast:V\to M\) satisfies:

\[
\operatorname{rank}G=\dim V,
\]

\[
d_H(G(V))\ge \delta m,
\]

\[
d_H(\ker G^\ast)\ge \delta^\perp m,
\]

and, more strongly, the associated \(q\)-ary lattices

\[
\Lambda_{\mathrm{prim}}
=
\{z\in\mathbb Z^m:z\bmod q\in G(V)\},
\]

\[
\Lambda_{\mathrm{dual}}
=
\{z\in\mathbb Z^m:G^\ast z\equiv 0\pmod q\}
\]

have no unexpectedly short vectors.

For proof purposes, distance alone is insufficient. We need **lattice smoothing** or at least strong anti-concentration:

\[
\Pr[A^\ast z=0\bmod q]
\]

must be tiny for every short integer vector \(z\), not merely every low-Hamming-weight vector.

So the projection code should satisfy:

\[
\boxed{
\textbf{balanced primal distance + balanced dual distance + no short }q\textbf{-ary lattice relations.}
}
\]

This is the actual design target.

---

## 5. Candidate outer codes \(B\)

There are four serious candidates.

### Candidate 1: random linear \(B\)

Take \(G=B^\ast\) as a uniformly random full-rank \(m\times n\) matrix over \(\mathbb F_q\), where

\[
n=\dim V.
\]

This gives the cleanest theorem. It is not compact, but it lets us prove the exact-gradient projection theorem without HDX-specific distractions.

With

\[
J=DB,
\]

the public map is

\[
A=GL_0.
\]

Since \(L_0\) is invertible,

\[
A(V)=G(V),
\qquad
\ker A^\ast=\ker G^\ast.
\]

So the cryptographic assumption becomes ordinary noisy decoding/LWE over a random code, but interpreted as noisy descent.

This should be the **control case**.

### Candidate 2: MDS / algebraic-geometric outer code

Let

\[
G:V\to \mathbb F_q^m
\]

be a Reed–Solomon or algebraic-geometric encoder. Then \(G(V)\) and its dual can both have strong Hamming-distance guarantees, and \(G\) can be represented succinctly.

This gives near-optimal transversality in Hamming terms. The danger is algebraic structure: noisy polynomial reconstruction, AG-code structure, or specialized decoding may produce attacks. So this is mathematically attractive but cryptographically suspicious.

Still, it is useful as a theorem testbed: if we cannot prove the desired properties for MDS-like \(G\), we do not yet understand the projection code.

### Candidate 3: rank-condenser / subspace-design outer code

This is closest to the proof obligations. We want \(G\) to preserve dimension and entropy of subspaces, not merely separate individual vectors.

Linear-algebraic pseudorandomness already studies objects like **rank condensers**, **dimension expanders**, **subspace designs**, and **subspace-evasive sets**; Forbes and Gur explicitly frame these as finite-field analogues where dimension plays the role of entropy, and Guruswami–Kopparty-style work develops lossless dimension/rank-expander constructions. citeturn278912search0turn278912search1

For us, the desired statement is:

\[
\dim G(U)\approx \dim U
\]

for every relevant subspace

\[
U\subseteq V,
\]

together with large support and no short dual lattice relations.

This suggests a projection code built from an explicit **lossless rank condenser** plus an additional distance-amplifying layer.

### Candidate 4: HDX/sheaf Tanner outer code

Use another expanding sheaf/Tanner construction to define \(G\). This is philosophically closest to the program.

First and Kaufman show a framework connecting good \(2\)-query locally testable codes with expanding sheaves on high-dimensional expanders, using towers of coverings; they define expanding sheaves and aim at constant-rate, linear-distance code families. citeturn270378search2

That is highly relevant, but there is a catch: many locally testable/Tanner codes have sparse local constraints, and sparse constraints can imply low-weight dual codewords. For our use, we need **both** \(G(V)\) and its dual to avoid low-weight structure.

So the HDX/sheaf-code route must be designed as a **balanced code**, not merely an LTC.

---

## 6. The exact-gradient projection is safe but maybe too conservative

The design

\[
J=DB
\]

is maximally transverse, but it also makes the HDX part less essential:

\[
A=B^\ast L_0.
\]

Since \(L_0\) is invertible, the code is essentially \(B^\ast(V)\). So the security is inherited from \(B\).

That is not a failure. It is an important baseline. But if we want the HDX/descent structure to contribute nontrivially, we should add a controlled perturbation.

---

## 7. The stronger design: tilted exact-gradient projection

Define

\[
\boxed{
J = DB + C\Delta
}
\]

where:

\[
B:M^\ast\to V
\]

is the balanced outer code,

\[
C:M^\ast\to C^1
\]

is an HDX-local cochain sampler,

and

\[
\Delta:M^\ast\to M^\ast
\]

is a public random diagonal scaling:

\[
\Delta e_i=\theta_i e_i,
\qquad
\theta_i\in\mathbb F_q^\times.
\]

Columnwise:

\[
J e_i
=
D b_i+\theta_i c_i.
\]

The measurement against \(y\in C^1\) is

\[
(\Pi y)_i
=
\langle Db_i+\theta_i c_i,\ y\rangle.
\]

For honest descent data \(y=Ds\),

\[
(\Pi Ds)_i
=
\langle b_i,L_0s\rangle
+
\theta_i\langle c_i,Ds\rangle.
\]

So each measurement has:

\[
\boxed{
\textbf{an exact-gradient safety component}
}
\]

plus

\[
\boxed{
\textbf{a genuine local descent-sampling component.}
}
\]

This is the design I like best.

The exact part prevents cycle leakage. The tilted part reintroduces HDX-native measurements without allowing them to dominate the security proof.

---

## 8. How to choose \(C\)

The map

\[
C:M^\ast\to C^1
\]

should be built from local patches of the HDX.

For each measurement coordinate \(i\), choose a small patch

\[
P_i\subseteq X_1
\]

such as:

\[
\operatorname{st}(v)\cap X_1,
\]

the boundary of a 2-cell,

a small ball in the 1-skeleton,

or a higher-dimensional local view determined by a face.

Then choose

\[
c_i\in C^1
\]

supported on \(P_i\), with coefficients from a small local inner code.

The patch family should be a **double sampler/agreement sampler**: it should sample large edge sets proportionally, and intersections of patches should also sample well. This is exactly the type of combinatorial behavior studied in HDX agreement expansion and double-sampler work; high-dimensional expanders have been used to derandomize direct-product/agreement tests, and double samplers abstract the relevant sampling behavior. citeturn270378search1turn270378search9

So \(C\) is not arbitrary sparse noise. It is a **local cochain sampler** satisfying:

\[
\operatorname{supp}(c_i)\text{ small},
\]

\[
D^\ast c_i\neq 0,
\]

\[
\{P_i\}\text{ samples every large support in }C^1,
\]

and combinations

\[
\sum_i r_i c_i
\]

do not become co-closed except through high-complexity \(r\).

---

## 9. The robust transversality lemma we want

The proof target for the tilted design is:

\[
J=DB+C\Delta.
\]

Then

\[
A^\ast r
=
D^\ast Jr
=
L_0Br+D^\ast C\Delta r.
\]

Suppose \(r\) is low-complexity.

If

\[
Br=0,
\]

then \(r\in\ker B\), so the outer dual-distance condition says \(r\) cannot be low-complexity.

If

\[
Br\neq 0,
\]

then

\[
L_0Br\neq 0
\]

because \(L_0\) is invertible. The only way to have

\[
A^\ast r=0
\]

is cancellation:

\[
D^\ast C\Delta r=-L_0Br.
\]

The random diagonal \(\Delta\) is designed to make this cancellation unlikely. For fixed \(r\), the left side depends on the independent public scalars \(\theta_i\) over \(\operatorname{supp}(r)\). If \(C\) has enough rank on that support, the cancellation probability is roughly

\[
q^{-\Omega(\operatorname{rank})}.
\]

Then union-bound over all low-complexity \(r\).

So the desired theorem is:

\[
\boxed{
\begin{aligned}
&\textbf{Robust Transversality Lemma:}\\
&\text{If }B\text{ is balanced and }C\text{ is an HDX local sampler,}\\
&\text{then with high probability over public }\Delta,\\
&\ker(D^\ast(DB+C\Delta))\text{ has no low-complexity vectors.}
\end{aligned}
}
\]

That is a concrete theorem we can attack.

The primal side is similar:

\[
A x
=
J^\ast D x
=
B^\ast L_0x+\Delta^\ast C^\ast D x.
\]

The base term

\[
B^\ast L_0x
\]

is a good codeword because \(B^\ast(V)\) is good and \(L_0\) is invertible. The tilted term should not cancel too many coordinates. Again, random diagonal scaling and sampler properties should control this.

---

## 10. The projection code should be designed in two layers

The most principled architecture is:

\[
\boxed{
J = \underbrace{DB}_{\text{exact-gradient safety}}
+
\underbrace{C\Delta}_{\text{HDX-local descent sampling}}.
}
\]

### Layer 1: exact-gradient safety

This layer eliminates topological/cycle attacks.

It requires:

\[
B:M^\ast\to V
\]

with both \(B^\ast(V)\) and \(\ker B\) good.

### Layer 2: HDX-local sampler

This layer prevents the system from collapsing into ordinary code-LWE and gives it descent geometry.

It requires:

\[
C:M^\ast\to C^1
\]

whose columns are local cochains supported on HDX patches.

### Layer 3: public diagonal randomization

This layer prevents algebraic cancellation between the exact part and local part:

\[
\Delta=\operatorname{diag}(\theta_1,\dots,\theta_m).
\]

The \(\theta_i\)’s are public; they are not a trapdoor. Their role is to make the bad-cancellation set small.

---

## 11. Why not choose \(J\) fully sparse?

A fully sparse \(J\) is tempting, because it makes computation fast.

But if \(J\) is too sparse, then \(A=J^\ast D\) may have:

\[
\text{short primal codewords}
\]

or

\[
\text{short dual relations}.
\]

That is exactly what we cannot allow.

Sparse LDPC-like matrices are good for error correction because they have efficient local checks. In this setting, efficient checks may become distinguishers. So the projection code should not be merely sparse. It should be **locally computable but globally dense in code behavior**.

That is why the two-layer design is useful. \(C\) can be sparse/local. But \(B\) must be balanced and code-like, not merely sparse.

---

## 12. A concrete version of \(B\)

For a first proof, take \(G=B^\ast\) to be a random linear encoder:

\[
G\in\mathbb F_q^{m\times n}.
\]

Then set

\[
B=G^\ast.
\]

This gives a theorem baseline.

For a more explicit construction, I would try:

\[
G = \text{balanced AG/rank-condenser encoder}
\]

or

\[
G = \text{HDX/sheaf Tanner encoder with dense local inner code}.
\]

The “dense local inner code” is important. A pure sparse Tanner code risks low-weight dual codewords. A dense local inner code can preserve local computability while improving dual distance.

Small-bias and \(k\)-wise independence tools are also relevant for derandomizing \(G\). Small-bias spaces provide pseudorandomness against linear tests with short seeds, and almost \(k\)-wise independent constructions can be built from them; these are exactly the kind of tools one would use to replace a fully random linear map by an explicit seeded family. citeturn278912search3turn278912search12

---

## 13. A concrete version of \(C\)

Let \(X\) be a 2- or 3-dimensional HDX.

Index measurements by a family of local patches:

\[
\mathcal P=\{P_i\}_{i=1}^m.
\]

For example:

\[
P_i=\partial \sigma_i
\]

for selected 2-faces \(\sigma_i\), or

\[
P_i=\operatorname{st}(\tau_i)\cap X_1
\]

for selected vertices/faces \(\tau_i\).

Choose an inner local code

\[
\mathcal L_i\subseteq C^1(P_i,\mathcal F)
\]

with no local co-closed words except zero, or more modestly with

\[
\|D^\ast c\|\ge \gamma\|c\|
\]

locally.

Then choose

\[
c_i\in \mathcal L_i
\]

publicly, perhaps from a small-bias source.

Set

\[
C e_i=c_i.
\]

The HDX requirements are:

\[
\{P_i\}\text{ is a sampler for supports of }Dx,
\]

\[
\{P_i\cap P_j\}\text{ controls correlations},
\]

\[
D^\ast C r\text{ has large support for low-complexity }r.
\]

This is where cosystolic and local spectral expansion enter. Evra–Kaufman prove bounded-degree cosystolic expanders using local-to-global criteria and Ramanujan complexes; this is the kind of expansion theorem we need on the \(D\)-side. citeturn270378search3

---

## 14. The clean theorem statement

The projection-code theorem should look like this.

Let

\[
(X,\mathcal F)
\]

be a sheaf complex with reduced Laplacian

\[
L_0=D^\ast D
\]

invertible on

\[
V=C^0/H^0.
\]

Let

\[
G=B^\ast:V\to M
\]

be a balanced outer code.

Let

\[
C:M^\ast\to C^1
\]

be an HDX local sampler.

Let

\[
\Delta=\operatorname{diag}(\theta_i)
\]

with public independent \(\theta_i\in\mathbb F_q^\times\).

Define

\[
J=DB+C\Delta,
\qquad
A=J^\ast D.
\]

Then with overwhelming probability over \(\Delta\):

\[
\operatorname{rank}A=\dim V,
\]

\[
d_H(A(V))\ge \delta' m,
\]

\[
d_H(\ker A^\ast)\ge \delta^{\perp\prime}m,
\]

and the associated primal and dual \(q\)-ary lattices have no vectors below the desired norm threshold.

That is the **transverse projection code theorem**.

The proof decomposes into:

\[
\text{exact-gradient base case}
\]

plus

\[
\text{random-diagonal no-cancellation}
\]

plus

\[
\text{HDX sampler concentration}.
\]

---

## 15. The design I would actually pursue

The strongest principled candidate is:

\[
\boxed{
J e_i = D b_i+\theta_i c_i.
}
\]

Here:

\[
b_i\in V
\]

comes from a balanced outer code \(B\), and

\[
c_i\in C^1
\]

comes from a local HDX patch.

Equivalently:

\[
\boxed{
J=DB+C\Delta.
}
\]

Then:

\[
\boxed{
A=J^\ast D
=
B^\ast L_0+\Delta^\ast C^\ast D.
}
\]

Interpretation:

\[
B^\ast L_0
=
\textbf{safe Laplacian-coded coordinate measurements},
\]

\[
\Delta^\ast C^\ast D
=
\textbf{local noisy descent measurements}.
\]

This gives us a projection that is **transverse by construction**, but still genuinely tied to descent geometry.

---

## 16. The main design principles

The final principles are these.

**Do not design \(\Pi\) directly.** Design

\[
J=\Pi^\ast.
\]

**Do not let \(J(M^\ast)\) wander freely in \(C^1\).** Anchor it in exact coboundaries:

\[
\operatorname{im}D.
\]

**Do not use a merely sparse outer code.** Use a balanced code whose primal and dual are both hard.

**Do not trust HDX locality alone.** Use the HDX-local component only as a controlled tilt from the exact-gradient safe baseline.

**Do not rely on heuristic transversality.** Prove low-complexity cancellation impossible via random diagonal scaling and sampler expansion.

The most compact summary is:

\[
\boxed{
\textbf{The projection should be an exact coboundary lift of a balanced code, plus a randomized HDX-local tilt.}
}
\]

Or in one formula:

\[
\boxed{
J=DB+C\Delta.
}
\]

That formula gives us a concrete object, a proof strategy, and a path to implementation.

## 51. User

see how far you can take the tilted exact-gradient projection approach. Make it as concrete as you can

## 52. Assistant

I’ll make \(J=DB+C\Delta\) the concrete primitive and push it through definitions, algorithms, security conditions, and proof obligations. The key issue is controlling short dual relations while preserving descent geometry.

## 53. Assistant

A finite-field correction matters: the exact anchor should use a public adjoint lift or weighted pairing so the reduced Laplacian is nonsingular, rather than assuming \(D^\ast D\) is invertible.

## 54. Assistant

The tilted exact-gradient construction can be made quite concrete. The clean version is not

\[
J=DB+C\Delta,
\]

but rather

\[
\boxed{
J=\widetilde D B+C\Delta
}
\]

where \(\widetilde D\) is a **dual coboundary lift** satisfying

\[
D^\ast \widetilde D=I.
\]

That correction matters. Over \(\mathbb F_q\), even if \(D\) is injective, \(D^\ast D\) need not be invertible. The right exact-gradient anchor is therefore not \(D\) itself, but a chosen public lift \(\widetilde D\) dual to \(D\).

Below is the most concrete version I see.

---

# 1. Algebraic setup

Work over

\[
\mathbb F_q.
\]

Let

\[
V=C^0(X,\mathcal F)/H^0(X,\mathcal F)\cong \mathbb F_q^n
\]

be gauge-fixed local trivializations, and let

\[
W=C^1(X,\mathcal F)\cong \mathbb F_q^N
\]

be 1-cochains, i.e. gluing/descent data. The sheaf coboundary is

\[
D=d^0:V\to W,
\]

represented as an \(N\times n\) full-column-rank matrix.

The HDX/sheaf background is not decorative: high-dimensional expanders give the local-to-global expansion structure we eventually need. Evra–Kaufman construct bounded-degree cosystolic expanders using a local-to-global criterion, Oppenheim’s local spectral expansion framework gives spectral gaps for links and global Laplacians, and First–Kaufman explicitly develop sheaves on high-dimensional expanders for locally testable codes. Those are exactly the kinds of ingredients we want for noisy descent. citeturn428081search0turn428081search1turn428081search2

Now choose a public left inverse

\[
R:W\to V
\]

such that

\[
RD=I_V.
\]

Equivalently, set

\[
\widetilde D=R^\ast:V^\ast\to W^\ast\cong W,
\]

so that

\[
D^\ast\widetilde D=I_V.
\]

In coordinates, if \(D\in\mathbb F_q^{N\times n}\), choose \(n\) rows \(T\) of \(D\) such that

\[
D_T\in\mathbb F_q^{n\times n}
\]

is invertible. Then define

\[
R=D_T^{-1}P_T,
\]

where \(P_T:W\to\mathbb F_q^n\) selects those \(n\) coordinates. Then

\[
RD=D_T^{-1}P_TD=D_T^{-1}D_T=I.
\]

For a constant sheaf on a connected graph, \(T\) is just a spanning tree after fixing one vertex gauge. For a local-system sheaf, \(T\) is a sheaf-theoretic spanning tree/cobasis.

---

# 2. Measurement space split: guard block plus tilted block

Let the measurement space be

\[
M=M_0\oplus M_1
\]

with dimensions

\[
\dim M_0=m_0,\qquad \dim M_1=m_1,\qquad m=m_0+m_1.
\]

The **guard block** is pure exact-gradient measurement. The **tilted block** is exact-gradient measurement plus local HDX descent measurement.

The adjoint projection code is

\[
J:M^\ast\to W.
\]

Write an element of \(M^\ast\) as

\[
r=(r_0,r_1),\qquad r_0\in M_0^\ast,\quad r_1\in M_1^\ast.
\]

Define

\[
\boxed{
J(r_0,r_1)
=
\widetilde D B_0 r_0
+
\widetilde D B_1 r_1
+
C\Delta r_1.
}
\]

Equivalently,

\[
\boxed{
J=
\begin{bmatrix}
\widetilde D B_0 & \widetilde D B_1+C\Delta
\end{bmatrix}.
}
\]

Here:

\[
B_0:M_0^\ast\to V,
\]

\[
B_1:M_1^\ast\to V
\]

are columns of a balanced outer code

\[
B=\begin{bmatrix}B_0&B_1\end{bmatrix}:M^\ast\to V,
\]

\[
C:M_1^\ast\to W
\]

is a local HDX patch sampler, and

\[
\Delta:M_1^\ast\to M_1^\ast
\]

is a public diagonal random scaling.

The actual public map used by the KEM is

\[
A=J^\ast D:V\to M.
\]

Using

\[
D^\ast\widetilde D=I,
\]

we get

\[
\boxed{
A
=
\begin{bmatrix}
B_0^\ast\\
B_1^\ast+\Delta^\ast C^\ast D
\end{bmatrix}.
}
\]

So for \(s\in V\),

\[
\boxed{
A(s)
=
\left(
B_0^\ast s,\;
B_1^\ast s+\Delta^\ast C^\ast D s
\right).
}
\]

The dual map is

\[
\boxed{
A^\ast(r_0,r_1)
=
B_0r_0+B_1r_1+D^\ast C\Delta r_1.
}
\]

This is the central formula.

The guard block protects distance and rank. The tilted block injects genuine noisy descent geometry.

---

# 3. Choose \(B\) as a Cauchy MDS disperser

For maximal concreteness, take \(B\) to be a Cauchy matrix.

Choose distinct field elements

\[
x_1,\ldots,x_n,\quad \alpha_1,\ldots,\alpha_m\in\mathbb F_q,
\]

with

\[
q>n+m.
\]

Define

\[
\boxed{
B_{j,i}=\frac{1}{x_j-\alpha_i}.
}
\]

Split its columns:

\[
B=\begin{bmatrix}B_0&B_1\end{bmatrix}
\]

where \(B_0\) contains the first \(m_0\) columns and \(B_1\) contains the remaining \(m_1\).

This is a very strong choice because Cauchy matrices are totally nonsingular: every square minor is nonzero. Concretely, if \(r\in\mathbb F_q^m\) has Hamming weight

\[
t=\operatorname{wt}(r)\le n,
\]

then

\[
\boxed{
\operatorname{wt}(Br)\ge n-t+1.
}
\]

Proof sketch: \(Br\) is the evaluation at \(x_1,\ldots,x_n\) of the rational function

\[
f(z)=\sum_{i\in \operatorname{supp}r}\frac{r_i}{z-\alpha_i}.
\]

After clearing denominators, the numerator has degree at most \(t-1\), so it has at most \(t-1\) zeros.

Similarly, for any nonzero \(s\in\mathbb F_q^n\),

\[
\boxed{
\operatorname{wt}(B_0^\ast s)\ge m_0-n+1.
}
\]

Thus \(B_0^\ast(V)\subseteq \mathbb F_q^{m_0}\) is an MDS code of parameters

\[
[m_0,n,m_0-n+1]_q.
\]

This is why the guard block is powerful: no matter what the tilted block does, the public codeword

\[
A(s)
\]

has at least

\[
m_0-n+1
\]

nonzero guard coordinates for every \(s\ne 0\).

So the primal distance is protected **deterministically**.

---

# 4. Choose \(C\) as an HDX-local wedge sampler

Now define the tilted local part.

For each tilted coordinate \(i=1,\ldots,m_1\), choose a small local patch in \(X\). In the simplest constant-sheaf case, use a 2-simplex

\[
\sigma_i=(v_0,v_1,v_2)
\]

and take a **wedge**, not the full boundary:

\[
(v_0,v_1),\qquad (v_0,v_2).
\]

Choose public nonzero coefficients

\[
a_i,b_i\in\mathbb F_q^\times
\]

with

\[
a_i+b_i\ne 0.
\]

Define a 1-cochain \(c_i\in W=C^1\) by

\[
c_i(v_0,v_1)=a_i,
\]

\[
c_i(v_0,v_2)=b_i,
\]

and zero elsewhere.

Then for \(s\in C^0\),

\[
c_i^\ast Ds
=
a_i(s_{v_1}-s_{v_0})+b_i(s_{v_2}-s_{v_0}).
\]

The divergence

\[
h_i=D^\ast c_i
\]

has support on at most three vertices:

\[
h_i(v_1)=a_i,
\]

\[
h_i(v_2)=b_i,
\]

\[
h_i(v_0)=-(a_i+b_i).
\]

Thus

\[
\boxed{
\operatorname{wt}(h_i)\le L=3.
}
\]

For a sheaf with fiber dimension \(k\), this becomes block support at at most three vertices, so the coordinate support is at most \(3k\). The block-support version is the right one mathematically.

Collect the \(c_i\)’s as columns:

\[
C=
\begin{bmatrix}
c_1&\cdots&c_{m_1}
\end{bmatrix}:M_1^\ast\to W.
\]

Then

\[
H:=D^\ast C: M_1^\ast\to V
\]

has columns

\[
h_i=D^\ast c_i
\]

with support size at most \(L\).

The HDX requirement is that the selected wedges form a good local sampler: patches should cover the complex evenly, have controlled overlap, and make the columns \(h_i\) rank-expand on small sets. This is where local spectral/cosystolic expansion enters. Explicit Ramanujan-complex towers are natural candidate base complexes because Lubotzky–Samuels–Vishne construct Ramanujan complexes as quotients of Bruhat–Tits buildings for \(\mathrm{PGL}_d\). citeturn428081search3

---

# 5. Public diagonal tilt

Let

\[
\Delta=\operatorname{diag}(\theta_1,\ldots,\theta_{m_1}),
\]

where each

\[
\theta_i\in\mathbb F_q^\times
\]

is public and sampled uniformly from a public seed.

The tilted rows of \(A\) are

\[
\boxed{
a_{m_0+i}^\ast
=
b_{m_0+i}^\ast+\theta_i c_i^\ast D.
}
\]

So the public map is:

\[
\boxed{
A(s)
=
\left(
B_0^\ast s,\;
B_1^\ast s+\Delta C^\ast D s
\right).
}
\]

This is extremely implementable.

You never need to materialize \(J\). For KEM operations, you only need:

\[
A_{\mathrm{mul}}(s)=A(s),
\]

\[
A_{\mathrm{T,mul}}(r)=A^\ast r.
\]

Explicitly:

\[
\boxed{
A_{\mathrm{mul}}(s)
=
\left(
B_0^\ast s,\;
B_1^\ast s+\Delta C^\ast D s
\right).
}
\]

\[
\boxed{
A_{\mathrm{T,mul}}(r_0,r_1)
=
B_0r_0+B_1r_1+D^\ast C\Delta r_1.
}
\]

---

# 6. Deterministic transversality theorem

Here is the first theorem we can actually prove.

Let

\[
A=
\begin{bmatrix}
B_0^\ast\\
B_1^\ast+\Delta C^\ast D
\end{bmatrix},
\]

where \(B\) is Cauchy and every column of

\[
H=D^\ast C
\]

has support at most \(L\).

Then:

## Theorem: guarded tilted transversality

Assume \(m_0\ge n\). Then:

\[
\operatorname{rank} A=n.
\]

Moreover, the primal code

\[
A(V)\subseteq \mathbb F_q^{m_0+m_1}
\]

has minimum Hamming distance at least

\[
\boxed{
d_{\mathrm{prim}}\ge m_0-n+1.
}
\]

Finally, the dual code

\[
\ker A^\ast\subseteq \mathbb F_q^{m_0+m_1}
\]

has no nonzero vector of Hamming weight

\[
t<\frac{n+1}{L+1}.
\]

Equivalently,

\[
\boxed{
d_{\mathrm{dual}}> \frac{n+1}{L+1}.
}
\]

For the wedge construction with \(L=3\),

\[
\boxed{
d_{\mathrm{dual}}> \frac{n+1}{4}.
}
\]

### Proof sketch

Rank and primal distance are immediate from the guard block. Since

\[
B_0^\ast
\]

generates an MDS code of dimension \(n\) and length \(m_0\),

\[
B_0^\ast s\ne 0
\]

for \(s\ne 0\), and

\[
\operatorname{wt}(B_0^\ast s)\ge m_0-n+1.
\]

For dual transversality, suppose

\[
A^\ast(r_0,r_1)=0.
\]

Then

\[
B_0r_0+B_1r_1+D^\ast C\Delta r_1=0.
\]

Let

\[
r=(r_0,r_1),
\]

and let

\[
t=\operatorname{wt}(r).
\]

Because \(B\) is Cauchy,

\[
\operatorname{wt}(Br)\ge n-t+1
\]

for \(t\le n\).

Meanwhile,

\[
D^\ast C\Delta r_1
\]

is a linear combination of at most

\[
\operatorname{wt}(r_1)
\]

columns, each supported on at most \(L\) coordinates. Therefore

\[
\operatorname{wt}(D^\ast C\Delta r_1)\le L\operatorname{wt}(r_1)\le Lt.
\]

For cancellation to occur,

\[
Br=-D^\ast C\Delta r_1,
\]

so the support of \(Br\) must fit inside the support of the local term. Therefore

\[
n-t+1\le Lt.
\]

Equivalently,

\[
t\ge \frac{n+1}{L+1}.
\]

So no smaller-support dual relation exists.

This is a genuine, non-heuristic transversality statement.

---

# 7. Why the guard block is essential

Without the guard block, additive tilting can destroy primal distance by accidental cancellation:

\[
B_1^\ast s+\Delta C^\ast Ds.
\]

The guard block prevents that. It gives a protected exact-gradient component whose distance is independent of \(C\) and \(\Delta\).

So the fully principled construction is not pure additive tilt. It is:

\[
\boxed{
\textbf{guarded additive tilt.}
}
\]

The measurement rows are:

\[
\text{guard rows:}\qquad a_i^\ast=b_i^\ast,
\]

\[
\text{tilted rows:}\qquad a_i^\ast=b_i^\ast+\theta_i c_i^\ast D.
\]

The exact-gradient anchor supplies provable transversality. The tilted rows inject HDX-local descent geometry.

---

# 8. Concrete parameter template

Let

\[
n=\dim V.
\]

Choose

\[
m_0=n+\mu,
\]

so the guard block gives primal distance

\[
d_{\mathrm{prim}}\ge \mu+1.
\]

Choose wedge patches, so

\[
L=3,
\]

giving dual small-support distance

\[
d_{\mathrm{dual}}> \frac{n+1}{4}.
\]

A reasonable research-scale template is:

\[
m_0=\frac54 n,
\]

\[
m_1=\frac12 n,
\]

so

\[
m=\frac74 n.
\]

Then:

\[
d_{\mathrm{prim}}\gtrsim \frac14 n,
\]

\[
d_{\mathrm{dual}}\gtrsim \frac14 n.
\]

Pick a prime

\[
q>n+m
\]

so the Cauchy matrix exists. For example, if

\[
n=1024,\qquad m_0=1280,\qquad m_1=512,
\]

then

\[
m=1792,
\]

and any prime

\[
q>2816
\]

works. Practical choices might be \(q=4093,7681,12289\), depending on noise and correctness margins.

This gives public key size approximately

\[
m\log_2 q.
\]

For \(n=1024,m=1792,q=12289\), that is roughly

\[
1792\cdot 14\approx 25088\text{ bits}\approx 3.1\text{ KB}.
\]

That is not ECC-sized, but it is in the plausible post-quantum KEM range.

---

# 9. Fast multiplication by the Cauchy block

Naively,

\[
B^\ast s
\]

costs \(O(nm)\). But because \(B\) is Cauchy, multiplication is structured.

For

\[
B^\ast s,
\]

the \(i\)-th coordinate is

\[
(B^\ast s)_i
=
\sum_{j=1}^n \frac{s_j}{x_j-\alpha_i}.
\]

This is evaluation at \(\alpha_i\) of a rational function with poles at \(x_j\). It can be computed with fast multipoint-evaluation machinery in quasi-linear time. For a first prototype, \(O(nm)\) is fine. For a real implementation, use Cauchy/Vandermonde fast transforms.

Likewise,

\[
Br
\]

has coordinates

\[
(Br)_j
=
\sum_{i=1}^m \frac{r_i}{x_j-\alpha_i}.
\]

Again, this is fast rational evaluation.

The HDX-local term is sparse:

\[
C^\ast Ds
\]

costs roughly

\[
O(m_1\cdot \text{patch size}).
\]

With wedge patches, patch size is \(2\) edges, so this is tiny.

---

# 10. Concrete algorithms

## Setup

```text
Setup(λ):

    Choose q, n, m0, m1 with q > n + m0 + m1.

    Construct a finite sheaf complex:
        V = C^0(X,F)/H^0
        W = C^1(X,F)
        D : V -> W

    Choose a public cobasis T of n rows of D.
        D_T must be invertible.

    Define:
        R = D_T^{-1} P_T
        so R D = I.

    Choose distinct field elements:
        x_1,...,x_n
        α_1,...,α_m

    Define Cauchy matrix:
        B[j,i] = 1 / (x_j - α_i)

    Split:
        B0 = first m0 columns
        B1 = last m1 columns

    For i = 1..m1:
        choose an HDX-local wedge patch P_i
        choose coefficients a_i,b_i
        define c_i ∈ C^1 supported on P_i

    Let C = [c_1 ... c_m1].

    Choose public diagonal:
        Δ = diag(θ_1,...,θ_m1), θ_i ∈ F_q^×

    Define public multiplication oracles:
        A_mul(s)  = (B0^T s, B1^T s + Δ C^T D s)
        AT_mul(r0,r1) = B0 r0 + B1 r1 + D^T C Δ r1
```

No trapdoor. Every object is generated from public seeds.

## Projection on arbitrary 1-cochains

If we want the literal projected descent map

\[
\Pi:W\to M,
\]

use

\[
\boxed{
\Pi(y)
=
\left(
B_0^\ast Ry,\;
B_1^\ast Ry+\Delta C^\ast y
\right).
}
\]

Then for exact descent data \(y=Ds\),

\[
\Pi(Ds)
=
\left(
B_0^\ast s,\;
B_1^\ast s+\Delta C^\ast Ds
\right)
=
A(s).
\]

This is the clean descent interpretation:

\[
\boxed{
\Pi = \text{exact reconstruction through }R \text{ plus local tilted descent tests.}
}
\]

## Key generation

```text
KeyGen(pp):

    s ← small distribution on V
    e ← small distribution on M0 ⊕ M1

    t = A_mul(s) + e mod q

    pk = t
    sk = s

    return pk, sk
```

## Encapsulation core

```text
Encaps(pp, pk):

    μ ← random seed

    derive small r0, r1, e1, e2 from μ and pk

    u = AT_mul(r0,r1) + e1

    v = <(r0,r1), pk> + e2 + Encode(μ)

    ss = KDF(μ, pk, u, v)

    return ct=(u,v), ss
```

## Decapsulation core

```text
Decaps(pp, sk=s, ct=(u,v)):

    w = v - <u,s>

    μ' = Decode(w)

    rederive r0', r1', e1', e2' from μ' and pk

    recompute ct'

    if ct' == ct:
        return KDF(μ', pk, ct)
    else:
        return pseudorandom fallback
```

Correctness uses the usual cancellation:

\[
\langle (r_0,r_1),As\rangle
=
\langle A^\ast(r_0,r_1),s\rangle.
\]

So

\[
v-\langle u,s\rangle
=
\operatorname{Encode}(\mu)
+
e_2
+
\langle (r_0,r_1),e\rangle
-
\langle e_1,s\rangle.
\]

Noise parameters are chosen so this decodes correctly with negligible failure probability.

---

# 11. What \(\Delta\) buys beyond the deterministic theorem

The deterministic theorem already gives good Hamming transversality. The diagonal \(\Delta\) is for stronger anti-cancellation statements.

Let

\[
H=D^\ast C.
\]

Then

\[
A^\ast(r_0,r_1)
=
B_0r_0+B_1r_1+H\Delta r_1.
\]

For fixed \(r_0,r_1\), the tilted term is

\[
H\Delta r_1
=
\sum_{i\in\operatorname{supp}r_1}
\theta_i r_{1,i} h_i.
\]

If the columns

\[
\{h_i:i\in S\}
\]

have rank \(\rho(S)\), then as \(\Delta\) varies, the possible tilted sums occupy a subspace of dimension \(\rho(S)\). For a fixed target vector, the collision probability is roughly

\[
q^{-\rho(S)}.
\]

So the next theorem target is:

\[
\boxed{
\operatorname{rank}\{h_i:i\in S\}\ge \kappa |S|
\quad
\text{for all } |S|\le \tau n.
}
\]

This is the **HDX patch-rank expansion** property. It is exactly where the HDX/local-sheaf geometry should be used.

The deterministic support bound handles very low-complexity attacks. The random diagonal plus patch-rank expansion should handle medium-complexity cancellation.

---

# 12. The HDX property we now need

The wedge sampler \(C\) should satisfy three concrete properties.

First, **bounded locality**:

\[
\operatorname{wt}(D^\ast c_i)\le L.
\]

For wedge patches, \(L=3\) in block support.

Second, **patch-rank expansion**:

\[
\operatorname{rank}\{D^\ast c_i:i\in S\}
\ge \kappa |S|
\]

for all

\[
|S|\le \tau n.
\]

Third, **sampling regularity**: every large subset of vertices/edges is hit by approximately the expected number of wedges, and overlaps are controlled.

The HDX literature gives plausible routes to these properties: cosystolic expansion gives local-to-global cochain expansion, local spectral expansion gives Laplacian/link gaps, and sheaves on HDX already supply a code-theoretic framework where local tests produce global distance. citeturn428081search0turn428081search1turn428081search2

So the refined mathematical target is not merely “find an HDX.” It is:

\[
\boxed{
\textbf{Construct an HDX wedge sampler whose divergences form a local rank expander.}
}
\]

That is a crisp object.

---

# 13. The three operating modes

There are three versions of the construction.

## Conservative mode

Use a large guard block:

\[
m_0=n+\Theta(n),
\]

and a moderate tilted block:

\[
m_1=\Theta(n).
\]

Security has a robust exact-gradient fallback. This is the version I would try to prove first.

## Balanced mode

Use a smaller guard block:

\[
m_0=n+O(\lambda),
\]

and rely more heavily on the tilted block plus patch-rank expansion. This gives smaller public keys, but the proof is harder.

## Aggressive mode

Set

\[
m_0=0
\]

and use pure additive tilt:

\[
A=B_1^\ast+\Delta C^\ast D.
\]

This is elegant but too dangerous as the first target. There is no protected primal distance. I would not start here.

The **conservative guarded mode** is the right proof-first design.

---

# 14. What remains genuinely open

The deterministic transversality theorem is already within reach. The remaining hard parts are:

**First**, extend Hamming transversality to Lee/Euclidean \(q\)-ary lattice transversality. A low-Hamming dual relation is only one kind of distinguisher. We need to exclude short integer vectors

\[
z\in\mathbb Z^m
\]

such that

\[
A^\ast z\equiv 0\pmod q
\]

and

\[
\|z\|
\]

is small.

**Second**, prove patch-rank expansion for the chosen HDX wedge sampler.

**Third**, prove average-case P-TND hardness from worst-case nearest projected coboundary. This is the analogue of the LWE average-to-worst-case theorem, but now over projected descent complexes.

**Fourth**, analyze whether Cauchy/MDS structure introduces algebraic attacks. Cauchy \(B\) is perfect for transversality, but algebraic structure is always a cryptanalytic liability. If this becomes problematic, replace \(B\) by a more pseudorandom balanced code. Linear-algebraic pseudorandomness, such as rank condensers and dimension expanders, is a natural derandomization toolkit here; Forbes–Guruswami explicitly frame these as finite-field analogues where rank/dimension plays the role of entropy. citeturn534174search1 Small-bias spaces are another tool for public-seed derandomization against linear tests. citeturn534174search4

But I would begin with Cauchy \(B\), because it gives clean algebraic theorems immediately.

---

# 15. The proposal in one formula

The concrete projection code is:

\[
\boxed{
J=
\begin{bmatrix}
\widetilde D B_0
&
\widetilde D B_1+C\Delta
\end{bmatrix},
}
\]

with

\[
D^\ast\widetilde D=I.
\]

The actual public map is:

\[
\boxed{
A=J^\ast D
=
\begin{bmatrix}
B_0^\ast\\
B_1^\ast+\Delta C^\ast D
\end{bmatrix}.
}
\]

Choose:

\[
B=\text{Cauchy MDS disperser},
\]

\[
C=\text{HDX-local wedge sampler},
\]

\[
\Delta=\text{public random diagonal}.
\]

Then we get, already:

\[
\boxed{
d_{\mathrm{prim}}\ge m_0-n+1
}
\]

and

\[
\boxed{
d_{\mathrm{dual}}> \frac{n+1}{L+1}.
}
\]

With wedge patches,

\[
L=3,
\]

so

\[
\boxed{
d_{\mathrm{dual}}> \frac{n+1}{4}.
}
\]

This is a very concrete object: it has implementable multiplication routines, explicit parameter knobs, an initial deterministic theorem, and clear next proof obligations.

The clean research target is now:

\[
\boxed{
\textbf{Prove that guarded Cauchy-MDS exact-gradient projections with HDX wedge tilts yield P-TND instances whose average-case hardness reduces to worst-case nearest projected coboundary.}
}
\]

That is the point where this becomes a real cryptographic research program rather than an analogy.

## 55. User

excellent progress. Now develop the HDX wedge sampler to meet the bounded locality, patch-rank expansion, and simpling regularity requirements.

## 56. Assistant

I’ll define the wedge sampler as its own combinatorial object, derive locality/rank/regularity axioms from the proof, then compare probabilistic and explicit HDX constructions.

## 57. Assistant

The refinement is to analyze \(H=D^\ast C\): a sparse divergence matrix whose columns come from local wedges. Rank expansion should follow from Hall/unique-neighbor properties of its support hypergraph.

## 58. Assistant

The rank problem reduces to the divergence matrix \(H=D^\ast C\): design bounded-support columns with Hall/unique-neighbor expansion, then generic coefficients yield rank by pivoting.

## 59. Assistant

Yes. The wedge sampler can be made much sharper. The right object is a **fiber-boosted, typed, public-thinned wedge system** on a spectral/cosystolic HDX.

The key move is to stop asking scalar wedges to do too much. A scalar wedge gives a \(3\)-sparse divergence column. That can work, but rank expansion is fragile. Instead, give each vertex a small coefficient fiber \(\mathbb F_q^k\). Then a wedge column still touches only **three vertices**, but it carries a \(k\)-dimensional local vector at each touched vertex. Patch-rank expansion becomes a **capacitated peeling** statement rather than a brittle unique-neighbor statement.

The sampler should therefore be:

\[
\boxed{
\textbf{typed HDX wedges}
+
\textbf{public sparse thinning}
+
\textbf{local }k\textbf{-wise independent vertex fibers}.
}
\]

This gives us a concrete path to the three requirements: **bounded locality**, **patch-rank expansion**, and **sampling regularity**.

---

## 1. Base HDX object

Use a pure, type-regular, \(d\)-dimensional simplicial complex

\[
X
\]

with a type map

\[
\tau:X(0)\to \{0,\ldots,d\}.
\]

For first implementation, \(d=2\) is enough. For stronger sampler regularity, \(d\ge 3\) is preferable because high-order walks and double-sampler arguments become more useful.

The base complex should be a **local spectral/cosystolic HDX**. Evra–Kaufman give bounded-degree cosystolic expanders in every dimension via a local-to-global criterion applied to Ramanujan complexes; Oppenheim’s local spectral framework shows that local spectral expansion gives spectral gaps in links and global Laplacians; First–Kaufman explicitly develop sheaves on HDXs and define expanding sheaves for code constructions. These are the exact existing ingredients our proof wants. citeturn872745view1turn548183view4turn872745view0

For an explicit arithmetic family, Chevalley/coset HDXs are attractive: O’Donnell–Pratt construct strongly explicit spectral HDX families from Chevalley groups, and their construction has a group action simply transitive on top-dimensional faces, which is useful for transparent enumeration and regularity. citeturn548183view2

---

## 2. Wedge universe

Pick an oriented type triple

\[
\theta=(a;b,c)
\]

with \(a,b,c\) distinct. For every typed \(2\)-face

\[
\sigma=\{v_a,v_b,v_c\}
\]

with

\[
\tau(v_a)=a,\qquad \tau(v_b)=b,\qquad \tau(v_c)=c,
\]

define one wedge

\[
w=(v_a;v_b,v_c).
\]

The patch is the two-edge star

\[
P_w=\{(v_a,v_b),(v_a,v_c)\}.
\]

So the wedge universe is

\[
\Omega_\theta
=
\{(v_a;v_b,v_c):\{v_a,v_b,v_c\}\in X(2),\ \tau=(a,b,c)\}.
\]

Use **one wedge per typed triangle**, not all three possible anchors. This avoids immediate local linear dependencies. For a scalar constant sheaf, all divergences supported on one triangle lie in a \(2\)-dimensional zero-sum subspace; including three anchored wedges on the same triangle creates a local relation. One anchor per typed triangle is the safe default.

If type balance is needed, choose a small set of oriented type triples

\[
\Theta=\{(a;b,c)\}
\]

with no unordered triple repeated too often. For example, in dimension \(d\ge 4\), use cyclic triples

\[
(a;a+1,a+2)\pmod{d+1}.
\]

Then

\[
\Omega=\bigcup_{\theta\in\Theta}\Omega_\theta.
\]

---

## 3. Coefficient sheaf and wedge column

Start with the constant coefficient sheaf

\[
\mathcal F(v)=\mathbb F_q^k
\]

on vertices and

\[
\mathcal F(e)=\mathbb F_q^k
\]

on edges, with identity restrictions. Then

\[
C^0(X,\mathcal F)\cong (\mathbb F_q^k)^{X(0)},
\]

\[
C^1(X,\mathcal F)\cong (\mathbb F_q^k)^{X(1)}.
\]

For an oriented edge \(e=(u,v)\),

\[
(Ds)_e=s_v-s_u.
\]

For a wedge

\[
w=(a;b,c),
\]

choose two public vectors

\[
x_w,y_w\in\mathbb F_q^k.
\]

Define a \(1\)-cochain column

\[
c_w\in C^1(X,\mathcal F)
\]

by

\[
c_w(a,b)=x_w,
\]

\[
c_w(a,c)=y_w,
\]

and zero elsewhere.

Then its divergence is

\[
h_w=D^\ast c_w\in C^0(X,\mathcal F),
\]

with block values

\[
h_w(a)=-(x_w+y_w),
\]

\[
h_w(b)=x_w,
\]

\[
h_w(c)=y_w,
\]

and zero elsewhere.

So each wedge has:

\[
\boxed{
\operatorname{block\ support}(c_w)=2\text{ edges}
}
\]

and

\[
\boxed{
\operatorname{block\ support}(h_w)=3\text{ vertices}.
}
\]

The coordinate support is at most \(2k\) in \(C^1\) and \(3k\) in \(C^0\), still constant for fixed \(k\).

This is the bounded-locality property.

---

## 4. Public thinning

Do not use all wedges. Use a public thinning to get a linear-size sampler with controllable density.

Let

\[
N=|X(0)|.
\]

Choose a target tilted-block size

\[
m_1=\rho N
\]

for a constant \(\rho\), say initially

\[
0.25\le \rho\le 1.
\]

Generate a public seed. For every wedge \(w\in\Omega\), include it if

\[
\operatorname{Hash}(\text{seed},w)<p
\]

where

\[
p\approx \frac{\rho N}{|\Omega|}.
\]

Let the selected wedge set be

\[
\mathcal W\subseteq \Omega.
\]

This is not a heuristic patch. It gives a public distribution over samplers. The theorem target should say that, except with negligible probability over the public seed, \(\mathcal W\) satisfies the wedge-sampler axioms.

If exact size matters, sort the accepted wedges by hash and take the first \(m_1\). This preserves transparency.

---

## 5. Local fiber design

At each vertex \(v\), define the local incident wedge set

\[
\mathcal W(v)=\{w\in\mathcal W:v\in w\}.
\]

For every incidence \((v,w)\), the divergence column contributes a vector

\[
g_{v,w}=h_w(v)\in \mathbb F_q^k.
\]

We want the local vectors at each vertex to be **\(k\)-wise independent**:

\[
\boxed{
\text{for every }v,\text{ every subset }T\subseteq\mathcal W(v)
\text{ with }|T|\le k,
\{g_{v,w}:w\in T\}\text{ is linearly independent}.
}
\]

This can be generated publicly. Since \(|\mathcal W(v)|\) is bounded or small, choose distinct field elements

\[
\xi_{v,w}\in\mathbb F_q
\]

and assign local MDS/Vandermonde vectors

\[
g_{v,w}=(1,\xi_{v,w},\xi_{v,w}^2,\ldots,\xi_{v,w}^{k-1}).
\]

For a wedge \(w=(a;b,c)\), we need

\[
h_w(a)+h_w(b)+h_w(c)=0
\]

in the constant-sheaf model. So choose

\[
g_{b,w},g_{c,w}
\]

from the local MDS sets and set

\[
g_{a,w}=-(g_{b,w}+g_{c,w}).
\]

Then verify locally that the resulting anchor vectors also satisfy \(k\)-wise independence at each anchor vertex. This verification is cheap because the selected degree is constant or logarithmic. If needed, resample the public coefficient seed. This is a **public local condition**, not a secret trapdoor.

Equivalently, choose \(x_w,y_w\) directly and check the three endpoint vector sets. Since \(q\) is already larger than \(n+m\) for the Cauchy block, the failure probability is small for fixed \(k\) and bounded local degrees.

---

## 6. Patch-rank expansion via capacitated peeling

Now define the selected wedge hypergraph

\[
\mathcal H=(X(0),\mathcal W),
\]

where each wedge is a \(3\)-uniform hyperedge touching its three vertices.

For a set of wedges

\[
S\subseteq \mathcal W,
\]

let

\[
\deg_S(v)=|\{w\in S:v\in w\}|.
\]

### Definition: \(k\)-peelability

The hypergraph \(\mathcal H\) is **\((\tau,k)\)-peelable** if every nonempty

\[
S\subseteq\mathcal W,\qquad |S|\le \tau N
\]

contains a vertex \(v\) with

\[
1\le \deg_S(v)\le k.
\]

Equivalently, there is no small \((k+1)\)-core: no nonempty small wedge set in which every incident vertex has residual degree at least \(k+1\).

### Lemma: peelability implies rank expansion

Let

\[
H=D^\ast C
\]

be the matrix whose columns are

\[
h_w=D^\ast c_w.
\]

Assume:

\[
\mathcal H\text{ is }(\tau,k)\text{-peelable}
\]

and local endpoint vectors are \(k\)-wise independent at every vertex. Then for every

\[
S\subseteq\mathcal W,\qquad |S|\le \tau N,
\]

we have

\[
\boxed{
\operatorname{rank}(H_S)=|S|.
}
\]

**Proof sketch.** Suppose there is a linear relation

\[
\sum_{w\in S}\alpha_w h_w=0.
\]

By peelability, choose a vertex \(v\) with

\[
1\le \deg_S(v)\le k.
\]

Looking only at the \(v\)-block gives

\[
\sum_{w\in S:v\in w}\alpha_w g_{v,w}=0.
\]

The involved vectors are linearly independent, so all those \(\alpha_w\) vanish. Remove those wedges and repeat. Induction kills all coefficients.

This is much stronger than generic-rank expansion. It is deterministic once the local MDS condition and peelability hold.

---

## 7. Neighbor expansion criterion for peelability

A convenient sufficient condition is small-set vertex expansion.

Suppose for every

\[
S\subseteq\mathcal W,\qquad |S|\le\tau N,
\]

the vertex neighborhood

\[
\Gamma(S)=\{v:\exists w\in S,\ v\in w\}
\]

satisfies

\[
\boxed{
|\Gamma(S)|>\frac{3}{k+1}|S|.
}
\]

Then \(\mathcal H\) is \((\tau,k)\)-peelable.

Indeed, if \(S\) were a \((k+1)\)-core, then every incident vertex would have degree at least \(k+1\). Counting incidences gives

\[
3|S|
=
\sum_{v\in\Gamma(S)}\deg_S(v)
\ge
(k+1)|\Gamma(S)|,
\]

so

\[
|\Gamma(S)|\le \frac{3}{k+1}|S|,
\]

contradiction.

This is why fiber boosting is powerful. For scalar wedges \(k=1\), the expansion requirement would be

\[
|\Gamma(S)|>\frac32 |S|,
\]

which is demanding. For \(k=4\), it becomes

\[
|\Gamma(S)|>0.6|S|.
\]

For \(k=8\), it becomes

\[
|\Gamma(S)|>\frac13 |S|.
\]

That is a major relaxation.

So the design target for the HDX wedge sampler is:

\[
\boxed{
\textbf{small selected wedge sets must not collapse onto too few vertices.}
}
\]

Fiber dimension \(k\) converts that into rank expansion.

---

## 8. How HDX gives sampling regularity

The wedge universe \(\Omega\) comes from typed \(2\)-faces. In a local spectral HDX, incidence walks between faces of different dimensions mix like the corresponding walks in a complete complex. Dinur–Kaufman’s agreement-expander result proves strong mixing for high-order random walks and formulates a double-sampling property: sets at one face level are seen with the correct proportion from another face level. citeturn548183view1

For us, this gives the regularity backbone:

\[
X(2)\leftrightarrow X(1)\leftrightarrow X(0)
\]

and, after restricting to typed faces,

\[
\Omega_\theta\leftrightarrow X(0),\qquad
\Omega_\theta\leftrightarrow X(1).
\]

The desired regularity statements are:

\[
\Pr_{w\in\Omega}[v_a(w)\in U_a]
=
\frac{|U_a|}{|X_a(0)|}\pm \varepsilon,
\]

and similarly for the \(b,c\) endpoints.

For edge supports \(E'\subseteq X(1)\),

\[
\Pr_{w\in\Omega}[P_w\cap E'\ne\varnothing]
\approx
\frac{|E'|}{|X(1)|}
\]

up to constants depending on the two-edge patch shape.

After public thinning,

\[
\mathcal W\subseteq\Omega,
\]

the same estimates hold for all large enough sets with high probability:

\[
\Pr_{w\in\mathcal W}[v_j(w)\in U]
=
\Pr_{w\in\Omega}[v_j(w)\in U]\pm \varepsilon_{\mathrm{thin}}.
\]

This is the **sampling regularity** requirement.

So HDX is used for **global sampler behavior**; fiber-boosted peeling is used for **local rank behavior**.

---

## 9. How to get small-set expansion after thinning

The remaining nontrivial property is:

\[
|\Gamma(S)|>\frac{3}{k+1}|S|
\]

for all small selected wedge sets \(S\).

A sufficient proof route is:

### Universe bounded overlap

The full wedge universe \(\Omega\) has bounded local complexity:

\[
\deg_\Omega(v)\le \Delta_0,
\]

\[
\deg_\Omega(e)\le \Delta_1,
\]

\[
\#\{w:w\supseteq\{u,v\}\}\le \Delta_2.
\]

This follows from bounded-degree HDX structure.

### Public thinning below local core threshold

Choose the thinning probability \(p\) so that the selected hypergraph has no small \((k+1)\)-core with high probability.

A standard counting argument goes as follows. A minimal connected stopping configuration of \(s\) wedges can be grown in at most

\[
N\cdot A^s
\]

ways, where \(A\) depends only on the bounded local degree of the wedge universe. The probability that all its wedges survive thinning is

\[
p^s.
\]

So the expected number of selected connected stopping configurations of size \(s\) is at most

\[
N(Ap)^s.
\]

If

\[
Ap<1,
\]

then small stopping configurations are suppressed. For larger \(s\), HDX sampler expansion and Chernoff bounds take over.

This yields the theorem target:

\[
\boxed{
\text{public thinning of a bounded-overlap HDX wedge universe is }(\tau,k)\text{-peelable w.h.p.}
}
\]

This is now a specific combinatorial theorem, not a vague diagnostic.

---

## 10. Full HDX wedge-sampler definition

A selected wedge sampler

\[
\mathsf{WS}=(X,\mathcal F,\mathcal W,\{x_w,y_w\}_{w\in\mathcal W})
\]

is **\((L,k,\tau,\varepsilon)\)-admissible** if:

### Bounded locality

For each \(w\),

\[
|\operatorname{supp}_{\mathrm{blocks}}(c_w)|\le 2,
\]

\[
|\operatorname{supp}_{\mathrm{blocks}}(D^\ast c_w)|\le 3.
\]

Here

\[
L=3
\]

on the divergence side.

### Local \(k\)-wise independence

For every vertex \(v\), every subset of at most \(k\) incident divergence vectors

\[
\{h_w(v):w\in\mathcal W(v)\}
\]

is linearly independent.

### \(k\)-peelability

Every nonempty

\[
S\subseteq\mathcal W,\qquad |S|\le\tau N
\]

has some vertex \(v\) with

\[
1\le \deg_S(v)\le k.
\]

### Sampling regularity

For all sufficiently large vertex/edge sets, the empirical incidence counts of \(\mathcal W\) approximate the typed HDX incidence probabilities within \(\varepsilon\). In particular, endpoint marginals, edge-patch marginals, and pair-overlap counts are controlled.

Then:

\[
\boxed{
\operatorname{rank}\{D^\ast c_w:w\in S\}=|S|
\quad
\text{for all }|S|\le\tau N.
}
\]

That is the exact patch-rank expansion we need.

---

## 11. Concrete sampler generation algorithm

```text
HDX-WedgeSampler(seed, X, q, k, rho, type_triples Θ):

    Ω = []

    for each oriented type triple θ = (a; b, c) in Θ:
        for each typed 2-face σ = {v_a, v_b, v_c} in X:
            Ω.append(w = (v_a; v_b, v_c))

    p = rho * |X(0)| / |Ω|

    W = []

    for w in Ω in canonical order:
        if Hash(seed_select, w) < p:
            W.append(w)

    for each w = (a; b, c) in W:
        choose x_w, y_w ∈ F_q^k from public PRG
        require:
            x_w ≠ 0
            y_w ≠ 0
            x_w + y_w ≠ 0

        define:
            c_w(a,b) = x_w
            c_w(a,c) = y_w

    for every vertex v:
        verify local k-wise independence of {h_w(v): w incident to v}
        if local check fails, increment public nonce and regenerate coefficients

    output C = [c_w]_{w∈W}
```

The local check is feasible because selected vertex degree is bounded or at most logarithmic. It is not a cryptanalytic diagnostic; it is part of the public sampler definition, and it enforces one of the proof axioms.

The expensive global property is \(k\)-peelability. We do **not** want to verify it by exhaustive search. We want the theorem:

\[
\Pr_{\text{public seed}}[
\mathcal W\text{ is not }(\tau,k)\text{-peelable}
]
\le \operatorname{negl}(\lambda).
\]

That theorem should be proved from bounded overlap, HDX sampling, and thinning density.

---

## 12. Plugging back into tilted exact-gradient projection

Recall the tilted projection map:

\[
A=
\begin{bmatrix}
B_0^\ast\\
B_1^\ast+\Delta C^\ast D
\end{bmatrix}.
\]

The wedge sampler supplies

\[
C:M_1^\ast\to C^1(X,\mathcal F).
\]

The local rank object is

\[
H=D^\ast C.
\]

The wedge-sampler theorem gives

\[
\operatorname{rank}(H_S)=|S|
\]

for all small

\[
S\subseteq[m_1].
\]

The random diagonal

\[
\Delta=\operatorname{diag}(\theta_i)
\]

then helps prevent medium-complexity cancellations in

\[
A^\ast(r_0,r_1)
=
B_0r_0+B_1r_1+D^\ast C\Delta r_1.
\]

If

\[
\operatorname{supp}(r_1)=S
\]

and

\[
|S|\le\tau N,
\]

then \(D^\ast C\Delta r_1\) lives in a rank-\(|S|\) subspace controlled by the wedge columns, with no hidden low-rank collapse.

That is the exact use of patch-rank expansion.

---

## 13. Suggested first parameters

Let

\[
N=|X(0)|,
\qquad
k=4\text{ or }8,
\qquad
n=k(N-\dim H^0).
\]

Choose:

\[
m_1=\rho N,
\qquad
0.25\le\rho\le1.
\]

Then selected wedge average vertex degree is about

\[
3\rho,
\]

independent of \(k\). With \(k=4\), small \((k+1)\)-cores are already unlikely at such densities under random thinning. With \(k=8\), the peelability margin is much larger.

For the guarded exact-gradient part, choose

\[
m_0=n+\mu,
\]

with

\[
\mu\approx 0.1n \text{ to } 0.25n
\]

depending on the desired primal distance.

Then total public measurement dimension is

\[
m=m_0+m_1
\approx
kN+\mu+\rho N.
\]

For \(k=4,\rho=0.5,\mu=0.2n\),

\[
m\approx 4N+0.8N+0.5N=5.3N.
\]

This is not tiny, but it is a proof-first regime. After the theorem is understood, \(k,\rho,\mu\) can be optimized.

---

## 14. The proof obligations are now precise

The wedge sampler gives us four concrete lemmas to prove.

### Lemma 1: HDX typed-face regularity

For the chosen HDX family and type triples \(\Theta\), the full wedge universe \(\Omega\) is a double sampler between vertices, edges, and wedges.

This should follow from local spectral HDX/high-order walk machinery. Dinur–Kaufman’s result is the relevant template: high-order walks in HDXs have strong spectral behavior and yield double-sampling properties. citeturn548183view1

### Lemma 2: thinning preserves large-set regularity

Public thinning

\[
\Omega\to\mathcal W
\]

preserves endpoint and patch sampling for all large sets.

This is probabilistic combinatorics over a sampler. It should be proved by Chernoff-style concentration plus the HDX sampler structure.

### Lemma 3: thinning gives small-core exclusion

For suitable \(\rho,k,\tau\),

\[
\mathcal W
\]

has no small \((k+1)\)-core with overwhelming probability.

Equivalently, it is \((\tau,k)\)-peelable.

This is the central new combinatorial lemma.

### Lemma 4: local MDS fibers turn peelability into rank

Given local \(k\)-wise independence at every vertex,

\[
(\tau,k)\text{-peelability}
\Rightarrow
\operatorname{rank}(D^\ast C)_S=|S|
\]

for every

\[
|S|\le\tau N.
\]

This lemma is straightforward and should be the first one written formally.

---

## 15. The concrete object to carry forward

The sampler we should now use is:

\[
\boxed{
\begin{aligned}
&\textbf{Base: typed local-spectral/cosystolic HDX }X.\\
&\textbf{Universe: one oriented wedge per selected typed }2\textbf{-face}.\\
&\textbf{Selection: public Bernoulli/hash thinning to }m_1=\rho |X(0)|.\\
&\textbf{Fibers: constant }\mathbb F_q^k\textbf{ coefficient sheaf.}\\
&\textbf{Column: }c_w(a,b)=x_w,\quad c_w(a,c)=y_w.\\
&\textbf{Divergence: }h_w(a)=-(x_w+y_w),\ h_w(b)=x_w,\ h_w(c)=y_w.\\
&\textbf{Local condition: endpoint vectors are }k\textbf{-wise independent at every vertex.}\\
&\textbf{Global condition: selected wedge hypergraph has no small }(k+1)\textbf{-core.}
\end{aligned}
}
\]

Then the three requirements are met as follows:

\[
\boxed{
\textbf{bounded locality}
}
\]

because each \(c_w\) touches two edge blocks and each \(D^\ast c_w\) touches three vertex blocks.

\[
\boxed{
\textbf{patch-rank expansion}
}
\]

because \(k\)-peelability plus local \(k\)-wise independence implies full rank on every small patch set.

\[
\boxed{
\textbf{sampling regularity}
}
\]

because typed HDX face incidence gives double-sampler behavior, and public thinning preserves it at large scales.

This is the strongest version of the wedge sampler so far: **rank is protected locally by fiber capacity, while distributional regularity comes from the HDX.**

## 60. User

what would a funny name for this be? 10 options

## 61. Assistant

Here are ten names that try to capture the flavor of the construction: **local-to-global gluing, descent, wedges, expansion, transversality, and anti-backdoor geometry**, while still sounding like a real cryptographic primitive rather than a paper title.

1. **WedgeLock**  
   Clean and memorable. Suggests local wedge constraints “locking” the hidden trivialization.

2. **Coboundary**  
   Stark and mathematical. Almost too direct, but has real gravitas.  
   “Coboundary KEM” sounds plausible.

3. **Transveil**  
   From *transverse* + *veil*. Hides coordinates by transverse projection.

4. **Gluon**  
   Short, physics-adjacent, and thematically perfect: the system is literally about gluing local data.  
   Risk: existing physics meaning.

5. **Patchwork**  
   Honest about the mathematics: patches, overlaps, descent.  
   “Patchwork KEM” sounds surprisingly natural.

6. **ShearLock**  
   Suggests both sheaf structure and geometric transversality/shearing.

7. **Cathedral**  
   This one I actually like a lot.  
   The system is fundamentally local pieces coherently assembled into a global structure. Also evokes Gothic geometry and layered support.  
   “Cathedral KEM” sounds serious without sounding generic.

8. **TorsorTrap**  
   Slightly playful/meta: the scheme is explicitly designed to avoid abelian torsor collapse.  
   More of an internal codename than a standards-track name.

9. **Wedgewood**  
   Funny and weirdly elegant.  
   Sounds like an old cryptographic codename from Bell Labs.

10. **Descender**  
   Directly from descent theory.  
   “Descender-512” honestly sounds plausible as a standardized primitive.

My favorites, depending on tone:

- **Cathedral** → best overall identity
- **Transveil** → best “new cryptography primitive” feel
- **WedgeLock** → best technical codename
- **Descender** → best standards-style naming
- **Patchwork** → best reflection of the actual mathematics

If you want, I can also generate:
- more **Bell Labs / Unix / Plan 9 style** names,
- more **mathematically loaded** names,
- more **military-industrial** names,
- or more **cute codename** style names like Kyber/Saber/Frodo.

## 62. User

standards style

## 63. Assistant

For standards-style names, the pattern is usually:

- short,
- pronounceable,
- emotionally neutral,
- slightly technical but not descriptive,
- scalable into parameter sets,
- easy to say in meetings,
- not obviously tied to one narrow mathematical interpretation.

Think:

- Kyber
- Dilithium
- Falcon
- Saber
- HQC
- FrodoKEM
- BIKE
- Classic McEliece

Your construction has themes of:
- descent,
- projection,
- local-to-global coherence,
- expansion,
- gluing,
- transversality,
- sheaves/cochains/coboundaries,
- layered local structure.

Here are the best standards-style candidates I can come up with:

---

### 1. **Descender**
Probably the strongest overall.

- sounds like a real primitive,
- subtly references descent theory,
- not overly mathematical,
- parameter sets sound natural:

```text
Descender-512
Descender-768
Descender-1024
```

This is genuinely believable as a future NIST candidate name.

---

### 2. **Traverse**
From transversality and traversing complexes.

- modern sounding,
- neutral,
- geometric without being obscure,
- good protocol naming feel.

```text
Traverse-768
```

Very standards-like.

---

### 3. **Patch**
Surprisingly good.

- local patches/gluing,
- minimalistic,
- memorable,
- almost feels like an old code-based crypto name.

```text
PATCH-KEM
Patch-512
```

Could also become an acronym later.

---

### 4. **Cohere**
Strong thematic alignment.

- local-to-global coherence,
- sounds trustworthy/stable,
- slightly enterprise sounding.

```text
Cohere-768
```

Potential collision with the AI company though.

---

### 5. **Vertex**
Geometric and infrastructure-like.

- fits HDX/sheaf geometry,
- sounds robust,
- good engineering vibe.

```text
Vertex-1024
```

---

### 6. **Latticeworks**
A little longer, but actually excellent branding-wise.

- evokes structure rather than raw lattices,
- “works” suggests assembly/gluing.

Could shorten to:

```text
Latworks
```

or

```text
LWK
```

---

### 7. **Relay**
I like this more than I expected.

- local information propagating globally,
- understated,
- sounds protocol-native.

```text
Relay-768
```

---

### 8. **Concord**
Very good thematic fit.

- global consistency from local agreement,
- mature standards feel,
- sounds stable and institutional.

```text
Concord-512
```

Honestly excellent.

---

### 9. **Tandem**
Suggests coupled local/global structure.

- simple,
- memorable,
- standards-compatible.

```text
Tandem-768
```

---

### 10. **Axiom**
Maybe too broad, but has the right feel.

- foundational,
- abstract,
- cryptography-adjacent branding.

```text
Axiom-1024
```

---

My strongest recommendations:

| Name | Why |
|---|---|
| **Descender** | Most mathematically authentic while still standards-grade |
| **Concord** | Best “serious deployed cryptography” feel |
| **Traverse** | Best geometric/transversal feel |
| **Vertex** | Best HDX/geometric flavor |
| **Relay** | Best understated engineering tone |

If I were actually submitting this to NIST someday, I would probably choose:

\[
\boxed{\textbf{Descender}}
\]

because it quietly encodes the central mathematical idea:

\[
\textbf{descent data}
\]

without sounding academic or gimmicky.
