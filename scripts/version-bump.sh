#!/usr/bin/env bash
# Bump the project version. cyrius.cyml pulls the version via
# ${file:VERSION} interpolation, so VERSION is the single source
# of truth — no manifest edit needed. CHANGELOG.md still requires
# a manual section header for the new version.
set -euo pipefail

[ $# -ne 1 ] && { echo "Usage: $0 <semver>"; exit 1; }
NEW_VERSION="$1"

echo "$NEW_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$' || {
  echo "ERROR: '$NEW_VERSION' is not semver x.y.z"; exit 1; }

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "$NEW_VERSION" > "$REPO_ROOT/VERSION"

if ! grep -q '^version = "${file:VERSION}"' "$REPO_ROOT/cyrius.cyml"; then
  echo "::warning:: cyrius.cyml does not use \${file:VERSION} — manifest version may drift"
fi

# ⭐ 2.19.0 — THERE IS NOTHING TO REWRITE HERE ANY MORE, AND THAT IS THE POINT.
# src/main.cyr USED TO hardcode the string the CLI prints — the one version site
# ${file:VERSION} could not reach, so it was a manual edit this script did not
# make and no gate checked, and the 2.9.1 -> 2.9.2 bump left the CLI reporting
# 2.9.1. This script then grew a sed for it and CI grew a grep.
# Since 2.19.0 it prints CYRIUS_PKG_VERSION, which `cyrius build` declares from
# [package].version -- that is, from ${file:VERSION} -- so the second copy does
# not exist and drift is impossible rather than merely automated.
# ⚠ THE OLD WORDING REMOVED FROM HERE CLAIMED "Cyrius has no build-time string
# interpolation". True when written (2026-08-09); FALSE FOUR DAYS LATER, when
# cyrius 6.5.21 (2026-08-13) shipped CYRIUS_PKG_VERSION, and it stood for a
# further four weeks. What stays true is only the narrower claim it was confused
# with: ${file:VERSION} is a MANIFEST-side expansion and cannot itself reach a
# .cyr string literal.
# This block now VERIFIES rather than rewrites; CI's "Verify printed version"
# step is the real gate, and it runs the built binary.
if grep -v '^[[:space:]]*#' "$REPO_ROOT/src/main.cyr" | grep -q 'CYRIUS_PKG_VERSION'; then
  echo "src/main.cyr: prints CYRIUS_PKG_VERSION (no rewrite needed)"
else
  echo "::warning:: src/main.cyr does not print CYRIUS_PKG_VERSION — the CLI version may drift"
fi
if grep -v '^[[:space:]]*#' "$REPO_ROOT/src/main.cyr" | grep -qE '"hisab [0-9]+\.[0-9]+\.[0-9]+"'; then
  echo "::warning:: src/main.cyr has a hardcoded version literal again — it will go stale"
fi

echo "VERSION: $NEW_VERSION"
echo "Next:"
echo "  1) Add a '## [${NEW_VERSION}] - $(date -u +%Y-%m-%d)' section in CHANGELOG.md"
echo "  2) Run 'cyrius distlib' — dist/hisab.cyr embeds the version in its header"
echo "  3) git commit, tag '$NEW_VERSION' (or 'v${NEW_VERSION}'), push tag"
