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
#[path = "token_tests.rs"]
mod token_tests;
