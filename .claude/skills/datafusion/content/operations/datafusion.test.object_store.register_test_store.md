# `datafusion::test::object_store::register_test_store`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test.object_store.register_test_store.json).

<a id="op-be6c90ffbc64c744e01d7bf9"></a>
## register_test_store

`function` · `datafusion::test::object_store::register_test_store` · datafusion 55.1.0

```rust
fn register_test_store(ctx: &prelude::SessionContext, files: &[(&str, u64)])
```

Source: `src/test/object_store.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a test object store with the provided `ctx`
