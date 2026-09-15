# `datafusion_expr::logical_plan::plan`

Crate `datafusion-expr` · 31 public items · structured records in [`model/datafusion_expr.logical_plan.plan.json`](../model/datafusion_expr.logical_plan.plan.json)

## Distinct

`enum` · `datafusion_expr::logical_plan::plan::Distinct`

Also reachable as `datafusion::logical_expr::Distinct`, `datafusion_expr::Distinct`, `datafusion_expr::logical_plan::Distinct`

```rust
enum Distinct
```

**Variants**: `All`, `On`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn input(&self) -> &Arc<LogicalPlan>
```

Removes duplicate rows from the input

---

## FetchType

`enum` · `datafusion_expr::logical_plan::plan::FetchType`

Also reachable as `datafusion::logical_expr::FetchType`, `datafusion_expr::FetchType`, `datafusion_expr::logical_plan::FetchType`

```rust
enum FetchType
```

**Variants**: `Literal`, `UnsupportedExpr`

Different types of fetch expression in Limit plan.

---

## LogicalPlan

`enum` · `datafusion_expr::logical_plan::plan::LogicalPlan`

Also reachable as `datafusion::logical_expr::LogicalPlan`, `datafusion_expr::LogicalPlan`, `datafusion_expr::logical_plan::LogicalPlan`

```rust
enum LogicalPlan
```

**Variants**: `Projection`, `Filter`, `Window`, `Aggregate`, `Sort`, `Join`, `Repartition`, `Union`, `TableScan`, `EmptyRelation`, `Subquery`, `SubqueryAlias`, `Limit`, `Statement`, `Values`, `Explain`, `Analyze`, `Extension`, `Distinct`, `Dml`, `Ddl`, `Copy`, `DescribeTable`, `Unnest`, `RecursiveQuery`

**Implements**: `core::fmt::Display`, `datafusion_common::display::ToStringifiedPlan`, `datafusion_common::tree_node::TreeNode`, `datafusion_common::tree_node::TreeNodeContainer`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (40)

```rust
fn all_out_ref_exprs(&LogicalPlan) -> Vec<Expr>
fn apply_expressions<F: FnMut(&Expr) -> Result<TreeNodeRecursion>>(&self, f: F) -> Result<TreeNodeRecursion>
fn apply_subqueries<F: FnMut(&Self) -> Result<TreeNodeRecursion>>(&self, f: F) -> Result<TreeNodeRecursion>
fn apply_with_subqueries<F: FnMut(&Self) -> Result<TreeNodeRecursion>>(&self, f: F) -> Result<TreeNodeRecursion>
fn check_invariants(&self, check: InvariantLevel) -> Result<()>
fn columnized_output_exprs(&self) -> Result<Vec<(&Expr, Column)>>
fn contains_outer_reference(&self) -> bool
fn describe_schema() -> Schema
fn display(&self) -> impl Display + '_
fn display_graphviz(&self) -> impl Display + '_
fn display_indent(&self) -> impl Display + '_
fn display_indent_schema(&self) -> impl Display + '_
fn display_pg_json(&self) -> impl Display + '_
fn explain_schema() -> SchemaRef
fn expressions(&LogicalPlan) -> Vec<Expr>
fn fallback_normalize_schemas(&self) -> Vec<&DFSchema>
fn fetch(&self) -> Result<Option<usize>>
fn get_parameter_fields(&self) -> Result<HashMap<String, Option<FieldRef>>, DataFusionError>
fn get_parameter_names(&self) -> Result<HashSet<String>>
fn get_parameter_types(&self) -> Result<HashMap<String, Option<DataType>>, DataFusionError>
fn head_output_expr(&self) -> Result<Option<Expr>>
fn inputs(&self) -> Vec<&LogicalPlan>
fn map_expressions<F: FnMut(Expr) -> Result<Transformed<Expr>>>(self, f: F) -> Result<Transformed<Self>>
fn map_subqueries<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
fn map_uncorrelated_subqueries<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
fn max_rows(&LogicalPlan) -> Option<usize>
fn recompute_schema(self) -> Result<Self>
fn replace_params_with_values(self, param_values: &ParamValues) -> Result<LogicalPlan>
fn resolve_lambda_variables(self) -> Result<Transformed<LogicalPlan>>
fn rewrite_with_subqueries<R: TreeNodeRewriter<Node = Self>>(self, rewriter: &mut R) -> Result<Transformed<Self>>
fn schema(&self) -> &DFSchemaRef
fn skip(&self) -> Result<Option<usize>>
fn transform_down_up_with_subqueries<FD: FnMut(Self) -> Result<Transformed<Self>>, FU: FnMut(Self) -> Result<Transformed<Self>>>(self, f_down: FD, f_up: FU) -> Result<Transformed<Self>>
fn transform_down_with_subqueries<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
fn transform_up_with_subqueries<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
fn transform_with_subqueries<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
fn using_columns(&self) -> Result<Vec<HashSet<Column>>, DataFusionError>
fn visit_with_subqueries<V: for<'n> TreeNodeVisitor<'n, Node = Self>>(&self, visitor: &mut V) -> Result<TreeNodeRecursion>
fn with_new_exprs(&self, expr: Vec<Expr>, inputs: Vec<LogicalPlan>) -> Result<LogicalPlan>
fn with_param_values(self, param_values: impl Into<ParamValues>) -> Result<LogicalPlan>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `datafusion_common::display::ToStringifiedPlan`**

