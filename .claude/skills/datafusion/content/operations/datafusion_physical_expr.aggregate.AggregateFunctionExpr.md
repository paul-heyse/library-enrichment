# `datafusion_physical_expr::aggregate::AggregateFunctionExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.aggregate.AggregateFunctionExpr.json).

<a id="op-a082484e03fcd21a48e2b705"></a>
## AggregateFunctionExpr

`struct` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr` · datafusion-physical-expr 55.1.0

```rust
struct AggregateFunctionExpr
```

Source: `src/aggregate.rs:645`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Physical aggregate expression of a UDAF.

Instances are constructed via [`AggregateExprBuilder`](../operations/datafusion_physical_expr.aggregate.AggregateExprBuilder.md#op-65df368c42420e560d9b75ee).

<a id="op-f2c0254552d40d5b2c0beaf2"></a>
## all_expressions

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::all_expressions` · datafusion-physical-expr 55.1.0

```rust
fn all_expressions(&self) -> AggregatePhysicalExpressions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:997`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns all expressions used in the [`AggregateFunctionExpr`](../operations/datafusion_physical_expr.aggregate.AggregateFunctionExpr.md#op-a082484e03fcd21a48e2b705).
These expressions are  (1)function arguments, (2) order by expressions.

<a id="op-17275c0c687500ccb62f4c4d"></a>
## clone

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> AggregateFunctionExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [644, 17], "end": [644, 22], "filename": "src/aggregate.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aggregate.rs:644`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e61901673f61b8298f7f119f"></a>
## create_accumulator

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::create_accumulator` · datafusion-physical-expr 55.1.0

```rust
fn create_accumulator(&self) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:737`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

the accumulator used to accumulate values from the expressions.
the accumulator expects the same number of arguments as `expressions` and must
return states with the same description as `state_fields`

<a id="op-d4b79181c1d2314a445ceda7"></a>
## create_groups_accumulator

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::create_groups_accumulator` · datafusion-physical-expr 55.1.0

```rust
fn create_groups_accumulator(&self) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:919`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return a specialized [`GroupsAccumulator`](../operations/datafusion_expr_common.groups_accumulator.GroupsAccumulator.md#op-9c9c43a7d8bf357eb3adcb6e) that manages state
for all groups.

For maximum performance, a [`GroupsAccumulator`](../operations/datafusion_expr_common.groups_accumulator.GroupsAccumulator.md#op-9c9c43a7d8bf357eb3adcb6e) should be
implemented in addition to [`Accumulator`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-2911d7ffb2098886b7dd6ba8).

<a id="op-26d439cf41af0af1553ff95f"></a>
## create_sliding_accumulator

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::create_sliding_accumulator` · datafusion-physical-expr 55.1.0

```rust
fn create_sliding_accumulator(&self) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:829`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates accumulator implementation that supports retract

<a id="op-9ba4278cbd0af3772e9bef42"></a>
## default_value

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::default_value` · datafusion-physical-expr 55.1.0

```rust
fn default_value(&self, data_type: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:1070`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns default value of the function given the input is Null
Most of the aggregate function return Null if input is Null,
while `count` returns 0 if input is Null

<a id="op-d9091b0febb108832a725210"></a>
## eq

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1105, 1], "end": [1117, 2], "filename": "src/aggregate.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/aggregate.rs:1106`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0dac8726e4c4bae17938f18a"></a>
## expressions

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::expressions` · datafusion-physical-expr 55.1.0

```rust
fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:677`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

expressions that are passed to the Accumulator.
Single-column aggregations such as `sum` return a single value, others (e.g. `cov`) return many.

<a id="op-92c2664356261314c6bb4b70"></a>
## field

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::field` · datafusion-physical-expr 55.1.0

```rust
fn field(&self) -> FieldRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:726`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

the field of the final result of this aggregation.

<a id="op-32730580f10513dc074511e0"></a>
## fmt

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [644, 10], "end": [644, 15], "filename": "src/aggregate.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate.rs:644`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc460cc760002a0c1e1f4547"></a>
## fun

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::fun` · datafusion-physical-expr 55.1.0

```rust
fn fun(&self) -> &AggregateUDF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:671`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return the `AggregateUDF` used by this `AggregateFunctionExpr`

<a id="op-83578a1b49e3611919dc3a7f"></a>
## get_minmax_desc

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::get_minmax_desc` · datafusion-physical-expr 55.1.0

```rust
fn get_minmax_desc(&self) -> Option<(FieldRef, bool)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:1063`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

If this function is max, return (output_field, true)
if the function is min, return (output_field, false)
otherwise return None (the default)

output_field is the name of the column produced by this aggregate

Note: this is used to use special aggregate implementations in certain conditions

<a id="op-956c8bfd83f6ea3c97af4063"></a>
## get_result_ordering

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::get_result_ordering` · datafusion-physical-expr 55.1.0

```rust
fn get_result_ordering(&self, aggr_func_idx: usize) -> Option<PhysicalSortExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:1083`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns `PhysicalSortExpr` based on the set monotonicity of the function.

<a id="op-7429c805732b5ae39b1a52f8"></a>
## groups_accumulator_supported

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::groups_accumulator_supported` · datafusion-physical-expr 55.1.0

```rust
fn groups_accumulator_supported(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:899`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

If the aggregate expression has a specialized
[`GroupsAccumulator`](../operations/datafusion_expr_common.groups_accumulator.GroupsAccumulator.md#op-9c9c43a7d8bf357eb3adcb6e) implementation. If this returns true,
`[Self::create_groups_accumulator`] will be called.

<a id="op-267664e79a5b68abe35f7144"></a>
## human_display

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::human_display` · datafusion-physical-expr 55.1.0

```rust
fn human_display(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:687`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Simplified name for `tree` explain.

<a id="op-6d061ccf3aae69e4defd0552"></a>
## ignore_nulls

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::ignore_nulls` · datafusion-physical-expr 55.1.0

```rust
fn ignore_nulls(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:711`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return if the aggregation ignores nulls

<a id="op-dd2ac96fbe0833f067762d89"></a>
## is_distinct

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::is_distinct` · datafusion-physical-expr 55.1.0

```rust
fn is_distinct(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:706`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return if the aggregation is distinct

<a id="op-3b87c7d1f49277fe2f587edb"></a>
## is_nullable

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::is_nullable` · datafusion-physical-expr 55.1.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:721`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return if the aggregation is nullable

<a id="op-db3246b5bc4101c14cee423e"></a>
## is_reversed

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::is_reversed` · datafusion-physical-expr 55.1.0

```rust
fn is_reversed(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:716`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return if the aggregation is reversed

<a id="op-c2a3bcc97750b43009e2721a"></a>
## name

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::name` · datafusion-physical-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:682`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Human readable name such as `"MIN(c2)"`.

<a id="op-9c043ff83f1d9b3224dc4a0c"></a>
## order_bys

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::order_bys` · datafusion-physical-expr 55.1.0

```rust
fn order_bys(&self) -> &[PhysicalSortExpr]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:767`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the ORDER BY expressions for the aggregate function.

<a id="op-9bd02fba1fafaf73ca87a85e"></a>
## order_sensitivity

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::order_sensitivity` · datafusion-physical-expr 55.1.0

```rust
fn order_sensitivity(&self) -> AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:778`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Indicates whether aggregator can produce the correct result with any
arbitrary input ordering. By default, we assume that aggregate expressions
are order insensitive.

<a id="op-8ab646a754226451c0ed0d3b"></a>
## reverse_expr

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::reverse_expr` · datafusion-physical-expr 55.1.0

```rust
fn reverse_expr(&self) -> Option<AggregateFunctionExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:938`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Construct an expression that calculates the aggregate in reverse.
Typically the "reverse" expression is itself (e.g. SUM, COUNT).
For aggregates that do not support calculation in reverse,
returns None (which is the default value).

<a id="op-43cb8106cfc389a00d944835"></a>
## set_monotonicity

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::set_monotonicity` · datafusion-physical-expr 55.1.0

```rust
fn set_monotonicity(&self) -> SetMonotonicity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:1076`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Indicates whether the aggregation function is monotonic as a set
function. See [`SetMonotonicity`](../operations/datafusion_expr.udaf.SetMonotonicity.md#op-a7821c42e6182e86e59e092e) for details.

<a id="op-b33f1be5d9fad6cd835b6a9a"></a>
## state_fields

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::state_fields` · datafusion-physical-expr 55.1.0

```rust
fn state_fields(&self) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:754`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

the field of the final result of this aggregation.

<a id="op-dfe1e02ef45265af6d58d9bd"></a>
## with_beneficial_ordering

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::with_beneficial_ordering` · datafusion-physical-expr 55.1.0

```rust
fn with_beneficial_ordering(Arc<self>, beneficial_ordering: bool) -> Result<Option<AggregateFunctionExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:798`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Sets the indicator whether ordering requirements of the aggregator is
satisfied by its input. If this is not the case, aggregators with order
sensitivity `AggregateOrderSensitivity::Beneficial` can still produce
the correct result with possibly more work internally.

# Returns

Returns `Ok(Some(updated_expr))` if the process completes successfully.
If the expression can benefit from existing input ordering, but does
not implement the method, returns an error. Order insensitive and hard
requirement aggregators return `Ok(None)`.

<a id="op-2ee9d11d7b7b183466692bdc"></a>
## with_new_expressions

`function` · `datafusion_physical_expr::aggregate::AggregateFunctionExpr::with_new_expressions` · datafusion-physical-expr 55.1.0

```rust
fn with_new_expressions(&self, args: Vec<Arc<dyn PhysicalExpr>>, order_by_exprs: Vec<Arc<dyn PhysicalExpr>>) -> Option<AggregateFunctionExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateFunctionExpr", "path": "AggregateFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [1095, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:1013`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Rewrites [`AggregateFunctionExpr`](../operations/datafusion_physical_expr.aggregate.AggregateFunctionExpr.md#op-a082484e03fcd21a48e2b705), with new expressions given. The argument should be consistent
with the return value of the [`AggregateFunctionExpr::all_expressions`](../operations/datafusion_physical_expr.aggregate.AggregateFunctionExpr.md#op-f2c0254552d40d5b2c0beaf2) method.
Returns `Some(Arc<dyn AggregateExpr>)` if re-write is supported, otherwise returns `None`.
