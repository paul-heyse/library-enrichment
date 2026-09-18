# `deltalake_core::operations::load_cdf::CdfLoadBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.load_cdf.CdfLoadBuilder.json).

<a id="op-f0e83221d70bc885239ff83c"></a>
## CdfLoadBuilder

`struct` · `deltalake_core::operations::load_cdf::CdfLoadBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CdfLoadBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L53).

Source: `crates/core/src/operations/load_cdf.rs:53`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Builder for create a read of change data feeds for delta tables

<a id="op-c42fdd7acb1de4224cf941c2"></a>
## build

`function` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::build` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn build(&self, session: &dyn Session, filters: Option<&Arc<dyn PhysicalExpr>>) -> DeltaResult<Arc<dyn ExecutionPlan>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L534).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load_cdf::CdfLoadBuilder", "path": "CdfLoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [704, 2], "filename": "crates/core/src/operations/load_cdf.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/load_cdf.rs:534`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Executes the scan with no metric set

<a id="op-52940ee116097799a600f742"></a>
## build_with_metrics

`function` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::build_with_metrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn build_with_metrics(&self, session: &dyn Session, filters: Option<&Arc<dyn PhysicalExpr>>, metrics: Option<ExecutionPlanMetricsSet>) -> DeltaResult<Arc<dyn ExecutionPlan>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L543).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load_cdf::CdfLoadBuilder", "path": "CdfLoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [704, 2], "filename": "crates/core/src/operations/load_cdf.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/load_cdf.rs:543`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Executes the scan with a metric set to report

<a id="op-a7858959445dac13ec936352"></a>
## clone

`function` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> CdfLoadBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L52).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load_cdf::CdfLoadBuilder", "path": "CdfLoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 10], "end": [52, 15], "filename": "crates/core/src/operations/load_cdf.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/operations/load_cdf.rs:52`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c424edf4f6bcd904cbaa6af1"></a>
## fmt

`function` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L77).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load_cdf::CdfLoadBuilder", "path": "CdfLoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [88, 2], "filename": "crates/core/src/operations/load_cdf.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/load_cdf.rs:77`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba2fd6c37af303a8f4036c61"></a>
## with_allow_out_of_range

`function` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::with_allow_out_of_range` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_allow_out_of_range(self) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L135).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load_cdf::CdfLoadBuilder", "path": "CdfLoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [704, 2], "filename": "crates/core/src/operations/load_cdf.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/load_cdf.rs:135`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Enable ending version or timestamp exceeding the last commit

<a id="op-b9a8b0f514736e593b2986ef"></a>
## with_ending_timestamp

`function` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::with_ending_timestamp` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_ending_timestamp(self, timestamp: DateTime<Utc>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L123).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load_cdf::CdfLoadBuilder", "path": "CdfLoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [704, 2], "filename": "crates/core/src/operations/load_cdf.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/load_cdf.rs:123`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Timestamp (inclusive) to end at

<a id="op-9459546417f032ba45df812d"></a>
## with_ending_version

`function` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::with_ending_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_ending_version(self, ending_version: Version) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L117).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load_cdf::CdfLoadBuilder", "path": "CdfLoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [704, 2], "filename": "crates/core/src/operations/load_cdf.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/load_cdf.rs:117`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Version (inclusive) to end at

<a id="op-971ca142fe4b3fd54fcce7d1"></a>
## with_starting_timestamp

`function` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::with_starting_timestamp` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_starting_timestamp(self, timestamp: DateTime<Utc>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L129).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load_cdf::CdfLoadBuilder", "path": "CdfLoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [704, 2], "filename": "crates/core/src/operations/load_cdf.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/load_cdf.rs:129`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Timestamp to start from

<a id="op-3915e51b02a4844bb40b6979"></a>
## with_starting_version

`function` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::with_starting_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_starting_version(self, starting_version: Version) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L111).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::load_cdf::CdfLoadBuilder", "path": "CdfLoadBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [704, 2], "filename": "crates/core/src/operations/load_cdf.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/load_cdf.rs:111`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Version to start at (version 0 if not provided)

<a id="op-625ca35657c885e07ff576b9"></a>
## allow_out_of_range

`struct_field` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::allow_out_of_range` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
allow_out_of_range: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L67).

Source: `crates/core/src/operations/load_cdf.rs:67`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Enable ending version or timestamp exceeding the last commit

<a id="op-16ef09e025fd30b937b7642b"></a>
## ending_timestamp

`struct_field` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::ending_timestamp` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
ending_timestamp: Option<chrono::DateTime<chrono::Utc>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L65).

Source: `crates/core/src/operations/load_cdf.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Ending timestamp of commits to accept

<a id="op-fe54c373de7ca5bbcb68e93f"></a>
## ending_version

`struct_field` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::ending_version` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
ending_version: Option<kernel::Version>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L61).

Source: `crates/core/src/operations/load_cdf.rs:61`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Version to stop reading at

<a id="op-5b5c5bcceec0ea551bddb527"></a>
## filter

`struct_field` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::filter` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
filter: Option<datafusion::logical_expr::Expr>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L72).

Source: `crates/core/src/operations/load_cdf.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Optional logical predicate used ONLY to prune files by their partition
values. This is never applied as a row-level filter, so any non-partition
conjuncts are ignored here and row-level correctness must be enforced by a
separate `FilterExec` wrapped around the resulting plan.

<a id="op-8a7caf49d7ba2370d3a9e8d5"></a>
## log_store

`struct_field` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L57).

Source: `crates/core/src/operations/load_cdf.rs:57`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta object store for handling data files

<a id="op-b41b1f6df74620e3f7460055"></a>
## parquet_metadata_cache

`struct_field` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::parquet_metadata_cache` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
parquet_metadata_cache: std::sync::Arc<datafusion::datasource::physical_plan::parquet::CachedParquetFileReaderFactory>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L73).

Source: `crates/core/src/operations/load_cdf.rs:73`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cad610d49abd79461869f684"></a>
## snapshot

`struct_field` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: Option<kernel::EagerSnapshot>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L55).

Source: `crates/core/src/operations/load_cdf.rs:55`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A snapshot of the to-be-loaded table's state

<a id="op-a0793cf3a9842c6b977e90e6"></a>
## starting_timestamp

`struct_field` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::starting_timestamp` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
starting_timestamp: Option<chrono::DateTime<chrono::Utc>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L63).

Source: `crates/core/src/operations/load_cdf.rs:63`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Starting timestamp of commits to accept

<a id="op-965b6b5e87c6d251def1c8ee"></a>
## starting_version

`struct_field` · `deltalake_core::operations::load_cdf::CdfLoadBuilder::starting_version` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
starting_version: Option<kernel::Version>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L59).

Source: `crates/core/src/operations/load_cdf.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Version to read from
