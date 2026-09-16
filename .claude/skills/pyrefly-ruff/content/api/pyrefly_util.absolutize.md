# `pyrefly_util::absolutize`

Crate `pyrefly_util` · 1 public items · structured records in [`model/pyrefly_util.absolutize.json`](../model/pyrefly_util.absolutize.json)

## Absolutize

`trait` · `pyrefly_util::absolutize::Absolutize`

```rust
trait Absolutize
```

**Implementors** (1)

- `std::path::Path`

**Methods** (3)

```rust
fn absolutize(&self) -> PathBuf
fn absolutize_from(&self, base: &Path) -> PathBuf
fn relativize_from(&self, base: &Path) -> PathBuf
```

---
