#!/usr/bin/env bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# Build all betlang packages

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
VERSION="${VERSION:-0.1.0}"
RELEASE="${RELEASE:-1}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $*"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

# Detect platform
detect_platform() {
    case "$(uname -s)" in
        Linux*)  echo "linux" ;;
        Darwin*) echo "macos" ;;
        MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
        *) echo "unknown" ;;
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64) echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        armv7*) echo "armv7" ;;
        riscv64) echo "riscv64" ;;
        *) echo "$(uname -m)" ;;
    esac
}

# Build Rust binaries
build_rust() {
    local target="${1:-}"
    info "Building Rust binaries${target:+ for $target}..."

    cd "$PROJECT_ROOT"

    if [[ -n "$target" ]]; then
        cargo build --release --target "$target"
    else
        cargo build --release
    fi
}

# Build Gleam LSP
build_lsp() {
    info "Building Gleam LSP server..."

    cd "$PROJECT_ROOT/lsp/bet-lsp"

    if command -v gleam &>/dev/null; then
        gleam build --target erlang
        gleam export erlang-shipment
    else
        warn "Gleam not found, skipping LSP build"
    fi
}

# Package for Debian/Ubuntu
build_deb() {
    info "Building Debian package..."

    local pkg_dir="$PROJECT_ROOT/packaging/deb/betlang_${VERSION}-${RELEASE}_$(detect_arch)"
    rm -rf "$pkg_dir"
    mkdir -p "$pkg_dir/DEBIAN"
    mkdir -p "$pkg_dir/usr/bin"
    mkdir -p "$pkg_dir/usr/lib/betlang"
    mkdir -p "$pkg_dir/usr/share/betlang"
    mkdir -p "$pkg_dir/usr/share/doc/betlang"
    mkdir -p "$pkg_dir/usr/share/man/man1"

    # Control file
    cat > "$pkg_dir/DEBIAN/control" << EOF
Package: betlang
Version: ${VERSION}-${RELEASE}
Section: devel
Priority: optional
Architecture: $(detect_arch | sed 's/x86_64/amd64/')
Depends: erlang-base (>= 24.0), racket (>= 8.0)
Maintainer: Hyperpolymath <support@hyperpolymath.com>
Description: Ternary probabilistic programming language
 Betlang is a domain-specific language for probabilistic modeling
 and symbolic wagers. The core primitive is the bet { A, B, C }
 form which randomly selects one of three values.
Homepage: https://github.com/hyperpolymath/betlang
EOF

    # Copy binaries
    if [[ -f "$PROJECT_ROOT/target/release/bet-cli" ]]; then
        cp "$PROJECT_ROOT/target/release/bet-cli" "$pkg_dir/usr/bin/"
    fi

    # Copy LSP
    if [[ -d "$PROJECT_ROOT/lsp/bet-lsp/build/erlang-shipment" ]]; then
        cp -r "$PROJECT_ROOT/lsp/bet-lsp/build/erlang-shipment" "$pkg_dir/usr/lib/betlang/lsp"
        cat > "$pkg_dir/usr/bin/bet-lsp" << 'SCRIPT'
#!/bin/sh
exec /usr/lib/betlang/lsp/entrypoint.sh "$@"
SCRIPT
        chmod +x "$pkg_dir/usr/bin/bet-lsp"
    fi

    # Copy core files
    cp -r "$PROJECT_ROOT/core" "$pkg_dir/usr/share/betlang/"
    cp -r "$PROJECT_ROOT/lib" "$pkg_dir/usr/share/betlang/"
    cp -r "$PROJECT_ROOT/stdlib" "$pkg_dir/usr/share/betlang/" 2>/dev/null || true

    # Documentation
    cp "$PROJECT_ROOT/README.adoc" "$pkg_dir/usr/share/doc/betlang/"
    cp "$PROJECT_ROOT/LICENSE.txt" "$pkg_dir/usr/share/doc/betlang/"

    # Build package
    dpkg-deb --build "$pkg_dir"
    info "Created: ${pkg_dir}.deb"
}

