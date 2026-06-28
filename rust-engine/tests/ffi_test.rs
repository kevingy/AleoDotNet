// FFI layer tests
// TODO: Add comprehensive tests for all FFI functions

use aleo_dotnet_engine::{
    AleoAddress, AleoPrivateKey, AleoRecord, AleoTransaction, AleoViewKey,
    aleo_address_from_string, aleo_address_to_string, aleo_alloc, aleo_free_address,
    aleo_free_private_key, aleo_free_record, aleo_free_string, aleo_free_transaction,
    aleo_free_view_key,
    aleo_generate_private_key, aleo_derive_view_key, aleo_derive_address,
    aleo_build_transfer, aleo_sign_transaction,
    aleo_decrypt_record, aleo_encrypt_record, aleo_record_to_json,
    aleo_get_last_error,
    AleoErrorCode,
};

#[test]
fn test_error_codes() {
    // Test that error codes are defined correctly
    assert_eq!(AleoErrorCode::Success as i32, 0);
    assert_eq!(AleoErrorCode::InvalidInput as i32, 1);
    assert_eq!(AleoErrorCode::MemoryAllocationFailed as i32, 2);
}

#[test]
fn test_generate_private_key_null_pointer() {
    // Test that null pointer is handled correctly
    let result = aleo_generate_private_key(std::ptr::null_mut());
    assert_eq!(result, AleoErrorCode::InvalidInput);
}

#[test]
fn test_generate_private_key() {
    // TODO: Test actual key generation once implemented
    // This is a placeholder test
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    let result = aleo_generate_private_key(&mut private_key);
    
    // Currently returns Success with dummy implementation
    assert_eq!(result, AleoErrorCode::Success);
    
    // Clean up
    if !private_key.is_null() {
        aleo_free_private_key(private_key);
    }
}

#[test]
fn test_address_from_string_null_pointer() {
    let result = aleo_address_from_string(std::ptr::null(), std::ptr::null_mut());
    assert_eq!(result, AleoErrorCode::InvalidInput);
}

#[test]
fn test_address_from_string() {
    let addr_str = std::ffi::CString::new("aleo1test").unwrap();
    let mut address: *mut AleoAddress = std::ptr::null_mut();
    
    let result = aleo_address_from_string(addr_str.as_ptr(), &mut address);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!address.is_null());
    
    // Clean up
    if !address.is_null() {
        aleo_free_address(address);
    }
}

#[test]
fn test_address_from_string_null_out() {
    let addr_str = std::ffi::CString::new("aleo1test").unwrap();
    let result = aleo_address_from_string(addr_str.as_ptr(), std::ptr::null_mut());
    assert_eq!(result, AleoErrorCode::InvalidInput);
}

#[test]
fn test_address_to_string_null_addr() {
    let mut out_string: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_address_to_string(std::ptr::null(), &mut out_string);
    assert_eq!(result, AleoErrorCode::InvalidInput);
}

#[test]
fn test_address_to_string_null_out() {
    let addr_str = std::ffi::CString::new("aleo1test").unwrap();
    let mut address: *mut AleoAddress = std::ptr::null_mut();
    let result = aleo_address_from_string(addr_str.as_ptr(), &mut address);
    assert_eq!(result, AleoErrorCode::Success);
    
    let result = aleo_address_to_string(address, std::ptr::null_mut());
    assert_eq!(result, AleoErrorCode::InvalidInput);
    
    aleo_free_address(address);
}

#[test]
fn test_address_to_string() {
    let original = "aleo1testaddress";
    let addr_str = std::ffi::CString::new(original).unwrap();
    let mut address: *mut AleoAddress = std::ptr::null_mut();
    let result = aleo_address_from_string(addr_str.as_ptr(), &mut address);
    assert_eq!(result, AleoErrorCode::Success);
    
    let mut out_string: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_address_to_string(address, &mut out_string);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!out_string.is_null());
    
    let retrieved = unsafe {
        std::ffi::CStr::from_ptr(out_string)
            .to_string_lossy()
            .into_owned()
    };
    assert_eq!(retrieved, original);
    
    aleo_free_string(out_string);
    aleo_free_address(address);
}

