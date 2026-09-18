# `datafusion_physical_expr::physical_expr`

Crate `datafusion-physical-expr` · 10 public items · structured records in [`model/datafusion_physical_expr.physical_expr.json`](../model/datafusion_physical_expr.physical_expr.json)

## add_offset_to_expr

`function` · `datafusion_physical_expr::physical_expr::add_offset_to_expr`

Also reachable as `datafusion::physical_expr::add_offset_to_expr`, `datafusion_physical_expr::add_offset_to_expr`

```rust
fn add_offset_to_expr(expr: std::sync::Arc<dyn PhysicalExpr>, offset: isize) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.physical_expr.add_offset_to_expr.md).


Adds the `offset` value to `Column` indices inside `expr`. This function is
generally used during the update of the right table schema in join operations.

---

## add_offset_to_physical_sort_exprs

`function` · `datafusion_physical_expr::physical_expr::add_offset_to_physical_sort_exprs`

Also reachable as `datafusion::physical_expr::add_offset_to_physical_sort_exprs`, `datafusion_physical_expr::add_offset_to_physical_sort_exprs`

```rust
fn add_offset_to_physical_sort_exprs(sort_exprs: impl IntoIterator<Item = PhysicalSortExpr>, offset: isize) -> datafusion_common::Result<Vec<PhysicalSortExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.physical_expr.add_offset_to_physical_sort_exprs.md).


---

## create_lex_ordering

`function` · `datafusion_physical_expr::physical_expr::create_lex_ordering`

Also reachable as `datafusion::physical_expr::create_lex_ordering`, `datafusion_physical_expr::create_lex_ordering`

```rust
fn create_lex_ordering(schema: &arrow::datatypes::SchemaRef, sort_order: &[Vec<datafusion_expr::SortExpr>], execution_props: &datafusion_expr::execution_props::ExecutionProps) -> datafusion_common::Result<Vec<LexOrdering>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.physical_expr.create_lex_ordering.md).


Creates a vector of [LexOrdering] from a vector of logical expression

---

## create_ordering

`function` · `datafusion_physical_expr::physical_expr::create_ordering`

Also reachable as `datafusion::datasource::create_ordering`, `datafusion::physical_expr::create_ordering`, `datafusion_physical_expr::create_ordering`

```rust
fn create_ordering(schema: &arrow::datatypes::Schema, sort_order: &[Vec<datafusion_expr::SortExpr>]) -> datafusion_common::Result<Vec<LexOrdering>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.physical_expr.create_ordering.md).


Converts logical sort expressions to physical sort expressions.

This function transforms a collection of logical sort expressions into their
physical representation that can be used during query execution.

# Arguments

* `schema` - The schema containing column definitions.
* `sort_order` - A collection of logical sort expressions grouped into
  lexicographic orderings.

# Returns

A vector of lexicographic orderings for physical execution, or an error if
the transformation fails.

# Examples

```
// Create orderings from columns "id" and "name"
# use arrow::datatypes::{Schema, Field, DataType};
# use datafusion_physical_expr::create_ordering;
# use datafusion_common::Column;
# use datafusion_expr::{Expr, SortExpr};
#
// Create a schema with two fields
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, false),
    Field::new("name", DataType::Utf8, false),
]);

let sort_exprs = vec![
    vec![SortExpr {
        expr: Expr::Column(Column::new(Some("t"), "id")),
        asc: true,
        nulls_first: false,
    }],
    vec![SortExpr {
        expr: Expr::Column(Column::new(Some("t"), "name")),
        asc: false,
        nulls_first: true,
    }],
];
let result = create_ordering(&schema, &sort_exprs).unwrap();
```

---

## create_physical_partitioning

`function` · `datafusion_physical_expr::physical_expr::create_physical_partitioning`

Also reachable as `datafusion::physical_expr::create_physical_partitioning`, `datafusion_physical_expr::create_physical_partitioning`

