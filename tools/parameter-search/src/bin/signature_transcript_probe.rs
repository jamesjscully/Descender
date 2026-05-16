use serde::Serialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
struct Config {
    q: u64,
    n: usize,
    secret_eta: i64,
    mask_bounds: Vec<i64>,
    transcript_counts: Vec<usize>,
    trials: usize,
    seed: u64,
    output_json: PathBuf,
    output_md: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            q: 12289,
            n: 48,
            secret_eta: 12,
            mask_bounds: vec![12, 32, 64, 128, 256, 512, 1024, 2048],
            transcript_counts: vec![1, 2, 4, 8, 16, 32],
            trials: 64,
            seed: 0xD35CE5DE_517A_172,
            output_json: PathBuf::from(
                "research/cryptanalysis/signature_readiness_20260516/transcript_probe.json",
            ),
            output_md: PathBuf::from(
                "research/cryptanalysis/signature_readiness_20260516/transcript_probe.md",
            ),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct ProbeOutput {
    model: String,
    q: u64,
    n: usize,
    secret_eta: i64,
    trials: usize,
    rows: Vec<ProbeRow>,
    conclusions: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
struct ProbeRow {
    mask_bound: i64,
    transcripts: usize,
    recovered_trials: usize,
    full_recovery_rate: f64,
    average_coordinate_recovery_rate: f64,
    average_candidate_gap: f64,
    verdict: String,
}

#[derive(Clone)]
struct Rng64 {
    state: u64,
}

impl Rng64 {
    fn new(seed: u64) -> Self {
        Self { state: seed | 1 }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    fn uniform(&mut self, modulus: u64) -> u64 {
        self.next_u64() % modulus
    }

    fn nonzero_mod(&mut self, q: u64) -> u64 {
        1 + self.uniform(q - 1)
    }

    fn centered(&mut self, bound: i64) -> i64 {
        if bound <= 0 {
            return 0;
        }
        self.uniform((2 * bound + 1) as u64) as i64 - bound
    }
}

fn parse_csv_i64(s: &str) -> Vec<i64> {
    s.split(',')
        .filter(|part| !part.is_empty())
        .map(|part| part.parse().expect("invalid integer CSV"))
        .collect()
}

fn parse_csv_usize(s: &str) -> Vec<usize> {
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
            "--secret-eta" => {
                cfg.secret_eta = value("--secret-eta")?
                    .parse()
                    .map_err(|_| "invalid --secret-eta")?
            }
            "--mask-bounds" => cfg.mask_bounds = parse_csv_i64(&value("--mask-bounds")?),
            "--transcripts" => cfg.transcript_counts = parse_csv_usize(&value("--transcripts")?),
            "--trials" => {
                cfg.trials = value("--trials")?.parse().map_err(|_| "invalid --trials")?
            }
            "--seed" => cfg.seed = value("--seed")?.parse().map_err(|_| "invalid --seed")?,
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
        "Probe scalar Fiat-Shamir transcript leakage z = y + c*s mod q\n\
         \n\
         Defaults target the 172-byte coordinate-subview row.\n\
         \n\
         Options:\n\
           --q N\n\
           --n N\n\
           --secret-eta N\n\
           --mask-bounds CSV\n\
           --transcripts CSV\n\
           --trials N\n\
           --seed N\n\
           --output-json PATH\n\
           --output-md PATH"
    );
}

fn mod_q(x: i128, q: u64) -> u64 {
    let q = q as i128;
    let r = x.rem_euclid(q);
    r as u64
}

fn centered_mod(x: u64, q: u64) -> i64 {
    let r = x % q;
    if r > q / 2 {
        -((q - r) as i64)
    } else {
        r as i64
    }
}

fn abs_centered_residual_le(z: u64, c: u64, guess: i64, q: u64, bound: i64) -> bool {
    let predicted = mod_q(c as i128 * guess as i128, q);
    let residual = centered_mod(mod_q(z as i128 - predicted as i128, q), q);
    residual.unsigned_abs() <= bound as u64
}

fn residual_score(zs: &[u64], cs: &[u64], guess: i64, q: u64, bound: i64) -> usize {
    zs.iter()
        .zip(cs)
        .filter(|&(z, c)| abs_centered_residual_le(*z, *c, guess, q, bound))
        .count()
}

fn run_trial(
    rng: &mut Rng64,
    q: u64,
    n: usize,
    secret_eta: i64,
    mask_bound: i64,
    transcripts: usize,
) -> (usize, f64) {
    let secret: Vec<i64> = (0..n).map(|_| rng.centered(secret_eta)).collect();
    let mut challenges = Vec::with_capacity(transcripts);
    let mut responses = vec![Vec::with_capacity(transcripts); n];
    for _ in 0..transcripts {
        let c = rng.nonzero_mod(q);
        challenges.push(c);
        for j in 0..n {
            let y = rng.centered(mask_bound);
            let z = mod_q(y as i128 + c as i128 * secret[j] as i128, q);
            responses[j].push(z);
        }
    }

    let mut recovered = 0;
    let mut total_gap = 0.0;
    for j in 0..n {
        let mut scored: Vec<(usize, i64)> = (-secret_eta..=secret_eta)
            .map(|guess| {
                (
                    residual_score(&responses[j], &challenges, guess, q, mask_bound),
                    guess,
                )
            })
            .collect();
        scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.abs().cmp(&b.1.abs())));
        if scored[0].1 == secret[j] {
            recovered += 1;
        }
        let second = scored.get(1).map(|x| x.0).unwrap_or(0);
        total_gap += (scored[0].0 as i64 - second as i64) as f64;
    }
    (recovered, total_gap / n as f64)
}

