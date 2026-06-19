//! `recursive_air` (capstone monolithic verifier-AIR, Option A) — bus + consumers in one window.
//!
//! Per the composition decision (`research/path2_recursion_air_composition_architecture.md`,
//! Option A) the recursion is ONE wide monolithic AIR over a shared column layout that INLINES
//! every gadget's constraints and — crucially — wires the CROSS-GADGET BUSES that the standalone
//! gadget AIRs leave as free columns. The standalone gadgets each validated their own constraints
//! ([`crate::challenger`], [`crate::merkle_air`], [`crate::fri_fold_air`], [`crate::fri_chain_air`],
//! [`crate::deep_air`], [`crate::public_io_air`]) but in isolation a malicious prover can still
//! choose, independently per gadget: which query index to answer ([`crate::index_bus`] note), and
//! what fold point `s` / challenge `beta` / OOD point `zeta` to fold/open at. The monolith closes
//! those by binding the gadgets' free columns to gadget-1's Fiat–Shamir exports OVER ONE WINDOW.
//!
//! ## What this monolith wires + tests (each cross-binding tamper-rejected)
//!
//! ONE window (one row) per query. Blocks are config-gated ([`RecursiveConfig`]); absent blocks
//! cost no columns. Every block is validated honest-accept + tamper-reject via
//! `p3_air::check_all_constraints` (the rigor gate: no underconstrained binding).
//!
//!   1. **index ↔ challenger draw** (reuse [`crate::index_bus`]): the query index is the low
//!      `log_height` bits of gadget-1's `SampleDraw` export (`draw_out`), via the bit
//!      decomposition + canonical recomposition. A forged (favorable) index is rejected.
//!   2. **fold point `s` ↔ index** (product tower): `s = fold_point(log_height, index) =
//!      g^{rev_bits(index)}`, realized as a COMMITTED product tower over the index bits (degree-2
//!      steps), so `s` is pinned to the index — NOT a free column. A prover cannot fold at a
//!      favorable point.
//!   3. **FRI fold identity** (reuse [`crate::fri_fold_air`]): the cleared fold identity (★)
//!      `folded·2s == e0(beta+s) − e1(beta−s)` over the deg-8 extension at the BUS-bound `s`.
//!   4. **(a) Merkle index ↔ SAME bus** (NEW; reuse [`crate::merkle_air`]): a salted Merkle path is
//!      inlined level-by-level in the same window; each level's index-bit placement reads `bit_j`
//!      of THIS window's decomposition, so the Merkle opening and the FRI fold answer the SAME
//!      FS-forced query. Leaf/root bound by public values. Tamper: a Merkle path opened at a
//!      DIFFERENT index than the fold point is rejected ([`tests::air_rejects_merkle_at_wrong_index`]).
//!   5. **(b) beta / DEEP zeta ↔ gadget-1 SampleDraw** (NEW): a `Challenge` round challenge is 8
//!      consecutive base-field `SampleDraw` draws; the monolith carries those draw columns and pins
//!      `beta[i] == draw_i` (and likewise `zeta`). A forged/swapped challenge (≠ the transcript
//!      draw) is rejected ([`tests::air_rejects_forged_beta`], [`tests::air_rejects_forged_zeta`]).
//!   6. **(c) multi-round descent** (NEW; reuse [`crate::fri_chain_air`] chaining IN-WINDOW): the
//!      window threads `num_rounds` commit-phase folds — `s_0` tower-bound to the bus index,
//!      `s_{r+1} == s_r²` (point-square), `e0_{r+1} == folded_r` (chain-link). A broken descent
//!      link is rejected ([`tests::air_rejects_broken_descent`]).
//!   7. **(b′) DEEP OOD block** (NEW; reuse [`crate::deep_air`]): the OOD quotient relation
//!      `C(zeta) == Z_H(zeta)·Q(zeta)` at the bus-bound `zeta`, with `zeta` pinned to the challenger
//!      draws by (b). Tamper: wrong `C` rejected.
//!
//! ## Remaining assembly toward the full monolith (documented, NOT yet wired)
//!
//! - **inline the challenger sponge** so the `draw_out` / `draw_i` exports are themselves the
//!   constrained output of the in-AIR duplex sponge ([`crate::challenger::air::ChallengerAir`] +
//!   [`crate::inlined_perm::eval_perm`] for every perm). TODAY the bus takes the draws as committed
//!   columns (the gadget-1 AIR separately proves they are the sponge outputs); the full monolith
//!   derives them in the same window. This is the single largest remaining piece (it folds the
//!   ~190-absorb / multi-permute transcript into the window).
//! - **deferred leaf C(zeta)**: the DEEP block opens `C(zeta)` / `Q(zeta)` as committed values and
//!   constrains the QUOTIENT RELATION; the full `C(zeta) =` leaf-AIR-constraint-polynomial
//!   evaluation (re-expressing the entire leaf folder) + the DEEP linear-combination / chunk
//!   recomposition stay deferred (same boundary [`crate::deep_air`] documents).
//! - **public-IO block**: [`crate::public_io_air`] statement preservation in the same window
//!   (statement equality is structurally trivial; deferred only for layout brevity here).
//! - **Merkle ↔ FRI eval binding**: today the Merkle block binds the leaf/root to public values and
//!   its index to the bus; binding the OPENED leaf digest to the FRI `e0` limbs (so the Merkle-
//!   verified value IS the folded eval) is the natural next cross-binding (the columns are present;
//!   the equality is a follow-up).

