# `datafusion::test::object_store::make_test_store_and_state`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test.object_store.make_test_store_and_state.json).

<a id="op-f9b1074ad885f4f853c521f0"></a>
## make_test_store_and_state

`function` · `datafusion::test::object_store::make_test_store_and_state` · datafusion 55.1.0

```rust
fn make_test_store_and_state(files: &[(&str, u64)]) -> (std::sync::Arc<object_store::memory::InMemory>, execution::context::SessionState)
```

Source: `src/test/object_store.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a test object store with the provided files
