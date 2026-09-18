# `deltalake_core::errors::DeltaTableError::ObjectStore`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.ObjectStore.json).

<a id="op-24ebdd006f05a2fc227849b7"></a>
## source

`struct_field` · `deltalake_core::errors::DeltaTableError::ObjectStore::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: object_store::Error
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L41).

Source: `crates/core/src/errors.rs:41`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Storage error details when reading the delta log object failed.
