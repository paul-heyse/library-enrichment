# `datafusion_physical_plan`

Crate `datafusion-physical-plan` · 4 public items · structured records in [`model/datafusion_physical_plan.json`](../model/datafusion_physical_plan.json)

## check_if_same_properties

`macro` · `datafusion_physical_plan::check_if_same_properties`

Also reachable as `datafusion::physical_plan::check_if_same_properties`

```rust
macro_rules! check_if_same_properties
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.check_if_same_properties.md).


Helper macro to avoid properties re-computation if passed children properties
the same as plan already has. Could be used to implement fast-path for method
[`ExecutionPlan::with_new_children`].

New call sites should route through [`replace_children_if_necessary`],
which applies this check together with the child-pointer short-circuit
(see [`replace_children_if_necessary`] for the layered policy). This
macro remains for direct-caller sites that have not been migrated yet.

---

## expect_plan_variant

`macro` · `datafusion_physical_plan::expect_plan_variant`

Also reachable as `datafusion::physical_plan::expect_plan_variant`

```rust
macro_rules! expect_plan_variant
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.expect_plan_variant.md).


Assert that a [`PhysicalPlanNode`] carries the expected `PhysicalPlanType`
variant, returning a reference to the inner payload, else an `internal_err!`.
Mirrors `expect_expr_variant!` on the expression side. Field access on the
result auto-derefs through the `Box` that boxed variants use.

---

## handle_state

`macro` · `datafusion_physical_plan::handle_state`

Also reachable as `datafusion::physical_plan::handle_state`

```rust
macro_rules! handle_state
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.handle_state.md).


The `handle_state` macro is designed to process the result of a state-changing
operation. It operates on a `StatefulStreamResult` by matching its variants and
executing corresponding actions. This macro is used to streamline code that deals
with state transitions, reducing boilerplate and improving readability.

# Cases

- `Ok(StatefulStreamResult::Continue)`: Continues the loop, indicating the
  stream join operation should proceed to the next step.
- `Ok(StatefulStreamResult::Ready(result))`: Returns a `Poll::Ready` with the
  result, either yielding a value or indicating the stream is awaiting more
  data.
- `Err(e)`: Returns a `Poll::Ready` containing an error, signaling an issue
  during the stream join operation.

# Arguments

* `$match_case`: An expression that evaluates to a `Result<StatefulStreamResult<_>>`.

---

## validate_child_count

`macro` · `datafusion_physical_plan::validate_child_count`

Also reachable as `datafusion::physical_plan::validate_child_count`

```rust
macro_rules! validate_child_count
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.validate_child_count.md).


Helper macro to validate that replacement children match a plan's existing
child count.

This is useful for [`ExecutionPlan::replace_children`] implementations that
need to preserve the same child-count validation behavior.

---