```rust
fn to_stringified(&self, plan_type: PlanType) -> StringifiedPlan
```

**via `datafusion_common::tree_node::TreeNode`**

```rust
fn apply_children<'n, F: FnMut(&'n Self) -> Result<TreeNodeRecursion>>(&'n self, f: F) -> Result<TreeNodeRecursion>
fn map_children<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

**via `datafusion_common::tree_node::TreeNodeContainer`**

```rust
fn apply_elements<F: FnMut(&'a Self) -> Result<TreeNodeRecursion>>(&'a self, f: F) -> Result<TreeNodeRecursion>
fn map_elements<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
```

A `LogicalPlan` is a node in a tree of relational operators (such as
Projection or Filter).

Represents transforming an input relation (table) to an output relation
(table) with a potentially different schema. Plans form a dataflow tree
where data flows from leaves up to the root to produce the query result.

`LogicalPlan`s can be created by the SQL query planner, the DataFrame API,
or programmatically (for example custom query languages).

# See also:
* [`Expr`]: For the expressions that are evaluated by the plan
* [`LogicalPlanBuilder`]: For building `LogicalPlan`s
* [`tree_node`]: To inspect and rewrite `LogicalPlan`s

[`tree_node`]: crate::logical_plan::tree_node

# Examples

## Creating a LogicalPlan from SQL:

See [`SessionContext::sql`](https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.sql)

## Creating a LogicalPlan from the DataFrame API:

See [`DataFrame::logical_plan`](https://docs.rs/datafusion/latest/datafusion/dataframe/struct.DataFrame.html#method.logical_plan)

## Creating a LogicalPlan programmatically:

See [`LogicalPlanBuilder`]

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

---

## Partitioning

`enum` · `datafusion_expr::logical_plan::plan::Partitioning`

Also reachable as `datafusion::logical_expr::Partitioning`, `datafusion::prelude::Partitioning`, `datafusion_expr::Partitioning`, `datafusion_expr::logical_plan::Partitioning`

```rust
enum Partitioning
```

**Variants**: `RoundRobinBatch`, `Hash`, `Range`, `DistributeBy`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn partition_count(&self) -> Option<usize>
```

Logical partitioning schemes.

A scheme can describe either requested repartitioning in
[`LogicalPlan::Repartition`] or a partitioning property declared by a source.
Some schemes are only valid as metadata until planner support is added.

For physical execution partitioning, see
[`datafusion_physical_expr::Partitioning`].

[`datafusion_physical_expr::Partitioning`]: https://docs.rs/datafusion/latest/datafusion/physical_expr/enum.Partitioning.html#

---

## SkipType

`enum` · `datafusion_expr::logical_plan::plan::SkipType`

Also reachable as `datafusion::logical_expr::SkipType`, `datafusion_expr::SkipType`, `datafusion_expr::logical_plan::SkipType`

```rust
enum SkipType
```

