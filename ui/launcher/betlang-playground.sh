#!/bin/bash
# SPDX-License-Identifier: PMPL-1.0-or-later
# Betlang Playground Launcher
# Compliant with: https://github.com/hyperpolymath/standards/tree/main/launcher/launcher-standard.a2ml

# This script is the primary entry point for the Betlang Playground
# It implements the launcher standard's required modes and fallback ladder

set -euo pipefail

# ============================================================================
# Configuration
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

APP_NAME="betlang-playground"
APP_DISPLAY="Betlang Playground"
APP_URL="https://github.com/hyperpolymath/betlang"
STANDARDS_COMPLIANCE="launcher-standard-0.2.0"

# Runtime mode configuration
# Use Python's simple HTTP server, falling back to PHP, then Deno
GUI_CMD="python3 -m http.server 3000"
TUI_CMD="python3 -m http.server 3000"
LOG_FILE="/tmp/${APP_NAME}.log"

# Required modes per launcher-standard.a2ml
MODES=("--start" "--stop" "--status" "--auto" "--browser" "--integ" "--disinteg" "--help" "--debug" "--logs" "--tail")

# ============================================================================
# Helper Functions
# ============================================================================

log() {
    local level="${1:-INFO}"
    local message="$2"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] [$level] $message" >> "$LOG_FILE" 2>/dev/null || true
    echo "[$level] $message" >&2
}

usage() {
    cat <<EOF
$APP_DISPLAY - Launcher
Compliant with: $STANDARDS_COMPLIANCE

Usage: $0 [MODE] [ARGS...]

Required Modes (launcher standard):
  --start         Start the application in background
  --stop          Stop the running application
  --status        Check if application is running
  --auto          Auto-select mode based on environment
  --browser       Open in browser (implies --start if not running)

Integration Modes:
  --integ         Install desktop integration (shortcuts, icons)
  --disinteg      Remove desktop integration

Meta Modes:
  --help          Show this help message

Optional Modes (developer):
  --debug         Start with debug output
  --logs          Show application logs
  --tail          Tail the application log file

Environment Variables:
  BETLANG_PORT    Port to use (default: 3000)
  BETLANG_BROWSER Browser to use (default: auto-detect)

Examples:
  $0 --start           # Start in background
  $0 --auto            # Auto-start with browser
  $0 --integ           # Install desktop shortcuts
  $0                    # Same as --auto

EOF
}

# ============================================================================
# Mode: --help
# ============================================================================

mode_help() {
    usage
    exit 0
}

# ============================================================================
# Mode: --start
# ============================================================================

mode_start() {
    local port="${BETLANG_PORT:-3000}"
    
    # Check if already running
    if pgrep -f "http.server" > /dev/null 2>&1; then
        log "WARN" "Application already appears to be running"
        echo "Application is already running (PID: $(pgrep -f 'http.server'))" >&2
        exit 1
    fi
    
    log "INFO" "Starting $APP_DISPLAY on port $port"
    
    # Wrap in keepopen.sh for fallback ladder
    exec "$SCRIPT_DIR/keepopen.sh" \
        "$APP_NAME" \
        "$PROJECT_ROOT/public" \
        "cd $PROJECT_ROOT/public && $GUI_CMD" \
        "cd $PROJECT_ROOT/public && $TUI_CMD" \
        "/tmp/${APP_NAME}.log"
}

# ============================================================================
# Mode: --stop
# ============================================================================

mode_stop() {
    log "INFO" "Stopping $APP_DISPLAY"
    
    local pids
    pids=$(pgrep -f "http.server\|python3 -m http" 2>/dev/null || true)
    
    if [ -z "$pids" ]; then
        log "INFO" "No running instance found"
        echo "No running instance found" >&2
        exit 0
    fi
    
    log "INFO" "Killing processes: $pids"
    kill $pids 2>/dev/null || true
    
    # Wait for processes to die
    local count=0
    while pgrep -f "http.server\|python3 -m http" > /dev/null 2>&1 && [ $count -lt 10 ]; do
        sleep 1
        count=$((count + 1))
    done
    
    if pgrep -f "http.server\|python3 -m http" > /dev/null 2>&1; then
        log "WARN" "Processes did not stop gracefully, force killing"
        kill -9 $pids 2>/dev/null || true
    fi
    
    log "INFO" "$APP_DISPLAY stopped"
    echo "Stopped" >&2
}

