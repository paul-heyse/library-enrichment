# `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.accumulator.AccumulatorArgs.json).

<a id="op-0011bde6a704a46a9e4e009c"></a>
## AccumulatorArgs

`struct` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs` · datafusion-functions-aggregate-common 55.1.0

```rust
struct AccumulatorArgs<'a>
```

Source: `src/accumulator.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

[`AccumulatorArgs`](../operations/datafusion_functions_aggregate_common.accumulator.AccumulatorArgs.md#op-0011bde6a704a46a9e4e009c) contains information about how an aggregate
function was called, including the types of its arguments and any optional
ordering expressions.

<a id="op-81a960868302c023e2436251"></a>
## clone

`function` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs::clone` · datafusion-functions-aggregate-common 55.1.0

```rust
fn clone(&self) -> AccumulatorArgs<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::accumulator::AccumulatorArgs", "path": "AccumulatorArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 17], "end": [28, 22], "filename": "src/accumulator.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/accumulator.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4804124e8ed3e75edee6313f"></a>
## expr_fields

`struct_field` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs::expr_fields` · datafusion-functions-aggregate-common 55.1.0

```rust
expr_fields: &'a [arrow::datatypes::FieldRef]
```

Source: `src/accumulator.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Fields corresponding to each expr (same order & length).

<a id="op-241935779fa5b6c76242ba33"></a>
## exprs

`struct_field` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs::exprs` · datafusion-functions-aggregate-common 55.1.0

```rust
exprs: &'a [std::sync::Arc<dyn PhysicalExpr>]
```

Source: `src/accumulator.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

The physical expression of arguments the aggregate function takes.

<a id="op-f3d6c313b025caa6f1532c73"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::accumulator::AccumulatorArgs", "path": "AccumulatorArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 10], "end": [28, 15], "filename": "src/accumulator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/accumulator.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cae99f53cf6bd8c1cb2b732"></a>
## ignore_nulls

`struct_field` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs::ignore_nulls` · datafusion-functions-aggregate-common 55.1.0

```rust
ignore_nulls: bool
```

Source: `src/accumulator.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Whether to ignore nulls.

SQL allows the user to specify `IGNORE NULLS`, for example:

```sql
SELECT FIRST_VALUE(column1) IGNORE NULLS FROM t;
```

<a id="op-65cc7cabe0da8bd91d7e1299"></a>
## is_distinct

`struct_field` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs::is_distinct` · datafusion-functions-aggregate-common 55.1.0

```rust
is_distinct: bool
```

Source: `src/accumulator.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Whether the aggregate function is distinct.

```sql
SELECT COUNT(DISTINCT column1) FROM t;
```

<a id="op-f6574d84cb1119d264e36ac3"></a>
## is_reversed

`struct_field` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs::is_reversed` · datafusion-functions-aggregate-common 55.1.0

```rust
is_reversed: bool
```

Source: `src/accumulator.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Whether the aggregation is running in reverse order

<a id="op-fbd2528781d742607f324bf6"></a>
## name

`struct_field` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs::name` · datafusion-functions-aggregate-common 55.1.0

```rust
name: &'a str
```

Source: `src/accumulator.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

The name of the aggregate expression

<a id="op-90babc03761cc959f57e6efe"></a>
## order_bys

`struct_field` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs::order_bys` · datafusion-functions-aggregate-common 55.1.0

```rust
order_bys: &'a [datafusion_physical_expr_common::sort_expr::PhysicalSortExpr]
```

Source: `src/accumulator.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

The expressions in the `ORDER BY` clause passed to this aggregator.

SQL allows the user to specify the ordering of arguments to the
aggregate using an `ORDER BY`. For example:

```sql
SELECT FIRST_VALUE(column1 ORDER BY column2) FROM t;
```

<a id="op-465f467a311d060f7ee38ea7"></a>
## return_field

`struct_field` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs::return_field` · datafusion-functions-aggregate-common 55.1.0

```rust
return_field: arrow::datatypes::FieldRef
```

Source: `src/accumulator.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

The return field of the aggregate function.

<a id="op-5163e57108e5406698489756"></a>
## return_type

`function` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs::return_type` · datafusion-functions-aggregate-common 55.1.0

```rust
fn return_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::accumulator::AccumulatorArgs", "path": "AccumulatorArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [81, 2], "filename": "src/accumulator.rs"}, "trait": null, "trait_path": null}`

Source: `src/accumulator.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Returns the return type of the aggregate function.

<a id="op-98f7a48e2da62ea2d3916650"></a>
## schema

`struct_field` · `datafusion_functions_aggregate_common::accumulator::AccumulatorArgs::schema` · datafusion-functions-aggregate-common 55.1.0

```rust
schema: &'a arrow::datatypes::Schema
```

Source: `src/accumulator.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Input schema to the aggregate function. If you need to check data type, nullability
or metadata of input arguments then you should use `expr_fields` below instead.
