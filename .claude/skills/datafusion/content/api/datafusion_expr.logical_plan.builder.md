# `datafusion_expr::logical_plan::builder`

Crate `datafusion-expr` · 22 public items · structured records in [`model/datafusion_expr.logical_plan.builder.json`](../model/datafusion_expr.logical_plan.builder.json)

## UNNAMED_TABLE

`constant` · `datafusion_expr::logical_plan::builder::UNNAMED_TABLE`

Also reachable as `datafusion::logical_expr::UNNAMED_TABLE`, `datafusion_expr::UNNAMED_TABLE`, `datafusion_expr::logical_plan::UNNAMED_TABLE`

```rust
const UNNAMED_TABLE: &str = "?table?"
```

Default table name for unnamed table

---

## add_group_by_exprs_from_dependencies

`function` · `datafusion_expr::logical_plan::builder::add_group_by_exprs_from_dependencies`

```rust
fn add_group_by_exprs_from_dependencies(group_expr: Vec<Expr>, schema: &datafusion_common::DFSchemaRef) -> datafusion_common::Result<Vec<Expr>>
```

Add additional "synthetic" group by expressions based on functional
dependencies.

For example, if we are grouping on `[c1]`, and we know from
functional dependencies that column `c1` determines `c2`, this function
adds `c2` to the group by list.

This allows MySQL style selects like
`SELECT col FROM t WHERE pk = 5` if col is unique

---

## build_join_schema

`function` · `datafusion_expr::logical_plan::builder::build_join_schema`

Also reachable as `datafusion::logical_expr::build_join_schema`, `datafusion_expr::build_join_schema`, `datafusion_expr::logical_plan::build_join_schema`

```rust
fn build_join_schema(left: &datafusion_common::DFSchema, right: &datafusion_common::DFSchema, join_type: &logical_plan::JoinType) -> datafusion_common::Result<datafusion_common::DFSchema>
```

Creates a schema for a join operation.
The fields from the left side are first

---

## get_struct_unnested_columns

`function` · `datafusion_expr::logical_plan::builder::get_struct_unnested_columns`

```rust
fn get_struct_unnested_columns(col_name: &String, inner_fields: &arrow::datatypes::Fields) -> Vec<datafusion_common::Column>
```

---

## project

`function` · `datafusion_expr::logical_plan::builder::project`

```rust
fn project(plan: logical_plan::LogicalPlan, expr: impl IntoIterator<Item = impl Into<select_expr::SelectExpr>>) -> datafusion_common::Result<logical_plan::LogicalPlan>
```

Create Projection
# Errors
This function errors under any of the following conditions:
* Two or more expressions have the same name
* An invalid expression is used (e.g. a `sort` expression)

---

## requalify_sides_if_needed

`function` · `datafusion_expr::logical_plan::builder::requalify_sides_if_needed`

Also reachable as `datafusion::logical_expr::requalify_sides_if_needed`, `datafusion_expr::logical_plan::requalify_sides_if_needed`, `datafusion_expr::requalify_sides_if_needed`

```rust
fn requalify_sides_if_needed(left: LogicalPlanBuilder, right: LogicalPlanBuilder) -> datafusion_common::Result<(LogicalPlanBuilder, LogicalPlanBuilder, bool)>
```

(Re)qualify the sides of a join if needed, i.e. if the columns from one side would otherwise
conflict with the columns from the other.
This is especially useful for queries that come as Substrait, since Substrait doesn't currently allow specifying
aliases, neither for columns nor for tables.  DataFusion requires columns to be uniquely identifiable, in some
places (see e.g. DFSchema::check_names).
The function returns:
- The requalified or original left logical plan
- The requalified or original right logical plan
- If a requalification was needed or not

---

## subquery_alias

`function` · `datafusion_expr::logical_plan::builder::subquery_alias`

```rust
fn subquery_alias(plan: logical_plan::LogicalPlan, alias: impl Into<datafusion_common::TableReference>) -> datafusion_common::Result<logical_plan::LogicalPlan>
```

Create a SubqueryAlias to wrap a LogicalPlan.

---

## table_scan

`function` · `datafusion_expr::logical_plan::builder::table_scan`

Also reachable as `datafusion::logical_expr::table_scan`, `datafusion_expr::logical_plan::table_scan`, `datafusion_expr::table_scan`

```rust
fn table_scan(name: Option<impl Into<datafusion_common::TableReference>>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>) -> datafusion_common::Result<LogicalPlanBuilder>
```

Create a LogicalPlanBuilder representing a scan of a table with the provided name and schema.
This is mostly used for testing and documentation.

---

## table_scan_with_filter_and_fetch

`function` · `datafusion_expr::logical_plan::builder::table_scan_with_filter_and_fetch`

