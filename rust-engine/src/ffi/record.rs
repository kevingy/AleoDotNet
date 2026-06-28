// Record operations FFI functions
// TODO: Implement actual record operations

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use snarkvm::console::account::ViewKey;
use snarkvm::console::network::Testnet3;
use snarkvm::console::program::{Ciphertext, Plaintext, Record as SnarkRecord};
use snarkvm::prelude::FromBytes;

use crate::core::{AleoError, AleoErrorCode, set_last_error};
use crate::ffi::types::{AleoAddress, AleoRecord, AleoViewKey};

/// Decrypt a record
/// 
/// # Arguments
/// * `view_key` - The view key for decryption
/// * `encrypted_record` - The encrypted record data (as a string)
/// * `out_decrypted_record` - Pointer to receive the allocated `AleoRecord`
/// 
/// # Returns
/// Error code (0 on success)
/// 
/// # Safety
/// Caller must free the returned `AleoRecord` using `aleo_free_record`
#[no_mangle]
pub extern "C" fn aleo_decrypt_record(
    view_key: *const AleoViewKey,
    encrypted_record: *const c_char,
    out_decrypted_record: *mut *mut AleoRecord,
) -> AleoErrorCode {
    // Validate pointers
    if view_key.is_null() {
        set_last_error(&AleoError::InvalidInput("view_key is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }

    if encrypted_record.is_null() {
        set_last_error(&AleoError::InvalidInput("encrypted_record is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }

    if out_decrypted_record.is_null() {
        set_last_error(&AleoError::InvalidInput("out_decrypted_record is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }

    // Convert encrypted record to Rust string
    let encrypted_str = unsafe {
        CStr::from_ptr(encrypted_record)
            .to_string_lossy()
            .into_owned()
    };

    // Convert FFI view key → snarkVM view key
    let ffi_vk = unsafe { &*view_key };
    let vk = match ViewKey::<Testnet3>::from_bytes_le(&ffi_vk.bytes) {
        Ok(v) => v,
        Err(e) => {
            set_last_error(&AleoError::CryptoError(format!("Invalid view key: {e}")));
            return AleoErrorCode::CryptoError;
        }
    };

    // Parse the encrypted record (bech32m string with "record1" prefix)
    let ciphertext: SnarkRecord<Testnet3, Ciphertext<Testnet3>> = match encrypted_str.parse() {
        Ok(c) => c,
        Err(e) => {
            set_last_error(&AleoError::CryptoError(format!("Invalid ciphertext: {e}")));
            return AleoErrorCode::CryptoError;
        }
    };

    // Attempt decryption
    let record: SnarkRecord<Testnet3, Plaintext<Testnet3>> = match ciphertext.decrypt(&vk) {
        Ok(r) => r,
        Err(e) => {
            set_last_error(&AleoError::CryptoError(format!("Decryption failed: {e}")));
            return AleoErrorCode::CryptoError;
        }
    };

    // Convert decrypted record → JSON
    let json = match serde_json::to_string(&record) {
        Ok(j) => j,
        Err(e) => {
            set_last_error(&AleoError::CryptoError(format!("JSON encode failed: {e}")));
            return AleoErrorCode::CryptoError;
        }
    };

    // Wrap JSON bytes in an AleoRecord
    let aleo_record = AleoRecord::from_bytes(json.as_bytes());
    let boxed = Box::new(aleo_record);

    unsafe {
        *out_decrypted_record = Box::into_raw(boxed);
    }

    AleoErrorCode::Success
}

/// Encrypt a record
/// 
/// # Arguments
/// * `recipient` - The recipient address
/// * `plaintext_record` - The plaintext record data (as a string)
/// * `out_encrypted_record` - Pointer to receive the allocated encrypted record string
/// 
/// # Returns
/// Error code (0 on success)
/// 
/// # Safety
/// Caller must free the string using aleo_free_string
#[no_mangle]
pub extern "C" fn aleo_encrypt_record(
    recipient: *const AleoAddress,
    plaintext_record: *const std::os::raw::c_char,
    out_encrypted_record: *mut *mut std::os::raw::c_char,
) -> AleoErrorCode {
    if recipient.is_null() {
        set_last_error(&AleoError::InvalidInput("recipient is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }
    
    if plaintext_record.is_null() {
        set_last_error(&AleoError::InvalidInput("plaintext_record is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }
    
    if out_encrypted_record.is_null() {
        set_last_error(&AleoError::InvalidInput("out_encrypted_record is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }
    
    let plaintext_str = unsafe {
        std::ffi::CStr::from_ptr(plaintext_record)
            .to_string_lossy()
            .into_owned()
    };
    
    // TODO: Implement actual record encryption using Aleo cryptographic libraries
    // Placeholder: return the "encrypted:" prefix
    let encrypted = format!("encrypted:{}", plaintext_str);
    
    let c_string = std::ffi::CString::new(encrypted).unwrap_or_else(|_| {
        std::ffi::CString::new("invalid").unwrap()
    });
    
    unsafe {
        *out_encrypted_record = c_string.into_raw();
    }
    
    AleoErrorCode::Success
}

/// Convert a record to a JSON string
///
/// The record data bytes are expected to be valid UTF-8 JSON content
/// (e.g. from a prior call to `aleo_decrypt_record`).
///
/// # Arguments
/// * `record` - Pointer to an `AleoRecord`
/// * `out_string` - Pointer to receive the allocated JSON string
///
/// # Returns
/// Error code (0 on success)
///
/// # Safety
/// Caller must free the returned string using `aleo_free_string`
#[no_mangle]
pub extern "C" fn aleo_record_to_json(
    record: *const AleoRecord,
    out_string: *mut *mut c_char,
) -> AleoErrorCode {
    // Validate pointers
    if record.is_null() {
        set_last_error(&AleoError::InvalidInput("record is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }

    if out_string.is_null() {
        set_last_error(&AleoError::InvalidInput("out_string is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }

    let r = unsafe { &*record };
    let bytes = unsafe { r.as_bytes() };

    // Convert data bytes to a Rust string (must be valid UTF-8)
    let json = match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => {
            set_last_error(&AleoError::CryptoError("Record data is not valid UTF-8".to_string()));
            return AleoErrorCode::CryptoError;
        }
    };

    // Convert JSON → C string
    let c_string = match CString::new(json) {
        Ok(s) => s,
        Err(_) => {
            set_last_error(&AleoError::CryptoError("CString conversion failed".to_string()));
            return AleoErrorCode::CryptoError;
        }
    };

    // Return allocated string
    unsafe {
        *out_string = c_string.into_raw();
    }

    AleoErrorCode::Success
}
