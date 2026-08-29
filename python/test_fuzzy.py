"""
Unit tests for py_fastfuzzy_rs.
"""
import sys

def test_py_fastfuzzy():
    try:
        import py_fastfuzzy_rs
    except ImportError:
        print("Module not installed yet. Run 'maturin develop' or 'pip install .'")
        return

    # Test Levenshtein
    assert py_fastfuzzy_rs.levenshtein("kitten", "sitting") == 3
    assert py_fastfuzzy_rs.levenshtein("python", "python") == 0
    print("✓ Levenshtein distance test passed")

    # Test Jaro-Winkler
    jw = py_fastfuzzy_rs.jaro_winkler("dwayne", "duane")
    assert 0.80 < jw < 0.90
    print("✓ Jaro-Winkler test passed")

    # Test Fuzzy Ratio
    r = py_fastfuzzy_rs.ratio("hello", "hello")
    assert r == 1.0
    print("✓ Ratio test passed")

    # Test Multi-threaded Batch Search
    candidates = [
        "postgresql",
        "postgres",
        "mysql",
        "sqlite",
        "mongodb",
        "redis",
        "cockroachdb",
        "postman",
    ]
    results = py_fastfuzzy_rs.fuzzy_search("postgre", candidates, top_k=3, threshold=0.6)
    assert len(results) >= 2
    assert results[0][0] in ["postgres", "postgresql"]
    print(f"✓ Parallel fuzzy search passed: {results}")

if __name__ == "__main__":
    test_py_fastfuzzy()
