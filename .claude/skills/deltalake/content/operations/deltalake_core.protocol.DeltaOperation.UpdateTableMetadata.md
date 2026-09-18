# `deltalake_core::protocol::DeltaOperation::UpdateTableMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.DeltaOperation.UpdateTableMetadata.json).

<a id="op-284cc2829b452b341a6dab98"></a>
## metadata_update

`struct_field` · `deltalake_core::protocol::DeltaOperation::UpdateTableMetadata::metadata_update` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
metadata_update: operations::update_table_metadata::TableMetadataUpdate
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L379).

Source: `crates/core/src/protocol/mod.rs:379`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The metadata update to apply
