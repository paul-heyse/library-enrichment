# `datafusion_physical_plan::execution_plan::check_default_invariants`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.check_default_invariants.json).

<a id="op-55643d6e49309cb47ba00c0a"></a>
## check_default_invariants

`function` · `datafusion_physical_plan::execution_plan::check_default_invariants` · datafusion-physical-plan 55.1.0

```rust
fn check_default_invariants<P: ExecutionPlan + ?Sized>(plan: &P, check: InvariantLevel) -> datafusion_common::Result<(), datafusion_common::DataFusionError>
```

Source: `src/execution_plan.rs:1637`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Checks a set of invariants that apply to all ExecutionPlan implementations.
Returns an error if the given node does not conform.
