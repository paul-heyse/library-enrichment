# EvaluationHandler

`buoyant_kernel::EvaluationHandler`

```rust
trait EvaluationHandler: AsAny
```

Also reachable as `delta_kernel::EvaluationHandler`

Prose: [`api/buoyant_kernel.md`](../api/buoyant_kernel.md#evaluationhandler) · records: [`model/buoyant_kernel.json`](../model/buoyant_kernel.json)

## Required

Every implementation must supply these.

```rust
fn create_many(&self, schema: SchemaRef, rows: &[&[Scalar]]) -> DeltaResult<Box<dyn EngineData>>
fn new_expression_evaluator(&self, input_schema: SchemaRef, expression: ExpressionRef, output_type: DataType) -> DeltaResult<Arc<dyn ExpressionEvaluator>>
fn new_predicate_evaluator(&self, input_schema: SchemaRef, predicate: PredicateRef) -> DeltaResult<Arc<dyn PredicateEvaluator>>
fn null_row(&self, output_schema: SchemaRef) -> DeltaResult<Box<dyn EngineData>>
```

## Implementors (1)

Read one before writing your own.

- `buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler`

## Documentation

Provides expression evaluation capability to Delta Kernel.

Delta Kernel can use this handler to evaluate a predicate on partition filters,
fill up partition column values, and any computation on data using Expressions.
