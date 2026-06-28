// Tests for the crypto-level Address type (src/crypto/address.rs)
//
// Covers:
// - from_string with valid and invalid inputs
// - to_string roundtrip
// - from_view_key with a valid view key
// - Address equality

use aleo_dotnet_engine::crypto::Address;
use snarkvm::console::account::{Address as SnarkAddress, PrivateKey, ViewKey};
use snarkvm::console::network::Testnet3;
use std::convert::TryFrom;

fn generate_valid_address_bech32() -> String {
    let rng = &mut rand::thread_rng();
    let pk = PrivateKey::<Testnet3>::new(rng).unwrap();
    let addr = SnarkAddress::<Testnet3>::try_from(&pk).unwrap();
    addr.to_string()
}

fn generate_valid_view_key() -> ViewKey<Testnet3> {
    let rng = &mut rand::thread_rng();
    let pk = PrivateKey::<Testnet3>::new(rng).unwrap();
    ViewKey::<Testnet3>::try_from(pk).unwrap()
}

#[test]
fn test_from_string_valid_address() {
    let bech32 = generate_valid_address_bech32();
    let addr = Address::from_string(&bech32);
    assert!(addr.is_ok(), "Valid address should parse successfully");
}

#[test]
fn test_from_string_roundtrip() {
    let bech32 = generate_valid_address_bech32();
    let addr = Address::from_string(&bech32).unwrap();
    assert_eq!(addr.to_string(), bech32, "to_string must return the original bech32 string");
}

#[test]
fn test_from_string_invalid_format() {
    let result = Address::from_string("not_an_aleo_address");
    assert!(result.is_err(), "Invalid format should return an error");
}

#[test]
fn test_from_string_empty() {
    let result = Address::from_string("");
    assert!(result.is_err(), "Empty string should return an error");
}

#[test]
fn test_from_string_random_gibberish() {
    let result = Address::from_string("aleo1???invalid");
    assert!(result.is_err(), "Gibberish should return an error");
}

#[test]
fn test_from_string_too_short() {
    let result = Address::from_string("aleo1");
    assert!(result.is_err(), "Truncated address should return an error");
}

#[test]
fn test_multiple_addresses_roundtrip() {
    let bech32_1 = generate_valid_address_bech32();
    let bech32_2 = generate_valid_address_bech32();

    let addr1 = Address::from_string(&bech32_1).unwrap();
    let addr2 = Address::from_string(&bech32_2).unwrap();

    assert_eq!(addr1.to_string(), bech32_1);
    assert_eq!(addr2.to_string(), bech32_2);
    assert_ne!(addr1.to_string(), addr2.to_string(), "Different addresses must differ");
}

#[test]
fn test_equality() {
    let bech32 = generate_valid_address_bech32();
    let addr1 = Address::from_string(&bech32).unwrap();
    let addr2 = Address::from_string(&bech32).unwrap();
    assert_eq!(addr1, addr2, "Same address parsed twice should be equal");
}

#[test]
fn test_inequality() {
    let bech32_1 = generate_valid_address_bech32();
    let bech32_2 = generate_valid_address_bech32();

    let addr1 = Address::from_string(&bech32_1).unwrap();
    let addr2 = Address::from_string(&bech32_2).unwrap();
    assert_ne!(addr1, addr2, "Different addresses should not be equal");
}

#[test]
fn test_from_view_key() {
    let vk = generate_valid_view_key();
    let addr = Address::from_view_key(&vk);
    assert!(addr.is_ok(), "Address derivation from valid view key should succeed");

    let expected = SnarkAddress::<Testnet3>::try_from(&vk).unwrap();
    assert_eq!(addr.unwrap().to_string(), expected.to_string());
}

#[test]
fn test_from_view_key_consistency() {
    let vk = generate_valid_view_key();
    let addr1 = Address::from_view_key(&vk).unwrap();
    let addr2 = Address::from_view_key(&vk).unwrap();
    assert_eq!(addr1, addr2, "Same view key must produce the same address");
}
