# `deltalake_core::errors::DeltaTableError::PartitionError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.PartitionError.json).

<a id="op-199f58039e86e20fe9966f19"></a>
## partition

`struct_field` · `deltalake_core::errors::DeltaTableError::PartitionError::partition` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L128).

Source: `crates/core/src/errors.rs:128`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The malformed partition used.
