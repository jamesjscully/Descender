use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Config {
    output_json: PathBuf,
    output_md: PathBuf,
}

#[derive(Debug, Serialize)]
struct SignatureQ12289Profile {
    profile: String,
    q: u64,
    n: usize,
    m0_view_coordinates: usize,
    m1: usize,
    eta: u64,
    target_lee: u64,
    group_count: usize,
    rows_per_group: usize,
    local_free: usize,
    local_pivot: usize,
    public_key_seed_bytes: usize,
    public_key_payload_bytes: usize,
    public_key_bytes: usize,
    challenge_bytes: usize,
    relation_payload_bytes: usize,
    signature_bytes: usize,
    canonical_field_bits: usize,
    implicit_response_bits_per_coordinate: usize,
    implicit_response_coordinates_per_repetition: usize,
    implicit_response_bytes_per_repetition: usize,
    implicit_response_bound: u64,
    implicit_commitment_signature_bytes: usize,
    implicit_commitment_verifier_binary: String,
    implicit_commitment_hash_domain: String,
    implicit_commitment_challenge_encoding: String,
    implicit_commitment_response_encoding: String,
    implicit_commitment_public_key_encoding: String,
    implicit_commitment_reconstructed_commitment_encoding: String,
    explicit_commitment_bytes_per_repetition: usize,
    explicit_commitment_signature_bytes: usize,
    checked_centered_pair_count: u64,
    independent_min_l1: u64,
    sidecar_json: String,
    sidecar_verification_json: String,
    lean_reconstructed_commitment: String,
    lean_implicit_commitment_profile: String,
    lean_implicit_verifier_bridge: String,
    lean_q12289_implicit_verifier_alias: String,
    lean_q12289_implicit_signature_bytes: String,
    lean_q12289_implicit_core_payload_check: String,
    lean_q12289_implicit_domain_tag: String,
    lean_q12289_implicit_response_view: String,
    lean_q12289_reconstructed_commitment_view: String,
    lean_q12289_product_block_soundness_constructor: String,
    lean_q12289_commitment_view_encoder: String,
    lean_q12289_encoded_reconstruction_certificate: String,
    lean_q12289_encoded_reconstruction_bridge: String,
    lean_q12289_hash_frame: String,
    lean_q12289_hash_frame_length: String,
    lean_q12289_hash_frame_certificate: String,
    lean_q12289_hash_frame_bridge: String,
    lean_q12289_production_oracle_agreement: String,
    lean_q12289_production_certificate_constructor: String,
    lean_q12289_production_oracle_bridge: String,
    lean_q12289_local_lee_check: String,
    lean_q12289_internal_local_lee_certificate: String,
    lean_q12289_internal_local_lee_lower: String,
    lean_q12289_concrete_verifier_certificate: String,
    lean_q12289_concrete_verifier_bridge: String,
    lean_implicit_product_block_accept_bridge: String,
    lean_implicit_product_block_one_shot_bridge: String,
    rust_implicit_fixture_challenge_hex: String,
    rust_implicit_fixture_reconstructed_sha256: String,
    rust_implicit_fixture_hash_frame_sha256: String,
    lean_global_certificate: String,
    lean_fs_core_certificate: String,
    lean_no_failure_certificate: String,
    lean_no_failure_bound: String,
    required_implementation_obligations: Vec<String>,
}