**Variants**: `Literal`, `UnsupportedExpr`

Different types of skip expression in Limit plan.

---

## projection_schema

`function` · `datafusion_expr::logical_plan::plan::projection_schema`

Also reachable as `datafusion::logical_expr::projection_schema`, `datafusion_expr::logical_plan::projection_schema`, `datafusion_expr::projection_schema`

```rust
fn projection_schema(input: &LogicalPlan, exprs: &[Expr]) -> datafusion_common::Result<std::sync::Arc<datafusion_common::DFSchema>>
```

Computes the schema of the result produced by applying a projection to the input logical plan.

# Arguments

* `input`: A reference to the input `LogicalPlan` for which the projection schema
  will be computed.
* `exprs`: A slice of `Expr` expressions representing the projection operation to apply.

# Metadata Handling

- **Schema-level metadata**: Passed through unchanged from the input schema
- **Field-level metadata**: Determined by each expression via [`exprlist_to_fields`], which
  calls [`Expr::to_field`] to handle expression-specific metadata (literals, aliases, etc.)

# Returns

A `Result` containing an `Arc<DFSchema>` representing the schema of the result
produced by the projection operation. If the schema computation is successful,
the `Result` will contain the schema; otherwise, it will contain an error.

---

## Aggregate

`struct` · `datafusion_expr::logical_plan::plan::Aggregate`

Also reachable as `datafusion::logical_expr::Aggregate`, `datafusion_expr::Aggregate`, `datafusion_expr::logical_plan::Aggregate`

```rust
struct Aggregate
```

**Fields**: `input`, `group_expr`, `aggr_expr`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn group_expr_len(&self) -> Result<usize>
fn grouping_id_type(group_exprs: usize, max_ordinal: usize) -> DataType
fn try_new(input: Arc<LogicalPlan>, group_expr: Vec<Expr>, aggr_expr: Vec<Expr>) -> Result<Self>
fn try_new_with_schema(input: Arc<LogicalPlan>, group_expr: Vec<Expr>, aggr_expr: Vec<Expr>, schema: DFSchemaRef) -> Result<Self>
```

Aggregates its input based on a set of grouping and aggregate
expressions (e.g. SUM).

# Output Schema

The output schema is the group expressions followed by the aggregate
expressions in order.

For example, given the input schema `"A", "B", "C"` and the aggregate
`SUM(A) GROUP BY C+B`, the output schema will be `"C+B", "SUM(A)"` where
"C+B" and "SUM(A)" are the names of the output columns. Note that "C+B" is a
single new column

---

## Analyze

`struct` · `datafusion_expr::logical_plan::plan::Analyze`

Also reachable as `datafusion::logical_expr::Analyze`, `datafusion_expr::Analyze`, `datafusion_expr::logical_plan::Analyze`

```rust
struct Analyze
```

**Fields**: `verbose`, `format`, `input`, `schema`, `analyze_level`, `analyze_categories`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

Runs the actual plan, and then prints the physical plan with
with execution metrics.

---

## ColumnUnnestList

`struct` · `datafusion_expr::logical_plan::plan::ColumnUnnestList`

Also reachable as `datafusion::logical_expr::ColumnUnnestList`, `datafusion_expr::ColumnUnnestList`, `datafusion_expr::logical_plan::ColumnUnnestList`

```rust
struct ColumnUnnestList
```

**Fields**: `output_column`, `depth`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Represent the unnesting operation on a list column, such as the recursion depth and
the output column name after unnesting

Example: given `ColumnUnnestList { output_column: "output_name", depth: 2 }`

```text
  input             output_name
 ┌─────────┐      ┌─────────┐
 │{{1,2}}  │      │ 1       │
 ├─────────┼─────►├─────────┤
 │{{3}}    │      │ 2       │
 ├─────────┤      ├─────────┤
 │{{4},{5}}│      │ 3       │
 └─────────┘      ├─────────┤
                  │ 4       │
                  ├─────────┤
                  │ 5       │
                  └─────────┘
```

---

## DescribeTable

`struct` · `datafusion_expr::logical_plan::plan::DescribeTable`

Also reachable as `datafusion::logical_expr::DescribeTable`, `datafusion_expr::DescribeTable`, `datafusion_expr::logical_plan::DescribeTable`

```rust
struct DescribeTable
```

**Fields**: `schema`, `output_schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

