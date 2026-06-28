#!/bin/bash
# Build script for Linux/macOS
# Compiles the Rust engine and copies the binary to the .NET interop directory

set -e

echo "Building AleoDotNet Rust engine..."

# Build the Rust project in release mode
cargo build --release

# Detect platform
OS=$(uname -s)
ARCH=$(uname -m)

# Determine target directory based on platform
if [ "$OS" = "Linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        TARGET_DIR="../Aleo.Crypto/Interop/linux-x64"
        BINARY_NAME="libaleo_dotnet_engine.so"
    else
        echo "Unsupported architecture: $ARCH"
        exit 1
    fi
elif [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "arm64" ]; then
        TARGET_DIR="../Aleo.Crypto/Interop/osx-arm64"
        BINARY_NAME="libaleo_dotnet_engine.dylib"
    else
        echo "Unsupported architecture: $ARCH"
        exit 1
    fi
else
    echo "Unsupported OS: $OS"
    exit 1
fi

# Create target directory if it doesn't exist
mkdir -p "$TARGET_DIR"

# Copy the compiled binary
echo "Copying $BINARY_NAME to $TARGET_DIR"
cp "target/release/$BINARY_NAME" "$TARGET_DIR/"

# Copy the generated C header
echo "Copying aleo_dotnet_engine.h to ../Aleo.Crypto/Interop/"
cp "include/aleo_dotnet_engine.h" "../Aleo.Crypto/Interop/"

echo "Build complete!"
echo "Binary: $TARGET_DIR/$BINARY_NAME"
echo "Header: ../Aleo.Crypto/Interop/aleo_dotnet_engine.h"
