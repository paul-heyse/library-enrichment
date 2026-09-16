# `pyrefly_util::events`

Crate `pyrefly_util` · 1 public items · structured records in [`model/pyrefly_util.events.json`](../model/pyrefly_util.events.json)

## CategorizedEvents

`struct` · `pyrefly_util::events::CategorizedEvents`

```rust
struct CategorizedEvents
```

**Fields**: `created`, `modified`, `removed`, `unknown`

**Derives**: Clone, Debug, Default

**Methods** (6)

```rust
fn extend(&mut self, other: CategorizedEvents)
fn is_empty(&self) -> bool
fn iter(&self) -> impl Iterator<Item = &PathBuf>
fn new_lsp(events: Vec<lsp_types::FileEvent>) -> CategorizedEvents
fn new_notify(events: Vec<notify::Event>) -> Self
fn should_ignore(path: &Path) -> bool
```

---
