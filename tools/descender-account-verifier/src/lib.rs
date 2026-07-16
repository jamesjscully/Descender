//! # Path 2 account-creation verifier transport (Option A: separate crate)
//!
//! This crate is the SINGLE place where the Plonky3 STARK verifier enters the
//! account-admission verification path. Signer-side `descender-parameter-search`
//! builds depend on THIS crate only behind an off-by-default feature, so a
//! wallet/signer build that does not verify creation proofs links ZERO `p3-*`
//! crates. A validator build enables the feature and pulls Plonky3 transitively
//! through here.
//!
//! ## Re-export discipline (single source of truth)
//!
//! The pinned FRI / soundness / config constants live in `descender-account-stark`
//! and are RE-EXPORTED here verbatim — this crate never re-declares them, so the
//! verifier can never drift from the prover's parameters.
//!
//! ## What this crate binds
//!
//! [`verify_deployed_account_creation`] runs the STARK relation verifier and then
//! binds the verified public statement to the deployed admission leaf's plain
//! anchors:
//!
//! | anchor                    | binding                                            |
//! |---------------------------|----------------------------------------------------|
//! | `account_id`              | direct equality (raw bytes, in-circuit-bound)      |
//! | `slot_id`                 | direct equality (in-circuit-bound)                 |
//! | `account_state_root`      | direct equality (transcript-bound)                 |
//! | `lifecycle_anchor`        | direct equality (in-circuit-bound)                 |
//! | `verifier_version`        | direct equality (raw u32, transcript-bound)        |
//! | `masked_projection[6]`    | direct equality (q-residues as u16)                |
//! | `native_role_root_commitment` | **export bridge** `sha256(DS ‖ role_root)`     |
//! | `packed_witness_commitment` | direct equality (byte-encoded syndrome/projection commitment) |
//! | `pad_seed_nullifier`      | direct equality (8 canonical little-endian field limbs) |
//!
//! ### The one named seam: the role-root export bridge
//!
//! The STARK exposes the role root as field elements (`role_root: [Val; 8]`),
//! while the deployed leaf carries it as a 32-byte SHA commitment
//! (`native_role_root_commitment`). [`role_root_export_digest`] is the canonical
//! bridge between them. For the binding to hold end-to-end, the admission-proof
//! PRODUCER must set the deployed commitment via this exact function (re-exported
//! so it cannot drift). This is the "verifier-exporter bridge" seam named in the
//! SECURITY_DOSSIER; this crate ENFORCES it on the verify side.

use p3_field::{PrimeCharacteristicRing, PrimeField32};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// ---------------------------------------------------------------------------
// Re-export discipline: the pinned config is the SINGLE source of truth.
// ---------------------------------------------------------------------------
pub use descender_account_stark::{
    prove_account_creation, prove_account_creation_with_seed, AccountCreationPublic,
    AccountCreationSecrets, Error as StarkError, Proof, Val, COMMITMENT_NONCE_BYTES,
    KEY_COMMITMENT_BYTES, NULLIFIER_ELEMS, N_COMMIT_LANES, N_LANES,
    PACKED_WITNESS_COMMITMENT_BYTES, RELATION_ID_ELEMS, ROLE_ROOT_ELEMS, SYNDROME_LANES,
};
pub use descender_account_stark::{
    PROD_LOG_BLOWUP, PROD_NUM_QUERIES, PROD_QUERY_POW_BITS, PROD_SOUNDNESS_BITS,
};

const Q12289: u32 = 12289;

/// Domain separator for the role-root export bridge (see [`role_root_export_digest`]).
pub const ROLE_ROOT_EXPORT_DOMAIN: &[u8] = b"DESCENDER_ROLE_ROOT_EXPORT_V1";

/// Canonical bridge from the STARK's field-element `role_root` to the deployed
/// leaf's 32-byte `native_role_root_commitment`. The PRODUCER of a deployed
/// admission leaf MUST set `native_role_root_commitment = role_root_export_digest(&public.role_root)`
/// so this verifier's bridge check passes. Re-exported (not re-declared) so the
/// two sides cannot drift.
pub fn role_root_export_digest(role_root: &[Val]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(ROLE_ROOT_EXPORT_DOMAIN);
    for v in role_root {
        h.update(v.as_canonical_u32().to_le_bytes());
    }
    h.finalize().into()
}

