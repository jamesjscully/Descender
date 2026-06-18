//! Standalone KoalaBear prime field `F_p`, `p = 2^31 - 2^24 + 1 = 0x7f000001`.
//!
//! This is the SAME field Plonky3's `p3-koala-bear` operates over (see
//! `p3-koala-bear-0.6.1/src/koala_bear.rs`: `PRIME = 0x7f000001`). We work in the
//! plain *canonical* (standard) integer representation — NOT Montgomery form —
//! because all we need is correct field arithmetic; the Montgomery encoding is an
//! internal implementation detail of `MontyField31` that never escapes a Poseidon2
//! permutation (inputs and outputs are canonical field elements). The constants
//! exported by `p3-koala-bear` (e.g. `KOALABEAR_POSEIDON2_RC_16_*`) are the
//! canonical integers passed through `KoalaBear::new(..)`, so we pin those same
//! canonical integers here.

/// KoalaBear prime: `2^31 - 2^24 + 1`.
pub const P: u32 = 0x7f00_0001; // 2_130_706_433

/// A KoalaBear field element in canonical form, `0 <= value < P`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct F(u32);

impl F {
    pub const ZERO: F = F(0);
    pub const ONE: F = F(1);

    /// Construct from a canonical integer (any `u32`, reduced mod `P`).
    #[inline]
    pub const fn new(x: u32) -> F {
        F(x % P)
    }

    /// Construct from a `u64`, reduced mod `P`.
    #[inline]
    pub fn from_u64(x: u64) -> F {
        F((x % P as u64) as u32)
    }

    /// The canonical integer representative in `[0, P)`.
    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    #[inline]
    pub fn add(self, other: F) -> F {
        let mut s = self.0 as u64 + other.0 as u64;
        if s >= P as u64 {
            s -= P as u64;
        }
        F(s as u32)
    }

    #[inline]
    pub fn sub(self, other: F) -> F {
        let s = (self.0 as u64 + P as u64 - other.0 as u64) % P as u64;
        F(s as u32)
    }

    #[inline]
    pub fn neg(self) -> F {
        if self.0 == 0 {
            F(0)
        } else {
            F(P - self.0)
        }
    }

    #[inline]
    pub fn mul(self, other: F) -> F {
        F(((self.0 as u64 * other.0 as u64) % P as u64) as u32)
    }

    #[inline]
    pub fn double(self) -> F {
        self.add(self)
    }

    /// `x^e` by square-and-multiply.
    pub fn pow(self, mut e: u64) -> F {
        let mut base = self;
        let mut acc = F::ONE;
        while e > 0 {
            if e & 1 == 1 {
                acc = acc.mul(base);
            }
            base = base.mul(base);
            e >>= 1;
        }
        acc
    }

    /// Multiplicative inverse via Fermat: `x^(P-2)`.
    pub fn inverse(self) -> F {
        debug_assert!(self.0 != 0, "inverse of zero");
        self.pow((P - 2) as u64)
    }

    /// The S-box `x -> x^3` (KoalaBear Poseidon2 alpha = 3).
    #[inline]
    pub fn sbox(self) -> F {
        let sq = self.mul(self);
        sq.mul(self)
    }
}
