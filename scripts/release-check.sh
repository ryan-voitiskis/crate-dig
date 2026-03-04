#!/usr/bin/env bash
set -Eeuo pipefail

# Pre-release gate — run before tagging a release.
# Sequential, fail-fast. Exit on first failure.

cd "$(git rev-parse --show-toplevel)"

echo "==> Format check"
dprint check
cargo fmt --check

echo "==> Clippy"
cargo clippy --all-targets -- -D warnings

echo "==> Tests"
cargo test

echo "==> Release build"
cargo build --release

# Broker tests — only if broker/ changed since last tag (or ever, if no tags).
last_tag=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
if [ -n "$last_tag" ]; then
  broker_changed=$(git diff --name-only "$last_tag"..HEAD -- broker/ | head -1)
else
  broker_changed="initial"
fi

if [ -n "$broker_changed" ]; then
  echo "==> Broker tests"
  (cd broker && npm ci && npm test)
else
  echo "==> Broker unchanged since $last_tag — skipping broker tests"
fi

echo "==> All checks passed"
