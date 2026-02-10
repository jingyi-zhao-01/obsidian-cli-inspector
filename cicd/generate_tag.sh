#!/usr/bin/env bash
set -euo pipefail

git fetch --tags --force
LATEST_TAG=$(git describe --abbrev=0 --tags 2>/dev/null || echo "v0.0.0")
VERSION=${LATEST_TAG#v}
IFS='.' read -r MAJOR MINOR PATCH <<< "$VERSION"
MAJOR=${MAJOR:-0}
MINOR=${MINOR:-0}
PATCH=${PATCH:-0}
PATCH=$((PATCH + 1))
NEW_TAG="v${MAJOR}.${MINOR}.${PATCH}"

git tag "$NEW_TAG"

git push origin "$NEW_TAG"

echo "TAG=${NEW_TAG}"