fn profile() -> SignatureQ12289Profile {
    SignatureQ12289Profile {
        profile: "validated-q12289-n48-m1-32-single-carrier".to_string(),
        q: 12289,
        n: 48,
        m0_view_coordinates: 80,
        m1: 32,
        eta: 12,
        target_lee: 512,
        group_count: 16,
        rows_per_group: 3,
        local_free: 2,
        local_pivot: 3,
        public_key_seed_bytes: 32,
        public_key_payload_bytes: 140,
        public_key_bytes: 172,
        challenge_bytes: 17,
        relation_payload_bytes: 140,
        signature_bytes: 157,
        canonical_field_bits: 14,
        implicit_response_bits_per_coordinate: 9,
        implicit_response_coordinates_per_repetition: 32,
        implicit_response_bytes_per_repetition: 36,
        implicit_response_bound: 224,
        implicit_commitment_signature_bytes: 269,
        implicit_commitment_verifier_binary: "signature_q12289_implicit_verifier".to_string(),
        implicit_commitment_hash_domain:
            "Descender.Signature.q12289.implicit_commitment.v1".to_string(),
        implicit_commitment_challenge_encoding:
            "7 rounds, 19 bits/round: 4-bit omitted block plus 15 sign bits, little-endian, top 3 bits zero".to_string(),
        implicit_commitment_response_encoding:
            "7 chunks of 36 bytes; each chunk packs 32 signed 9-bit two's-complement free-response coordinates, little-endian".to_string(),
        implicit_commitment_public_key_encoding:
            "32-byte seed followed by 80 q-coordinates packed as 48 pivot then 32 free coordinates at 14 bits each".to_string(),
        implicit_commitment_reconstructed_commitment_encoding:
            "7 repetitions of reconstructed t' = A*z - chi*u, serialized as 80 q-coordinates packed at 14 bits each".to_string(),
        explicit_commitment_bytes_per_repetition: 140,
        explicit_commitment_signature_bytes: 1_235,
        checked_centered_pair_count: 523_264,
        independent_min_l1: 533,
        sidecar_json: "research/parameter_search/product_block_lee_sidecars_broad_20260516/q12289_m132_target512_rows_per_group3.json".to_string(),
        sidecar_verification_json: "research/parameter_search/product_block_lee_sidecars_broad_20260516/q12289_m132_target512_rows_per_group3.verify.json".to_string(),
        lean_reconstructed_commitment:
            "SignatureDistribution.productBlockReconstructedCommitment_honest".to_string(),
        lean_implicit_commitment_profile:
            "N32CompletedResponseSignatureByteProfile.leadingN32_9bit_2free_implicitCommitment".to_string(),
        lean_implicit_verifier_bridge:
            "N32ImplicitCommitmentByteVerifierInputs.toSerializedVerifierInputs".to_string(),
        lean_q12289_implicit_verifier_alias:
            "N32Q12289ImplicitCommitmentByteVerifier.Inputs".to_string(),
        lean_q12289_implicit_signature_bytes:
            "N32Q12289ImplicitCommitmentByteVerifier.serializedSignatureBytes_eq".to_string(),
        lean_q12289_implicit_core_payload_check:
            "N32Q12289ImplicitCommitmentByteVerifier.corePayloadCheck".to_string(),
        lean_q12289_implicit_domain_tag:
            "N32Q12289ImplicitCommitmentByteVerifier.implicitCommitmentDomainTag".to_string(),
        lean_q12289_implicit_response_view:
            "N32Q12289ImplicitCommitmentByteVerifier.responseView".to_string(),
        lean_q12289_reconstructed_commitment_view:
            "N32Q12289ImplicitCommitmentByteVerifier.reconstructedCommitmentView".to_string(),
        lean_q12289_product_block_soundness_constructor:
            "N32Q12289ImplicitCommitmentByteVerifier.productBlockEquationSoundness".to_string(),
        lean_q12289_commitment_view_encoder:
            "N32Q12289ImplicitCommitmentByteVerifier.commitmentViewEncoder".to_string(),
        lean_q12289_encoded_reconstruction_certificate:
            "N32Q12289ImplicitCommitmentByteVerifier.EncodedReconstructionVerifierCertificate"
                .to_string(),
        lean_q12289_encoded_reconstruction_bridge:
            "N32Q12289ImplicitCommitmentByteVerifier.EncodedReconstructionVerifierCertificate.verifierEquation_of_oneShotAccepts"
                .to_string(),
        lean_q12289_hash_frame:
            "N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame".to_string(),
        lean_q12289_hash_frame_length:
            "N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame_length".to_string(),
        lean_q12289_hash_frame_certificate:
            "N32Q12289ImplicitCommitmentByteVerifier.HashFrameEncodedReconstructionVerifierCertificate"
                .to_string(),
        lean_q12289_hash_frame_bridge:
            "N32Q12289ImplicitCommitmentByteVerifier.HashFrameEncodedReconstructionVerifierCertificate.verifierEquation_of_oneShotAccepts"
                .to_string(),
        lean_q12289_production_oracle_agreement:
            "N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement"
                .to_string(),
        lean_q12289_production_certificate_constructor:
            "N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement.toProductionHashFrameVerifierCertificate"
                .to_string(),
        lean_q12289_production_oracle_bridge:
            "N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement.verifierEquation_of_oneShotAccepts"
                .to_string(),
        lean_q12289_local_lee_check:
            "SharedRowProductBlock12289Rows3.boundedCenteredPairCheck_eq_true".to_string(),
        lean_q12289_internal_local_lee_certificate:
            "SharedRowProductBlock12289Rows3.internalLocalLeeCertificate".to_string(),
        lean_q12289_internal_local_lee_lower:
            "SharedRowProductBlock12289Rows3.internalLocalLeeCertificate_lower".to_string(),
        lean_q12289_concrete_verifier_certificate:
            "N32Q12289ImplicitCommitmentByteVerifier.ConcreteVerifierCertificate".to_string(),
        lean_q12289_concrete_verifier_bridge:
            "N32Q12289ImplicitCommitmentByteVerifier.ConcreteVerifierCertificate.verifierEquation_of_oneShotAccepts".to_string(),
        lean_implicit_product_block_accept_bridge:
            "N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_acceptsTranscript".to_string(),
        lean_implicit_product_block_one_shot_bridge:
            "N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_oneShotAccepts".to_string(),
        rust_implicit_fixture_challenge_hex: "500b7e675f1970875c294ccb0d00eee306".to_string(),
        rust_implicit_fixture_reconstructed_sha256:
            "b849747fe881a40d5e534e90128d6975e470de03a308a0c82fd2bc6226cd9c85"
                .to_string(),
        rust_implicit_fixture_hash_frame_sha256:
            "500b7e675f1970875c294ccb0d00eee386910b1a37c8ecf5b78d97e0af51993c"
                .to_string(),
        lean_global_certificate: "N32Q12289Validated48x32CanonicalBaseSecurityCertificate".to_string(),
        lean_fs_core_certificate:
            "N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate".to_string(),
        lean_no_failure_certificate: "N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate".to_string(),
        lean_no_failure_bound: "Pr[forgery] <= 2^-133".to_string(),
        required_implementation_obligations: vec![
            "Supply N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement for the production verifier: SHA-256-to-challenge over q12289HashFrame agrees with the query-log oracle on accepted transcripts.".to_string(),
            "Keep the Rust verifier conformance fixtures pinned to the Lean q12289 byte and hash-frame contract.".to_string(),
        ],
    }
}

