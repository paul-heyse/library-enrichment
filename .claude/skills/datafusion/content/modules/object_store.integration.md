# `object_store::integration`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.integration.json).

<a id="op-af7b98e7884f23761a0b554d"></a>
## integration

`module` · `object_store::integration` · object_store 0.13.2

```rust
mod integration
```

Source: `src/integration.rs:18`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Integration tests for custom object store implementations

NB: These tests will delete everything present in the provided [`DynObjectStore`](../operations/object_store.DynObjectStore.md#op-defcd6fd5cb1e6bb62bb8a3f).

These tests are not a stable part of the public API and breaking changes may be made
in patch releases.

They are intended solely for testing purposes.
