#!/bin/bash

set -euo pipefail

# Constants
REPO="gelleson/codemerge"
INSTALL_DIR="/usr/local/bin"
TMP_DIR="$(mktemp -d)"
BINARY="codemerge"

# Cleanup temporary directory on exit
trap 'rm -rf "$TMP_DIR"' EXIT

# Logging functions
log() {
    echo "[INFO] $1"
}

error() {
    echo "[ERROR] $1" >&2
    exit 1
}

# Fetch the latest release tag from GitHub
get_latest_release() {
    log "Fetching the latest release tag from GitHub..."
    TAG=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    if [ -z "$TAG" ]; then
        error "Failed to fetch the latest release tag."
    fi
    echo "$TAG"
}

# Determine the correct asset based on OS and architecture
get_asset_name() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Darwin) OS="Darwin" ;;
        Linux)  OS="Linux" ;;
        *)      error "Unsupported OS: $OS" ;;
    esac

    case "$ARCH" in
        x86_64) ARCH="x86_64" ;;
        arm64)  ARCH="arm64" ;;
        *)      error "Unsupported architecture: $ARCH" ;;
    esac

    echo "codemerge_${OS}_${ARCH}.tar.gz"
}

# Download and extract the asset
download_and_extract() {
    ASSET_NAME="$1"
    ASSET_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/${ASSET_NAME}"

    log "Downloading asset: $ASSET_NAME..."
    if ! curl -L "$ASSET_URL" --output "$TMP_DIR/$ASSET_NAME"; then
        error "Failed to download asset: $ASSET_URL"
    fi

    log "Extracting asset..."
    if ! tar -xzf "$TMP_DIR/$ASSET_NAME" -C "$TMP_DIR"; then
        error "Failed to extract asset: $ASSET_NAME"
    fi
}

# Install the binary
install_binary() {
    if [ ! -f "$TMP_DIR/$BINARY" ]; then
        error "Binary '$BINARY' not found in extracted files."
    fi

    log "Installing '$BINARY' to '$INSTALL_DIR'..."
    sudo mv "$TMP_DIR/$BINARY" "$INSTALL_DIR/$BINARY"
    sudo chmod +x "$INSTALL_DIR/$BINARY"
}

# Main function
main() {
    log "Starting installation of 'codemerge'..."

    LATEST_TAG=$(get_latest_release)
    log "Latest release tag: $LATEST_TAG"

    ASSET_NAME=$(get_asset_name)
    log "Asset to download: $ASSET_NAME"

    download_and_extract "$ASSET_NAME"
    install_binary

    log "Installation complete! You can now use 'codemerge' from the command line."
}

# Run the script
main
