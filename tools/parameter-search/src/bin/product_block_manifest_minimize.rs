use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Manifest {
    family: String,
    m1: usize,
    group_size: usize,
    #[serde(default)]
    trials_per_point: usize,
    #[serde(default)]
    local_search_steps: usize,
    #[serde(default)]
    seed: u64,
    #[serde(default)]
    q_values: Vec<u64>,
    #[serde(default)]
    target_l1_values: Vec<u64>,
    #[serde(default)]
    eta_values: Vec<u64>,
    #[serde(default)]
    rows_per_group_values: Vec<usize>,
    entries: Vec<Entry>,
    #[serde(default)]
    notes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Entry {
    q: u64,
    target_l1: u64,
    #[serde(default)]
    eta: Option<u64>,
    m1: usize,
    rows_per_group: usize,
    n: usize,
    used_rows: usize,
    keygen_mul_ops: usize,
    passes_target: bool,
    certified_l1: u64,
    selected_trial: usize,
    checked_centered_pair_count: u64,
    sidecar_json: Option<String>,
    validator_args: Option<String>,
}

#[derive(Debug)]
struct Config {
    input_jsons: Vec<PathBuf>,
    output_json: PathBuf,
    output_md: PathBuf,
}

fn parse_args() -> Config {
    let mut args = std::env::args().skip(1);
    let mut cfg = Config {
        input_jsons: Vec::new(),
        output_json: PathBuf::from("research/parameter_search/product_block_lee_sidecars_broad_20260516/manifest_minimal.json"),
        output_md: PathBuf::from("research/parameter_search/product_block_lee_sidecars_broad_20260516/manifest_minimal.md"),
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input-json" => cfg.input_jsons.push(PathBuf::from(args.next().unwrap())),
            "--output-json" => cfg.output_json = PathBuf::from(args.next().unwrap()),
            "--output-md" => cfg.output_md = PathBuf::from(args.next().unwrap()),
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => panic!("unknown argument: {arg}"),
        }
    }
    cfg
}

fn print_help() {
    println!(
        "Product-block sidecar manifest minimizer\n\
         \n\
         Options:\n\
           --input-json PATH\n\
                      Repeat to merge several manifests.\n\
           --output-json PATH\n\
           --output-md PATH"
    );
}

fn entry_key(entry: &Entry) -> (u64, Option<u64>, usize) {
    (entry.q, entry.eta, entry.m1)
}

fn better(a: &Entry, b: &Entry) -> bool {
    (a.rows_per_group, a.used_rows, a.keygen_mul_ops, a.selected_trial)
        < (b.rows_per_group, b.used_rows, b.keygen_mul_ops, b.selected_trial)
}

fn minimize(mut manifest: Manifest) -> Manifest {
    let mut best_by_key: BTreeMap<(u64, Option<u64>, usize), Entry> = BTreeMap::new();
    for entry in manifest.entries.into_iter().filter(|entry| entry.passes_target) {
        let key = entry_key(&entry);
        match best_by_key.get(&key) {
            Some(current) if !better(&entry, current) => {}
            _ => {
                best_by_key.insert(key, entry);
            }
        }
    }
    manifest.entries = best_by_key.into_values().collect();
    manifest.q_values = manifest.entries.iter().map(|entry| entry.q).collect();
    manifest.q_values.sort_unstable();
    manifest.q_values.dedup();
    manifest.eta_values = manifest.entries.iter().filter_map(|entry| entry.eta).collect();
    manifest.eta_values.sort_unstable();
    manifest.eta_values.dedup();
    manifest.target_l1_values = manifest.entries.iter().map(|entry| entry.target_l1).collect();
    manifest.target_l1_values.sort_unstable();
    manifest.target_l1_values.dedup();
    manifest.rows_per_group_values = manifest
        .entries
        .iter()
        .map(|entry| entry.rows_per_group)
        .collect();
    manifest.rows_per_group_values.sort_unstable();
    manifest.rows_per_group_values.dedup();
    manifest.notes.push(
        "Minimized to the cheapest passing sidecar per (q, eta, m1).".to_string(),
    );
    manifest
}

fn merge(manifests: Vec<Manifest>) -> Manifest {
    let mut iter = manifests.into_iter();
    let mut merged = iter.next().expect("at least one input manifest");
    for manifest in iter {
        merged.entries.extend(manifest.entries);
        merged.q_values.extend(manifest.q_values);
        merged.target_l1_values.extend(manifest.target_l1_values);
        merged.eta_values.extend(manifest.eta_values);
        merged
            .rows_per_group_values
            .extend(manifest.rows_per_group_values);
        merged.notes.extend(manifest.notes);
    }
    merged.q_values.sort_unstable();
    merged.q_values.dedup();
    merged.target_l1_values.sort_unstable();
    merged.target_l1_values.dedup();
    merged.eta_values.sort_unstable();
    merged.eta_values.dedup();
    merged.rows_per_group_values.sort_unstable();
    merged.rows_per_group_values.dedup();
    merged
}

fn markdown(manifest: &Manifest) -> String {
    let mut lines = vec![
        "# Minimal Product-Block Lee Sidecar Manifest".to_string(),
        String::new(),
        format!("- family: `{}`", manifest.family),
        format!("- group size: `{}`", manifest.group_size),
        format!("- entries: `{}`", manifest.entries.len()),
        String::new(),
        "| q | eta | target l1 | rows/group | used rows | qamp ops | sidecar |".to_string(),
        "|---:|---:|---:|---:|---:|---:|---|".to_string(),
    ];
    for entry in &manifest.entries {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} |",
            entry.q,
            entry.eta.map_or("-".to_string(), |eta| eta.to_string()),
            entry.target_l1,
            entry.rows_per_group,
            entry.used_rows,
            entry.keygen_mul_ops,
            entry.sidecar_json.as_deref().unwrap_or("-"),
        ));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn main() {
    let mut cfg = parse_args();
    if cfg.input_jsons.is_empty() {
        cfg.input_jsons.push(PathBuf::from(
            "research/parameter_search/product_block_lee_sidecars_broad_20260516/manifest.json",
        ));
    }
    let manifests: Vec<_> = cfg
        .input_jsons
        .iter()
        .map(|path| {
            let text = fs::read_to_string(path).expect("read input manifest");
            serde_json::from_str::<Manifest>(&text).expect("parse input manifest")
        })
        .collect();
    let minimized = minimize(merge(manifests));
    if let Some(parent) = cfg.output_json.parent() {
        fs::create_dir_all(parent).expect("create output json parent");
    }
    if let Some(parent) = cfg.output_md.parent() {
        fs::create_dir_all(parent).expect("create output md parent");
    }
    fs::write(
        &cfg.output_json,
        serde_json::to_string_pretty(&minimized).unwrap() + "\n",
    )
    .expect("write output manifest");
    fs::write(&cfg.output_md, markdown(&minimized)).expect("write markdown");
    println!("wrote {}", cfg.output_json.display());
    println!("wrote {}", cfg.output_md.display());
}
