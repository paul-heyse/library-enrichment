# `deltalake_core::operations::generate`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.operations.generate.json`](../model/deltalake_core.operations.generate.json)

## GenerateBuilder

`struct` · `deltalake_core::operations::generate::GenerateBuilder`

Also reachable as `deltalake::operations::generate::GenerateBuilder`

```rust
struct GenerateBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Derives**: Clone

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

**via `deltalake_core::operations::Operation`**

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
fn log_store(&self) -> &LogStoreRef
```

Simple builder to generate the manifest

---