Describe the schema of table

# Example output:

```sql
> describe traces;
+--------------------+-----------------------------+-------------+
| column_name        | data_type                   | is_nullable |
+--------------------+-----------------------------+-------------+
| attributes         | Utf8                        | YES         |
| duration_nano      | Int64                       | YES         |
| end_time_unix_nano | Int64                       | YES         |
| service.name       | Dictionary(Int32, Utf8)     | YES         |
| span.kind          | Utf8                        | YES         |
| span.name          | Utf8                        | YES         |
| span_id            | Dictionary(Int32, Utf8)     | YES         |
| time               | Timestamp(Nanosecond, None) | NO          |
| trace_id           | Dictionary(Int32, Utf8)     | YES         |
| otel.status_code   | Utf8                        | YES         |
| parent_span_id     | Utf8                        | YES         |
+--------------------+-----------------------------+-------------+
```

---

## DistinctOn

`struct` · `datafusion_expr::logical_plan::plan::DistinctOn`

Also reachable as `datafusion::logical_expr::DistinctOn`, `datafusion_expr::DistinctOn`, `datafusion_expr::logical_plan::DistinctOn`

```rust
struct DistinctOn
```

**Fields**: `on_expr`, `select_expr`, `sort_expr`, `input`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn try_new(on_expr: Vec<Expr>, select_expr: Vec<Expr>, sort_expr: Option<Vec<SortExpr>>, input: Arc<LogicalPlan>) -> Result<Self>
fn with_sort_expr(self, sort_expr: Vec<SortExpr>) -> Result<Self>
```

Removes duplicate rows from the input

---

## EmptyRelation

`struct` · `datafusion_expr::logical_plan::plan::EmptyRelation`

Also reachable as `datafusion::logical_expr::EmptyRelation`, `datafusion_expr::EmptyRelation`, `datafusion_expr::logical_plan::EmptyRelation`

```rust
struct EmptyRelation
```

**Fields**: `produce_one_row`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

Relationship produces 0 or 1 placeholder rows with specified output schema
In most cases the output schema for `EmptyRelation` would be empty,
however, it can be non-empty typically for optimizer rules

---

## Explain

`struct` · `datafusion_expr::logical_plan::plan::Explain`

Also reachable as `datafusion::logical_expr::Explain`, `datafusion_expr::Explain`, `datafusion_expr::logical_plan::Explain`

```rust
struct Explain
```

**Fields**: `verbose`, `explain_format`, `plan`, `stringified_plans`, `schema`, `logical_optimization_succeeded`, `show_statistics`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

Produces a relation with string representations of
various parts of the plan

See [the documentation] for more information

[the documentation]: https://datafusion.apache.org/user-guide/sql/explain.html

---

## ExplainOption

`struct` · `datafusion_expr::logical_plan::plan::ExplainOption`

Also reachable as `datafusion::logical_expr::ExplainOption`, `datafusion_expr::ExplainOption`, `datafusion_expr::logical_plan::ExplainOption`

```rust
struct ExplainOption
```

**Fields**: `verbose`, `analyze`, `format`, `show_statistics`, `analyze_level`, `analyze_categories`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn with_analyze(self, analyze: bool) -> Self
fn with_analyze_categories(self, analyze_categories: Option<ExplainAnalyzeCategories>) -> Self
fn with_analyze_level(self, analyze_level: Option<MetricType>) -> Self
fn with_format(self, format: ExplainFormat) -> Self
fn with_show_statistics(self, show_statistics: Option<bool>) -> Self
fn with_verbose(self, verbose: bool) -> Self
```

Options for EXPLAIN

---

## Extension

`struct` · `datafusion_expr::logical_plan::plan::Extension`

Also reachable as `datafusion::logical_expr::Extension`, `datafusion_expr::Extension`, `datafusion_expr::logical_plan::Extension`

```rust
struct Extension
```

**Fields**: `node`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd

Extension operator defined outside of DataFusion

---

## Filter

`struct` · `datafusion_expr::logical_plan::plan::Filter`

