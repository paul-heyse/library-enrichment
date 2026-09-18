# `datafusion_sql::planner`

Crate `datafusion-sql` · 7 public items · structured records in [`model/datafusion_sql.planner.json`](../model/datafusion_sql.planner.json)

## NullOrdering

`enum` · `datafusion_sql::planner::NullOrdering`

```rust
enum NullOrdering
```

**Variants**: `NullsMax`, `NullsMin`, `NullsFirst`, `NullsLast`

**Implements**: `core::convert::From`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug

**Methods** (1)

```rust
fn nulls_first(&self, asc: bool) -> bool
```

**via `core::convert::From`**

```rust
fn from(s: &str) -> Self
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.planner.NullOrdering.md).


Represents the null ordering for sorting expressions.

---

## object_name_to_qualifier

`function` · `datafusion_sql::planner::object_name_to_qualifier`

```rust
fn object_name_to_qualifier(sql_table_name: &sqlparser::ast::ObjectName, enable_normalization: bool) -> datafusion_common::Result<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.planner.object_name_to_qualifier.md).


Construct a WHERE qualifier suitable for e.g. information_schema filtering
from the provided object identifiers (catalog, schema and table names).

---

## object_name_to_table_reference

`function` · `datafusion_sql::planner::object_name_to_table_reference`

```rust
fn object_name_to_table_reference(object_name: sqlparser::ast::ObjectName, enable_normalization: bool) -> datafusion_common::Result<datafusion_common::TableReference>
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.planner.object_name_to_table_reference.md).


Create a [`TableReference`] after normalizing the specified ObjectName

Examples
```text
['foo']          -> Bare { table: "foo" }
['"foo.bar"]]    -> Bare { table: "foo.bar" }
['foo', 'Bar']   -> Partial { schema: "foo", table: "bar" } <-- note lower case "bar"
['foo', 'bar']   -> Partial { schema: "foo", table: "bar" }
['foo', '"Bar"'] -> Partial { schema: "foo", table: "Bar" }
```

---

## IdentNormalizer

`struct` · `datafusion_sql::planner::IdentNormalizer`

```rust
struct IdentNormalizer
```

**Derives**: Debug, Default

**Methods** (2)

```rust
fn new(normalize: bool) -> Self
fn normalize(&self, ident: Ident) -> String
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.planner.IdentNormalizer.md).


Ident Normalizer

---

## ParserOptions

`struct` · `datafusion_sql::planner::ParserOptions`

```rust
struct ParserOptions
```

**Fields**: `parse_float_as_decimal`, `enable_ident_normalization`, `support_varchar_with_length`, `enable_options_value_normalization`, `collect_spans`, `map_string_types_to_utf8view`, `default_null_ordering`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Default

**Methods** (8)

```rust
fn new() -> Self
fn with_collect_spans(self, value: bool) -> Self
fn with_default_null_ordering(self, value: NullOrdering) -> Self
fn with_enable_ident_normalization(self, value: bool) -> Self
fn with_enable_options_value_normalization(self, value: bool) -> Self
fn with_map_string_types_to_utf8view(self, value: bool) -> Self
fn with_parse_float_as_decimal(self, value: bool) -> Self
fn with_support_varchar_with_length(self, value: bool) -> Self
```

**via `core::convert::From`**

```rust
fn from(options: &SqlParserOptions) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.planner.ParserOptions.md).


SQL parser options

---

## PlannerContext

`struct` · `datafusion_sql::planner::PlannerContext`

```rust
struct PlannerContext
```

**Derives**: Clone, Debug, Default

**Methods** (18)

```rust
fn append_outer_query_schema(&mut self, schema: DFSchemaRef)
fn contains_cte(&self, cte_name: &str) -> bool
fn extend_outer_from_schema(&mut self, schema: &DFSchemaRef) -> Result<()>
fn get_cte(&self, cte_name: &str) -> Option<&LogicalPlan>
fn insert_cte(&mut self, cte_name: impl Into<String>, plan: LogicalPlan)
fn lambda_parameters(&self) -> &HashMap<String, FieldRef>
fn latest_outer_query_schema(&self) -> Option<&DFSchemaRef>
fn new() -> Self
fn outer_from_schema(&self) -> Option<Arc<DFSchema>>
fn outer_queries_schemas(&self) -> &[DFSchemaRef]
fn outer_schemas_iter(&self) -> impl Iterator<Item = &DFSchemaRef>
fn pop_outer_query_schema(&mut self) -> Option<DFSchemaRef>
fn prepare_param_data_types(&self) -> &[Option<FieldRef>]
fn set_outer_from_schema(&mut self, schema: Option<DFSchemaRef>) -> Option<DFSchemaRef>
fn set_table_schema(&mut self, schema: Option<DFSchemaRef>) -> Option<DFSchemaRef>
fn table_schema(&self) -> Option<DFSchemaRef>
fn with_lambda_parameters(self, parameters: impl IntoIterator<Item = FieldRef>) -> Self
fn with_prepare_param_data_types(self, prepare_param_data_types: Vec<Option<FieldRef>>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.planner.PlannerContext.md).


Struct to store the states used by the Planner. The Planner will leverage the states
to resolve CTEs, Views, subqueries and PREPARE statements. The states include
Common Table Expression (CTE) provided with WITH clause and
Parameter Data Types provided with PREPARE statement and the query schema of the
outer query plan.

# Cloning

Only the `ctes` are truly cloned when the `PlannerContext` is cloned.
This helps resolve scoping issues of CTEs.
By using cloning, a subquery can inherit CTEs from the outer query
and can also define its own private CTEs without affecting the outer query.

---

## SqlToRel

`struct` · `datafusion_sql::planner::SqlToRel`

```rust
struct SqlToRel<'a, S: ContextProvider>
```

**Methods** (10)

```rust
fn build_schema(&self, columns: Vec<SQLColumnDef>) -> Result<Schema>
fn new(context_provider: &'a S) -> Self
fn new_constraint_from_table_constraints(&self, constraints: &[TableConstraint], df_schema: &DFSchemaRef) -> Result<Constraints>
fn new_with_options(context_provider: &'a S, options: ParserOptions) -> Self
fn sql_statement_to_plan(&self, statement: Statement) -> Result<LogicalPlan>
fn sql_statement_to_plan_with_context(&self, statement: Statement, planner_context: &mut PlannerContext) -> Result<LogicalPlan>
fn sql_to_expr(&self, sql: SQLExpr, schema: &DFSchema, planner_context: &mut PlannerContext) -> Result<Expr>
fn sql_to_expr_with_alias(&self, sql: SQLExprWithAlias, schema: &DFSchema, planner_context: &mut PlannerContext) -> Result<Expr>
fn statement_to_plan(&self, statement: DFStatement) -> Result<LogicalPlan>
fn take_warnings(&self) -> Vec<Diagnostic>
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.planner.SqlToRel.md).


SQL query planner and binder

This struct is used to convert a SQL AST into a [`LogicalPlan`].

You can control the behavior of the planner by providing [`ParserOptions`].

It performs the following tasks:

1. Name and type resolution (called "binding" in other systems). This
   phase looks up table and column names using the [`ContextProvider`].
2. Mechanical translation of the AST into a [`LogicalPlan`].

It does not perform type coercion, or perform optimization, which are done
by subsequent passes.

Key interfaces are:
* [`Self::sql_statement_to_plan`]: Convert a statement
  (e.g. `SELECT ...`) into a [`LogicalPlan`]
* [`Self::sql_to_expr`]: Convert an expression (e.g. `1 + 2`) into an [`Expr`]

---
