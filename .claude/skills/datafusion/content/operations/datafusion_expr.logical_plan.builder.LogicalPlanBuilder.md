# `datafusion_expr::logical_plan::builder::LogicalPlanBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.LogicalPlanBuilder.json).

<a id="op-e319e4d14d504d0b1af00d77"></a>
## LogicalPlanBuilder

`struct` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder` · datafusion-expr 55.1.0

```rust
struct LogicalPlanBuilder
```

Source: `src/logical_plan/builder.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Builder for logical plans

# Example building a simple plan
```
# use datafusion_expr::{lit, col, LogicalPlanBuilder, logical_plan::table_scan};
# use datafusion_common::Result;
# use arrow::datatypes::{Schema, DataType, Field};
#
# fn main() -> Result<()> {
#
# fn employee_schema() -> Schema {
#    Schema::new(vec![
#           Field::new("id", DataType::Int32, false),
#           Field::new("first_name", DataType::Utf8, false),
#           Field::new("last_name", DataType::Utf8, false),
#           Field::new("state", DataType::Utf8, false),
#           Field::new("salary", DataType::Int32, false),
#       ])
#   }
#
// Create a plan similar to
// SELECT last_name
// FROM employees
// WHERE salary < 1000
let plan = table_scan(Some("employee"), &employee_schema(), None)?
 // Keep only rows where salary < 1000
 .filter(col("salary").lt(lit(1000)))?
 // only show "last_name" in the final results
 .project(vec![col("last_name")])?
 .build()?;

// Convert from plan back to builder
let builder = LogicalPlanBuilder::from(plan);

# Ok(())
# }
```

