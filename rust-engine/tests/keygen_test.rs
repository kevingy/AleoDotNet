// Test key generation FFI functions

use aleo_dotnet_engine::{
    AleoPrivateKey,
    aleo_free_private_key, aleo_generate_private_key,
    AleoErrorCode,
};

#[test]
fn test_generate_private_key() {
    let mut private_key_ptr: *mut AleoPrivateKey = std::ptr::null_mut();
    
    let result = aleo_generate_private_key(&mut private_key_ptr);
    
    assert_eq!(result, AleoErrorCode::Success, "Key generation should succeed");
    assert!(!private_key_ptr.is_null(), "Private key pointer should not be null");
    
    // Verify the key has some non-zero bytes (not all zeros)
    unsafe {
        let key = &*private_key_ptr;
        let has_non_zero = key.bytes.iter().any(|&b| b != 0);
        assert!(has_non_zero, "Generated key should have non-zero bytes");
    }
    
    // Clean up
    aleo_free_private_key(private_key_ptr);
}

#[test]
fn test_generate_private_key_null_pointer() {
    let result = aleo_generate_private_key(std::ptr::null_mut());
    
    assert_eq!(result, AleoErrorCode::InvalidInput, "Should return InvalidInput for null pointer");
}