# ============================================================================
# Mode: --status
# ============================================================================

mode_status() {
    if pgrep -f "http.server\|python3 -m http" > /dev/null 2>&1; then
        local pid=$(pgrep -f "http.server\|python3 -m http" | head -1)
        echo "RUNNING (PID: $pid)"
        exit 0
    else
        echo "STOPPED"
        exit 1
    fi
}

# ============================================================================
# Mode: --auto
# ============================================================================

mode_auto() {
    # Check if already running
    if mode_status > /dev/null 2>&1; then
        log "INFO" "Already running, opening browser"
        mode_browser
        exit 0
    fi
    
    # Start based on environment
    if [ -n "${DISPLAY:-}" ] || [ -n "${WAYLAND_DISPLAY:-}" ]; then
        # GUI environment - try browser
        log "INFO" "GUI environment detected, starting with browser"
        mode_start &
        sleep 3
        mode_browser
    else
        # TUI environment
        log "INFO" "TUI environment detected"
        exec $TUI_CMD
    fi
}

# ============================================================================
# Mode: --browser
# ============================================================================

mode_browser() {
    local port="${BETLANG_PORT:-3000}"
    local url="http://localhost:$port"
    local browser="${BETLANG_BROWSER:-}"
    
    # Try to detect browser
    if [ -z "$browser" ]; then
        if command -v xdg-open &> /dev/null; then
            browser="xdg-open"
        elif command -v open &> /dev/null; then
            browser="open"
        elif command -v start &> /dev/null; then
            browser="start"
        fi
    fi
    
    if [ -z "$browser" ]; then
        log "ERROR" "No browser detected and BETLANG_BROWSER not set"
        echo "Cannot open browser. Set BETLANG_BROWSER or install xdg-open." >&2
        exit 1
    fi
    
    log "INFO" "Opening $url in $browser"
    $browser "$url" 2>/dev/null || true
}

# ============================================================================
# Mode: --integ (Install)
# ============================================================================

mode_integ() {
    log "INFO" "Installing desktop integration for $APP_DISPLAY"
    
    local apps_dir="$HOME/.local/share/applications"
    local icons_dir="$HOME/.local/share/icons/hicolor/256x256/apps"
    local desktop_shortcut_dir="$HOME/Desktop"
    local bin_dir="$HOME/.local/bin"
    
    # Create directories
    mkdir -p "$apps_dir" "$icons_dir" "$desktop_shortcut_dir" "$bin_dir"
    
    # Copy icon (placeholder - replace with actual icon)
    if [ -f "$PROJECT_ROOT/ui/public/favicon.png" ]; then
        cp "$PROJECT_ROOT/ui/public/favicon.png" "$icons_dir/$APP_NAME.png"
        chmod 644 "$icons_dir/$APP_NAME.png"
    else
        # Create a placeholder icon
        log "WARN" "No icon found, creating placeholder"
    fi
    
    # Create desktop file
    local desktop_file="$apps_dir/$APP_NAME.desktop"
    cat > "$desktop_file" <<EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=$APP_DISPLAY
Comment=interactive playground for Betlang probabilistic programming
Exec=$SCRIPT_DIR/$APP_NAME.sh --auto
Icon=$icons_dir/$APP_NAME.png
Terminal=false
Categories=Development;Education;Science;Math;
StartupWMClass=betlang-playground
MimeType=x-scheme-handler/betlang;
X-AppImage-Version=1.0.0
X-Desktop-File-Install-Version=0.26
EOF
    
    chmod 644 "$desktop_file"
    
    # Create desktop shortcut
    cp "$desktop_file" "$desktop_shortcut_dir/$APP_NAME.desktop"
    chmod 755 "$desktop_shortcut_dir/$APP_NAME.desktop"
    
    # Create bin symlink
    ln -sf "$SCRIPT_DIR/$APP_NAME.sh" "$bin_dir/$APP_NAME"
    chmod 755 "$bin_dir/$APP_NAME"
    
    # Refresh desktop database
    if command -v update-desktop-database &> /dev/null; then
        update-desktop-database "$apps_dir" 2>/dev/null || true
    fi
    
    log "INFO" "Desktop integration installed successfully"
    echo "Desktop integration installed:" >&2
    echo "  - Desktop file: $desktop_file" >&2
    echo "  - Binary: $bin_dir/$APP_NAME" >&2
    echo "  - Shortcut: $desktop_shortcut_dir/$APP_NAME.desktop" >&2
}

