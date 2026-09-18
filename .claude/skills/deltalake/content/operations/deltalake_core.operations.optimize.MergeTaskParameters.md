# `deltalake_core::operations::optimize::MergeTaskParameters`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.optimize.MergeTaskParameters.json).

<a id="op-683d187cc54b0896cb906591"></a>
## MergeTaskParameters

`struct` · `deltalake_core::operations::optimize::MergeTaskParameters` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct MergeTaskParameters
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L603).

Source: `crates/core/src/operations/optimize.rs:603`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parameters passed to individual merge tasks

<a id="op-47fd4c5fa735239a00ed35bb"></a>
## fmt

`function` · `deltalake_core::operations::optimize::MergeTaskParameters::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L602).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::MergeTaskParameters", "path": "MergeTaskParameters"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [602, 10], "end": [602, 15], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/optimize.rs:602`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95a02fc199b8800e7e3f64e9"></a>
## file_schema

`struct_field` · `deltalake_core::operations::optimize::MergeTaskParameters::file_schema` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_schema: arrow::datatypes::SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L605).

Source: `crates/core/src/operations/optimize.rs:605`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Schema of written files

<a id="op-63283e0fa38a69da256710d3"></a>
## input_parameters

`struct_field` · `deltalake_core::operations::optimize::MergeTaskParameters::input_parameters` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
input_parameters: OptimizeInput
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L609).

Source: `crates/core/src/operations/optimize.rs:609`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Input parameters for the optimize operation

<a id="op-5bb915d55d904b21988a8804"></a>
## num_indexed_cols

`struct_field` · `deltalake_core::operations::optimize::MergeTaskParameters::num_indexed_cols` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
num_indexed_cols: delta_kernel::table_properties::DataSkippingNumIndexedCols
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L611).

Source: `crates/core/src/operations/optimize.rs:611`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Num index cols to collect stats for

<a id="op-b3da7b8532b8c0b01133a4eb"></a>
## stats_columns

`struct_field` · `deltalake_core::operations::optimize::MergeTaskParameters::stats_columns` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
stats_columns: Option<Vec<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L613).

Source: `crates/core/src/operations/optimize.rs:613`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Stats columns, specific columns to collect stats from, takes precedence over num_indexed_cols

<a id="op-b982c16a0aef3072ba7b3c4a"></a>
## writer_properties

`struct_field` · `deltalake_core::operations::optimize::MergeTaskParameters::writer_properties` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
writer_properties: parquet::file::properties::WriterProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L607).

Source: `crates/core/src/operations/optimize.rs:607`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Properties passed to parquet writer
