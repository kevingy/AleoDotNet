// Core error types for the Aleo cryptographic engine
// TODO: Expand error types based on actual cryptographic operations

use std::fmt;
use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;

/// Represents errors that can occur in the cryptographic engine
#[derive(Debug, Clone, PartialEq)]
pub enum AleoError {
    /// Invalid input parameter
    InvalidInput(String),
    
    /// Memory allocation failure
    MemoryAllocationFailed,
    
    /// Cryptographic operation failed
    CryptoError(String),
    
    /// Serialization/deserialization error
    SerializationError(String),
    
    /// Invalid key format
    InvalidKeyFormat(String),
    
    /// Invalid address format
    InvalidAddressFormat(String),
    
    /// Signature verification failed
    SignatureVerificationFailed,
    
    /// Proof generation failed
    ProofGenerationFailed(String),
    
    /// Record encryption/decryption failed
    RecordCryptoError(String),
    
    /// Unknown error
    Unknown(String),
}

impl fmt::Display for AleoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AleoError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AleoError::MemoryAllocationFailed => write!(f, "Memory allocation failed"),
            AleoError::CryptoError(msg) => write!(f, "Cryptographic error: {}", msg),
            AleoError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            AleoError::InvalidKeyFormat(msg) => write!(f, "Invalid key format: {}", msg),
            AleoError::InvalidAddressFormat(msg) => write!(f, "Invalid address format: {}", msg),
            AleoError::SignatureVerificationFailed => write!(f, "Signature verification failed"),
            AleoError::ProofGenerationFailed(msg) => write!(f, "Proof generation failed: {}", msg),
            AleoError::RecordCryptoError(msg) => write!(f, "Record crypto error: {}", msg),
            AleoError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for AleoError {}

/// Convert AleoError to FFI error code
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AleoErrorCode {
    Success = 0,
    InvalidInput = 1,
    MemoryAllocationFailed = 2,
    CryptoError = 3,
    SerializationError = 4,
    InvalidKeyFormat = 5,
    InvalidAddressFormat = 6,
    SignatureVerificationFailed = 7,
    ProofGenerationFailed = 8,
    RecordCryptoError = 9,
    Unknown = 99,
}

impl From<AleoError> for AleoErrorCode {
    fn from(err: AleoError) -> Self {
        match err {
            AleoError::InvalidInput(_) => AleoErrorCode::InvalidInput,
            AleoError::MemoryAllocationFailed => AleoErrorCode::MemoryAllocationFailed,
            AleoError::CryptoError(_) => AleoErrorCode::CryptoError,
            AleoError::SerializationError(_) => AleoErrorCode::SerializationError,
            AleoError::InvalidKeyFormat(_) => AleoErrorCode::InvalidKeyFormat,
            AleoError::InvalidAddressFormat(_) => AleoErrorCode::InvalidAddressFormat,
            AleoError::SignatureVerificationFailed => AleoErrorCode::SignatureVerificationFailed,
            AleoError::ProofGenerationFailed(_) => AleoErrorCode::ProofGenerationFailed,
            AleoError::RecordCryptoError(_) => AleoErrorCode::RecordCryptoError,
            AleoError::Unknown(_) => AleoErrorCode::Unknown,
        }
    }
}

/// Free a string allocated by Rust
#[no_mangle]
pub extern "C" fn aleo_free_string(ptr: *mut std::os::raw::c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = std::ffi::CString::from_raw(ptr);
        }
    }
}

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = RefCell::new(None);
}

pub fn set_last_error(err: &AleoError) {
    LAST_ERROR.with(|slot| {
        let msg = err.to_string();
        *slot.borrow_mut() = CString::new(msg).ok();
    });
}

#[no_mangle]
pub extern "C" fn aleo_get_last_error() -> *const c_char {
    LAST_ERROR.with(|slot| {
        if let Some(ref cstr) = *slot.borrow() {
            cstr.as_ptr()
        } else {
            std::ptr::null()
        }
    })
}


