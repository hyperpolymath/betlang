#!/bin/bash
# SPDX-License-Identifier: PMPL-1.0-or-later
# keepopen.sh - Fallback ladder wrapper for Betlang Playground
# Compliant with: https://github.com/hyperpolymath/standards/tree/main/launcher/launcher-standard.a2ml

# Usage: keepopen.sh APP_NAME REPO_DIR "GUI_CMD" "TUI_CMD" [LOG_FILE]
#
# Fallback ladder:
# 1. GUI stage (yellow banner on failure) - show banner then try TUI
# 2. TUI stage (red banner on failure) - show banner then drop to shell
# 3. Shell stage (green) - exec bash login at repo dir
#
# Banner visibility is intentionally loud (ugly) - visibility beats aesthetics

set -euo pipefail

# ============================================================================
# Configuration
# ============================================================================

APP_NAME="${1:-betlang-playground}"
REPO_DIR="${2:-"(unknown)"}"
GUI_CMD="${3:-"echo GUI mode not configured"}"
TUI_CMD="${4:-"echo TUI mode not configured"}"
LOG_FILE="${5:-/tmp/${APP_NAME}.log}"

# Banner colors per stage
GUI_COLOR="yellow"
TUI_COLOR="red"
SHELL_COLOR="green"

# GUI dialog tools to try (in order of preference)
GUI_DIALOG_TOOLS=("kdialog" "zenity" "notify-send" "xmessage")

# ============================================================================
# Helper Functions
# ============================================================================

log() {
    local msg="[$(date '+%Y-%m-%d %H:%M:%S')] $1"
    echo "$msg" >> "$LOG_FILE" 2>/dev/null || true
    echo "$msg" >&2
}

show_banner() {
    local colour="$1"
    local stage="$2"
    local message="$3"
    local next_attempt="$4"
    
    log "[FALLBACK] Stage: $stage | Colour: $colour | Message: $message"
    
    # Always log to stderr
    echo "" >&2
    echo "╔════════════════════════════════════════════════════════════════╗" >&2
    echo "║  BETLANG QUANTUM PLAYGROUND - LAUNCHER FAILURE                ║" >&2
    echo "╠════════════════════════════════════════════════════════════════╣" >&2
    echo "║  Stage: $stage                                                ║" >&2
    echo "║  Colour: $colour                                              ║" >&2
    echo "║                                                               ║" >&2
    echo "║  Message: $message                                    ║" >&2
    if [ -n "$next_attempt" ]; then
        echo "║                                                               ║" >&2
        echo "║  Next attempt: $next_attempt                                  ║" >&2
    fi
    echo "║                                                               ║" >&2
    echo "║  Log file: $LOG_FILE                                         ║" >&2
    echo "╚════════════════════════════════════════════════════════════════╝" >&2
    echo "" >&2
    
    # Try GUI dialog if available and we're in a GUI context
    if [ -n "${DISPLAY:-}" ] || [ -n "${WAYLAND_DISPLAY:-}" ]; then
        for tool in "${GUI_DIALOG_TOOLS[@]}"; do
            if command -v "$tool" &> /dev/null; then
                case "$tool" in
                    kdialog)
                        KDIALOG_TITLE="Betlang Launcher Failure - $stage"
                        KDIALOG_MSG="<b>Stage:</b> $stage<br/><b>Message:</b> $message"
                        if [ -n "$next_attempt" ]; then
                            KDIALOG_MSG="$KDIALOG_MSG<br/><br/><b>Next:</b> $next_attempt"
                        fi
                        kdialog --msgbox "$KDIALOG_MSG" --title "$KDIALOG_TITLE" --icon error 2>/dev/null || true
                        ;;
                    zenity)
                        ZENITY_MSG="Stage: $stage\n\nMessage: $message"
                        if [ -n "$next_attempt" ]; then
                            ZENITY_MSG="$ZENITY_MSG\n\nNext attempt: $next_attempt"
                        fi
                        zenity --error --text="$ZENITY_MSG" --title="Betlang Launcher Failure - $stage" 2>/dev/null || true
                        ;;
                    notify-send)
                        notify-send --urgency=critical "Betlang Launcher Failure" "Stage: $stage - $message" 2>/dev/null || true
                        ;;
                    xmessage)
                        echo "Stage: $stage\n\nMessage: $message\n\nNext: $next_attempt" | xmessage -center -file - 2>/dev/null || true
                        ;;
                esac
                break
            fi
        done
    fi
}

# ============================================================================
# Stage 1: GUI Mode
# ============================================================================

run_gui() {
    log "[STAGE 1] Attempting GUI mode: $GUI_CMD"
    
    if eval "$GUI_CMD" 2>> "$LOG_FILE"; then
        log "[STAGE 1] GUI mode succeeded"
        exit 0
    else
        local exit_code=$?
        log "[STAGE 1] GUI mode failed with exit code: $exit_code"
        show_banner "$GUI_COLOR" "GUI" "GUI mode failed: $GUI_CMD" "Trying TUI mode..."
    fi
}

# ============================================================================
# Stage 2: TUI Mode
# ============================================================================

run_tui() {
    log "[STAGE 2] Attempting TUI mode: $TUI_CMD"
    
    if eval "$TUI_CMD" 2>> "$LOG_FILE"; then
        log "[STAGE 2] TUI mode succeeded"
        exit 0
    else
        local exit_code=$?
        log "[STAGE 2] TUI mode failed with exit code: $exit_code"
        show_banner "$TUI_COLOR" "TUI" "TUI mode failed: $TUI_CMD" "Dropping to shell..."
    fi
}

# ============================================================================
# Stage 3: Shell Mode
# ============================================================================

run_shell() {
    log "[STAGE 3] Starting fallback shell at: $REPO_DIR"
    show_banner "$SHELL_COLOR" "SHELL" "Dropping to interactive shell" ""
    
    # Change to repo directory if it exists
    if [ -d "$REPO_DIR" ]; then
        cd "$REPO_DIR"
        log "[STAGE 3] Changed to: $(pwd)"
    else
        log "[STAGE 3] Warning: REPO_DIR not found: $REPO_DIR"
    fi
    
    # Start interactive bash login shell
    # Never show "press enter to close" - just drop to shell
    exec bash -l
}

# ============================================================================
# Main Fallback Ladder
# ============================================================================

log "========================================"
log "Starting keepopen.sh for $APP_NAME"
log "Repo directory: $REPO_DIR"
log "Log file: $LOG_FILE"
log "========================================"

# Ensure log directory exists
mkdir -p "$(dirname "$LOG_FILE")" 2>/dev/null || true

# Write initial marker
log "Launch attempt started at $(date)"

# Run the fallback ladder
run_gui
run_tui
run_shell
