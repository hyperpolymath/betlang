#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# Build script for Betlang containers
#
# Prefers nerdctl, falls back to podman

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Detect container runtime
detect_runtime() {
    if command -v nerdctl &> /dev/null; then
        echo "nerdctl"
    elif command -v podman &> /dev/null; then
        echo "podman"
    elif command -v docker &> /dev/null; then
        echo "docker"
    else
        echo "ERROR: No container runtime found. Install nerdctl (preferred) or podman." >&2
        exit 1
    fi
}

RUNTIME=$(detect_runtime)
echo "Using container runtime: $RUNTIME"

# Image settings
IMAGE_NAME="${IMAGE_NAME:-betlang}"
IMAGE_TAG="${IMAGE_TAG:-latest}"
DEV_IMAGE_NAME="${DEV_IMAGE_NAME:-betlang-dev}"

build_runtime() {
    echo "Building runtime image: $IMAGE_NAME:$IMAGE_TAG"
    $RUNTIME build \
        -t "$IMAGE_NAME:$IMAGE_TAG" \
        -f "$SCRIPT_DIR/Containerfile" \
        "$PROJECT_ROOT"
}

build_dev() {
    echo "Building development image: $DEV_IMAGE_NAME:$IMAGE_TAG"
    $RUNTIME build \
        -t "$DEV_IMAGE_NAME:$IMAGE_TAG" \
        -f "$SCRIPT_DIR/Containerfile.dev" \
        "$PROJECT_ROOT"
}

run_repl() {
    echo "Starting Betlang REPL..."
    $RUNTIME run -it --rm "$IMAGE_NAME:$IMAGE_TAG"
}

run_file() {
    local file="${1:-}"
    if [[ -z "$file" ]]; then
        echo "Usage: $0 run <file.bet>" >&2
        exit 1
    fi

    local dir=$(dirname "$file")
    local name=$(basename "$file")

    $RUNTIME run -it --rm \
        -v "$dir:/work:ro" \
        "$IMAGE_NAME:$IMAGE_TAG" \
        bet run "/work/$name"
}

run_dev() {
    echo "Starting development shell..."
    $RUNTIME run -it --rm \
        -v "$PROJECT_ROOT:/workspace" \
        -w /workspace \
        "$DEV_IMAGE_NAME:$IMAGE_TAG" \
        bash
}

push() {
    local registry="${1:-}"
    if [[ -z "$registry" ]]; then
        echo "Usage: $0 push <registry>" >&2
        exit 1
    fi

    $RUNTIME push "$registry/$IMAGE_NAME:$IMAGE_TAG"
}

usage() {
    cat <<EOF
Betlang Container Build Script

Usage: $0 <command> [args]

Commands:
    build       Build the runtime image
    build-dev   Build the development image
    repl        Run the REPL in a container
    run <file>  Run a betlang file in a container
    dev         Start a development shell
    push <reg>  Push image to registry

Environment Variables:
    IMAGE_NAME     Image name (default: betlang)
    IMAGE_TAG      Image tag (default: latest)
    DEV_IMAGE_NAME Dev image name (default: betlang-dev)

Examples:
    $0 build
    $0 repl
    $0 run examples/hello.bet
    $0 dev
EOF
}

# Main
case "${1:-}" in
    build)
        build_runtime
        ;;
    build-dev)
        build_dev
        ;;
    repl)
        run_repl
        ;;
    run)
        run_file "${2:-}"
        ;;
    dev)
        run_dev
        ;;
    push)
        push "${2:-}"
        ;;
    help|--help|-h)
        usage
        ;;
    *)
        usage
        exit 1
        ;;
esac
