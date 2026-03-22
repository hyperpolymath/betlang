#!/usr/bin/env bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# Create a release with all platform binaries

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
VERSION="${VERSION:-$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | head -1 | sed 's/.*"\(.*\)"/\1/')}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $*"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

# Targets for cross-compilation
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-gnu"
    "aarch64-unknown-linux-musl"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-msvc"
    "riscv64gc-unknown-linux-gnu"
)

# Build for a specific target
build_target() {
    local target="$1"
    info "Building for $target..."

    # Install target if needed
    if ! rustup target list --installed | grep -q "$target"; then
        rustup target add "$target" || {
            warn "Could not add target $target, skipping"
            return 1
        }
    fi

    # Use cross if available for cross-compilation
    if command -v cross &>/dev/null; then
        cross build --release --target "$target"
    else
        cargo build --release --target "$target"
    fi

    return 0
}

# Package a target
package_target() {
    local target="$1"
    local release_dir="$PROJECT_ROOT/release"
    local pkg_name="betlang-${VERSION}-${target}"
    local pkg_dir="$release_dir/$pkg_name"

    mkdir -p "$pkg_dir"

    # Determine binary extension
    local ext=""
    if [[ "$target" == *"windows"* ]]; then
        ext=".exe"
    fi

    # Copy binaries
    local target_dir="$PROJECT_ROOT/target/$target/release"
    if [[ -f "$target_dir/bet-cli$ext" ]]; then
        cp "$target_dir/bet-cli$ext" "$pkg_dir/"
    else
        warn "Binary not found for $target"
        return 1
    fi

    # Copy docs
    cp "$PROJECT_ROOT/README.adoc" "$pkg_dir/"
    cp "$PROJECT_ROOT/LICENSE.txt" "$pkg_dir/"
    cp "$PROJECT_ROOT/CHANGELOG.md" "$pkg_dir/"

    # Create archive
    cd "$release_dir"
    if [[ "$target" == *"windows"* ]]; then
        zip -r "${pkg_name}.zip" "$pkg_name"
    else
        tar -czvf "${pkg_name}.tar.gz" "$pkg_name"
    fi

    rm -rf "$pkg_dir"
    info "Created: $release_dir/${pkg_name}.tar.gz"
}

# Generate checksums
generate_checksums() {
    local release_dir="$PROJECT_ROOT/release"
    cd "$release_dir"

    sha256sum *.tar.gz *.zip 2>/dev/null > "SHA256SUMS.txt" || true
    info "Generated checksums: $release_dir/SHA256SUMS.txt"
}

# Main
main() {
    local release_dir="$PROJECT_ROOT/release"
    mkdir -p "$release_dir"

    info "Building Betlang v$VERSION for release..."

    for target in "${TARGETS[@]}"; do
        if build_target "$target"; then
            package_target "$target"
        fi
    done

    generate_checksums

    info "Release packages created in: $release_dir"
}

main "$@"
