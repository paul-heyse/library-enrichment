# `datafusion_physical_plan::visitor::ExecutionPlanVisitor`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.visitor.ExecutionPlanVisitor.json).

<a id="op-2b3e60aa91348196cf730331"></a>
## ExecutionPlanVisitor

`trait` · `datafusion_physical_plan::visitor::ExecutionPlanVisitor` · datafusion-physical-plan 55.1.0

```rust
trait ExecutionPlanVisitor
```

Source: `src/visitor.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Trait that implements the [Visitor
pattern](https://en.wikipedia.org/wiki/Visitor_pattern) for a
depth first walk of `ExecutionPlan` nodes. `pre_visit` is called
before any children are visited, and then `post_visit` is called
after all children have been visited.

To use, define a struct that implements this trait and then invoke
['accept'].

For example, for an execution plan that looks like:

```text
ProjectionExec: id
   FilterExec: state = CO
      DataSourceExec:
```

The sequence of visit operations would be:
```text
visitor.pre_visit(ProjectionExec)
visitor.pre_visit(FilterExec)
visitor.pre_visit(DataSourceExec)
visitor.post_visit(DataSourceExec)
visitor.post_visit(FilterExec)
visitor.post_visit(ProjectionExec)
```

<a id="op-9604746e7780e0c91e7c5cf7"></a>
## Error

`assoc_type` · `datafusion_physical_plan::visitor::ExecutionPlanVisitor::Error` · datafusion-physical-plan 55.1.0

```rust
Error
```

Source: `src/visitor.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The type of error returned by this visitor

<a id="op-3fde6dec9aa85d7318f226ce"></a>
## post_visit

`function` · `datafusion_physical_plan::visitor::ExecutionPlanVisitor::post_visit` · datafusion-physical-plan 55.1.0

```rust
fn post_visit(&mut self, _plan: &dyn ExecutionPlan) -> Result<bool, Self::Error>
```

Source: `src/visitor.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Invoked on an `ExecutionPlan` plan *after* all of its child
inputs have been visited. The return value is handled the same
as the return value of `pre_visit`. The provided default
implementation returns `Ok(true)`.

<a id="op-9c89a230057f79f434fcade4"></a>
## pre_visit

`function` · `datafusion_physical_plan::visitor::ExecutionPlanVisitor::pre_visit` · datafusion-physical-plan 55.1.0

```rust
fn pre_visit(&mut self, plan: &dyn ExecutionPlan) -> Result<bool, Self::Error>
```

Source: `src/visitor.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Invoked on an `ExecutionPlan` plan before any of its child
inputs have been visited. If Ok(true) is returned, the
recursion continues. If Err(..) or Ok(false) are returned, the
recursion stops immediately and the error, if any, is returned
to `accept`
