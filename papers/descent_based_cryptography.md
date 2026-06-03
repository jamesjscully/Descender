# Coordinate Hiding and Descent-Based Cryptography

## Working Thesis

Public-key cryptography can be viewed as the art of hiding a coordinate while
leaving enough public structure for useful computation. Classical systems hide
a coordinate inside a global algebraic object: a scalar in a cyclic group, a
factorization behind an integer, or a short vector behind a noisy linear view.

In that classical pattern, the public object is one global presentation of the
secret:

```text
secret coordinate s  ->  public object F(s)
```

The adversary's problem is to recover the hidden coordinate, or enough of it,
from the public presentation. Even when noise is added, as in LWE-style systems,
the basic shape remains coordinate recovery from public measurements.

Descent-based cryptography proposes a different primitive object. The public
object is not a single global presentation of a secret coordinate. It is a
quotient or descended object, and the secret is a local representative lying
over it:

```text
hidden representative h  ->  public quotient q
fiber over q = all local representatives describing the same public object
```

This is helpful because it separates three roles that are usually fused
together: public meaning, private presentation, and validity. The public
quotient fixes the meaning visible to the protocol. The hidden representative
is allowed to vary inside the fiber without changing that meaning. Validity is
the statement that the public object descends, or equivalently that local data
can glue into a coherent object. The adversary is therefore not merely trying
to invert a public map and recover an absolute coordinate; it is trying to
select or distinguish a private local presentation inside a quotient whose
public semantics have already been factored out.

The conceptual advantage is that useful public structure can live in the
quotient, while private variation lives in the fiber. A good descent-based
scheme should make the quotient rich enough for verification and computation,
but too poor to determine the representative. This gives a different answer to
the old coordinate-hiding problem: hide not the position of a point in one
global coordinate system, but the choice of local trivialization behind a
public descended object.

The shortest algebraic slogan is:

```text
A = Pi D
```

Here `D` is the local-to-global descent operator, and `Pi` is the public
measurement of its descended data. This factorization is the point. It means
the public map is not just a table of measurements. It remembers that the
measurements came from local data that had to fit together.

One can picture `D` on a graph. Put private labels on vertices. The coboundary
operator records edge differences. Not every assignment of edge values is
valid: around every closed loop, the differences must add consistently. Thus
validity has a public shape, a vanishing defect or obstruction. At the same
time, the vertex labels are not unique: shifting all labels by a common gauge
can leave the edge differences unchanged. The public object can therefore
remember the glued boundary while forgetting the local representative that
produced it.

This is the intuitive content behind the four central benefits:

1. **Validity becomes obstruction vanishing.** A public object is not merely a
   vector that might have been sampled honestly. It can carry a visible
   consistency condition: do these local pieces glue?
2. **Secrets become fibers.** The private object is not a single absolute
   coordinate but one representative among many lying over the same public
   quotient.
3. **Meaning is quotient-level.** If two representatives lie over the same
   public object, they must induce the same cryptographic meaning. Private
   variation becomes presentation, not semantics.
4. **Hardness has a geometric target.** The natural search problem is no longer
   just "invert a noisy linear map"; it is "find or distinguish a nearby
   projected descended object." This points toward worst-case problems about
   nearest projected coboundaries rather than arbitrary public matrices.

The paper should be a theoretical exposition of this viewpoint. It should not
present Descender algorithms. "Descender" is the name for concrete schemes that
may instantiate the theory; the paper is about the theory itself.

## Provisional Title Options

1. Coordinate Hiding and Descent-Based Cryptography
2. Cryptography as Coordinate Hiding
3. Descent as a Cryptographic Principle
4. Local Representatives and Public Quotients
5. A Descent-Theoretic View of Public-Key Cryptography

## 1. Introduction: Cryptography as Coordinate Hiding

Start with the simplest possible intuition:

```text
secret coordinate -> public object
```

The cryptographic problem is not merely secrecy. It is asymmetric visibility:
one party knows a coordinate, while everyone else sees an object from which the
coordinate is hard to recover but still useful for public computation.

Develop this with a clean example before introducing any new language.

### Preferred Example: Diffie-Hellman / Elliptic Curves

Use a cyclic group because the coordinate is explicit.

```text
secret: x
public: X = g^x              finite-field Diffie-Hellman
public: X = xG               elliptic-curve Diffie-Hellman
```

The public object `X` is a point in a group. The hidden coordinate is the scalar
`x` relative to the public generator. Security is the difficulty of recovering
the coordinate from the point.

