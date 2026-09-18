# `datafusion_physical_plan::placeholder_row::PlaceholderRowExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.placeholder_row.PlaceholderRowExec.json).

<a id="op-958d09e27b7688f38583f75a"></a>
## PlaceholderRowExec

`struct` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec` · datafusion-physical-plan 55.1.0

```rust
struct PlaceholderRowExec
```

Source: `src/placeholder_row.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Execution plan for empty relation with produce_one_row=true

<a id="op-78922fa8a55d0faeb9af0e1a"></a>
## apply_expressions

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [232, 2], "filename": "src/placeholder_row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/placeholder_row.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81a9516b1b1a9e1401f49480"></a>
## children

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [232, 2], "filename": "src/placeholder_row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/placeholder_row.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1705d1b098f452c02d5eb39f"></a>
## clone

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> PlaceholderRowExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 17], "end": [43, 22], "filename": "src/placeholder_row.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/placeholder_row.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e486bf407708fd0e7a21e50"></a>
## execute

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [232, 2], "filename": "src/placeholder_row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/placeholder_row.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b9a2da053bb4b8b79a03f1f"></a>
## fmt

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 10], "end": [43, 15], "filename": "src/placeholder_row.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/placeholder_row.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cf074fcf720dad0ee3b94b1"></a>
## fmt_as

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [126, 2], "filename": "src/placeholder_row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/placeholder_row.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6e705722677b4a856cffb74"></a>
## name

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [232, 2], "filename": "src/placeholder_row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/placeholder_row.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aca210fd3606f99cafc9a795"></a>
## new

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [110, 2], "filename": "src/placeholder_row.rs"}, "trait": null, "trait_path": null}`

Source: `src/placeholder_row.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new PlaceholderRowExec

<a id="op-ae2f15a12049419fa242b99e"></a>
## properties

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [232, 2], "filename": "src/placeholder_row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/placeholder_row.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-24d427370ed53d50d421e717"></a>
## replace_children

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [232, 2], "filename": "src/placeholder_row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/placeholder_row.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0472dfd6e7286af18fcadabd"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, _input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [232, 2], "filename": "src/placeholder_row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/placeholder_row.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f63efa0aef4087f0e2f9f62"></a>
## try_from_proto

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, _ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [260, 2], "filename": "src/placeholder_row.rs"}, "trait": null, "trait_path": null}`

Source: `src/placeholder_row.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a [`PlaceholderRowExec`](../operations/datafusion_physical_plan.placeholder_row.PlaceholderRowExec.md#op-958d09e27b7688f38583f75a) from its protobuf representation.

<a id="op-cb5d46ce05f35994be0af63f"></a>
## try_to_proto

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, _ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [232, 2], "filename": "src/placeholder_row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/placeholder_row.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8adb5d999a8babcc86c4c106"></a>
## with_new_children

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [232, 2], "filename": "src/placeholder_row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/placeholder_row.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78c883d6ab4368e45e9905e3"></a>
## with_partitions

`function` · `datafusion_physical_plan::placeholder_row::PlaceholderRowExec::with_partitions` · datafusion-physical-plan 55.1.0

```rust
fn with_partitions(self, partitions: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::placeholder_row::PlaceholderRowExec", "path": "PlaceholderRowExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [110, 2], "filename": "src/placeholder_row.rs"}, "trait": null, "trait_path": null}`

Source: `src/placeholder_row.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new PlaceholderRowExecPlaceholderRowExec with specified partition number
