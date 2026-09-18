# `datafusion_expr::logical_plan::plan::LogicalPlan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.LogicalPlan.json).

<a id="op-2f2092c4f87ff1cc0b33c3da"></a>
## LogicalPlan

`enum` · `datafusion_expr::logical_plan::plan::LogicalPlan` · datafusion-expr 55.1.0

```rust
enum LogicalPlan
```

Source: `src/logical_plan/plan.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A `LogicalPlan` is a node in a tree of relational operators (such as
Projection or Filter).

Represents transforming an input relation (table) to an output relation
(table) with a potentially different schema. Plans form a dataflow tree
where data flows from leaves up to the root to produce the query result.

`LogicalPlan`s can be created by the SQL query planner, the DataFrame API,
or programmatically (for example custom query languages).

# See also:
* [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc): For the expressions that are evaluated by the plan
* [`LogicalPlanBuilder`](../operations/datafusion_expr.logical_plan.builder.LogicalPlanBuilder.md#op-e319e4d14d504d0b1af00d77): For building `LogicalPlan`s
* [`tree_node`]: To inspect and rewrite `LogicalPlan`s

[`tree_node`]: crate::logical_plan::tree_node

# Examples

## Creating a LogicalPlan from SQL:

See [`SessionContext::sql`](https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.sql)

## Creating a LogicalPlan from the DataFrame API:

See [`DataFrame::logical_plan`](https://docs.rs/datafusion/latest/datafusion/dataframe/struct.DataFrame.html#method.logical_plan)

## Creating a LogicalPlan programmatically:

See [`LogicalPlanBuilder`](../operations/datafusion_expr.logical_plan.builder.LogicalPlanBuilder.md#op-e319e4d14d504d0b1af00d77)

# Visiting and Rewriting `LogicalPlan`s

Using the [`tree_node`] API, you can recursively walk all nodes in a
`LogicalPlan`. For example, to find all column references in a plan:

```
# use std::collections::HashSet;
# use arrow::datatypes::{DataType, Field, Schema};
# use datafusion_expr::{Expr, col, lit, LogicalPlan, LogicalPlanBuilder, table_scan};
# use datafusion_common::tree_node::{TreeNodeRecursion, TreeNode};
# use datafusion_common::{Column, Result};
# fn employee_schema() -> Schema {
#    Schema::new(vec![
#           Field::new("name", DataType::Utf8, false),
#           Field::new("salary", DataType::Int32, false),
#       ])
#   }
// Projection(name, salary)
//   Filter(salary > 1000)
//     TableScan(employee)
# fn main() -> Result<()> {
let plan = table_scan(Some("employee"), &employee_schema(), None)?
 .filter(col("salary").gt(lit(1000)))?
 .project(vec![col("name")])?
 .build()?;

// use apply to walk the plan and collect all expressions
let mut expressions = HashSet::new();
plan.apply(|node| {
  // collect all expressions in the plan
  node.apply_expressions(|expr| {
   expressions.insert(expr.clone());
   Ok(TreeNodeRecursion::Continue) // control walk of expressions
  })?;
  Ok(TreeNodeRecursion::Continue) // control walk of plan nodes
}).unwrap();

// we found the expression in projection and filter
assert_eq!(expressions.len(), 2);
println!("Found expressions: {:?}", expressions);
// found predicate in the Filter: employee.salary > 1000
let salary = Expr::Column(Column::new(Some("employee"), "salary"));
assert!(expressions.contains(&salary.gt(lit(1000))));
// found projection in the Projection: employee.name
let name = Expr::Column(Column::new(Some("employee"), "name"));
assert!(expressions.contains(&name));
# Ok(())
# }
```

You can also rewrite plans using the [`tree_node`] API. For example, to
replace the filter predicate in a plan:

```
# use std::collections::HashSet;
# use arrow::datatypes::{DataType, Field, Schema};
# use datafusion_expr::{Expr, col, lit, LogicalPlan, LogicalPlanBuilder, table_scan};
# use datafusion_common::tree_node::{TreeNodeRecursion, TreeNode};
# use datafusion_common::{Column, Result};
# fn employee_schema() -> Schema {
#    Schema::new(vec![
#           Field::new("name", DataType::Utf8, false),
#           Field::new("salary", DataType::Int32, false),
#       ])
#   }
// Projection(name, salary)
//   Filter(salary > 1000)
//     TableScan(employee)
# fn main() -> Result<()> {
use datafusion_common::tree_node::Transformed;
let plan = table_scan(Some("employee"), &employee_schema(), None)?
 .filter(col("salary").gt(lit(1000)))?
 .project(vec![col("name")])?
 .build()?;

// use transform to rewrite the plan
let transformed_result = plan.transform(|node| {
  // when we see the filter node
  if let LogicalPlan::Filter(mut filter) = node {
    // replace predicate with salary < 2000
    filter.predicate = Expr::Column(Column::new(Some("employee"), "salary")).lt(lit(2000));
    let new_plan = LogicalPlan::Filter(filter);
    return Ok(Transformed::yes(new_plan)); // communicate the node was changed
  }
  // return the node unchanged
  Ok(Transformed::no(node))
}).unwrap();