#[test]
fn test_address_roundtrip_with_special_chars() {
    let original = "aleo1abcdefghijklmnopqrstuvwxyz234567";
    let addr_str = std::ffi::CString::new(original).unwrap();
    let mut address: *mut AleoAddress = std::ptr::null_mut();
    let result = aleo_address_from_string(addr_str.as_ptr(), &mut address);
    assert_eq!(result, AleoErrorCode::Success);
    
    let mut out_string: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_address_to_string(address, &mut out_string);
    assert_eq!(result, AleoErrorCode::Success);
    
    let retrieved = unsafe {
        std::ffi::CStr::from_ptr(out_string)
            .to_string_lossy()
            .into_owned()
    };
    assert_eq!(retrieved, original);
    
    aleo_free_string(out_string);
    aleo_free_address(address);
}

#[test]
fn test_free_address_null() {
    // Should not panic or crash on null pointer
    aleo_free_address(std::ptr::null_mut());
}

#[test]
fn test_free_address_valid() {
    // Allocate an address via from_string, then free it
    let addr_str = std::ffi::CString::new("aleo1testfree").unwrap();
    let mut address: *mut AleoAddress = std::ptr::null_mut();
    let result = aleo_address_from_string(addr_str.as_ptr(), &mut address);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!address.is_null());

    // Free should not panic
    aleo_free_address(address);
}

#[test]
fn test_free_address_default() {
    // An address created via default() has a null bech32 pointer
    let addr = Box::into_raw(Box::new(AleoAddress::default()));
    // Free should handle null inner pointer gracefully
    aleo_free_address(addr);
}

#[test]
fn test_free_address_multiple_distinct() {
    // Free multiple independently allocated addresses
    let strs = ["aleo1aaa", "aleo1bbb", "aleo1ccc"];
    let mut addrs: Vec<*mut AleoAddress> = Vec::new();

    for s in &strs {
        let c_str = std::ffi::CString::new(*s).unwrap();
        let mut addr: *mut AleoAddress = std::ptr::null_mut();
        let result = aleo_address_from_string(c_str.as_ptr(), &mut addr);
        assert_eq!(result, AleoErrorCode::Success);
        addrs.push(addr);
    }

    for addr in addrs {
        aleo_free_address(addr);
    }
}

#[test]
fn test_free_view_key_null() {
    aleo_free_view_key(std::ptr::null_mut());
}

#[test]
fn test_free_view_key_valid() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    let result = aleo_generate_private_key(&mut private_key);
    assert_eq!(result, AleoErrorCode::Success);

    let mut view_key: *mut AleoViewKey = std::ptr::null_mut();
    let result = aleo_derive_view_key(private_key, &mut view_key);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!view_key.is_null());

    aleo_free_private_key(private_key);
    aleo_free_view_key(view_key);
}

#[test]
fn test_free_view_key_default() {
    let vk = Box::into_raw(Box::new(AleoViewKey::default()));
    aleo_free_view_key(vk);
}

#[test]
fn test_free_view_key_multiple_distinct() {
    let mut keys: Vec<*mut AleoViewKey> = Vec::new();

    for _ in 0..3 {
        let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
        aleo_generate_private_key(&mut private_key);
        let mut view_key: *mut AleoViewKey = std::ptr::null_mut();
        aleo_derive_view_key(private_key, &mut view_key);
        assert!(!view_key.is_null());
        keys.push(view_key);
        aleo_free_private_key(private_key);
    }

    for vk in keys {
        aleo_free_view_key(vk);
    }
}

#[test]
fn test_memory_allocation() {
    // Test basic memory allocation
    let ptr = aleo_alloc(100);
    assert!(!ptr.is_null());
    
    // Note: aleo_free is a placeholder and doesn't actually deallocate
    // TODO: Implement proper memory tracking for deallocation
    // aleo_free(ptr);
}

#[test]
fn test_string_allocation() {
    // Test string allocation and freeing
    let test_str = "test string";
    let ptr = std::ffi::CString::new(test_str).unwrap().into_raw();
    
    assert!(!ptr.is_null());
    
    // Free the string
    aleo_free_string(ptr);
}

#[test]
fn test_free_string_null() {
    aleo_free_string(std::ptr::null_mut());
}

