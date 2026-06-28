// AleoDotNet Rust Native Cryptographic Engine
// 
// This library provides a stable C ABI for .NET interop, exposing
// cryptographic operations for the Aleo blockchain.

// Core modules
pub mod core;
pub mod ffi;
pub mod crypto;
pub mod utils;

// Re-export FFI functions for cbindgen
pub use ffi::{
    // Key generation
    aleo_generate_private_key,
    aleo_derive_view_key,
    aleo_derive_address,
    
    // Address operations
    aleo_address_from_string,
    aleo_address_to_string,
    
    // Transfer operations
    aleo_build_transfer,
    aleo_sign_transaction,
    
    // Record operations
    aleo_decrypt_record,
    aleo_encrypt_record,
    aleo_record_to_json,
    
    // Memory management
    aleo_alloc,
    aleo_free,
    aleo_free_string,
    aleo_free_bytes,
    
    // Type deallocation
    aleo_free_address,
    aleo_free_record,
    aleo_free_transaction,
    aleo_free_private_key,
    aleo_free_view_key,
    
    // Error handling
    aleo_get_last_error,
};

// Re-export error codes
pub use core::error::AleoErrorCode;

// Re-export FFI types
pub use ffi::types::{
    AleoPrivateKey,
    AleoViewKey,
    AleoAddress,
    AleoRecord,
    AleoTransaction,
};
