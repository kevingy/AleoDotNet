// Integration tests
// TODO: Add comprehensive integration tests

use aleo_dotnet_engine::{
    AleoAddress, AleoPrivateKey, AleoTransaction, AleoViewKey,
    aleo_address_from_string, aleo_address_to_string,
    aleo_build_transfer, aleo_decrypt_record, aleo_derive_address,
    aleo_derive_view_key, aleo_encrypt_record, aleo_free_address,
    aleo_free_private_key, aleo_free_string, aleo_free_transaction,
    aleo_free_view_key, aleo_generate_private_key,
    AleoErrorCode,
};

#[test]
fn test_key_generation_flow() {
    // TODO: Test the full key generation flow once crypto is implemented
    // 1. Generate private key
    // 2. Derive view key
    // 3. Derive address
    // 4. Verify all are valid
    
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    let result = aleo_generate_private_key(&mut private_key);
    assert_eq!(result, AleoErrorCode::Success);
    
    let mut view_key: *mut AleoViewKey = std::ptr::null_mut();
    let result = aleo_derive_view_key(private_key, &mut view_key);
    assert_eq!(result, AleoErrorCode::Success);
    
    let mut address: *mut AleoAddress = std::ptr::null_mut();
    let result = aleo_derive_address(view_key, &mut address);
    assert_eq!(result, AleoErrorCode::Success);
    
    // Clean up
    if !private_key.is_null() {
        aleo_free_private_key(private_key);
    }
    if !view_key.is_null() {
        aleo_free_view_key(view_key);
    }
    if !address.is_null() {
        aleo_free_address(address);
    }
}

#[test]
fn test_address_roundtrip() {
    // TODO: Test address serialization/deserialization
    let original_addr = "aleo1testaddress";
    let addr_cstr = std::ffi::CString::new(original_addr).unwrap();
    
    let mut address: *mut AleoAddress = std::ptr::null_mut();
    let result = aleo_address_from_string(addr_cstr.as_ptr(), &mut address);
    assert_eq!(result, AleoErrorCode::Success);
    
    let mut out_string: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_address_to_string(address, &mut out_string);
    assert_eq!(result, AleoErrorCode::Success);
    
    if !out_string.is_null() {
        let _retrieved = unsafe {
            std::ffi::CStr::from_ptr(out_string)
                .to_string_lossy()
                .into_owned()
        };
        // TODO: Compare with original once proper implementation is done
        // assert_eq!(retrieved, original_addr);
        
        aleo_free_string(out_string);
    }
    
    if !address.is_null() {
        aleo_free_address(address);
    }
}

#[test]
fn test_transfer_building() {
    // TODO: Test transfer building once implemented
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);
    
    let recipient_str = std::ffi::CString::new("aleo1recipient").unwrap();
    let mut recipient: *mut AleoAddress = std::ptr::null_mut();
    aleo_address_from_string(recipient_str.as_ptr(), &mut recipient);
    
    let mut transaction: *mut AleoTransaction = std::ptr::null_mut();
    let result = aleo_build_transfer(private_key, recipient, 1000, &mut transaction);
    assert_eq!(result, AleoErrorCode::Success);
    
    // Clean up
    if !private_key.is_null() {
        aleo_free_private_key(private_key);
    }
    if !recipient.is_null() {
        aleo_free_address(recipient);
    }
    if !transaction.is_null() {
        aleo_free_transaction(transaction);
    }
}

#[test]
fn test_record_encryption_decryption() {
    // TODO: Test record encryption/decryption once implemented
    let mut view_key: *mut AleoViewKey = std::ptr::null_mut();
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);
    aleo_derive_view_key(private_key, &mut view_key);
    
    let recipient_str = std::ffi::CString::new("aleo1recipient").unwrap();
    let mut recipient: *mut AleoAddress = std::ptr::null_mut();
    aleo_address_from_string(recipient_str.as_ptr(), &mut recipient);
    
    let plaintext = std::ffi::CString::new("plaintext record").unwrap();
    let mut encrypted: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_encrypt_record(recipient, plaintext.as_ptr(), &mut encrypted);
    assert_eq!(result, AleoErrorCode::Success);
    
    let mut decrypted: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_decrypt_record(view_key, encrypted, &mut decrypted);
    assert_eq!(result, AleoErrorCode::Success);
    
    // Clean up
    if !private_key.is_null() {
        aleo_free_private_key(private_key);
    }
    if !view_key.is_null() {
        aleo_free_view_key(view_key);
    }
    if !recipient.is_null() {
        aleo_free_address(recipient);
    }
    if !encrypted.is_null() {
        aleo_free_string(encrypted);
    }
    if !decrypted.is_null() {
        aleo_free_string(decrypted);
    }
}
