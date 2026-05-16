#!/bin/bash
# SPDX-License-Identifier: PMPL-1.0-or-later
# Betlang Playground - Self-Contained Installation Script
#
# This script installs the Betlang Playground as a complete, standalone package
# with all dependencies handled via containerisation (Podman with Chainguard images)
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/hyperpolymath/betlang/main/ui/INSTALL.sh | bash
#   OR
#   ./INSTALL.sh
#
# Requirements:
#   - Podman (preferred) or Docker
#   - Bash 4+
#   - sudo (for system-wide install)
#
# The script will:
#   1. Detect platform and requirements
#   2. Install Podman if not present (on Linux)
#   3. Build or pull the Chainguard-based container image
#   4. Install launcher scripts
#   5. Set up desktop integration (optional)
#   6. Verify installation

set -euo pipefail

# ============================================================================
# Configuration
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

APP_NAME="betlang-playground"
APP_DISPLAY="Betlang Playground"
APP_VERSION="1.0.0"

# Installation directories
INSTALL_DIR="/opt/${APP_NAME}"
BIN_DIR="/usr/local/bin"
APPS_DIR="$HOME/.local/share/applications"
ICONS_DIR="$HOME/.local/share/icons/hicolor/256x256/apps"
DESKTOP_DIR="$HOME/Desktop"

# Container settings
IMAGE_NAME="${APP_NAME}"
IMAGE_TAG="${APP_VERSION}"
CONTAINERFILE="${PROJECT_ROOT}/containers/Containerfile.chainguard"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ============================================================================
# Helper Functions
# ============================================================================

log() {
    local level="${1:-INFO}"
    local message="$2"
    local color=""
    
    case "$level" in
        ERROR) color="$RED" ;;
        WARN) color="$YELLOW" ;;
        SUCCESS) color="$GREEN" ;;
        INFO) color="$BLUE" ;;
        *) color="" ;;
    esac
    
    echo -e "${color}[${level}]${NC} ${message}"
}

echo_header() {
    echo ""
    echo "╔════════════════════════════════════════════════════════════════════"
    echo "║  $APP_DISPLAY Installation v$APP_VERSION                            ║"
    echo "╚════════════════════════════════════════════════════════════════════"
    echo ""
}

echo_section() {
    echo ""
    echo "=== $1 ==="
    echo ""
}

die() {
    log "ERROR" "$1"
    exit 1
}

check_command() {
    local cmd="$1"
    if ! command -v "$cmd" &> /dev/null; then
        return 1
    fi
    return 0
}

# ============================================================================
# Platform Detection
# ============================================================================

detect_platform() {
    log "INFO" "Detecting platform..."
    
    PLATFORM=$(uname -s)
    ARCH=$(uname -m)
    
    log "INFO" "Platform: $PLATFORM"
    log "INFO" "Architecture: $ARCH"
    
    case "$PLATFORM" in
        Linux*)     PLATFORM="linux" ;;
        Darwin*)    PLATFORM="macos" ;;
        CYGWIN*|MINGW*|MSYS*) PLATFORM="windows" ;;
        *)          PLATFORM="unknown" ;;
    esac
    
    case "$ARCH" in
        x86_64*)    ARCH="amd64" ;;
        aarch64*)   ARCH="arm64" ;;
        arm64*)     ARCH="arm64" ;;
        *)          ARCH="unknown" ;;
    esac
    
    if [ "$PLATFORM" = "unknown" ] || [ "$ARCH" = "unknown" ]; then
        die "Unsupported platform: $PLATFORM/$ARCH"
    fi
}

# ============================================================================
# Requirement Checking
# ============================================================================

