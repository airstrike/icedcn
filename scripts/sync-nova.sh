#!/usr/bin/env bash
# Sync the local mirror of upstream nova source into .nova/ (gitignored).
#
# Idempotent — first run does a sparse clone, subsequent runs pull updates.
# Paths mirrored (see CLAUDE.md "Local nova source mirror"):
#
#   .nova/apps/v4/styles/radix-nova/ui/        — nova radix component impls
#   .nova/apps/v4/examples/radix/              — example .tsx sources
#   .nova/apps/v4/registry/styles/             — style-nova.css
#   .nova/apps/v4/registry/new-york-v4/ui/     — base new-york-v4 (cross-ref)
#   .nova/apps/v4/app/                         — docs-site Next app (sidebar / nav overrides)
#   .nova/apps/v4/components/                  — docs-site shared components
#   .nova/apps/v4/lib/                         — docs-site helpers (highlight, etc.)
#   .nova/apps/v4/hooks/                       — docs-site hooks

set -euo pipefail

cd "$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"

NOVA_DIR=".nova"
SPARSE_PATHS=(
    "apps/v4/styles/radix-nova/ui"
    "apps/v4/examples/radix"
    "apps/v4/registry/styles"
    "apps/v4/registry/new-york-v4/ui"
    "apps/v4/app"
    "apps/v4/components"
    "apps/v4/lib"
    "apps/v4/hooks"
)

if [ -d "$NOVA_DIR/.git" ]; then
    echo "Updating $NOVA_DIR (sparse pull) ..."
    # Reapply sparse-checkout in case the path list changed since the last pull.
    git -C "$NOVA_DIR" sparse-checkout set "${SPARSE_PATHS[@]}"
    git -C "$NOVA_DIR" pull --quiet --depth=1 --ff-only
else
    echo "Cloning upstream (sparse) into $NOVA_DIR ..."
    # Resolve the reference repository by its permanent GitHub ID.
    NOVA_REPO=$(curl -fsSL https://api.github.com/repositories/585146387 |
        python3 -c 'import json, sys; print(json.load(sys.stdin)["clone_url"])')
    git clone --quiet --depth=1 --filter=blob:none --sparse \
        "$NOVA_REPO" "$NOVA_DIR"
    git -C "$NOVA_DIR" sparse-checkout set "${SPARSE_PATHS[@]}"
fi

sha="$(git -C "$NOVA_DIR" rev-parse --short HEAD)"
branch="$(git -C "$NOVA_DIR" rev-parse --abbrev-ref HEAD)"
echo "Pinned to ${branch}@${sha}"
