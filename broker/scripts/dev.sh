#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
npm run d1:migrate:local
npm run dev