check_requirements() {
    echo_section "Checking Requirements"
    
    local missing=()
    
    # Check for Podman (preferred)
    if ! check_command podman; then
        if ! check_command docker; then
            log "WARN" "Neither podman nor docker found"
            missing+=("podman or docker")
        else
            CONTAINER_RUNTIME="docker"
            log "INFO" "Found docker (will use as fallback)"
        fi
    else
        CONTAINER_RUNTIME="podman"
        log "INFO" "Found podman"
    fi
    
    # Check for bash 4+
    if [ "${BASH_VERSINFO[0]}" -lt 4 ] 2>/dev/null; then
        missing+=("bash 4+")
    else
        log "INFO" "Found bash ${BASH_VERSINFO[0]}.${BASH_VERSINFO[1]}"
    fi
    
    # Check for sudo (for system install)
    if ! check_command sudo; then
        log "WARN" "sudo not found - will install to user directory only"
    else
        log "INFO" "Found sudo"
    fi
    
    if [ ${#missing[@]} -gt 0 ]; then
        die "Missing requirements: ${missing[*]}"
    fi
}

# ============================================================================
# Podman Installation
# ============================================================================

install_podman() {
    echo_section "Installing Podman"
    
    case "$PLATFORM" in
        linux)
            log "INFO" "Installing Podman on Linux..."
            
            # Try different Linux distros
            if check_command apt-get; then
                # Debian/Ubuntu
                sudo apt-get update
                sudo apt-get install -y podman
            elif check_command dnf; then
                # Fedora/RHEL
                sudo dnf install -y podman
            elif check_command yum; then
                # CentOS/RHEL
                sudo yum install -y podman
            elif check_command apk; then
                # Alpine
                sudo apk add --no-cache podman
            elif check_command zypper; then
                # openSUSE
                sudo zypper install -y podman
            else
                die "Cannot install podman - unsupported package manager"
            fi
            ;;
        macos)
            log "INFO" "Installing Podman on macOS..."
            log "INFO" "Using Homebrew (may take a while)..."
            
            if ! check_command brew; then
                /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
            fi
            
            brew install podman podman-desktop
            ;;
        *)
            die "Cannot auto-install podman on $PLATFORM"
            ;;
    esac
    
    # Verify installation
    if ! check_command podman; then
        die "Podman installation failed"
    fi
    
    log "SUCCESS" "Podman installed successfully"
    CONTAINER_RUNTIME="podman"
}

# ============================================================================
# Build Container Image
# ============================================================================

build_container() {
    echo_section "Building Container Image"
    
    log "INFO" "Using container runtime: $CONTAINER_RUNTIME"
    log "INFO" "Building from: $CONTAINERFILE"
    
    # Change to project root
    cd "$PROJECT_ROOT"
    
    # Build the image
    if ! $CONTAINER_RUNTIME build \
        -t "$IMAGE_NAME:$IMAGE_TAG" \
        -f "$CONTAINERFILE" \
        . 2>&1 | tee /tmp/build.log; then
        
        log "ERROR" "Container build failed"
        log "INFO" "Build log saved to: /tmp/build.log"
        die "Check /tmp/build.log for details"
    fi
    
    log "SUCCESS" "Container image built: $IMAGE_NAME:$IMAGE_TAG"
}

pull_container() {
    echo_section "Pulling Container Image"
    
    # Try to pull from GitHub Container Registry
    log "INFO" "Attempting to pull pre-built image..."
    
    if $CONTAINER_RUNTIME pull "ghcr.io/hyperpolymath/${IMAGE_NAME}:${IMAGE_TAG}" 2>&1 | grep -q "Downloaded"; then
        log "SUCCESS" "Pulled pre-built image"
        return 0
    else
        log "INFO" "Pre-built image not available, will build locally"
        return 1
    fi
}

# ============================================================================
# Install Launcher Scripts
# ============================================================================

