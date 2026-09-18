# `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.DeltaScanConfig.json).

<a id="op-cabf2f35a8a66ba9825171fc"></a>
## DeltaScanConfig

`struct` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaScanConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L166).

Source: `crates/core/src/delta_datafusion/table_provider.rs:166`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Include additional metadata columns during a [`crate::delta_datafusion::DeltaScanNext`](../operations/deltalake_core.delta_datafusion.table_provider.next.DeltaScan.md#op-e35fcdd4ce5cf85731bf3f69)

<a id="op-ee466e444232908cfcb544e6"></a>
## clone

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeltaScanConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L164).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfig", "path": "DeltaScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [164, 17], "end": [164, 22], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:164`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-773e4dc93558017fa7a0105e"></a>
## default

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L181).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfig", "path": "DeltaScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 1], "end": [184, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:181`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da04a576250d6846f660ed37"></a>
## deserialize

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L164).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfig", "path": "DeltaScanConfig"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [164, 35], "end": [164, 46], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:164`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21ef381f2db5b36340390286"></a>
## enable_parquet_pushdown

`struct_field` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::enable_parquet_pushdown` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enable_parquet_pushdown: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L172).

Source: `crates/core/src/delta_datafusion/table_provider.rs:172`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Allow pushdown of the scan filter, defaults to true

<a id="op-c97b85ec6a38f1abd5b71165"></a>
## file_column_name

`struct_field` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::file_column_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
file_column_name: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L168).

Source: `crates/core/src/delta_datafusion/table_provider.rs:168`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Include the source path for each record

<a id="op-78da9fc368b814269c03c291"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L164).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfig", "path": "DeltaScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [164, 10], "end": [164, 15], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:164`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08f2519ebe6a7dbf44e9829a"></a>
## new

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L188).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfig", "path": "DeltaScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [238, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:188`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new default [`DeltaScanConfig`](../operations/deltalake_core.delta_datafusion.table_provider.DeltaScanConfig.md#op-cabf2f35a8a66ba9825171fc)

<a id="op-3ccff81e9c945462ba7451c0"></a>
## new_from_session

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::new_from_session` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_from_session(session: &dyn Session) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L200).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfig", "path": "DeltaScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [238, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:200`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a config seeded from an active DataFusion [`Session`], inheriting its
Parquet pushdown and view-type settings so the scan matches the session's behavior.

Unresolved upstream links (retained, not inferred): ``Session``.

<a id="op-3833313cc1f0e8054c635e78"></a>
## schema

`struct_field` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
schema: Option<arrow::datatypes::SchemaRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L177).

Source: `crates/core/src/delta_datafusion/table_provider.rs:177`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Schema to read as

<a id="op-39b1f25e114497b86cdbdfeb"></a>
## schema_force_view_types

`struct_field` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::schema_force_view_types` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
schema_force_view_types: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L175).

Source: `crates/core/src/delta_datafusion/table_provider.rs:175`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

If true, parquet reader will read columns of `Utf8`/`Utf8Large`
with Utf8View, and `Binary`/`BinaryLarge` with `BinaryView`

<a id="op-f602ba324e44a1404717f67a"></a>
## serialize

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L164).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfig", "path": "DeltaScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [164, 24], "end": [164, 33], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:164`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a68b4c7d9740b76ee11f84b"></a>
## with_file_column_name

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::with_file_column_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_file_column_name<S: ToString>(self, name: S) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L212).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfig", "path": "DeltaScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [238, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:212`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set the name of the synthetic column that exposes each row's source file path.

<a id="op-931818f3e6a7b7cafaeb0fc2"></a>
## with_parquet_pushdown

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::with_parquet_pushdown` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_parquet_pushdown(self, pushdown: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L224).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfig", "path": "DeltaScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [238, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:224`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Allow pushdown of the scan filter

<a id="op-4c795b3349c6132230c3d290"></a>
## with_schema

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::with_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_schema(self, schema: SchemaRef) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L234).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfig", "path": "DeltaScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [238, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:234`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Use the provided [SchemaRef] for the [`crate::delta_datafusion::DeltaScanNext`](../operations/deltalake_core.delta_datafusion.table_provider.next.DeltaScan.md#op-e35fcdd4ce5cf85731bf3f69)

This schema will be used when reading data from the underlying files.
The column names must match those in the table schema, but can have
different (yet compatible) types - e.g. string view types can be used

Unresolved upstream links (retained, not inferred): `SchemaRef`.

<a id="op-34a9519682f777eed6a71f2b"></a>
## with_wrap_partition_values

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::with_wrap_partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_wrap_partition_values(self, wrap: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L218).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfig", "path": "DeltaScanConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [238, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:218`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether to wrap partition values in a dictionary encoding

<a id="op-30c7524841b6a251f5d5269b"></a>
## wrap_partition_values

`struct_field` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig::wrap_partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
wrap_partition_values: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L170).

Source: `crates/core/src/delta_datafusion/table_provider.rs:170`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Wrap partition values in a dictionary encoding, defaults to true