use crate::fri_fold::{fold_pair, fold_point};
use crate::fri_fold_air::{limbs, D, W};
use crate::index_bus::{assert_canonical_pub, FIELD_BITS};
use crate::inlined_perm::{self, eval_perm, ConstantsExpr, PermCells, C_OUTPUT, PERM_COLS};
use descender_account_stark::{Challenge, Val, DIGEST_ELEMS, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::{PrimeCharacteristicRing, PrimeField32, TwoAdicField};

extern crate alloc;
use alloc::vec::Vec;

// ======================================================================================
// Config + dynamic column layout.
//
// The window grows additively with optional blocks. A `RecursiveConfig` with all blocks off
// reproduces the original single-window (index↔draw, point↔index tower, single FRI fold).
// ======================================================================================

/// Which optional cross-binding blocks this monolith window carries.
#[derive(Clone, Copy, Debug)]
pub struct RecursiveConfig {
    /// FRI leaf-layer log-height for this query (`fold_point`'s `log_height`).
    pub log_height: usize,
    /// (a) Number of Merkle path levels inlined in the window (0 = no Merkle block). Each level's
    /// index bit is bound to `bit_level` of the bus decomposition.
    pub merkle_levels: usize,
    /// (b) Bind the round-`beta` limbs to challenger `SampleDraw` columns (8 draws per challenge).
    pub bind_beta_to_draws: bool,
    /// (c) Number of commit-phase descent rounds threaded in the window (0/1 = single fold; the
    /// chaining transitions apply for r → r+1 within the window). Each extra round adds an
    /// `(e0,e1,beta,s,folded)` block; `s_0` is the tower point, `s_{r+1}==s_r²`, `e0_{r+1}==folded_r`.
    pub num_rounds: usize,
    /// (b′) DEEP OOD block: open `(zeta,Q,C)` + squaring tower at `log_degree` and check
    /// `C == Z_H(zeta)·Q`. `Some(log_degree)` enables it; `zeta` is bound to challenger draws.
    pub deep_log_degree: Option<usize>,
}

impl RecursiveConfig {
    /// The original single-window config (no Merkle, no beta-draw binding, single round, no DEEP).
    pub fn single_window(log_height: usize) -> Self {
        Self {
            log_height,
            merkle_levels: 0,
            bind_beta_to_draws: false,
            num_rounds: 1,
            deep_log_degree: None,
        }
    }
    /// Number of descent rounds actually laid out (≥ 1).
    pub fn rounds(&self) -> usize {
        self.num_rounds.max(1)
    }
}

// ----- shared monolithic column layout (one query window per row) -----
//
// Index-bus block (gadget-1 export + decomposition):
/// Gadget-1 `SampleDraw` export: the base-field element this query index reduces.
pub const C_DRAW_OUT: usize = 0;
/// Full canonical bit decomposition of `draw_out` (LSB first), `FIELD_BITS` columns.
pub const C_BITS: usize = C_DRAW_OUT + 1;
/// The bus index value (`Σ_{i<log_height} bit_i·2^i`) the consumers read.
pub const C_INDEX: usize = C_BITS + FIELD_BITS;
// FRI-fold block (gadget-3 consumer), round 0:
/// First sibling eval `e0 = f(s)` (8 limbs).
pub const C_E0: usize = C_INDEX + 1;
/// Second sibling eval `e1 = f(-s)` (8 limbs).
pub const C_E1: usize = C_E0 + D;
/// Round fold challenge `beta` (8 limbs).
pub const C_BETA: usize = C_E1 + D;
/// The fold point `s` — BUS-BOUND to the index by the product tower (not free).
pub const C_S: usize = C_BETA + D;
/// The claimed folded eval (8 limbs).
pub const C_FOLDED: usize = C_S + 1;
/// Fold-point product tower `sp[0..=log_height]`: `sp[0]=1`, `sp[j+1]=sp[j]·sel_j`,
/// `sp[log_height]==s`. `(log_height+1)` columns. (The committed-tower trick; see module doc.)
pub const C_SP: usize = C_FOLDED + D;

/// Per-round FRI block size (e0,e1,beta,s,folded) = 4·D + 1.
pub const ROUND_BLOCK: usize = 4 * D + 1;

/// End of the round-0 + tower block (start of the appended extra-round / DEEP / draw / Merkle
/// blocks). The single-fold columns above (C_E0..C_SP) are round 0; extra rounds for the (c)
/// descent are appended after the tower.
const fn tower_end(log_height: usize) -> usize {
    C_SP + (log_height + 1)
}

/// Base of extra descent round `r` (r ≥ 1): each is an `(e0,e1,beta,s,folded)` block.
fn extra_round_base(cfg: &RecursiveConfig, r: usize) -> usize {
    debug_assert!(r >= 1);
    tower_end(cfg.log_height) + (r - 1) * ROUND_BLOCK
}

/// Base of the DEEP OOD block (zeta[D], Q[D], C[D], pow[(k+1)·D]) — present iff `deep_log_degree`.
fn deep_base(cfg: &RecursiveConfig) -> usize {
    tower_end(cfg.log_height) + cfg.rounds().saturating_sub(1) * ROUND_BLOCK
}

/// DEEP block sub-offsets relative to `deep_base`.
const DEEP_ZETA: usize = 0;
const DEEP_Q: usize = DEEP_ZETA + D;
const DEEP_C: usize = DEEP_Q + D;
const DEEP_POW: usize = DEEP_C + D; // pow[0..=k]

fn deep_size(log_degree: usize) -> usize {
    DEEP_POW + (log_degree + 1) * D
}

/// Base of the (b) challenger-draw block: 8 draw columns per challenge bound. We bind round betas
/// (one per round) and optionally zeta. `draws_count` challenges × D columns.
fn draw_block_base(cfg: &RecursiveConfig) -> usize {
    let mut base = deep_base(cfg);
    if let Some(k) = cfg.deep_log_degree {
        base += deep_size(k);
    }
    base
}

/// Number of challenges whose limbs are bound to draw columns: one beta per round (+ zeta if DEEP),
/// iff `bind_beta_to_draws`.
fn draw_challenge_count(cfg: &RecursiveConfig) -> usize {
    if !cfg.bind_beta_to_draws {
        return 0;
    }
    cfg.rounds() + if cfg.deep_log_degree.is_some() { 1 } else { 0 }
}

/// Base of the (a) Merkle block: `merkle_levels` levels, each `running[8] ‖ sibling[8] ‖ perm[164]`.
fn merkle_base(cfg: &RecursiveConfig) -> usize {
    draw_block_base(cfg) + draw_challenge_count(cfg) * D
}

/// Per-level Merkle sub-block size.
pub const MERKLE_LEVEL: usize = 2 * DIGEST_ELEMS + PERM_COLS;
/// Sub-offsets within a Merkle level block.
const ML_RUNNING: usize = 0;
const ML_SIBLING: usize = DIGEST_ELEMS;
const ML_PERM: usize = 2 * DIGEST_ELEMS;

fn merkle_level_base(cfg: &RecursiveConfig, level: usize) -> usize {
    merkle_base(cfg) + level * MERKLE_LEVEL
}

/// Total column count for a window with this config.
pub fn n_cols(cfg: &RecursiveConfig) -> usize {
    merkle_base(cfg) + cfg.merkle_levels * MERKLE_LEVEL
}

/// The `j`-th tower column (`sp[j]`).
const fn sp_col(j: usize) -> usize {
    C_SP + j
}

/// The fixed tower base constants `base_j = g^{2^j}` for `g = two_adic_generator(log_height+1)`.
fn tower_bases(log_height: usize) -> Vec<Val> {
    let g = Val::two_adic_generator(log_height + 1);
    let mut bases = Vec::with_capacity(log_height);
    let mut p = g;
    for _ in 0..log_height {
        bases.push(p);
        p = p * p;
    }
    bases
}

// ======================================================================================
// Window builder.
// ======================================================================================

/// All honest inputs to one query window. The Merkle / beta-draw / descent / DEEP fields are only
/// consumed when the corresponding config block is enabled.
pub struct WindowInputs<'a> {
    pub draw_out: Val,
    /// Round 0 sibling evals + challenge (and, for the descent, the per-round betas/siblings come
    /// via `descent`).
    pub e0: Challenge,
    pub e1: Challenge,
    pub beta: Challenge,
    /// (c) Extra descent rounds: per round `r≥1`, the freshly-opened sibling `e1_r` and `beta_r`.
    /// `e0_r` and `s_r` are derived (chain-link + point-square). Empty = single fold.
    pub descent: &'a [(Challenge, Challenge)], // (e1_r, beta_r) for r = 1..num_rounds
    /// (a) Merkle path: leaf digest + bottom-to-top siblings. Length must equal `merkle_levels`.
    pub merkle_leaf: [Val; DIGEST_ELEMS],
    pub merkle_siblings: &'a [[Val; DIGEST_ELEMS]],
    /// (b′) DEEP OOD inputs: `(zeta, Q)`; `C` is derived `Z_H(zeta)·Q`.
    pub deep_zeta: Challenge,
    pub deep_q: Challenge,
}

impl<'a> WindowInputs<'a> {
    /// Minimal single-fold inputs (no Merkle / descent / DEEP).
    pub fn single(draw_out: Val, e0: Challenge, e1: Challenge, beta: Challenge) -> Self {
        Self {
            draw_out,
            e0,
            e1,
            beta,
            descent: &[],
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: Challenge::ZERO,
            deep_q: Challenge::ZERO,
        }
    }
}

