# PredicateEvaluator

`buoyant_kernel::PredicateEvaluator`

```rust
trait PredicateEvaluator: AsAny
```

Also reachable as `delta_kernel::PredicateEvaluator`

Prose: [`api/buoyant_kernel.md`](../api/buoyant_kernel.md#predicateevaluator) · records: [`model/buoyant_kernel.json`](../model/buoyant_kernel.json)

## Required

Every implementation must supply these.

```rust
fn evaluate(&self, batch: &dyn EngineData) -> DeltaResult<Box<dyn EngineData>>
```

## Implementors (1)

Read one before writing your own.

- `buoyant_kernel::engine::arrow_expression::DefaultPredicateEvaluator`

## Documentation

Trait for implementing a Predicate evaluator.

It contains one Predicate which can be evaluated on multiple ColumnarBatches.
Connectors can implement this trait to optimize the evaluation using the
connector specific capabilities.
