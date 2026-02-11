//! Fuzzy matching helpers.
//!
//! Used for "did you mean" suggestions in CLI error messages.

#[derive(Debug, Clone, PartialEq, Eq)]
/// A candidate string scored by edit distance.
pub struct ScoredCandidate {
    /// The candidate string.
    pub candidate: String,
    /// The edit distance to the input.
    pub distance: usize,
    /// The original index in the input list (used for stable tie-breaking).
    pub index: usize,
}

/// Return up to `max` candidates closest to `input`.
///
/// Matches are sorted by Levenshtein distance and are stable on ties.
pub fn nearest_matches(input: &str, candidates: &[String], max: usize) -> Vec<String> {
    let mut scored = Vec::new();
    for (idx, c) in candidates.iter().enumerate() {
        scored.push(ScoredCandidate {
            candidate: c.clone(),
            distance: levenshtein(input, c),
            index: idx,
        });
    }

    // Match JS behavior: sort by distance, stable on original order.
    scored.sort_by(|a, b| a.distance.cmp(&b.distance).then(a.index.cmp(&b.index)));

    let mut out = Vec::new();
    for (idx, candidate) in scored.into_iter().enumerate() {
        if idx >= max {
            break;
        }
        out.push(candidate.candidate);
    }
    out
}

/// Compute the Levenshtein edit distance between two strings.
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for (i, row) in dp.iter_mut().enumerate() {
        row[0] = i;
    }

    for (j, value) in dp[0].iter_mut().enumerate() {
        *value = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            let del = dp[i - 1][j] + 1;
            let ins = dp[i][j - 1] + 1;
            let sub = dp[i - 1][j - 1] + cost;
            dp[i][j] = del.min(ins).min(sub);
        }
    }

    dp[m][n]
}

#[cfg(test)]
mod tests {
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
        // All have distance 1; preserve original order.
        assert_eq!(out, vec!["aa", "ab", "ac"]);
    }
}
