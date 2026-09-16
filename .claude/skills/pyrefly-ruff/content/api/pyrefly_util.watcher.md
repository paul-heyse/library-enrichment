# `pyrefly_util::watcher`

Crate `pyrefly_util` · 1 public items · structured records in [`model/pyrefly_util.watcher.json`](../model/pyrefly_util.watcher.json)

## Watcher

`struct` · `pyrefly_util::watcher::Watcher`

```rust
struct Watcher
```

**Methods** (3)

```rust
fn notify(paths: &[PathBuf]) -> anyhow::Result<Self>
async fn wait(&mut self) -> anyhow::Result<Vec<Event>>
async fn watchman(path: &Path) -> anyhow::Result<Self>
```

---
