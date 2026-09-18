# `datafusion::test::object_store::local_unpartitioned_file`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test.object_store.local_unpartitioned_file.json).

<a id="op-e4c8b0729ff8ff21497fe8c1"></a>
## local_unpartitioned_file

`function` · `datafusion::test::object_store::local_unpartitioned_file` · datafusion 55.1.0

```rust
fn local_unpartitioned_file(path: impl AsRef<std::path::Path>) -> object_store::ObjectMeta
```

Source: `src/test/object_store.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Helper method to fetch the file size and date at given path and create a `ObjectMeta`
