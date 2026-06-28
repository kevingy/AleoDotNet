// FFI type definitions
// These are ABI-stable types used for C interop
// TODO: Adjust sizes and layouts to match Aleo protocol specifications

use snarkvm::prelude::{PrivateKey, Testnet3, ToBytes};

/// FFI-safe representation of an Aleo private key
/// 
/// # Note
/// The exact size and layout must match Aleo's private key specification.
/// snarkVM private keys are 32 bytes.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AleoPrivateKey {
    /// Raw bytes of the private key (32 bytes for snarkVM)
    pub bytes: [u8; 32],
}

// Conversion from snarkVM PrivateKey to FFI type
impl From<PrivateKey<Testnet3>> for AleoPrivateKey {
    fn from(key: PrivateKey<Testnet3>) -> Self {
        // Serialize to little-endian bytes
        let mut bytes = key.to_bytes_le().expect("private key serialization failed");

        // The first 32 bytes are the actual private key
        bytes.truncate(32);

        // Convert Vec<u8> → [u8; 32]
        let bytes: [u8; 32] = bytes.try_into().expect("private key is not 32 bytes");

        Self { bytes }
    }
}

/// FFI-safe representation of an Aleo view key
/// 
/// # Note
/// The exact size and layout must match Aleo's view key specification.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AleoViewKey {
    /// Raw bytes of the view key
    /// TODO: Determine correct size based on Aleo protocol
    pub bytes: [u8; 32],
}

impl Default for AleoViewKey {
    fn default() -> Self {
        AleoViewKey {
            bytes: [0u8; 32],
        }
    }
}

/// FFI-safe representation of an Aleo address
/// 
/// # Note
/// This can either be a bech32 string or fixed bytes depending on the chosen representation.
/// The current implementation uses a bech32 string for flexibility.
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct AleoAddress {
    /// Null-terminated bech32 address string
    /// This pointer is owned by Rust and must be freed using aleo_free_address
    pub bech32: *mut std::os::raw::c_char,
}

impl AleoAddress {
    /// Create a new AleoAddress from a string
    /// 
    /// # Safety
    /// Caller must ensure the resulting AleoAddress is properly freed
    pub fn from_string(s: &str) -> Self {
        let c_string = std::ffi::CString::new(s).unwrap_or_else(|_| {
            std::ffi::CString::new("invalid").unwrap()
        });
        AleoAddress {
            bech32: c_string.into_raw(),
        }
    }
    
    /// Convert to a Rust string (for internal use)
    /// 
    /// # Safety
    /// The bech32 pointer must be valid and null-terminated
    pub unsafe fn to_string(&self) -> String {
        if self.bech32.is_null() {
            return String::new();
        }
        std::ffi::CStr::from_ptr(self.bech32)
            .to_string_lossy()
            .into_owned()
    }
}

impl Drop for AleoAddress {
    fn drop(&mut self) {
        if !self.bech32.is_null() {
            unsafe {
                let _ = std::ffi::CString::from_raw(self.bech32);
            }
        }
    }
}

impl Default for AleoAddress {
    fn default() -> Self {
        AleoAddress {
            bech32: std::ptr::null_mut(),
        }
    }
}

unsafe impl Send for AleoAddress {}
unsafe impl Sync for AleoAddress {}

/// FFI-safe representation of an Aleo record
/// 
/// # Note
/// Records are complex structures. This is a simplified placeholder.
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct AleoRecord {
    /// Serialized record data
    /// This pointer is owned by Rust and must be freed
    pub data: *mut u8,
    /// Length of the data
    pub len: usize,
}

