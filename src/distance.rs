use std::cmp::{max, min};
use rayon::prelude::*;

/// Computes the Levenshtein distance between two strings using single-row dynamic programming.
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let len1 = s1_chars.len();
    let len2 = s2_chars.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    // Work on the shorter string to minimize vector allocation
    let (src, tgt) = if len1 < len2 {
        (&s1_chars, &s2_chars)
    } else {
        (&s2_chars, &s1_chars)
    };

    let mut prev_row: Vec<usize> = (0..=src.len()).collect();
    let mut curr_row: Vec<usize> = vec![0; src.len() + 1];

    for (j, &c_tgt) in tgt.iter().enumerate() {
        curr_row[0] = j + 1;

        for (i, &c_src) in src.iter().enumerate() {
            let cost = if c_src == c_tgt { 0 } else { 1 };
            curr_row[i + 1] = min(
                curr_row[i] + 1, // Insertion
                min(
                    prev_row[i + 1] + 1, // Deletion
                    prev_row[i] + cost,  // Substitution
                ),
            );
        }
        prev_row.copy_from_slice(&curr_row);
    }

    prev_row[src.len()]
}

/// Computes Jaro-Winkler similarity (range 0.0 to 1.0).
pub fn jaro_winkler_similarity(s1: &str, s2: &str) -> f64 {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let len1 = s1_chars.len();
    let len2 = s2_chars.len();

    if len1 == 0 && len2 == 0 {
        return 1.0;
    }
    if len1 == 0 || len2 == 0 {
        return 0.0;
    }

    let match_window = max(len1, len2) / 2;
    if match_window == 0 {
        return if s1_chars[0] == s2_chars[0] { 1.0 } else { 0.0 };
    }

    let mut s1_matches = vec![false; len1];
    let mut s2_matches = vec![false; len2];
    let mut matches = 0;

    for i in 0..len1 {
        let start = i.saturating_sub(match_window);
        let end = min(i + match_window + 1, len2);

        for j in start..end {
            if s2_matches[j] || s1_chars[i] != s2_chars[j] {
                continue;
            }
            s1_matches[i] = true;
            s2_matches[j] = true;
            matches += 1;
            break;
        }
    }

    if matches == 0 {
        return 0.0;
    }

    // Count transpositions
    let mut k = 0;
    let mut transpositions = 0;
    for i in 0..len1 {
        if !s1_matches[i] {
            continue;
        }
        while !s2_matches[k] {
            k += 1;
        }
        if s1_chars[i] != s2_chars[k] {
            transpositions += 1;
        }
        k += 1;
    }

    let m = matches as f64;
    let jaro = (m / len1 as f64 + m / len2 as f64 + (m - (transpositions as f64 / 2.0)) / m) / 3.0;

    // Common prefix up to 4 characters
    let mut prefix_len = 0;
    for (c1, c2) in s1_chars.iter().zip(s2_chars.iter()).take(4) {
        if c1 == c2 {
            prefix_len += 1;
        } else {
            break;
        }
    }

    let p = 0.1; // standard scaling factor
    jaro + (prefix_len as f64 * p * (1.0 - jaro))
}

/// Normalized fuzzy ratio score (0.0 to 1.0) based on Levenshtein distance.
pub fn fuzzy_ratio(s1: &str, s2: &str) -> f64 {
    let max_len = max(s1.chars().count(), s2.chars().count());
    if max_len == 0 {
        return 1.0;
    }
    let dist = levenshtein_distance(s1, s2);
    1.0 - (dist as f64 / max_len as f64)
}

/// Match candidate item with similarity score.
#[derive(Debug, Clone)]
pub struct MatchResult {
    pub candidate: String,
    pub score: f64,
}

/// Executes parallel multi-threaded fuzzy search across a slice of candidate strings using Rayon.
pub fn parallel_fuzzy_search(
    query: &str,
    candidates: &[String],
    top_k: usize,
    threshold: f64,
) -> Vec<MatchResult> {
    let mut matches: Vec<MatchResult> = candidates
        .par_iter()
        .filter_map(|cand| {
            let score = jaro_winkler_similarity(query, cand);
            if score >= threshold {
                Some(MatchResult {
                    candidate: cand.clone(),
                    score,
                })
            } else {
                None
            }
        })
        .collect();

    // Sort descending by score
    matches.par_sort_unstable_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    if matches.len() > top_k {
        matches.truncate(top_k);
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("flaw", "lawn"), 2);
        assert_eq!(levenshtein_distance("", "hello"), 5);
        assert_eq!(levenshtein_distance("rust", "rust"), 0);
    }

    #[test]
    fn test_jaro_winkler() {
        let score = jaro_winkler_similarity("dwayne", "duane");
        assert!((score - 0.84).abs() < 0.05);

        let exact = jaro_winkler_similarity("python", "python");
        assert_eq!(exact, 1.0);
    }

    #[test]
    fn test_parallel_fuzzy_search() {
        let candidates = vec![
            "apple".to_string(),
            "apply".to_string(),
            "banana".to_string(),
            "application".to_string(),
            "applesauce".to_string(),
        ];

        let results = parallel_fuzzy_search("apple", &candidates, 3, 0.7);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].candidate, "apple");
        assert_eq!(results[0].score, 1.0);
    }
}