# Package for RPM (Fedora/RHEL/openSUSE)
build_rpm() {
    info "Building RPM package..."

    local rpm_dir="$PROJECT_ROOT/packaging/rpm"
    mkdir -p "$rpm_dir"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

    cat > "$rpm_dir/SPECS/betlang.spec" << EOF
Name:           betlang
Version:        ${VERSION}
Release:        ${RELEASE}%{?dist}
Summary:        Ternary probabilistic programming language
License:        MIT OR Apache-2.0
URL:            https://github.com/hyperpolymath/betlang
Source0:        betlang-${VERSION}.tar.gz

BuildRequires:  cargo, rust, erlang, gleam
Requires:       erlang >= 24.0, racket >= 8.0

%description
Betlang is a domain-specific language for probabilistic modeling
and symbolic wagers. The core primitive is the bet { A, B, C }
form which randomly selects one of three values.

%prep
%setup -q

%build
cargo build --release

%install
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/lib/betlang
mkdir -p %{buildroot}/usr/share/betlang
install -m 755 target/release/bet-cli %{buildroot}/usr/bin/
cp -r core lib %{buildroot}/usr/share/betlang/

%files
/usr/bin/bet-cli
/usr/lib/betlang
/usr/share/betlang

%changelog
* $(date '+%a %b %d %Y') Hyperpolymath <support@hyperpolymath.com> - ${VERSION}-${RELEASE}
- Initial package
EOF

    info "Created: $rpm_dir/SPECS/betlang.spec"
}

# Package for Homebrew (macOS/Linux)
build_homebrew() {
    info "Building Homebrew formula..."

    cat > "$PROJECT_ROOT/packaging/homebrew/betlang.rb" << 'EOF'
# SPDX-License-Identifier: MIT OR Apache-2.0
class Betlang < Formula
  desc "Ternary probabilistic programming language"
  homepage "https://github.com/hyperpolymath/betlang"
  url "https://github.com/hyperpolymath/betlang/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_SHA256"
  license any_of: ["MIT", "Apache-2.0"]
  head "https://github.com/hyperpolymath/betlang.git", branch: "main"

  depends_on "rust" => :build
  depends_on "gleam" => :build
  depends_on "erlang"
  depends_on "racket"

  def install
    system "cargo", "install", *std_cargo_args(path: "tools/bet-cli")

    # Build LSP
    cd "lsp/bet-lsp" do
      system "gleam", "build", "--target", "erlang"
      system "gleam", "export", "erlang-shipment"
      libexec.install "build/erlang-shipment"
    end

    # Create wrapper script for LSP
    (bin/"bet-lsp").write <<~EOS
      #!/bin/bash
      exec "#{libexec}/erlang-shipment/entrypoint.sh" "$@"
    EOS

    # Install core files
    share.install "core", "lib"
  end

  test do
    assert_match "betlang", shell_output("#{bin}/bet-cli --version")
  end
end
EOF

    info "Created: $PROJECT_ROOT/packaging/homebrew/betlang.rb"
}

