# `pyrefly_util::fs_anyhow`

Crate `pyrefly_util` · 5 public items · structured records in [`model/pyrefly_util.fs_anyhow.json`](../model/pyrefly_util.fs_anyhow.json)

## create_dir_all

`function` · `pyrefly_util::fs_anyhow::create_dir_all`

```rust
fn create_dir_all(path: &std::path::Path) -> anyhow::Result<()>
```

---

## read

`function` · `pyrefly_util::fs_anyhow::read`

```rust
fn read(path: &std::path::Path) -> anyhow::Result<Vec<u8>>
```

---

## read_dir

`function` · `pyrefly_util::fs_anyhow::read_dir`

```rust
fn read_dir(path: &std::path::Path) -> anyhow::Result<std::fs::ReadDir>
```

---

## read_to_string

`function` · `pyrefly_util::fs_anyhow::read_to_string`

```rust
fn read_to_string(path: &std::path::Path) -> anyhow::Result<String>
```

---

## write

`function` · `pyrefly_util::fs_anyhow::write`

```rust
fn write(path: &std::path::Path, contents: impl AsRef<[u8]>) -> Result<(), anyhow::Error>
```

---