/// Stable registry-key encoding of the in-circuit CLASS A-2 nullifier.
///
/// The encoding intentionally excludes verifier/profile versions. Exact
/// `(pad_seed, slot)` reuse must remain occupied across software upgrades.
pub fn creation_nullifier_v1_bytes(public: &AccountCreationPublic) -> [u8; NULLIFIER_ELEMS * 4] {
    let mut out = [0u8; NULLIFIER_ELEMS * 4];
    for (index, value) in public.nullifier.iter().enumerate() {
        out[index * 4..index * 4 + 4].copy_from_slice(&value.as_canonical_u32().to_le_bytes());
    }
    out
}

/// Plain-data (NO Plonky3 types) anchors the deployed admission leaf can supply.
/// `descender-parameter-search` builds this from its leaf/evidence/view WITHOUT
/// touching any `p3-*` type, then hands it (plus the opaque bundle bytes) to
/// [`verify_deployed_account_creation`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeployedCreationAnchors {
    pub account_id: [u8; 32],
    pub slot_id: u64,
    pub account_state_root: [u8; 32],
    pub lifecycle_anchor: [u8; 32],
    pub verifier_version: u32,
    /// The 6 masked-projection q-residue lanes carried in the deployed public view.
    pub masked_projection: [u16; 6],
    /// The deployed leaf's 32-byte role-root commitment (must equal the export bridge).
    pub native_role_root_commitment: [u8; 32],
    /// The deployed binding's 32-byte `packed_witness_commitment` (14-bit-LE pack of
    /// `[syndrome[0..12] ‖ masked_projection[0..6]]`). Binding the PROVEN packed
    /// commitment to this ties the creation proof's syndrome + masked_projection to
    /// the EXACT packed witness the spend's checked-core frame consumes — closing
    /// MED-2 (the syndrome was otherwise an unanchored prover-chosen public input).
    pub packed_witness_commitment: [u8; PACKED_WITNESS_COMMITMENT_BYTES],
    /// Stable canonical encoding of the in-circuit CLASS A-2 nullifier. The
    /// registry key is these eight little-endian field limbs without a verifier
    /// version prefix, so a version change cannot make an occupied nullifier
    /// reusable.
    pub pad_seed_nullifier: [u8; NULLIFIER_ELEMS * 4],
}

/// Plain decoded public anchors from the opaque account-creation bundle.
///
/// This keeps Plonky3 field types inside this verifier crate while giving
/// consensus-boundary evidence code a stable public view to compare against
/// deployed leaf/binding anchors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedCreationPublicAnchors {
    pub account_id: [u8; 32],
    pub slot_id: u64,
    pub account_state_root: [u8; 32],
    pub lifecycle_anchor: [u8; 32],
    pub verifier_version: u32,
    pub masked_projection: [u16; 6],
    pub native_role_root_commitment: [u8; 32],
    pub packed_witness_commitment: [u8; PACKED_WITNESS_COMMITMENT_BYTES],
    pub pad_seed_nullifier: [u8; NULLIFIER_ELEMS * 4],
}

/// Public anchors whose account-STARK proof and deployed anchor equalities have
/// both been verified. Construction is intentionally private so consensus code
/// cannot register a caller-selected nullifier obtained from the decode-only API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedCreationPublicAnchors {
    decoded: DecodedCreationPublicAnchors,
}

impl VerifiedCreationPublicAnchors {
    pub fn decoded(&self) -> &DecodedCreationPublicAnchors {
        &self.decoded
    }

    pub fn into_decoded(self) -> DecodedCreationPublicAnchors {
        self.decoded
    }

    pub fn pad_seed_nullifier(&self) -> [u8; NULLIFIER_ELEMS * 4] {
        self.decoded.pad_seed_nullifier
    }
}

/// Verification / binding failure.
#[derive(Debug)]
pub enum VerifyError {
    /// The bundle bytes could not be decoded.
    Decode(String),
    /// The STARK relation verifier rejected the proof.
    Stark(String),
    /// The verified statement did not match a deployed anchor (with which one).
    AnchorMismatch(String),
}

