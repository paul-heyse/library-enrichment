# `datafusion_physical_plan::visitor`

Crate `datafusion-physical-plan` · 3 public items · structured records in [`model/datafusion_physical_plan.visitor.json`](../model/datafusion_physical_plan.visitor.json)

## accept

`function` · `datafusion_physical_plan::visitor::accept`

Also reachable as `datafusion::physical_plan::accept`, `datafusion_physical_plan::accept`

```rust
fn accept<V: ExecutionPlanVisitor>(plan: &dyn ExecutionPlan, visitor: &mut V) -> Result<(), V::Error>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.visitor.accept.md).


Visit all children of this plan, according to the order defined on `ExecutionPlanVisitor`.

---

## visit_execution_plan

`function` · `datafusion_physical_plan::visitor::visit_execution_plan`

Also reachable as `datafusion::physical_plan::visit_execution_plan`, `datafusion_physical_plan::visit_execution_plan`

```rust
fn visit_execution_plan<V: ExecutionPlanVisitor>(plan: &dyn ExecutionPlan, visitor: &mut V) -> Result<(), V::Error>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.visitor.visit_execution_plan.md).


Recursively calls `pre_visit` and `post_visit` for this node and
all of its children, as described on [`ExecutionPlanVisitor`]

---

## ExecutionPlanVisitor

`trait` · `datafusion_physical_plan::visitor::ExecutionPlanVisitor`

Also reachable as `datafusion::physical_plan::ExecutionPlanVisitor`, `datafusion_physical_plan::ExecutionPlanVisitor`

```rust
trait ExecutionPlanVisitor
```

**Methods** (2)

```rust
fn post_visit(&mut self, _plan: &dyn ExecutionPlan) -> Result<bool, Self::Error>
fn pre_visit(&mut self, plan: &dyn ExecutionPlan) -> Result<bool, Self::Error>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.visitor.ExecutionPlanVisitor.md).


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

---
