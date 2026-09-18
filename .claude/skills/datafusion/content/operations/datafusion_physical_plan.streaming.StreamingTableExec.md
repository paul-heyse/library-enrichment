# `datafusion_physical_plan::streaming::StreamingTableExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.streaming.StreamingTableExec.json).

<a id="op-318fccba34a8cefdd69316b3"></a>
## StreamingTableExec

`struct` · `datafusion_physical_plan::streaming::StreamingTableExec` · datafusion-physical-plan 55.1.0

```rust
struct StreamingTableExec
```

Source: `src/streaming.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

An [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) for one or more [`PartitionStream`](../operations/datafusion_physical_plan.streaming.PartitionStream.md#op-477eb3d02543add98f425b63)s.

If your source can be represented as one or more [`PartitionStream`](../operations/datafusion_physical_plan.streaming.PartitionStream.md#op-477eb3d02543add98f425b63)s, you can
use this struct to implement [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673).

<a id="op-8d5eedd1873755fddb2b90a0"></a>
## apply_expressions

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [400, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/streaming.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d506406bee09f21e9b96b1aa"></a>
## children

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [400, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/streaming.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e79ab672fbfabb415a3a4b2"></a>
## clone

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> StreamingTableExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 15], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/streaming.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e33543c2cf3887ee31f5327"></a>
## execute

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, ctx: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [400, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/streaming.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-709ca68e770cf6762e078c14"></a>
## fetch

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [400, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/streaming.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5710da20a01ecc68c3b7170f"></a>
## fmt

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [202, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/streaming.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6abbd4c5d5eda4070a39391d"></a>
## fmt_as

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [259, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/streaming.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc1e7592da4264bdc3d81c26"></a>
## is_infinite

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::is_infinite` · datafusion-physical-plan 55.1.0

```rust
fn is_infinite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [196, 2], "filename": "src/streaming.rs"}, "trait": null, "trait_path": null}`

Source: `src/streaming.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc5a7d3a8daff9fb9eeb3b15"></a>
## limit

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::limit` · datafusion-physical-plan 55.1.0

```rust
fn limit(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [196, 2], "filename": "src/streaming.rs"}, "trait": null, "trait_path": null}`

Source: `src/streaming.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7e9e8064c1619ad7c95fcaa"></a>
## metrics

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [400, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/streaming.rs:384`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50a8d41d36e0de72100b4209"></a>
## name

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [400, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/streaming.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfa20943edef1ec9eed8ba64"></a>
## partition_schema

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::partition_schema` · datafusion-physical-plan 55.1.0

```rust
fn partition_schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [196, 2], "filename": "src/streaming.rs"}, "trait": null, "trait_path": null}`

Source: `src/streaming.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d51c75846ecbcb44a179216"></a>
## partitions

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::partitions` · datafusion-physical-plan 55.1.0

```rust
fn partitions(&self) -> &Vec<Arc<dyn PartitionStream>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [196, 2], "filename": "src/streaming.rs"}, "trait": null, "trait_path": null}`

Source: `src/streaming.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7634b7608c83a9979a94a583"></a>
## projected_output_ordering

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::projected_output_ordering` · datafusion-physical-plan 55.1.0

```rust
fn projected_output_ordering(&self) -> impl IntoIterator<Item = LexOrdering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [196, 2], "filename": "src/streaming.rs"}, "trait": null, "trait_path": null}`

Source: `src/streaming.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1da9a791307fefa127f22a8c"></a>
## projected_schema

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::projected_schema` · datafusion-physical-plan 55.1.0

```rust
fn projected_schema(&self) -> &Schema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [196, 2], "filename": "src/streaming.rs"}, "trait": null, "trait_path": null}`

Source: `src/streaming.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66c50ee5d1a7f77c990a84ce"></a>
## projection

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::projection` · datafusion-physical-plan 55.1.0

```rust
fn projection(&self) -> &Option<Arc<[usize]>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [196, 2], "filename": "src/streaming.rs"}, "trait": null, "trait_path": null}`

Source: `src/streaming.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20abe54b25cb6249f1db9da6"></a>
## properties

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [400, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/streaming.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd64b750a477c4f8c6cec0be"></a>
## replace_children

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [400, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/streaming.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-359e9380057f00e140549e36"></a>
## try_new

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(schema: SchemaRef, partitions: Vec<Arc<dyn PartitionStream>>, projection: Option<&Vec<usize>>, projected_output_ordering: impl IntoIterator<Item = LexOrdering>, infinite: bool, limit: Option<usize>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [196, 2], "filename": "src/streaming.rs"}, "trait": null, "trait_path": null}`

Source: `src/streaming.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Try to create a new [`StreamingTableExec`](../operations/datafusion_physical_plan.streaming.StreamingTableExec.md#op-318fccba34a8cefdd69316b3) returning an error if the schema is incorrect

<a id="op-2aed5502aee33796303e07eb"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [400, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/streaming.rs:337`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to embed `projection` to its input (`streaming table`).
If possible, returns [`StreamingTableExec`](../operations/datafusion_physical_plan.streaming.StreamingTableExec.md#op-318fccba34a8cefdd69316b3) as the top plan. Otherwise,
returns `None`.

<a id="op-3874a7aa744e99593c021693"></a>
## with_fetch

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [400, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/streaming.rs:388`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84a003a3d8e5e4678246897c"></a>
## with_new_children

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [400, 2], "filename": "src/streaming.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/streaming.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a9b4b2353bd79cc0be73145"></a>
## with_output_partitioning

`function` · `datafusion_physical_plan::streaming::StreamingTableExec::with_output_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn with_output_partitioning(self, output_partitioning: Partitioning) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::streaming::StreamingTableExec", "path": "StreamingTableExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [196, 2], "filename": "src/streaming.rs"}, "trait": null, "trait_path": null}`

Source: `src/streaming.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Declares the output partitioning of this stream.

`output_partitioning` must describe this plan's current output and have
the same number of partitions as the stream.
