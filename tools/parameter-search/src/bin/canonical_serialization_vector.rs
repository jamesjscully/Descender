use serde::Serialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
struct Config {
    q: u64,
    n: u64,
    m0: u64,
    m1: u64,
    eta: u64,
    sample_bits: u64,
    seed_bytes: u64,
    ciphertext_overhead_bytes: u64,
    profile_name: String,
    output_dir: PathBuf,
    stem: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            q: 32771,
            n: 64,
            m0: 104,
            m1: 32,
            eta: 8,
            sample_bits: 12,
            seed_bytes: 64,
            ciphertext_overhead_bytes: 32,
            profile_name: "Descender128ProductBlockCompressedProfile.serialization".to_string(),
            output_dir: PathBuf::from("research/parameter_search/canonical_serialization_vectors"),
            stem: "descender128_product_block_compressed".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct SerializationVector {
    profile_name: String,
    q: u64,
    n: u64,
    m0: u64,
    m1: u64,
    eta: u64,
    coordinate_count: u64,
    sample_bits: u64,
    rounding_eta: u64,
    effective_eta: u64,
    seed_bytes: u64,
    ciphertext_overhead_bytes: u64,
    encoded_sample_bytes: u64,
    public_key_bytes: u64,
    ciphertext_bytes: u64,
    rounding_bound_pass: bool,
    public_key_layout: Vec<LayoutField>,
    ciphertext_layout: Vec<LayoutField>,
    public_key_skeleton_file: String,
    ciphertext_skeleton_file: String,
    notes: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
struct LayoutField {
    name: String,
    offset: u64,
    length: u64,
    description: String,
}

fn field_element_bits(q: u64) -> u64 {
    (u64::BITS as u64 - q.leading_zeros() as u64).max(1)
}

fn packed_bytes(entries: u64, bits_per_entry: u64) -> u64 {
    entries.saturating_mul(bits_per_entry).saturating_add(7) / 8
}

fn sample_compression_error(q: u64, sample_bits: u64) -> u64 {
    let full_bits = field_element_bits(q);
    if sample_bits == 0 || sample_bits >= full_bits {
        return 0;
    }
    let denominator_bits = sample_bits.saturating_add(1).min(62);
    let denominator = 1_u64 << denominator_bits;
    q.saturating_add(denominator - 1) / denominator
}

fn sample_rounding_bound_pass(q: u64, sample_bits: u64, rounding_eta: u64) -> bool {
    let full_bits = field_element_bits(q);
    if sample_bits == 0 || sample_bits >= full_bits {
        return true;
    }
    let shift = sample_bits.saturating_add(1).min(127);
    let rhs = (rounding_eta as u128).saturating_mul(1_u128 << shift);
    q as u128 <= rhs
}

fn parse_args() -> Result<Config, String> {
    let mut cfg = Config::default();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = |flag: &str| {
            args.next()
                .ok_or_else(|| format!("{flag} requires a value"))
        };
        match arg.as_str() {
            "--q" => cfg.q = value("--q")?.parse().map_err(|_| "invalid --q")?,
            "--n" => cfg.n = value("--n")?.parse().map_err(|_| "invalid --n")?,
            "--m0" => cfg.m0 = value("--m0")?.parse().map_err(|_| "invalid --m0")?,
            "--m1" => cfg.m1 = value("--m1")?.parse().map_err(|_| "invalid --m1")?,
            "--eta" => cfg.eta = value("--eta")?.parse().map_err(|_| "invalid --eta")?,
            "--sample-bits" => {
                cfg.sample_bits = value("--sample-bits")?
                    .parse()
                    .map_err(|_| "invalid --sample-bits")?
            }
            "--seed-bytes" => {
                cfg.seed_bytes = value("--seed-bytes")?
                    .parse()
                    .map_err(|_| "invalid --seed-bytes")?
            }
            "--ciphertext-overhead-bytes" => {
                cfg.ciphertext_overhead_bytes = value("--ciphertext-overhead-bytes")?
                    .parse()
                    .map_err(|_| "invalid --ciphertext-overhead-bytes")?
            }
            "--profile-name" => cfg.profile_name = value("--profile-name")?,
            "--output-dir" => cfg.output_dir = PathBuf::from(value("--output-dir")?),
            "--stem" => cfg.stem = value("--stem")?,
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    Ok(cfg)
}

fn print_help() {
    println!(
        "Emit a canonical Descender public-key/ciphertext serialization vector\n\
         \n\
         Options:\n\
           --q N\n\
           --n N\n\
           --m0 N\n\
           --m1 N\n\
           --eta N\n\
           --sample-bits N\n\
           --seed-bytes N\n\
           --ciphertext-overhead-bytes N\n\
           --profile-name NAME\n\
           --output-dir PATH\n\
           --stem NAME"
    );
}

fn build_vector(cfg: &Config) -> SerializationVector {
    let coordinate_count = cfg.m0.saturating_add(cfg.m1);
    let sample_bits = if cfg.sample_bits == 0 {
        field_element_bits(cfg.q)
    } else {
        cfg.sample_bits
    };
    let rounding_eta = sample_compression_error(cfg.q, cfg.sample_bits);
    let encoded_sample_bytes = packed_bytes(coordinate_count, sample_bits);
    let public_key_bytes = cfg.seed_bytes.saturating_add(encoded_sample_bytes);
    let ciphertext_bytes = encoded_sample_bytes.saturating_add(cfg.ciphertext_overhead_bytes);
    let pk_file = format!("{}.pk.bin", cfg.stem);
    let ct_file = format!("{}.ct.bin", cfg.stem);

    SerializationVector {
        profile_name: cfg.profile_name.clone(),
        q: cfg.q,
        n: cfg.n,
        m0: cfg.m0,
        m1: cfg.m1,
        eta: cfg.eta,
        coordinate_count,
        sample_bits,
        rounding_eta,
        effective_eta: cfg.eta.saturating_add(rounding_eta),
        seed_bytes: cfg.seed_bytes,
        ciphertext_overhead_bytes: cfg.ciphertext_overhead_bytes,
        encoded_sample_bytes,
        public_key_bytes,
        ciphertext_bytes,
        rounding_bound_pass: sample_rounding_bound_pass(cfg.q, cfg.sample_bits, rounding_eta),
        public_key_layout: vec![
            LayoutField {
                name: "seed".to_string(),
                offset: 0,
                length: cfg.seed_bytes,
                description: "Domain-separated canonical setup seed.".to_string(),
            },
            LayoutField {
                name: "packed_public_sample".to_string(),
                offset: cfg.seed_bytes,
                length: encoded_sample_bytes,
                description: "Bit-packed public sample coordinates.".to_string(),
            },
        ],
        ciphertext_layout: vec![
            LayoutField {
                name: "packed_ciphertext_sample".to_string(),
                offset: 0,
                length: encoded_sample_bytes,
                description: "Bit-packed ciphertext coordinate vector.".to_string(),
            },
            LayoutField {
                name: "ciphertext_overhead".to_string(),
                offset: encoded_sample_bytes,
                length: cfg.ciphertext_overhead_bytes,
                description: "Fixed KEM overhead carried outside the coordinate vector.".to_string(),
            },
        ],
        public_key_skeleton_file: pk_file,
        ciphertext_skeleton_file: ct_file,
        notes: vec![
            "Skeleton binaries are zero-filled size/layout artifacts, not cryptographic test vectors.".to_string(),
            "Byte counts are the canonical public sample profile consumed by the parameter search accounting.".to_string(),
            "The rounding bound mirrors PublicSampleSerializationProfile.hRoundingBound.".to_string(),
        ],
    }
}

fn markdown(vector: &SerializationVector) -> String {
    let mut lines = vec![
        format!("# Canonical Serialization Vector: {}", vector.profile_name),
        String::new(),
        format!("- q: `{}`", vector.q),
        format!("- n: `{}`", vector.n),
        format!("- m0: `{}`", vector.m0),
        format!("- m1: `{}`", vector.m1),
        format!("- eta: `{}`", vector.eta),
        format!("- coordinate count: `{}`", vector.coordinate_count),
        format!("- sample bits: `{}`", vector.sample_bits),
        format!("- rounding eta: `{}`", vector.rounding_eta),
        format!("- effective eta: `{}`", vector.effective_eta),
        format!("- encoded sample bytes: `{}`", vector.encoded_sample_bytes),
        format!("- public key bytes: `{}`", vector.public_key_bytes),
        format!("- ciphertext bytes: `{}`", vector.ciphertext_bytes),
        format!("- rounding bound pass: `{}`", vector.rounding_bound_pass),
        String::new(),
        "## Public Key Layout".to_string(),
        String::new(),
        "| field | offset | length | description |".to_string(),
        "|---|---:|---:|---|".to_string(),
    ];
    for field in &vector.public_key_layout {
        lines.push(format!(
            "| {} | {} | {} | {} |",
            field.name, field.offset, field.length, field.description
        ));
    }
    lines.extend([
        String::new(),
        "## Ciphertext Layout".to_string(),
        String::new(),
        "| field | offset | length | description |".to_string(),
        "|---|---:|---:|---|".to_string(),
    ]);
    for field in &vector.ciphertext_layout {
        lines.push(format!(
            "| {} | {} | {} | {} |",
            field.name, field.offset, field.length, field.description
        ));
    }
    lines.extend([
        String::new(),
        "## Artifacts".to_string(),
        String::new(),
        format!(
            "- public key skeleton: `{}`",
            vector.public_key_skeleton_file
        ),
        format!(
            "- ciphertext skeleton: `{}`",
            vector.ciphertext_skeleton_file
        ),
        String::new(),
    ]);
    lines.join("\n")
}

fn main() -> Result<(), String> {
    let cfg = parse_args()?;
    let vector = build_vector(&cfg);
    fs::create_dir_all(&cfg.output_dir)
        .map_err(|e| format!("create {}: {e}", cfg.output_dir.display()))?;

    let json_path = cfg.output_dir.join(format!("{}.json", cfg.stem));
    let md_path = cfg.output_dir.join(format!("{}.md", cfg.stem));
    let pk_path = cfg.output_dir.join(&vector.public_key_skeleton_file);
    let ct_path = cfg.output_dir.join(&vector.ciphertext_skeleton_file);

    let json = serde_json::to_string_pretty(&vector).map_err(|e| format!("serialize json: {e}"))?;
    fs::write(&json_path, format!("{json}\n"))
        .map_err(|e| format!("write {}: {e}", json_path.display()))?;
    fs::write(&md_path, markdown(&vector))
        .map_err(|e| format!("write {}: {e}", md_path.display()))?;
    fs::write(&pk_path, vec![0_u8; vector.public_key_bytes as usize])
        .map_err(|e| format!("write {}: {e}", pk_path.display()))?;
    fs::write(&ct_path, vec![0_u8; vector.ciphertext_bytes as usize])
        .map_err(|e| format!("write {}: {e}", ct_path.display()))?;

    println!("wrote {}", json_path.display());
    println!("wrote {}", md_path.display());
    println!("wrote {}", pk_path.display());
    println!("wrote {}", ct_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_vector_matches_descender128_lean_profile() {
        let vector = build_vector(&Config::default());
        assert_eq!(vector.coordinate_count, 136);
        assert_eq!(vector.sample_bits, 12);
        assert_eq!(vector.rounding_eta, 5);
        assert_eq!(vector.effective_eta, 13);
        assert_eq!(vector.encoded_sample_bytes, 204);
        assert_eq!(vector.public_key_bytes, 268);
        assert_eq!(vector.ciphertext_bytes, 236);
        assert!(vector.rounding_bound_pass);
    }

    #[test]
    fn layout_offsets_are_contiguous() {
        let vector = build_vector(&Config::default());
        assert_eq!(vector.public_key_layout[0].offset, 0);
        assert_eq!(vector.public_key_layout[1].offset, vector.seed_bytes);
        assert_eq!(
            vector.public_key_layout[1].offset + vector.public_key_layout[1].length,
            vector.public_key_bytes
        );
        assert_eq!(vector.ciphertext_layout[0].offset, 0);
        assert_eq!(
            vector.ciphertext_layout[1].offset,
            vector.encoded_sample_bytes
        );
        assert_eq!(
            vector.ciphertext_layout[1].offset + vector.ciphertext_layout[1].length,
            vector.ciphertext_bytes
        );
    }
}