fn markdown(p: &SignatureQ12289Profile) -> String {
    format!(
        "# q12289 Validated Signature Profile\n\n\
         | field | value |\n\
         |---|---:|\n\
         | profile | `{profile}` |\n\
         | q | {q} |\n\
         | n | {n} |\n\
         | m0 view coordinates | {m0} |\n\
         | m1 | {m1} |\n\
         | eta | {eta} |\n\
         | target Lee | {target_lee} |\n\
         | groups | {groups} |\n\
         | rows/group | {rows_per_group} |\n\
         | public key bytes | {pk_bytes} |\n\
         | challenge bytes | {challenge_bytes} |\n\
         | relation payload bytes | {payload_bytes} |\n\
         | single-carrier relation signature bytes | {sig_bytes} |\n\
         | implicit response bits/coordinate | {implicit_bits} |\n\
         | implicit response coordinates/repetition | {implicit_coords} |\n\
         | implicit response bytes/repetition | {implicit_response_bytes} |\n\
         | implicit response bound | {implicit_response_bound} |\n\
         | implicit commitment signature bytes | {implicit_sig_bytes} |\n\
         | implicit commitment verifier | `{implicit_verifier}` |\n\
         | implicit commitment hash domain | `{implicit_hash_domain}` |\n\
         | explicit commitment bytes/repetition | {explicit_commitment_bytes} |\n\
         | explicit commitment signature bytes | {explicit_sig_bytes} |\n\
         | checked centered pairs | {checked_pairs} |\n\
         | independent min l1 | {min_l1} |\n\n\
         ## Lean Endpoints\n\n\
         - reconstructed commitment: `{reconstructed_commitment}`\n\
         - implicit commitment byte profile: `{implicit_profile}`\n\
         - implicit verifier bridge: `{implicit_verifier_bridge}`\n\
         - q12289 implicit verifier alias: `{q12289_implicit_alias}`\n\
         - q12289 implicit signature bytes: `{q12289_implicit_sig_bytes}`\n\
         - q12289 implicit core payload check: `{q12289_implicit_core_payload}`\n\
         - q12289 implicit domain tag: `{q12289_implicit_domain_tag}`\n\
         - q12289 implicit response view: `{q12289_implicit_response_view}`\n\
         - q12289 reconstructed commitment view: `{q12289_reconstructed_view}`\n\
         - q12289 product-block soundness constructor: `{q12289_product_block_constructor}`\n\
         - q12289 commitment view encoder: `{q12289_commitment_view_encoder}`\n\
         - q12289 encoded-reconstruction certificate: `{q12289_encoded_reconstruction_certificate}`\n\
         - q12289 encoded-reconstruction bridge: `{q12289_encoded_reconstruction_bridge}`\n\
         - q12289 hash frame: `{q12289_hash_frame}`\n\
         - q12289 hash frame length theorem: `{q12289_hash_frame_length}`\n\
         - q12289 hash-frame certificate: `{q12289_hash_frame_certificate}`\n\
         - q12289 hash-frame bridge: `{q12289_hash_frame_bridge}`\n\
         - q12289 production oracle agreement: `{q12289_production_oracle_agreement}`\n\
         - q12289 production certificate constructor: `{q12289_production_certificate_constructor}`\n\
         - q12289 production oracle bridge: `{q12289_production_oracle_bridge}`\n\
         - q12289 bounded local Lee check: `{q12289_local_lee_check}`\n\
         - q12289 internal local Lee certificate: `{q12289_internal_local_lee_certificate}`\n\
         - q12289 internal local Lee lower bound: `{q12289_internal_local_lee_lower}`\n\
         - q12289 concrete verifier certificate: `{q12289_concrete_certificate}`\n\
         - q12289 concrete verifier bridge: `{q12289_concrete_bridge}`\n\
         - implicit accepted-transcript product-block bridge: `{implicit_product_block_accept_bridge}`\n\
         - implicit one-shot product-block bridge: `{implicit_product_block_one_shot_bridge}`\n\
         - global certificate: `{global_certificate}`\n\
         - FS-core no-failure certificate: `{fs_core_certificate}`\n\
         - canonical no-failure certificate: `{no_failure_certificate}`\n\
         - no-failure bound: `{no_failure_bound}`\n\n\
         ## Artifacts\n\n\
         - sidecar: `{sidecar}`\n\
         - verification: `{verification}`\n\n\
         ## Concrete Implicit-Commitment Codec\n\n\
         - challenge: {challenge_encoding}\n\
         - response: {response_encoding}\n\
         - public key: {public_key_encoding}\n\
         - reconstructed commitment: {reconstructed_commitment_encoding}\n\n\
         ## Rust Conformance Fixtures\n\n\
         - zero-public-key challenge: `{fixture_challenge_hex}`\n\
         - zero-public-key reconstructed commitment SHA-256: `{fixture_reconstructed_sha256}`\n\
         - zero-public-key hash frame SHA-256: `{fixture_hash_frame_sha256}`\n\n\
         ## Remaining Implementation Obligations\n\n\
         {obligations}\n",
        profile = p.profile,
        q = p.q,
        n = p.n,
        m0 = p.m0_view_coordinates,
        m1 = p.m1,
        eta = p.eta,
        target_lee = p.target_lee,
        groups = p.group_count,
        rows_per_group = p.rows_per_group,
        pk_bytes = p.public_key_bytes,
        challenge_bytes = p.challenge_bytes,
        payload_bytes = p.relation_payload_bytes,
        sig_bytes = p.signature_bytes,
        implicit_bits = p.implicit_response_bits_per_coordinate,
        implicit_coords = p.implicit_response_coordinates_per_repetition,
        implicit_response_bytes = p.implicit_response_bytes_per_repetition,
        implicit_response_bound = p.implicit_response_bound,
        implicit_sig_bytes = p.implicit_commitment_signature_bytes,
        implicit_verifier = p.implicit_commitment_verifier_binary,
        implicit_hash_domain = p.implicit_commitment_hash_domain,
        explicit_commitment_bytes = p.explicit_commitment_bytes_per_repetition,
        explicit_sig_bytes = p.explicit_commitment_signature_bytes,
        checked_pairs = p.checked_centered_pair_count,
        min_l1 = p.independent_min_l1,
        reconstructed_commitment = p.lean_reconstructed_commitment,
        implicit_profile = p.lean_implicit_commitment_profile,
        implicit_verifier_bridge = p.lean_implicit_verifier_bridge,
        q12289_implicit_alias = p.lean_q12289_implicit_verifier_alias,
        q12289_implicit_sig_bytes = p.lean_q12289_implicit_signature_bytes,
        q12289_implicit_core_payload = p.lean_q12289_implicit_core_payload_check,
        q12289_implicit_domain_tag = p.lean_q12289_implicit_domain_tag,
        q12289_implicit_response_view = p.lean_q12289_implicit_response_view,
        q12289_reconstructed_view = p.lean_q12289_reconstructed_commitment_view,
        q12289_product_block_constructor = p.lean_q12289_product_block_soundness_constructor,
        q12289_commitment_view_encoder = p.lean_q12289_commitment_view_encoder,
        q12289_encoded_reconstruction_certificate =
            p.lean_q12289_encoded_reconstruction_certificate,
        q12289_encoded_reconstruction_bridge = p.lean_q12289_encoded_reconstruction_bridge,
        q12289_hash_frame = p.lean_q12289_hash_frame,
        q12289_hash_frame_length = p.lean_q12289_hash_frame_length,
        q12289_hash_frame_certificate = p.lean_q12289_hash_frame_certificate,
        q12289_hash_frame_bridge = p.lean_q12289_hash_frame_bridge,
        q12289_production_oracle_agreement = p.lean_q12289_production_oracle_agreement,
        q12289_production_certificate_constructor =
            p.lean_q12289_production_certificate_constructor,
        q12289_production_oracle_bridge = p.lean_q12289_production_oracle_bridge,
        q12289_local_lee_check = p.lean_q12289_local_lee_check,
        q12289_internal_local_lee_certificate = p.lean_q12289_internal_local_lee_certificate,
        q12289_internal_local_lee_lower = p.lean_q12289_internal_local_lee_lower,
        q12289_concrete_certificate = p.lean_q12289_concrete_verifier_certificate,
        q12289_concrete_bridge = p.lean_q12289_concrete_verifier_bridge,
        implicit_product_block_accept_bridge = p.lean_implicit_product_block_accept_bridge,
        implicit_product_block_one_shot_bridge = p.lean_implicit_product_block_one_shot_bridge,
        global_certificate = p.lean_global_certificate,
        fs_core_certificate = p.lean_fs_core_certificate,
        no_failure_certificate = p.lean_no_failure_certificate,
        no_failure_bound = p.lean_no_failure_bound,
        sidecar = p.sidecar_json,
        verification = p.sidecar_verification_json,
        challenge_encoding = p.implicit_commitment_challenge_encoding,
        response_encoding = p.implicit_commitment_response_encoding,
        public_key_encoding = p.implicit_commitment_public_key_encoding,
        reconstructed_commitment_encoding = p.implicit_commitment_reconstructed_commitment_encoding,
        fixture_challenge_hex = p.rust_implicit_fixture_challenge_hex,
        fixture_reconstructed_sha256 = p.rust_implicit_fixture_reconstructed_sha256,
        fixture_hash_frame_sha256 = p.rust_implicit_fixture_hash_frame_sha256,
        obligations = p
            .required_implementation_obligations
            .iter()
            .map(|x| format!("- {x}"))
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

fn parse_args() -> Config {
    let mut args = std::env::args().skip(1);
    let mut cfg = Config {
        output_json: PathBuf::from(
            "research/cryptanalysis/signature_readiness_20260516/q12289_validated_signature_profile.json",
        ),
        output_md: PathBuf::from(
            "research/cryptanalysis/signature_readiness_20260516/q12289_validated_signature_profile.md",
        ),
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output-json" => cfg.output_json = PathBuf::from(args.next().unwrap()),
            "--output-md" => cfg.output_md = PathBuf::from(args.next().unwrap()),
            _ => panic!("unknown argument: {arg}"),
        }
    }
    cfg
}

fn main() {
    let cfg = parse_args();
    let p = profile();
    if let Some(parent) = cfg.output_json.parent() {
        fs::create_dir_all(parent).expect("create output json parent");
    }
    if let Some(parent) = cfg.output_md.parent() {
        fs::create_dir_all(parent).expect("create output md parent");
    }
    fs::write(
        &cfg.output_json,
        serde_json::to_string_pretty(&p).unwrap() + "\n",
    )
    .expect("write output json");
    fs::write(&cfg.output_md, markdown(&p)).expect("write output md");
    println!("wrote {}", cfg.output_json.display());
    println!("wrote {}", cfg.output_md.display());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_profile_matches_current_target() {
        let p = profile();
        assert_eq!(p.public_key_bytes, 172);
        assert_eq!(p.challenge_bytes, 17);
        assert_eq!(p.relation_payload_bytes, 140);
        assert_eq!(p.signature_bytes, 157);
        assert_eq!(p.implicit_response_coordinates_per_repetition, 32);
        assert_eq!(p.implicit_response_bytes_per_repetition, 36);
        assert_eq!(p.implicit_response_bound, 224);
        assert_eq!(p.implicit_commitment_signature_bytes, 269);
        assert_eq!(
            p.implicit_commitment_verifier_binary,
            "signature_q12289_implicit_verifier"
        );
        assert_eq!(
            p.implicit_commitment_hash_domain,
            "Descender.Signature.q12289.implicit_commitment.v1"
        );
        assert_eq!(
            p.lean_implicit_verifier_bridge,
            "N32ImplicitCommitmentByteVerifierInputs.toSerializedVerifierInputs"
        );
        assert_eq!(
            p.lean_q12289_implicit_signature_bytes,
            "N32Q12289ImplicitCommitmentByteVerifier.serializedSignatureBytes_eq"
        );
        assert_eq!(
            p.lean_q12289_product_block_soundness_constructor,
            "N32Q12289ImplicitCommitmentByteVerifier.productBlockEquationSoundness"
        );
        assert_eq!(
            p.lean_q12289_commitment_view_encoder,
            "N32Q12289ImplicitCommitmentByteVerifier.commitmentViewEncoder"
        );
        assert_eq!(
            p.lean_q12289_encoded_reconstruction_certificate,
            "N32Q12289ImplicitCommitmentByteVerifier.EncodedReconstructionVerifierCertificate"
        );
        assert_eq!(
            p.lean_q12289_hash_frame,
            "N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame"
        );
        assert_eq!(
            p.lean_q12289_hash_frame_length,
            "N32Q12289ImplicitCommitmentByteVerifier.q12289HashFrame_length"
        );
        assert_eq!(
            p.lean_q12289_hash_frame_certificate,
            "N32Q12289ImplicitCommitmentByteVerifier.HashFrameEncodedReconstructionVerifierCertificate"
        );
        assert_eq!(
            p.lean_q12289_production_oracle_agreement,
            "N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement"
        );
        assert_eq!(
            p.lean_q12289_production_certificate_constructor,
            "N32Q12289ImplicitCommitmentByteVerifier.ProductionHashFrameOracleAgreement.toProductionHashFrameVerifierCertificate"
        );
        assert_eq!(
            p.lean_q12289_local_lee_check,
            "SharedRowProductBlock12289Rows3.boundedCenteredPairCheck_eq_true"
        );
        assert_eq!(
            p.lean_q12289_internal_local_lee_certificate,
            "SharedRowProductBlock12289Rows3.internalLocalLeeCertificate"
        );
        assert_eq!(
            p.lean_q12289_internal_local_lee_lower,
            "SharedRowProductBlock12289Rows3.internalLocalLeeCertificate_lower"
        );
        assert_eq!(
            p.lean_q12289_concrete_verifier_certificate,
            "N32Q12289ImplicitCommitmentByteVerifier.ConcreteVerifierCertificate"
        );
        assert_eq!(
            p.lean_implicit_product_block_accept_bridge,
            "N32ImplicitProductBlockVerifierEquationSoundness.verifierEquation_of_acceptsTranscript"
        );
        assert_eq!(
            p.rust_implicit_fixture_challenge_hex,
            "500b7e675f1970875c294ccb0d00eee306"
        );
        assert_eq!(
            p.rust_implicit_fixture_hash_frame_sha256,
            "500b7e675f1970875c294ccb0d00eee386910b1a37c8ecf5b78d97e0af51993c"
        );
        assert_eq!(p.explicit_commitment_bytes_per_repetition, 140);
        assert_eq!(p.explicit_commitment_signature_bytes, 1_235);
        assert_eq!(p.checked_centered_pair_count, 523_264);
        assert_eq!(p.independent_min_l1, 533);
    }

    #[test]
    fn no_failure_endpoint_is_recorded() {
        let p = profile();
        assert_eq!(
            p.lean_fs_core_certificate,
            "N32Q12289Validated48x32FSCoreNoFailureSecurityCertificate"
        );
        assert_eq!(
            p.lean_no_failure_certificate,
            "N32Q12289Validated48x32CanonicalNoFailureSecurityCertificate"
        );
        assert_eq!(p.lean_no_failure_bound, "Pr[forgery] <= 2^-133");
    }
}