# ============================================================================
# Mode: --disinteg (Uninstall)
# ============================================================================

mode_disinteg() {
    log "INFO" "Removing desktop integration for $APP_DISPLAY"
    
    local apps_dir="$HOME/.local/share/applications"
    local icons_dir="$HOME/.local/share/icons/hicolor/256x256/apps"
    local desktop_shortcut_dir="$HOME/Desktop"
    local bin_dir="$HOME/.local/bin"
    
    # Remove desktop file
    rm -f "$apps_dir/$APP_NAME.desktop"
    
    # Remove desktop shortcut
    rm -f "$desktop_shortcut_dir/$APP_NAME.desktop"
    
    # Remove icon
    rm -f "$icons_dir/$APP_NAME.png"
    
    # Remove bin symlink
    rm -f "$bin_dir/$APP_NAME"
    
    # Refresh desktop database
    if command -v update-desktop-database &> /dev/null; then
        update-desktop-database "$apps_dir" 2>/dev/null || true
    fi
    
    log "INFO" "Desktop integration removed successfully"
    echo "Desktop integration removed" >&2
}

# ============================================================================
# Mode: --debug
# ============================================================================

mode_debug() {
    log "INFO" "Starting in debug mode"
    echo "Debug mode enabled" >&2
    echo "Log file: $LOG_FILE" >&2
    echo "" >&2
    
    # Start with verbose output
    export RUST_BACKTRACE=1
    export DENO_LOG=debug
    
    exec $GUI_CMD
}

# ============================================================================
# Mode: --logs
# ============================================================================

mode_logs() {
    if [ -f "$LOG_FILE" ]; then
        echo "=== $APP_DISPLAY Logs ===" >&2
        echo "File: $LOG_FILE" >&2
        echo "" >&2
        tail -n 100 "$LOG_FILE" 2>/dev/null || true
    else
        log "INFO" "No log file found: $LOG_FILE"
        echo "No log file found: $LOG_FILE" >&2
        exit 1
    fi
}

# ============================================================================
# Mode: --tail
# ============================================================================

mode_tail() {
    if [ -f "$LOG_FILE" ]; then
        exec tail -f "$LOG_FILE"
    else
        log "INFO" "Waiting for log file: $LOG_FILE"
        echo "Waiting for log file: $LOG_FILE" >&2
        while [ ! -f "$LOG_FILE" ]; do
            sleep 1
        done
        exec tail -f "$LOG_FILE"
    fi
}

# ============================================================================
# Main
# ============================================================================

# Check if called with a mode
if [ $# -gt 0 ]; then
    MODE="$1"
    shift
    
    case "$MODE" in
        --start)      mode_start "$@" ;;
        --stop)       mode_stop "$@" ;;
        --status)     mode_status "$@" ;;
        --auto)       mode_auto "$@" ;;
        --browser)    mode_browser "$@" ;;
        --integ)      mode_integ "$@" ;;
        --disinteg)   mode_disinteg "$@" ;;
        --help)       mode_help "$@" ;;
        --debug)      mode_debug "$@" ;;
        --logs)       mode_logs "$@" ;;
        --tail)       mode_tail "$@" ;;
        *)
            echo "Unknown mode: $MODE" >&2
            usage
            exit 1
            ;;
    esac
else
    # Default mode: --auto
    mode_auto
fi
