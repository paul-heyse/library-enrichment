# ExpressionEvaluator

`buoyant_kernel::ExpressionEvaluator`

```rust
trait ExpressionEvaluator: AsAny
```

Also reachable as `delta_kernel::ExpressionEvaluator`

Prose: [`api/buoyant_kernel.md`](../api/buoyant_kernel.md#expressionevaluator) · records: [`model/buoyant_kernel.json`](../model/buoyant_kernel.json)

## Required

Every implementation must supply these.

```rust
fn evaluate(&self, batch: &dyn EngineData) -> DeltaResult<Box<dyn EngineData>>
```

## Implementors (1)

Read one before writing your own.

- `buoyant_kernel::engine::arrow_expression::DefaultExpressionEvaluator`

## Documentation

Trait for implementing an Expression evaluator.

It contains one Expression which can be evaluated on multiple ColumnarBatches.
Connectors can implement this trait to optimize the evaluation using the
connector specific capabilities.