install_launcher() {
    echo_section "Installing Launcher Scripts"
    
    log "INFO" "Creating installation directory: $INSTALL_DIR"
    
    # Use sudo if available and INSTALL_DIR requires it
    if [ -w "$(dirname "$INSTALL_DIR")" ]; then
        SUDO=""
    else
        SUDO="sudo"
    fi
    
    $SUDO mkdir -p "$INSTALL_DIR/launcher" "$INSTALL_DIR/ui"
    
    # Copy launcher scripts
    log "INFO" "Copying launcher scripts..."
    $SUDO cp -r "$SCRIPT_DIR/launcher/"* "$INSTALL_DIR/launcher/"
    $SUDO chmod +x "$INSTALL_DIR/launcher/"*.sh
    
    # Copy UI files (for source-based installation)
    log "INFO" "Copying UI files..."
    $SUDO cp -r "$SCRIPT_DIR/public/"* "$INSTALL_DIR/ui/public/"
    $SUDO cp -r "$SCRIPT_DIR/src/"* "$INSTALL_DIR/ui/src/"
    $SUDO cp "$SCRIPT_DIR/deno.json" "$SCRIPT_DIR/.json" "$SCRIPT_DIR/vite.config.js" "$INSTALL_DIR/ui/"
    
    # Create symlink in PATH
    log "INFO" "Creating symlink: $BIN_DIR/$APP_NAME -> $INSTALL_DIR/launcher/$APP_NAME.sh"
    $SUDO mkdir -p "$BIN_DIR"
    $SUDO ln -sf "$INSTALL_DIR/launcher/$APP_NAME.sh" "$BIN_DIR/$APP_NAME"
    
    log "SUCCESS" "Launcher installed to $INSTALL_DIR"
}

# ============================================================================
# Desktop Integration
# ============================================================================

install_desktop() {
    echo_section "Installing Desktop Integration"
    
    # Create directories
    mkdir -p "$APPS_DIR" "$ICONS_DIR" "$DESKTOP_DIR"
    
    # Create icon (placeholder - in production, use actual icon)
    log "INFO" "Creating icon..."
    cat > "$ICONS_DIR/$APP_NAME.png" <<'EOF'
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256">
<rect width="256" height="256" fill="#1a1b26"/>
<circle cx="128" cy="128" r="80" fill="#7aa2f7"/>
<text x="128" y="140" text-anchor="middle" fill="white" font-size="40" font-family="sans-serif">B</text>
</svg>
EOF
    chmod 644 "$ICONS_DIR/$APP_NAME.png"
    
    # Create desktop file
    log "INFO" "Creating desktop file..."
    cat > "$APPS_DIR/$APP_NAME.desktop" <<EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=$APP_DISPLAY
Comment=interactive playground for Betlang probabilistic programming
Exec=$INSTALL_DIR/launcher/$APP_NAME.sh --auto
Icon=$ICONS_DIR/$APP_NAME.png
Terminal=false
Categories=Development;Education;Science;Math;
StartupWMClass=$APP_NAME
MimeType=x-scheme-handler/betlang;
X-AppImage-Version=$APP_VERSION
X-Desktop-File-Install-Version=0.26
EOF
    chmod 644 "$APPS_DIR/$APP_NAME.desktop"
    
    # Create desktop shortcut
    cp "$APPS_DIR/$APP_NAME.desktop" "$DESKTOP_DIR/$APP_NAME.desktop"
    chmod 755 "$DESKTOP_DIR/$APP_NAME.desktop"
    
    # Refresh desktop database
    if check_command update-desktop-database; then
        update-desktop-database "$APPS_DIR" 2>/dev/null || true
    fi
    
    log "SUCCESS" "Desktop integration installed"
}

# ============================================================================
# Verification
# ============================================================================

