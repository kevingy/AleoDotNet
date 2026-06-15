# AleoDotNet

**The official .NET ecosystem for the Aleo blockchain — SDK, wallet, cryptographic engine, and developer tools.**

AleoDotNet aims to provide a complete, modern, and high‑performance .NET interface to the Aleo zero‑knowledge blockchain.  
Planned components include:

- A Rust‑based native cryptographic engine (planned)  
- A safe and minimal P/Invoke interop layer (planned)
- Strongly‑typed Aleo primitives (scaffolded)  
- A full RPC client for interacting with Aleo nodes (planned)  
- A secure wallet implementation (planned)  
- A high‑level SDK for building Aleo applications in C# (planned)

AleoDotNet is designed for long‑term stability, clarity, and developer experience — enabling .NET developers to build private, verifiable applications on Aleo with confidence.

---

## Features

- **Native Rust Engine**  
  High‑performance cryptographic operations exposed through a stable C ABI.

- **Safe Interop Layer**  
  Carefully designed P/Invoke bindings with strict memory and error‑handling rules.

- **Aleo Primitives**  
  Strongly‑typed representations of Aleo addresses, records, fields, scalars, and more.

- **RPC Client**  
  A modern, async‑first client for interacting with Aleo nodes.

- **Wallet Support**  
  Key management, record handling, and transaction construction.

- **High‑Level SDK**  
  A clean, intuitive API for building Aleo applications in .NET.

---

## Repository Structure

AleoDotNet/
rust-engine/        # Native Rust cryptographic engine
src/
Aleo.Primitives/  # Core Aleo types
Aleo.Crypto/      # FFI bindings + safe wrappers
Aleo.Rpc/         # RPC client
Aleo.Wallet/      # Wallet + transaction logic
Aleo.Sdk/         # High-level SDK façade
tests/
Aleo.Tests/       # Test suite


---

## Getting Started

### Prerequisites

- .NET 10 or later  
- Rust (latest stable)  
- Cargo  
- Visual Studio 2022 or VS Code  

### Building the Solution

```bash
dotnet build
```

### Running Tests

```bash
dotnet test
```

## Roadmap
- [ ] Rust engine ABI definition
- [ ] P/Invoke interop layer
- [ ] Aleo primitives
- [ ] RPC client
- [ ] Wallet implementation
- [ ] High‑level SDK
- [ ] Developer templates
- [ ] Documentation site

## License
AleoDotNet is licensed under the Apache License 2.0, matching the Aleo ecosystem.

## Contributing
Contributions are welcome!
Please see .github/copilot-instructions.md and CONTRIBUTING.md (coming soon) for guidelines.

## Status
🚧 Active Development  
AleoDotNet is under construction and not yet ready for production use.