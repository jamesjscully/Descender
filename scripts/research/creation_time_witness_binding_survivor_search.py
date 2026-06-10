#!/usr/bin/env python3
"""Creation-time account-witness binding survivor search.

The spend-only negative control
`projection_carrier_tuple_does_not_prove_full_witness_opening`
(tools/production-reference/src/lib.rs:2205) rebinds the SAME six-value
projection-opening frame plus selected carrier lanes to TWO DIFFERENT packed
Fq12 account-witness commitments and BOTH accept.  That is the spend-only
survivor: the carrier tuple does not pin the account-witness commitment, because
nothing in the spend tuple forces a fixed commitment.

This probe asks whether the CREATION-TIME certificate kills that freedom.  The
creation certificate adds the masked_state18 packed-witness opening which must
REPACK to a LEAF-FIXED `account_witness_commitment`.  We reproduce the exact
bit-packing / mask algebra from
tools/parameter-search/src/production_reference.rs and test, for a fixed leaf
(fixed opening_commitment -> fixed account_witness_commitment, fixed pad_seed,
fixed slot_id):

  Q1. Can the spend-only survivor (two distinct commitments w_a != w_b) still
      satisfy the creation certificate for the SAME leaf?
  Q2. Is the map  witness_commitment  <->  (12 syndrome residues,
      6 account_projection residues)  a bijection?  If so, a leaf-fixed
      commitment fixes all 18 residues, so two distinct witnesses cannot share
      the same leaf commitment.
  Q3. Is the masked_state18 repack-to-commitment an independent constraint, or
      an algebraic tautology (identity)?  (If it is identity, the binding power
      comes solely from the leaf fixing the commitment, not from the repack.)

Algebra mirrored (production_reference.rs):
  FIELD_BITS = 14, DIGEST_BYTES = 32, 18 residues * 14 = 252 packed bits,
  4 slack bits MUST be zero (any_bits_set_le check). write_bits_le packs LE.
  pack_masked_witness_state18_from_parts:  masked[i] = (account_proj[i]+mask[i]) % Q
  masked_state18_packed_witness_opening:   account_proj[i] = (masked[i]-mask[i]) % Q
    then repacks and asserts repack == commitment.
"""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
OUT_JSON = ROOT / "research/cryptanalysis/creation_time_witness_binding_survivor_search_n32_q12289.json"

Q = 12289
FIELD_BITS = 14
DIGEST_BYTES = 32
STATE18_ROWS = 18
SYNDROME_ROWS = 12        # REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS
PROJECTION_LANES = 6      # HIDDEN_SOURCE_FALLBACK_ACCOUNT_PROJECTION_LANES
PROJECTION_OFFSET = SYNDROME_ROWS
PACKED_BITS = STATE18_ROWS * FIELD_BITS          # 252
SLACK_BITS = DIGEST_BYTES * 8 - PACKED_BITS      # 4
MASK_DOMAIN = "Descender.Signature.q12289.local_delta.reviewed_fq12_packed_witness_state18_projection_mask.v1"


def write_bits_le(buf: bytearray, bit_offset: int, bit_len: int, value: int) -> None:
    for i in range(bit_len):
        bit = (value >> i) & 1
        byte_idx = (bit_offset + i) // 8
        mask = 1 << ((bit_offset + i) % 8)
        if bit == 0:
            buf[byte_idx] &= ~mask & 0xFF
        else:
            buf[byte_idx] |= mask


