#!/usr/bin/env python3
"""
WS4 #11 empirical leakage battery over REAL Descender signatures.

Input: research/cryptanalysis/leakage_corpus.jsonl
  - line 0: META record with ground-truth secret witness per slot
      (full_response[23], pad_table[16][2]) and public_free[16][2].
  - remaining lines: one real signature each, decoded transcript components
      slot, block[26], sign[26], resp[52], open[52].

Tests:
  (A) chi-square uniformity on each transcript component
  (B) autocorrelation of the response / opening streams
  (C) 3x2 repeated local-block-structure test
  (D) WITNESS-CORRELATION / key-recovery: do the public transcript openings
      reveal the SECRET pad table (the "active pad outputs" the HIGH caveat
      says must be excluded from the hiding sampler)? Exact reconstruction +
      correlation against ground truth.
  (E) response determinism check: are signed responses a deterministic public
      function of (challenge, public_free)? (=> leak nothing beyond pubkey)

Multiple-testing: Bonferroni + Benjamini-Hochberg over all p-values.
"""
import json
import sys
import numpy as np
from scipy import stats

Q = 12289
REPS = 26
GROUPS = 16
LOCAL_FREE = 2

CORPUS = sys.argv[1] if len(sys.argv) > 1 else "research/cryptanalysis/leakage_corpus.jsonl"


def load(path):
    meta = None
    recs = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            o = json.loads(line)
            if o.get("meta"):
                meta = o
            else:
                recs.append(o)
    return meta, recs


