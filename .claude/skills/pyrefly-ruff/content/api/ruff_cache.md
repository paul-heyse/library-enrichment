# `ruff_cache`

Crate `ruff_cache` · 2 public items · structured records in [`model/ruff_cache.json`](../model/ruff_cache.json)

## CACHE_DIR_NAME

`constant` · `ruff_cache::CACHE_DIR_NAME`

```rust
const CACHE_DIR_NAME: &str = ".ruff_cache"
```

---

## cache_dir

`function` · `ruff_cache::cache_dir`

```rust
fn cache_dir(project_root: &std::path::Path) -> std::path::PathBuf
```

Return the cache directory for a given project root.

---
