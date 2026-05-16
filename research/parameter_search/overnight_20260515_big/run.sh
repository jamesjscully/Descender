#!/usr/bin/env bash
set -uo pipefail
ROOT=/home/jack/code/Descender
cd "$ROOT"
OUT=research/parameter_search/overnight_20260515_big
BIN=tools/parameter-search/target/release/descender-parameter-search
VAL=tools/parameter-search/target/release/validate_frontier
SHARED_LEE_CERT=${SHARED_LEE_CERT:-research/cryptanalysis/n32_m1_32_structure/shared_lee_global_q32771_target3277.json}
SHARED_LEE_CERT_DIR=${SHARED_LEE_CERT_DIR:-research/parameter_search/product_block_lee_sidecars_broad_20260516}
PRODUCT_BLOCK_MANIFEST=${PRODUCT_BLOCK_MANIFEST:-research/parameter_search/product_block_lee_sidecars_combined_20260516/manifest_minimal.json}
THREADS=12
ITER=350000
RESTARTS=768
TOP=1200
Q_VALUES=12289,40961,65537,131071,262147,524309,1048583
N_VALUES=24,28,32,36,40,48,56,64,72,80,96,112,128,160,192,224,256,320,384,512
M0_VALUES=24,28,32,36,40,44,48,56,64,72,80,88,96,112,128,160,192,224,256,320,384,512
M1_MULT=1,2,3,4,5,6,8,10,12
FIBERS=2,3,4,5,6,8,10,12,16,24,32,48,64
ETAS=1,2,3,4,5,6,8,10,12,16
SAMPLE_BITS=0,10,11,12,13,14,15,16,17,18,19,20
VIEWS=0,24,28,32,36,40,44,48,56,64,72,80,88,96,112,128,160,192,224,256,320,384,512
mkdir -p "$OUT"
printf 'started %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee "$OUT/status.log"
printf 'threads=%s iterations=%s restarts=%s top=%s\n' "$THREADS" "$ITER" "$RESTARTS" "$TOP" | tee -a "$OUT/status.log"
SHARED_LEE_CERT_ARGS=()
if [ -d "$SHARED_LEE_CERT_DIR" ]; then
  SHARED_LEE_CERT_ARGS+=(--shared-lee-cert-dir "$SHARED_LEE_CERT_DIR")
  printf 'shared_lee_cert_dir=%s\n' "$SHARED_LEE_CERT_DIR" | tee -a "$OUT/status.log"
else
  printf 'shared_lee_cert_dir=missing:%s\n' "$SHARED_LEE_CERT_DIR" | tee -a "$OUT/status.log"
fi
if [ -f "$SHARED_LEE_CERT" ]; then
  SHARED_LEE_CERT_ARGS+=(--shared-lee-cert-json "$SHARED_LEE_CERT")
  printf 'shared_lee_cert=%s\n' "$SHARED_LEE_CERT" | tee -a "$OUT/status.log"
else
  printf 'shared_lee_cert=missing:%s\n' "$SHARED_LEE_CERT" | tee -a "$OUT/status.log"
fi
PRODUCT_BLOCK_MANIFEST_ARGS=()
if [ -f "$PRODUCT_BLOCK_MANIFEST" ]; then
  PRODUCT_BLOCK_MANIFEST_ARGS=(--product-block-sidecar-manifest "$PRODUCT_BLOCK_MANIFEST")
  SHARED_LEE_CERT_ARGS+=(--shared-lee-cert-manifest "$PRODUCT_BLOCK_MANIFEST")
  printf 'product_block_manifest=%s\n' "$PRODUCT_BLOCK_MANIFEST" | tee -a "$OUT/status.log"
else
  printf 'product_block_manifest=missing:%s\n' "$PRODUCT_BLOCK_MANIFEST" | tee -a "$OUT/status.log"
