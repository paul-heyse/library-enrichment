# `datafusion_datasource::sink::DataSinkExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.sink.DataSinkExec.json).

<a id="op-79c64cf00b80b5de2d143f4e"></a>
## DataSinkExec

`struct` · `datafusion_datasource::sink::DataSinkExec` · datafusion-datasource 55.1.0

```rust
struct DataSinkExec
```

Source: `src/sink.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Execution plan for writing record batches to a [`DataSink`](../operations/datafusion_datasource.sink.DataSink.md#op-9a57c258a80fb4e5a004e0e1)

Returns a single row with the number of values written

<a id="op-6b5889f683491860c82ce95a"></a>
## apply_expressions

`function` · `datafusion_datasource::sink::DataSinkExec::apply_expressions` · datafusion-datasource 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc67be1042ebc0c2d174e77c"></a>
## benefits_from_input_partitioning

`function` · `datafusion_datasource::sink::DataSinkExec::benefits_from_input_partitioning` · datafusion-datasource 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7749bce6abe93c315d2672b4"></a>
## children

`function` · `datafusion_datasource::sink::DataSinkExec::children` · datafusion-datasource 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce279d1dd60ccc47404f560c"></a>
## clone

`function` · `datafusion_datasource::sink::DataSinkExec::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> DataSinkExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 10], "end": [107, 15], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sink.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1edbd6861b3431b4898984d5"></a>
## decode_sort_order

`function` · `datafusion_datasource::sink::DataSinkExec::decode_sort_order` · datafusion-datasource 55.1.0

```rust
fn decode_sort_order(collection: Option<&datafusion_proto_models::protobuf::PhysicalSortExprNodeCollection>, ctx: &datafusion_physical_plan::proto::ExecutionPlanDecodeCtx<'_>, schema: &Schema) -> Result<Option<LexRequirement>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [247, 2], "filename": "src/sink.rs"}, "trait": null, "trait_path": null}`

Source: `src/sink.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Decode the optional sink ordering from a protobuf plan node.

<a id="op-8e421a7f7046c4dfda0ddd00"></a>
## encode_sort_order

`function` · `datafusion_datasource::sink::DataSinkExec::encode_sort_order` · datafusion-datasource 55.1.0

```rust
fn encode_sort_order(&self, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalSortExprNodeCollection>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [247, 2], "filename": "src/sink.rs"}, "trait": null, "trait_path": null}`

Source: `src/sink.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Encode the optional sink ordering for a protobuf plan node.

<a id="op-cc5460e38cb5b68fc9802d8e"></a>
## execute

`function` · `datafusion_datasource::sink::DataSinkExec::execute` · datafusion-datasource 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Execute the plan and return a stream of `RecordBatch`es for
the specified partition.

<a id="op-67400dee3e7465b162f394dc"></a>
## fmt

`function` · `datafusion_datasource::sink::DataSinkExec::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [124, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sink.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce549a802ba1c9f391f83204"></a>
## fmt_as

`function` · `datafusion_datasource::sink::DataSinkExec::fmt_as` · datafusion-datasource 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [249, 1], "end": [259, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/sink.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-506cf5cebade15af09671047"></a>
## input

`function` · `datafusion_datasource::sink::DataSinkExec::input` · datafusion-datasource 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [247, 2], "filename": "src/sink.rs"}, "trait": null, "trait_path": null}`

Source: `src/sink.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Input execution plan

<a id="op-9ba037b72c5642f542166ffa"></a>
## input_distribution_requirements

`function` · `datafusion_datasource::sink::DataSinkExec::input_distribution_requirements` · datafusion-datasource 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db832355cffc17008ca5049b"></a>
## maintains_input_order

`function` · `datafusion_datasource::sink::DataSinkExec::maintains_input_order` · datafusion-datasource 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:296`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1ed5671550bce8ae204b17c"></a>
## metrics

`function` · `datafusion_datasource::sink::DataSinkExec::metrics` · datafusion-datasource 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns the metrics of the underlying [DataSink](../operations/datafusion_datasource.sink.DataSink.md#op-9a57c258a80fb4e5a004e0e1)

<a id="op-4cfedda701d53838f30ba657"></a>
## name

`function` · `datafusion_datasource::sink::DataSinkExec::name` · datafusion-datasource 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a61b3456584e847522a56cc"></a>
## new

`function` · `datafusion_datasource::sink::DataSinkExec::new` · datafusion-datasource 55.1.0

```rust
fn new(input: Arc<dyn ExecutionPlan>, sink: Arc<dyn DataSink>, sort_order: Option<LexRequirement>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [247, 2], "filename": "src/sink.rs"}, "trait": null, "trait_path": null}`

Source: `src/sink.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a plan to write to `sink`
Note: DataSinkExec requires its input to have a single partition.
If the input has multiple partitions, the physical optimizer will
automatically insert a Merge-related operator to merge them.
If you construct PhysicalPlan without going through the physical optimizer,
you must ensure that the input has a single partition.

<a id="op-b4cb51ee809b512aa67c29f6"></a>
## properties

`function` · `datafusion_datasource::sink::DataSinkExec::properties` · datafusion-datasource 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-38be74b368bc63412d849cb7"></a>
## replace_children

`function` · `datafusion_datasource::sink::DataSinkExec::replace_children` · datafusion-datasource 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84e2477a4300852e30754c71"></a>
## required_input_distribution

`function` · `datafusion_datasource::sink::DataSinkExec::required_input_distribution` · datafusion-datasource 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c741efb274f521586f02442"></a>
## required_input_ordering

`function` · `datafusion_datasource::sink::DataSinkExec::required_input_ordering` · datafusion-datasource 55.1.0

```rust
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf1390bbf537b613ce50150e"></a>
## sink

`function` · `datafusion_datasource::sink::DataSinkExec::sink` · datafusion-datasource 55.1.0

```rust
fn sink(&self) -> &dyn DataSink
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [247, 2], "filename": "src/sink.rs"}, "trait": null, "trait_path": null}`

Source: `src/sink.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns insert sink

<a id="op-c7e3fb483eae0cd94f07070d"></a>
## sort_order

`function` · `datafusion_datasource::sink::DataSinkExec::sort_order` · datafusion-datasource 55.1.0

```rust
fn sort_order(&self) -> &Option<LexRequirement>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [247, 2], "filename": "src/sink.rs"}, "trait": null, "trait_path": null}`

Source: `src/sink.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Optional sort order for output data

<a id="op-f0e3caedb5a55dffee7f7ab8"></a>
## try_to_proto

`function` · `datafusion_datasource::sink::DataSinkExec::try_to_proto` · datafusion-datasource 55.1.0

```rust
fn try_to_proto(&self, ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:377`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Delegates protobuf serialization to the underlying sink.

<a id="op-c48e726cada5eda67965fbc5"></a>
## with_new_children

`function` · `datafusion_datasource::sink::DataSinkExec::with_new_children` · datafusion-datasource 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::sink::DataSinkExec", "path": "DataSinkExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [383, 2], "filename": "src/sink.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sink.rs:320`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
