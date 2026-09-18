# `deltalake_core::errors::DeltaTableError::InvalidPartitionFilter`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.InvalidPartitionFilter.json).

<a id="op-6cb92934e5e3a8711d546718"></a>
## partition_filter

`struct_field` · `deltalake_core::errors::DeltaTableError::InvalidPartitionFilter::partition_filter` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_filter: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L135).

Source: `crates/core/src/errors.rs:135`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The invalid partition filter used.