impl core::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            VerifyError::Decode(m) => write!(f, "bundle decode failed: {m}"),
            VerifyError::Stark(m) => write!(f, "stark verify failed: {m}"),
            VerifyError::AnchorMismatch(m) => write!(f, "deployed anchor mismatch: {m}"),
        }
    }
}
impl std::error::Error for VerifyError {}

// ---------------------------------------------------------------------------
// Transport bundle: a self-contained (AccountCreationPublic, Proof) blob carried
// opaquely through the deployed payload's `proof_bytes`. The deployed crate never
// inspects it — only this crate (which has Plonky3) packs/unpacks it.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
struct PublicWire {
    pad_source_commitment: Vec<u32>,
    opening_tuple_commitment: Vec<u32>,
    masked_projection: Vec<u32>,
    account_id: [u8; 32],
    slot_id: u64,
    verifier_version: u32,
    account_state_root: [u8; 32],
    lifecycle_anchor: [u8; 32],
    role_root: Vec<u32>,
    /// CLASS A-2: the F2 single-use nullifier (`Poseidon2(DOMAIN ‖ pad_seed ‖ slot)`).
    nullifier: Vec<u32>,
    relation_id: Vec<u32>,
    syndrome: Vec<u32>,
    packed_witness_commitment: Vec<u32>,
    /// CLASS A-3: the account's registered checked-core key commitment.
    checked_core_key_commitment: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct BundleWire {
    public: PublicWire,
    /// bincode-serialized `Proof` (serialized separately to keep this layer flat).
    proof: Vec<u8>,
}

fn vals_to_u32(xs: &[Val]) -> Vec<u32> {
    xs.iter().map(|v| v.as_canonical_u32()).collect()
}

fn u32s_to_vals<const N: usize>(xs: &[u32], what: &str) -> Result<[Val; N], VerifyError> {
    if xs.len() != N {
        return Err(VerifyError::Decode(format!(
            "{what}: expected {N} field elems, got {}",
            xs.len()
        )));
    }
    let mut out = [Val::ZERO; N];
    for (i, &x) in xs.iter().enumerate() {
        if x >= Val::ORDER_U32 {
            return Err(VerifyError::Decode(format!(
                "{what}[{i}] = {x} is not a canonical KoalaBear field element"
            )));
        }
        out[i] = Val::from_u32(x);
    }
    Ok(out)
}

fn public_to_wire(p: &AccountCreationPublic) -> PublicWire {
    PublicWire {
        pad_source_commitment: vals_to_u32(&p.pad_source_commitment),
        opening_tuple_commitment: vals_to_u32(&p.opening_tuple_commitment),
        masked_projection: vals_to_u32(&p.masked_projection),
        account_id: p.account_id,
        slot_id: p.slot_id,
        verifier_version: p.verifier_version,
        account_state_root: p.account_state_root,
        lifecycle_anchor: p.lifecycle_anchor,
        role_root: vals_to_u32(&p.role_root),
        nullifier: vals_to_u32(&p.nullifier),
        relation_id: vals_to_u32(&p.relation_id),
        syndrome: vals_to_u32(&p.syndrome),
        packed_witness_commitment: vals_to_u32(&p.packed_witness_commitment),
        checked_core_key_commitment: vals_to_u32(&p.checked_core_key_commitment),
    }
}

/// Canonical transport encoding of the public account-creation statement only.
///
/// A zero-knowledge simulator consumes this statement surface and produces the
/// proof portion of a bundle. It must not receive the already-produced proof
/// bytes (or a digest of the complete replay payload) as an input.
pub fn pack_public_statement(public: &AccountCreationPublic) -> Result<Vec<u8>, VerifyError> {
    bincode::serialize(&public_to_wire(public))
        .map_err(|e| VerifyError::Decode(format!("public statement encode: {e}")))
}

/// Decode the canonical public-statement transport without any proof bytes.
pub fn unpack_public_statement(bytes: &[u8]) -> Result<AccountCreationPublic, VerifyError> {
    let wire: PublicWire = bincode::deserialize(bytes)
        .map_err(|e| VerifyError::Decode(format!("public statement: {e}")))?;
    wire_to_public(&wire)
}

/// Extract and canonically re-encode only the public statement from a bundle.
pub fn public_statement_bytes_from_bundle(bundle: &[u8]) -> Result<Vec<u8>, VerifyError> {
    let wire: BundleWire =
        bincode::deserialize(bundle).map_err(|e| VerifyError::Decode(format!("bundle: {e}")))?;
    bincode::serialize(&wire.public)
        .map_err(|e| VerifyError::Decode(format!("public statement encode: {e}")))
}

fn wire_to_public(w: &PublicWire) -> Result<AccountCreationPublic, VerifyError> {
    Ok(AccountCreationPublic {
        pad_source_commitment: u32s_to_vals::<N_COMMIT_LANES>(
            &w.pad_source_commitment,
            "pad_source_commitment",
        )?,
        opening_tuple_commitment: u32s_to_vals::<N_COMMIT_LANES>(
            &w.opening_tuple_commitment,
            "opening_tuple_commitment",
        )?,
        masked_projection: u32s_to_vals::<N_LANES>(&w.masked_projection, "masked_projection")?,
        account_id: w.account_id,
        slot_id: w.slot_id,
        verifier_version: w.verifier_version,
        account_state_root: w.account_state_root,
        lifecycle_anchor: w.lifecycle_anchor,
        role_root: u32s_to_vals::<ROLE_ROOT_ELEMS>(&w.role_root, "role_root")?,
        nullifier: u32s_to_vals::<NULLIFIER_ELEMS>(&w.nullifier, "nullifier")?,
        relation_id: u32s_to_vals::<RELATION_ID_ELEMS>(&w.relation_id, "relation_id")?,
        syndrome: u32s_to_vals::<SYNDROME_LANES>(&w.syndrome, "syndrome")?,
        packed_witness_commitment: u32s_to_vals::<PACKED_WITNESS_COMMITMENT_BYTES>(
            &w.packed_witness_commitment,
            "packed_witness_commitment",
        )?,
        checked_core_key_commitment: u32s_to_vals::<KEY_COMMITMENT_BYTES>(
            &w.checked_core_key_commitment,
            "checked_core_key_commitment",
        )?,
    })
}

/// Pack `(public, proof)` into the opaque transport bundle the deployed payload
/// carries. Used by the admission-proof PRODUCER.
pub fn pack_bundle(public: &AccountCreationPublic, proof: &Proof) -> Result<Vec<u8>, VerifyError> {
    let proof_bytes =
        bincode::serialize(proof).map_err(|e| VerifyError::Decode(format!("proof encode: {e}")))?;
    let wire = BundleWire {
        public: public_to_wire(public),
        proof: proof_bytes,
    };
    bincode::serialize(&wire).map_err(|e| VerifyError::Decode(format!("bundle encode: {e}")))
}

/// Decode the opaque transport bundle back into `(public, proof)`.
pub fn unpack_bundle(bytes: &[u8]) -> Result<(AccountCreationPublic, Proof), VerifyError> {
    let wire: BundleWire =
        bincode::deserialize(bytes).map_err(|e| VerifyError::Decode(format!("bundle: {e}")))?;
    let public = wire_to_public(&wire.public)?;
    let proof: Proof = bincode::deserialize(&wire.proof)
        .map_err(|e| VerifyError::Decode(format!("proof: {e}")))?;
    Ok((public, proof))
}

fn decoded_creation_public_anchors(
    public: &AccountCreationPublic,
) -> Result<DecodedCreationPublicAnchors, VerifyError> {
    let mut masked_projection = [0u16; 6];
    for (index, value) in public.masked_projection.iter().enumerate() {
        let residue = value.as_canonical_u32();
        if residue >= Q12289 {
            return Err(VerifyError::Decode(format!(
                "masked_projection[{index}] = {residue} not a canonical q-residue"
            )));
        }
        masked_projection[index] = residue as u16;
    }

    let mut packed_witness_commitment = [0u8; PACKED_WITNESS_COMMITMENT_BYTES];
    for (index, value) in public.packed_witness_commitment.iter().enumerate() {
        let byte = value.as_canonical_u32();
        if byte >= 256 {
            return Err(VerifyError::Decode(format!(
                "packed_witness_commitment[{index}] = {byte} not a byte"
            )));
        }
        packed_witness_commitment[index] = byte as u8;
    }

    let pad_seed_nullifier = creation_nullifier_v1_bytes(public);

    Ok(DecodedCreationPublicAnchors {
        account_id: public.account_id,
        slot_id: public.slot_id,
        account_state_root: public.account_state_root,
        lifecycle_anchor: public.lifecycle_anchor,
        verifier_version: public.verifier_version,
        masked_projection,
        native_role_root_commitment: role_root_export_digest(&public.role_root),
        packed_witness_commitment,
        pad_seed_nullifier,
    })
}

/// Decode only the public anchors from an account-creation bundle.
///
/// This does not verify the STARK proof; callers that use the decoded anchors as
/// acceptance evidence must also call [`verify_deployed_account_creation`].
pub fn decode_deployed_account_creation_public_anchors(
    bundle: &[u8],
) -> Result<DecodedCreationPublicAnchors, VerifyError> {
    let (public, _) = unpack_bundle(bundle)?;
    decoded_creation_public_anchors(&public)
}

/// THE deployed entry point. Decodes the bundle, runs the STARK relation
/// verifier, then binds the verified statement to the deployed leaf anchors
/// (direct equalities + the role-root export bridge). Returns `Ok(())` only if
/// the proof is valid AND attests THIS account/slot/state/lifecycle/version with
/// THIS masked projection and role root.
pub fn verify_and_decode_deployed_account_creation(
    bundle: &[u8],
    anchors: &DeployedCreationAnchors,
) -> Result<VerifiedCreationPublicAnchors, VerifyError> {
    let (public, proof) = unpack_bundle(bundle)?;

    // 1. The STARK relation (C1∧C2∧C4∧C6 + 8 classes) must verify.
    descender_account_stark::verify_account_creation(&public, &proof)
        .map_err(|e: StarkError| VerifyError::Stark(e.to_string()))?;

    // 2. Direct context bindings (raw values — no bridge needed).
    if public.account_id != anchors.account_id {
        return Err(VerifyError::AnchorMismatch("account_id".into()));
    }
    if public.slot_id != anchors.slot_id {
        return Err(VerifyError::AnchorMismatch("slot_id".into()));
    }
    if public.account_state_root != anchors.account_state_root {
        return Err(VerifyError::AnchorMismatch("account_state_root".into()));
    }
    if public.lifecycle_anchor != anchors.lifecycle_anchor {
        return Err(VerifyError::AnchorMismatch("lifecycle_anchor".into()));
    }
    if public.verifier_version != anchors.verifier_version {
        return Err(VerifyError::AnchorMismatch("verifier_version".into()));
    }

    // 3. masked_projection: q-residue lanes, compared as u16.
    for (j, &v) in public.masked_projection.iter().enumerate() {
        let r = v.as_canonical_u32();
        if r >= Q12289 {
            return Err(VerifyError::AnchorMismatch(format!(
                "masked_projection[{j}] = {r} not a canonical q-residue"
            )));
        }
        if r as u16 != anchors.masked_projection[j] {
            return Err(VerifyError::AnchorMismatch(format!(
                "masked_projection[{j}]"
            )));
        }
    }

    // 4. role-root export bridge (the one named seam).
    if role_root_export_digest(&public.role_root) != anchors.native_role_root_commitment {
        return Err(VerifyError::AnchorMismatch(
            "native_role_root_commitment (role-root export bridge)".into(),
        ));
    }

    // 5. CLASS C — bind the PROVEN packed-witness commitment to the deployed
    //    binding's (closes MED-2). The STARK internally proves
    //    packed_witness_commitment = pack(syndrome ‖ masked_projection), so pinning
    //    these 32 bytes to the deployed binding ties BOTH the syndrome and the
    //    masked projection to the exact packed witness the spend consumes — the
    //    syndrome is no longer an unanchored prover-chosen public input.
    for (k, &v) in public.packed_witness_commitment.iter().enumerate() {
        let b = v.as_canonical_u32();
        if b >= 256 {
            return Err(VerifyError::AnchorMismatch(format!(
                "packed_witness_commitment[{k}] = {b} not a byte"
            )));
        }
        if b as u8 != anchors.packed_witness_commitment[k] {
            return Err(VerifyError::AnchorMismatch(format!(
                "packed_witness_commitment[{k}]"
            )));
        }
    }

    let decoded = decoded_creation_public_anchors(&public)?;
    if decoded.pad_seed_nullifier != anchors.pad_seed_nullifier {
        return Err(VerifyError::AnchorMismatch("pad_seed_nullifier".into()));
    }

    Ok(VerifiedCreationPublicAnchors { decoded })
}

/// Verify the deployed account-creation proof and all anchor equalities.
pub fn verify_deployed_account_creation(
    bundle: &[u8],
    anchors: &DeployedCreationAnchors,
) -> Result<(), VerifyError> {
    verify_and_decode_deployed_account_creation(bundle, anchors).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_root_export_digest_is_domain_separated_and_ordered() {
        let role_root = [
            Val::from_u32(1),
            Val::from_u32(2),
            Val::from_u32(3),
            Val::from_u32(4),
            Val::from_u32(5),
            Val::from_u32(6),
            Val::from_u32(7),
            Val::from_u32(8),
        ];
        let digest = role_root_export_digest(&role_root);
        assert_eq!(
            digest,
            [
                117, 63, 7, 64, 6, 54, 9, 245, 118, 89, 164, 167, 41, 177, 117, 52, 216, 12, 230,
                18, 58, 86, 144, 172, 42, 81, 203, 10, 213, 122, 128, 128,
            ],
            "role-root export digest drifted"
        );

        let mut reordered = role_root;
        reordered.swap(0, 1);
        assert_ne!(
            digest,
            role_root_export_digest(&reordered),
            "role-root export bridge must be order-sensitive"
        );
    }

    #[test]
    fn public_wire_field_elements_must_be_canonical() {
        let error = u32s_to_vals::<1>(&[Val::ORDER_U32], "nullifier").unwrap_err();
        match error {
            VerifyError::Decode(message) => assert_eq!(
                message,
                format!(
                    "nullifier[0] = {} is not a canonical KoalaBear field element",
                    Val::ORDER_U32
                )
            ),
            other => panic!("unexpected canonical-wire error: {other}"),
        }
    }

    #[test]
    fn creation_nullifier_bytes_are_canonical_and_version_independent() {
        let nullifier = core::array::from_fn(|index| Val::from_u32((index as u32 * 17) + 1));
        let public = AccountCreationPublic {
            pad_source_commitment: [Val::ZERO; N_COMMIT_LANES],
            opening_tuple_commitment: [Val::ZERO; N_COMMIT_LANES],
            masked_projection: [Val::ZERO; N_LANES],
            account_id: [0_u8; 32],
            slot_id: 0,
            verifier_version: 1,
            account_state_root: [0_u8; 32],
            lifecycle_anchor: [0_u8; 32],
            role_root: [Val::ZERO; ROLE_ROOT_ELEMS],
            nullifier,
            relation_id: [Val::ZERO; RELATION_ID_ELEMS],
            syndrome: [Val::ZERO; SYNDROME_LANES],
            packed_witness_commitment: [Val::ZERO; PACKED_WITNESS_COMMITMENT_BYTES],
            checked_core_key_commitment: [Val::ZERO; KEY_COMMITMENT_BYTES],
        };
        let mut expected = [0_u8; NULLIFIER_ELEMS * 4];
        for (index, value) in nullifier.iter().enumerate() {
            expected[index * 4..(index + 1) * 4]
                .copy_from_slice(&value.as_canonical_u32().to_le_bytes());
        }
        assert_eq!(creation_nullifier_v1_bytes(&public), expected);

        let mut next_version = public;
        next_version.verifier_version = 2;
        assert_eq!(
            creation_nullifier_v1_bytes(&next_version),
            expected,
            "verifier version must not partition the global nullifier key"
        );
    }
}
