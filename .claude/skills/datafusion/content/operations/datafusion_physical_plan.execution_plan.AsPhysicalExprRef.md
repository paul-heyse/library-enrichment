# `datafusion_physical_plan::execution_plan::AsPhysicalExprRef`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.AsPhysicalExprRef.json).

<a id="op-a001d8107c4727717e178699"></a>
## AsPhysicalExprRef

`trait` · `datafusion_physical_plan::execution_plan::AsPhysicalExprRef` · datafusion-physical-plan 55.1.0

```rust
trait AsPhysicalExprRef
```

Source: `src/execution_plan.rs:1067`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Allows a type to be treated as a reference to an
[`Arc<dyn PhysicalExpr>`].

Used by [`apply_expression_roots`](../operations/datafusion_physical_plan.execution_plan.apply_expression_roots.md#op-d8f6900037992304af2eeed5).

Unresolved upstream links (retained, not inferred): ``Arc<dyn PhysicalExpr>``.

<a id="op-c70abe54382263362ed54b37"></a>
## as_physical_expr_ref

`function` · `datafusion_physical_plan::execution_plan::AsPhysicalExprRef::as_physical_expr_ref` · datafusion-physical-plan 55.1.0

```rust
fn as_physical_expr_ref(&self) -> &Arc<dyn PhysicalExpr>
```

Source: `src/execution_plan.rs:1069`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the referenced physical expression.
