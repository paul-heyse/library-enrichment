# `ruff_server::server::connection`

Crate `ruff_server` · 1 public items · structured records in [`model/ruff_server.server.connection.json`](../model/ruff_server.server.connection.json)

## ConnectionInitializer

`struct` · `ruff_server::server::connection::ConnectionInitializer`

Also reachable as `ruff_server::ConnectionInitializer`

```rust
struct ConnectionInitializer
```

**Methods** (1)

```rust
fn memory() -> (Self, lsp::Connection)
```

A builder for `Connection` that handles LSP initialization.

---