impl AleoRecord {
    /// Create a new AleoRecord from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return AleoRecord {
                data: std::ptr::null_mut(),
                len: 0,
            };
        }
        
        let layout = std::alloc::Layout::from_size_align(bytes.len(), 1).unwrap();
        unsafe {
            let ptr = std::alloc::alloc(layout);
            if !ptr.is_null() {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
            }
            AleoRecord {
                data: ptr,
                len: bytes.len(),
            }
        }
    }
    
    /// Convert to a Rust byte slice (for internal use)
    /// 
    /// # Safety
    /// The data pointer must be valid and len must be correct
    pub unsafe fn as_bytes(&self) -> &[u8] {
        if self.data.is_null() || self.len == 0 {
            return &[];
        }
        std::slice::from_raw_parts(self.data, self.len)
    }
}

impl Drop for AleoRecord {
    fn drop(&mut self) {
        if !self.data.is_null() && self.len > 0 {
            let layout = std::alloc::Layout::from_size_align(self.len, 1).unwrap();
            unsafe {
                std::alloc::dealloc(self.data, layout);
            }
        }
    }
}

impl Default for AleoRecord {
    fn default() -> Self {
        AleoRecord {
            data: std::ptr::null_mut(),
            len: 0,
        }
    }
}

unsafe impl Send for AleoRecord {}
unsafe impl Sync for AleoRecord {}

/// FFI-safe representation of a transaction
/// 
/// # Note
/// Transactions are complex structures. This is a simplified placeholder.
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct AleoTransaction {
    /// Serialized transaction data
    /// This pointer is owned by Rust and must be freed
    pub data: *mut u8,
    /// Length of the data
    pub len: usize,
}

impl AleoTransaction {
    /// Create a new AleoTransaction from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return AleoTransaction {
                data: std::ptr::null_mut(),
                len: 0,
            };
        }
        
        let layout = std::alloc::Layout::from_size_align(bytes.len(), 1).unwrap();
        unsafe {
            let ptr = std::alloc::alloc(layout);
            if !ptr.is_null() {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
            }
            AleoTransaction {
                data: ptr,
                len: bytes.len(),
            }
        }
    }
    
    /// Convert to a Rust byte slice (for internal use)
    /// 
    /// # Safety
    /// The data pointer must be valid and len must be correct
    pub unsafe fn as_bytes(&self) -> &[u8] {
        if self.data.is_null() || self.len == 0 {
            return &[];
        }
        std::slice::from_raw_parts(self.data, self.len)
    }
}

impl Drop for AleoTransaction {
    fn drop(&mut self) {
        if !self.data.is_null() && self.len > 0 {
            let layout = std::alloc::Layout::from_size_align(self.len, 1).unwrap();
            unsafe {
                std::alloc::dealloc(self.data, layout);
            }
        }
    }
}

impl Default for AleoTransaction {
    fn default() -> Self {
        AleoTransaction {
            data: std::ptr::null_mut(),
            len: 0,
        }
    }
}

unsafe impl Send for AleoTransaction {}
unsafe impl Sync for AleoTransaction {}

// FFI functions for freeing allocated types

/// Free an AleoAddress
///
/// # Safety
/// Caller must ensure `addr` was allocated by this library and is not used after this call.
#[no_mangle]
pub extern "C" fn aleo_free_address(addr: *mut AleoAddress) {
    if addr.is_null() {
        return;
    }

    unsafe {
        // Reconstruct the Box so Rust will drop it.
        // The AleoAddress Drop impl handles freeing the inner bech32 string.
        let _ = Box::from_raw(addr);
    }
}

/// Free an AleoRecord
#[no_mangle]
pub extern "C" fn aleo_free_record(record: *mut AleoRecord) {
    if record.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(record);
    }
}

/// Free an AleoTransaction
#[no_mangle]
pub extern "C" fn aleo_free_transaction(tx: *mut AleoTransaction) {
    if tx.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(tx);
    }
}

/// Free an AleoPrivateKey
#[no_mangle]
pub extern "C" fn aleo_free_private_key(key: *mut AleoPrivateKey) {
    if key.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(key);
    }
}

/// Free an AleoViewKey
#[no_mangle]
pub extern "C" fn aleo_free_view_key(key: *mut AleoViewKey) {
    if key.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(key);
    }
}
