# Expressions and predicates

Two expression languages meet here. DataFusion expressions drive DML predicates and queries; kernel expressions drive data skipping and log replay. Data skipping evaluates a predicate against per-file statistics rather than rows, so a predicate it cannot interpret is not wrong -- it simply prunes nothing, and the scan silently reads everything.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `buoyant_kernel::expressions::Expression` | enum | 40 | [prose](../api/buoyant_kernel.expressions.md#expression) | [records](../model/buoyant_kernel.expressions.json) |
| `buoyant_kernel::expressions::Predicate` | enum | 32 | [prose](../api/buoyant_kernel.expressions.md#predicate) | [records](../model/buoyant_kernel.expressions.json) |
| `buoyant_kernel::expressions::column_names::ColumnName` | struct | 23 | [prose](../api/buoyant_kernel.expressions.column_names.md#columnname) | [records](../model/buoyant_kernel.expressions.column_names.json) |
| `buoyant_kernel::expressions::scalars::Scalar` | enum | 42 | [prose](../api/buoyant_kernel.expressions.scalars.md#scalar) | [records](../model/buoyant_kernel.expressions.scalars.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `buoyant_kernel::transforms::expression::ExpressionTransform` | 0 | 33 | 4 | ExpressionTransform |
| `buoyant_kernel::transforms::schema::SchemaTransform` | 0 | 14 | 25 | SchemaTransform |
| `buoyant_kernel::ExpressionEvaluator` | 1 | 0 | 1 | [ExpressionEvaluator](../traits/ExpressionEvaluator.md) |
| `buoyant_kernel::PredicateEvaluator` | 1 | 0 | 1 | [PredicateEvaluator](../traits/PredicateEvaluator.md) |

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Reference partition and statistics columns directly in a predicate; wrapping one in a function defeats skipping.
- Confirm pruning by the number of files read, not by the predicate looking correct.

## Anti-patterns

- Assuming a predicate prunes because it is selective. Skipping needs statistics it can interpret.
- Mixing the two expression languages without noticing which one an API takes.

## Agent checklist

- Does the predicate touch columns that carry statistics?
- Was pruning measured rather than assumed?