// Transformed result contains rewritten plan and information about
// whether the plan was changed
assert!(transformed_result.transformed);
let rewritten_plan = transformed_result.data;

// we found the filter
assert_eq!(rewritten_plan.display_indent().to_string(),
"Projection: employee.name\
\n  Filter: employee.salary < Int32(2000)\
\n    TableScan: employee");
# Ok(())
# }
```

<a id="op-bf4a033a263e04085087045f"></a>
## Aggregate

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Aggregate` · datafusion-expr 55.1.0

```rust
Aggregate
```

Source: `src/logical_plan/plan.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Aggregates its input based on a set of grouping and aggregate
expressions (e.g. SUM). This is used to implement SQL aggregates
and `GROUP BY`.

See [`Aggregate`](../operations/datafusion_expr.logical_plan.plan.Aggregate.md#op-ec4cb4b20e4e0b500a880004) for more details

<a id="op-9b114316743f9549567e0a37"></a>
## Analyze

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Analyze` · datafusion-expr 55.1.0

```rust
Analyze
```

Source: `src/logical_plan/plan.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Runs the input, and prints annotated physical plan as a string
with execution metric. This is used to implement SQL
`EXPLAIN ANALYZE`.

<a id="op-7f706e1a728011166a8b5da8"></a>
## Copy

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Copy` · datafusion-expr 55.1.0

```rust
Copy
```

Source: `src/logical_plan/plan.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`COPY TO` for writing plan results to files

<a id="op-780c1bac924552dd4d86a344"></a>
## Ddl

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Ddl` · datafusion-expr 55.1.0

```rust
Ddl
```

Source: `src/logical_plan/plan.rs:287`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Data Definition Language (DDL): CREATE / DROP TABLES / VIEWS / SCHEMAS

<a id="op-57f3d86566305bdc4f919f59"></a>
## DescribeTable

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::DescribeTable` · datafusion-expr 55.1.0

```rust
DescribeTable
```

Source: `src/logical_plan/plan.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Describe the schema of the table. This is used to implement the
SQL `DESCRIBE` command from MySQL.

<a id="op-01b098140dfdd76e72a89d1f"></a>
## Distinct

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Distinct` · datafusion-expr 55.1.0

```rust
Distinct
```

Source: `src/logical_plan/plan.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Remove duplicate rows from the input. This is used to
implement SQL `SELECT DISTINCT ...`.

<a id="op-6212fe4ac3f19c830ef381d3"></a>
## Dml

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Dml` · datafusion-expr 55.1.0

```rust
Dml
```

Source: `src/logical_plan/plan.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Data Manipulation Language (DML): Insert / Update / Delete

<a id="op-51e2a0bf1028cc782c1a5679"></a>
## EmptyRelation

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::EmptyRelation` · datafusion-expr 55.1.0

```rust
EmptyRelation
```

Source: `src/logical_plan/plan.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Produces no rows: An empty relation with an empty schema that
produces 0 or 1 row. This is used to implement SQL `SELECT`
that has no values in the `FROM` clause.

<a id="op-3e7673001f88fd4f6f2719f1"></a>
## Explain

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Explain` · datafusion-expr 55.1.0

```rust
Explain
```

Source: `src/logical_plan/plan.rs:273`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Produces a relation with string representations of
various parts of the plan. This is used to implement SQL `EXPLAIN`.

<a id="op-668ccb8cbe155ed07cf5a315"></a>
## Extension

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Extension` · datafusion-expr 55.1.0

```rust
Extension
```

Source: `src/logical_plan/plan.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Extension operator defined outside of DataFusion. This is used
to extend DataFusion with custom relational operations that

<a id="op-0db7c924f38d73ed1007a273"></a>
## Filter

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Filter` · datafusion-expr 55.1.0

```rust
Filter
```

Source: `src/logical_plan/plan.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Filters rows from its input that do not match an
expression (essentially a WHERE clause with a predicate
expression).

Semantically, `<predicate>` is evaluated for each row of the
input; If the value of `<predicate>` is true, the input row is
passed to the output. If the value of `<predicate>` is false
(or null), the row is discarded.

<a id="op-87fdef961a274cee84db03ea"></a>
## Join

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Join` · datafusion-expr 55.1.0

```rust
Join
```

Source: `src/logical_plan/plan.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Join two logical plans on one or more join columns.
This is used to implement SQL `JOIN`

<a id="op-9199caeee3f568a9d0f535b3"></a>
## Limit

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Limit` · datafusion-expr 55.1.0

```rust
Limit
```

Source: `src/logical_plan/plan.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Skip some number of rows, and then fetch some number of rows.

<a id="op-355e9362720824ea20f96f5c"></a>
## Projection

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Projection` · datafusion-expr 55.1.0

```rust
Projection
```

Source: `src/logical_plan/plan.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Evaluates an arbitrary list of expressions (essentially a
SELECT with an expression list) on its input.

<a id="op-b23126513812ba80ac42c166"></a>
## RecursiveQuery

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::RecursiveQuery` · datafusion-expr 55.1.0

