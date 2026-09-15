# User-defined functions

Three trait families, each with a registration call on `SessionContext`. Beyond `invoke`, every one carries optimizer-facing hooks — `simplify`, `evaluate_bounds`, `propagate_constraints`, `set_monotonicity`, `short_circuits`, `groups_accumulator_supported` — and those are the difference between a function that works and one the planner can reason about. Scalar UDFs can also be async, which is what makes remote calls from an expression viable.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion_expr::udf::ScalarUDF` | struct | 38 | [prose](../api/datafusion_expr.udf.md#scalarudf) | [records](../model/datafusion_expr.udf.json) |
| `datafusion_expr::udaf::AggregateUDF` | struct | 43 | [prose](../api/datafusion_expr.udaf.md#aggregateudf) | [records](../model/datafusion_expr.udaf.json) |
| `datafusion_expr::udwf::WindowUDF` | struct | 26 | [prose](../api/datafusion_expr.udwf.md#windowudf) | [records](../model/datafusion_expr.udwf.json) |
| `datafusion_common::scalar::ScalarValue` | enum | 104 | [prose](../api/datafusion_common.scalar.md#scalarvalue) | [records](../model/datafusion_common.scalar.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `datafusion_expr::udf::ScalarUDFImpl` | 4 | 20 | 235 | [ScalarUDFImpl](../traits/ScalarUDFImpl.md) |
| `datafusion_expr::udaf::AggregateUDFImpl` | 4 | 25 | 39 | [AggregateUDFImpl](../traits/AggregateUDFImpl.md) |
| `datafusion_expr::udwf::WindowUDFImpl` | 4 | 8 | 8 | [WindowUDFImpl](../traits/WindowUDFImpl.md) |
| `datafusion_expr_common::accumulator::Accumulator` | 5 | 2 | 37 | [Accumulator](../traits/Accumulator.md) |
| `datafusion_expr_common::groups_accumulator::GroupsAccumulator` | 6 | 0 | 7 | [GroupsAccumulator](../traits/GroupsAccumulator.md) |
| `datafusion_expr::partition_evaluator::PartitionEvaluator` | 0 | 9 | 0 | [PartitionEvaluator](../traits/PartitionEvaluator.md) |
| `datafusion_session::table::TableFunctionImpl` | 0 | 2 | 3 | [TableFunctionImpl](../traits/TableFunctionImpl.md) |
| `datafusion::execution::context::FunctionFactory` | 1 | 0 | 0 | [FunctionFactory](../traits/FunctionFactory.md) |

## Runnable examples (11)

- [`corpus/examples/udf/advanced_udaf.rs`](../corpus/examples/udf/advanced_udaf.rs)
- [`corpus/examples/udf/advanced_udf.rs`](../corpus/examples/udf/advanced_udf.rs)
- [`corpus/examples/udf/advanced_udwf.rs`](../corpus/examples/udf/advanced_udwf.rs)
- [`corpus/examples/udf/async_udf.rs`](../corpus/examples/udf/async_udf.rs)
- [`corpus/examples/udf/main.rs`](../corpus/examples/udf/main.rs)
- [`corpus/examples/udf/simple_udaf.rs`](../corpus/examples/udf/simple_udaf.rs)
- [`corpus/examples/udf/simple_udf.rs`](../corpus/examples/udf/simple_udf.rs)
- [`corpus/examples/udf/simple_udtf.rs`](../corpus/examples/udf/simple_udtf.rs)
- [`corpus/examples/udf/simple_udwf.rs`](../corpus/examples/udf/simple_udwf.rs)
- [`corpus/examples/udf/struct_returning_udaf.rs`](../corpus/examples/udf/struct_returning_udaf.rs)
- [`corpus/examples/udf/table_list_udtf.rs`](../corpus/examples/udf/table_list_udtf.rs)

## Upstream guides

- [`corpus/guides/library-user-guide/functions/adding-udfs.md`](../corpus/guides/library-user-guide/functions/adding-udfs.md)
- [`corpus/guides/user-guide/sql/scalar_functions.md`](../corpus/guides/user-guide/sql/scalar_functions.md)
- [`corpus/guides/user-guide/sql/aggregate_functions.md`](../corpus/guides/user-guide/sql/aggregate_functions.md)
- [`corpus/guides/user-guide/sql/window_functions.md`](../corpus/guides/user-guide/sql/window_functions.md)

## Decision rules

- Implement `groups_accumulator_supported` and `create_groups_accumulator` for any aggregate that will run over many groups; the row-at-a-time `Accumulator` path is much slower.
- `simplify` lets a UDF rewrite itself at plan time, which is often cheaper than evaluating it.
- Declare `Volatility` honestly — marking a volatile function `Immutable` lets the optimizer fold it away.

## Anti-patterns

- Implementing only `invoke_with_args` and leaving every optimizer hook at its default.
- Registering a UDF globally when it is only needed for one session or one query.

## Agent checklist

- Is the signature's coercion behaviour what you meant, or did it default to exact matching?
- Is volatility correct?
- For aggregates: is there a groups accumulator?
