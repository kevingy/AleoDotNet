# Build script for Windows
# Compiles the Rust engine and copies the binary to the .NET interop directory

$ErrorActionPreference = "Stop"

Write-Host "Building AleoDotNet Rust engine..." -ForegroundColor Green

# Build the Rust project in release mode
cargo build --release

# Create target directory
$targetDir = "..\Aleo.Crypto\Interop\win-x64"
$binaryName = "aleo_dotnet_engine.dll"

if (-not (Test-Path $targetDir)) {
    New-Item -ItemType Directory -Path $targetDir -Force | Out-Null
}

# Copy the compiled binary
Write-Host "Copying $binaryName to $targetDir" -ForegroundColor Cyan
Copy-Item "target\release\$binaryName" -Destination "$targetDir\" -Force

# Copy the generated C header
Write-Host "Copying aleo_dotnet_engine.h to ..\Aleo.Crypto\Interop\" -ForegroundColor Cyan
Copy-Item "include\aleo_dotnet_engine.h" -Destination "..\Aleo.Crypto\Interop\" -Force

Write-Host "Build complete!" -ForegroundColor Green
Write-Host "Binary: $targetDir\$binaryName" -ForegroundColor Cyan
Write-Host "Header: ..\Aleo.Crypto\Interop\aleo_dotnet_engine.h" -ForegroundColor Cyan