```rust
RecursiveQuery
```

Source: `src/logical_plan/plan.rs:297`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A variadic query (e.g. "Recursive CTEs")

<a id="op-7bb9b257178543f2e2c9aafd"></a>
## Repartition

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Repartition` · datafusion-expr 55.1.0

```rust
Repartition
```

Source: `src/logical_plan/plan.rs:245`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Repartitions the input based on a partitioning scheme. This is
used to add parallelism and is sometimes referred to as an
"exchange" operator in other systems

<a id="op-d9dbacc2a543480f3f51a75d"></a>
## Sort

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Sort` · datafusion-expr 55.1.0

```rust
Sort
```

Source: `src/logical_plan/plan.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Sorts its input according to a list of sort expressions. This
is used to implement SQL `ORDER BY`

<a id="op-0340a5a4a6a9701976257e29"></a>
## Statement

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Statement` · datafusion-expr 55.1.0

```rust
Statement
```

Source: `src/logical_plan/plan.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A DataFusion [`Statement`](../operations/datafusion_expr.logical_plan.statement.Statement.md#op-fb6e5de33854cdf98894fa4f) such as `SET VARIABLE` or `START TRANSACTION`

<a id="op-7aea84c95b0724ddbe084378"></a>
## Subquery

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Subquery` · datafusion-expr 55.1.0

```rust
Subquery
```

Source: `src/logical_plan/plan.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Produces the output of running another query.  This is used to
implement SQL subqueries

<a id="op-977ebcce3bc94378dc3ea5b9"></a>
## SubqueryAlias

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::SubqueryAlias` · datafusion-expr 55.1.0

```rust
SubqueryAlias
```

Source: `src/logical_plan/plan.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Aliased relation provides, or changes, the name of a relation.

<a id="op-3b721470f2fbd687699e58a8"></a>
## TableScan

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::TableScan` · datafusion-expr 55.1.0

```rust
TableScan
```

Source: `src/logical_plan/plan.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Produces rows from a [`TableSource`](../operations/datafusion_expr.table_source.TableSource.md#op-d697a775a03008dbce355bf7), used to implement SQL
`FROM` tables or views.

<a id="op-3f80523be9002332b09def43"></a>
## Union

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Union` · datafusion-expr 55.1.0

```rust
Union
```

Source: `src/logical_plan/plan.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Union multiple inputs with the same schema into a single
output stream. This is used to implement SQL `UNION [ALL]` and
`INTERSECT [ALL]`.

<a id="op-1cdf6ed5deb59cb55472f8c2"></a>
## Unnest

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Unnest` · datafusion-expr 55.1.0

```rust
Unnest
```

Source: `src/logical_plan/plan.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Unnest a column that contains a nested list type such as an
ARRAY. This is used to implement SQL `UNNEST`

<a id="op-7db395a652fcd9db03d6a924"></a>
## Values

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Values` · datafusion-expr 55.1.0

```rust
Values
```

Source: `src/logical_plan/plan.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Values expression. See
[Postgres VALUES](https://www.postgresql.org/docs/current/queries-values.html)
documentation for more details. This is used to implement SQL such as
`VALUES (1, 2), (3, 4)`

<a id="op-12a35534270a5ee67324957f"></a>
## Window

`variant` · `datafusion_expr::logical_plan::plan::LogicalPlan::Window` · datafusion-expr 55.1.0

```rust
Window
```

Source: `src/logical_plan/plan.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Windows input based on a set of window spec and window
function (e.g. SUM or RANK).  This is used to implement SQL
window functions, and the `OVER` clause.

