
# Depclean

Depclean is a Rust command-line tool to analyze and optimize project dependencies by parsing `Cargo.lock`, building dependency graphs, detecting duplicates, and suggesting version alignments.

## Installation

### Via Cargo
```bash
cargo install depclean
```


Verify installation:
```bash
depclean --help
```

### From Source
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/depclean.git
   cd depclean
   ```
2. Build and run:
   ```bash
   cargo build --release
   ./target/release/depclean --help
   ```

## Usage
Analyze dependencies in a Cargo project:
```bash
depclean --lockfile Cargo.lock
```

Output includes:
- Dependency graph visualization
- Duplicate dependency detection
- Version alignment suggestions

## Example Output
```plaintext
[+] Found 12 duplicate dependencies:
  - `serde` (versions: 1.0.152, 1.0.155)
  - `tokio` (versions: 1.25.0, 1.27.0)
[!] Suggested alignments:
  - Upgrade `serde` to 1.0.155 (used by 8 crates)
  - Unify `tokio` to 1.27.0 (compatible with all features)
```

## Contributing
1. Fork the repository
2. Create a feature/bugfix branch
3. Add tests for changes
4. Submit a PR with `[FEAT]`/`[FIX]`/`[DOCS]` prefix

## License
MIT License - See [LICENSE](LICENSE) for details.