/// Build one query window honestly. The index, fold point, tower, descent chain, Merkle perm
/// blocks, and DEEP tower are all DERIVED (the authority the AIR is checked against); tamper tests
/// mutate the matrix afterwards.
pub fn pack_window(cfg: &RecursiveConfig, inp: &WindowInputs) -> Vec<Val> {
    let h = cfg.log_height;
    let canon = inp.draw_out.as_canonical_u32();
    let index = (canon & ((1u32 << h) - 1)) as usize;
    let s0 = fold_point(h, index);
    let folded0 = fold_pair(inp.e0, inp.e1, inp.beta, s0);

    let mut row = vec![Val::ZERO; n_cols(cfg)];
    row[C_DRAW_OUT] = inp.draw_out;
    for i in 0..FIELD_BITS {
        row[C_BITS + i] = Val::from_u32((canon >> i) & 1);
    }
    row[C_INDEX] = Val::from_u32(index as u32);
    row[C_E0..C_E0 + D].copy_from_slice(&limbs(inp.e0));
    row[C_E1..C_E1 + D].copy_from_slice(&limbs(inp.e1));
    row[C_BETA..C_BETA + D].copy_from_slice(&limbs(inp.beta));
    row[C_S] = s0;
    row[C_FOLDED..C_FOLDED + D].copy_from_slice(&limbs(folded0));

    // (2) product tower.
    let bases = tower_bases(h);
    let mut acc = Val::ONE;
    row[sp_col(0)] = acc;
    for j in 0..h {
        let rev_bit = (canon >> (h - 1 - j)) & 1 == 1;
        if rev_bit {
            acc *= bases[j];
        }
        row[sp_col(j + 1)] = acc;
    }
    debug_assert_eq!(acc, s0, "tower product != fold_point — base/bit convention mismatch");

    // (c) descent rounds r = 1..num_rounds: s_r = s_{r-1}², e0_r = folded_{r-1}, e1_r/beta_r given.
    let rounds = cfg.rounds();
    if rounds > 1 {
        assert_eq!(inp.descent.len(), rounds - 1, "descent must supply rounds-1 (e1,beta) pairs");
    }
    let mut s_prev = s0;
    let mut folded_prev = folded0;
    for r in 1..rounds {
        let (e1_r, beta_r) = inp.descent[r - 1];
        let e0_r = folded_prev; // chain-link
        let s_r = s_prev * s_prev; // point-square
        let folded_r = fold_pair(e0_r, e1_r, beta_r, s_r);
        let base = extra_round_base(cfg, r);
        row[base..base + D].copy_from_slice(&limbs(e0_r));
        row[base + D..base + 2 * D].copy_from_slice(&limbs(e1_r));
        row[base + 2 * D..base + 3 * D].copy_from_slice(&limbs(beta_r));
        row[base + 3 * D] = s_r;
        row[base + 3 * D + 1..base + 4 * D + 1].copy_from_slice(&limbs(folded_r));
        s_prev = s_r;
        folded_prev = folded_r;
    }

    // (b′) DEEP OOD block.
    if let Some(k) = cfg.deep_log_degree {
        let zh = crate::deep::vanishing_poly_at(inp.deep_zeta, k);
        let c = zh * inp.deep_q;
        let db = deep_base(cfg);
        row[db + DEEP_ZETA..db + DEEP_ZETA + D].copy_from_slice(&limbs(inp.deep_zeta));
        row[db + DEEP_Q..db + DEEP_Q + D].copy_from_slice(&limbs(inp.deep_q));
        row[db + DEEP_C..db + DEEP_C + D].copy_from_slice(&limbs(c));
        let mut p = inp.deep_zeta;
        for j in 0..=k {
            row[db + DEEP_POW + j * D..db + DEEP_POW + j * D + D].copy_from_slice(&limbs(p));
            p = p * p;
        }
    }

    // (b) challenger-draw columns: one per bound challenge (= the challenge limbs themselves; the
    // binding constraint pins beta/zeta == these draw columns). Honest builder copies the limbs.
    if cfg.bind_beta_to_draws {
        let mut dbase = draw_block_base(cfg);
        // round betas (round 0 beta, then descent betas).
        let mut betas: Vec<Challenge> = Vec::with_capacity(rounds);
        betas.push(inp.beta);
        for r in 1..rounds {
            betas.push(inp.descent[r - 1].1);
        }
        for b in &betas {
            row[dbase..dbase + D].copy_from_slice(&limbs(*b));
            dbase += D;
        }
        if cfg.deep_log_degree.is_some() {
            row[dbase..dbase + D].copy_from_slice(&limbs(inp.deep_zeta));
        }
    }

    // (a) Merkle path inlined level-by-level. Index bit at level j = bit_j of the bus index.
    if cfg.merkle_levels > 0 {
        assert_eq!(
            inp.merkle_siblings.len(),
            cfg.merkle_levels,
            "merkle_siblings must have merkle_levels entries"
        );
        let consts = inlined_perm::Constants::pinned();
        let mut running = inp.merkle_leaf;
        let mut idx = index;
        for level in 0..cfg.merkle_levels {
            let bit = (idx & 1) == 1;
            let sib = inp.merkle_siblings[level];
            // placement exactly like merkle_air::pack_level / fold_step.
            let mut input = [Val::ZERO; WIDTH];
            let (a, b) = if bit { (sib, running) } else { (running, sib) };
            input[..DIGEST_ELEMS].copy_from_slice(&a);
            input[DIGEST_ELEMS..].copy_from_slice(&b);
            let perm_cols = inlined_perm::generate_perm_cols(input, &consts);
            let lbase = merkle_level_base(cfg, level);
            row[lbase + ML_RUNNING..lbase + ML_RUNNING + DIGEST_ELEMS].copy_from_slice(&running);
            row[lbase + ML_SIBLING..lbase + ML_SIBLING + DIGEST_ELEMS].copy_from_slice(&sib);
            row[lbase + ML_PERM..lbase + ML_PERM + PERM_COLS].copy_from_slice(&perm_cols);
            // next running = perm output[..8].
            running.copy_from_slice(&perm_cols[C_OUTPUT..C_OUTPUT + DIGEST_ELEMS]);
            idx >>= 1;
        }
    }

    row
}

/// The reconstructed Merkle root of a packed window (last level's perm output[..8]).
pub fn window_merkle_root(cfg: &RecursiveConfig, row: &[Val]) -> [Val; DIGEST_ELEMS] {
    assert!(cfg.merkle_levels > 0);
    let lbase = merkle_level_base(cfg, cfg.merkle_levels - 1);
    let mut out = [Val::ZERO; DIGEST_ELEMS];
    out.copy_from_slice(&row[lbase + ML_PERM + C_OUTPUT..lbase + ML_PERM + C_OUTPUT + DIGEST_ELEMS]);
    out
}

/// Decode the bus index value of a packed window.
pub fn window_index(row: &[Val]) -> u32 {
    row[C_INDEX].as_canonical_u32()
}

/// Public values layout for the Merkle block: `leaf(DIGEST_ELEMS) ‖ root(DIGEST_ELEMS)`. Present
/// only when `merkle_levels > 0`. (No public values otherwise.)
pub const PV_LEAF: usize = 0;
pub const PV_ROOT: usize = PV_LEAF + DIGEST_ELEMS;
pub const N_PUBLIC_MERKLE: usize = PV_ROOT + DIGEST_ELEMS;

/// Octic multiply over `X^D = W` as in-AIR expressions (same formula as `fri_fold_air`).
fn octic_mul<AB: AirBuilder>(a: &[AB::Expr; D], b: &[AB::Expr; D]) -> [AB::Expr; D] {
    let w = AB::Expr::from(AB::F::from_u32(W));
    core::array::from_fn(|k| {
        let mut acc = AB::Expr::ZERO;
        for i in 0..D {
            for j in 0..D {
                if i + j == k {
                    acc += a[i].clone() * b[j].clone();
                } else if i + j == k + D {
                    acc += w.clone() * a[i].clone() * b[j].clone();
                }
            }
        }
        acc
    })
}

/// In-AIR cleared FRI fold identity (★) `folded·2s == e0(beta+s) − e1(beta−s)` over deg-8.
fn fold_identity<AB: AirBuilder>(
    builder: &mut AB,
    e0: &[AB::Expr; D],
    e1: &[AB::Expr; D],
    beta: &[AB::Expr; D],
    s: AB::Expr,
    folded: &[AB::Expr; D],
) {
    let mut beta_plus_s = beta.clone();
    beta_plus_s[0] = beta[0].clone() + s.clone();
    let mut beta_minus_s = beta.clone();
    beta_minus_s[0] = beta[0].clone() - s.clone();
    let p0 = octic_mul::<AB>(e0, &beta_plus_s);
    let p1 = octic_mul::<AB>(e1, &beta_minus_s);
    let two_s = s.double();
    for k in 0..D {
        builder.assert_zero(folded[k].clone() * two_s.clone() - (p0[k].clone() - p1[k].clone()));
    }
}

/// `recursive_air` (capstone): index/challenge bus + all consumers in one window. Wires
/// (1) index↔draw, (2) fold point↔index tower, (3) FRI fold (★), (a) Merkle index↔bus,
/// (b) beta/zeta↔draws, (b′) DEEP OOD, (c) multi-round descent — config-gated.
pub struct RecursiveAir {
    pub cfg: RecursiveConfig,
}

impl RecursiveAir {
    /// Legacy single-window constructor (index↔draw + tower + single fold).
    pub fn single(log_height: usize) -> Self {
        Self { cfg: RecursiveConfig::single_window(log_height) }
    }
}

impl<F: Sync> BaseAir<F> for RecursiveAir {
    fn width(&self) -> usize {
        n_cols(&self.cfg)
    }
}

/// Reads the window's perm sub-block at an absolute offset (for the Merkle inlined perms).
struct WindowCells<AB: AirBuilder<F = Val>> {
    row: Vec<AB::Var>,
}
impl<AB: AirBuilder<F = Val>> PermCells<AB> for WindowCells<AB> {
    fn at(&self, idx: usize) -> AB::Expr {
        self.row[idx].into()
    }
}

