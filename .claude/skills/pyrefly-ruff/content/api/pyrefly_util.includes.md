# `pyrefly_util::includes`

Crate `pyrefly_util` · 1 public items · structured records in [`model/pyrefly_util.includes.json`](../model/pyrefly_util.includes.json)

## Includes

`trait` · `pyrefly_util::includes::Includes`

```rust
trait Includes
```

**Implementors** (1)

- `pyrefly_util::globs::FilteredGlobs`

**Methods** (5)

```rust
fn covers(&self, path: &Path) -> bool
fn covers_ignoring_excludes(&self, path: &Path) -> bool
fn errors(&mut self) -> Vec<anyhow::Error>
fn files_iter(&self) -> anyhow::Result<Box<dyn Iterator<Item = PathBuf> + '_>>
fn roots(&self) -> Vec<PathBuf>
```

---
