//! Stellar address and contract ID validation.

use crate::types::error::{AddressError, PrismError, PrismResult};
use stellar_strkey::Strkey;

/// A validated Stellar address (G...) or contract ID (C...).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Address(String);

impl Address {
    /// Create a new Address from a string, validating its format and checksum.
    pub fn from_string(s: &str) -> PrismResult<Self> {
        match Strkey::from_string(s) {
            Ok(Strkey::PublicKeyEd25519(_)) | Ok(Strkey::Contract(_)) => Ok(Self(s.to_string())),
            Ok(_) => Err(PrismError::AddressError(AddressError::UnexpectedPrefix(
                s.chars().next().unwrap_or('?'),
            ))),
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("checksum") {
                    Err(PrismError::AddressError(AddressError::InvalidChecksum))
                } else {
                    Err(PrismError::AddressError(AddressError::InvalidFormat(err_str)))
                }
            }
        }
    }

    /// Convert to string.
    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    /// Get inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stellar_strkey::{Strkey, ed25519, Contract};

    #[test]
    fn test_address_from_string_valid_public_key() {
        // Generate a valid G-address (Account Edition)
        let raw_bytes = [0u8; 32];
        let pk = Strkey::PublicKeyEd25519(ed25519::PublicKey(raw_bytes)).to_string();
        
        // Now test the round-trip
        let address = Address::from_string(&pk).expect("Should parse valid generated key");
        assert_eq!(address.to_string(), pk);
    }

    #[test]
    fn test_address_from_string_valid_contract_id() {
        // Sample contract ID generation
        let raw_bytes = [1u8; 32];
        let pk = Strkey::Contract(Contract(raw_bytes)).to_string();
        let address = Address::from_string(&pk).expect("Failed to parse valid contract ID");
        assert_eq!(address.to_string(), pk);
    }

    #[test]
    fn test_address_from_string_invalid_checksum() {
        let pk = Strkey::PublicKeyEd25519(ed25519::PublicKey([0u8; 32])).to_string();
        
        // Break the string by changing the last character
        let mut corrupted = pk;
        let last_char = corrupted.pop().unwrap();
        let replacement = if last_char == 'A' { 'B' } else { 'A' };
        corrupted.push(replacement);
        
        assert!(Address::from_string(&corrupted).is_err());
    }
}
