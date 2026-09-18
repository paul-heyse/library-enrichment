# `datafusion_datasource::memory::MemorySourceConfig`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.memory.MemorySourceConfig.json).

<a id="op-1f40b13a0861192d2453d384"></a>
## MemorySourceConfig

`struct` · `datafusion_datasource::memory::MemorySourceConfig` · datafusion-datasource 55.1.0

```rust
struct MemorySourceConfig
```

Source: `src/memory.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Data source configuration for reading in-memory batches of data

<a id="op-95853f83e943b81522bfddbf"></a>
## apply_expressions

`function` · `datafusion_datasource::memory::MemorySourceConfig::apply_expressions` · datafusion-datasource 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [319, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/memory.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02a943737d6c372471bc0694"></a>
## clone

`function` · `datafusion_datasource::memory::MemorySourceConfig::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> MemorySourceConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 10], "end": [57, 15], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/memory.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02a0bcc3770cc4b53d63d3ce"></a>
## eq_properties

`function` · `datafusion_datasource::memory::MemorySourceConfig::eq_properties` · datafusion-datasource 55.1.0

```rust
fn eq_properties(&self) -> EquivalenceProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [319, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/memory.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06780c11976fc2f9ef8020e3"></a>
## fetch

`function` · `datafusion_datasource::memory::MemorySourceConfig::fetch` · datafusion-datasource 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [319, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/memory.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45ad0b06265b6cccd1f07ed1"></a>
## fmt

`function` · `datafusion_datasource::memory::MemorySourceConfig::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 17], "end": [57, 22], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89d89a2601db8fd1f50354c4"></a>
## fmt_as

