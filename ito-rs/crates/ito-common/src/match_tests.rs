use super::*;

#[test]
fn levenshtein_matches_ts_examples() {
    assert_eq!(levenshtein("kitten", "sitting"), 3);
    assert_eq!(levenshtein("", "a"), 1);
    assert_eq!(levenshtein("a", ""), 1);
    assert_eq!(levenshtein("a", "a"), 0);
}

#[test]
fn nearest_matches_is_stable_on_ties() {
    let candidates = vec!["aa".to_string(), "ab".to_string(), "ac".to_string()];
    let out = nearest_matches("a", &candidates, 3);
    assert_eq!(out, vec!["aa", "ab", "ac"]);
}
