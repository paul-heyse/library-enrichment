# `deltalake_core::kernel::schema::partitions::DeltaTablePartition`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.partitions.DeltaTablePartition.json).

<a id="op-10929185eee585c5a8cdb725"></a>
## DeltaTablePartition

`struct` · `deltalake_core::kernel::schema::partitions::DeltaTablePartition` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaTablePartition
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L16).

Source: `crates/core/src/kernel/schema/partitions.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A Struct DeltaTablePartition used to represent a partition of a DeltaTable.

<a id="op-8b44a84f364fdf1f297f26d5"></a>
## Error

`assoc_type` · `deltalake_core::kernel::schema::partitions::DeltaTablePartition::Error` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = DeltaTableError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::partitions::DeltaTablePartition", "path": "DeltaTablePartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [65, 2], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `crates/core/src/kernel/schema/partitions.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f93373e2a576e78924f3f96"></a>
## clone

`function` · `deltalake_core::kernel::schema::partitions::DeltaTablePartition::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeltaTablePartition
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L15).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::partitions::DeltaTablePartition", "path": "DeltaTablePartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 10], "end": [15, 15], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/schema/partitions.rs:15`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b63daeac8dd5a6401b3d98f1"></a>
## eq

`function` · `deltalake_core::kernel::schema::partitions::DeltaTablePartition::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &DeltaTablePartition) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L15).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::partitions::DeltaTablePartition", "path": "DeltaTablePartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 24], "end": [15, 33], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/schema/partitions.rs:15`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d83790a8078d18e1351e242a"></a>
## fmt

`function` · `deltalake_core::kernel::schema::partitions::DeltaTablePartition::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L15).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::partitions::DeltaTablePartition", "path": "DeltaTablePartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 17], "end": [15, 22], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/schema/partitions.rs:15`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb41d5309ab710d1286fe8d6"></a>
## from_partition_value

`function` · `deltalake_core::kernel::schema::partitions::DeltaTablePartition::from_partition_value` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_partition_value(partition_value: (&str, &Scalar)) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L27).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::partitions::DeltaTablePartition", "path": "DeltaTablePartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [34, 2], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/schema/partitions.rs:27`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a DeltaTable partition from a Tuple of (key, value).

<a id="op-953e65e98ef689189eee78ee"></a>
## key

`struct_field` · `deltalake_core::kernel::schema::partitions::DeltaTablePartition::key` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
key: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L18).

Source: `crates/core/src/kernel/schema/partitions.rs:18`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The key of the DeltaTable partition.

<a id="op-dd32c2c1637a8816ffa3f83f"></a>
## try_from

`function` · `deltalake_core::kernel::schema::partitions::DeltaTablePartition::try_from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(partition: &str) -> Result<Self, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L53).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::partitions::DeltaTablePartition", "path": "DeltaTablePartition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [65, 2], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `crates/core/src/kernel/schema/partitions.rs:53`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Try to create a DeltaTable partition from a HivePartition string.
Returns a DeltaTableError if the string is not in the form of a HivePartition.

<a id="op-f99f4bc58c4d19be407a9964"></a>
## value

`struct_field` · `deltalake_core::kernel::schema::partitions::DeltaTablePartition::value` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
value: delta_kernel::expressions::Scalar
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L20).

Source: `crates/core/src/kernel/schema/partitions.rs:20`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The value of the DeltaTable partition.