This example is ideal because it lets the paper say:

```text
The public point is easy to compute from the coordinate.
The coordinate is hard to recover from the public point.
The group law lets public objects interact without revealing coordinates.
```

Then explain the quantum failure mode conceptually: the coordinate is hidden in
an exact algebraic symmetry, and quantum algorithms are effective precisely
against such globally coherent hidden-coordinate problems.

### Optional Supporting Examples

RSA can be used as a second example, but probably not the first. The hidden
coordinate is less visually clean. It is more like hidden factorization or
hidden trapdoor structure:

```text
public: N = pq
secret: p, q or equivalent trapdoor data
```

LWE can then appear as the modern noisy version:

```text
public samples: b = <a, s> + e
secret coordinate: s
```

This gives the transition:

```text
exact coordinate hiding -> noisy coordinate hiding
```

The paper's move is then:

```text
global coordinate hiding -> local representative hiding
```

## 2. The General Coordinate-Hiding Template

Abstract the examples into a common form:

```text
S    coordinate space
P    public object space
F    public map

secret s in S
public p = F(s)
```

Cryptography needs three things:

1. easy forward computation;
2. hard coordinate recovery;
3. enough public structure to compute, verify, encapsulate, sign, or agree.

In this template, the secret is a point in a coordinate system. The public
object is the image of that coordinate. The classical risk is that the public
structure may preserve too much exact global symmetry.

This section should introduce the central distinction:

```text
coordinate as absolute position
coordinate as local representative
```

Classical cryptography mostly hides absolute position. Descent-based
cryptography hides a representative in a fiber.

## 3. From Coordinates to Representatives

Introduce the core conceptual shift.

In many mathematical settings, a global object can be described by local data.
The local descriptions are not unique. Different local representatives may
describe the same global object, related by gauge transformations, choices of
trivialization, or changes of coordinates.

The cryptographic idea:

```text
public object: descended global object
secret: local representative over that object
```

The public object is not supposed to reveal the representative. The
representative is not an arbitrary hidden witness; it is a local presentation of
the same descended object.

Key vocabulary:

```text
H    hidden local representatives
Q    public quotient / descended objects
boundary : H -> Q
fiber over q = boundary^{-1}(q)
```

The hidden data lives in the fiber. The public data is the quotient point.

## 4. Descent as the Public Validity Principle

Now introduce descent.

The public object should be accepted when local data can glue into a coherent
global object. Instead of saying "there exists a witness satisfying a relation,"
the theory says:

```text
the public boundary has zero obstruction
```

or:

```text
local representatives over this boundary descend to a valid object
```

Core diagram:

```text
H  --meaning-->  M
|                ^
| boundary       |
v                |
Q  --obs-->      O
```

Where:

```text
H = hidden representatives
Q = public quotient
M = public semantic meaning
O = obstruction space
```

The theoretical laws:

```text
effective descent:
  obs(q) = 0 implies q has a locally valid representative

safety saturation:
  representatives in the same fiber have the same public meaning

projection completeness:
  every public protocol field factors through q

fiber privacy:
  the fiber over q contains hidden variation not determined by q
```

These should be presented as axioms or desiderata of the theory, not as
engineering checklists.

## 5. The Central Contrast

This is the heart of the paper.

Classical coordinate hiding:

```text
hide x such that X = action(x, base)
```

Noisy coordinate hiding:

```text
hide s such that t = A s + e
```

Descent-based hiding:

```text
hide h in boundary^{-1}(q)
```

The public object is not merely an image of a secret coordinate. It is a
quotient object whose fiber contains many possible local descriptions. Security
comes from the difficulty of selecting, recovering, or distinguishing the
representative, while correctness comes from the fact that all valid
representatives over the public object mean the same thing.

This section should argue that descent-based cryptography changes the primitive
question:

```text
not "which coordinate produced this public point?"
but "which local representative lies over this descended object?"
```

## 6. Noisy Descent as an Algebraic Model

Only after the abstract framing is clear, introduce a minimal algebraic model.

```text
D : local representatives -> descent data
Pi : descent data -> public measurements
A = Pi D
t = A s + e
```

This is the bridge to lattice/code-style post-quantum cryptography, but it
should not dominate the paper.

Interpretation:

```text
D extracts descent/coboundary information from a local representative.
Pi exposes only a public measured quotient of that information.
e hides exact membership and supports average-case hardness.
```

The associated hard problem:

```text
given t near the projected descended image, recover or distinguish the hidden
representative
```

