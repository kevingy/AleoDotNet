// Cryptographic module
// 
// IMPORTANT: All cryptographic implementations in this module must be manually
// authored by developers with cryptographic expertise. Do not attempt to
// implement cryptographic operations without proper knowledge of the Aleo
// protocol and cryptographic best practices.
//
// This module provides the actual cryptographic operations that are called
// by the FFI layer. The FFI layer handles type conversion and memory management,
// while this module handles the actual cryptographic logic.

pub mod keygen;
pub mod address;
pub mod signature;
pub mod proof;
pub mod record;

// Re-export types for FFI layer
pub use keygen::{PrivateKey, ViewKey};
pub use address::Address;
