# `buoyant_kernel::engine::arrow_expression`

Crate `buoyant_kernel` · 3 public items · structured records in [`model/buoyant_kernel.engine.arrow_expression.json`](../model/buoyant_kernel.engine.arrow_expression.json)

## ArrowEvaluationHandler

`struct` · `buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler`

Also reachable as `delta_kernel::engine::arrow_expression::ArrowEvaluationHandler`

```rust
struct ArrowEvaluationHandler
```

**Implements**: `buoyant_kernel::EvaluationHandler`

**Derives**: Debug

**via `buoyant_kernel::EvaluationHandler`**

```rust
fn create_many(&self, schema: SchemaRef, rows: &[&[Scalar]]) -> DeltaResult<Box<dyn EngineData>>
fn new_expression_evaluator(&self, schema: SchemaRef, expression: ExpressionRef, output_type: DataType) -> DeltaResult<Arc<dyn ExpressionEvaluator>>
fn new_predicate_evaluator(&self, schema: SchemaRef, predicate: PredicateRef) -> DeltaResult<Arc<dyn PredicateEvaluator>>
fn null_row(&self, output_schema: SchemaRef) -> DeltaResult<Box<dyn EngineData>>
```

---

## DefaultExpressionEvaluator

`struct` · `buoyant_kernel::engine::arrow_expression::DefaultExpressionEvaluator`

Also reachable as `delta_kernel::engine::arrow_expression::DefaultExpressionEvaluator`

```rust
struct DefaultExpressionEvaluator
```

**Implements**: `buoyant_kernel::ExpressionEvaluator`

**Derives**: Debug

**via `buoyant_kernel::ExpressionEvaluator`**

```rust
fn evaluate(&self, batch: &dyn EngineData) -> DeltaResult<Box<dyn EngineData>>
```

---

## DefaultPredicateEvaluator

`struct` · `buoyant_kernel::engine::arrow_expression::DefaultPredicateEvaluator`

Also reachable as `delta_kernel::engine::arrow_expression::DefaultPredicateEvaluator`

```rust
struct DefaultPredicateEvaluator
```

**Implements**: `buoyant_kernel::PredicateEvaluator`

**Derives**: Debug

**via `buoyant_kernel::PredicateEvaluator`**

```rust
fn evaluate(&self, batch: &dyn EngineData) -> DeltaResult<Box<dyn EngineData>>
```

---
