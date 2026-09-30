#!/usr/bin/env bash
# What "checked" means for this repo (xcp-hl#148). Jenkins dev/xoa-deploy-patcher runs it on every PR; run it locally too.
# Contract: exit 0 when clean; JUnit goes to $CI_RESULTS; on failure $CI_RESULTS/current-step names the failed check.
set -euo pipefail
cd "$(dirname "$0")/.."

OUT="${CI_RESULTS:-ci-results}"
mkdir -p "$OUT"
step() { echo "==> $1"; echo "$1" > "$OUT/current-step"; }
trap 'cp target/nextest/ci/junit.xml "$OUT/junit.xml" 2>/dev/null || true' EXIT

step "cargo fmt --check"
cargo fmt --all -- --check

step "cargo clippy"
cargo clippy --locked --all-targets -- -D warnings

# No tests yet (follow-up on xcp-hl#148): reported, not failed, until the first one lands.
step "cargo nextest"
cargo nextest run --locked --profile ci --no-tests=warn

rm -f "$OUT/current-step"
echo "all checks passed"