```rust
fn create_physical_partitioning(partitioning: &datafusion_expr::Partitioning, input_dfschema: &datafusion_common::DFSchema, execution_props: &datafusion_expr::execution_props::ExecutionProps, planning_ctx: &datafusion_expr::physical_planning_context::PhysicalPlanningContext) -> datafusion_common::Result<Partitioning>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.physical_expr.create_physical_partitioning.md).


Create physical partitioning from logical partitioning.

See [`create_physical_expr`] for details on the `planning_ctx` argument.

---

## create_physical_sort_expr

`function` · `datafusion_physical_expr::physical_expr::create_physical_sort_expr`

Also reachable as `datafusion::physical_expr::create_physical_sort_expr`, `datafusion_physical_expr::create_physical_sort_expr`

```rust
fn create_physical_sort_expr(e: &datafusion_expr::SortExpr, input_dfschema: &datafusion_common::DFSchema, execution_props: &datafusion_expr::execution_props::ExecutionProps, planning_ctx: &datafusion_expr::physical_planning_context::PhysicalPlanningContext) -> datafusion_common::Result<PhysicalSortExpr>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.physical_expr.create_physical_sort_expr.md).


Create a physical sort expression from a logical expression

See [`create_physical_expr`] for details on the `planning_ctx` argument.

---

## create_physical_sort_exprs

`function` · `datafusion_physical_expr::physical_expr::create_physical_sort_exprs`

Also reachable as `datafusion::physical_expr::create_physical_sort_exprs`, `datafusion_physical_expr::create_physical_sort_exprs`

```rust
fn create_physical_sort_exprs(exprs: &[datafusion_expr::SortExpr], input_dfschema: &datafusion_common::DFSchema, execution_props: &datafusion_expr::execution_props::ExecutionProps, planning_ctx: &datafusion_expr::physical_planning_context::PhysicalPlanningContext) -> datafusion_common::Result<Vec<PhysicalSortExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.physical_expr.create_physical_sort_exprs.md).


Create vector of physical sort expression from a vector of logical expression

See [`create_physical_expr`] for details on the `planning_ctx` argument.

---

## physical_exprs_bag_equal

`function` · `datafusion_physical_expr::physical_expr::physical_exprs_bag_equal`

Also reachable as `datafusion::physical_expr::physical_exprs_bag_equal`, `datafusion_physical_expr::physical_exprs_bag_equal`

```rust
fn physical_exprs_bag_equal(lhs: &[std::sync::Arc<dyn PhysicalExpr>], rhs: &[std::sync::Arc<dyn PhysicalExpr>]) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.physical_expr.physical_exprs_bag_equal.md).


Checks whether the given physical expression slices are equal in the sense
of bags (multi-sets), disregarding their orderings.

---

## physical_exprs_contains

`function` · `datafusion_physical_expr::physical_expr::physical_exprs_contains`

Also reachable as `datafusion::physical_expr::physical_exprs_contains`, `datafusion_physical_expr::physical_exprs_contains`

```rust
fn physical_exprs_contains(physical_exprs: &[std::sync::Arc<dyn PhysicalExpr>], expr: &std::sync::Arc<dyn PhysicalExpr>) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.physical_expr.physical_exprs_contains.md).


This function is similar to the `contains` method of `Vec`. It finds
whether `expr` is among `physical_exprs`.

---

## physical_exprs_equal

`function` · `datafusion_physical_expr::physical_expr::physical_exprs_equal`

Also reachable as `datafusion::physical_expr::physical_exprs_equal`, `datafusion_physical_expr::physical_exprs_equal`

```rust
fn physical_exprs_equal(lhs: &[std::sync::Arc<dyn PhysicalExpr>], rhs: &[std::sync::Arc<dyn PhysicalExpr>]) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.physical_expr.physical_exprs_equal.md).


Checks whether the given physical expression slices are equal.

---