#[test]
fn test_free_string_from_address_to_string() {
    let original = "aleo1freestringtest";
    let addr_str = std::ffi::CString::new(original).unwrap();
    let mut address: *mut AleoAddress = std::ptr::null_mut();
    let result = aleo_address_from_string(addr_str.as_ptr(), &mut address);
    assert_eq!(result, AleoErrorCode::Success);

    let mut out_string: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_address_to_string(address, &mut out_string);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!out_string.is_null());

    aleo_free_address(address);
    aleo_free_string(out_string);
}

#[test]
fn test_free_string_multiple() {
    let strs = ["str_a", "str_b", "str_c"];
    let mut ptrs: Vec<*mut std::os::raw::c_char> = Vec::new();

    for s in &strs {
        let ptr = std::ffi::CString::new(*s).unwrap().into_raw();
        assert!(!ptr.is_null());
        ptrs.push(ptr);
    }

    for ptr in ptrs {
        aleo_free_string(ptr);
    }
}

#[test]
fn test_address_to_string_and_free_direct() {
    // Construct address via Rust API (not FFI), then roundtrip through FFI
    let addr = AleoAddress::from_string("aleo1testaddress123456789");
    let addr_ptr = Box::into_raw(Box::new(addr));

    let mut out_str: *mut std::os::raw::c_char = std::ptr::null_mut();
    let code = aleo_address_to_string(addr_ptr, &mut out_str);
    assert_eq!(code, AleoErrorCode::Success);
    assert!(!out_str.is_null());

    let rust_str = unsafe {
        std::ffi::CStr::from_ptr(out_str)
            .to_string_lossy()
            .into_owned()
    };
    assert_eq!(rust_str, "aleo1testaddress123456789");

    aleo_free_string(out_str);
    out_str = std::ptr::null_mut();

    // Second free with nulled pointer is safe (no-op)
    aleo_free_string(out_str);

    unsafe { drop(Box::from_raw(addr_ptr)); }
}

#[test]
fn test_decrypt_record_null_view_key() {
    let encrypted = std::ffi::CString::new("encrypted_data").unwrap();
    let mut out: *mut AleoRecord = std::ptr::null_mut();
    let result = aleo_decrypt_record(
        std::ptr::null(),
        encrypted.as_ptr(),
        &mut out,
    );
    assert_eq!(result, AleoErrorCode::InvalidInput);
}

#[test]
fn test_decrypt_record_null_encrypted() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);
    let mut view_key: *mut AleoViewKey = std::ptr::null_mut();
    aleo_derive_view_key(private_key, &mut view_key);
    let mut out: *mut AleoRecord = std::ptr::null_mut();
    let result = aleo_decrypt_record(
        view_key,
        std::ptr::null(),
        &mut out,
    );
    assert_eq!(result, AleoErrorCode::InvalidInput);
    aleo_free_private_key(private_key);
    aleo_free_view_key(view_key);
}

#[test]
fn test_decrypt_record_null_out() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);
    let mut view_key: *mut AleoViewKey = std::ptr::null_mut();
    aleo_derive_view_key(private_key, &mut view_key);
    let encrypted = std::ffi::CString::new("encrypted_data").unwrap();
    let result = aleo_decrypt_record(
        view_key,
        encrypted.as_ptr(),
        std::ptr::null_mut(),
    );
    assert_eq!(result, AleoErrorCode::InvalidInput);
    aleo_free_private_key(private_key);
    aleo_free_view_key(view_key);
}

#[test]
fn test_decrypt_record_invalid_ciphertext() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);
    let mut view_key: *mut AleoViewKey = std::ptr::null_mut();
    aleo_derive_view_key(private_key, &mut view_key);

    let input = "not_a_valid_ciphertext";
    let encrypted = std::ffi::CString::new(input).unwrap();
    let mut out: *mut AleoRecord = std::ptr::null_mut();
    let result = aleo_decrypt_record(view_key, encrypted.as_ptr(), &mut out);
    assert_eq!(result, AleoErrorCode::CryptoError);
    assert!(out.is_null());

    aleo_free_view_key(view_key);
    aleo_free_private_key(private_key);
}

#[test]
fn test_decrypt_record_empty_string() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);
    let mut view_key: *mut AleoViewKey = std::ptr::null_mut();
    aleo_derive_view_key(private_key, &mut view_key);

    let encrypted = std::ffi::CString::new("").unwrap();
    let mut out: *mut AleoRecord = std::ptr::null_mut();
    let result = aleo_decrypt_record(view_key, encrypted.as_ptr(), &mut out);
    assert_eq!(result, AleoErrorCode::CryptoError);
    assert!(out.is_null());

    aleo_free_view_key(view_key);
    aleo_free_private_key(private_key);
}