See [`Window`](../operations/datafusion_expr.logical_plan.plan.Window.md#op-7386d6673f6d599d6f92aec3) for more details

<a id="op-56660589badf2ed1c731655d"></a>
## all_out_ref_exprs

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::all_out_ref_exprs` · datafusion-expr 55.1.0

```rust
fn all_out_ref_exprs(&LogicalPlan) -> Vec<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:428`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns all the out reference(correlated) expressions (recursively) in the current
logical plan nodes and all its descendant nodes.

<a id="op-5da8c13d1e12c2c94ad45cc8"></a>
## apply_children

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::apply_children` · datafusion-expr 55.1.0

```rust
fn apply_children<'n, F: FnMut(&'n Self) -> Result<TreeNodeRecursion>>(&'n self, f: F) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [373, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNode", "path": "TreeNode"}, "trait_path": "datafusion_common::tree_node::TreeNode"}`

Source: `src/logical_plan/tree_node.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9016e5d46a244b1414456990"></a>
## apply_elements

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::apply_elements` · datafusion-expr 55.1.0

```rust
fn apply_elements<F: FnMut(&'a Self) -> Result<TreeNodeRecursion>>(&'a self, f: F) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [326, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeContainer", "path": "TreeNodeContainer"}, "trait_path": "datafusion_common::tree_node::TreeNodeContainer"}`

Source: `src/logical_plan/plan.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cadaa16cd6ade9680d6cb6a9"></a>
## apply_expressions

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::apply_expressions` · datafusion-expr 55.1.0

```rust
fn apply_expressions<F: FnMut(&Expr) -> Result<TreeNodeRecursion>>(&self, f: F) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [1036, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/tree_node.rs:418`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Calls `f` on all expressions in the current `LogicalPlan` node.

# Notes
* Similar to [`TreeNode::apply`](../operations/datafusion_common.tree_node.TreeNode.md#op-bd071500b276296c970d8bcd) but for this node's expressions.
* Does not include expressions in input `LogicalPlan` nodes
* Visits only the top level expressions (Does not recurse into each expression)

<a id="op-98ae917dee0a2ab135e1c90d"></a>
## apply_subqueries

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::apply_subqueries` · datafusion-expr 55.1.0

```rust
fn apply_subqueries<F: FnMut(&Self) -> Result<TreeNodeRecursion>>(&self, f: F) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [1036, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/tree_node.rs:913`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Similarly to [`Self::apply`](../operations/datafusion_common.tree_node.TreeNode.md#op-bd071500b276296c970d8bcd), calls `f` on this node and its inputs,
including subqueries that may appear in expressions such as `IN (SELECT
...)`.

<a id="op-3645c45d939696c6a1ec91ee"></a>
## apply_with_subqueries

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::apply_with_subqueries` · datafusion-expr 55.1.0

```rust
fn apply_with_subqueries<F: FnMut(&Self) -> Result<TreeNodeRecursion>>(&self, f: F) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [1036, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/tree_node.rs:799`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Similarly to [`Self::apply`](../operations/datafusion_common.tree_node.TreeNode.md#op-bd071500b276296c970d8bcd), calls `f` on this node and all its inputs,
including subqueries that may appear in expressions such as `IN (SELECT
...)`.

<a id="op-9fb62adc981670d4d997b85a"></a>
## check_invariants

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::check_invariants` · datafusion-expr 55.1.0

```rust
fn check_invariants(&self, check: InvariantLevel) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1217`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

checks that the plan conforms to the listed invariant level, returning an Error if not

<a id="op-a5aca85f43f0fa204a66b10b"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> LogicalPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 17], "end": [210, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3bea1d6d5adf03f4a2b1766"></a>
## columnized_output_exprs

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::columnized_output_exprs` · datafusion-expr 55.1.0

```rust
fn columnized_output_exprs(&self) -> Result<Vec<(&Expr, Column)>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1552`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get the output expressions and their corresponding columns.

The parent node may reference the output columns of the plan by expressions, such as
projection over aggregate or window functions. This method helps to convert the
referenced expressions into columns.

See also: [`crate::utils::columnize_expr`](../operations/datafusion_expr.utils.columnize_expr.md#op-c5f32dcaba2add07cb13e432)

<a id="op-469940bc20c853246ca98118"></a>
## contains_outer_reference

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::contains_outer_reference` · datafusion-expr 55.1.0

```rust
fn contains_outer_reference(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1531`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

If this node's expressions contains any references to an outer subquery

<a id="op-968776d6de173289108d970d"></a>
## default

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::default` · datafusion-expr 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [300, 1], "end": [310, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/logical_plan/plan.rs:301`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f72d11dc987e55842fc92d2a"></a>
## describe_schema

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::describe_schema` · datafusion-expr 55.1.0

```rust
fn describe_schema() -> Schema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:391`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the (fixed) output schema for `DESCRIBE` plans

<a id="op-a74af8650ebcf89cbff8609b"></a>
## display

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::display` · datafusion-expr 55.1.0

```rust
fn display(&self) -> impl Display + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 1], "end": [2266, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1906`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a `format`able structure with the a human readable
description of this LogicalPlan node per node, not including
children. For example:

```text
Projection: id
```
```
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_expr::{col, lit, logical_plan::table_scan, LogicalPlanBuilder};
let schema = Schema::new(vec![Field::new("id", DataType::Int32, false)]);
let plan = table_scan(Some("t1"), &schema, None)
    .unwrap()
    .build()
    .unwrap();

// Format using display
let display_string = format!("{}", plan.display());

assert_eq!("TableScan: t1", display_string);
```

<a id="op-af2e3b07ed6190436d1dbbf0"></a>
## display_graphviz

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::display_graphviz` · datafusion-expr 55.1.0

```rust
fn display_graphviz(&self) -> impl Display + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 1], "end": [2266, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1855`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a `format`able structure that produces lines meant for
graphical display using the `DOT` language. This format can be
visualized using software from
[`graphviz`](https://graphviz.org/)

This currently produces two graphs -- one with the basic
structure, and one with additional details such as schema.

```
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_expr::{col, lit, logical_plan::table_scan, LogicalPlanBuilder};
let schema = Schema::new(vec![Field::new("id", DataType::Int32, false)]);
let plan = table_scan(Some("t1"), &schema, None)
    .unwrap()
    .filter(col("id").eq(lit(5)))
    .unwrap()
    .build()
    .unwrap();

// Format using display_graphviz
let graphviz_string = format!("{}", plan.display_graphviz());
```

If graphviz string is saved to a file such as `/tmp/example.dot`, the following
commands can be used to render it as a pdf:

```bash
  dot -Tpdf < /tmp/example.dot  > /tmp/example.pdf
```

<a id="op-cd42c6d6c1a673f15fa02074"></a>
## display_indent

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::display_indent` · datafusion-expr 55.1.0

```rust
fn display_indent(&self) -> impl Display + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 1], "end": [2266, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1743`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a `format`able structure that produces a single line
per node.

# Example

```text
Projection: employee.id
   Filter: employee.state Eq Utf8(\"CO\")\
      CsvScan: employee projection=Some([0, 3])
```

```
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_expr::{col, lit, logical_plan::table_scan, LogicalPlanBuilder};
let schema = Schema::new(vec![Field::new("id", DataType::Int32, false)]);
let plan = table_scan(Some("t1"), &schema, None)
    .unwrap()
    .filter(col("id").eq(lit(5)))
    .unwrap()
    .build()
    .unwrap();

// Format using display_indent
let display_string = format!("{}", plan.display_indent());

assert_eq!("Filter: t1.id = Int32(5)\n  TableScan: t1", display_string);
```

<a id="op-8356380124e8bc3da098b0d6"></a>
## display_indent_schema

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::display_indent_schema` · datafusion-expr 55.1.0

```rust
fn display_indent_schema(&self) -> impl Display + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 1], "end": [2266, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1789`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a `format`able structure that produces a single line
per node that includes the output schema. For example:

```text
Projection: employee.id [id:Int32]\
   Filter: employee.state = Utf8(\"CO\") [id:Int32, state:Utf8]\
     TableScan: employee projection=[0, 3] [id:Int32, state:Utf8]";
```

```
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_expr::{col, lit, logical_plan::table_scan, LogicalPlanBuilder};
let schema = Schema::new(vec![Field::new("id", DataType::Int32, false)]);
let plan = table_scan(Some("t1"), &schema, None)
    .unwrap()
    .filter(col("id").eq(lit(5)))
    .unwrap()
    .build()
    .unwrap();

// Format using display_indent_schema
let display_string = format!("{}", plan.display_indent_schema());

assert_eq!(
    "Filter: t1.id = Int32(5) [id:Int32]\
            \n  TableScan: t1 [id:Int32]",
    display_string
);
```

<a id="op-c21920f4609617e0c39fe834"></a>
## display_pg_json

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::display_pg_json` · datafusion-expr 55.1.0

```rust
fn display_pg_json(&self) -> impl Display + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 1], "end": [2266, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1809`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a displayable structure that produces plan in postgresql JSON format.

Users can use this format to visualize the plan in existing plan visualization tools, for example [dalibo](https://explain.dalibo.com/)

<a id="op-0ad10c31a5b47b9745ba1523"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &LogicalPlan) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 24], "end": [210, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b61ca3de77636aafd16d9fa"></a>
## explain_schema

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::explain_schema` · datafusion-expr 55.1.0

```rust
fn explain_schema() -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:383`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the (fixed) output schema for explain plans

<a id="op-586c0dd5f6fcdb0f90eeae74"></a>
## expressions

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::expressions` · datafusion-expr 55.1.0

```rust
fn expressions(&LogicalPlan) -> Vec<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:415`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns all expressions (non-recursively) evaluated by the current
logical plan node. This does not include expressions in any children.

Note this method `clone`s all the expressions. When possible, the
[`tree_node`] API should be used instead of this API.

The returned expressions do not necessarily represent or even
contributed to the output schema of this node. For example,
`LogicalPlan::Filter` returns the filter expression even though the
output of a Filter has the same columns as the input.

The expressions do contain all the columns that are used by this plan,
so if there are columns not referenced by these expressions then
DataFusion's optimizer attempts to optimize them away.

[`tree_node`]: crate::logical_plan::tree_node

<a id="op-ef485b8bb35a84cd8922e820"></a>
## fallback_normalize_schemas

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::fallback_normalize_schemas` · datafusion-expr 55.1.0

```rust
fn fallback_normalize_schemas(&self) -> Vec<&DFSchema>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Used for normalizing columns, as the fallback schemas to the main schema
of the plan.

<a id="op-dd3ad856920593205fe59a29"></a>
## fetch

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::fetch` · datafusion-expr 55.1.0

```rust
fn fetch(&self) -> Result<Option<usize>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1497`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the fetch (limit) of this plan node, if it has one.

[`LogicalPlan::Sort`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-d9dbacc2a543480f3f51a75d), [`LogicalPlan::TableScan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-3b721470f2fbd687699e58a8), and
[`LogicalPlan::Limit`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-9199caeee3f568a9d0f535b3) may carry a fetch value; all other variants
return `Ok(None)`.

<a id="op-c5ce06f18068d610102d6738"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2268, 1], "end": [2272, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/logical_plan/plan.rs:2269`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f82b2d65f344101f6e43f34b"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 10], "end": [210, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d939dc6a424fc0412d9f81fd"></a>
## get_parameter_fields

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::get_parameter_fields` · datafusion-expr 55.1.0

```rust
fn get_parameter_fields(&self) -> Result<HashMap<String, Option<FieldRef>>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 1], "end": [2266, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1678`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Walk the logical plan, find any `Placeholder` tokens, and return a map of their IDs and FieldRefs

<a id="op-97bcc34c29a8eb945a07eda2"></a>
## get_parameter_names

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::get_parameter_names` · datafusion-expr 55.1.0

```rust
fn get_parameter_names(&self) -> Result<HashSet<String>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 1], "end": [2266, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1646`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Walk the logical plan, find any `Placeholder` tokens, and return a set of their names.

<a id="op-3d0c18fd13b33febcf1c9358"></a>
## get_parameter_types

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::get_parameter_types` · datafusion-expr 55.1.0

```rust
fn get_parameter_types(&self) -> Result<HashMap<String, Option<DataType>>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 1], "end": [2266, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1665`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Walk the logical plan, find any `Placeholder` tokens, and return a map of their IDs and DataTypes

Note that this will drop any extension or field metadata attached to parameters. Use
[`LogicalPlan::get_parameter_fields`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-d939dc6a424fc0412d9f81fd) to keep extension metadata.

<a id="op-9069910547994625cb9a680b"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 51], "end": [210, 55], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fa57df7fa72cb34c8abc994"></a>
## head_output_expr

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::head_output_expr` · datafusion-expr 55.1.0

```rust
fn head_output_expr(&self) -> Result<Option<Expr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:530`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

returns the first output expression of this `LogicalPlan` node.

<a id="op-ab83f9bc0afdc0bcf1753a7d"></a>
## inputs

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::inputs` · datafusion-expr 55.1.0

```rust
fn inputs(&self) -> Vec<&LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:454`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns all inputs / children of this `LogicalPlan` node.

Note does not include inputs to inputs, or subqueries.

<a id="op-dd2607b8f62b1af6eaee90dd"></a>
## map_children

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::map_children` · datafusion-expr 55.1.0

```rust
fn map_children<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [373, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNode", "path": "TreeNode"}, "trait_path": "datafusion_common::tree_node::TreeNode"}`

Source: `src/logical_plan/tree_node.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Applies `f` to each child (input) of this plan node, rewriting them *in place.*

# Notes

Inputs include ONLY direct children, not embedded `LogicalPlan`s for
subqueries, for example such as are in [`Expr::Exists`].

[`Expr::Exists`]: crate::Expr::Exists

<a id="op-d35819d33ff4560d47fbc19f"></a>
## map_elements

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::map_elements` · datafusion-expr 55.1.0

```rust
fn map_elements<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [326, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}}], "constraints": []}}, "id": "datafusion_common::tree_node::TreeNodeContainer", "path": "TreeNodeContainer"}, "trait_path": "datafusion_common::tree_node::TreeNodeContainer"}`

Source: `src/logical_plan/plan.rs:320`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5aa96e67c3559c62a7922a8a"></a>
## map_expressions

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::map_expressions` · datafusion-expr 55.1.0

```rust
fn map_expressions<F: FnMut(Expr) -> Result<Transformed<Expr>>>(self, f: F) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [1036, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/tree_node.rs:510`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Rewrites all expressions in the current `LogicalPlan` node using `f`.

Returns the current node.

# Notes
* Similar to [`TreeNode::map_children`](../operations/datafusion_common.tree_node.TreeNode.md#op-68847d001fe9160150e23725) but for this node's expressions.
* Visits only the top level expressions (Does not recurse into each expression)

<a id="op-a12fe5f5d618b539a3e48e66"></a>
## map_subqueries

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::map_subqueries` · datafusion-expr 55.1.0

```rust
fn map_subqueries<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [1036, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/tree_node.rs:961`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Similarly to [`Self::map_children`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-dd2607b8f62b1af6eaee90dd), rewrites all subqueries that may
appear in expressions such as `IN (SELECT ...)` using `f`.

Returns the current node.

<a id="op-4d39975fdd3173f8ea02ccce"></a>
## map_uncorrelated_subqueries

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::map_uncorrelated_subqueries` · datafusion-expr 55.1.0

```rust
fn map_uncorrelated_subqueries<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [1036, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/tree_node.rs:1025`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Similar to [`Self::map_subqueries`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-a12fe5f5d618b539a3e48e66), but only applies `f` to
uncorrelated subqueries (those with no outer column references).

<a id="op-59f27ef3eae861df6f02d95d"></a>
## max_rows

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::max_rows` · datafusion-expr 55.1.0

```rust
fn max_rows(&LogicalPlan) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1366`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the maximum number of rows that this plan can output, if known.

If `None`, the plan can return any number of rows.
If `Some(n)` then the plan can return at most `n` rows but may return fewer.

<a id="op-f83aab7b15e95395a2305155"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &LogicalPlan) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 39], "end": [210, 49], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fac9cbd6556a5b3e75e5917"></a>
## recompute_schema

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::recompute_schema` · datafusion-expr 55.1.0

```rust
fn recompute_schema(self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:628`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Recomputes schema and type information for this LogicalPlan if needed.

Some `LogicalPlan`s may need to recompute their schema if the number or
type of expressions have been changed (for example due to type
coercion). For example [`LogicalPlan::Projection`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-355e9362720824ea20f96f5c)s schema depends on
its expressions.

Some `LogicalPlan`s schema is unaffected by any changes to their
expressions. For example [`LogicalPlan::Filter`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-0db7c924f38d73ed1007a273) schema is always the
same as its input schema.

This is useful after modifying a plans `Expr`s (or input plans) via
methods such as [Self::map_children](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-dd2607b8f62b1af6eaee90dd) and [Self::map_expressions](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-5aa96e67c3559c62a7922a8a). Unlike
[Self::with_new_exprs](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-48b69499563a92bef71505fb), this method does not require a new set of
expressions or inputs plans.

# Return value
Returns an error if there is some issue recomputing the schema.

# Notes

* Does not recursively recompute schema for input (child) plans.

<a id="op-15e279ebf413c8de0df69c19"></a>
## replace_params_with_values

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::replace_params_with_values` · datafusion-expr 55.1.0

```rust
fn replace_params_with_values(self, param_values: &ParamValues) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 1], "end": [2266, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1592`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a `LogicalPlan` with all placeholders (e.g $1 $2,
...) replaced with corresponding values provided in
`params_values`

See [`Self::with_param_values`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-657f9d6ffce2cb68d278ba81) for examples and usage with an owned
`ParamValues`

<a id="op-d1d663c1430a1eb0e6c976f3"></a>
## resolve_lambda_variables

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::resolve_lambda_variables` · datafusion-expr 55.1.0

```rust
fn resolve_lambda_variables(self) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 1], "end": [2266, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:2259`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a `LogicalPLan` with all [`LambdaVariable`]'s resolved

[`LambdaVariable`]: crate::expr::LambdaVariable

<a id="op-20798c575f68193a4ddc1f04"></a>
## rewrite_with_subqueries

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::rewrite_with_subqueries` · datafusion-expr 55.1.0

```rust
fn rewrite_with_subqueries<R: TreeNodeRewriter<Node = Self>>(self, rewriter: &mut R) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [1036, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/tree_node.rs:784`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Similarly to [`Self::rewrite`](../operations/datafusion_common.tree_node.TreeNode.md#op-af82c9ed79a9f7ee9db9ed17), rewrites this node and its inputs using `f`,
including subqueries that may appear in expressions such as `IN (SELECT
...)`.

<a id="op-01e2eee13c72fc93809fff67"></a>
## schema

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::schema` · datafusion-expr 55.1.0

```rust
fn schema(&self) -> &DFSchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get a reference to the logical plan's schema

<a id="op-1d7cb09e71bc312234a42cad"></a>
## skip

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::skip` · datafusion-expr 55.1.0

```rust
fn skip(&self) -> Result<Option<usize>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1458`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the skip (offset) of this plan node, if it has one.

Only [`LogicalPlan::Limit`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-9199caeee3f568a9d0f535b3) carries a skip value; all other variants
return `Ok(None)`. Returns `Ok(None)` for a zero skip.

<a id="op-f75d2299eca33446700e5fa9"></a>
## to_stringified

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::to_stringified` · datafusion-expr 55.1.0

```rust
fn to_stringified(&self, plan_type: PlanType) -> StringifiedPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2274, 1], "end": [2278, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "datafusion_common::display::ToStringifiedPlan", "path": "ToStringifiedPlan"}, "trait_path": "datafusion_common::display::ToStringifiedPlan"}`

Source: `src/logical_plan/plan.rs:2275`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e03c77d2af1558ccae3bb87"></a>
## transform_down_up_with_subqueries

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::transform_down_up_with_subqueries` · datafusion-expr 55.1.0

```rust
fn transform_down_up_with_subqueries<FD: FnMut(Self) -> Result<Transformed<Self>>, FU: FnMut(Self) -> Result<Transformed<Self>>>(self, f_down: FD, f_up: FU) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [1036, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/tree_node.rs:883`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Similarly to [`Self::transform_down`](../operations/datafusion_common.tree_node.TreeNode.md#op-7f8f7d15d1860327f1638078), rewrites this node and its inputs using `f`,
including subqueries that may appear in expressions such as `IN (SELECT
...)`.

<a id="op-c30452bcb69399869beb6f75"></a>
## transform_down_with_subqueries

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::transform_down_with_subqueries` · datafusion-expr 55.1.0

```rust
fn transform_down_with_subqueries<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [1036, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/tree_node.rs:834`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Similarly to [`Self::transform_down`](../operations/datafusion_common.tree_node.TreeNode.md#op-7f8f7d15d1860327f1638078), rewrites this node and its inputs using `f`,
including subqueries that may appear in expressions such as `IN (SELECT
...)`.

<a id="op-30a05ed218343725ef8ff4e8"></a>
## transform_up_with_subqueries

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::transform_up_with_subqueries` · datafusion-expr 55.1.0

```rust
fn transform_up_with_subqueries<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [1036, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/tree_node.rs:859`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Similarly to [`Self::transform_up`](../operations/datafusion_common.tree_node.TreeNode.md#op-86d9d244e6b5fe16b173f4ed), rewrites this node and its inputs using `f`,
including subqueries that may appear in expressions such as `IN (SELECT
...)`.

<a id="op-28c839c1a19c721fdcc8ac78"></a>
## transform_with_subqueries

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::transform_with_subqueries` · datafusion-expr 55.1.0

```rust
fn transform_with_subqueries<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [1036, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/tree_node.rs:824`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Similarly to [`Self::transform`](../operations/datafusion_common.tree_node.TreeNode.md#op-335537caf61ac03893f423ab), rewrites this node and its inputs using `f`,
including subqueries that may appear in expressions such as `IN (SELECT
...)`.

<a id="op-046ff8974eb5b8e56fd605a5"></a>
## using_columns

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::using_columns` · datafusion-expr 55.1.0

```rust
fn using_columns(&self) -> Result<Vec<HashSet<Column>>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:494`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

returns all `Using` join columns in a logical plan

<a id="op-d2f725875c279049fc96f907"></a>
## visit_with_subqueries

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::visit_with_subqueries` · datafusion-expr 55.1.0

```rust
fn visit_with_subqueries<V: for<'n> TreeNodeVisitor<'n, Node = Self>>(&self, visitor: &mut V) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "crate::LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [1036, 2], "filename": "src/logical_plan/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/tree_node.rs:765`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Visits a plan similarly to [`Self::visit`](../operations/datafusion_common.tree_node.TreeNode.md#op-b7ab737d845b0a801118a434), including subqueries that
may appear in expressions such as `IN (SELECT ...)`.

<a id="op-48b69499563a92bef71505fb"></a>
## with_new_exprs

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::with_new_exprs` · datafusion-expr 55.1.0

```rust
fn with_new_exprs(&self, expr: Vec<Expr>, inputs: Vec<LogicalPlan>) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:796`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a new `LogicalPlan` based on `self` with inputs and
expressions replaced.

Note this method creates an entirely new node, which requires a large
amount of clone'ing. When possible, the [`tree_node`] API should be used
instead of this API.

The exprs correspond to the same order of expressions returned
by [`Self::expressions`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-586c0dd5f6fcdb0f90eeae74). This function is used by optimizers
to rewrite plans using the following pattern:

[`tree_node`]: crate::logical_plan::tree_node

```text
let new_inputs = optimize_children(..., plan, props);

// get the plans expressions to optimize
let exprs = plan.expressions();

// potentially rewrite plan expressions
let rewritten_exprs = rewrite_exprs(exprs);

// create new plan using rewritten_exprs in same position
let new_plan = plan.new_with_exprs(rewritten_exprs, new_inputs);
```

<a id="op-657f9d6ffce2cb68d278ba81"></a>
## with_param_values

`function` · `datafusion_expr::logical_plan::plan::LogicalPlan::with_param_values` · datafusion-expr 55.1.0

```rust
fn with_param_values(self, param_values: impl Into<ParamValues>) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [1583, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:1341`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Replaces placeholder param values (like `$1`, `$2`) in [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da)
with the specified `param_values`.

[`Prepare`](../operations/datafusion_expr.logical_plan.statement.Prepare.md#op-6e444d0f4fa105dfc8b9126e) statements are converted to
their inner logical plan for execution.

# Example
```
# use arrow::datatypes::{Field, Schema, DataType};
use datafusion_common::ScalarValue;
# use datafusion_expr::{lit, col, LogicalPlanBuilder, logical_plan::table_scan, placeholder};
# let schema = Schema::new(vec![
#     Field::new("id", DataType::Int32, false),
# ]);
// Build SELECT * FROM t1 WHERE id = $1
let plan = table_scan(Some("t1"), &schema, None).unwrap()
    .filter(col("id").eq(placeholder("$1"))).unwrap()
    .build().unwrap();

assert_eq!(
  "Filter: t1.id = $1\
  \n  TableScan: t1",
  plan.display_indent().to_string()
);

// Fill in the parameter $1 with a literal 3
let plan = plan.with_param_values(vec![
  ScalarValue::from(3i32) // value at index 0 --> $1
]).unwrap();

assert_eq!(
   "Filter: t1.id = Int32(3)\
   \n  TableScan: t1",
   plan.display_indent().to_string()
 );

// Note you can also used named parameters
// Build SELECT * FROM t1 WHERE id = $my_param
let plan = table_scan(Some("t1"), &schema, None).unwrap()
    .filter(col("id").eq(placeholder("$my_param"))).unwrap()
    .build().unwrap()
    // Fill in the parameter $my_param with a literal 3
    .with_param_values(vec![
      ("my_param", ScalarValue::from(3i32)),
    ]).unwrap();

assert_eq!(
   "Filter: t1.id = Int32(3)\
   \n  TableScan: t1",
   plan.display_indent().to_string()
 );
```