fi
for PROFILE in descender-128 descender-192 descender-256; do
  printf '\n=== compressed lossy/diagonal %s %s ===\n' "$PROFILE" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee -a "$OUT/status.log"
  timeout 3h "$BIN" \
    --mode anneal \
    --objective bandwidth \
    --public-key-model canonical \
    --coefficient-model seeded \
    --incidence-model seeded \
    --profile "$PROFILE" \
    --output-dir "$OUT/compressed_$PROFILE" \
    --q-values "$Q_VALUES" \
    --n-values "$N_VALUES" \
    --m0-values "$M0_VALUES" \
    --m1-multipliers "$M1_MULT" \
    --fiber-dims "$FIBERS" \
    --eta-values "$ETAS" \
    --sample-bits "$SAMPLE_BITS" \
    --view-coords "$VIEWS" \
    --proof-strategies full-surjective,support-covered,lossy-budget \
    --primal-distance-models guard-only,combined-proxy \
    --security-extra-cap 24 \
    --guard-slack-target 4 \
    --combined-certification-weight 0.75 \
    --anneal-restarts "$RESTARTS" \
    --anneal-iterations "$ITER" \
    --threads "$THREADS" \
    --top "$TOP" \
    --seed 20260515 \
    > "$OUT/compressed_$PROFILE.log" 2>&1
  RC=$?
  printf 'compressed %s rc=%s %s\n' "$PROFILE" "$RC" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee -a "$OUT/status.log"
  if [ -f "$OUT/compressed_$PROFILE/$PROFILE.json" ]; then
    timeout 90m "$VAL" --input-dir "$OUT/compressed_$PROFILE" --output-dir "$OUT/compressed_${PROFILE}_validation" --top 300 --dual-search-weight 3 "${SHARED_LEE_CERT_ARGS[@]}" > "$OUT/compressed_${PROFILE}_validation.log" 2>&1
    printf 'validated compressed %s rc=%s %s\n' "$PROFILE" "$?" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee -a "$OUT/status.log"
  fi

  printf '\n=== exact/full-view validated lane %s %s ===\n' "$PROFILE" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee -a "$OUT/status.log"
  timeout 90m "$BIN" \
    --mode anneal \
    --objective bandwidth \
    --public-key-model canonical \
    --coefficient-model seeded \
    --incidence-model seeded \
    --profile "$PROFILE" \
    --output-dir "$OUT/exact_$PROFILE" \
    --q-values "$Q_VALUES" \
    --n-values "$N_VALUES" \
    --m0-values "$M0_VALUES" \
    --m1-multipliers "$M1_MULT" \
    --fiber-dims "$FIBERS" \
    --eta-values "$ETAS" \
    --sample-bits 0 \
    --view-coords 0 \
    --proof-strategies full-surjective \
    --primal-distance-models guard-only \
    --security-extra-cap 24 \
    --guard-slack-target 4 \
    --anneal-restarts 384 \
    --anneal-iterations 250000 \
    --threads "$THREADS" \
    --top 800 \
    --seed 20260516 \
    > "$OUT/exact_$PROFILE.log" 2>&1
  RC=$?
  printf 'exact %s rc=%s %s\n' "$PROFILE" "$RC" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee -a "$OUT/status.log"
  if [ -f "$OUT/exact_$PROFILE/$PROFILE.json" ]; then
    timeout 90m "$VAL" --input-dir "$OUT/exact_$PROFILE" --output-dir "$OUT/exact_${PROFILE}_validation" --top 300 --dual-search-weight 3 "${SHARED_LEE_CERT_ARGS[@]}" > "$OUT/exact_${PROFILE}_validation.log" 2>&1
    printf 'validated exact %s rc=%s %s\n' "$PROFILE" "$?" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee -a "$OUT/status.log"
  fi

  if [ ${#PRODUCT_BLOCK_MANIFEST_ARGS[@]} -gt 0 ]; then
    printf '\n=== certified product-block lane %s %s ===\n' "$PROFILE" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee -a "$OUT/status.log"
    timeout 90m "$BIN" \
      --mode grid \
      --objective bandwidth \
      --public-key-model component \
      --coefficient-model seeded \
      --incidence-model seeded \
      --profile "$PROFILE" \
      --output-dir "$OUT/product_block_$PROFILE" \
      --q-values "$Q_VALUES" \
      --n-values "$N_VALUES" \
      --m0-values "$M0_VALUES" \
      --m1-values 32 \
      --fiber-dims "$FIBERS" \
      --eta-values "$ETAS" \
      --sample-bits 0 \
      --view-coords 0 \
      --proof-strategies full-surjective \
      --primal-distance-models guard-only \
      --qary-amplitude-strategies product-block-lee \
      "${PRODUCT_BLOCK_MANIFEST_ARGS[@]}" \
      --security-extra-cap 24 \
      --guard-slack-target 4 \
      --threads "$THREADS" \
      --top 800 \
      > "$OUT/product_block_$PROFILE.log" 2>&1
    RC=$?
    printf 'product_block %s rc=%s %s\n' "$PROFILE" "$RC" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee -a "$OUT/status.log"
    if [ -f "$OUT/product_block_$PROFILE/$PROFILE.json" ]; then
      timeout 90m "$VAL" --input-dir "$OUT/product_block_$PROFILE" --output-dir "$OUT/product_block_${PROFILE}_validation" --top 300 --dual-search-weight 3 "${SHARED_LEE_CERT_ARGS[@]}" > "$OUT/product_block_${PROFILE}_validation.log" 2>&1
      printf 'validated product_block %s rc=%s %s\n' "$PROFILE" "$?" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee -a "$OUT/status.log"
    fi
  fi
done
printf '\nfinished %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee -a "$OUT/status.log"
