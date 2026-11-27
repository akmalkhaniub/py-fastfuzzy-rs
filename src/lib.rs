#![allow(clippy::useless_conversion)]

pub mod distance;

use distance::{fuzzy_ratio, jaro_winkler_similarity, levenshtein_distance, parallel_fuzzy_search};
use pyo3::prelude::*;

/// Computes the Levenshtein edit distance between two strings.
#[pyfunction]
fn levenshtein(s1: &str, s2: &str) -> usize {
    levenshtein_distance(s1, s2)
}

/// Computes the Jaro-Winkler similarity score (0.0 to 1.0) between two strings.
#[pyfunction]
fn jaro_winkler(s1: &str, s2: &str) -> f64 {
    jaro_winkler_similarity(s1, s2)
}

/// Computes normalized fuzzy match ratio (0.0 to 1.0).
#[pyfunction]
fn ratio(s1: &str, s2: &str) -> f64 {
    fuzzy_ratio(s1, s2)
}

/// Performs multi-threaded fuzzy candidate matching across large lists, releasing the Python GIL.
#[pyfunction]
#[pyo3(signature = (query, candidates, top_k = None, threshold = None))]
fn fuzzy_search(
    py: Python<'_>,
    query: &str,
    candidates: Vec<String>,
    top_k: Option<usize>,
    threshold: Option<f64>,
) -> PyResult<Vec<(String, f64)>> {
    let k = top_k.unwrap_or(10);
    let thresh = threshold.unwrap_or(0.0);

    // Release the Python GIL to allow true multi-core parallel execution across threads
    let results = py.allow_threads(|| {
        parallel_fuzzy_search(query, &candidates, k, thresh)
    });

    let py_results = results
        .into_iter()
        .map(|r| (r.candidate, r.score))
        .collect();

    Ok(py_results)
}

/// Native Python module definition.
#[pymodule]
fn py_fastfuzzy_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(levenshtein, m)?)?;
    m.add_function(wrap_pyfunction!(jaro_winkler, m)?)?;
    m.add_function(wrap_pyfunction!(ratio, m)?)?;
    m.add_function(wrap_pyfunction!(fuzzy_search, m)?)?;
    Ok(())
}
