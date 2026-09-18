# `datafusion_physical_plan::unnest::UnnestExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.unnest.UnnestExec.json).

<a id="op-85e9ef0e06ed05c2c6bfc4a3"></a>
## UnnestExec

`struct` · `datafusion_physical_plan::unnest::UnnestExec` · datafusion-physical-plan 55.1.0

```rust
struct UnnestExec
```

Source: `src/unnest.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Unnest the given columns (either with type struct or list)
For list unnesting, each row is vertically transformed into multiple rows
For struct unnesting, each column is horizontally transformed into multiple columns,
Thus the original RecordBatch with dimension (n x m) may have new dimension (n' x m')

See [`UnnestOptions`](../operations/datafusion_common.unnest.UnnestOptions.md#op-fc8c0e778d2849560cc4457a) for more details and an example.

<a id="op-88a38bea4d63b65347368c07"></a>
## apply_expressions

`function` · `datafusion_physical_plan::unnest::UnnestExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [398, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/unnest.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25f960d69635d0c9fcb37c2b"></a>
## children

`function` · `datafusion_physical_plan::unnest::UnnestExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [398, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/unnest.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-497c5c02984c4c15b2757c8e"></a>
## clone

`function` · `datafusion_physical_plan::unnest::UnnestExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> UnnestExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 17], "end": [66, 22], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unnest.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6208287a1f1e1d8bab3f9516"></a>
## execute

`function` · `datafusion_physical_plan::unnest::UnnestExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [398, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/unnest.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d6b425d312fc0c62d2ff92e"></a>
## fmt

`function` · `datafusion_physical_plan::unnest::UnnestExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 10], "end": [66, 15], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/unnest.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4e4742b1d60da48516bda68"></a>
## fmt_as

`function` · `datafusion_physical_plan::unnest::UnnestExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [217, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/unnest.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d00a24de5ee8ecdfd7a4a4e"></a>
## input

`function` · `datafusion_physical_plan::unnest::UnnestExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [200, 2], "filename": "src/unnest.rs"}, "trait": null, "trait_path": null}`

Source: `src/unnest.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Input execution plan

<a id="op-ec993dbd99460826bc401225"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::unnest::UnnestExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [398, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/unnest.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a389b94fc300b8805a7918d"></a>
## list_column_indices

`function` · `datafusion_physical_plan::unnest::UnnestExec::list_column_indices` · datafusion-physical-plan 55.1.0

```rust
fn list_column_indices(&self) -> &[ListUnnest]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [200, 2], "filename": "src/unnest.rs"}, "trait": null, "trait_path": null}`

Source: `src/unnest.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Indices of the list-typed columns in the input schema

<a id="op-a49d1bbcc137ca4a6a05e185"></a>
## metrics

`function` · `datafusion_physical_plan::unnest::UnnestExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [398, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/unnest.rs:321`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-725797b18c1d628b4a5d5108"></a>
## name

`function` · `datafusion_physical_plan::unnest::UnnestExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [398, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/unnest.rs:220`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a503ccd4a79c92603fb1e56d"></a>
## new

`function` · `datafusion_physical_plan::unnest::UnnestExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(input: Arc<dyn ExecutionPlan>, list_column_indices: Vec<ListUnnest>, struct_column_indices: Vec<usize>, schema: SchemaRef, options: UnnestOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [200, 2], "filename": "src/unnest.rs"}, "trait": null, "trait_path": null}`

Source: `src/unnest.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [UnnestExec](../operations/datafusion_physical_plan.unnest.UnnestExec.md#op-85e9ef0e06ed05c2c6bfc4a3).

<a id="op-50ce70d4e8f51cee1516ab3a"></a>
## options

`function` · `datafusion_physical_plan::unnest::UnnestExec::options` · datafusion-physical-plan 55.1.0

```rust
fn options(&self) -> &UnnestOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [200, 2], "filename": "src/unnest.rs"}, "trait": null, "trait_path": null}`

Source: `src/unnest.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16e93201c12e1ab96115c54f"></a>
## properties

`function` · `datafusion_physical_plan::unnest::UnnestExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [398, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/unnest.rs:224`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-763bd0ad561caf16e2579414"></a>
## replace_children

`function` · `datafusion_physical_plan::unnest::UnnestExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [398, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/unnest.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4314b06b7401d53788314f1"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::unnest::UnnestExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [398, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/unnest.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5aa0b6ffa30a877a42454900"></a>
## struct_column_indices

`function` · `datafusion_physical_plan::unnest::UnnestExec::struct_column_indices` · datafusion-physical-plan 55.1.0

```rust
fn struct_column_indices(&self) -> &[usize]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [200, 2], "filename": "src/unnest.rs"}, "trait": null, "trait_path": null}`

Source: `src/unnest.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Indices of the struct-typed columns in the input schema

<a id="op-848d232c5a681da9ab9cbf20"></a>
## try_from_proto

`function` · `datafusion_physical_plan::unnest::UnnestExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [401, 1], "end": [488, 2], "filename": "src/unnest.rs"}, "trait": null, "trait_path": null}`

Source: `src/unnest.rs:407`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct an [`UnnestExec`](../operations/datafusion_physical_plan.unnest.UnnestExec.md#op-85e9ef0e06ed05c2c6bfc4a3) from its protobuf representation.

The exact inverse of [`ExecutionPlan::try_to_proto`].

[`ExecutionPlan::try_to_proto`]: crate::ExecutionPlan::try_to_proto

<a id="op-3910aa110e92c1dc447ea5dc"></a>
## try_to_proto

`function` · `datafusion_physical_plan::unnest::UnnestExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [398, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/unnest.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b73d4fecf5a00a34820be7e8"></a>
## with_new_children

`function` · `datafusion_physical_plan::unnest::UnnestExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [398, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/unnest.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-511cf33d38758e1357c67231"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::unnest::UnnestExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::unnest::UnnestExec", "path": "UnnestExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [398, 2], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/unnest.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
