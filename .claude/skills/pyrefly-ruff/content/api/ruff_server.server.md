# `ruff_server::server`

Crate `ruff_server` · 1 public items · structured records in [`model/ruff_server.server.json`](../model/ruff_server.server.json)

## Server

`struct` · `ruff_server::server::Server`

Also reachable as `ruff_server::Server`

```rust
struct Server
```

**Methods** (2)

```rust
fn new(worker_threads: NonZeroUsize, connection: ConnectionInitializer, preview: Option<bool>, is_test: bool) -> anyhow::Result<Self>
fn run(self) -> anyhow::Result<()>
```

---