This can be called projected noisy descent or projected transparent noisy
descent, depending on how much terminology we want to introduce.

## 7. Why This Is Not Merely LWE With Different Words

This section should be short but important.

LWE begins with arbitrary or structured public linear measurements of a hidden
vector. Descent-based cryptography begins with a public local-to-global
operator and then measures its descended image.

The difference is not the equation `t = A s + e`; the difference is the origin
and interpretation of `A`.

```text
LWE view:
  A is a public measurement matrix.

descent view:
  A factors as Pi D, where D is a descent/coboundary operator.
```

In the LWE view, `A` tells us how the secret coordinate is observed. In the
descent view, `D` first tells us what it means for local data to have coherent
boundary data, and `Pi` then tells us what part of that boundary the public is
allowed to see.

That two-stage structure is the source of the theory.

**Public validity as obstruction vanishing.**

In a generic noisy linear system, a public sample is usually valid because it
was generated by the sampling process. Descent gives a stronger mathematical
notion: the public object can have an intrinsic defect. In the graph picture,
edge differences are valid only if they close around cycles. The obstruction is
the failure to close. Thus validity is not just membership in a distribution;
it is the vanishing of a local-to-global inconsistency.

**Local representatives as private fibers.**

In LWE, the secret is a hidden vector. In descent, the secret is a hidden
representative over a quotient. The quotient deliberately forgets some
presentation data. Many representatives may describe the same public descended
object, just as many choices of local coordinates may describe the same
geometric object. Privacy lives in that forgotten presentation data.

**Semantic invariance across a fiber.**

For this to be cryptographic rather than merely ambiguous, all representatives
over the same public quotient must mean the same thing. The fiber is allowed to
hide presentation, not meaning. This is the conceptual reason quotient choice is
central: the quotient must forget enough to hide the representative, but not so
much that it forgets the protocol semantics.

**Worst-case problems as nearest projected descended objects.**

Once `A` is understood as `Pi D`, the natural hard problem is not just inversion
of a public matrix. It is recovery near the projected image of coherent descent
data. In geometric language, the adversary sees a noisy point near the public
shadow of the descended objects and must recover, distinguish, or decode the
hidden representative. This suggests worst-case problems about nearest
projected coboundaries: find the closest valid descended object after public
projection.

## 8. Public Meaning and Fiber Safety

A theoretical paper needs to state why hiding in a fiber is not enough.

If two representatives in the same public fiber imply different public behavior,
the quotient is unsound. Therefore the quotient must be semantically saturated:

```text
boundary(h) = boundary(h') implies meaning(h) = meaning(h')
```

This section should connect the theory to cryptographic correctness without
becoming implementation-heavy.

Examples of "meaning" in different cryptographic settings:

```text
KEM: shared-secret derivation class
signature: authorization relation
account system: namespace, lifecycle, nullifier, transcript binding
commitment: opened message or admissible opening class
```

The point is general: the public object must determine the public semantics,
while the private representative remains hidden.

## 9. The Theoretical Claim

Do not include a "what remains to prove" section. Replace it with a statement
of what the theory claims to organize.

Possible formulation:

```text
Descent-based cryptography is the study of cryptographic systems whose public
objects are descended quotients and whose secrets are local representatives in
the fibers of those quotients.
```

The central claim:

```text
The right abstraction for many post-quantum public-key systems is not only
noisy coordinate hiding, but noisy local-representative hiding over a public
quotient.
```

This should read like a theoretical lens, not a product roadmap.

## 10. Examples and Non-Examples

Use examples sparingly.

Examples:

1. finite-field or elliptic-curve Diffie-Hellman as exact coordinate hiding;
2. LWE as noisy coordinate hiding;
3. projected noisy descent as noisy representative hiding.

Non-examples:

1. any scheme where the quotient reveals a unique representative;
2. any scheme where different representatives over the same quotient have
   different public meanings;
3. any construction where "descent" is only terminology for an ordinary hidden
   witness proof.

## 11. Conclusion

The conclusion should be philosophical and compact.

Cryptography has long relied on hidden coordinates. The post-quantum transition
has already forced a move from exact coordinates to noisy coordinates.
Descent-based cryptography proposes the next abstraction: hide not an absolute
coordinate, but a local representative of a public descended object.

The value of the theory is that it separates three ideas that are often mixed:

```text
public meaning
hidden representative
valid descent
```

When these are cleanly separated, cryptographic design can ask sharper
questions: what is public, what is local, what glues, and what remains hidden in
the fiber?
