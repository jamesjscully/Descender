use serde::Serialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
struct Config {
    q: u64,
    n: u64,
    eta: u64,
    product_block_groups: u64,
    target_bits: f64,
    digest_bytes: u64,
    max_log2_retries: f64,
    max_signature_bytes: u64,
    scalar_bounds: Vec<u64>,
    block_weights: Vec<u64>,
    z_bounds: Vec<u64>,
    output_json: PathBuf,
    output_md: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            q: 12289,
            n: 48,
            eta: 12,
            product_block_groups: 16,
            target_bits: 128.0,
            digest_bytes: 32,
            max_log2_retries: 20.0,
            max_signature_bytes: 2048,
            scalar_bounds: vec![1, 2, 3, 4, 7, 15, 31, 63, 127, 255, 383, 511],
            block_weights: (1..=16).collect(),
            z_bounds: vec![
                12, 16, 24, 32, 48, 64, 96, 128, 192, 256, 384, 512, 768, 1024, 1536, 2048, 3072,
                4096, 6144, 8192,
            ],
            output_json: PathBuf::from(
                "research/cryptanalysis/signature_readiness_20260516/aborts_size_search.json",
            ),
            output_md: PathBuf::from(
                "research/cryptanalysis/signature_readiness_20260516/aborts_size_search.md",
            ),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct SearchOutput {
    q: u64,
    n: u64,
    eta: u64,
    target_bits: f64,
    digest_bytes: u64,
    max_log2_retries: f64,
    scalar_best: Vec<SearchRow>,
    block_endomorphism_best: Vec<SearchRow>,
    conclusions: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
struct SearchRow {
    model: String,
    proof_status: String,
    challenge_description: String,
    challenge_entropy_bits: f64,
    repetitions: u64,
    secret_multiplier_bound: u64,
    verifier_noise_bound: u64,
    verifier_noise_headroom: i64,
    accepted_z_bound: u64,
    z_bits_per_coordinate: u64,
    response_bytes_per_repetition: u64,
    estimated_signature_bytes: u64,
    accept_probability_per_repetition: f64,
    log2_expected_retries: f64,
    verdict: String,
}

fn parse_csv_u64(s: &str) -> Vec<u64> {
    s.split(',')
        .filter(|part| !part.is_empty())
        .map(|part| part.parse().expect("invalid integer CSV"))
        .collect()
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
            "--eta" => cfg.eta = value("--eta")?.parse().map_err(|_| "invalid --eta")?,
            "--product-block-groups" => {
                cfg.product_block_groups = value("--product-block-groups")?
                    .parse()
                    .map_err(|_| "invalid --product-block-groups")?
            }
            "--target-bits" => {
                cfg.target_bits = value("--target-bits")?
                    .parse()
                    .map_err(|_| "invalid --target-bits")?
            }
            "--digest-bytes" => {
                cfg.digest_bytes = value("--digest-bytes")?
                    .parse()
                    .map_err(|_| "invalid --digest-bytes")?
            }
            "--max-log2-retries" => {
                cfg.max_log2_retries = value("--max-log2-retries")?
                    .parse()
                    .map_err(|_| "invalid --max-log2-retries")?
            }
            "--max-signature-bytes" => {
                cfg.max_signature_bytes = value("--max-signature-bytes")?
                    .parse()
                    .map_err(|_| "invalid --max-signature-bytes")?
            }
            "--scalar-bounds" => cfg.scalar_bounds = parse_csv_u64(&value("--scalar-bounds")?),
            "--block-weights" => cfg.block_weights = parse_csv_u64(&value("--block-weights")?),
            "--z-bounds" => cfg.z_bounds = parse_csv_u64(&value("--z-bounds")?),
            "--output-json" => cfg.output_json = PathBuf::from(value("--output-json")?),
            "--output-md" => cfg.output_md = PathBuf::from(value("--output-md")?),
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
        "Search small-signature Fiat-Shamir-with-aborts tradeoffs\n\
         \n\
         Defaults target the 172-byte coordinate-subview row.\n\
         \n\
         Options:\n\
           --q N\n\
           --n N\n\
           --eta N\n\
           --product-block-groups N\n\
           --target-bits FLOAT\n\
           --digest-bytes N\n\
           --max-log2-retries FLOAT\n\
           --max-signature-bytes N\n\
           --scalar-bounds CSV\n\
           --block-weights CSV\n\
           --z-bounds CSV\n\
           --output-json PATH\n\
           --output-md PATH"
    );
}

fn ceil_log2(n: u64) -> u64 {
    if n <= 1 {
        0
    } else {
        u64::BITS as u64 - (n - 1).leading_zeros() as u64
    }
}

fn packed_bytes(entries: u64, bits_per_entry: u64) -> u64 {
    entries.saturating_mul(bits_per_entry).saturating_add(7) / 8
}

fn log2_binomial(n: u64, k: u64) -> f64 {
    if k > n {
        return f64::NEG_INFINITY;
    }
    let k = k.min(n - k);
    (0..k)
        .map(|i| ((n - i) as f64).log2() - ((i + 1) as f64).log2())
        .sum()
}

fn build_row(
    cfg: &Config,
    model: &str,
    proof_status: &str,
    challenge_description: String,
    challenge_entropy_bits: f64,
    secret_multiplier_bound: u64,
    z_bound: u64,
) -> Option<SearchRow> {
    if challenge_entropy_bits <= 0.0 {
        return None;
    }
    let verifier_noise_bound = secret_multiplier_bound.saturating_mul(cfg.eta);
    let verifier_noise_headroom = (cfg.q / 2) as i64 - verifier_noise_bound as i64;
    if verifier_noise_headroom < 0 {
        return None;
    }
    let repetitions = (cfg.target_bits / challenge_entropy_bits).ceil() as u64;
    let gamma = z_bound.saturating_add(verifier_noise_bound);
    let accept_probability_per_repetition =
        (2 * z_bound + 1) as f64 / (2 * gamma + 1).max(1) as f64;
    if accept_probability_per_repetition <= 0.0 {
        return None;
    }
    let log2_expected_retries = repetitions as f64 * (-accept_probability_per_repetition.log2());
    let z_bits_per_coordinate = ceil_log2(2 * z_bound + 1);
    let response_bytes_per_repetition = packed_bytes(cfg.n, z_bits_per_coordinate);
    let estimated_signature_bytes = cfg
        .digest_bytes
        .saturating_add(repetitions.saturating_mul(response_bytes_per_repetition));
    let verdict = if log2_expected_retries > cfg.max_log2_retries {
        "retry-cost-too-high"
    } else if estimated_signature_bytes > cfg.max_signature_bytes {
        "too-large"
    } else if proof_status == "hypothetical-endomorphism-required" {
        "size-promising-proof-gap"
    } else {
        "candidate"
    }
    .to_string();
    Some(SearchRow {
        model: model.to_string(),
        proof_status: proof_status.to_string(),
        challenge_description,
        challenge_entropy_bits,
        repetitions,
        secret_multiplier_bound,
        verifier_noise_bound,
        verifier_noise_headroom,
        accepted_z_bound: z_bound,
        z_bits_per_coordinate,
        response_bytes_per_repetition,
        estimated_signature_bytes,
        accept_probability_per_repetition,
        log2_expected_retries,
        verdict,
    })
}

fn keep_best(mut rows: Vec<SearchRow>, limit: usize) -> Vec<SearchRow> {
    rows.sort_by(|a, b| {
        a.estimated_signature_bytes
            .cmp(&b.estimated_signature_bytes)
            .then_with(|| {
                a.log2_expected_retries
                    .partial_cmp(&b.log2_expected_retries)
                    .unwrap()
            })
            .then_with(|| {
                b.challenge_entropy_bits
                    .partial_cmp(&a.challenge_entropy_bits)
                    .unwrap()
            })
    });
    rows.truncate(limit);
    rows
}

fn run_search(cfg: &Config) -> SearchOutput {
    let mut scalar = Vec::new();
    for &bound in &cfg.scalar_bounds {
        let entropy = (2 * bound).max(1) as f64;
        let entropy = entropy.log2();
        for &z_bound in &cfg.z_bounds {
            if let Some(row) = build_row(
                cfg,
                "scalar-fs-with-aborts",
                "compatible-with-current-linear-public-relation",
                format!("scalar c in [-{bound},{bound}] \\ {{0}}"),
                entropy,
                bound,
                z_bound,
            ) {
                if row.verdict == "candidate" {
                    scalar.push(row);
                }
            }
        }
    }

    let mut block = Vec::new();
    for &weight in &cfg.block_weights {
        if weight > cfg.product_block_groups {
            continue;
        }
        let entropy = log2_binomial(cfg.product_block_groups, weight) + weight as f64;
        for &z_bound in &cfg.z_bounds {
            if let Some(row) = build_row(
                cfg,
                "product-block-sparse-challenge",
                "hypothetical-endomorphism-required",
                format!(
                    "choose {weight} signed product blocks among {}",
                    cfg.product_block_groups
                ),
                entropy,
                1,
                z_bound,
            ) {
                if row.verdict == "size-promising-proof-gap" {
                    block.push(row);
                }
            }
        }
    }

    SearchOutput {
        q: cfg.q,
        n: cfg.n,
        eta: cfg.eta,
        target_bits: cfg.target_bits,
        digest_bytes: cfg.digest_bytes,
        max_log2_retries: cfg.max_log2_retries,
        scalar_best: keep_best(scalar, 20),
        block_endomorphism_best: keep_best(block, 20),
        conclusions: vec![
            "Scalar Fiat-Shamir-with-aborts is the conservative algebraic route because the verifier can multiply the public relation by a scalar challenge, but its best small-signature rows are still around the kilobyte scale under practical retry limits.".to_string(),
            "A high-entropy product-block challenge with coefficients in {-1,0,1} would be dramatically smaller, because it keeps the secret/noise multiplier at one while extracting many challenge bits per repetition.".to_string(),
            "The product-block challenge rows are not currently schemes: they require a new proof/algebraic design showing that the public view supports the corresponding block-local challenge action.".to_string(),
            "If sub-kilobyte signatures are mandatory, the research target should be an endomorphism/equivariance theorem for product-block challenges, not a generic ZK proof of the KEM relation.".to_string(),
        ],
    }
}

fn markdown(out: &SearchOutput) -> String {
    let mut lines = vec![
        "# Signature Abort-Size Search".to_string(),
        String::new(),
        format!("- q: `{}`", out.q),
        format!("- n: `{}`", out.n),
        format!("- eta: `{}`", out.eta),
        format!("- target challenge bits: `{:.1}`", out.target_bits),
        format!("- digest bytes: `{}`", out.digest_bytes),
        format!("- max log2 expected retries: `{:.1}`", out.max_log2_retries),
        String::new(),
        "## Best Scalar FS-With-Aborts Rows".to_string(),
        String::new(),
    ];
    append_table(&mut lines, &out.scalar_best);
    lines.extend([
        String::new(),
        "## Best Hypothetical Product-Block Challenge Rows".to_string(),
        String::new(),
    ]);
    append_table(&mut lines, &out.block_endomorphism_best);
    lines.extend([String::new(), "## Conclusions".to_string(), String::new()]);
    for conclusion in &out.conclusions {
        lines.push(format!("- {conclusion}"));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn append_table(lines: &mut Vec<String>, rows: &[SearchRow]) {
    lines.push("| model | challenge | entropy | reps | z bound | z bits | response bytes | sig bytes | log2 retries | noise headroom | verdict |".to_string());
    lines.push("|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---|".to_string());
    for row in rows {
        lines.push(format!(
            "| {} | {} | {:.2} | {} | {} | {} | {} | {} | {:.2} | {} | {} |",
            row.model,
            row.challenge_description,
            row.challenge_entropy_bits,
            row.repetitions,
            row.accepted_z_bound,
            row.z_bits_per_coordinate,
            row.response_bytes_per_repetition,
            row.estimated_signature_bytes,
            row.log2_expected_retries,
            row.verifier_noise_headroom,
            row.verdict
        ));
    }
}

fn main() -> Result<(), String> {
    let cfg = parse_args()?;
    let out = run_search(&cfg);
    if let Some(parent) = cfg.output_json.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    if let Some(parent) = cfg.output_md.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(&out).map_err(|e| format!("serialize json: {e}"))?;
    fs::write(&cfg.output_json, format!("{json}\n"))
        .map_err(|e| format!("write {}: {e}", cfg.output_json.display()))?;
    fs::write(&cfg.output_md, markdown(&out))
        .map_err(|e| format!("write {}: {e}", cfg.output_md.display()))?;
    println!("wrote {}", cfg.output_json.display());
    println!("wrote {}", cfg.output_md.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_finds_scalar_and_block_rows() {
        let out = run_search(&Config::default());
        assert!(!out.scalar_best.is_empty());
        assert!(!out.block_endomorphism_best.is_empty());
    }

    #[test]
    fn block_challenge_can_be_subkilobyte_in_size_model() {
        let out = run_search(&Config::default());
        assert!(
            out.block_endomorphism_best
                .iter()
                .any(|row| row.estimated_signature_bytes < 512)
        );
    }
}
