#!/usr/bin/env bash
set -euo pipefail

# Generate PKGBUILD for AUR -git package
# Usage: generate-pkgbuild-git.sh <pkgname> <repo_owner> <repo_name>

build_pkgbuild_git() {
    local pkgname="$1"
    local repo_owner="$2"
    local repo_name="$3"

    printf '%s\n' \
        "pkgname=${pkgname}-git" \
        "pkgver=0" \
        "pkgrel=1" \
        'pkgdesc="Local-first CLI/TUI for indexing and querying Obsidian vaults (unstable git version)"' \
        "arch=('x86_64')" \
        "url=\"https://github.com/${repo_owner}/${repo_name}\"" \
        "license=('Apache-2.0')" \
        "depends=('sqlite')" \
        "makedepends=('cargo' 'gcc' 'pkgconf' 'git')" \
        "options=('!lto' '!debug')" \
        "source=(\"git+https://github.com/${repo_owner}/${repo_name}.git#branch=master\")" \
        "sha256sums=('SKIP')" \
        '' \
        'pkgver() {' \
        '  cd "${srcdir}/${pkgname%-git}"' \
        '  git describe --long --tags --always | sed -E "s/^v//; s/-([0-9]+)-g/.r\\1.g/; s/-/./g; s/[^0-9A-Za-z.]/./g"' \
        '}' \
        '' \
        'build() {' \
        '  cd "${srcdir}/${pkgname%-git}"' \
        '' \
        '  # Prevent CI or user environment from injecting static Rust flags' \
        '  unset RUSTFLAGS' \
        '  unset CARGO_ENCODED_RUSTFLAGS' \
        '  unset CARGO_NET_OFFLINE' \
        '' \
        '  # Ensure GNU linker is used (avoid ld.lld issues)' \
        '  export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=gcc' \
        '' \
        '  cargo build --release --locked' \
        '}' \
        '' \
        'package() {' \
        '  cd "${srcdir}/${pkgname%-git}"' \
        '  install -Dm755 target/release/obsidian-cli-inspector "${pkgdir}/usr/bin/obsidian-cli-inspector"' \
        '}'
}

# Main execution
if [[ $# -ne 3 ]]; then
    echo "Usage: $0 <pkgname> <repo_owner> <repo_name>" >&2
    exit 1
fi

build_pkgbuild_git "$@"
