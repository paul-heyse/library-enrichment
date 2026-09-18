# `object_store::integration::list_with_offset_exclusivity`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.integration.list_with_offset_exclusivity.json).

<a id="op-023c3f642ea4c4ce9ca39337"></a>
## list_with_offset_exclusivity

`function` · `object_store::integration::list_with_offset_exclusivity` · object_store 0.13.2

```rust
async fn list_with_offset_exclusivity(storage: &DynObjectStore)
```

Source: `src/integration.rs:1272`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Tests that [`ObjectStore::list_with_offset`](../operations/object_store.ObjectStore.md#op-6c9f6dac06dc9bcb986fc957) returns an exclusive list
that does not include the offset value itself.
This is needed because some object stores (i.e. Azure) return inclusive results,
while AWS S3 and GCP return exclusive results.
