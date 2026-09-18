# `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.DeltaScanConfigBuilder.json).

<a id="op-3b07b2d250c05deafcf54dcc"></a>
## DeltaScanConfigBuilder

`struct` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaScanConfigBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L75).

Source: `crates/core/src/delta_datafusion/table_provider.rs:75`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Used to specify if additional metadata columns are exposed to the user

<a id="op-e8c097fbc2097fb164190c75"></a>
## build

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::build` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(&self, snapshot: &EagerSnapshot) -> DeltaResult<DeltaScanConfig>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L144).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder", "path": "DeltaScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [162, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:144`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build a DeltaScanConfig and ensure no column name conflicts occur during downstream processing

<a id="op-79848dd63cf274338da44f2b"></a>
## clone

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeltaScanConfigBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L73).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder", "path": "DeltaScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 17], "end": [73, 22], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:73`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39276665414dd098f84a27ea"></a>
## default

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L92).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder", "path": "DeltaScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [101, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:92`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef6b6a913af6a2e6ca5c75b9"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L73).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder", "path": "DeltaScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 10], "end": [73, 15], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:73`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b720fffa8f3b77c9cf1b144a"></a>
## new

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L105).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder", "path": "DeltaScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [162, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:105`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Construct a new instance of `DeltaScanConfigBuilder`

<a id="op-6aaec82ddbfacc6414de4504"></a>
## with_file_column

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::with_file_column` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_file_column(self, include: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L111).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder", "path": "DeltaScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [162, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:111`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Indicate that a column containing a records file path is included.
Column name is generated and can be determined once this Config is built

<a id="op-f12c36d616a3c5bc296a020f"></a>
## with_file_column_name

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::with_file_column_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_file_column_name<S: ToString>(self, name: &S) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L118).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder", "path": "DeltaScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [162, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:118`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Indicate that a column containing a records file path is included and column name is user defined.

<a id="op-8a885600a22ec8ebdf1fc2ac"></a>
## with_parquet_pushdown

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::with_parquet_pushdown` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_parquet_pushdown(self, pushdown: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L132).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder", "path": "DeltaScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [162, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:132`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Allow pushdown of the scan filter
When disabled the filter will only be used for pruning files

<a id="op-2ff6d59ea87f0c3b13c4ea97"></a>
## with_schema

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::with_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_schema(self, schema: SchemaRef) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L138).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder", "path": "DeltaScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [162, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:138`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Use the provided [SchemaRef] for the [`crate::delta_datafusion::DeltaScanNext`](../operations/deltalake_core.delta_datafusion.table_provider.next.DeltaScan.md#op-e35fcdd4ce5cf85731bf3f69)

Unresolved upstream links (retained, not inferred): `SchemaRef`.

<a id="op-03b5b4b822a794da931593f6"></a>
## wrap_partition_values

`function` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::wrap_partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn wrap_partition_values(self, wrap: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L125).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder", "path": "DeltaScanConfigBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [162, 2], "filename": "crates/core/src/delta_datafusion/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/delta_datafusion/table_provider.rs:125`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether to wrap partition values in a dictionary encoding

<a id="op-2abbfe86b847344460d54e59"></a>
## enable_parquet_pushdown

`struct_field` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::enable_parquet_pushdown` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
enable_parquet_pushdown: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L86).

Source: `crates/core/src/delta_datafusion/table_provider.rs:86`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether to push down filter in end result or just prune the files

<a id="op-b62609ebd24903f86866dc66"></a>
## file_column_name

`struct_field` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::file_column_name` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_column_name: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L82).

Source: `crates/core/src/delta_datafusion/table_provider.rs:82`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Column name that contains the source path.

If include_file_column is true and the name is None then it will be auto-generated
Otherwise the user provided name will be used

<a id="op-29d2a60dbbfd5f65864b6633"></a>
## include_file_column

`struct_field` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::include_file_column` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
include_file_column: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L77).

Source: `crates/core/src/delta_datafusion/table_provider.rs:77`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Include the source path for each record. The name of this column is determined by `file_column_name`

<a id="op-6ea71fd9337a9951e2e1a4ad"></a>
## schema

`struct_field` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::schema` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
schema: Option<arrow::datatypes::SchemaRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L88).

Source: `crates/core/src/delta_datafusion/table_provider.rs:88`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Schema to scan table with

<a id="op-fececb9a6c28f1ce92e6751e"></a>
## wrap_partition_values

`struct_field` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder::wrap_partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
wrap_partition_values: Option<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs#L84).

Source: `crates/core/src/delta_datafusion/table_provider.rs:84`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether to wrap partition values in a dictionary encoding to potentially save space