```rust
fn table_scan_with_filter_and_fetch(name: Option<impl Into<datafusion_common::TableReference>>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>, filters: Vec<Expr>, fetch: Option<usize>) -> datafusion_common::Result<LogicalPlanBuilder>
```

Create a LogicalPlanBuilder representing a scan of a table with the provided name and schema,
filters, and inlined fetch.
This is mostly used for testing and documentation.

---

## table_scan_with_filters

`function` · `datafusion_expr::logical_plan::builder::table_scan_with_filters`

```rust
fn table_scan_with_filters(name: Option<impl Into<datafusion_common::TableReference>>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>, filters: Vec<Expr>) -> datafusion_common::Result<LogicalPlanBuilder>
```

Create a LogicalPlanBuilder representing a scan of a table with the provided name and schema,
and inlined filters.
This is mostly used for testing and documentation.

---

## table_source

`function` · `datafusion_expr::logical_plan::builder::table_source`

```rust
fn table_source(table_schema: &arrow::datatypes::Schema) -> std::sync::Arc<dyn TableSource>
```

---

## table_source_with_constraints

`function` · `datafusion_expr::logical_plan::builder::table_source_with_constraints`

```rust
fn table_source_with_constraints(table_schema: &arrow::datatypes::Schema, constraints: datafusion_common::Constraints) -> std::sync::Arc<dyn TableSource>
```

---

## union

`function` · `datafusion_expr::logical_plan::builder::union`

Also reachable as `datafusion::logical_expr::union`, `datafusion_expr::logical_plan::union`, `datafusion_expr::union`

```rust
fn union(left_plan: logical_plan::LogicalPlan, right_plan: logical_plan::LogicalPlan) -> datafusion_common::Result<logical_plan::LogicalPlan>
```

Union two [`LogicalPlan`]s.

Constructs the UNION plan, but does not perform type-coercion. Therefore the
subtree expressions will not be properly typed until the optimizer pass.

If a properly typed UNION plan is needed, refer to [`TypeCoercionRewriter::coerce_union`]
or alternatively, merge the union input schema using [`coerce_union_schema`] and
apply the expression rewrite with [`coerce_plan_expr_for_schema`].

[`TypeCoercionRewriter::coerce_union`]: https://docs.rs/datafusion-optimizer/latest/datafusion_optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#method.coerce_union
[`coerce_union_schema`]: https://docs.rs/datafusion-optimizer/latest/datafusion_optimizer/analyzer/type_coercion/fn.coerce_union_schema.html

---

## union_by_name

`function` · `datafusion_expr::logical_plan::builder::union_by_name`

```rust
fn union_by_name(left_plan: logical_plan::LogicalPlan, right_plan: logical_plan::LogicalPlan) -> datafusion_common::Result<logical_plan::LogicalPlan>
```

Like [`union`], but combine rows from different tables by name, rather than
by position.

---

## unique_field_aliases

`function` · `datafusion_expr::logical_plan::builder::unique_field_aliases`

```rust
fn unique_field_aliases(fields: &arrow::datatypes::Fields) -> Vec<Option<String>>
```

Returns aliases to make field names unique.

Returns a vector of optional aliases, one per input field. `None` means keep the original name,
`Some(alias)` means rename to the alias to ensure uniqueness.

Used when creating [`SubqueryAlias`] or similar operations that strip table qualifiers but need
to maintain unique column names.

# Example
Input fields: `[a, a, b, b, a, a:1]` ([`DFSchema`] valid when duplicate fields have different qualifiers)
Returns: `[None, Some("a:1"), None, Some("b:1"), Some("a:2"), Some("a:1:1")]`

---

## unnest

`function` · `datafusion_expr::logical_plan::builder::unnest`

```rust
fn unnest(input: logical_plan::LogicalPlan, columns: Vec<datafusion_common::Column>) -> datafusion_common::Result<logical_plan::LogicalPlan>
```

Create a [`LogicalPlan::Unnest`] plan

---

## unnest_with_options

`function` · `datafusion_expr::logical_plan::builder::unnest_with_options`

```rust
fn unnest_with_options(input: logical_plan::LogicalPlan, columns_to_unnest: Vec<datafusion_common::Column>, options: datafusion_common::UnnestOptions) -> datafusion_common::Result<logical_plan::LogicalPlan>
```

Create a [`LogicalPlan::Unnest`] plan with options
This function receive a list of columns to be unnested
because multiple unnest can be performed on the same column (e.g unnest with different depth)
The new schema will contains post-unnest fields replacing the original field

For example:
Input schema as
```text
+---------------------+-------------------+
| col1                | col2              |
+---------------------+-------------------+
| Struct(INT64,INT32) | List(List(Int64)) |
+---------------------+-------------------+
```



