# `datafusion_optimizer::simplify_expressions::expr_simplifier`

Crate `datafusion-optimizer` · 3 public items · structured records in [`model/datafusion_optimizer.simplify_expressions.expr_simplifier.json`](../model/datafusion_optimizer.simplify_expressions.expr_simplifier.json)

## DEFAULT_MAX_SIMPLIFIER_CYCLES

`constant` · `datafusion_optimizer::simplify_expressions::expr_simplifier::DEFAULT_MAX_SIMPLIFIER_CYCLES`

Also reachable as `datafusion_optimizer::simplify_expressions::DEFAULT_MAX_SIMPLIFIER_CYCLES`

```rust
const DEFAULT_MAX_SIMPLIFIER_CYCLES: u32 = 3
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.simplify_expressions.expr_simplifier.DEFAULT_MAX_SIMPLIFIER_CYCLES.md).


---

## THRESHOLD_INLINE_INLIST

`constant` · `datafusion_optimizer::simplify_expressions::expr_simplifier::THRESHOLD_INLINE_INLIST`

Also reachable as `datafusion_optimizer::simplify_expressions::THRESHOLD_INLINE_INLIST`

```rust
const THRESHOLD_INLINE_INLIST: usize = 3
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.simplify_expressions.expr_simplifier.THRESHOLD_INLINE_INLIST.md).


---

## ExprSimplifier

`struct` · `datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier`

Also reachable as `datafusion_optimizer::simplify_expressions::ExprSimplifier`

```rust
struct ExprSimplifier
```

**Methods** (8)

```rust
fn coerce(&self, expr: Expr, schema: &DFSchema) -> Result<Expr>
fn new(info: SimplifyContext) -> Self
fn simplify(&self, expr: Expr) -> Result<Expr>
fn simplify_with_cycle_count(&self, expr: Expr) -> Result<(Expr, u32)>
fn simplify_with_cycle_count_transformed(&self, expr: Expr) -> Result<(Transformed<Expr>, u32)>
fn with_canonicalize(self, canonicalize: bool) -> Self
fn with_guarantees(self, guarantees: Vec<(Expr, NullableInterval)>) -> Self
fn with_max_cycles(self, max_simplifier_cycles: u32) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.simplify_expressions.expr_simplifier.ExprSimplifier.md).


This structure handles API for expression simplification

Provides simplification information based on DFSchema and
[`ExecutionProps`]. This is the default implementation used by DataFusion

For example:
```
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_common::{DataFusionError, ToDFSchema};
use datafusion_expr::simplify::SimplifyContext;
use datafusion_expr::{col, lit};
use datafusion_optimizer::simplify_expressions::ExprSimplifier;

// Create the schema
let schema = Schema::new(vec![Field::new("i", DataType::Int64, false)])
    .to_dfschema_ref()
    .unwrap();

// Create the simplifier
let context = SimplifyContext::builder().with_schema(schema).build();
let simplifier = ExprSimplifier::new(context);

// Use the simplifier

// b < 2 or (1 > 3)
let expr = col("b").lt(lit(2)).or(lit(1).gt(lit(3)));

// b < 2
let simplified = simplifier.simplify(expr).unwrap();
assert_eq!(simplified, col("b").lt(lit(2)));
```

---