<a id="op-d4eb28341772ae8609f9d3d8"></a>
## aggregate

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::aggregate` · datafusion-expr 55.1.0

```rust
fn aggregate(self, group_expr: impl IntoIterator<Item = impl Into<Expr>>, aggr_expr: impl IntoIterator<Item = impl Into<Expr>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1295`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply an aggregate: grouping on the `group_expr` expressions
and calculating `aggr_expr` aggregates for each distinct
value of the `group_expr`;

<a id="op-1782be14392c4ff75e56bb42"></a>
## alias

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::alias` · datafusion-expr 55.1.0

```rust
fn alias(self, alias: impl Into<TableReference>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:687`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply an alias

<a id="op-826e7583b44966eb75f9c437"></a>
## build

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::build` · datafusion-expr 55.1.0

```rust
fn build(self) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1451`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Build the plan

<a id="op-86e920ac25ac74612d279d5e"></a>
## clone

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> LogicalPlanBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 17], "end": [126, 22], "filename": "src/logical_plan/builder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/builder.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02d2852669c2ec956f2c73c1"></a>
## copy_to

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::copy_to` · datafusion-expr 55.1.0

```rust
fn copy_to(input: LogicalPlan, output_url: String, file_type: Arc<dyn FileType>, options: HashMap<String, String>, partition_by: Vec<String>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a [CopyTo](../operations/datafusion_expr.logical_plan.dml.CopyTo.md#op-07f8bb3eb141720e9af3dcd6) for copying the contents of this builder to the specified file(s)

<a id="op-936ec87a71eb27f1708b46f6"></a>
## cross_join

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::cross_join` · datafusion-expr 55.1.0

```rust
fn cross_join(self, right: LogicalPlan) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1256`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a cross join

<a id="op-738cfc090b5f68a88571b28f"></a>
## distinct

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::distinct` · datafusion-expr 55.1.0

```rust
fn distinct(self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:907`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply deduplication: Only distinct (different) values are returned)

<a id="op-e98d30d26d7da2328439fb51"></a>
## distinct_on

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::distinct_on` · datafusion-expr 55.1.0

```rust
fn distinct_on(self, on_expr: Vec<Expr>, select_expr: Vec<Expr>, sort_expr: Option<Vec<SortExpr>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:913`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Project first values of the specified expression list according to the provided
sorting expressions grouped by the `DISTINCT ON` clause expressions.

<a id="op-5da7389391777bd81ad3940c"></a>
## empty

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::empty` · datafusion-expr 55.1.0

```rust
fn empty(produce_one_row: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create an empty relation.

`produce_one_row` set to true means this empty node needs to produce a placeholder row.

<a id="op-2ac19fc8abdfd1a372b8333e"></a>
## except

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::except` · datafusion-expr 55.1.0

```rust
fn except(left_plan: LogicalPlan, right_plan: LogicalPlan, is_all: bool) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1376`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Process except set operator

<a id="op-6bf1f70ba692ad6fdaf51c38"></a>
## explain

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::explain` · datafusion-expr 55.1.0

```rust
fn explain(self, verbose: bool, analyze: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1320`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create an expression to represent the explanation of the plan

if `analyze` is true, runs the actual plan and produces
information about metrics during run.

if `verbose` is true, prints out additional details.

<a id="op-a140c7fa1cc3c25abe247ff5"></a>
## explain_option_format

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::explain_option_format` · datafusion-expr 55.1.0

```rust
fn explain_option_format(self, explain_option: ExplainOption) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1332`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create an expression to represent the explanation of the plan
The`explain_option` is used to specify the format and verbosity of the explanation.
Details see [`ExplainOption`](../operations/datafusion_expr.logical_plan.plan.ExplainOption.md#op-784cf352b14885c813b5fb5d).

<a id="op-39965680ea0a6f738fae30c0"></a>
## filter

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::filter` · datafusion-expr 55.1.0

```rust
fn filter(self, expr: impl Into<Expr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a filter

<a id="op-a2ea6fe815962e48f179bb90"></a>
## fmt

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 10], "end": [126, 15], "filename": "src/logical_plan/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/builder.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7afa772d271b4503bcfa646d"></a>
## from

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::from` · datafusion-expr 55.1.0

```rust
fn from(plan: Arc<LogicalPlan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1566, 1], "end": [1570, 2], "filename": "src/logical_plan/builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/logical_plan/builder.rs:1567`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6e2e771fc039469b49a76c4"></a>
## from

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::from` · datafusion-expr 55.1.0

```rust
fn from(plan: LogicalPlan) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1560, 1], "end": [1564, 2], "filename": "src/logical_plan/builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::LogicalPlan", "path": "LogicalPlan"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/logical_plan/builder.rs:1561`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7e6b74fe35685fa1d6dbe43"></a>
## having

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::having` · datafusion-expr 55.1.0

```rust
fn having(self, expr: impl Into<Expr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:641`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a filter which is used for a having clause

<a id="op-ae53ca8b2a99dfdf8de8d6f2"></a>
## insert_into

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::insert_into` · datafusion-expr 55.1.0

```rust
fn insert_into(input: LogicalPlan, table_name: impl Into<TableReference>, target: Arc<dyn TableSource>, insert_op: InsertOp) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:471`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a [`DmlStatement`](../operations/datafusion_expr.logical_plan.dml.DmlStatement.md#op-2ac6db89e384424dd62ef114) for inserting the contents of this builder into the named table.

Note,  use a [`DefaultTableSource`] to insert into a [`TableProvider`]

[`DefaultTableSource`]: https://docs.rs/datafusion/latest/datafusion/datasource/default_table_source/struct.DefaultTableSource.html
[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html

# Example:
```
# use datafusion_expr::{lit, LogicalPlanBuilder,
#  logical_plan::builder::LogicalTableSource,
# };
# use std::sync::Arc;
# use arrow::datatypes::{Schema, DataType, Field};
# use datafusion_expr::dml::InsertOp;
#
# fn test() -> datafusion_common::Result<()> {
# let employee_schema = Arc::new(Schema::new(vec![
#     Field::new("id", DataType::Int32, false),
# ])) as _;
# let table_source = Arc::new(LogicalTableSource::new(employee_schema));
// VALUES (1), (2)
let input = LogicalPlanBuilder::values(vec![vec![lit(1)], vec![lit(2)]])?.build()?;
// INSERT INTO MyTable VALUES (1), (2)
let insert_plan = LogicalPlanBuilder::insert_into(
    input,
    "MyTable",
    table_source,
    InsertOp::Append,
)?;
# Ok(())
# }
```

<a id="op-eac092804b9ba67e296e546e"></a>
## intersect

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::intersect` · datafusion-expr 55.1.0

```rust
fn intersect(left_plan: LogicalPlan, right_plan: LogicalPlan, is_all: bool) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1362`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Process intersect set operator

<a id="op-2dae0656827321dd29d86407"></a>
## join

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::join` · datafusion-expr 55.1.0

```rust
fn join(self, right: LogicalPlan, join_type: JoinType, join_keys: (Vec<impl Into<Column>>, Vec<impl Into<Column>>), filter: Option<Expr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:937`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a join to `right` using explicitly specified columns and an
optional filter expression.

See [`join_on`](Self::join_on) for a more concise way to specify the
join condition. Since DataFusion will automatically identify and
optimize equality predicates there is no performance difference between
this function and `join_on`

`left_cols` and `right_cols` are used to form "equijoin" predicates (see
example below), which are then combined with the optional `filter`
expression.

Note that in case of outer join, the `filter` is applied to only matched rows.

<a id="op-64a9db7694c7b5c67f94e9bd"></a>
## join_detailed

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::join_detailed` · datafusion-expr 55.1.0

```rust
fn join_detailed(self, right: LogicalPlan, join_type: JoinType, join_keys: (Vec<impl Into<Column>>, Vec<impl Into<Column>>), filter: Option<Expr>, null_equality: NullEquality) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1031`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a join with on constraint and specified null equality.

The behavior is the same as [`join`](Self::join) except that it allows
specifying the null equality behavior.

The `null_equality` dictates how `null` values are joined.

<a id="op-a726c31deae929cf3f4af716"></a>
## join_detailed_with_options

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::join_detailed_with_options` · datafusion-expr 55.1.0

```rust
fn join_detailed_with_options(self, right: LogicalPlan, join_type: JoinType, join_keys: (Vec<impl Into<Column>>, Vec<impl Into<Column>>), filter: Option<Expr>, null_equality: NullEquality, null_aware: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1049`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e5da6d8cf07f9e5c807f7da"></a>
## join_on

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::join_on` · datafusion-expr 55.1.0

```rust
fn join_on(self, right: LogicalPlan, join_type: JoinType, on_exprs: impl IntoIterator<Item = Expr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:993`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a join using the specified expressions.

Note that DataFusion automatically optimizes joins, including
identifying and optimizing equality predicates.

# Example

```
# use datafusion_expr::{Expr, col, LogicalPlanBuilder,
#  logical_plan::builder::LogicalTableSource, logical_plan::JoinType,};
# use std::sync::Arc;
# use arrow::datatypes::{Schema, DataType, Field};
# use datafusion_common::Result;
# fn main() -> Result<()> {
let example_schema = Arc::new(Schema::new(vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Int32, false),
    Field::new("c", DataType::Int32, false),
]));
let table_source = Arc::new(LogicalTableSource::new(example_schema));
let left_table = table_source.clone();
let right_table = table_source.clone();

let right_plan = LogicalPlanBuilder::scan("right", right_table, None)?.build()?;

// Form the expression `(left.a != right.a)` AND `(left.b != right.b)`
let exprs = vec![
    col("left.a").eq(col("right.a")),
    col("left.b").not_eq(col("right.b")),
];

// Perform the equivalent of `left INNER JOIN right ON (a != a2 AND b != b2)`
// finding all pairs of rows from `left` and `right` where
// where `a = a2` and `b != b2`.
let plan = LogicalPlanBuilder::scan("left", left_table, None)?
    .join_on(right_plan, JoinType::Inner, exprs)?
    .build()?;
# Ok(())
# }
```

<a id="op-57d9f0c71c62289a822cf55d"></a>
## join_using

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::join_using` · datafusion-expr 55.1.0

```rust
fn join_using(self, right: LogicalPlan, join_type: JoinType, using_keys: Vec<Column>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1179`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a join with using constraint, which duplicates all join columns in output schema.

<a id="op-287ee35fa133b3dd170c48a4"></a>
## join_with_expr_keys

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::join_with_expr_keys` · datafusion-expr 55.1.0

```rust
fn join_with_expr_keys(self, right: LogicalPlan, join_type: JoinType, equi_exprs: (Vec<impl Into<Expr>>, Vec<impl Into<Expr>>), filter: Option<Expr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1469`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a join with both explicit equijoin and non equijoin predicates.

Note this is a low level API that requires identifying specific
predicate types. Most users should use  [`join_on`](Self::join_on) that
automatically identifies predicates appropriately.

`equi_exprs` defines equijoin predicates, of the form `l = r)` for each
`(l, r)` tuple. `l`, the first element of the tuple, must only refer
to columns from the existing input. `r`, the second element of the tuple,
must only refer to columns from the right input.

`filter` contains any other filter expression to apply during the
join. Note that `equi_exprs` predicates are evaluated more efficiently
than the filter expressions, so they are preferred.

<a id="op-d7183db83d67899a20e706bb"></a>
## limit

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::limit` · datafusion-expr 55.1.0

```rust
fn limit(self, skip: usize, fetch: Option<usize>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:665`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Limit the number of rows returned

`skip` - Number of rows to skip before fetch any row.

`fetch` - Maximum number of rows to fetch, after skipping `skip` rows,
         if specified.

<a id="op-6f61ff7e4f77e8650d48783f"></a>
## limit_by_expr

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::limit_by_expr` · datafusion-expr 55.1.0

```rust
fn limit_by_expr(self, skip: Option<Expr>, fetch: Option<Expr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:678`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Limit the number of rows returned

Similar to `limit` but uses expressions for `skip` and `fetch`

<a id="op-34e5f4b8324a4328bcef30e6"></a>
## new

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::new` · datafusion-expr 55.1.0

```rust
fn new(plan: LogicalPlan) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a builder from an existing plan

<a id="op-70b12389389a0b22da35bd75"></a>
## new_from_arc

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::new_from_arc` · datafusion-expr 55.1.0

```rust
fn new_from_arc(plan: Arc<LogicalPlan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a builder from an existing plan

<a id="op-11b1ac886616197f53c84d1c"></a>
## plan

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::plan` · datafusion-expr 55.1.0

```rust
fn plan(&self) -> &LogicalPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the LogicalPlan of the plan build so far

<a id="op-02164cdd28e21eafb32a3640"></a>
## prepare

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::prepare` · datafusion-expr 55.1.0

```rust
fn prepare(self, name: String, fields: Vec<FieldRef>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:649`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Make a builder for a prepare logical plan from the builder's plan

<a id="op-6b143b34bbf304177a89b391"></a>
## project

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::project` · datafusion-expr 55.1.0

```rust
fn project(self, expr: impl IntoIterator<Item = impl Into<SelectExpr>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:591`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a projection without alias.

<a id="op-cc8c278d2ce129888b75ca9e"></a>
## project_with_validation

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::project_with_validation` · datafusion-expr 55.1.0

```rust
fn project_with_validation(self, expr: Vec<(impl Into<SelectExpr>, bool)>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:600`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a projection without alias with optional validation
(true to validate, false to not validate)

<a id="op-1cec923a93f42b0cae98a6e1"></a>
## project_with_validation_and_schema

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::project_with_validation_and_schema` · datafusion-expr 55.1.0

```rust
fn project_with_validation_and_schema(self, expr: impl IntoIterator<Item = impl Into<SelectExpr>>, schema: &DFSchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:610`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a projection, aliasing non-Column/non-Alias expressions to
match the field names from the provided schema.

<a id="op-c4b5ef773269d3b301fb404d"></a>
## repartition

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::repartition` · datafusion-expr 55.1.0

```rust
fn repartition(self, partitioning_scheme: Partitioning) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1272`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Repartition

<a id="op-5c6f94e9da069773c002b417"></a>
## scan

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::scan` · datafusion-expr 55.1.0

```rust
fn scan(table_name: impl Into<TableReference>, table_source: Arc<dyn TableSource>, projection: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:413`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Convert a table provider into a builder with a TableScan

Note that if you pass a string as `table_name`, it is treated
as a SQL identifier, as described on [`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0) and
thus is normalized

# Example:
```
# use datafusion_expr::{lit, col, LogicalPlanBuilder,
#  logical_plan::builder::LogicalTableSource, logical_plan::table_scan
# };
# use std::sync::Arc;
# use arrow::datatypes::{Schema, DataType, Field};
# use datafusion_common::TableReference;
#
# let employee_schema = Arc::new(Schema::new(vec![
#           Field::new("id", DataType::Int32, false),
# ])) as _;
# let table_source = Arc::new(LogicalTableSource::new(employee_schema));
// Scan table_source with the name "mytable" (after normalization)
# let table = table_source.clone();
let scan = LogicalPlanBuilder::scan("MyTable", table, None);

// Scan table_source with the name "MyTable" by enclosing in quotes
# let table = table_source.clone();
let scan = LogicalPlanBuilder::scan(r#""MyTable""#, table, None);

// Scan table_source with the name "MyTable" by forming the table reference
# let table = table_source.clone();
let table_reference = TableReference::bare("MyTable");
let scan = LogicalPlanBuilder::scan(table_reference, table, None);
```

<a id="op-f315008fa8417977365d9f6a"></a>
## scan_with_filters

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::scan_with_filters` · datafusion-expr 55.1.0

```rust
fn scan_with_filters(table_name: impl Into<TableReference>, table_source: Arc<dyn TableSource>, projection: Option<Vec<usize>>, filters: Vec<Expr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Convert a table provider into a builder with a TableScan

<a id="op-b543f353d98238391a50a497"></a>
## scan_with_filters_fetch

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::scan_with_filters_fetch` · datafusion-expr 55.1.0

```rust
fn scan_with_filters_fetch(table_name: impl Into<TableReference>, table_source: Arc<dyn TableSource>, projection: Option<Vec<usize>>, filters: Vec<Expr>, fetch: Option<usize>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:496`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Convert a table provider into a builder with a TableScan with filter and fetch

<a id="op-f157aa4bffcbdcfb546d4945"></a>
## schema

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::schema` · datafusion-expr 55.1.0

```rust
fn schema(&self) -> &DFSchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the output schema of the plan build so far

<a id="op-5b645ddfdae022e61d2f093e"></a>
## select

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::select` · datafusion-expr 55.1.0

```rust
fn select(self, indices: impl IntoIterator<Item = usize>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:624`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Select the given column indices

<a id="op-f11dd53081ec63ae2d3c6434"></a>
## sort

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::sort` · datafusion-expr 55.1.0

```rust
fn sort(self, sorts: impl IntoIterator<Item = impl Into<SortExpr>> + Clone) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:815`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3045805a01a7fa2db3e678fa"></a>
## sort_by

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::sort_by` · datafusion-expr 55.1.0

```rust
fn sort_by(self, expr: impl IntoIterator<Item = impl Into<Expr>> + Clone) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:804`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a sort by provided expressions with default direction

<a id="op-8be36a8b0e1388dfb1dd6031"></a>
## sort_with_limit

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::sort_with_limit` · datafusion-expr 55.1.0

```rust
fn sort_with_limit(self, sorts: impl IntoIterator<Item = impl Into<SortExpr>> + Clone, fetch: Option<usize>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:823`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a sort

<a id="op-62253e435d06d3858be64d9c"></a>
## to_recursive_query

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::to_recursive_query` · datafusion-expr 55.1.0

```rust
fn to_recursive_query(self, name: String, recursive_term: LogicalPlan, is_distinct: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Convert a regular plan into a recursive query.
`is_distinct` indicates whether the recursive term should be de-duplicated (`UNION`) after each iteration or not (`UNION ALL`).

<a id="op-53ef73ae510931040b66cb40"></a>
## union

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::union` · datafusion-expr 55.1.0

```rust
fn union(self, plan: LogicalPlan) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:877`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a union, preserving duplicate rows

<a id="op-1f55d5c7e3cf15fc88fbbbfd"></a>
## union_by_name

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::union_by_name` · datafusion-expr 55.1.0

```rust
fn union_by_name(self, plan: LogicalPlan) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:882`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a union by name, preserving duplicate rows

<a id="op-d2b372b6baca2291433ba5db"></a>
## union_by_name_distinct

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::union_by_name_distinct` · datafusion-expr 55.1.0

```rust
fn union_by_name_distinct(self, plan: LogicalPlan) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:887`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a union by name, removing duplicate rows

<a id="op-f9b10abbaf87abd023dee82c"></a>
## union_distinct

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::union_distinct` · datafusion-expr 55.1.0

```rust
fn union_distinct(self, plan: LogicalPlan) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:897`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a union, removing duplicate rows

<a id="op-117385ea226196d448385eae"></a>
## unnest_column

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::unnest_column` · datafusion-expr 55.1.0

```rust
fn unnest_column(self, column: impl Into<Column>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1531`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Unnest the given column.

<a id="op-a21246d1577471e6e6c301a0"></a>
## unnest_column_with_options

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::unnest_column_with_options` · datafusion-expr 55.1.0

```rust
fn unnest_column_with_options(self, column: impl Into<Column>, options: UnnestOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1536`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Unnest the given column given [`UnnestOptions`](../operations/datafusion_common.unnest.UnnestOptions.md#op-fc8c0e778d2849560cc4457a)

<a id="op-b7582a9a3243de05e4a8c428"></a>
## unnest_columns_with_options

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::unnest_columns_with_options` · datafusion-expr 55.1.0

```rust
fn unnest_columns_with_options(self, columns: Vec<Column>, options: UnnestOptions) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1550`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Unnest the given columns with the given [`UnnestOptions`](../operations/datafusion_common.unnest.UnnestOptions.md#op-fc8c0e778d2849560cc4457a)

<a id="op-03ac5e16f1c92595258be3e1"></a>
## values

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::values` · datafusion-expr 55.1.0

```rust
fn values(values: Vec<Vec<Expr>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a values list based relation, and the schema is inferred from data, consuming
`value`. See the [Postgres VALUES](https://www.postgresql.org/docs/current/queries-values.html)
documentation for more details.

so it's usually better to override the default names with a table alias list.

If the values include params/binders such as $1, $2, $3, etc, then the `param_data_types` should be provided.

<a id="op-22deba24048e7f9763d96025"></a>
## values_with_schema

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::values_with_schema` · datafusion-expr 55.1.0

```rust
fn values_with_schema(values: Vec<Vec<Expr>>, schema: &DFSchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a values list based relation, and the schema is inferred from data itself or table schema if provided, consuming
`value`. See the [Postgres VALUES](https://www.postgresql.org/docs/current/queries-values.html)
documentation for more details.

By default, it assigns the names column1, column2, etc. to the columns of a VALUES table.
The column names are not specified by the SQL standard and different database systems do it differently,
so it's usually better to override the default names with a table alias list.

If the values include params/binders such as $1, $2, $3, etc, then the `param_data_types` should be provided.

<a id="op-d2a64e02597cde666b9eed0f"></a>
## window

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::window` · datafusion-expr 55.1.0

```rust
fn window(self, window_expr: impl IntoIterator<Item = impl Into<Expr>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:1280`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Apply a window functions to extend the schema

<a id="op-2f6782e58fb553b44a43470a"></a>
## window_plan

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::window_plan` · datafusion-expr 55.1.0

```rust
fn window_plan(input: LogicalPlan, window_exprs: impl IntoIterator<Item = Expr>) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:553`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Wrap a plan in a window

<a id="op-3e9f7e2111aa5f11dd698a6d"></a>
## with_options

`function` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder::with_options` · datafusion-expr 55.1.0

```rust
fn with_options(self, options: LogicalPlanBuilderOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::builder::LogicalPlanBuilder", "path": "LogicalPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [1558, 2], "filename": "src/logical_plan/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/builder.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
