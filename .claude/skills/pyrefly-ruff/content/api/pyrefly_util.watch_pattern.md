# `pyrefly_util::watch_pattern`

Crate `pyrefly_util` · 1 public items · structured records in [`model/pyrefly_util.watch_pattern.json`](../model/pyrefly_util.watch_pattern.json)

## WatchPattern

`enum` · `pyrefly_util::watch_pattern::WatchPattern`

```rust
enum WatchPattern
```

**Variants**: `File`, `Root`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn file(path: PathBuf) -> Self
fn root(root: InternedPath, pattern: String) -> Self
```

Some kind of pattern that can be used by filesystem watchers. Some filesystem watchers
expect the path to be separate from the pattern part, so this enables downstream logic
to handle pattern components as they need.

---
