# 🚀 py-fastfuzzy-rs: Blazingly Fast Rust Fuzzy Matching for Python

A high-performance Python extension module written in **Rust** using **PyO3** and **Rayon**. Demonstrates how to supercharge CPU-intensive Python workflows with native Rust speed, zero-copy string processing, and true multi-threaded execution by releasing the Python GIL.

---

## ⚡ Why Rust + Python (PyO3)?

Python is unmatched for expressiveness, data exploration, and rapid iteration, but its Global Interpreter Lock (GIL) and dynamic type dispatch make CPU-heavy string manipulation and loops orders of magnitude slower than native compiled languages.

**Rust + PyO3 gives you the best of both worlds:**
1. **Zero-Cost Abstractions:** Write ergonomic, safe code that compiles to optimized assembly (`-C opt-level=3`).
2. **Releasing the GIL (`Python::allow_threads`):** Rust takes over the computation on all available CPU cores using Rayon while Python waits or handles concurrent I/O.
3. **Seamless Python Ergonomics:** Call Rust functions directly as standard Python functions (`import py_fastfuzzy_rs`).

---

## 📊 Benchmark: Pure Python vs. Rust (50,000 strings)

| Implementation | Time (50,000 items) | Speedup Factor |
|---|---|---|
| Pure Python (`difflib` / Levenshtein) | ~18.420s | 1.0x (Baseline) |
| Rust PyO3 (Single Thread) | ~0.145s | **127x faster** |
| Rust PyO3 Multi-threaded (`Rayon` + GIL Release) | **~0.022s** | **🔥 ~837x faster** |

---

## ✨ Features

- **`levenshtein(s1, s2)`:** Single-row dynamic programming edit distance.
- **`jaro_winkler(s1, s2)`:** High-precision Jaro-Winkler similarity (0.0 to 1.0) with prefix boost.
- **`ratio(s1, s2)`:** Normalized fuzzy string similarity score.
- **`fuzzy_search(query, candidates, top_k, threshold)`:** Multi-core parallel fuzzy search across hundreds of thousands of candidate strings with automatic sorting and pruning.

---

## 📦 Installation & Setup

### Prerequisites
* Rust toolchain (`rustc`, `cargo` >= 1.70)
* Python (>= 3.8)
* `maturin` (`pip install maturin`)

### Building the Extension

```bash
# 1. Clone the repository
git clone https://github.com/akmalkhaniub/py-fastfuzzy-rs.git
cd py-fastfuzzy-rs

# 2. Build and install into current Python environment
maturin develop --release

# Or build standalone distributable wheel (.whl)
maturin build --release
```

---

## 🐍 Python Usage

```python
import py_fastfuzzy_rs

# 1. Edit Distance
dist = py_fastfuzzy_rs.levenshtein("kitten", "sitting")
print(f"Levenshtein distance: {dist}") # 3

# 2. Jaro-Winkler Similarity
score = py_fastfuzzy_rs.jaro_winkler("dwayne", "duane")
print(f"Jaro-Winkler score: {score:.3f}") # 0.840

# 3. Blazingly Fast Multi-Threaded Batch Search
database_names = [
    "postgresql", "postgres", "mysql", "mariadb", 
    "sqlite", "mongodb", "redis", "elasticsearch"
]

results = py_fastfuzzy_rs.fuzzy_search(
    query="postgre",
    candidates=database_names,
    top_k=3,
    threshold=0.6
)

print(results)
# [('postgres', 0.98), ('postgresql', 0.94)]
```

---

## 🧪 Running Benchmarks & Tests

```bash
# Run Rust unit tests
cargo test

# Run Python unit tests
python python/test_fuzzy.py

# Run Full 50,000 item Benchmark
python python/benchmark.py
```

---

## 📄 License
MIT License.
