#!/bin/bash
# Install script for silconspire - Linux/macOS

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Installing SiliconSpire QAP Solver...${NC}"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $OS in
    linux*)
        PLATFORM="linux"
        ;;
    darwin*)
        PLATFORM="macos"
        ;;
    *)
        echo -e "${RED}Unsupported operating system: $OS${NC}"
        exit 1
        ;;
esac

case $ARCH in
    x86_64|amd64)
        ARCH="amd64"
        ;;
    *)
        echo -e "${RED}Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

# Get latest release info
REPO="Prajwal-k-tech/silconspire.rs"
API_URL="https://api.github.com/repos/$REPO/releases/latest"

echo "Fetching latest release information..."
LATEST_RELEASE=$(curl -s $API_URL)

if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to fetch release information${NC}"
    exit 1
fi

VERSION=$(echo $LATEST_RELEASE | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/silconspire-$PLATFORM-$ARCH"

if [ "$PLATFORM" = "linux" ]; then
    BINARY_NAME="silconspire-$PLATFORM-$ARCH"
else
    BINARY_NAME="silconspire-$PLATFORM-$ARCH"
fi

echo "Downloading silconspire $VERSION for $PLATFORM-$ARCH..."
curl -L -o silconspire "$DOWNLOAD_URL"

if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to download binary${NC}"
    exit 1
fi

# Make binary executable
chmod +x silconspire

# Install to system
INSTALL_DIR="/usr/local/bin"
if [ -w "$INSTALL_DIR" ]; then
    mv silconspire "$INSTALL_DIR/silconspire"
    echo -e "${GREEN}✓ Installed silconspire to $INSTALL_DIR${NC}"
else
    echo -e "${YELLOW}⚠ Need sudo to install to $INSTALL_DIR${NC}"
    sudo mv silconspire "$INSTALL_DIR/silconspire"
    echo -e "${GREEN}✓ Installed silconspire to $INSTALL_DIR${NC}"
fi

echo -e "${GREEN}🎉 Installation complete!${NC}"
echo ""
echo "Usage:"
echo "  silconspire --help"
echo "  silconspire --input-file my_problem.txt --pack-size 50"
echo ""
echo "For more information, visit: https://github.com/$REPO"