fn run_probe(cfg: &Config) -> ProbeOutput {
    let mut rows = Vec::new();
    for &mask_bound in &cfg.mask_bounds {
        for &transcripts in &cfg.transcript_counts {
            let mut full = 0;
            let mut recovered_coords = 0;
            let mut gap = 0.0;
            for trial in 0..cfg.trials {
                let seed = cfg.seed
                    ^ ((mask_bound as u64).wrapping_mul(0x9E3779B97F4A7C15))
                    ^ ((transcripts as u64).wrapping_mul(0xBF58476D1CE4E5B9))
                    ^ trial as u64;
                let mut rng = Rng64::new(seed);
                let (recovered, avg_gap) = run_trial(
                    &mut rng,
                    cfg.q,
                    cfg.n,
                    cfg.secret_eta,
                    mask_bound,
                    transcripts,
                );
                recovered_coords += recovered;
                gap += avg_gap;
                if recovered == cfg.n {
                    full += 1;
                }
            }
            let coord_rate = recovered_coords as f64 / (cfg.trials * cfg.n) as f64;
            let full_rate = full as f64 / cfg.trials as f64;
            let verdict = if full_rate >= 0.5 {
                "break"
            } else if coord_rate >= 0.5 {
                "partial-leak"
            } else {
                "inconclusive"
            }
            .to_string();
            rows.push(ProbeRow {
                mask_bound,
                transcripts,
                recovered_trials: full,
                full_recovery_rate: full_rate,
                average_coordinate_recovery_rate: coord_rate,
                average_candidate_gap: gap / cfg.trials as f64,
                verdict,
            });
        }
    }
    ProbeOutput {
        model: "scalar Fiat-Shamir leakage probe: z = y + c*s mod q, with bounded y and small centered s".to_string(),
        q: cfg.q,
        n: cfg.n,
        secret_eta: cfg.secret_eta,
        trials: cfg.trials,
        rows,
        conclusions: vec![
            "Direct linear responses with bounded masks are unsafe when the secret has small coordinate support; each coordinate can be attacked independently by maximum-likelihood enumeration.".to_string(),
            "This probe is not an attack on the KEM/public-view candidate; it is a warning against naive Schnorr/Lyubashevsky-style signing without a masking and rejection proof sized for transcript leakage.".to_string(),
            "Signature work should start from a transcript relation whose responses are statistically independent of the secret, or from a proof system that does not expose linear masked secrets.".to_string(),
        ],
    }
}

fn markdown(out: &ProbeOutput) -> String {
    let mut lines = vec![
        "# Signature Transcript Leakage Probe".to_string(),
        String::new(),
        format!("- model: `{}`", out.model),
        format!("- q: `{}`", out.q),
        format!("- n: `{}`", out.n),
        format!("- secret eta: `{}`", out.secret_eta),
        format!("- trials per row: `{}`", out.trials),
        String::new(),
        "| mask bound | transcripts | full recovery | coordinate recovery | avg score gap | verdict |".to_string(),
        "|---:|---:|---:|---:|---:|---|".to_string(),
    ];
    for row in &out.rows {
        lines.push(format!(
            "| {} | {} | {:.3} | {:.3} | {:.2} | {} |",
            row.mask_bound,
            row.transcripts,
            row.full_recovery_rate,
            row.average_coordinate_recovery_rate,
            row.average_candidate_gap,
            row.verdict
        ));
    }
    lines.extend([String::new(), "## Conclusions".to_string(), String::new()]);
    for conclusion in &out.conclusions {
        lines.push(format!("- {conclusion}"));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn main() -> Result<(), String> {
    let cfg = parse_args()?;
    let out = run_probe(&cfg);
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
    fn small_masks_recover_quickly() {
        let cfg = Config {
            mask_bounds: vec![12],
            transcript_counts: vec![2],
            trials: 8,
            ..Config::default()
        };
        let out = run_probe(&cfg);
        let row = &out.rows[0];
        assert!(row.average_coordinate_recovery_rate > 0.9);
    }

    #[test]
    fn larger_masks_need_more_transcripts() {
        let cfg = Config {
            mask_bounds: vec![1024],
            transcript_counts: vec![1, 32],
            trials: 8,
            ..Config::default()
        };
        let out = run_probe(&cfg);
        assert!(
            out.rows[1].average_coordinate_recovery_rate
                >= out.rows[0].average_coordinate_recovery_rate
        );
    }
}
