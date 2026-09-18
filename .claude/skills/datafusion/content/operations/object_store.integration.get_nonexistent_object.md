# `object_store::integration::get_nonexistent_object`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.integration.get_nonexistent_object.json).

<a id="op-5507f75fe3678b12047d1808"></a>
## get_nonexistent_object

`function` · `object_store::integration::get_nonexistent_object` · object_store 0.13.2

```rust
async fn get_nonexistent_object(storage: &DynObjectStore, location: Option<path::Path>) -> Result<bytes::Bytes>
```

Source: `src/integration.rs:913`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Tests fetching a non-existent object returns a not found error
