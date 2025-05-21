#!/usr/bin/env bash

# Mew Language Installer
# This script downloads the latest release of Mew language from GitHub,
# extracts it to ~/.mew, and adds it to your PATH environment variable

# Show a welcome message
echo -e "\033[36m   Mew Language Installer\033[0m"
echo -e "\033[36m============================\033[0m"

# Detect platform and architecture
OS="$(uname)"
ARCH="$(uname -m)"

if [[ "$OS" == "Darwin" ]]; then
    PLATFORM="macos"
    if [[ "$ARCH" == "arm64" ]]; then
        ASSET_NAME="mew-macos-arm64.zip"
    else
        ASSET_NAME="mew-macos-x86_64.zip"
    fi
    elif [[ "$OS" == "Linux" ]]; then
    PLATFORM="linux"
    if [[ "$ARCH" == "aarch64" || "$ARCH" == "arm64" ]]; then
        ASSET_NAME="mew-linux-arm64.zip"
        elif [[ "$ARCH" == "armv7l" ]]; then
        ASSET_NAME="mew-linux-armv7.zip"
    else
        ASSET_NAME="mew-linux-x86_64.zip"
    fi
else
    echo -e "\033[31mUnsupported platform: $OS\033[0m"
    exit 1
fi

INSTALL_DIR="$HOME/.mew"
TMP_FILE="/tmp/$ASSET_NAME"

mkdir -p "$INSTALL_DIR"

API_URL="https://api.github.com/repos/mewisme/mew-language/releases/latest"
RELEASE_INFO=$(curl -s -H "Accept: application/vnd.github.v3+json" -H "User-Agent: Mew-Installer" "$API_URL")

if [ $? -ne 0 ]; then
    echo -e "\033[31mError fetching release information\033[0m"
    exit 1
fi

DOWNLOAD_URL=$(echo "$RELEASE_INFO" | grep -o "\"browser_download_url\": \"[^\"]*$ASSET_NAME\"" | cut -d\" -f4)
VERSION=$(echo "$RELEASE_INFO" | grep -o "\"tag_name\": \"[^\"]*\"" | cut -d\" -f4)

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "\033[31mCould not find $ASSET_NAME in the latest release\033[0m"
    exit 1
fi

curl -L -o "$TMP_FILE" "$DOWNLOAD_URL"

if [ $? -ne 0 ]; then
    echo -e "\033[31mError downloading Mew\033[0m"
    exit 1
fi

rm -f "$INSTALL_DIR/mew" "$INSTALL_DIR/README.md" "$INSTALL_DIR/LICENSE"
rm -rf "$INSTALL_DIR/examples"

unzip -o "$TMP_FILE" -d "$INSTALL_DIR"

if [ $? -ne 0 ]; then
    echo -e "\033[31mError extracting files\033[0m"
    exit 1
fi

rm -f "$TMP_FILE"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    SHELL_NAME=$(basename "$SHELL")
    if [[ "$SHELL_NAME" == "bash" ]]; then
        CONFIG_FILE="$HOME/.bashrc"
        if [[ "$PLATFORM" == "macos" ]]; then
            CONFIG_FILE="$HOME/.bash_profile"
            if [ ! -f "$CONFIG_FILE" ]; then
                CONFIG_FILE="$HOME/.profile"
            fi
        fi
        elif [[ "$SHELL_NAME" == "zsh" ]]; then
        CONFIG_FILE="$HOME/.zshrc"
    else
        CONFIG_FILE="$HOME/.profile"
    fi
    
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$CONFIG_FILE"
    echo -e "\033[33mAdded Mew to $CONFIG_FILE\033[0m"
    echo -e "\033[33mYou may need to restart your terminal or run 'source $CONFIG_FILE'\033[0m"
fi

# Make the mew executable
chmod +x "$INSTALL_DIR/mew"

# Verify installation
if [ -f "$INSTALL_DIR/mew" ]; then
    echo -e "\033[32mMew has been successfully installed to $INSTALL_DIR\033[0m"
    if command -v "$INSTALL_DIR/mew" &> /dev/null; then
        VERSION_OUTPUT=$("$INSTALL_DIR/mew" version 2>&1)
        echo -e "\033[36m$VERSION_OUTPUT\033[0m"
    else
        echo -e "\033[33mInstalled but couldn't verify version. You may need to restart your terminal.\033[0m"
    fi
else
    echo -e "\033[31mInstallation may have failed. Could not find mew in $INSTALL_DIR\033[0m"
fi

echo -e "\n\033[36mThank you for installing Mew!\033[0m"
echo -e "\033[36mFor help, type 'mew --help' in your terminal\033[0m"