Then unnesting columns with:
- (col1,Struct)
- (col2,List(\[depth=1,depth=2\]))

will generate a new schema as
```text
+---------+---------+---------------------+---------------------+
| col1.c0 | col1.c1 | unnest_col2_depth_1 | unnest_col2_depth_2 |
+---------+---------+---------------------+---------------------+
| Int64   | Int32   | List(Int64)         |  Int64              |
+---------+---------+---------------------+---------------------+
```

---

## validate_unique_names

`function` · `datafusion_expr::logical_plan::builder::validate_unique_names`

```rust
fn validate_unique_names<'a>(node_name: &str, expressions: impl IntoIterator<Item = &'a Expr>) -> datafusion_common::Result<()>
```

Errors if one or more expressions have equal names.

---

## wrap_projection_for_join_if_necessary

`function` · `datafusion_expr::logical_plan::builder::wrap_projection_for_join_if_necessary`

Also reachable as `datafusion::logical_expr::wrap_projection_for_join_if_necessary`, `datafusion_expr::logical_plan::wrap_projection_for_join_if_necessary`, `datafusion_expr::wrap_projection_for_join_if_necessary`

```rust
fn wrap_projection_for_join_if_necessary(join_keys: &[Expr], input: logical_plan::LogicalPlan) -> datafusion_common::Result<(logical_plan::LogicalPlan, Vec<datafusion_common::Column>, bool)>
```

Wrap projection for a plan, if the join keys contains normal expression.

---

## LogicalPlanBuilder

`struct` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilder`

Also reachable as `datafusion::logical_expr::LogicalPlanBuilder`, `datafusion_expr::LogicalPlanBuilder`, `datafusion_expr::logical_plan::LogicalPlanBuilder`

```rust
struct LogicalPlanBuilder
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**Methods** (52)

