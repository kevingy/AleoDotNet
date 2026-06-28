// Address operations FFI functions
// TODO: Implement actual address operations

use crate::core::{AleoError, AleoErrorCode, set_last_error};
use crate::ffi::types::AleoAddress;

/// Parse address from string
/// 
/// # Arguments
/// * `str` - Null-terminated string containing the address
/// * `out_address` - Pointer to receive the allocated address
/// 
/// # Returns
/// Error code (0 on success)
/// 
/// # Safety
/// Caller must free the address using aleo_free_address
#[no_mangle]
pub extern "C" fn aleo_address_from_string(
    str: *const std::os::raw::c_char,
    out_address: *mut *mut AleoAddress,
) -> AleoErrorCode {
    if str.is_null() {
        set_last_error(&AleoError::InvalidInput("str is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }
    
    if out_address.is_null() {
        set_last_error(&AleoError::InvalidInput("out_address is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }
    
    let address_str = unsafe {
        std::ffi::CStr::from_ptr(str)
            .to_string_lossy()
            .into_owned()
    };
    
    // TODO: Validate address format according to Aleo specification
    // Placeholder: accept any string for now
    
    let address = Box::new(AleoAddress::from_string(&address_str));
    
    unsafe {
        *out_address = Box::into_raw(address);
    }
    
    AleoErrorCode::Success
}

/// Convert address to string
/// 
/// # Arguments
/// * `addr` - The address to convert
/// * `out_string` - Pointer to receive the allocated string
/// 
/// # Returns
/// Error code (0 on success)
/// 
/// # Safety
/// Caller must free the string using aleo_free_string
#[no_mangle]
pub extern "C" fn aleo_address_to_string(
    addr: *const AleoAddress,
    out_string: *mut *mut std::os::raw::c_char,
) -> AleoErrorCode {
    if addr.is_null() {
        set_last_error(&AleoError::InvalidInput("addr is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }
    
    if out_string.is_null() {
        set_last_error(&AleoError::InvalidInput("out_string is null".to_string()));
        return AleoErrorCode::InvalidInput;
    }
    
    let address_str = unsafe {
        (*addr).to_string()
    };
    
    let c_string = std::ffi::CString::new(address_str).unwrap_or_else(|_| {
        std::ffi::CString::new("invalid").unwrap()
    });
    
    unsafe {
        *out_string = c_string.into_raw();
    }
    
    AleoErrorCode::Success
}

