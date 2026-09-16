# `ruff_linter::fs`

Crate `ruff_linter` · 3 public items · structured records in [`model/ruff_linter.fs.json`](../model/ruff_linter.fs.json)

## normalize_path

`function` · `ruff_linter::fs::normalize_path`

```rust
fn normalize_path<P: AsRef<std::path::Path>>(path: P) -> std::path::PathBuf
```

Convert any path to an absolute path (based on the current working
directory).

---

## normalize_path_to

`function` · `ruff_linter::fs::normalize_path_to`

```rust
fn normalize_path_to<P: AsRef<std::path::Path>, R: AsRef<std::path::Path>>(path: P, project_root: R) -> std::path::PathBuf
```

Convert any path to an absolute path (based on the specified project root).

---

## relativize_path

`function` · `ruff_linter::fs::relativize_path`

```rust
fn relativize_path<P: AsRef<std::path::Path>>(path: P) -> String
```

Convert an absolute path to be relative to the current working directory.

---