Also reachable as `datafusion::logical_expr::Filter`, `datafusion_expr::Filter`, `datafusion_expr::logical_plan::Filter`

```rust
struct Filter
```

**Fields**: `predicate`, `input`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn try_new(predicate: Expr, input: Arc<LogicalPlan>) -> Result<Self>
```

Filters rows from its input that do not match an
expression (essentially a WHERE clause with a predicate
expression).

Semantically, `<predicate>` is evaluated for each row of the input;
If the value of `<predicate>` is true, the input row is passed to
the output. If the value of `<predicate>` is false, the row is
discarded.

Filter should not be created directly but instead use `try_new()`
and that these fields are only pub to support pattern matching

---

## Join

`struct` · `datafusion_expr::logical_plan::plan::Join`

Also reachable as `datafusion::logical_expr::Join`, `datafusion_expr::Join`, `datafusion_expr::logical_plan::Join`

```rust
struct Join
```

**Fields**: `left`, `right`, `on`, `filter`, `join_type`, `join_constraint`, `schema`, `null_equality`, `null_aware`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn try_new(left: Arc<LogicalPlan>, right: Arc<LogicalPlan>, on: Vec<(Expr, Expr)>, filter: Option<Expr>, join_type: JoinType, join_constraint: JoinConstraint, null_equality: NullEquality, null_aware: bool) -> Result<Self>
fn try_new_with_project_input(original: &LogicalPlan, left: Arc<LogicalPlan>, right: Arc<LogicalPlan>, column_on: (Vec<Column>, Vec<Column>)) -> Result<(Self, bool)>
```

Join two logical plans on one or more join columns

---

## Limit

`struct` · `datafusion_expr::logical_plan::plan::Limit`

Also reachable as `datafusion::logical_expr::Limit`, `datafusion_expr::Limit`, `datafusion_expr::logical_plan::Limit`

```rust
struct Limit
```

**Fields**: `skip`, `fetch`, `input`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn get_fetch_type(&self) -> Result<FetchType>
fn get_skip_type(&self) -> Result<SkipType>
```

Produces the first `n` tuples from its input and discards the rest.

---

## Projection

`struct` · `datafusion_expr::logical_plan::plan::Projection`

Also reachable as `datafusion::logical_expr::Projection`, `datafusion_expr::Projection`, `datafusion_expr::logical_plan::Projection`

```rust
struct Projection
```

**Fields**: `expr`, `input`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn new_from_schema(input: Arc<LogicalPlan>, schema: DFSchemaRef) -> Self
fn try_new(expr: Vec<Expr>, input: Arc<LogicalPlan>) -> Result<Self>
fn try_new_with_schema(expr: Vec<Expr>, input: Arc<LogicalPlan>, schema: DFSchemaRef) -> Result<Self>
```

Evaluates an arbitrary list of expressions (essentially a
SELECT with an expression list) on its input.

---

## RangePartitioning

`struct` · `datafusion_expr::logical_plan::plan::RangePartitioning`

Also reachable as `datafusion::logical_expr::RangePartitioning`, `datafusion_expr::RangePartitioning`, `datafusion_expr::logical_plan::RangePartitioning`

```rust
struct RangePartitioning
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn ordering(&self) -> &[SortExpr]
fn partition_count(&self) -> usize
fn split_points(&self) -> &[SplitPoint]
fn try_new(ordering: Vec<SortExpr>, split_points: Vec<SplitPoint>) -> Result<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Logical range partitioning.

[`RangePartitioning`] describes an ordered logical key space with split points.

- `ordering` defines the partitioning key and ordering using logical
  [`SortExpr`]s.
- `split_points` define the boundaries between adjacent partitions.

Comparisons use the lexicographic order defined by `ordering`,
including `ASC`/`DESC` and null ordering. Split points must be ordered
according to that ordering, and each split point must have one value per
ordering expression. See [`SplitPoint`] for the shared boundary contract.

The expressions are resolved against the declaring plan's schema. This
constructor does not validate split point value types against the resolved
expression types. Like other user-specified data properties such as
sortedness, if a source declares range partitioning, it is responsible for
placing each row in the partition described by the split points. DataFusion
will not validate this is upheld.

NOTE: Range-aware optimizer and execution behavior will be introduced
incrementally. See
<https://github.com/apache/datafusion/issues/22395>.

---

## RecursiveQuery

`struct` · `datafusion_expr::logical_plan::plan::RecursiveQuery`

Also reachable as `datafusion::logical_expr::RecursiveQuery`, `datafusion_expr::RecursiveQuery`, `datafusion_expr::logical_plan::RecursiveQuery`

```rust
struct RecursiveQuery
```

**Fields**: `name`, `static_term`, `recursive_term`, `is_distinct`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn try_new(name: String, static_term: Arc<LogicalPlan>, recursive_term: Arc<LogicalPlan>, is_distinct: bool) -> Result<Self>
```

