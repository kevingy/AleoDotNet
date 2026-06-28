// FFI module - exports all FFI functions for C interop

pub mod types;
pub mod keygen;
pub mod address;
pub mod transfer;
pub mod record;

// Re-export types for convenience
pub use types::{
    AleoPrivateKey, AleoViewKey, AleoAddress, AleoRecord, AleoTransaction,
    aleo_free_address, aleo_free_record, aleo_free_transaction,
    aleo_free_private_key, aleo_free_view_key,
};

// Re-export FFI functions
pub use keygen::{aleo_generate_private_key, aleo_derive_view_key, aleo_derive_address};
pub use address::{aleo_address_from_string, aleo_address_to_string};
pub use transfer::{aleo_build_transfer, aleo_sign_transaction};
pub use record::{aleo_decrypt_record, aleo_encrypt_record};

// Re-export memory management functions
pub use crate::core::memory::{aleo_alloc, aleo_free, aleo_free_bytes};

// Re-export error handling
pub use crate::core::error::{aleo_last_error_message, aleo_free_string, AleoErrorCode};
