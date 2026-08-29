"""
Performance benchmark: Pure Python vs Rust PyO3 (py_fastfuzzy_rs)
"""
import difflib
import random
import string
import time

def generate_random_words(count: int = 50_000) -> list[str]:
    vocab = ["database", "server", "postgres", "redis", "kubernetes", "docker", "pipeline", "cluster", "goroutine", "compiler"]
    words = []
    for _ in range(count):
        base = random.choice(vocab)
        # Introduce random typos / permutations
        if random.random() < 0.4:
            idx = random.randint(0, len(base) - 1)
            char = random.choice(string.ascii_lowercase)
            base = base[:idx] + char + base[idx+1:]
        words.append(base)
    return words

def py_levenshtein(s1: str, s2: str) -> int:
    if len(s1) < len(s2):
        return py_levenshtein(s2, s1)
    if len(s2) == 0:
        return len(s1)
    previous_row = range(len(s2) + 1)
    for i, c1 in enumerate(s1):
        current_row = [i + 1]
        for j, c2 in enumerate(s2):
            insertions = previous_row[j + 1] + 1
            deletions = current_row[j] + 1
            substitutions = previous_row[j] + (c1 != c2)
            current_row.append(min(insertions, deletions, substitutions))
        previous_row = current_row
    return previous_row[-1]

def run_benchmark():
    print("=" * 65)
    print("⚡ BENCHMARK: Pure Python vs Rust PyO3 (py_fastfuzzy_rs)")
    print("=" * 65)

    dataset_size = 50_000
    print(f"Generating dataset with {dataset_size:,} candidate strings...")
    candidates = generate_random_words(dataset_size)
    query = "postgress_db"

    try:
        import py_fastfuzzy_rs
    except ImportError:
        print("\n[!] py_fastfuzzy_rs is not installed yet.")
        print("    Build with: maturin develop --release")
        return

    # 1. Pure Python Benchmark (Sampled on 5,000 items to avoid taking minutes)
    sample_size = 5_000
    print(f"\n1. Running Pure Python Levenshtein on {sample_size:,} items (sampled)...")
    start_py = time.perf_counter()
    py_results = [py_levenshtein(query, w) for w in candidates[:sample_size]]
    py_duration = time.perf_counter() - start_py
    py_extrapolated = py_duration * (dataset_size / sample_size)
    print(f"   Python time ({sample_size:,} items): {py_duration:.3f}s")
    print(f"   Python extrapolated ({dataset_size:,} items): {py_extrapolated:.3f}s")

    # 2. Rust PyO3 Single-thread Levenshtein
    print(f"\n2. Running Rust Single-call Levenshtein on all {dataset_size:,} items...")
    start_rs_single = time.perf_counter()
    rs_single_results = [py_fastfuzzy_rs.levenshtein(query, w) for w in candidates]
    rs_single_duration = time.perf_counter() - start_rs_single
    print(f"   Rust single-call time: {rs_single_duration:.3f}s")

    # 3. Rust PyO3 Multi-threaded Batch Search (Rayon + GIL Release)
    print(f"\n3. Running Rust Multi-threaded Parallel Batch Search on {dataset_size:,} items...")
    start_rs_batch = time.perf_counter()
    matches = py_fastfuzzy_rs.fuzzy_search(query, candidates, top_k=5, threshold=0.7)
    rs_batch_duration = time.perf_counter() - start_rs_batch
    print(f"   Rust parallel batch time: {rs_batch_duration:.3f}s")
    print(f"   Top 5 Matches: {matches}")

    # Summary
    speedup = py_extrapolated / rs_batch_duration
    print("\n" + "=" * 65)
    print(f"🔥 FINAL RESULT: Rust is {speedup:.1f}x FASTER than Pure Python!")
    print("=" * 65)

if __name__ == "__main__":
    run_benchmark()