verify_installation() {
    echo_section "Verifying Installation"
    
    local errors=()
    
    # Check launcher script
    if [ ! -x "$BIN_DIR/$APP_NAME" ] && [ ! -x "$INSTALL_DIR/launcher/$APP_NAME.sh" ]; then
        errors+=("launcher script not found")
    else
        log "INFO" "✓ Launcher script installed"
    fi
    
    # Check container image
    if ! $CONTAINER_RUNTIME image exists "$IMAGE_NAME:$IMAGE_TAG" &> /dev/null; then
        errors+=("container image not found")
    else
        log "INFO" "✓ Container image available"
    fi
    
    # Check desktop integration
    if [ -f "$APPS_DIR/$APP_NAME.desktop" ]; then
        log "INFO" "✓ Desktop file installed"
    fi
    
    # Check bin symlink
    if [ -L "$BIN_DIR/$APP_NAME" ]; then
        log "INFO" "✓ Binary symlink created"
    fi
    
    if [ ${#errors[@]} -gt 0 ]; then
        log "WARN" "Verification issues: ${errors[*]}"
    else
        log "SUCCESS" "All verification checks passed!"
    fi
}

# ============================================================================
# Main Installation
# ============================================================================

main() {
    echo_header
    
    # Step 1: Detect platform
    detect_platform
    
    # Step 2: Check requirements
    check_requirements
    
    # Step 3: Install Podman if needed
    if ! check_command podman && ! check_command docker; then
        read -p "Podman/Docker not found. Install Podman? [Y/n] " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
            install_podman
        else
            die "Container runtime required. Please install podman or docker manually."
        fi
    fi
    
    # Step 4: Try to pull pre-built image
    if ! pull_container; then
        # Step 5: Build from source
        build_container
    fi
    
    # Step 6: Install launcher
    install_launcher
    
    # Step 7: Desktop integration (optional)
    read -p "Install desktop integration? [Y/n] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
        install_desktop
    fi
    
    # Step 8: Verify
    verify_installation
    
    # Final message
    echo ""
    log "SUCCESS" "========================================"
    log "SUCCESS" "Installation Complete!"
    log "SUCCESS" "========================================"
    echo ""
    log "INFO" "To start the playground, run:"
    log "INFO" "  $APP_NAME --auto"
    echo ""
    log "INFO" "Or use the desktop shortcut if you installed desktop integration."
    echo ""
    log "INFO" "For more information:"
    log "INFO" "  https://github.com/hyperpolymath/betlang"
    echo ""
}

# ============================================================================
# Main Entry Point
# ============================================================================

# Check if we're being sourced
if [[ "${BASH_SOURCE[0]}" != "${0}" ]]; then
    echo "This script must be executed, not sourced" >&2
    exit 1
fi

# Parse arguments
INSTALL_ONLY=false
UNINSTALL=false

while [ $# -gt 0 ]; do
    case "$1" in
        --install-only) INSTALL_ONLY=true ;;
        --uninstall) UNINSTALL=true ;;
        --help|-h) usage; exit 0 ;;
        *) die "Unknown argument: $1" ;;
    esac
    shift
done

if [ "$UNINSTALL" = true ]; then
    echo_header
    echo_section "Uninstalling $APP_DISPLAY"
    
    # Remove bin symlink
    if [ -L "$BIN_DIR/$APP_NAME" ]; then
        log "INFO" "Removing symlink: $BIN_DIR/$APP_NAME"
        sudo rm -f "$BIN_DIR/$APP_NAME"
    fi
    
    # Remove installation directory
    if [ -d "$INSTALL_DIR" ]; then
        log "INFO" "Removing installation directory: $INSTALL_DIR"
        sudo rm -rf "$INSTALL_DIR"
    fi
    
    # Remove desktop integration
    if [ -f "$APPS_DIR/$APP_NAME.desktop" ]; then
        log "INFO" "Removing desktop file"
        rm -f "$APPS_DIR/$APP_NAME.desktop"
    fi
    if [ -f "$DESKTOP_DIR/$APP_NAME.desktop" ]; then
        log "INFO" "Removing desktop shortcut"
        rm -f "$DESKTOP_DIR/$APP_NAME.desktop"
    fi
    if [ -f "$ICONS_DIR/$APP_NAME.png" ]; then
        log "INFO" "Removing icon"
        rm -f "$ICONS_DIR/$APP_NAME.png"
    fi
    if [ -f "$BIN_DIR/$APP_NAME" ]; then
        log "INFO" "Removing bin entry"
        sudo rm -f "$BIN_DIR/$APP_NAME"
    fi
    
    # Refresh desktop database
    if check_command update-desktop-database; then
        update-desktop-database "$APPS_DIR" 2>/dev/null || true
    fi
    
    log "SUCCESS" "Uninstallation complete"
    exit 0
fi

# Run main installation
main
