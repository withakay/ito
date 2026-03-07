//! Cryptographic token generation for backend server authentication.
//!
//! Produces URL-safe base64-encoded random strings suitable for use as admin
//! tokens and HMAC token seeds.

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rand::Rng;

/// Minimum entropy in bytes for generated tokens.
const TOKEN_ENTROPY_BYTES: usize = 32;

/// Generate a cryptographically random URL-safe base64 token.
///
/// The token contains at least 32 bytes of entropy from the OS CSPRNG,
/// encoded as URL-safe base64 (no padding). This produces a 43-character
/// string.
pub fn generate_token() -> String {
    let mut bytes = [0u8; TOKEN_ENTROPY_BYTES];
    rand::rng().fill(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_token_has_expected_length() {
        let token = generate_token();
        // 32 bytes -> ceil(32 * 4 / 3) = 43 base64 chars (no padding)
        assert_eq!(token.len(), 43);
    }

    #[test]
    fn generated_token_is_url_safe() {
        let token = generate_token();
        for ch in token.chars() {
            assert!(
                ch.is_ascii_alphanumeric() || ch == '-' || ch == '_',
                "unexpected character in token: {ch}"
            );
        }
    }

    #[test]
    fn two_tokens_are_distinct() {
        let a = generate_token();
        let b = generate_token();
        assert_ne!(a, b, "two generated tokens should differ");
    }

    #[test]
    fn url_safe_base64_roundtrip_known_value() {
        // All-zero bytes should produce all 'A's
        let zeros = [0u8; 32];
        let encoded = URL_SAFE_NO_PAD.encode(zeros);
        assert_eq!(encoded.len(), 43);
        assert!(encoded.chars().all(|c| c == 'A'));
    }

    #[test]
    fn url_safe_base64_encode_known_vector() {
        // "Hello" -> base64 "SGVsbG8" (URL-safe, no padding)
        let encoded = URL_SAFE_NO_PAD.encode(b"Hello");
        assert_eq!(encoded, "SGVsbG8");
    }
}