def read_bits_le(buf: bytes, bit_offset: int, bit_len: int) -> int:
    out = 0
    for i in range(bit_len):
        bit = bit_offset + i
        out |= ((buf[bit // 8] >> (bit % 8)) & 1) << i
    return out


def any_bits_set_le(buf: bytes, bit_offset: int, bit_len: int) -> bool:
    return any(((buf[(bit_offset + i) // 8] >> ((bit_offset + i) % 8)) & 1) for i in range(bit_len))


def pack_state18(residues: list[int]) -> bytes:
    assert len(residues) == STATE18_ROWS
    out = bytearray(DIGEST_BYTES)
    for idx, v in enumerate(residues):
        if not (0 <= v < Q):
            raise ValueError(f"non-canonical residue {idx}: {v}")
        write_bits_le(out, idx * FIELD_BITS, FIELD_BITS, v)
    return bytes(out)


def unpack_state18(commitment: bytes) -> list[int]:
    if any_bits_set_le(commitment, PACKED_BITS, SLACK_BITS):
        raise ValueError("non-canonical: slack high bits set")
    out = []
    for idx in range(STATE18_ROWS):
        v = read_bits_le(commitment, idx * FIELD_BITS, FIELD_BITS)
        if v >= Q:
            raise ValueError(f"non-canonical residue {idx}: {v}")
        out.append(v)
    return out


# --- mirror uniform_mod_from_hash for projection masks -----------------------
# We don't need the exact PRF bytes to answer the binding question; the mask is
# a DETERMINISTIC function of (pad_seed, slot_id, lane).  We model it as a fixed
# deterministic pseudo-random value so the repack algebra is exercised exactly.
# The bijection / tautology conclusions are mask-value-independent because masks
# cancel; we use a concrete deterministic mask to make the demonstration runnable.

def projection_masks(pad_seed: bytes, slot_id: int) -> list[int]:
    seed = pad_seed + slot_id.to_bytes(8, "little")
    masks = []
    for lane in range(PROJECTION_LANES):
        h = hashlib.sha256()
        h.update(MASK_DOMAIN.encode())
        h.update(seed)
        h.update(slot_id.to_bytes(8, "little"))
        h.update(lane.to_bytes(8, "little"))
        masks.append(int.from_bytes(h.digest()[:4], "little") % Q)
    return masks


def pack_masked_state18_from_parts(syndrome: list[int], account_projection: list[int],
                                   pad_seed: bytes, slot_id: int) -> bytes:
    masks = projection_masks(pad_seed, slot_id)
    masked_projection = [(account_projection[i] + masks[i]) % Q for i in range(PROJECTION_LANES)]
    residues = list(syndrome) + masked_projection
    return pack_state18(residues)


def masked_state18_opening(commitment: bytes, pad_seed: bytes, slot_id: int) -> dict[str, Any]:
    """Mirror reviewed_fq12_masked_state18_packed_witness_opening (incl. repack check)."""
    residues = unpack_state18(commitment)
    syndrome = residues[:SYNDROME_ROWS]
    masked_projection = residues[PROJECTION_OFFSET:PROJECTION_OFFSET + PROJECTION_LANES]
    masks = projection_masks(pad_seed, slot_id)
    account_projection = [(masked_projection[i] - masks[i]) % Q for i in range(PROJECTION_LANES)]
    repacked = pack_masked_state18_from_parts(syndrome, account_projection, pad_seed, slot_id)
    repack_ok = repacked == commitment
    return {
        "witness_syndrome": syndrome,
        "masked_projection": masked_projection,
        "projection_masks": masks,
        "account_projection": account_projection,
        "repacked_commitment": repacked.hex(),
        "repack_matches_commitment": repack_ok,
    }


def main() -> None:
    pad_seed = bytes([0x5A] * 32)
    slot_id = 0

    results: dict[str, Any] = {}

    # ----- Q3: is the repack a tautology? -----------------------------------
    # For an ARBITRARY canonical commitment, run the opening and check the
    # repack always matches.  If it always matches, the repack adds NO
    # constraint; the binding comes solely from the leaf fixing the commitment.
    import random
    rng = random.Random(0xC0FFEE)
    repack_always_ok = True
    repack_trials = 2000
    for _ in range(repack_trials):
        residues = [rng.randrange(Q) for _ in range(STATE18_ROWS)]
        commitment = pack_state18(residues)
        opening = masked_state18_opening(commitment, pad_seed, slot_id)
        if not opening["repack_matches_commitment"]:
            repack_always_ok = False
            break
    results["repack_is_tautology"] = {
        "trials": repack_trials,
        "repack_always_matched_commitment": repack_always_ok,
        "conclusion": (
            "repack is an algebraic identity (masked = (masked-mask)+mask); it adds "
            "no independent constraint. The binding power is entirely that the leaf "
            "FIXES account_witness_commitment, which fixes all 18 residues."
        ),
    }

    # ----- Q2: is commitment <-> (12 syndrome, 6 account_projection) a bijection? -----
    # Forward: parts -> commitment is injective iff distinct parts give distinct
    # commitments (it is, since packing is a bijection on canonical 18-tuples and
    # masks are a fixed permutation per lane: account_proj |-> (account_proj+mask) is
    # a bijection mod Q).  Inverse: commitment -> parts is well-defined (unpack).
    # Empirically verify injectivity on random parts.
    seen: dict[bytes, tuple] = {}
    collision = None
    bij_trials = 20000
    for _ in range(bij_trials):
        syndrome = [rng.randrange(Q) for _ in range(SYNDROME_ROWS)]
        account_projection = [rng.randrange(Q) for _ in range(PROJECTION_LANES)]
        commitment = pack_masked_state18_from_parts(syndrome, account_projection, pad_seed, slot_id)
        # round-trip back through opening
        opening = masked_state18_opening(commitment, pad_seed, slot_id)
        roundtrip_ok = (opening["witness_syndrome"] == syndrome and
                        opening["account_projection"] == account_projection)
        key = (tuple(syndrome), tuple(account_projection))
        if commitment in seen and seen[commitment] != key:
            collision = (seen[commitment], key, commitment.hex())
            break
        seen[commitment] = key
        if not roundtrip_ok:
            collision = ("roundtrip_failure", key, commitment.hex())
            break
    results["bijection"] = {
        "trials": bij_trials,
        "distinct_parts_gave_distinct_commitments": collision is None,
        "round_trip_recovered_parts": collision is None,
        "collision": collision,
        "conclusion": (
            "witness_commitment <-> (12 syndrome residues, 6 account_projection "
            "residues) is a bijection on canonical inputs for fixed (pad_seed, slot_id). "
            "Therefore a LEAF-FIXED commitment fixes ALL 18 residues, hence the "
            "account_projection (the 6-lane witness opening) is UNIQUELY determined."
        ),
    }

    # ----- Q1: replay the spend-only survivor under the creation constraint -----
    # Spend-only survivor (lib.rs:2205): witness_a = pack(syndrome all-zero),
    # witness_b = pack(syndrome[0]=1).  Both accepted because the spend tuple let
    # account_witness_commitment move FREELY (it is a free input to the carrier
    # binding, never pinned to a fixed leaf commitment).
    #
    # Creation constraint: the leaf fixes ONE opening_commitment, which is
    # SHA256(... || account_witness_commitment || ...).  So the leaf pins exactly
    # ONE account_witness_commitment.  Two distinct commitments w_a != w_b cannot
    # both satisfy the SAME leaf unless SHA256 collides on the opening frame.
    syndrome_a = [0] * SYNDROME_ROWS
    syndrome_b = [0] * SYNDROME_ROWS
    syndrome_b[0] = 1
    proj = [3, 5, 8, 13, 21, 34]
    w_a = pack_masked_state18_from_parts(syndrome_a, proj, pad_seed, slot_id)
    w_b = pack_masked_state18_from_parts(syndrome_b, proj, pad_seed, slot_id)

    def opening_commitment_digest(witness_commitment: bytes) -> str:
        # Mirror reviewed_fq12_state_assisted_opening_commitment_frame: the
        # account_witness_commitment is a hashed field.  Other opening fields
        # held identical between the two survivors EXCEPT the commitment.
        h = hashlib.sha256()
        h.update(b"Descender.creation_probe.opening_commitment.frame")
        h.update(slot_id.to_bytes(8, "little"))
        h.update(witness_commitment)
        h.update(b"randomizer_commitment_fixed")
        h.update(b"restore_anchor_fixed")
        h.update(b"witness_opening_digest_fixed")
        h.update(b"pad_seed_digest_fixed")
        h.update(b"carrier_handle_secret_fixed")
        return h.hexdigest()

    oc_a = opening_commitment_digest(w_a)
    oc_b = opening_commitment_digest(w_b)
    results["spend_survivor_under_creation"] = {
        "witness_a": w_a.hex(),
        "witness_b": w_b.hex(),
        "witnesses_distinct": w_a != w_b,
        "opening_commitment_a": oc_a,
        "opening_commitment_b": oc_b,
        "opening_commitments_distinct": oc_a != oc_b,
        "conclusion": (
            "The two spend-only survivor witnesses map to TWO DISTINCT "
            "opening_commitments. The leaf fixes exactly ONE opening_commitment. "
            "So at most ONE of {w_a, w_b} can satisfy a given leaf. The spend-only "
            "two-witnesses survivor is KILLED at creation time: it required the "
            "account_witness_commitment to move freely, but the leaf pins it."
        ),
    }

    # ----- Residual question: witness-free / same-commitment survivor --------
    # Could two DIFFERENT account_projection openings share the SAME commitment
    # for the SAME leaf?  No: bijection (Q2) forbids it.  Could a witness-free
    # assignment satisfy the certificate?  The opening is DERIVED from the fixed
    # commitment (masked_state18_opening reads it out), so there is exactly one
    # account_projection per leaf.  The only remaining freedom would be a SHA256
    # collision on the opening frame (oc_a == oc_b with w_a != w_b) or on the
    # witness_opening_digest frame -- a hash-collision break, not an algebraic
    # survivor.
    results["residual"] = {
        "same_commitment_two_openings": "impossible (bijection)",
        "witness_free_assignment": (
            "impossible: account_projection is read out of the fixed commitment; "
            "exactly one opening per leaf"
        ),
        "only_remaining_attack": (
            "SHA256 collision on the opening_commitment frame or the "
            "witness_opening_digest frame -- this is the named "
            "`prevalidation_leaf_binding_break` / `qrom_or_domain_separation_break`, "
            "NOT an algebraic creation-time survivor"
        ),
    }

    survivor_found = (
        results["spend_survivor_under_creation"]["opening_commitments_distinct"] is False
        or results["bijection"]["distinct_parts_gave_distinct_commitments"] is False
    )

    payload = {
        "schema": "Descender.creation_time_witness_binding_survivor_search.v1",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "q": Q,
        "field_bits": FIELD_BITS,
        "state18_rows": STATE18_ROWS,
        "syndrome_rows": SYNDROME_ROWS,
        "projection_lanes": PROJECTION_LANES,
        "slack_bits": SLACK_BITS,
        "creation_survivor_found": survivor_found,
        "results": results,
        "verdict": (
            "NO creation-time algebraic survivor: the masked_state18 repack-to-"
            "LEAF-FIXED-commitment makes the account-witness opening map injective. "
            "The spend-only two-witnesses survivor is killed because the leaf pins a "
            "single account_witness_commitment and the 18-residue packing is a "
            "bijection. The binding reduces to SHA256 collision resistance "
            "(a named break), not an open algebraic gap."
        ) if not survivor_found else "CREATION-TIME SURVIVOR FOUND -- see results.",
    }
    OUT_JSON.parent.mkdir(parents=True, exist_ok=True)
    OUT_JSON.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(json.dumps(payload, indent=2, sort_keys=True))
    print(f"\nwrote {OUT_JSON}")


if __name__ == "__main__":
    main()
