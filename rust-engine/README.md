# Rust Native Cryptographic Engine

The Rust native cryptographic engine for AleoDotNet. This module provides high-performance cryptographic operations exposed through a stable C ABI for .NET interop.

## Status

Phase 1 (Foundation) and Phase 2 (FFI Layer) are complete. The project structure is in place with stub implementations.

## Project Structure

```
rust-engine/
├── Cargo.toml                    # Project manifest
├── build.rs                      # Build script for cbindgen
├── cbindgen.toml                 # C header generation config
├── src/
│   ├── lib.rs                    # Library entry point
│   ├── core/                     # Core types and utilities
│   │   ├── error.rs              # Error types
│   │   ├── memory.rs             # Memory management
│   │   └── mod.rs
│   ├── ffi/                      # FFI layer
│   │   ├── mod.rs                # FFI exports
│   │   ├── types.rs              # FFI type definitions
│   │   ├── keygen.rs             # Key generation FFI
│   │   ├── address.rs            # Address operations FFI
│   │   ├── transfer.rs           # Transfer operations FFI
│   │   └── record.rs             # Record operations FFI
│   ├── crypto/                   # Cryptographic operations (TODO: implement)
│   │   ├── mod.rs
│   │   ├── keygen.rs
│   │   ├── address.rs
│   │   ├── signature.rs
│   │   ├── proof.rs
│   │   └── record.rs
│   └── utils/                    # Utility functions
│       ├── mod.rs
│       └── conversion.rs
├── include/
│   └── aleo_dotnet_engine.h      # Generated C header
├── scripts/
│   ├── build.sh                  # Linux/macOS build script
│   └── build.ps1                 # Windows build script
└── tests/
    ├── ffi_test.rs               # FFI layer tests
    └── integration_test.rs       # Integration tests
```

## Building

### Prerequisites

- Rust (latest stable)
- Cargo

### Build Commands

**Windows:**
```powershell
.\scripts\build.ps1
```

**Linux/macOS:**
```bash
./scripts/build.sh
```

Or manually:
```bash
cargo build --release
```

## Implementation Status

### Completed

- ✅ Project structure and Cargo.toml
- ✅ Core error handling and memory management
- ✅ FFI type definitions (AleoPrivateKey, AleoViewKey, AleoAddress, etc.)
- ✅ FFI function stubs for key generation, address operations, transfers, and records
- ✅ Build scripts for cross-platform compilation
- ✅ Test stubs for FFI and integration testing
- ✅ cbindgen configuration for C header generation

### TODO (Requires Developer Implementation)

- ⏳ Select and add actual cryptographic dependencies (aleo-std, snarkvm, blst, etc.)
- ⏳ Implement cryptographic operations in `src/crypto/` module
- ⏳ Adjust FFI type sizes to match Aleo protocol specifications
- ⏳ Implement proper memory tracking for deallocation
- ⏳ Add comprehensive test vectors
- ⏳ Set up CI/CD for automated builds

## Important Notes

- All cryptographic implementations in `src/crypto/` must be manually authored by developers with cryptographic expertise
- The FFI layer provides stub implementations that return placeholder data
- Type sizes in `src/ffi/types.rs` are placeholders and must be adjusted based on Aleo protocol
- Memory management needs proper size tracking for safe deallocation

## Documentation

See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for detailed implementation guidance.