# Package for AUR (Arch Linux)
build_aur() {
    info "Building AUR PKGBUILD..."

    cat > "$PROJECT_ROOT/packaging/aur/PKGBUILD" << 'EOF'
# SPDX-License-Identifier: MIT OR Apache-2.0
# Maintainer: Hyperpolymath <support@hyperpolymath.com>

pkgname=betlang
pkgver=0.1.0
pkgrel=1
pkgdesc="Ternary probabilistic programming language"
arch=('x86_64' 'aarch64' 'riscv64')
url="https://github.com/hyperpolymath/betlang"
license=('MIT' 'Apache-2.0')
depends=('erlang' 'racket')
makedepends=('cargo' 'rust' 'gleam')
source=("$pkgname-$pkgver.tar.gz::https://github.com/hyperpolymath/$pkgname/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"

    # Build CLI
    cargo build --release

    # Build LSP
    cd lsp/bet-lsp
    gleam build --target erlang
    gleam export erlang-shipment
}

package() {
    cd "$pkgname-$pkgver"

    # Install CLI
    install -Dm755 target/release/bet-cli "$pkgdir/usr/bin/bet-cli"

    # Install LSP
    install -dm755 "$pkgdir/usr/lib/betlang/lsp"
    cp -r lsp/bet-lsp/build/erlang-shipment/* "$pkgdir/usr/lib/betlang/lsp/"

    # Create wrapper script
    cat > "$pkgdir/usr/bin/bet-lsp" << 'SCRIPT'
#!/bin/sh
exec /usr/lib/betlang/lsp/entrypoint.sh "$@"
SCRIPT
    chmod 755 "$pkgdir/usr/bin/bet-lsp"

    # Install core files
    install -dm755 "$pkgdir/usr/share/betlang"
    cp -r core lib "$pkgdir/usr/share/betlang/"

    # Install license
    install -Dm644 LICENSE.txt "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
EOF

    # Create .SRCINFO
    cat > "$PROJECT_ROOT/packaging/aur/.SRCINFO" << 'EOF'
pkgbase = betlang
	pkgdesc = Ternary probabilistic programming language
	pkgver = 0.1.0
	pkgrel = 1
	url = https://github.com/hyperpolymath/betlang
	arch = x86_64
	arch = aarch64
	arch = riscv64
	license = MIT
	license = Apache-2.0
	makedepends = cargo
	makedepends = rust
	makedepends = gleam
	depends = erlang
	depends = racket
	source = betlang-0.1.0.tar.gz::https://github.com/hyperpolymath/betlang/archive/v0.1.0.tar.gz

pkgname = betlang
EOF

    info "Created: $PROJECT_ROOT/packaging/aur/PKGBUILD"
}

# Package for Flatpak
build_flatpak() {
    info "Building Flatpak manifest..."

    cat > "$PROJECT_ROOT/packaging/flatpak/com.hyperpolymath.betlang.yml" << 'EOF'
# SPDX-License-Identifier: MIT OR Apache-2.0
app-id: com.hyperpolymath.betlang
runtime: org.freedesktop.Platform
runtime-version: '23.08'
sdk: org.freedesktop.Sdk
sdk-extensions:
  - org.freedesktop.Sdk.Extension.rust-stable
command: bet-cli

finish-args:
  - --share=network
  - --filesystem=home

modules:
  - name: erlang
    buildsystem: autotools
    sources:
      - type: archive
        url: https://github.com/erlang/otp/releases/download/OTP-26.0/otp_src_26.0.tar.gz
        sha256: REPLACE_WITH_SHA256

  - name: racket
    buildsystem: simple
    build-commands:
      - ./configure --prefix=/app
      - make -j$FLATPAK_BUILDER_N_JOBS
      - make install
    sources:
      - type: archive
        url: https://download.racket-lang.org/installers/8.11/racket-8.11-src.tgz
        sha256: REPLACE_WITH_SHA256

  - name: betlang
    buildsystem: simple
    build-options:
      append-path: /usr/lib/sdk/rust-stable/bin
    build-commands:
      - cargo build --release
      - install -Dm755 target/release/bet-cli /app/bin/bet-cli
      - mkdir -p /app/share/betlang
      - cp -r core lib /app/share/betlang/
    sources:
      - type: git
        url: https://github.com/hyperpolymath/betlang.git
        tag: v0.1.0
EOF

    info "Created: $PROJECT_ROOT/packaging/flatpak/com.hyperpolymath.betlang.yml"
}

# Package for Windows (Scoop/Chocolatey/winget)
build_windows() {
    info "Building Windows package manifests..."

    # Scoop manifest
    cat > "$PROJECT_ROOT/packaging/windows/betlang.json" << 'EOF'
{
    "version": "0.1.0",
    "description": "Ternary probabilistic programming language",
    "homepage": "https://github.com/hyperpolymath/betlang",
    "license": {
        "identifier": "MIT OR Apache-2.0",
        "url": "https://github.com/hyperpolymath/betlang/blob/main/LICENSE.txt"
    },
    "architecture": {
        "64bit": {
            "url": "https://github.com/hyperpolymath/betlang/releases/download/v0.1.0/betlang-0.1.0-x86_64-pc-windows-msvc.zip",
            "hash": "REPLACE_WITH_SHA256"
        }
    },
    "bin": [
        "bet-cli.exe",
        "bet-lsp.exe"
    ],
    "checkver": "github",
    "autoupdate": {
        "architecture": {
            "64bit": {
                "url": "https://github.com/hyperpolymath/betlang/releases/download/v$version/betlang-$version-x86_64-pc-windows-msvc.zip"
            }
        }
    }
}
EOF

    # Chocolatey nuspec
    cat > "$PROJECT_ROOT/packaging/windows/betlang.nuspec" << 'EOF'
<?xml version="1.0" encoding="utf-8"?>
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
<package xmlns="http://schemas.microsoft.com/packaging/2015/06/nuspec.xsd">
  <metadata>
    <id>betlang</id>
    <version>0.1.0</version>
    <title>Betlang</title>
    <authors>Hyperpolymath</authors>
    <owners>hyperpolymath</owners>
    <projectUrl>https://github.com/hyperpolymath/betlang</projectUrl>
    <licenseUrl>https://github.com/hyperpolymath/betlang/blob/main/LICENSE.txt</licenseUrl>
    <requireLicenseAcceptance>false</requireLicenseAcceptance>
    <description>Ternary probabilistic programming language for probabilistic modeling and symbolic wagers.</description>
    <summary>A DSL for probabilistic programming with ternary choice primitives</summary>
    <tags>programming language probabilistic dsl</tags>
  </metadata>
  <files>
    <file src="tools\**" target="tools" />
  </files>
</package>
EOF

    # winget manifest
    mkdir -p "$PROJECT_ROOT/packaging/windows/winget"
    cat > "$PROJECT_ROOT/packaging/windows/winget/Hyperpolymath.Betlang.yaml" << 'EOF'
# SPDX-License-Identifier: MIT OR Apache-2.0
PackageIdentifier: Hyperpolymath.Betlang
PackageVersion: 0.1.0
PackageLocale: en-US
Publisher: Hyperpolymath
PublisherUrl: https://github.com/hyperpolymath
PackageName: Betlang
PackageUrl: https://github.com/hyperpolymath/betlang
License: MIT OR Apache-2.0
LicenseUrl: https://github.com/hyperpolymath/betlang/blob/main/LICENSE.txt
ShortDescription: Ternary probabilistic programming language
Description: Betlang is a domain-specific language for probabilistic modeling and symbolic wagers with a bet { A, B, C } primitive.
Moniker: betlang
Tags:
  - programming-language
  - probabilistic
  - dsl
Installers:
  - Architecture: x64
    InstallerType: zip
    InstallerUrl: https://github.com/hyperpolymath/betlang/releases/download/v0.1.0/betlang-0.1.0-x86_64-pc-windows-msvc.zip
    InstallerSha256: REPLACE_WITH_SHA256
ManifestType: singleton
ManifestVersion: 1.4.0
EOF

    info "Created Windows package manifests"
}

# Build for RISC-V
build_riscv() {
    info "Building for RISC-V..."

    # Check for cross-compilation toolchain
    if ! rustup target list --installed | grep -q "riscv64gc-unknown-linux-gnu"; then
        warn "RISC-V target not installed. Installing..."
        rustup target add riscv64gc-unknown-linux-gnu
    fi

    cd "$PROJECT_ROOT"

    # Cross-compile
    cargo build --release --target riscv64gc-unknown-linux-gnu

    info "Built for RISC-V: target/riscv64gc-unknown-linux-gnu/release/bet-cli"
}

# Main
main() {
    local command="${1:-all}"

    case "$command" in
        rust) build_rust "${2:-}" ;;
        lsp) build_lsp ;;
        deb) build_deb ;;
        rpm) build_rpm ;;
        homebrew) build_homebrew ;;
        aur) build_aur ;;
        flatpak) build_flatpak ;;
        windows) build_windows ;;
        riscv) build_riscv ;;
        all)
            build_rust
            build_lsp
            build_deb
            build_rpm
            build_homebrew
            build_aur
            build_flatpak
            build_windows
            ;;
        *)
            echo "Usage: $0 {rust|lsp|deb|rpm|homebrew|aur|flatpak|windows|riscv|all}"
            exit 1
            ;;
    esac
}

main "$@"