def main():
    meta, recs = load(CORPUS)
    n = len(recs)
    print(f"# Leakage battery over {n} REAL signatures, {len(meta['slots'])} slots\n")

    slot = np.array([r["slot"] for r in recs])
    block = np.array([r["block"] for r in recs], dtype=np.int64)          # n x 26
    sign = np.array([r["sign"] for r in recs], dtype=np.int64)            # n x 26
    resp = np.array([r["resp"] for r in recs], dtype=np.int64).reshape(n, REPS, LOCAL_FREE)
    opn = np.array([r["open"] for r in recs], dtype=np.int64).reshape(n, REPS, LOCAL_FREE)

    public_free = np.array(meta["public_free"], dtype=np.int64)          # 16 x 2
    pad_tables = {s["slot"]: np.array(s["pad_table"], dtype=np.int64) for s in meta["slots"]}
    full_resp = {s["slot"]: np.array(s["full_response"], dtype=np.int64) for s in meta["slots"]}

    pvals = []           # collect (label, p) for multiple-testing correction
    findings = []

    # ---------------------------------------------------------------
    # (A) chi-square uniformity of the challenge block selection (5-bit -> block 0..15)
    # The challenge is the only per-message randomness; if block selection is biased
    # the FS hash is biased. Test uniformity of blocks over all reps.
    flat_block = block.reshape(-1)
    counts = np.bincount(flat_block, minlength=GROUPS)
    chi, p = stats.chisquare(counts)
    pvals.append(("A.block_uniform", p))
    print(f"(A) challenge block selection chi-square: chi2={chi:.1f}, dof={GROUPS-1}, p={p:.4g} "
          f"(N={flat_block.size})")
    # sign uniformity
    sc = np.bincount((sign.reshape(-1) > 0).astype(int), minlength=2)
    chi_s, p_s = stats.chisquare(sc)
    pvals.append(("A.sign_uniform", p_s))
    print(f"    challenge sign bit chi-square: chi2={chi_s:.2f}, p={p_s:.4g}")

    # ---------------------------------------------------------------
    # (E) response determinism: is resp[rep,coord] == center(mask[coord] + sign*public_free[block][coord])?
    mask = np.array([12, Q - 12], dtype=np.int64)

    def center(v):
        v = v % Q
        return np.where(v > Q // 2, v - Q, v)

    pred = np.zeros((n, REPS, LOCAL_FREE), dtype=np.int64)
    for coord in range(LOCAL_FREE):
        pf = public_free[block, coord]                       # n x 26
        pred[:, :, coord] = center(mask[coord] + sign * pf)
    resp_det = np.array_equal(pred, resp)
    print(f"\n(E) responses are an EXACT deterministic public function of "
          f"(challenge, public_free): {resp_det}")
    if resp_det:
        findings.append("RESPONSES carry zero secret-dependent randomness: each signed "
                        "residue equals center(mask + sign*public_free[block]) exactly, "
                        "computable by anyone holding the public key. No witness info in resp.")

    # ---------------------------------------------------------------
    # (D) WITNESS-CORRELATION / key-recovery on the OPENINGS.
    # opening[rep,coord] should be center-free value add_mod(mask[coord], pad[block][coord]).
    # i.e. open == (mask[coord] + pad_table[slot][block][coord]) mod Q.
    # Test: can we EXACTLY reconstruct the SECRET pad_table from public openings?
    print("\n(D) witness-correlation / key-recovery on OPENINGS (active pad outputs):")
    total_recovered = 0
    total_cells = 0
    exact_match_all = True
    for s in sorted(pad_tables):
        sel = slot == s
        recovered = np.full((GROUPS, LOCAL_FREE), -1, dtype=np.int64)
        for coord in range(LOCAL_FREE):
            ob = block[sel]                                  # m x 26
            ov = opn[sel, :, coord]                          # m x 26
            for b in range(GROUPS):
                hits = ov[ob == b]
                if hits.size:
                    # recovered pad = (open - mask) mod Q ; deterministic so any obs works
                    recovered[b, coord] = (hits[0] - mask[coord]) % Q
        observed = recovered >= 0
        truth = pad_tables[s]
        match = (recovered == truth) | (~observed)
        n_obs = int(observed.sum())
        n_match = int((recovered[observed] == truth[observed]).sum())
        total_recovered += n_match
        total_cells += n_obs
        if not match.all():
            exact_match_all = False
        print(f"    slot {s}: recovered {n_match}/{n_obs} secret pad cells observed "
              f"(of {GROUPS*LOCAL_FREE} total); exact={n_match==n_obs}")
    print(f"    => secret pad cells reconstructed exactly: {total_recovered}/{total_cells}")
    if total_recovered == total_cells and total_cells > 0:
        findings.append(
            f"KEY-RECOVERY LEAK: the public signature openings reveal the SECRET "
            f"pad table EXACTLY. open[rep,coord] = (12 or Q-12) + pad(seed,block) mod Q "
            f"is a deterministic function of the secret seed and the challenge-selected "
            f"block. Over the corpus all {total_cells} observed pad cells were recovered "
            f"bit-for-bit. These are exactly the 'active pad outputs' the HIGH caveat "
            f"(stateful.rs:6459) says must be EXCLUDED from the zero-TV hiding sampler.")

    # Statistical witness-correlation framing: regress opening on the known pad.
    # Build paired (open_minus_mask, true_pad) across all reps/coords/slots and
    # report Pearson r + p. A deterministic relation => r ~ 1, p ~ 0.
    xs, ys = [], []
    for s in sorted(pad_tables):
        sel = slot == s
        for coord in range(LOCAL_FREE):
            ob = block[sel].reshape(-1)
            ov = (opn[sel, :, coord].reshape(-1) - mask[coord]) % Q
            xs.append(pad_tables[s][ob, coord])
            ys.append(ov)
    xs = np.concatenate(xs).astype(float)
    ys = np.concatenate(ys).astype(float)
    r, pr = stats.pearsonr(xs, ys)
    pvals.append(("D.opening_pad_corr", pr if pr > 0 else 1e-300))
    print(f"    Pearson(open-mask, true_pad): r={r:.6f}, p={pr:.3g}, pairs={xs.size}")

    # ---------------------------------------------------------------
    # (B) autocorrelation of opening residue stream (lag 1..5), per coord.
    print("\n(B) autocorrelation of opening residue streams (lag 1..5):")
    for coord in range(LOCAL_FREE):
        stream = opn[:, :, coord].reshape(-1).astype(float)
        stream = stream - stream.mean()
        denom = np.dot(stream, stream)
        for lag in range(1, 6):
            ac = np.dot(stream[:-lag], stream[lag:]) / denom
            # approximate significance: r*sqrt(N) ~ N(0,1) under white-noise null
            N = stream.size - lag
            z = ac * np.sqrt(N)
            p = 2 * stats.norm.sf(abs(z))
            pvals.append((f"B.autocorr_c{coord}_lag{lag}", p))
            if lag <= 2:
                print(f"    coord {coord} lag {lag}: r={ac:+.4f}, z={z:+.2f}, p={p:.3g}")

    # ---------------------------------------------------------------
    # (C) 3x2 repeated local-block-structure test.
    # Within a single signature, the same block can be selected in multiple reps.
    # If so, the opening for that block MUST repeat identically (deterministic pad).
    # Test: across signatures, when block b appears in >=2 reps, are the openings
    # identical? (a structural fingerprint that links reps -> same block -> linkable).
    print("\n(C) 3x2 repeated local-block-structure test:")
    repeat_consistent = 0
    repeat_total = 0
    for i in range(min(n, 5000)):
        b = block[i]
        for bb in np.unique(b):
            idxs = np.where(b == bb)[0]
            if idxs.size >= 2:
                vals = opn[i, idxs, :]
                repeat_total += 1
                if np.all(vals == vals[0]):
                    repeat_consistent += 1
    print(f"    intra-signature repeated-block openings identical: "
          f"{repeat_consistent}/{repeat_total} "
          f"({'STRUCTURAL FINGERPRINT' if repeat_total and repeat_consistent==repeat_total else 'mixed'})")
    if repeat_total and repeat_consistent == repeat_total:
        findings.append(
            "STRUCTURAL FINGERPRINT: within a signature, whenever the challenge selects "
            "the same block in two reps, the openings are byte-identical (deterministic "
            "pad). The 3x2 repeated-block structure is fully present and observable.")

    # ---------------------------------------------------------------
    # Multiple-testing correction over collected p-values
    print("\n## Multiple-testing correction")
    labels = [l for l, _ in pvals]
    ps = np.array([min(max(p, 1e-300), 1.0) for _, p in pvals])
    m = ps.size
    bonf = ps * m
    order = np.argsort(ps)
    bh = np.empty_like(ps)
    prev = 1.0
    for rank in range(m - 1, -1, -1):
        i = order[rank]
        prev = min(prev, ps[i] * m / (rank + 1))
        bh[i] = prev
    print(f"   {m} hypotheses tested. Bonferroni alpha=0.05/{m}={0.05/m:.2g}")
    sig = [(labels[i], ps[i], bonf[i], bh[i]) for i in range(m)
           if bh[i] < 0.05]
    sig.sort(key=lambda x: x[1])
    for lab, p, b, q in sig[:20]:
        print(f"   SIG  {lab}: p={p:.3g} BH-q={q:.3g}")

    # ---------------------------------------------------------------
    # Compare to theoretical claim residual_hiding_meets_140 (>=140 hiding bits)
    print("\n## FINDINGS vs theoretical hiding claim (residual_hiding_meets_140 >= 140 bits)")
    for fnd in findings:
        print(" * " + fnd)
    if total_cells and total_recovered == total_cells:
        print("\n VERDICT: LEAK. The theoretical zero-TV / >=140-bit hiding certificate is")
        print(" certified ONLY for the conditioned quotient-fiber sampler that EXCLUDES")
        print(" active pad outputs. The DEPLOYED reference_sign_with_proof EMITS those pad")
        print(" outputs in the clear via the openings, so the empirical transcript hiding")
        print(" is 0 bits for the pad witness, not >=140. The required_closure at")
        print(" stateful.rs:6464 (enforce the rejection predicate / check active-pad")
        print(" equations OUTSIDE the hidden sampler) is NOT met by the deployed signer.")


if __name__ == "__main__":
    main()
