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