```rust
fn aggregate(self, group_expr: impl IntoIterator<Item = impl Into<Expr>>, aggr_expr: impl IntoIterator<Item = impl Into<Expr>>) -> Result<Self>
fn alias(self, alias: impl Into<TableReference>) -> Result<Self>
fn build(self) -> Result<LogicalPlan>
fn copy_to(input: LogicalPlan, output_url: String, file_type: Arc<dyn FileType>, options: HashMap<String, String>, partition_by: Vec<String>) -> Result<Self>
fn cross_join(self, right: LogicalPlan) -> Result<Self>
fn distinct(self) -> Result<Self>
fn distinct_on(self, on_expr: Vec<Expr>, select_expr: Vec<Expr>, sort_expr: Option<Vec<SortExpr>>) -> Result<Self>
fn empty(produce_one_row: bool) -> Self
fn except(left_plan: LogicalPlan, right_plan: LogicalPlan, is_all: bool) -> Result<LogicalPlan>
fn explain(self, verbose: bool, analyze: bool) -> Result<Self>
fn explain_option_format(self, explain_option: ExplainOption) -> Result<Self>
fn filter(self, expr: impl Into<Expr>) -> Result<Self>
fn having(self, expr: impl Into<Expr>) -> Result<Self>
fn insert_into(input: LogicalPlan, table_name: impl Into<TableReference>, target: Arc<dyn TableSource>, insert_op: InsertOp) -> Result<Self>
fn intersect(left_plan: LogicalPlan, right_plan: LogicalPlan, is_all: bool) -> Result<LogicalPlan>
fn join(self, right: LogicalPlan, join_type: JoinType, join_keys: (Vec<impl Into<Column>>, Vec<impl Into<Column>>), filter: Option<Expr>) -> Result<Self>
fn join_detailed(self, right: LogicalPlan, join_type: JoinType, join_keys: (Vec<impl Into<Column>>, Vec<impl Into<Column>>), filter: Option<Expr>, null_equality: NullEquality) -> Result<Self>
fn join_detailed_with_options(self, right: LogicalPlan, join_type: JoinType, join_keys: (Vec<impl Into<Column>>, Vec<impl Into<Column>>), filter: Option<Expr>, null_equality: NullEquality, null_aware: bool) -> Result<Self>
fn join_on(self, right: LogicalPlan, join_type: JoinType, on_exprs: impl IntoIterator<Item = Expr>) -> Result<Self>
fn join_using(self, right: LogicalPlan, join_type: JoinType, using_keys: Vec<Column>) -> Result<Self>
fn join_with_expr_keys(self, right: LogicalPlan, join_type: JoinType, equi_exprs: (Vec<impl Into<Expr>>, Vec<impl Into<Expr>>), filter: Option<Expr>) -> Result<Self>
fn limit(self, skip: usize, fetch: Option<usize>) -> Result<Self>
fn limit_by_expr(self, skip: Option<Expr>, fetch: Option<Expr>) -> Result<Self>
fn new(plan: LogicalPlan) -> Self
fn new_from_arc(plan: Arc<LogicalPlan>) -> Self
fn plan(&self) -> &LogicalPlan
fn prepare(self, name: String, fields: Vec<FieldRef>) -> Result<Self>
fn project(self, expr: impl IntoIterator<Item = impl Into<SelectExpr>>) -> Result<Self>
fn project_with_validation(self, expr: Vec<(impl Into<SelectExpr>, bool)>) -> Result<Self>
fn project_with_validation_and_schema(self, expr: impl IntoIterator<Item = impl Into<SelectExpr>>, schema: &DFSchemaRef) -> Result<Self>
fn repartition(self, partitioning_scheme: Partitioning) -> Result<Self>
fn scan(table_name: impl Into<TableReference>, table_source: Arc<dyn TableSource>, projection: Option<Vec<usize>>) -> Result<Self>
fn scan_with_filters(table_name: impl Into<TableReference>, table_source: Arc<dyn TableSource>, projection: Option<Vec<usize>>, filters: Vec<Expr>) -> Result<Self>
fn scan_with_filters_fetch(table_name: impl Into<TableReference>, table_source: Arc<dyn TableSource>, projection: Option<Vec<usize>>, filters: Vec<Expr>, fetch: Option<usize>) -> Result<Self>
fn schema(&self) -> &DFSchemaRef
fn select(self, indices: impl IntoIterator<Item = usize>) -> Result<Self>
fn sort(self, sorts: impl IntoIterator<Item = impl Into<SortExpr>> + Clone) -> Result<Self>
fn sort_by(self, expr: impl IntoIterator<Item = impl Into<Expr>> + Clone) -> Result<Self>
fn sort_with_limit(self, sorts: impl IntoIterator<Item = impl Into<SortExpr>> + Clone, fetch: Option<usize>) -> Result<Self>
fn to_recursive_query(self, name: String, recursive_term: LogicalPlan, is_distinct: bool) -> Result<Self>
fn union(self, plan: LogicalPlan) -> Result<Self>
fn union_by_name(self, plan: LogicalPlan) -> Result<Self>
fn union_by_name_distinct(self, plan: LogicalPlan) -> Result<Self>
fn union_distinct(self, plan: LogicalPlan) -> Result<Self>
fn unnest_column(self, column: impl Into<Column>) -> Result<Self>
fn unnest_column_with_options(self, column: impl Into<Column>, options: UnnestOptions) -> Result<Self>
fn unnest_columns_with_options(self, columns: Vec<Column>, options: UnnestOptions) -> Result<Self>
fn values(values: Vec<Vec<Expr>>) -> Result<Self>
fn values_with_schema(values: Vec<Vec<Expr>>, schema: &DFSchemaRef) -> Result<Self>
fn window(self, window_expr: impl IntoIterator<Item = impl Into<Expr>>) -> Result<Self>
fn window_plan(input: LogicalPlan, window_exprs: impl IntoIterator<Item = Expr>) -> Result<LogicalPlan>
fn with_options(self, options: LogicalPlanBuilderOptions) -> Self
```

**via `core::convert::From`**

```rust
fn from(plan: Arc<LogicalPlan>) -> Self
fn from(plan: LogicalPlan) -> Self
```

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

---

## LogicalPlanBuilderOptions

`struct` · `datafusion_expr::logical_plan::builder::LogicalPlanBuilderOptions`

Also reachable as `datafusion::logical_expr::LogicalPlanBuilderOptions`, `datafusion_expr::LogicalPlanBuilderOptions`, `datafusion_expr::logical_plan::LogicalPlanBuilderOptions`

```rust
struct LogicalPlanBuilderOptions
```

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn new() -> Self
fn with_add_implicit_group_by_exprs(self, add: bool) -> Self
```

Options for [`LogicalPlanBuilder`]

---

## LogicalTableSource

`struct` · `datafusion_expr::logical_plan::builder::LogicalTableSource`

Also reachable as `datafusion::logical_expr::LogicalTableSource`, `datafusion_expr::LogicalTableSource`, `datafusion_expr::logical_plan::LogicalTableSource`

```rust
struct LogicalTableSource
```

**Implements**: `datafusion_expr::table_source::TableSource`

**Methods** (2)

```rust
fn new(table_schema: SchemaRef) -> Self
fn with_constraints(self, constraints: Constraints) -> Self
```

**via `datafusion_expr::table_source::TableSource`**

```rust
fn constraints(&self) -> Option<&Constraints>
fn schema(&self) -> SchemaRef
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
```

Basic TableSource implementation intended for use in tests and documentation. It is expected
that users will provide their own TableSource implementations or use DataFusion's
DefaultTableSource.

---