impl<AB: AirBuilder<F = Val>> Air<AB> for RecursiveAir {
    fn eval(&self, builder: &mut AB) {
        let cfg = self.cfg;
        let h = cfg.log_height;
        let main = builder.main();
        let local: Vec<AB::Var> = main.current_slice().to_vec();
        let l = |i: usize| -> AB::Expr { local[i].into() };
        let two = AB::F::from_u32(2);

        // ===== (1) index ↔ challenger draw (reuse index_bus logic) =====
        let bits: Vec<AB::Expr> = (0..FIELD_BITS).map(|i| l(C_BITS + i)).collect();
        for b in &bits {
            builder.assert_zero(b.clone() * (b.clone() - AB::Expr::ONE)); // booleanity
        }
        let mut acc = AB::Expr::ZERO;
        let mut pow = AB::F::ONE;
        for b in &bits {
            acc += b.clone() * AB::Expr::from(pow);
            pow *= two;
        }
        builder.assert_zero(acc - l(C_DRAW_OUT));
        assert_canonical_pub::<AB>(builder, &bits);
        let mut idx = AB::Expr::ZERO;
        let mut pow = AB::F::ONE;
        for i in 0..h {
            idx += bits[i].clone() * AB::Expr::from(pow);
            pow *= two;
        }
        builder.assert_zero(idx - l(C_INDEX));

        // ===== (2) fold point s ↔ index (committed product tower) =====
        let bases = tower_bases(h);
        builder.assert_zero(l(sp_col(0)) - AB::Expr::ONE);
        for j in 0..h {
            let rev_bit = bits[h - 1 - j].clone();
            let base_j = AB::Expr::from(bases[j]);
            let sel = AB::Expr::ONE + rev_bit * (base_j - AB::Expr::ONE);
            builder.assert_zero(l(sp_col(j + 1)) - l(sp_col(j)) * sel);
        }
        builder.assert_zero(l(C_S) - l(sp_col(h)));

        // ===== (3) FRI fold identity (★) at the BUS-BOUND point s (round 0) =====
        let e0: [AB::Expr; D] = core::array::from_fn(|i| l(C_E0 + i));
        let e1: [AB::Expr; D] = core::array::from_fn(|i| l(C_E1 + i));
        let beta: [AB::Expr; D] = core::array::from_fn(|i| l(C_BETA + i));
        let folded: [AB::Expr; D] = core::array::from_fn(|i| l(C_FOLDED + i));
        fold_identity::<AB>(builder, &e0, &e1, &beta, l(C_S), &folded);

        // ===== (c) multi-round descent: per-round (★) + chain-link + point-square =====
        let rounds = cfg.rounds();
        // Round-r reader helpers: round 0 uses the canonical columns; r≥1 uses the extra block.
        let round_e0 = |r: usize| -> [AB::Expr; D] {
            if r == 0 {
                core::array::from_fn(|i| l(C_E0 + i))
            } else {
                let b = extra_round_base(&cfg, r);
                core::array::from_fn(|i| l(b + i))
            }
        };
        let round_e1 = |r: usize| -> [AB::Expr; D] {
            if r == 0 {
                core::array::from_fn(|i| l(C_E1 + i))
            } else {
                let b = extra_round_base(&cfg, r);
                core::array::from_fn(|i| l(b + D + i))
            }
        };
        let round_beta = |r: usize| -> [AB::Expr; D] {
            if r == 0 {
                core::array::from_fn(|i| l(C_BETA + i))
            } else {
                let b = extra_round_base(&cfg, r);
                core::array::from_fn(|i| l(b + 2 * D + i))
            }
        };
        let round_s = |r: usize| -> AB::Expr {
            if r == 0 {
                l(C_S)
            } else {
                let b = extra_round_base(&cfg, r);
                l(b + 3 * D)
            }
        };
        let round_folded = |r: usize| -> [AB::Expr; D] {
            if r == 0 {
                core::array::from_fn(|i| l(C_FOLDED + i))
            } else {
                let b = extra_round_base(&cfg, r);
                core::array::from_fn(|i| l(b + 3 * D + 1 + i))
            }
        };
        for r in 1..rounds {
            // per-round (★) at this round's point.
            let er0 = round_e0(r);
            let er1 = round_e1(r);
            let erb = round_beta(r);
            let erf = round_folded(r);
            fold_identity::<AB>(builder, &er0, &er1, &erb, round_s(r), &erf);
            // (point-square): s_r == s_{r-1}².
            let s_prev = round_s(r - 1);
            builder.assert_zero(round_s(r) - s_prev.clone() * s_prev);
            // (chain-link): e0_r == folded_{r-1}, limb-for-limb.
            let prev_folded = round_folded(r - 1);
            for k in 0..D {
                builder.assert_zero(er0[k].clone() - prev_folded[k].clone());
            }
        }

        // ===== (b′) DEEP OOD block: C == Z_H(zeta)·Q with squaring tower =====
        if let Some(k) = cfg.deep_log_degree {
            let db = deep_base(&cfg);
            let zeta: [AB::Expr; D] = core::array::from_fn(|i| l(db + DEEP_ZETA + i));
            let q: [AB::Expr; D] = core::array::from_fn(|i| l(db + DEEP_Q + i));
            let c: [AB::Expr; D] = core::array::from_fn(|i| l(db + DEEP_C + i));
            // pow[0] == zeta.
            let pow0: [AB::Expr; D] = core::array::from_fn(|i| l(db + DEEP_POW + i));
            for i in 0..D {
                builder.assert_zero(pow0[i].clone() - zeta[i].clone());
            }
            // pow[j] == pow[j-1]².
            for j in 1..=k {
                let prev: [AB::Expr; D] =
                    core::array::from_fn(|i| l(db + DEEP_POW + (j - 1) * D + i));
                let cur: [AB::Expr; D] = core::array::from_fn(|i| l(db + DEEP_POW + j * D + i));
                let sq = octic_mul::<AB>(&prev, &prev);
                for i in 0..D {
                    builder.assert_zero(cur[i].clone() - sq[i].clone());
                }
            }
            // Z_H = pow[k] - 1; C == Z_H·Q.
            let powk: [AB::Expr; D] = core::array::from_fn(|i| l(db + DEEP_POW + k * D + i));
            let mut zh = powk;
            zh[0] = zh[0].clone() - AB::Expr::ONE;
            let zhq = octic_mul::<AB>(&zh, &q);
            for i in 0..D {
                builder.assert_zero(c[i].clone() - zhq[i].clone());
            }
        }

        // ===== (b) beta / zeta ↔ challenger SampleDraw columns =====
        if cfg.bind_beta_to_draws {
            let mut dbase = draw_block_base(&cfg);
            // bind each round beta to its draw block.
            for r in 0..rounds {
                let br = round_beta(r);
                for i in 0..D {
                    builder.assert_zero(br[i].clone() - l(dbase + i));
                }
                dbase += D;
            }
            // bind zeta to its draw block.
            if cfg.deep_log_degree.is_some() {
                let db = deep_base(&cfg);
                for i in 0..D {
                    builder.assert_zero(l(db + DEEP_ZETA + i) - l(dbase + i));
                }
            }
        }

        // ===== (a) Merkle path: inlined perms, index bit ↔ bus, chain, leaf/root public =====
        if cfg.merkle_levels > 0 {
            let consts = ConstantsExpr::<AB>::pinned();
            let cells = WindowCells::<AB> { row: local.clone() };
            let one = AB::Expr::ONE;
            for level in 0..cfg.merkle_levels {
                let lbase = merkle_level_base(&cfg, level);
                // index bit at this level IS bit_level of the bus decomposition (the binding).
                let bit = bits[level].clone();
                // placement: perm.inputs[..8]/[8..] = running/sibling per the BUS bit.
                for i in 0..DIGEST_ELEMS {
                    let running_i = l(lbase + ML_RUNNING + i);
                    let sibling_i = l(lbase + ML_SIBLING + i);
                    let in_a = l(lbase + ML_PERM + inlined_perm::C_INPUTS + i);
                    let in_b = l(lbase + ML_PERM + inlined_perm::C_INPUTS + DIGEST_ELEMS + i);
                    builder.assert_zero(
                        in_a - ((one.clone() - bit.clone()) * running_i.clone()
                            + bit.clone() * sibling_i.clone()),
                    );
                    builder.assert_zero(
                        in_b - ((one.clone() - bit.clone()) * sibling_i
                            + bit.clone() * running_i),
                    );
                }
                // inlined Poseidon2 round constraints over this level's perm block.
                eval_perm::<AB, _>(builder, &cells, lbase + ML_PERM, &consts);
                // chain: next level's running == this level's output (within the same row).
                if level + 1 < cfg.merkle_levels {
                    let nbase = merkle_level_base(&cfg, level + 1);
                    for i in 0..DIGEST_ELEMS {
                        let this_out = l(lbase + ML_PERM + C_OUTPUT + i);
                        let next_running = l(nbase + ML_RUNNING + i);
                        builder.assert_zero(next_running - this_out);
                    }
                }
            }
            // leaf/root binding to public values (leaf = level-0 running, root = last output).
            let pubs: Vec<AB::Expr> =
                builder.public_values().iter().map(|p| (*p).into()).collect();
            if pubs.len() == N_PUBLIC_MERKLE {
                let l0 = merkle_level_base(&cfg, 0);
                for i in 0..DIGEST_ELEMS {
                    builder.assert_zero(l(l0 + ML_RUNNING + i) - pubs[PV_LEAF + i].clone());
                }
                let llast = merkle_level_base(&cfg, cfg.merkle_levels - 1);
                for i in 0..DIGEST_ELEMS {
                    builder.assert_zero(
                        l(llast + ML_PERM + C_OUTPUT + i) - pubs[PV_ROOT + i].clone(),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fri_fold::reverse_bits_len;
    use crate::merkle::{compressor, Digest};
    use p3_air::check_all_constraints;
    use p3_field::{BasedVectorSpace, PrimeCharacteristicRing};
    use p3_matrix::dense::RowMajorMatrix;
    use p3_symmetric::PseudoCompressionFunction;

    fn ext(x: u32) -> Challenge {
        Challenge::from(Val::from_u32(x))
    }
    fn ext_multi(seed: u32) -> Challenge {
        Challenge::from_basis_coefficients_fn(|i| {
            Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32 * 7 + 1))
        })
    }
    fn cfg_single(h: usize) -> RecursiveConfig {
        RecursiveConfig::single_window(h)
    }
    fn air(cfg: RecursiveConfig) -> RecursiveAir {
        RecursiveAir { cfg }
    }
    /// Single-fold matrix (legacy shape) from (draw, e0, e1, beta) tuples.
    fn matrix_single(
        queries: &[(Val, Challenge, Challenge, Challenge)],
        h: usize,
    ) -> RowMajorMatrix<Val> {
        let cfg = cfg_single(h);
        let mut flat = Vec::new();
        for &(d, e0, e1, beta) in queries {
            flat.extend_from_slice(&pack_window(&cfg, &WindowInputs::single(d, e0, e1, beta)));
        }
        RowMajorMatrix::new(flat, n_cols(&cfg))
    }

    // ---------------------------------------------------------------------------------
    // Existing single-window cross-bindings (1)(2)(3) — preserved.
    // ---------------------------------------------------------------------------------

    #[test]
    fn tower_reproduces_native_fold_point() {
        for h in [4usize, 5, 8, 12, 20] {
            for raw in [0u32, 1, 3, 6, 17, 0x9999, 0xABCDE, 1_000_003] {
                let d = Val::from_u32(raw);
                let cfg = cfg_single(h);
                let row = pack_window(&cfg, &WindowInputs::single(d, ext(1), ext(2), ext(3)));
                let index = (raw & ((1 << h) - 1)) as usize;
                let expected = fold_point(h, index);
                assert_eq!(row[C_S], expected, "tower s != fold_point at h={h}, raw={raw:#x}");
                assert_eq!(row[sp_col(h)], expected, "sp[h] != fold_point");
                let g = Val::two_adic_generator(h + 1);
                assert_eq!(expected, g.exp_u64(reverse_bits_len(index, h) as u64));
            }
        }
    }

    #[test]
    fn air_accepts_honest_windows() {
        for &h in &[5usize, 8, 12] {
            let queries: Vec<_> = [0u32, 1, 7, 0x9999, 0xABCDE, 1_000_003]
                .iter()
                .enumerate()
                .map(|(k, &raw)| {
                    let kk = k as u32;
                    (Val::from_u32(raw), ext_multi(10 + kk), ext_multi(40 + kk), ext_multi(70 + kk))
                })
                .collect();
            let report = check_all_constraints(&air(cfg_single(h)), &matrix_single(&queries, h), &[], None);
            assert!(report.is_ok(), "honest windows rejected at h={h}: {} failures", report.failures.len());
        }
    }

    #[test]
    fn air_rejects_forged_index() {
        let h = 12;
        let mut m = matrix_single(&[(Val::from_u32(0xABCDE), ext_multi(1), ext_multi(2), ext_multi(3))], h);
        m.values[C_INDEX] += Val::ONE;
        assert!(!check_all_constraints(&air(cfg_single(h)), &m, &[], None).is_ok());
    }

    #[test]
    fn air_rejects_forged_fold_point() {
        let h = 8;
        let draw = Val::from_u32(0xABCDE);
        let canon = draw.as_canonical_u32();
        let honest_index = (canon & ((1 << h) - 1)) as usize;
        let forged_index = honest_index ^ 1;
        let s_forged = fold_point(h, forged_index);
        assert_ne!(fold_point(h, honest_index), s_forged);
        let cfg = cfg_single(h);
        let mut row = pack_window(&cfg, &WindowInputs::single(draw, ext_multi(4), ext_multi(5), ext_multi(6)));
        row[C_S] = s_forged;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    #[test]
    fn air_rejects_tampered_tower() {
        let h = 12;
        let cfg = cfg_single(h);
        let mut row = pack_window(&cfg, &WindowInputs::single(Val::from_u32(0x55555), ext_multi(7), ext_multi(8), ext_multi(9)));
        row[sp_col(6)] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    #[test]
    fn air_rejects_bit_index_point_desync() {
        let h = 8;
        let draw = Val::from_u32(0xABCD0);
        let cfg = cfg_single(h);
        let mut row = pack_window(&cfg, &WindowInputs::single(draw, ext_multi(1), ext_multi(2), ext_multi(3)));
        row[C_BITS + 0] = Val::ONE;
        row[C_DRAW_OUT] += Val::ONE;
        row[C_INDEX] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    #[test]
    fn air_rejects_tampered_eval() {
        let h = 12;
        let mut m = matrix_single(&[(Val::from_u32(0x1234), ext_multi(11), ext_multi(12), ext_multi(13))], h);
        m.values[C_E0 + 2] += Val::ONE;
        assert!(!check_all_constraints(&air(cfg_single(h)), &m, &[], None).is_ok());
    }

    #[test]
    fn air_rejects_wrong_folded() {
        let h = 8;
        let mut m = matrix_single(&[(Val::from_u32(0x9999), ext_multi(14), ext_multi(15), ext_multi(16))], h);
        m.values[C_FOLDED + 1] += Val::ONE;
        assert!(!check_all_constraints(&air(cfg_single(h)), &m, &[], None).is_ok());
    }

    #[test]
    fn air_rejects_tampered_beta() {
        let h = 12;
        let mut m = matrix_single(&[(Val::from_u32(0x4242), ext_multi(17), ext_multi(18), ext_multi(19))], h);
        m.values[C_BETA + 4] += Val::ONE;
        assert!(!check_all_constraints(&air(cfg_single(h)), &m, &[], None).is_ok());
    }

    #[test]
    fn air_binds_draw_to_index_to_fold_point() {
        let h = 10;
        let draw = Val::from_u32(0x3ABCD);
        let cfg = cfg_single(h);
        let honest = matrix_single(&[(draw, ext_multi(21), ext_multi(22), ext_multi(23))], h);
        assert!(check_all_constraints(&air(cfg), &honest, &[], None).is_ok());

        let honest_index = (draw.as_canonical_u32() & ((1 << h) - 1)) as usize;
        let forged_index = honest_index ^ 0b101;
        let s_forged = fold_point(h, forged_index);
        let (e0, e1, beta) = (ext_multi(31), ext_multi(32), ext_multi(33));
        let folded = fold_pair(e0, e1, beta, s_forged);
        let mut row = pack_window(&cfg, &WindowInputs::single(draw, e0, e1, beta));
        row[C_S] = s_forged;
        row[C_FOLDED..C_FOLDED + D].copy_from_slice(&limbs(folded));
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "fold at a forged-index point accepted — draw→index→point binding not load-bearing"
        );
    }

    #[test]
    fn air_accepts_multirow() {
        let h = 12;
        let queries: Vec<_> = (0..8u32)
            .map(|q| {
                (Val::from_u32(0x40000 + q * 0x1111), ext_multi(100 + q), ext_multi(200 + q), ext_multi(300 + q))
            })
            .collect();
        let report = check_all_constraints(&air(cfg_single(h)), &matrix_single(&queries, h), &[], None);
        assert!(report.is_ok(), "honest multi-row recursive_air trace rejected: {} failures", report.failures.len());
    }

    // ---------------------------------------------------------------------------------
    // (a) MERKLE INDEX ↔ BUS — Merkle path inlined in the window, index bit bound to bits[level].
    // ---------------------------------------------------------------------------------

    fn digest(seed: u32) -> Digest {
        core::array::from_fn(|i| Val::from_u32(seed.wrapping_mul(2654435761).wrapping_add(i as u32)))
    }

    /// Reference binary Merkle tree built with the deployed compression (mirror of merkle_air).
    struct RefTree {
        levels: Vec<Vec<Digest>>,
    }
    impl RefTree {
        fn new(mut leaves: Vec<Digest>) -> Self {
            let c = compressor();
            let mut levels = vec![leaves.clone()];
            while leaves.len() > 1 {
                let next: Vec<Digest> =
                    leaves.chunks_exact(2).map(|p| c.compress([p[0], p[1]])).collect();
                levels.push(next.clone());
                leaves = next;
            }
            Self { levels }
        }
        fn root(&self) -> Digest {
            self.levels.last().unwrap()[0]
        }
        fn auth_path(&self, index: usize) -> Vec<Digest> {
            let mut sibs = Vec::new();
            let mut idx = index;
            for level in &self.levels[..self.levels.len() - 1] {
                sibs.push(level[idx ^ 1]);
                idx >>= 1;
            }
            sibs
        }
    }

    fn pubs(leaf: Digest, root: Digest) -> Vec<Val> {
        let mut p = Vec::with_capacity(N_PUBLIC_MERKLE);
        p.extend_from_slice(&leaf);
        p.extend_from_slice(&root);
        p
    }

    /// Config with a Merkle block of `levels` levels at log-height = levels (so the bus index has
    /// exactly the bits the path consumes, and the leaf-layer fold point uses the same index).
    fn cfg_merkle(levels: usize) -> RecursiveConfig {
        RecursiveConfig { merkle_levels: levels, ..RecursiveConfig::single_window(levels) }
    }

    /// Build a Merkle-augmented window for the query whose index is `draw`'s low `levels` bits,
    /// opening leaf `leaf` in `tree`.
    fn merkle_window(cfg: &RecursiveConfig, draw: Val, tree: &RefTree, leaf: Digest, index: usize) -> Vec<Val> {
        let sibs = tree.auth_path(index);
        let inp = WindowInputs {
            draw_out: draw,
            e0: ext_multi(1),
            e1: ext_multi(2),
            beta: ext_multi(3),
            descent: &[],
            merkle_leaf: leaf,
            merkle_siblings: &sibs,
            deep_zeta: Challenge::ZERO,
            deep_q: Challenge::ZERO,
        };
        pack_window(cfg, &inp)
    }

    /// GOLDEN: an honest Merkle path opened at the FS-forced index (= draw's low bits) ACCEPTS,
    /// folding to the public root, with the per-level index bits BOUND to the bus decomposition.
    #[test]
    fn air_accepts_merkle_path_bound_to_bus() {
        let levels = 4usize;
        let cfg = cfg_merkle(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        // pick draws whose low `levels` bits hit several distinct indices.
        for raw in [0u32, 1, 6, 0xB, 0xD, 0xF, 0xABCDA] {
            let draw = Val::from_u32(raw);
            let index = (draw.as_canonical_u32() as usize) & (n - 1);
            let row = merkle_window(&cfg, draw, &tree, leaves[index], index);
            let m = RowMajorMatrix::new(row, n_cols(&cfg));
            let report = check_all_constraints(&air(cfg), &m, &pubs(leaves[index], root), None);
            assert!(report.is_ok(), "honest bus-bound Merkle path rejected (raw={raw:#x}): {} failures", report.failures.len());
        }
    }

    /// (a) FORGERY — MERKLE AT THE WRONG INDEX: open the Merkle path at a DIFFERENT index than the
    /// bus-forced query (the fold point's index). The path's per-level index bits then disagree
    /// with `bits[level]` of the draw decomposition ⇒ the placement constraint (bound to the bus)
    /// rejects. THIS is binding (a): the Merkle opening and the FRI fold must answer the SAME query.
    #[test]
    fn air_rejects_merkle_at_wrong_index() {
        let levels = 4usize;
        let cfg = cfg_merkle(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        // Honest bus index from the draw.
        let draw = Val::from_u32(0b0110);
        let honest_index = (draw.as_canonical_u32() as usize) & (n - 1);
        // Build a path/leaf for a DIFFERENT index (a favorable query the prover wants to answer).
        let forged_index = honest_index ^ 0b101;
        assert_ne!(forged_index, honest_index);
        // Pack the Merkle blocks for the FORGED index (low bits = forged_index), then overwrite the
        // bus block (draw_out/bits/index/tower/fold) to the HONEST query. Now bits[level] say
        // honest_index but the Merkle perm placements are for forged_index ⇒ (a) binding rejects.
        let forged_draw = Val::from_u32(forged_index as u32);
        let sibs = tree.auth_path(forged_index);
        let inp = WindowInputs {
            draw_out: forged_draw,
            e0: ext_multi(1), e1: ext_multi(2), beta: ext_multi(3),
            descent: &[],
            merkle_leaf: leaves[forged_index],
            merkle_siblings: &sibs,
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO,
        };
        let mut row = pack_window(&cfg, &inp);
        // CONTROL: the FORGED Merkle blocks are internally valid — with a MATCHING forged bus
        // (bits = forged_index), the same row ACCEPTS (every Merkle path folds to the same tree
        // root; leaf/root boundaries hold). So the ONLY thing that can reject the swapped-bus row
        // below is the index-bit↔bus binding (a), not the perm/fold/root machinery.
        let control = RowMajorMatrix::new(row.clone(), n_cols(&cfg));
        assert!(
            check_all_constraints(&air(cfg), &control, &pubs(leaves[forged_index], root), None).is_ok(),
            "forged-index control row should accept with its own (forged) bus — test would prove nothing otherwise"
        );
        // Now overwrite the bus block (draw_out/bits/index/tower/fold) with the HONEST query. The
        // Merkle perm placements remain for forged_index; bits[level] now say honest_index.
        let honest = pack_window(&cfg, &WindowInputs {
            draw_out: draw, e0: ext_multi(1), e1: ext_multi(2), beta: ext_multi(3),
            descent: &[], merkle_leaf: leaves[honest_index],
            merkle_siblings: &tree.auth_path(honest_index),
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO,
        });
        for c in C_DRAW_OUT..tower_end(levels) {
            row[c] = honest[c];
        }
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &pubs(leaves[forged_index], root), None).is_ok(),
            "a Merkle path opened at a DIFFERENT index than the fold point was accepted — (a) not load-bearing"
        );
    }

    /// (a) FORGERY — tampered sibling ⇒ recomputed node diverges ⇒ wrong root ⇒ rejected.
    #[test]
    fn air_rejects_merkle_tampered_sibling() {
        let levels = 4usize;
        let cfg = cfg_merkle(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let draw = Val::from_u32(0b0011);
        let index = (draw.as_canonical_u32() as usize) & (n - 1);
        let mut row = merkle_window(&cfg, draw, &tree, leaves[index], index);
        let lbase = merkle_level_base(&cfg, 0);
        row[lbase + ML_SIBLING] += Val::ONE; // tamper the level-0 sibling
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &pubs(leaves[index], root), None).is_ok());
    }

    /// (a) FORGERY — a tampered INTERIOR perm cell of a Merkle level ⇒ rejected by the inlined
    /// Poseidon2 round constraints (the perm sub-block is fully constrained inside the window).
    #[test]
    fn air_rejects_merkle_tampered_perm_interior() {
        let levels = 4usize;
        let cfg = cfg_merkle(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let draw = Val::from_u32(0b0101);
        let index = (draw.as_canonical_u32() as usize) & (n - 1);
        let mut row = merkle_window(&cfg, draw, &tree, leaves[index], index);
        let lbase = merkle_level_base(&cfg, 0);
        row[lbase + ML_PERM + inlined_perm::C_BEGIN_POST] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &pubs(leaves[index], root), None).is_ok());
    }

    /// (a) FORGERY — wrong root public value ⇒ rejected by the last-level root boundary.
    #[test]
    fn air_rejects_merkle_wrong_root() {
        let levels = 4usize;
        let cfg = cfg_merkle(levels);
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let draw = Val::from_u32(0b1001);
        let index = (draw.as_canonical_u32() as usize) & (n - 1);
        let row = merkle_window(&cfg, draw, &tree, leaves[index], index);
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        let mut bad_root = tree.root();
        bad_root[0] += Val::ONE;
        assert!(!check_all_constraints(&air(cfg), &m, &pubs(leaves[index], bad_root), None).is_ok());
    }

    // ---------------------------------------------------------------------------------
    // (b) beta / zeta ↔ challenger SampleDraw columns.
    // ---------------------------------------------------------------------------------

    fn cfg_beta_bound(h: usize) -> RecursiveConfig {
        RecursiveConfig { bind_beta_to_draws: true, ..RecursiveConfig::single_window(h) }
    }

    /// GOLDEN: an honest window with `beta` bound to its challenger-draw columns ACCEPTS.
    #[test]
    fn air_accepts_beta_bound_to_draws() {
        let h = 10;
        let cfg = cfg_beta_bound(h);
        let inp = WindowInputs::single(Val::from_u32(0xABCDE), ext_multi(11), ext_multi(12), ext_multi(13));
        let row = pack_window(&cfg, &inp);
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(check_all_constraints(&air(cfg), &m, &[], None).is_ok(), "honest beta-draw-bound window rejected");
    }

    /// (b) FORGERY — a forged beta NOT equal to the challenger draw: swap the beta limbs to a
    /// different challenge (self-consistent (★) at the swapped beta), but leave the draw columns at
    /// the transcript value. The (b) equality `beta == draw` rejects — ONLY (b) catches this, so
    /// the binding is load-bearing.
    #[test]
    fn air_rejects_forged_beta() {
        let h = 10;
        let cfg = cfg_beta_bound(h);
        let draw = Val::from_u32(0xABCDE);
        let s = {
            let index = (draw.as_canonical_u32() as usize) & ((1 << h) - 1);
            fold_point(h, index)
        };
        let honest_beta = ext_multi(13);
        let forged_beta = ext_multi(99);
        assert_ne!(honest_beta, forged_beta);
        let (e0, e1) = (ext_multi(11), ext_multi(12));
        let mut row = pack_window(&cfg, &WindowInputs::single(draw, e0, e1, honest_beta));
        let folded_forged = fold_pair(e0, e1, forged_beta, s);
        row[C_BETA..C_BETA + D].copy_from_slice(&limbs(forged_beta));
        row[C_FOLDED..C_FOLDED + D].copy_from_slice(&limbs(folded_forged));
        // CONTROL: the (★) fold is self-consistent at forged_beta — if the draw columns ALSO carry
        // forged_beta the row ACCEPTS, proving only the beta↔draw equality (b) rejects below.
        let mut control = row.clone();
        let dbase = draw_block_base(&cfg);
        control[dbase..dbase + D].copy_from_slice(&limbs(forged_beta));
        assert!(
            check_all_constraints(&air(cfg), &RowMajorMatrix::new(control, n_cols(&cfg)), &[], None).is_ok(),
            "forged-beta control (draw == beta) should accept — test would prove nothing otherwise"
        );
        // draw columns still carry honest_beta ⇒ (b) equality fails.
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a beta != the challenger draw was accepted — (b) binding not load-bearing"
        );
    }

    /// (b) FORGERY — a tampered challenger-draw column (≠ beta) is rejected too (binding is an
    /// equality, so corrupting EITHER side breaks it).
    #[test]
    fn air_rejects_tampered_beta_draw_column() {
        let h = 10;
        let cfg = cfg_beta_bound(h);
        let inp = WindowInputs::single(Val::from_u32(0x12345), ext_multi(21), ext_multi(22), ext_multi(23));
        let mut row = pack_window(&cfg, &inp);
        let dbase = draw_block_base(&cfg);
        row[dbase + 2] += Val::ONE; // corrupt a draw limb
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    // ---------------------------------------------------------------------------------
    // (c) MULTI-ROUND DESCENT — fri_chain_air chaining threaded into the window.
    // ---------------------------------------------------------------------------------

    fn cfg_descent(h: usize, rounds: usize) -> RecursiveConfig {
        RecursiveConfig { num_rounds: rounds, ..RecursiveConfig::single_window(h) }
    }

    /// Honest descent inputs: fold a degree-`2^rounds` poly through `rounds` commit-phase folds,
    /// returning (e0_0, e1_0, beta_0, [(e1_r, beta_r); rounds-1]). Mirrors fri_chain_air's helper.
    fn honest_descent(
        coeffs: &[Challenge],
        betas: &[Challenge],
        h0: usize,
        index: usize,
    ) -> (Challenge, Challenge, Challenge, Vec<(Challenge, Challenge)>) {
        let eval = |c: &[Challenge], x: Val| -> Challenge {
            let mut acc = Challenge::ZERO;
            let mut p = Challenge::ONE;
            for &ci in c {
                acc += ci * p;
                p *= Challenge::from(x);
            }
            acc
        };
        let fold_poly = |c: &[Challenge], beta: Challenge| -> Vec<Challenge> {
            let half = c.len() / 2;
            (0..half).map(|k| c[2 * k] + beta * c[2 * k + 1]).collect()
        };
        let mut c = coeffs.to_vec();
        let s0 = fold_point(h0, index);
        let e0_0 = eval(&c, s0);
        let e1_0 = eval(&c, -s0);
        let mut descent = Vec::new();
        let mut s_r = s0;
        for r in 1..betas.len() {
            c = fold_poly(&c, betas[r - 1]);
            s_r = s_r * s_r;
            let e1_r = eval(&c, -s_r);
            descent.push((e1_r, betas[r]));
        }
        (e0_0, e1_0, betas[0], descent)
    }

    fn descent_window(cfg: &RecursiveConfig, h0: usize, index_draw: Val) -> Vec<Val> {
        let rounds = cfg.rounds();
        let coeffs: Vec<Challenge> = (0..(1u32 << rounds)).map(|k| ext_multi(k + 1)).collect();
        let betas: Vec<Challenge> = (0..rounds as u32).map(|r| ext_multi(100 + r)).collect();
        let index = (index_draw.as_canonical_u32() as usize) & ((1 << h0) - 1);
        let (e0_0, e1_0, beta0, descent) = honest_descent(&coeffs, &betas, h0, index);
        let inp = WindowInputs {
            draw_out: index_draw,
            e0: e0_0, e1: e1_0, beta: beta0,
            descent: &descent,
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: Challenge::ZERO, deep_q: Challenge::ZERO,
        };
        pack_window(cfg, &inp)
    }

    /// GOLDEN: an honest multi-round descent threaded in ONE window — s_0 tower-bound,
    /// s_{r+1}==s_r², e0_{r+1}==folded_r, per-round (★) — ACCEPTS.
    #[test]
    fn air_accepts_multiround_descent() {
        let h0 = 12usize;
        for rounds in [2usize, 3, 4] {
            let cfg = cfg_descent(h0, rounds);
            let row = descent_window(&cfg, h0, Val::from_u32(0xABCDE));
            let m = RowMajorMatrix::new(row, n_cols(&cfg));
            let report = check_all_constraints(&air(cfg), &m, &[], None);
            assert!(report.is_ok(), "honest {rounds}-round descent rejected: {} failures", report.failures.len());
        }
    }

    /// (c) FORGERY — BROKEN DESCENT LINK: round r+1 opens a primary eval e0 inconsistent with
    /// round r's fold output ⇒ the (chain-link) equality rejects. THIS is binding (c): a prover
    /// cannot substitute an unrelated value between descent rounds.
    #[test]
    fn air_rejects_broken_descent() {
        let h0 = 12usize;
        let rounds = 4usize;
        let cfg = cfg_descent(h0, rounds);
        let mut row = descent_window(&cfg, h0, Val::from_u32(0xABCDE));
        // break the link into round 1: corrupt round-1's e0 (no longer == round-0 folded).
        let b1 = extra_round_base(&cfg, 1);
        row[b1 + 2] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a broken descent chain-link was accepted — (c) not load-bearing"
        );
    }

    /// (c) FORGERY — CHAIN-LINK ISOLATED: substitute round 1's primary eval e0 with a DIFFERENT
    /// value AND re-fold round 1's `folded` to be (★)-consistent at that e0. Now round-1 (★) holds,
    /// the point-square holds, and ONLY the chain-link `e0_1 == folded_0` is violated ⇒ rejected.
    /// Proves the chain-link itself is load-bearing (not piggybacking on (★)).
    #[test]
    fn air_rejects_descent_chain_link_isolated() {
        let h0 = 12usize;
        let rounds = 3usize;
        let cfg = cfg_descent(h0, rounds);
        let mut row = descent_window(&cfg, h0, Val::from_u32(0xABCDE));
        let b1 = extra_round_base(&cfg, 1);
        // read round-1 e1, beta, s; pick a forged e0 (≠ folded_0) and re-fold so (★) holds.
        let read = |base: usize| -> Challenge {
            Challenge::from_basis_coefficients_slice(&row[base..base + D]).unwrap()
        };
        let e1_1 = read(b1 + D);
        let beta_1 = read(b1 + 2 * D);
        let s_1 = row[b1 + 3 * D];
        let forged_e0 = read(b1) + Challenge::ONE; // a different primary eval
        let refolded = fold_pair(forged_e0, e1_1, beta_1, s_1);
        row[b1..b1 + D].copy_from_slice(&limbs(forged_e0));
        row[b1 + 3 * D + 1..b1 + 4 * D + 1].copy_from_slice(&limbs(refolded));
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "an (★)-consistent round whose e0 != previous folded was accepted — chain-link not load-bearing"
        );
    }

    /// (c) FORGERY — POINT-SQUARE violated: a next-round point that is NOT the square of the
    /// previous round's point ⇒ rejected by the (point-square) constraint (and (★) at that round).
    #[test]
    fn air_rejects_descent_wrong_next_point() {
        let h0 = 12usize;
        let rounds = 3usize;
        let cfg = cfg_descent(h0, rounds);
        let mut row = descent_window(&cfg, h0, Val::from_u32(0x9999));
        let b2 = extra_round_base(&cfg, 2);
        row[b2 + 3 * D] += Val::ONE; // round-2 point no longer == round-1 point squared
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    /// (c) — the descent's round-0 point is STILL the bus tower point: corrupt s_0 and the (2)
    /// tower binding + descent round-0 reads of s_0 reject. Ties the descent's leaf layer to the bus.
    #[test]
    fn air_descent_round0_point_is_bus_bound() {
        let h0 = 12usize;
        let rounds = 3usize;
        let cfg = cfg_descent(h0, rounds);
        let mut row = descent_window(&cfg, h0, Val::from_u32(0xABCDE));
        row[C_S] += Val::ONE; // s_0 != tower product ⇒ (2) rejects, and descent uses s_0
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    // ---------------------------------------------------------------------------------
    // (b′) DEEP OOD block, with zeta bound to draws.
    // ---------------------------------------------------------------------------------

    fn cfg_deep(h: usize, k: usize, bind: bool) -> RecursiveConfig {
        RecursiveConfig {
            deep_log_degree: Some(k),
            bind_beta_to_draws: bind,
            ..RecursiveConfig::single_window(h)
        }
    }

    fn deep_window(cfg: &RecursiveConfig, draw: Val, zeta: Challenge, q: Challenge) -> Vec<Val> {
        let inp = WindowInputs {
            draw_out: draw,
            e0: ext_multi(1), e1: ext_multi(2), beta: ext_multi(3),
            descent: &[],
            merkle_leaf: [Val::ZERO; DIGEST_ELEMS],
            merkle_siblings: &[],
            deep_zeta: zeta, deep_q: q,
        };
        pack_window(cfg, &inp)
    }

    /// GOLDEN: an honest DEEP OOD block (C == Z_H(zeta)·Q) with zeta bound to draws ACCEPTS.
    #[test]
    fn air_accepts_deep_ood_with_zeta_bound() {
        let h = 10;
        for k in [4usize, 8, 12] {
            let cfg = cfg_deep(h, k, true);
            let row = deep_window(&cfg, Val::from_u32(0xABCDE), ext_multi(55), ext_multi(66));
            let m = RowMajorMatrix::new(row, n_cols(&cfg));
            assert!(check_all_constraints(&air(cfg), &m, &[], None).is_ok(), "honest DEEP OOD (k={k}) rejected");
        }
    }

    /// (b′) FORGERY — wrong C(zeta): a constraint eval ≠ Z_H(zeta)·Q is rejected.
    #[test]
    fn air_rejects_deep_wrong_c() {
        let h = 10;
        let k = 8;
        let cfg = cfg_deep(h, k, true);
        let mut row = deep_window(&cfg, Val::from_u32(0x1234), ext_multi(7), ext_multi(9));
        let db = deep_base(&cfg);
        row[db + DEEP_C + 1] += Val::ONE;
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    /// (b′)+(b) FORGERY — forged zeta: swap zeta to a different OOD point (self-consistent DEEP
    /// block) but keep the draw columns at the transcript value. The zeta↔draw equality rejects (a
    /// prover cannot open at a favorable OOD point).
    #[test]
    fn air_rejects_forged_zeta() {
        let h = 10;
        let k = 8;
        let cfg = cfg_deep(h, k, true);
        let draw = Val::from_u32(0x4242);
        let honest_zeta = ext_multi(55);
        let forged_zeta = ext_multi(77);
        assert_ne!(honest_zeta, forged_zeta);
        let q = ext_multi(66);
        let mut row = deep_window(&cfg, draw, honest_zeta, q);
        let forged = deep_window(&cfg, draw, forged_zeta, q);
        let db = deep_base(&cfg);
        for c in db + DEEP_ZETA..db + deep_size(k) {
            row[c] = forged[c];
        }
        // CONTROL: the DEEP block is self-consistent at forged_zeta (its C = Z_H(forged_zeta)·Q).
        // With the zeta draw column ALSO set to forged_zeta the row ACCEPTS, isolating (b) as the
        // sole rejector below. The zeta draw block is the LAST draw block (after the round betas).
        let mut control = row.clone();
        let zeta_draw_base = draw_block_base(&cfg) + cfg.rounds() * D;
        control[zeta_draw_base..zeta_draw_base + D].copy_from_slice(&limbs(forged_zeta));
        assert!(
            check_all_constraints(&air(cfg), &RowMajorMatrix::new(control, n_cols(&cfg)), &[], None).is_ok(),
            "forged-zeta control (zeta draw == zeta) should accept — test would prove nothing otherwise"
        );
        // draw columns still carry honest_zeta ⇒ (b) zeta==draw fails.
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(
            !check_all_constraints(&air(cfg), &m, &[], None).is_ok(),
            "a zeta != the challenger draw was accepted — (b) zeta binding not load-bearing"
        );
    }

    /// (b′) FORGERY — a tampered intermediate DEEP tower power ⇒ the squaring chain breaks ⇒
    /// rejected (the Z_H tower is fully constrained, not just its endpoints).
    #[test]
    fn air_rejects_deep_tampered_tower() {
        let h = 10;
        let k = 12;
        let cfg = cfg_deep(h, k, false);
        let mut row = deep_window(&cfg, Val::from_u32(0x55), ext_multi(31), ext_multi(32));
        let db = deep_base(&cfg);
        row[db + DEEP_POW + 6 * D] += Val::ONE; // corrupt a middle tower power
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        assert!(!check_all_constraints(&air(cfg), &m, &[], None).is_ok());
    }

    // ---------------------------------------------------------------------------------
    // FULL MONOLITH: all blocks together in one window.
    // ---------------------------------------------------------------------------------

    /// GOLDEN: a window with ALL blocks on — Merkle path (a) + beta/zeta draws (b) + multi-round
    /// descent (c) + DEEP OOD (b′) — assembled in ONE window, ACCEPTS. The bus index drives the
    /// fold point, the Merkle index, and the descent's s_0; the betas+zeta are bound to draws.
    #[test]
    fn air_accepts_full_monolith_window() {
        let levels = 4usize; // merkle levels = log_height
        let rounds = 3usize;
        let k = 8usize; // deep log_degree
        let cfg = RecursiveConfig {
            log_height: levels,
            merkle_levels: levels,
            bind_beta_to_draws: true,
            num_rounds: rounds,
            deep_log_degree: Some(k),
        };
        let n = 1usize << levels;
        let leaves: Vec<Digest> = (0..n as u32).map(digest).collect();
        let tree = RefTree::new(leaves.clone());
        let root = tree.root();
        let draw = Val::from_u32(0b1011);
        let index = (draw.as_canonical_u32() as usize) & (n - 1);

        // descent inputs at log_height = levels.
        let coeffs: Vec<Challenge> = (0..(1u32 << rounds)).map(|c| ext_multi(c + 1)).collect();
        let betas: Vec<Challenge> = (0..rounds as u32).map(|r| ext_multi(100 + r)).collect();
        let (e0_0, e1_0, beta0, descent) = honest_descent(&coeffs, &betas, levels, index);

        let sibs = tree.auth_path(index);
        let inp = WindowInputs {
            draw_out: draw,
            e0: e0_0, e1: e1_0, beta: beta0,
            descent: &descent,
            merkle_leaf: leaves[index],
            merkle_siblings: &sibs,
            deep_zeta: ext_multi(55), deep_q: ext_multi(66),
        };
        let row = pack_window(&cfg, &inp);
        let m = RowMajorMatrix::new(row, n_cols(&cfg));
        let report = check_all_constraints(&air(cfg), &m, &pubs(leaves[index], root), None);
        assert!(report.is_ok(), "full monolith window rejected: {} failures", report.failures.len());
    }
}
