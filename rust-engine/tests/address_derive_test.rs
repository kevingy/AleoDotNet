// Address derivation FFI tests
//
// Tests cover:
// - Null pointer error handling
// - Invalid view key inputs
// - Successful address derivation via full key generation flow
// - Determinism (same view key → same address)
// - Uniqueness (different view keys → different addresses)
// - Address format verification

use aleo_dotnet_engine::{
    AleoAddress, AleoPrivateKey, AleoViewKey,
    aleo_derive_address, aleo_derive_view_key,
    aleo_free_address, aleo_free_private_key, aleo_free_view_key,
    aleo_generate_private_key,
    AleoErrorCode,
};

fn generate_valid_view_key() -> *mut AleoViewKey {
    let mut private_key: *mut AleoPrivateKey = std::ptr::null_mut();
    let result = aleo_generate_private_key(&mut private_key);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!private_key.is_null());

    let mut view_key: *mut AleoViewKey = std::ptr::null_mut();
    let result = aleo_derive_view_key(private_key, &mut view_key);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!view_key.is_null());

    aleo_free_private_key(private_key);
    view_key
}

fn address_to_string(addr: *mut AleoAddress) -> String {
    assert!(!addr.is_null());
    unsafe { (*addr).to_string() }
}

#[test]
fn test_derive_address_null_view_key() {
    let mut address: *mut AleoAddress = std::ptr::null_mut();
    let result = aleo_derive_address(std::ptr::null(), &mut address);
    assert_eq!(result, AleoErrorCode::InvalidInput);
    assert!(address.is_null());
}

#[test]
fn test_derive_address_null_out_address() {
    let view_key = AleoViewKey { bytes: [0u8; 32] };
    let result = aleo_derive_address(&view_key as *const AleoViewKey, std::ptr::null_mut());
    assert_eq!(result, AleoErrorCode::InvalidInput);
}

#[test]
fn test_derive_address_both_null() {
    let result = aleo_derive_address(std::ptr::null(), std::ptr::null_mut());
    assert_eq!(result, AleoErrorCode::InvalidInput);
}

#[test]
fn test_derive_address_success() {
    let view_key = generate_valid_view_key();

    let mut address: *mut AleoAddress = std::ptr::null_mut();
    let result = aleo_derive_address(view_key, &mut address);
    assert_eq!(result, AleoErrorCode::Success);
    assert!(!address.is_null());

    let addr_str = address_to_string(address);
    assert!(!addr_str.is_empty(), "Address string should not be empty");
    assert!(
        addr_str.starts_with("aleo1"),
        "Address should start with 'aleo1' prefix, got: {addr_str}"
    );

    aleo_free_address(address);
    aleo_free_view_key(view_key);
}

#[test]
fn test_derive_address_determinism() {
    let view_key = generate_valid_view_key();

    let mut address1: *mut AleoAddress = std::ptr::null_mut();
    let result1 = aleo_derive_address(view_key, &mut address1);
    assert_eq!(result1, AleoErrorCode::Success);

    let mut address2: *mut AleoAddress = std::ptr::null_mut();
    let result2 = aleo_derive_address(view_key, &mut address2);
    assert_eq!(result2, AleoErrorCode::Success);

    let addr1 = address_to_string(address1);
    let addr2 = address_to_string(address2);
    assert_eq!(addr1, addr2, "Same view key must produce identical address");

    aleo_free_address(address1);
    aleo_free_address(address2);
    aleo_free_view_key(view_key);
}

#[test]
fn test_derive_address_different_keys_produce_different_addresses() {
    let vk1 = generate_valid_view_key();
    let vk2 = generate_valid_view_key();

    let mut addr1: *mut AleoAddress = std::ptr::null_mut();
    aleo_derive_address(vk1, &mut addr1);

    let mut addr2: *mut AleoAddress = std::ptr::null_mut();
    aleo_derive_address(vk2, &mut addr2);

    let a1 = address_to_string(addr1);
    let a2 = address_to_string(addr2);
    assert_ne!(a1, a2, "Different view keys must produce different addresses");

    aleo_free_address(addr1);
    aleo_free_address(addr2);
    aleo_free_view_key(vk1);
    aleo_free_view_key(vk2);
}

#[test]
fn test_derive_address_invalid_view_key_bytes() {
    // Bytes representing a value >= BLS12-377 scalar field modulus (~253 bits)
    // should be rejected as an invalid view key.
    // Using [0xFF; 32] which is ~2^256 - 1, well above the modulus.
    let view_key = AleoViewKey { bytes: [0xFFu8; 32] };
    let mut address: *mut AleoAddress = std::ptr::null_mut();
    let result = aleo_derive_address(&view_key as *const AleoViewKey, &mut address);
    assert_eq!(result, AleoErrorCode::CryptoError);
    assert!(address.is_null(), "Address pointer should remain null on failure");
}