A variadic query operation, Recursive CTE.

# Recursive Query Evaluation

From the [Postgres Docs]:

1. Evaluate the non-recursive term. For `UNION` (but not `UNION ALL`),
   discard duplicate rows. Include all remaining rows in the result of the
   recursive query, and also place them in a temporary working table.

2. So long as the working table is not empty, repeat these steps:

* Evaluate the recursive term, substituting the current contents of the
  working table for the recursive self-reference. For `UNION` (but not `UNION
  ALL`), discard duplicate rows and rows that duplicate any previous result
  row. Include all remaining rows in the result of the recursive query, and
  also place them in a temporary intermediate table.

* Replace the contents of the working table with the contents of the
  intermediate table, then empty the intermediate table.

[Postgres Docs]: https://www.postgresql.org/docs/current/queries-with.html#QUERIES-WITH-RECURSIVE

---

## Repartition

`struct` · `datafusion_expr::logical_plan::plan::Repartition`

Also reachable as `datafusion::logical_expr::Repartition`, `datafusion_expr::Repartition`, `datafusion_expr::logical_plan::Repartition`

```rust
struct Repartition
```

**Fields**: `input`, `partitioning_scheme`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

---

## Sort

`struct` · `datafusion_expr::logical_plan::plan::Sort`

Also reachable as `datafusion::logical_expr::Sort`, `datafusion_expr::Sort`, `datafusion_expr::logical_plan::Sort`

```rust
struct Sort
```

**Fields**: `expr`, `input`, `fetch`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

Sorts its input according to a list of sort expressions.

---

## Subquery

`struct` · `datafusion_expr::logical_plan::plan::Subquery`

Also reachable as `datafusion::logical_expr::Subquery`, `datafusion_expr::Subquery`, `datafusion_expr::logical_plan::Subquery`

```rust
struct Subquery
```

**Fields**: `subquery`, `outer_ref_columns`, `spans`

**Implements**: `datafusion_common::cse::NormalizeEq`, `datafusion_common::cse::Normalizeable`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn try_from_expr(plan: &Expr) -> Result<&Subquery>
fn with_plan(&self, plan: Arc<LogicalPlan>) -> Subquery
```

**via `datafusion_common::cse::NormalizeEq`**

```rust
fn normalize_eq(&self, other: &Self) -> bool
```

**via `datafusion_common::cse::Normalizeable`**

```rust
fn can_normalize(&self) -> bool
```

Subquery

---

## SubqueryAlias

`struct` · `datafusion_expr::logical_plan::plan::SubqueryAlias`

Also reachable as `datafusion::logical_expr::SubqueryAlias`, `datafusion_expr::SubqueryAlias`, `datafusion_expr::logical_plan::SubqueryAlias`

```rust
struct SubqueryAlias
```

**Fields**: `input`, `alias`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn try_new(plan: Arc<LogicalPlan>, alias: impl Into<TableReference>) -> Result<Self>
```

Aliased subquery

---

## TableScan

`struct` · `datafusion_expr::logical_plan::plan::TableScan`

Also reachable as `datafusion::logical_expr::TableScan`, `datafusion_expr::TableScan`, `datafusion_expr::logical_plan::TableScan`

```rust
struct TableScan
```

**Fields**: `table_name`, `source`, `projection`, `projected_schema`, `filters`, `fetch`, `statistics_requests`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd

**Methods** (1)

```rust
fn try_new(table_name: impl Into<TableReference>, table_source: Arc<dyn TableSource>, projection: Option<Vec<usize>>, filters: Vec<Expr>, fetch: Option<usize>) -> Result<Self>
```

