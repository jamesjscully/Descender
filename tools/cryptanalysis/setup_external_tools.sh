#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VENV="${ROOT}/.venv-cryptanalysis"
EXT="${ROOT}/tools/external"

mkdir -p "${EXT}"

if python3 -m venv "${VENV}" 2>/tmp/descender-venv-error.log; then
  # shellcheck source=/dev/null
  source "${VENV}/bin/activate"
  python -m pip install --upgrade pip wheel

  echo "[1/4] Installing fpylll wrapper when wheels are available"
  python -m pip install fpylll || {
    echo "warning: fpylll install failed. Install system fplll/fpylll manually or use the emitted fplll basis files with the fplll CLI." >&2
  }

  echo "[2/4] Installing CryptographicEstimators when available"
  python -m pip install cryptographic-estimators || {
    echo "warning: cryptographic-estimators install failed. See https://github.com/Crypto-TII/CryptographicEstimators" >&2
  }
else
  cat /tmp/descender-venv-error.log >&2
  cat >&2 <<'EOF'
warning: Python venv setup failed. On Debian/Ubuntu this usually means
python3-venv is missing. Install it, or prepare fpylll/CryptographicEstimators
in another environment. The repository checkout steps below will still run.
EOF
fi

echo "[3/4] Cloning lattice-estimator"
if [ ! -d "${EXT}/lattice-estimator/.git" ]; then
  git clone https://github.com/malb/lattice-estimator "${EXT}/lattice-estimator"
else
  git -C "${EXT}/lattice-estimator" pull --ff-only || true
fi

echo "[4/4] Cloning G6K source for later heavy experiments"
if [ ! -d "${EXT}/g6k/.git" ]; then
  git clone https://github.com/fplll/g6k "${EXT}/g6k"
else
  git -C "${EXT}/g6k" pull --ff-only || true
fi

cat <<EOF

External tool setup attempted.

Activate with:
  source ${VENV}/bin/activate

Then run generated Descender jobs, for example:
  python research/cryptanalysis/<run>/run_fpylll_jobs.py

lattice-estimator often expects a Sage-compatible environment. If importing
\`estimator\` fails, use ${EXT}/lattice-estimator as the checkout to wire into
Sage/Python manually.
EOF