#[test]
fn test_decrypt_record_empty_view_key_bytes() {
    let vk = Box::into_raw(Box::new(AleoViewKey::default()));
    let input = "record1notavalidciphertext";
    let encrypted = std::ffi::CString::new(input).unwrap();
    let mut out: *mut AleoRecord = std::ptr::null_mut();
    let result = aleo_decrypt_record(vk, encrypted.as_ptr(), &mut out);
    assert_eq!(result, AleoErrorCode::CryptoError);
    assert!(out.is_null());
    aleo_free_view_key(vk);
}

#[test]
fn test_decrypt_record_output_null_after_crypto_error() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);
    let mut view_key: *mut AleoViewKey = std::ptr::null_mut();
    aleo_derive_view_key(private_key, &mut view_key);

    let encrypted = std::ffi::CString::new("garbage").unwrap();
    let mut out: *mut AleoRecord = std::ptr::null_mut();
    let result = aleo_decrypt_record(view_key, encrypted.as_ptr(), &mut out);
    assert_eq!(result, AleoErrorCode::CryptoError);
    assert!(out.is_null());

    aleo_free_record(out);
    aleo_free_view_key(view_key);
    aleo_free_private_key(private_key);
}

#[test]
fn test_free_record_null() {
    aleo_free_record(std::ptr::null_mut());
}

#[test]
fn test_free_record_valid() {
    let data = [0xAB, 0xCD, 0xEF];
    let record = AleoRecord::from_bytes(&data);
    let ptr = Box::into_raw(Box::new(record));
    aleo_free_record(ptr);
}

#[test]
fn test_free_record_default() {
    let record = Box::into_raw(Box::new(AleoRecord::default()));
    aleo_free_record(record);
}

#[test]
fn test_free_record_multiple_distinct() {
    let datasets = [&[1u8, 2, 3][..], &[4, 5, 6, 7][..], &[8, 9][..]];
    let mut records: Vec<*mut AleoRecord> = Vec::new();

    for data in &datasets {
        let record = AleoRecord::from_bytes(data);
        let ptr = Box::into_raw(Box::new(record));
        records.push(ptr);
    }

    for ptr in records {
        aleo_free_record(ptr);
    }
}

#[test]
fn test_record_to_json_null_record() {
    let mut out: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_record_to_json(std::ptr::null(), &mut out);
    assert_eq!(result, AleoErrorCode::InvalidInput);
}

#[test]
fn test_record_to_json_null_out() {
    let record = AleoRecord::from_bytes(b"test data");
    let ptr = Box::into_raw(Box::new(record));
    let result = aleo_record_to_json(ptr, std::ptr::null_mut());
    assert_eq!(result, AleoErrorCode::InvalidInput);
    aleo_free_record(ptr);
}

#[test]
fn test_record_to_json_success() {
    let data = b"hello record";
    let record = AleoRecord::from_bytes(data);
    let ptr = Box::into_raw(Box::new(record));

    let mut out: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_record_to_json(ptr, &mut out);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!out.is_null());

    let retrieved = unsafe {
        std::ffi::CStr::from_ptr(out)
            .to_string_lossy()
            .into_owned()
    };
    assert_eq!(retrieved, "hello record");

    aleo_free_string(out);
    aleo_free_record(ptr);
}

#[test]
fn test_record_to_json_empty() {
    let record = AleoRecord::from_bytes(b"");
    let ptr = Box::into_raw(Box::new(record));

    let mut out: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_record_to_json(ptr, &mut out);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!out.is_null());

    let retrieved = unsafe {
        std::ffi::CStr::from_ptr(out)
            .to_string_lossy()
            .into_owned()
    };
    assert_eq!(retrieved, "");

    aleo_free_string(out);
    aleo_free_record(ptr);
}

#[test]
fn test_record_to_json_and_free() {
    let data = b"free me";
    let record = AleoRecord::from_bytes(data);
    let ptr = Box::into_raw(Box::new(record));

    let mut out: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_record_to_json(ptr, &mut out);
    assert_eq!(result, AleoErrorCode::Success);

    let retrieved = unsafe {
        std::ffi::CStr::from_ptr(out)
            .to_string_lossy()
            .into_owned()
    };
    assert_eq!(retrieved, "free me");

    aleo_free_string(out);
    aleo_free_record(ptr);
}

