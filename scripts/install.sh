#!/bin/bash
#
# Codemerge Installer
#
# This script installs the 'codemerge' CLI tool by fetching the latest
# release from GitHub, determining your OS/architecture, downloading the correct
# asset, extracting it, and installing the binary into /usr/local/bin.
#
# License: MIT
#

set -euo pipefail

# Constants
REPO="gelleson/codemerge"
INSTALL_DIR="/usr/local/bin"
TMP_DIR="$(mktemp -d)"
BINARY="codemerge"

# Cleanup temporary directory on exit
trap 'rm -rf "$TMP_DIR"' EXIT

##########################
# Logging Functions
##########################

# log: Write an informational message to stderr.
# Arguments:
#   $1 - The message to log.
log() {
    echo "[INFO] $1" >&2
}

# error: Write an error message to stderr and exit.
# Arguments:
#   $1 - The error message.
error() {
    echo "[ERROR] $1" >&2
    exit 1
}

##########################
# GitHub Release Utilities
##########################

# get_latest_release: Retrieve the latest release tag from GitHub.
# Outputs:
#   The latest release tag.
get_latest_release() {
    log "Fetching the latest release tag from GitHub..."
    local tag
    tag=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" \
          | grep '"tag_name":' \
          | sed -E 's/.*"([^"]+)".*/\1/')
    if [ -z "$tag" ]; then
        error "Failed to fetch the latest release tag."
    fi
    echo "$tag"
}

##########################
# Asset Determination
##########################

# get_asset_name: Determine the correct asset file name based on OS and architecture.
# Outputs:
#   The asset file name.
get_asset_name() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Darwin)
            os="Darwin"
            ;;
        Linux)
            os="Linux"
            ;;
        *)
            error "Unsupported OS: $os"
            ;;
    esac

    case "$arch" in
        x86_64)
            arch="x86_64"
            ;;
        arm64)
            arch="arm64"
            ;;
        *)
            error "Unsupported architecture: $arch"
            ;;
    esac

    echo "codemerge_${os}_${arch}.tar.gz"
}

##########################
# Download and Extraction
##########################

# download_and_extract: Download and extract the release asset.
# Arguments:
#   $1 - The asset file name.
download_and_extract() {
    local asset_name="$1"
    local asset_url="https://github.com/${REPO}/releases/download/${LATEST_TAG}/${asset_name}"

    log "Downloading asset: $asset_name..."
    if ! curl -L "$asset_url" --output "$TMP_DIR/$asset_name"; then
        error "Failed to download asset: $asset_url"
    fi

    log "Extracting asset..."
    if ! tar -xzf "$TMP_DIR/$asset_name" -C "$TMP_DIR"; then
        error "Failed to extract asset: $asset_name"
    fi
}

##########################
# Binary Installation
##########################

# install_binary: Install the extracted binary to INSTALL_DIR.
install_binary() {
    if [ ! -f "$TMP_DIR/$BINARY" ]; then
        error "Binary '$BINARY' not found in extracted files."
    fi

    log "Installing '$BINARY' to '$INSTALL_DIR'..."
    sudo mv "$TMP_DIR/$BINARY" "$INSTALL_DIR/$BINARY"
    sudo chmod +x "$INSTALL_DIR/$BINARY"
}

##########################
# Main Function
##########################

# main: The main entry point of the installer.
main() {
    log "Starting installation of 'codemerge'..."

    LATEST_TAG="$(get_latest_release)"
    log "Latest release tag: $LATEST_TAG"

    local asset_name
    asset_name="$(get_asset_name)"
    log "Asset to download: $asset_name"

    download_and_extract "$asset_name"
    install_binary

    log "Installation complete! You can now use 'codemerge' from the command line."
}

# Execute the main function
main
