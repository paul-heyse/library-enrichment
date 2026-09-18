# `deltalake_core::kernel::schema::partitions::NULL_PARTITION_VALUE_DATA_PATH`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.partitions.NULL_PARTITION_VALUE_DATA_PATH.json).

<a id="op-1882fb0f5bd460f50b43f96e"></a>
## NULL_PARTITION_VALUE_DATA_PATH

`constant` · `deltalake_core::kernel::schema::partitions::NULL_PARTITION_VALUE_DATA_PATH` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const NULL_PARTITION_VALUE_DATA_PATH: &str = "__HIVE_DEFAULT_PARTITION__"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L12).

Source: `crates/core/src/kernel/schema/partitions.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A special value used in Hive to represent the null partition in partitioned tables
