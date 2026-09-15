# `datafusion::test::object_store`

Crate `datafusion` · 4 public items · structured records in [`model/datafusion.test.object_store.json`](../model/datafusion.test.object_store.json)

## ensure_head_concurrency

`function` · `datafusion::test::object_store::ensure_head_concurrency`

```rust
fn ensure_head_concurrency(object_store: std::sync::Arc<dyn ObjectStore>, concurrency: usize) -> std::sync::Arc<dyn ObjectStore>
```

Blocks the object_store `head` call until `concurrency` number of calls are pending.

---

## local_unpartitioned_file

`function` · `datafusion::test::object_store::local_unpartitioned_file`

```rust
fn local_unpartitioned_file(path: impl AsRef<std::path::Path>) -> object_store::ObjectMeta
```

Helper method to fetch the file size and date at given path and create a `ObjectMeta`

---

## make_test_store_and_state

`function` · `datafusion::test::object_store::make_test_store_and_state`

```rust
fn make_test_store_and_state(files: &[(&str, u64)]) -> (std::sync::Arc<object_store::memory::InMemory>, execution::context::SessionState)
```

Create a test object store with the provided files

---

## register_test_store

`function` · `datafusion::test::object_store::register_test_store`

```rust
fn register_test_store(ctx: &prelude::SessionContext, files: &[(&str, u64)])
```

Registers a test object store with the provided `ctx`

---