Produces rows from a table provider by reference or from the context

---

## TableScanBuilder

`struct` · `datafusion_expr::logical_plan::plan::TableScanBuilder`

Also reachable as `datafusion::logical_expr::TableScanBuilder`, `datafusion_expr::TableScanBuilder`, `datafusion_expr::logical_plan::TableScanBuilder`

```rust
struct TableScanBuilder
```

**Implements**: `core::convert::From`

**Methods** (6)

```rust
fn build(self) -> Result<TableScan>
fn new(table_name: impl Into<TableReference>, source: Arc<dyn TableSource>) -> Self
fn with_fetch(self, fetch: Option<usize>) -> Self
fn with_filters(self, filters: Vec<Expr>) -> Self
fn with_projection(self, projection: Option<Vec<usize>>) -> Self
fn with_statistics_requests(self, statistics_requests: BTreeSet<StatisticsRequest>) -> Self
```

**via `core::convert::From`**

```rust
fn from(scan: TableScan) -> Self
```

Builder for [`TableScan`].

Prefer this over constructing a [`TableScan`] directly: it derives the
`projected_schema` from the source schema and projection, and is resilient
to new fields being added to [`TableScan`]. An existing scan can be turned
back into a builder with `TableScanBuilder::from(scan)`, tweaked, and
rebuilt with [`TableScanBuilder::build`].

---

## Union

`struct` · `datafusion_expr::logical_plan::plan::Union`

Also reachable as `datafusion::logical_expr::Union`, `datafusion_expr::Union`, `datafusion_expr::logical_plan::Union`

```rust
struct Union
```

**Fields**: `inputs`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn try_new(inputs: Vec<Arc<LogicalPlan>>) -> Result<Self>
fn try_new_by_name(inputs: Vec<Arc<LogicalPlan>>) -> Result<Self>
fn try_new_with_loose_types(inputs: Vec<Arc<LogicalPlan>>) -> Result<Self>
```

Union multiple inputs

---

## Unnest

`struct` · `datafusion_expr::logical_plan::plan::Unnest`

Also reachable as `datafusion::logical_expr::Unnest`, `datafusion_expr::Unnest`, `datafusion_expr::logical_plan::Unnest`

```rust
struct Unnest
```

**Fields**: `input`, `exec_columns`, `list_type_columns`, `struct_type_columns`, `dependency_indices`, `schema`, `options`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn try_new(input: Arc<LogicalPlan>, exec_columns: Vec<Column>, options: UnnestOptions) -> Result<Self>
```

Unnest a column that contains a nested list type. See
[`UnnestOptions`] for more details.

---

## Values

`struct` · `datafusion_expr::logical_plan::plan::Values`

Also reachable as `datafusion::logical_expr::Values`, `datafusion_expr::Values`, `datafusion_expr::logical_plan::Values`

```rust
struct Values
```

**Fields**: `schema`, `values`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

Values expression. See
[Postgres VALUES](https://www.postgresql.org/docs/current/queries-values.html)
documentation for more details.

---

## Window

`struct` · `datafusion_expr::logical_plan::plan::Window`

Also reachable as `datafusion::logical_expr::Window`, `datafusion_expr::Window`, `datafusion_expr::logical_plan::Window`

```rust
struct Window
```

**Fields**: `input`, `window_expr`, `schema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn try_new(window_expr: Vec<Expr>, input: Arc<LogicalPlan>) -> Result<Self>
fn try_new_with_schema(window_expr: Vec<Expr>, input: Arc<LogicalPlan>, schema: DFSchemaRef) -> Result<Self>
```

Window its input based on a set of window spec and window function (e.g. SUM or RANK)

# Output Schema

The output schema is the input schema followed by the window function
expressions, in order.

For example, given the input schema `"A", "B", "C"` and the window function
`SUM(A) OVER (PARTITION BY B+1 ORDER BY C)`, the output schema will be `"A",
"B", "C", "SUM(A) OVER ..."` where `"SUM(A) OVER ..."` is the name of the
output column.

Note that the `PARTITION BY` expression "B+1" is not produced in the output
schema.

---
