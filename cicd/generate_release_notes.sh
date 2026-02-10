#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-}"
TAG="${2:-}"

if [[ -z "$VERSION" || -z "$TAG" ]]; then
  echo "Usage: $0 <version> <tag>" >&2
  exit 1
fi

PREVIOUS_TAG=$(git describe --abbrev=0 --tags $(git rev-list --tags --skip=1 --max-count=1) 2>/dev/null || echo "")

if [[ -z "$PREVIOUS_TAG" ]]; then
  CHANGELOG=$(git log --pretty=format:"- %s (%h)" --no-merges)
else
  CHANGELOG=$(git log ${PREVIOUS_TAG}..HEAD --pretty=format:"- %s (%h)" --no-merges)
fi

cat << EOF
## obsidian-cli-inspector v${VERSION}

### What's Changed

${CHANGELOG}

### Installation

#### From crates.io
\`\`\`bash
cargo install obsidian-cli-inspector
\`\`\`

#### From binary
Download the appropriate binary for your platform from the assets below and add it to your PATH.

### Full Changelog
${PREVIOUS_TAG:+https://github.com/${GITHUB_REPOSITORY:-jingyi-zhao-01/obsidian-cli}/compare/${PREVIOUS_TAG}...${TAG}}
EOF