#[test]
fn test_record_to_json_invalid_utf8() {
    let data = [0xFF, 0xFE, 0x00];
    let record = AleoRecord::from_bytes(&data);
    let ptr = Box::into_raw(Box::new(record));

    let mut out: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_record_to_json(ptr, &mut out);
    assert_eq!(result, AleoErrorCode::CryptoError);
    assert!(out.is_null());

    aleo_free_record(ptr);
}

#[test]
fn test_get_last_error_no_error() {
    let ptr = aleo_get_last_error();
    assert!(ptr.is_null());
}

#[test]
fn test_get_last_error_after_invalid_input() {
    aleo_generate_private_key(std::ptr::null_mut());

    let ptr = aleo_get_last_error();
    assert!(!ptr.is_null());

    let msg = unsafe {
        std::ffi::CStr::from_ptr(ptr)
            .to_string_lossy()
            .into_owned()
    };
    assert!(msg.contains("Invalid input"));
}

#[test]
fn test_encrypt_record_null_recipient() {
    let plaintext = std::ffi::CString::new("data").unwrap();
    let mut out: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_encrypt_record(
        std::ptr::null(),
        plaintext.as_ptr(),
        &mut out,
    );
    assert_eq!(result, AleoErrorCode::InvalidInput);
}

#[test]
fn test_encrypt_record_null_plaintext() {
    let addr_str = std::ffi::CString::new("aleo1recipient").unwrap();
    let mut addr: *mut AleoAddress = std::ptr::null_mut();
    aleo_address_from_string(addr_str.as_ptr(), &mut addr);

    let mut out: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_encrypt_record(addr, std::ptr::null(), &mut out);
    assert_eq!(result, AleoErrorCode::InvalidInput);

    aleo_free_address(addr);
}

#[test]
fn test_encrypt_record_null_out() {
    let addr_str = std::ffi::CString::new("aleo1recipient").unwrap();
    let mut addr: *mut AleoAddress = std::ptr::null_mut();
    aleo_address_from_string(addr_str.as_ptr(), &mut addr);

    let plaintext = std::ffi::CString::new("data").unwrap();
    let result = aleo_encrypt_record(addr, plaintext.as_ptr(), std::ptr::null_mut());
    assert_eq!(result, AleoErrorCode::InvalidInput);

    aleo_free_address(addr);
}

#[test]
fn test_encrypt_record_success() {
    let addr_str = std::ffi::CString::new("aleo1recipient").unwrap();
    let mut addr: *mut AleoAddress = std::ptr::null_mut();
    aleo_address_from_string(addr_str.as_ptr(), &mut addr);

    let plaintext = std::ffi::CString::new("hello").unwrap();
    let mut out: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_encrypt_record(addr, plaintext.as_ptr(), &mut out);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!out.is_null());

    let encrypted = unsafe {
        std::ffi::CStr::from_ptr(out)
            .to_string_lossy()
            .into_owned()
    };
    assert_eq!(encrypted, "encrypted:hello");

    aleo_free_string(out);
    aleo_free_address(addr);
}

#[test]
fn test_derive_view_key_null_private_key() {
    let mut out: *mut AleoViewKey = std::ptr::null_mut();
    let result = aleo_derive_view_key(std::ptr::null(), &mut out);
    assert_eq!(result, AleoErrorCode::InvalidInput);
}

#[test]
fn test_derive_view_key_null_out() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);
    let result = aleo_derive_view_key(private_key, std::ptr::null_mut());
    assert_eq!(result, AleoErrorCode::InvalidInput);
    aleo_free_private_key(private_key);
}

#[test]
fn test_derive_address_null_view_key() {
    let mut out: *mut AleoAddress = std::ptr::null_mut();
    let result = aleo_derive_address(std::ptr::null(), &mut out);
    assert_eq!(result, AleoErrorCode::InvalidInput);
}

#[test]
fn test_derive_address_null_out() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);
    let mut view_key: *mut AleoViewKey = std::ptr::null_mut();
    aleo_derive_view_key(private_key, &mut view_key);

    let result = aleo_derive_address(view_key, std::ptr::null_mut());
    assert_eq!(result, AleoErrorCode::InvalidInput);

    aleo_free_view_key(view_key);
    aleo_free_private_key(private_key);
}

