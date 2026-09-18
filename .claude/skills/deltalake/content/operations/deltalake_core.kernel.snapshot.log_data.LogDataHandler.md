# `deltalake_core::kernel::snapshot::log_data::LogDataHandler`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.snapshot.log_data.LogDataHandler.json).

<a id="op-e8afbd2b8997b79f881a8888"></a>
## LogDataHandler

`struct` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LogDataHandler<'a>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L59).

Source: `crates/core/src/kernel/snapshot/log_data.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provides semanitc access to the log data.

This is a helper struct that provides access to the log data in a more semantic way
to avid the necessiity of knowing the exact layout of the underlying log data.

<a id="op-22b2b66919a79787c6a8c167"></a>
## IntoIter

`assoc_type` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::IntoIter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoIter = Box<dyn Iterator<Item = <LogDataHandler<'_> as IntoIterator>::Item>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L101).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "LogDataHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [109, 2], "filename": "crates/core/src/kernel/snapshot/log_data.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `crates/core/src/kernel/snapshot/log_data.rs:101`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-181b1e52986f00c4a805d67d"></a>
## Item

`assoc_type` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::Item` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Item = LogicalFileView
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L100).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "LogDataHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [109, 2], "filename": "crates/core/src/kernel/snapshot/log_data.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `crates/core/src/kernel/snapshot/log_data.rs:100`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d432e310487776a6b4efbb9"></a>
## clone

`function` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LogDataHandler<'a>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L58).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "LogDataHandler"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 10], "end": [58, 15], "filename": "crates/core/src/kernel/snapshot/log_data.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/snapshot/log_data.rs:58`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bbbcec664d14d3010772790"></a>
## contained

`function` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::contained` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn contained(&self, column: &Column, value: &HashSet<ScalarValue>) -> Option<BooleanArray>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L363).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "LogDataHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 5], "end": [428, 6], "filename": "crates/core/src/kernel/snapshot/log_data.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `crates/core/src/kernel/snapshot/log_data.rs:363`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d43d3b71cd686b81fa5f48ef"></a>
## input_schema

`function` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::input_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn input_schema(&self) -> ArrowSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L188).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "crate::kernel::LogDataHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [210, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::DataFusionMixins", "path": "DataFusionMixins"}, "trait_path": "deltalake_core::delta_datafusion::DataFusionMixins"}`

Source: `crates/core/src/delta_datafusion/mod.rs:188`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dadd50dc055ec82ce0368bd0"></a>
## into_iter

`function` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::into_iter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_iter(self) -> Self::IntoIter
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L103).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "LogDataHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [109, 2], "filename": "crates/core/src/kernel/snapshot/log_data.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `crates/core/src/kernel/snapshot/log_data.rs:103`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e947d128591bdab95024caa"></a>
## iter

`function` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::iter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn iter(&self) -> impl Iterator<Item = LogicalFileView> + '_
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L92).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "LogDataHandler"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [97, 2], "filename": "crates/core/src/kernel/snapshot/log_data.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/log_data.rs:92`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Iterate over each file as a [`LogicalFileView`](../operations/deltalake_core.kernel.snapshot.iterators.LogicalFileView.md#op-f5a43d85036c73510b6d2a44) without materializing all of them at once.

<a id="op-fe08ad38e167034438af755f"></a>
## max_values

`function` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::max_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn max_values(&self, column: &Column) -> Option<ArrayRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L290).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "LogDataHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 5], "end": [428, 6], "filename": "crates/core/src/kernel/snapshot/log_data.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `crates/core/src/kernel/snapshot/log_data.rs:290`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

return the maximum values for the named column, if known.
Note: the returned array must contain `num_containers()` rows.

<a id="op-b7964d643e72949922f3026f"></a>
## min_values

`function` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::min_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn min_values(&self, column: &Column) -> Option<ArrayRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L284).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "LogDataHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 5], "end": [428, 6], "filename": "crates/core/src/kernel/snapshot/log_data.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `crates/core/src/kernel/snapshot/log_data.rs:284`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

return the minimum values for the named column, if known.
Note: the returned array must contain `num_containers()` rows

<a id="op-64551b3a574e5bfa01b6339e"></a>
## null_counts

`function` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::null_counts` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn null_counts(&self, column: &Column) -> Option<ArrayRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L304).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "LogDataHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 5], "end": [428, 6], "filename": "crates/core/src/kernel/snapshot/log_data.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `crates/core/src/kernel/snapshot/log_data.rs:304`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

return the number of null values for the named column as an
`Option<UInt64Array>`.

Note: the returned array must contain `num_containers()` rows.

<a id="op-9dcb5fa0c68beaac0301d53e"></a>
## num_containers

`function` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::num_containers` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn num_containers(&self) -> usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L296).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "LogDataHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 5], "end": [428, 6], "filename": "crates/core/src/kernel/snapshot/log_data.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `crates/core/src/kernel/snapshot/log_data.rs:296`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

return the number of containers (e.g. row groups) being
pruned with these statistics

<a id="op-94038107ada896d5e45f3de1"></a>
## num_files

`function` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::num_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn num_files(&self) -> usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L87).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "LogDataHandler"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [97, 2], "filename": "crates/core/src/kernel/snapshot/log_data.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/snapshot/log_data.rs:87`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The number of files in the log data.
Returns the total number of files represented across all underlying record batches.

<a id="op-712aa6a504b330c41e4f787a"></a>
## parse_predicate_expression

`function` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::parse_predicate_expression` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_predicate_expression(&self, expr: impl AsRef<str>, session: &dyn Session) -> DeltaResult<Expr>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L202).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "crate::kernel::LogDataHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [210, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::DataFusionMixins", "path": "DataFusionMixins"}, "trait_path": "deltalake_core::delta_datafusion::DataFusionMixins"}`

Source: `crates/core/src/delta_datafusion/mod.rs:202`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2e8d438e565182706c77e92"></a>
## read_schema

`function` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::read_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_schema(&self) -> ArrowSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L174).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "crate::kernel::LogDataHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [210, 2], "filename": "crates/core/src/delta_datafusion/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::DataFusionMixins", "path": "DataFusionMixins"}, "trait_path": "deltalake_core::delta_datafusion::DataFusionMixins"}`

Source: `crates/core/src/delta_datafusion/mod.rs:174`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57aee55e08b2b59a77797d01"></a>
## row_counts

`function` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::row_counts` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn row_counts(&self) -> Option<ArrayRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L332).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "deltalake_core::kernel::snapshot::log_data::LogDataHandler", "path": "LogDataHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 5], "end": [428, 6], "filename": "crates/core/src/kernel/snapshot/log_data.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `crates/core/src/kernel/snapshot/log_data.rs:332`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

return the number of rows in each container as an `Option<UInt64Array>`.

Note: the returned array must contain `num_containers()` rows

<a id="op-978111cc61de153ff1d45b09"></a>
## config

`struct_field` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::config` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
config: &'a delta_kernel::table_configuration::TableConfiguration
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L61).

Source: `crates/core/src/kernel/snapshot/log_data.rs:61`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4508a0b79941c0850b25ec1b"></a>
## data

`struct_field` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler::data` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
data: &'a [arrow_array::RecordBatch]
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/log_data.rs#L60).

Source: `crates/core/src/kernel/snapshot/log_data.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
