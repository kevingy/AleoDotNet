// Address derivation logic
//
// Implements address derivation and parsing using snarkVM's Aleo account types.

use crate::core::AleoError;
use snarkvm::console::account::{Address as SnarkAddress, ViewKey as SnarkViewKey};
use snarkvm::console::network::Testnet3;
use std::str::FromStr;
use std::convert::TryFrom;

/// Represents an Aleo address (Testnet3)
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    inner: SnarkAddress<Testnet3>,
}

impl Address {
    /// Derive an address from a view key
    ///
    /// Uses snarkVM's `Address::try_from(ViewKey)` for Testnet3.
    pub fn from_view_key(view_key: &SnarkViewKey<Testnet3>) -> Result<Self, AleoError> {
        // snarkVM implements TryFrom<&ViewKey<N>> for Address<N>, so we can pass a reference.
        match SnarkAddress::<Testnet3>::try_from(view_key) {
            Ok(addr) => Ok(Address { inner: addr }),
            Err(e) => Err(AleoError::CryptoError(format!(
                "Address derivation failed: {e}"
            ))),
        }
    }

    /// Parse an address from a bech32 string
    ///
    /// Uses snarkVM's `Address::from_str` for Testnet3.
    pub fn from_string(s: &str) -> Result<Self, AleoError> {
        match SnarkAddress::<Testnet3>::from_str(s) {
            Ok(addr) => Ok(Address { inner: addr }),
            Err(e) => Err(AleoError::CryptoError(format!(
                "Invalid address: {e}"
            ))),
        }
    }

    /// Convert address to bech32 string
    ///
    /// Uses snarkVM's `Display`/`ToString` implementation.
    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }
}
