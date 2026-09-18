# `datafusion::test::object_store::ensure_head_concurrency`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test.object_store.ensure_head_concurrency.json).

<a id="op-608ebf75c7654fa6a544c6eb"></a>
## ensure_head_concurrency

`function` · `datafusion::test::object_store::ensure_head_concurrency` · datafusion 55.1.0

```rust
fn ensure_head_concurrency(object_store: std::sync::Arc<dyn ObjectStore>, concurrency: usize) -> std::sync::Arc<dyn ObjectStore>
```

Source: `src/test/object_store.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Blocks the object_store `head` call until `concurrency` number of calls are pending.