#[test]
fn test_build_transfer_null_private_key() {
    let addr_str = std::ffi::CString::new("aleo1recipient").unwrap();
    let mut addr: *mut AleoAddress = std::ptr::null_mut();
    aleo_address_from_string(addr_str.as_ptr(), &mut addr);

    let mut tx: *mut AleoTransaction = std::ptr::null_mut();
    let result = aleo_build_transfer(std::ptr::null(), addr, 100, &mut tx);
    assert_eq!(result, AleoErrorCode::InvalidInput);

    aleo_free_address(addr);
}

#[test]
fn test_build_transfer_null_recipient() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);

    let mut tx: *mut AleoTransaction = std::ptr::null_mut();
    let result = aleo_build_transfer(private_key, std::ptr::null(), 100, &mut tx);
    assert_eq!(result, AleoErrorCode::InvalidInput);

    aleo_free_private_key(private_key);
}

#[test]
fn test_build_transfer_null_out() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);

    let addr_str = std::ffi::CString::new("aleo1recipient").unwrap();
    let mut addr: *mut AleoAddress = std::ptr::null_mut();
    aleo_address_from_string(addr_str.as_ptr(), &mut addr);

    let result = aleo_build_transfer(private_key, addr, 100, std::ptr::null_mut());
    assert_eq!(result, AleoErrorCode::InvalidInput);

    aleo_free_address(addr);
    aleo_free_private_key(private_key);
}

#[test]
fn test_sign_transaction_null_private_key() {
    let tx_str = std::ffi::CString::new("tx_data").unwrap();
    let mut out: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_sign_transaction(std::ptr::null(), tx_str.as_ptr(), &mut out);
    assert_eq!(result, AleoErrorCode::InvalidInput);
}

#[test]
fn test_sign_transaction_null_transaction() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);

    let mut out: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_sign_transaction(private_key, std::ptr::null(), &mut out);
    assert_eq!(result, AleoErrorCode::InvalidInput);

    aleo_free_private_key(private_key);
}

#[test]
fn test_sign_transaction_null_out() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);

    let tx_str = std::ffi::CString::new("tx_data").unwrap();
    let result = aleo_sign_transaction(private_key, tx_str.as_ptr(), std::ptr::null_mut());
    assert_eq!(result, AleoErrorCode::InvalidInput);

    aleo_free_private_key(private_key);
}

#[test]
fn test_sign_transaction_success() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);

    let tx_data = "my_transaction";
    let tx_str = std::ffi::CString::new(tx_data).unwrap();
    let mut out: *mut std::os::raw::c_char = std::ptr::null_mut();
    let result = aleo_sign_transaction(private_key, tx_str.as_ptr(), &mut out);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!out.is_null());

    let signed = unsafe {
        std::ffi::CStr::from_ptr(out)
            .to_string_lossy()
            .into_owned()
    };
    assert_eq!(signed, format!("signed:{tx_data}"));

    aleo_free_string(out);
    aleo_free_private_key(private_key);
}

#[test]
fn test_free_transaction_null() {
    aleo_free_transaction(std::ptr::null_mut());
}

#[test]
fn test_free_transaction_default() {
    let tx = Box::into_raw(Box::new(AleoTransaction::default()));
    aleo_free_transaction(tx);
}

#[test]
fn test_free_transaction_valid() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    aleo_generate_private_key(&mut private_key);

    let addr_str = std::ffi::CString::new("aleo1recipient").unwrap();
    let mut addr: *mut AleoAddress = std::ptr::null_mut();
    aleo_address_from_string(addr_str.as_ptr(), &mut addr);

    let mut tx: *mut AleoTransaction = std::ptr::null_mut();
    let result = aleo_build_transfer(private_key, addr, 500, &mut tx);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!tx.is_null());

    aleo_free_transaction(tx);
    aleo_free_address(addr);
    aleo_free_private_key(private_key);
}

#[test]
fn test_free_private_key_null() {
    aleo_free_private_key(std::ptr::null_mut());
}

#[test]
fn test_free_private_key_valid() {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    let result = aleo_generate_private_key(&mut private_key);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!private_key.is_null());
    aleo_free_private_key(private_key);
}
