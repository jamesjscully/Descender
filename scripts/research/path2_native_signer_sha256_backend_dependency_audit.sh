#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
crate_dir="$repo_root/tools/parameter-search"
lock_file="$repo_root/tools/parameter-search/Cargo.lock"

if [[ ! -f "$lock_file" ]]; then
  echo "parameter-search Cargo.lock missing" >&2
  exit 1
fi

sha2_block="$(
  awk '
    $0 == "[[package]]" { in_pkg = 0 }
    $0 == "name = \"sha2\"" { in_pkg = 1 }
    in_pkg { print }
    in_pkg && $0 == "]" { exit }
  ' "$lock_file"
)"

if [[ -z "$sha2_block" ]]; then
  echo "sha2 package missing from Cargo.lock" >&2
  exit 1
fi

sha2_version="$(awk -F\" '/^version = / { print $2; exit }' <<<"$sha2_block")"
if [[ "$sha2_version" != "0.10.9" ]]; then
  echo "unexpected sha2 version: $sha2_version" >&2
  exit 1
fi

for dep in cfg-if cpufeatures digest; do
  if ! grep -q "\"$dep\"" <<<"$sha2_block"; then
    echo "sha2 dependency $dep missing from lockfile block" >&2
    exit 1
  fi
done

if grep -q 'sha2-asm' "$lock_file"; then
  echo "sha2-asm dependency present; backend timing profile needs separate audit" >&2
  exit 1
fi

feature_tree="$(cd "$crate_dir" && cargo tree -p descender-parameter-search -e features --locked)"
if ! grep -q 'sha2 feature "force-soft"' <<<"$feature_tree"; then
  echo "sha2 force-soft feature is not enabled by active crate" >&2
  exit 1
fi

sha2_block_sha="$(printf '%s' "$sha2_block" | sha256sum | awk '{print $1}')"

printf 'schema=Descender.Path2.NativeSignerSha256BackendDependencyAudit.v1\n'
printf 'lock_file=%s\n' "$lock_file"
printf 'sha2_version=%s\n' "$sha2_version"
printf 'sha2_dep_cfg_if=1\n'
printf 'sha2_dep_cpufeatures=1\n'
printf 'sha2_dep_digest=1\n'
printf 'sha2_asm_dependency_present=0\n'
printf 'sha2_force_soft_feature_enabled=1\n'
printf 'sha2_lock_block_sha256=%s\n' "$sha2_block_sha"
