#!/usr/bin/env python3
"""
WS2-a / WS4 reproducible check: verify the deployed challenge-action extension
polynomial X^12 + X + 25 is irreducible over F_12289 via Rabin's test.

The Lean certificate (FullResponseChallengeActionCertificate) currently *assumes*
`extensionPolynomialIrreducible` as a Prop field. This script independently
confirms the assumption is TRUE (so the assumption is sound, just not yet
discharged in Lean). Rabin: f of degree n over F_q is irreducible iff
  (1) X^(q^n) ≡ X (mod f), and
  (2) for every prime p | n, gcd(X^(q^(n/p)) - X, f) = 1.
Exit 0 iff irreducible.
"""
import sys

Q = 12289
N = 12
# f = X^12 + X + 25, coeffs low->high (matches extensionPolynomial{Constant=25,LinearCoeff=1,Degree=12})
F = [25, 1] + [0] * 10 + [1]
PRIME_DIVISORS_OF_N = [2, 3]

def degree(a):
    d = len(a) - 1
    while d > 0 and a[d] == 0:
        d -= 1
    return d if any(a) else -1

def polmod(a, mod):
    a = a[:]
    dm = len(mod) - 1
    inv_lead = pow(mod[-1], Q - 2, Q)
    while len(a) - 1 >= dm and any(a):
        if a[-1] == 0:
            a.pop(); continue
        d = len(a) - 1
        c = (a[-1] * inv_lead) % Q
        for i in range(len(mod)):
            a[d - dm + i] = (a[d - dm + i] - c * mod[i]) % Q
        while a and a[-1] == 0:
            a.pop()
    return a if a else [0]

def polmulmod(a, b, mod):
    res = [0] * (len(a) + len(b) - 1)
    for i, ai in enumerate(a):
        if ai:
            for j, bj in enumerate(b):
                res[i + j] = (res[i + j] + ai * bj) % Q
    return polmod(res, mod)

def polpowmod(base, e, mod):
    result = [1]
    b = polmod(base[:], mod)
    while e > 0:
        if e & 1:
            result = polmulmod(result, b, mod)
        b = polmulmod(b, b, mod)
        e >>= 1
    return result

def polsub(a, b):
    L = max(len(a), len(b)); r = [0] * L
    for i in range(L):
        r[i] = ((a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)) % Q
    while len(r) > 1 and r[-1] == 0:
        r.pop()
    return r

def polgcd(a, b):
    a, b = a[:], b[:]
    while degree(b) >= 0 and any(b):
        a, b = b, polmod(a, b)
        if not any(b):
            break
    return a

def main():
    X = [0, 1]
    cond1 = polsub(polpowmod(X, Q ** N, F), X) == [0]
    print(f"(1) X^(q^{N}) == X (mod f) : {cond1}")
    ok = cond1
    for p in PRIME_DIVISORS_OF_N:
        g = polgcd(F[:], polsub(polpowmod(X, Q ** (N // p), F), X))
        d = degree(g)
        print(f"(2) p={p}: deg gcd(X^(q^{N//p}) - X, f) = {d}  (coprime iff 0)")
        ok = ok and (d == 0)
    print(f"X^12 + X + 25 irreducible over F_{Q}: {ok}")
    return 0 if ok else 1

if __name__ == "__main__":
    sys.exit(main())
