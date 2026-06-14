- @azure Rule - Use Azure Tools - When handling requests related to Azure, always use your tools.
- @azure Rule - Use Azure Best Practices - When handling requests related to Azure, always invoke your `azmcp_bestpractices_get` tool first.
- @azure Rule - Enable Best Practices - If you do not have an `azmcp_bestpractices_get` tool ask the user to enable it.

# Copilot Instructions for AleoDotNet

These instructions define how GitHub Copilot should assist within the **AleoDotNet** repository.  
Copilot must follow these rules when generating code, documentation, tests, or scaffolding.

---

## 1. Project Identity

**AleoDotNet** is the official .NET ecosystem for the Aleo blockchain.  
It includes:

- A Rust native cryptographic engine (`rust-engine/`)
- A P/Invoke FFI layer
- .NET primitives
- RPC client
- Wallet implementation
- High‑level Aleo SDK façade
- Test suite and developer tooling

Copilot should treat this repository as a **long‑term, multi‑layer ecosystem project**, not a single library.

---

## 2. Architecture Overview

### 2.1 Rust Engine (Native Layer)
- Lives in `rust-engine/`
- Provides cryptographic operations, record handling, and program execution
- Exposes a stable C ABI for .NET interop
- **Copilot must NOT generate Rust cryptographic code**

### 2.2 FFI Layer (Interop Layer)
- Lives in `Aleo.Crypto`
- Uses `DllImport` to call into the Rust engine
- Must be safe, explicit, and minimal
- **Copilot must NOT invent FFI signatures**
- Copilot may scaffold *empty* classes or placeholder methods

### 2.3 .NET Primitives
- Lives in `Aleo.Primitives`
- Contains types like `Address`, `Record`, `Field`, `Scalar`, etc.
- Must be immutable, deterministic, and serialization‑friendly

### 2.4 RPC Client
- Lives in `Aleo.Rpc`
- Wraps Aleo node RPC endpoints
- Uses `HttpClient` and JSON serialization
- Copilot may scaffold request/response models

### 2.5 Wallet Layer
- Lives in `Aleo.Wallet`
- Handles keys, signing, record management, and transaction building
- **Copilot must NOT generate signing or cryptographic logic**
- Copilot may scaffold class structures and method signatures

### 2.6 SDK Façade
- Lives in `Aleo.Sdk`
- Provides a high‑level, developer‑friendly API
- Copilot may scaffold public API surfaces

---

## 3. Repository Structure

AleoDotNet/
rust-engine/
src/
Aleo.Primitives/
Aleo.Crypto/
Aleo.Rpc/
Aleo.Wallet/
Aleo.Sdk/
tests/
Aleo.Tests/
.github/
README.md
LICENSE

Copilot should preserve this structure.

---

## 4. Coding Conventions

### 4.1 C#
- Use `file-scoped namespaces`
- Use `nullable enable`
- Use `readonly` where possible
- Prefer `record` for immutable models
- Prefer `static` classes for pure helpers
- Avoid unnecessary abstractions

### 4.2 Rust
- Copilot should NOT generate Rust code unless explicitly asked
- Never generate cryptographic logic
- Never generate unsafe FFI code

### 4.3 Interop
- All FFI calls must be manually verified by the developer
- Copilot may scaffold:
  - `SafeHandle` wrappers
  - `struct` layouts
  - `DllImport` placeholders (empty)

---

## 5. What Copilot IS Allowed to Generate

Copilot may generate:

- Class stubs
- Method signatures
- DTOs and RPC models
- Documentation comments
- README sections
- Test scaffolding
- GitHub Actions workflows
- `.editorconfig`, `.gitignore`, and build props
- High‑level SDK API shapes
- Example usage code
- Serialization helpers
- Error handling patterns
- Dependency injection extensions

---

## 6. What Copilot is NOT Allowed to Generate

Copilot must NOT generate:

### ❌ Cryptographic code  
- Hashing  
- Signing  
- Key generation  
- Encryption/decryption  
- Zero‑knowledge proof logic  

### ❌ Rust cryptography or unsafe FFI  
- No Rust crypto  
- No unsafe blocks  
- No ABI definitions unless explicitly provided  

### ❌ Transaction‑building internals  
- Record decryption  
- Proof generation  
- Signature creation  

### ❌ Fake or hallucinated Aleo protocol details  
- No invented RPC endpoints  
- No invented field formats  
- No invented transaction structures  

If Copilot is unsure, it should generate a placeholder and a TODO comment.

---

## 7. Testing Expectations

Copilot may generate:

- xUnit test classes
- Test scaffolding
- Mock RPC responses
- Serialization tests
- API surface tests

Copilot must NOT generate:

- Cryptographic test vectors  
- Proof‑related tests  
- Wallet signing tests  

These must be manually authored.

---

## 8. Documentation Rules

Copilot may generate:

- API documentation
- README examples
- Usage guides
- XML comments
- Architecture summaries

Copilot must NOT:

- Invent protocol details
- Describe cryptographic internals incorrectly

---

## 9. Scaffolding Rules

When adding new features, Copilot should:

1. Create folder + file structure
2. Add class stubs
3. Add method signatures
4. Add TODO comments for developer‑implemented logic
5. Add tests for the public API shape

Copilot should NOT implement:

- Cryptography  
- FFI logic  
- Protocol‑critical logic  

---

## 10. General Behavior

Copilot should:

- Prefer clarity over cleverness  
- Avoid over‑engineering  
- Follow .NET best practices  
- Use modern C# features  
- Keep public APIs clean and intuitive  
- Generate deterministic, predictable code  

---

# End of Copilot Instructions
