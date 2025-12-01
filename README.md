# WinGrep 🚀

##Right now it's still in progress and the only feature is for a plain text, but im working to add more features

A blazing fast, multi-threaded, recursive command-line search tool built in Rust.

Designed as a modern, cross-platform alternative to `grep` for Windows (and Linux/macOS), optimized for speed using memory mapping and parallel processing.

## ✨ Features

* **⚡ Zero-Copy Optimization:** Uses memory-mapping (`mmap`) to search files directly on disk without heavy RAM allocation.
* **🧵 Parallel Search:** Uses `rayon` to utilize all CPU cores when searching multiple files.
* **📂 Recursive Search:** Deep search functionality with the `-r` flag.
* **🎨 Colored Output:** Highlights matches and line numbers for better readability.
* **🪟 Cross-Platform:** Compiles to a single static binary (`.exe` on Windows).

## 📦 Installation

### From Source
You need to have [Rust installed](https://www.rust-lang.org/tools/install).

```bash
git clone [https://github.com/SoimanVasile/wingrep.git](https://github.com/SoimanVasile/wingrep.git)
cd wingrep
cargo build --release
```

The executable will be located in `target/release/wingrep` (or `wingrep.exe` on Windows).

## 🛠 Usage

```bash
# Basic search in a single file
wingrep "search_term" filename.txt

## Right now is not available about the directory and path, but im working on this
# Recursive search in a directory
wingrep -r "TODO" ./src

# Search with a specific path
wingrep "function" /path/to/project
```

## 📊 Benchmarking

To stress-test WinGrep against standard `grep`, we use a separate tool to generate massive datasets.

1.  **Get the Generator:**
    Clone the [Haystack Generator](https://github.com/SoimanVasile/haystack-generator.git) repository.
    ```bash
    git clone [https://github.com/SoimanVasile/haystack-generator.git](https://github.com/SoimanVasile/haystack-generator.git)
    ```

2.  **Generate Data:**
    Create a 70MB+ test file with hidden needles.
    ```bash
    cd haystack-generator
    python3 generate_data.py
    ```

3.  **Run the Benchmark:**
    Compare WinGrep against standard grep.
    ```bash
    # Linux / Git Bash
    time ../target/release/wingrep "NEEDLE" huge_test_file.txt > /dev/null

    # Windows PowerShell
    Measure-Command { .\wingrep.exe "NEEDLE" huge_test_file.txt | Out-Null }
    ```