`function` · `datafusion_datasource::memory::MemorySourceConfig::fmt_as` · datafusion-datasource 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [319, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/memory.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6281e29098261db7d233122b"></a>
## open

`function` · `datafusion_datasource::memory::MemorySourceConfig::open` · datafusion-datasource 55.1.0

```rust
fn open(&self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [319, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/memory.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70c5ca001da21d6d77ab9ea3"></a>
## original_schema

`function` · `datafusion_datasource::memory::MemorySourceConfig::original_schema` · datafusion-datasource 55.1.0

```rust
fn original_schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [668, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:528`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Arc clone of ref to original schema

<a id="op-0179b3b16c43bcc8aeeb77ce"></a>
## output_partitioning

`function` · `datafusion_datasource::memory::MemorySourceConfig::output_partitioning` · datafusion-datasource 55.1.0

```rust
fn output_partitioning(&self) -> Partitioning
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [319, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/memory.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d821c635d6832cc40398aac0"></a>
## partition_statistics

`function` · `datafusion_datasource::memory::MemorySourceConfig::partition_statistics` · datafusion-datasource 55.1.0

```rust
fn partition_statistics(&self, partition: Option<usize>) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [319, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/memory.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ac504e07dde88395b498a0e"></a>
## partitions

`function` · `datafusion_datasource::memory::MemorySourceConfig::partitions` · datafusion-datasource 55.1.0

```rust
fn partitions(&self) -> &[Vec<RecordBatch>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [668, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:457`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Ref to partitions

<a id="op-2fc9405c7910dec2ff8d02e6"></a>
## projection

`function` · `datafusion_datasource::memory::MemorySourceConfig::projection` · datafusion-datasource 55.1.0

```rust
fn projection(&self) -> &Option<Vec<usize>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [668, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:462`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Ref to projection

<a id="op-0761bdc26eb5fa0dd98a5a8e"></a>
## repartitioned

`function` · `datafusion_datasource::memory::MemorySourceConfig::repartitioned` · datafusion-datasource 55.1.0

```rust
fn repartitioned(&self, target_partitions: usize, _repartition_file_min_size: usize, output_ordering: Option<LexOrdering>) -> Result<Option<Arc<dyn DataSource>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [319, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/memory.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

If possible, redistribute batches across partitions according to their size.

Returns `Ok(None)` if unable to repartition. Preserve output ordering if exists.
Refer to [`DataSource::repartitioned`](../operations/datafusion_datasource.source.DataSource.md#op-2c6fe409f79db273197b6979) for further details.

<a id="op-c1f5d568ffbad0c1367ec6d8"></a>
## scheduling_type

`function` · `datafusion_datasource::memory::MemorySourceConfig::scheduling_type` · datafusion-datasource 55.1.0

```rust
fn scheduling_type(&self) -> SchedulingType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [319, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/memory.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61f3fb9dd01ea9e37454e0f9"></a>
## show_sizes

`function` · `datafusion_datasource::memory::MemorySourceConfig::show_sizes` · datafusion-datasource 55.1.0

```rust
fn show_sizes(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [668, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:467`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Show sizes

<a id="op-897909f606489b76ae25501c"></a>
## sort_information

`function` · `datafusion_datasource::memory::MemorySourceConfig::sort_information` · datafusion-datasource 55.1.0

```rust
fn sort_information(&self) -> &[LexOrdering]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [668, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Ref to sort information

<a id="op-5ebc8f1e44dfd042dfa06c61"></a>
## try_from_proto

`function` · `datafusion_datasource::memory::MemorySourceConfig::try_from_proto` · datafusion-datasource 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn datafusion_physical_plan::ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [671, 1], "end": [723, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:675`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Reconstruct a [`DataSourceExec`](../operations/datafusion_datasource.source.DataSourceExec.md#op-96b0a6eef9f3c044c580d9b6) wrapping a `MemorySourceConfig` from
its protobuf representation. Byte-compatible with the former central
`MemoryScan` arm in `datafusion-proto`.

<a id="op-940e1a66e07b9a43e6d8c691"></a>
## try_new

`function` · `datafusion_datasource::memory::MemorySourceConfig::try_new` · datafusion-datasource 55.1.0

```rust
fn try_new(partitions: &[Vec<RecordBatch>], schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [668, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:324`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a new `MemorySourceConfig` for reading in-memory record batches
The provided `schema` should not have the projection applied.

<a id="op-11ace190db824ffbb9519413"></a>
## try_new_as_values

`function` · `datafusion_datasource::memory::MemorySourceConfig::try_new_as_values` · datafusion-datasource 55.1.0

```rust
fn try_new_as_values(schema: SchemaRef, data: Vec<Vec<Arc<dyn PhysicalExpr>>>) -> Result<Arc<DataSourceExec>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [668, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a new execution plan from a list of constant values (`ValuesExec`)

<a id="op-ec6e65c97ae831faea0dcf44"></a>
## try_new_exec

`function` · `datafusion_datasource::memory::MemorySourceConfig::try_new_exec` · datafusion-datasource 55.1.0

```rust
fn try_new_exec(partitions: &[Vec<RecordBatch>], schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Arc<DataSourceExec>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [668, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:343`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a new `DataSourceExec` plan for reading in-memory record batches
The provided `schema` should not have the projection applied.

<a id="op-03a4cec845253bd2239ee041"></a>
## try_new_from_batches

`function` · `datafusion_datasource::memory::MemorySourceConfig::try_new_from_batches` · datafusion-datasource 55.1.0

```rust
fn try_new_from_batches(schema: SchemaRef, batches: Vec<RecordBatch>) -> Result<Arc<DataSourceExec>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [668, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:412`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a new plan using the provided schema and batches.

Errors if any of the batches don't match the provided schema, or if no
batches are provided.

<a id="op-661fee0599484b88de87a190"></a>
## try_swapping_with_projection

`function` · `datafusion_datasource::memory::MemorySourceConfig::try_swapping_with_projection` · datafusion-datasource 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExprs) -> Result<Option<Arc<dyn DataSource>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [319, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/memory.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6df241e0fce22be0349998f1"></a>
## try_to_proto

`function` · `datafusion_datasource::memory::MemorySourceConfig::try_to_proto` · datafusion-datasource 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [319, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/memory.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Serialize this `MemorySourceConfig` as a `MemoryScanExecNode` wrapped
in a [`PhysicalPlanNode`]. Byte-compatible with the former central
`MemoryScan` arm in `datafusion-proto`.

[`PhysicalPlanNode`]: datafusion_proto_models::protobuf::PhysicalPlanNode

<a id="op-1ebc4a375cb290d347fdf4e7"></a>
## try_with_sort_information

`function` · `datafusion_datasource::memory::MemorySourceConfig::try_with_sort_information` · datafusion-datasource 55.1.0

```rust
fn try_with_sort_information(self, sort_information: Vec<LexOrdering>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [668, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:495`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A memory table can be ordered by multiple expressions simultaneously.
[`EquivalenceProperties`](../operations/datafusion_physical_expr.equivalence.properties.EquivalenceProperties.md#op-eeda1c3d472a48e6007359b5) keeps track of expressions that describe the
global ordering of the schema. These columns are not necessarily same; e.g.
```text
┌-------┐
| a | b |
|---|---|
| 1 | 9 |
| 2 | 8 |
| 3 | 7 |
| 5 | 5 |
└---┴---┘
```
where both `a ASC` and `b DESC` can describe the table ordering. With
[`EquivalenceProperties`](../operations/datafusion_physical_expr.equivalence.properties.EquivalenceProperties.md#op-eeda1c3d472a48e6007359b5), we can keep track of these equivalences
and treat `a ASC` and `b DESC` as the same ordering requirement.

Note that if there is an internal projection, that projection will be
also applied to the given `sort_information`.

<a id="op-209e710c743252a6be4b3142"></a>
## with_fetch

`function` · `datafusion_datasource::memory::MemorySourceConfig::with_fetch` · datafusion-datasource 55.1.0

```rust
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn DataSource>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [319, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::source::DataSource", "path": "DataSource"}, "trait_path": "datafusion_datasource::source::DataSource"}`

Source: `src/memory.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-223acc6a6c9ec04e8a5e8e5d"></a>
## with_limit

`function` · `datafusion_datasource::memory::MemorySourceConfig::with_limit` · datafusion-datasource 55.1.0

```rust
fn with_limit(self, limit: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [668, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:445`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the limit of the files

<a id="op-4cf261b2ec94ecc7b0624522"></a>
## with_show_sizes

`function` · `datafusion_datasource::memory::MemorySourceConfig::with_show_sizes` · datafusion-datasource 55.1.0

```rust
fn with_show_sizes(self, show_sizes: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemorySourceConfig", "path": "MemorySourceConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [668, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:451`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set `show_sizes` to determine whether to display partition sizes
