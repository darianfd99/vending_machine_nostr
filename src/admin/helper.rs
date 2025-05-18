use nostr_sdk::{FromBech32, PublicKey};

/// Parses a public key from either a hex-encoded string or a Bech32 Nostr public key (`npub1...`).
///
/// This function attempts to parse the input as a Bech32-encoded public key first. If that fails,
/// it then checks if the input is a valid 64-character hex string.
///
/// # Arguments
/// * `input` - The public key string to parse, which can be either in Bech32 or hex format.
///
/// # Returns
/// - `Some(String)` if the public key is valid, represented as a normalized hex string.
pub fn parse_pubkey(input: &str) -> Option<nostr_sdk::PublicKey> {
    // Try Bech32 first
    if let Ok(pk) = PublicKey::from_bech32(input) {
        Some(pk)
    }
    // Then try 64-char hex
    else if let Ok(pk) = PublicKey::from_hex(input) {
        Some(pk)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use nostr_sdk::ToBech32;

    use super::*;
    #[test]
    fn test_parse_pubkey_bech32_valid() {
        let bech32 = "npub14cpg528xz69k3j25sq5ktm5ckyf798cyufyhj8vh8cqm6kp0s7js9x54c9"; // Dummy, valid format
        let parsed = parse_pubkey(bech32);
        assert!(parsed.is_some());
        assert_eq!(parsed.unwrap().to_bech32().unwrap(), bech32); // Should convert to hex
    }

    #[test]
    fn test_parse_pubkey_hex_valid() {
        let hex = "8c2fa6ac7b9f09d8d5ad52be317bf1f8eab428f3ffb3c15e0420be9e97f0d387";
        assert_eq!(parse_pubkey(hex).unwrap().to_hex(), hex.to_string());
    }

    #[test]
    fn test_parse_pubkey_invalid_format() {
        let invalid = "not_a_valid_key";
        assert_eq!(parse_pubkey(invalid), None);
    }
}
