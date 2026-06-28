// Key generation FFI functions

use crate::core::{AleoError, AleoErrorCode, set_last_error};
use crate::ffi::types::{AleoPrivateKey, AleoViewKey};
use snarkvm::console::account::{PrivateKey, ViewKey};
use snarkvm::console::network::Testnet3;
use snarkvm::prelude::{ToBytes, FromBytes};
use std::convert::TryFrom;

impl From<ViewKey<Testnet3>> for AleoViewKey {
    fn from(view_key: ViewKey<Testnet3>) -> Self {
        let mut bytes = view_key.to_bytes_le().expect("view key serialization failed");
        bytes.truncate(32);
        let bytes: [u8; 32] = bytes.try_into().expect("view key is not 32 bytes");
        Self { bytes }
    }
}

/// Generate a new private key
///
/// # Safety
/// Caller must free the private key using aleo_free_private_key
#[no_mangle]
pub extern "C" fn aleo_generate_private_key(out_key: *mut *mut AleoPrivateKey) -> AleoErrorCode {
    if out_key.is_null() {
        set_last_error(&AleoError::InvalidInput("out_key is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }

    let mut rng = rand::thread_rng();
    let key: PrivateKey<Testnet3> = match PrivateKey::new(&mut rng) {
        Ok(k) => k,
        Err(e) => {
            set_last_error(&AleoError::CryptoError(format!("Key generation failed: {e}")));
            return AleoErrorCode::CryptoError;
        }
    };

    let ffi_key = AleoPrivateKey::from(key);
    let boxed = Box::new(ffi_key);

    unsafe {
        *out_key = Box::into_raw(boxed);
    }

    AleoErrorCode::Success
}

/// Derive view key from private key
///
/// # Safety
/// Caller must free the view key using aleo_free_view_key
#[no_mangle]
pub extern "C" fn aleo_derive_view_key(
    private_key_ptr: *const AleoPrivateKey,
    out_view_key: *mut *mut AleoViewKey,
) -> AleoErrorCode {
    if private_key_ptr.is_null() || out_view_key.is_null() {
        set_last_error(&AleoError::InvalidInput("null pointer".to_string()));
        return AleoErrorCode::InvalidInput;
    }

    let private_key = unsafe { &*private_key_ptr };

    // FFI → snarkVM PrivateKey
    let sk = match PrivateKey::<Testnet3>::from_bytes_le(&private_key.bytes) {
        Ok(k) => k,
        Err(e) => {
            set_last_error(&AleoError::CryptoError(format!("Invalid private key: {e}")));
            return AleoErrorCode::CryptoError;
        }
    };

    // Derive the view key using TryFrom
    let vk = match ViewKey::<Testnet3>::try_from(sk) {
        Ok(v) => v,
        Err(e) => {
            set_last_error(&AleoError::CryptoError(format!("View key derivation failed: {e}")));
            return AleoErrorCode::CryptoError;
        }
    };

    let ffi_vk = AleoViewKey::from(vk);
    let boxed = Box::new(ffi_vk);

    unsafe {
        *out_view_key = Box::into_raw(boxed);
    }

    AleoErrorCode::Success
}

/// Derive address from view key
///
/// # Safety
/// Caller must free the address using aleo_free_address
#[no_mangle]
pub extern "C" fn aleo_derive_address(
    view_key_ptr: *const AleoViewKey,
    out_address: *mut *mut crate::ffi::types::AleoAddress,
) -> AleoErrorCode {
    use snarkvm::console::account::{Address as SnarkAddress, ViewKey as SnarkViewKey};
    use snarkvm::console::network::Testnet3;
    use snarkvm::prelude::FromBytes;
    use std::convert::TryFrom;

    // Validate pointers
    if view_key_ptr.is_null() {
        set_last_error(&AleoError::InvalidInput("view_key is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }

    if out_address.is_null() {
        set_last_error(&AleoError::InvalidInput("out_address is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }

    // Read FFI view key
    let ffi_vk = unsafe { &*view_key_ptr };

    // Convert FFI → snarkVM ViewKey<Testnet3>
    let vk = match SnarkViewKey::<Testnet3>::from_bytes_le(&ffi_vk.bytes) {
        Ok(v) => v,
        Err(e) => {
            set_last_error(&AleoError::CryptoError(format!(
                "Invalid view key: {e}"
            )));
            return AleoErrorCode::CryptoError;
        }
    };

    // Derive address using snarkVM
    let addr = match SnarkAddress::<Testnet3>::try_from(&vk) {
        Ok(a) => a,
        Err(e) => {
            set_last_error(&AleoError::CryptoError(format!(
                "Address derivation failed: {e}"
            )));
            return AleoErrorCode::CryptoError;
        }
    };

    // Convert snarkVM Address → FFI AleoAddress
    let bech32 = addr.to_string();
    let ffi_addr = crate::ffi::types::AleoAddress::from_string(&bech32);

    // Allocate for .NET
    let boxed = Box::new(ffi_addr);

    unsafe {
        *out_address = Box::into_raw(boxed);
    }

    AleoErrorCode::Success
}
