//! Cryptographic token generation for backend server authentication.
//!
//! Produces URL-safe base64-encoded random strings suitable for use as admin
//! tokens and HMAC token seeds.

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
    url_safe_base64_encode(&bytes)
}

/// Encode bytes as URL-safe base64 without padding.
fn url_safe_base64_encode(bytes: &[u8]) -> String {
    use std::fmt::Write;
    // URL-safe base64 alphabet: A-Z a-z 0-9 - _
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

    let mut result = String::with_capacity((bytes.len() * 4).div_ceil(3));

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };

        let triple = (b0 << 16) | (b1 << 8) | b2;

        let _ = result.write_char(ALPHABET[((triple >> 18) & 0x3F) as usize] as char);
        let _ = result.write_char(ALPHABET[((triple >> 12) & 0x3F) as usize] as char);

        if chunk.len() > 1 {
            let _ = result.write_char(ALPHABET[((triple >> 6) & 0x3F) as usize] as char);
        }
        if chunk.len() > 2 {
            let _ = result.write_char(ALPHABET[(triple & 0x3F) as usize] as char);
        }
    }

    result
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
        // All-zero bytes should produce "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
        let zeros = [0u8; 32];
        let encoded = url_safe_base64_encode(&zeros);
        assert_eq!(encoded.len(), 43);
        assert!(encoded.chars().all(|c| c == 'A'));
    }

    #[test]
    fn url_safe_base64_encode_known_vector() {
        // "Hello" -> base64 "SGVsbG8" (URL-safe, no padding)
        let encoded = url_safe_base64_encode(b"Hello");
        assert_eq!(encoded, "SGVsbG8");
    }
}
