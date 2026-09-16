# `deltalake_core::operations::load`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.operations.load.json`](../model/deltalake_core.operations.load.json)

## LoadBuilder

`struct` · `deltalake_core::operations::load::LoadBuilder`

```rust
struct LoadBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn with_columns(self, columns: impl IntoIterator<Item = impl Into<String>>) -> Self
fn with_session_state(self, session: Arc<dyn Session>) -> Self
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

**via `deltalake_core::operations::Operation`**

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
fn log_store(&self) -> &LogStoreRef
```

---
