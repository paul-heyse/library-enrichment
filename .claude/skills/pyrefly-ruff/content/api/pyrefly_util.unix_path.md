# `pyrefly_util::unix_path`

Crate `pyrefly_util` · 2 public items · structured records in [`model/pyrefly_util.unix_path.json`](../model/pyrefly_util.unix_path.json)

## path_to_unix_string

`function` · `pyrefly_util::unix_path::path_to_unix_string`

```rust
fn path_to_unix_string(path: &std::path::Path) -> String
```

Convert a Path to a unix string for use in pretty printing/error emission, converting all
(possibly Windows) separators to `/`.

---

## str_path_to_unix_string

`function` · `pyrefly_util::unix_path::str_path_to_unix_string`

```rust
fn str_path_to_unix_string(path_str: String) -> String
```

Convert a stringified Path to a unix string for use in pretty printing/error emission,
converting all (possibly Windows) separators to `/`.

---
