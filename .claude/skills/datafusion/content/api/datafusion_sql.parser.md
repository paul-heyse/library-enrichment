# `datafusion_sql::parser`

Crate `datafusion-sql` · 9 public items · structured records in [`model/datafusion_sql.parser.json`](../model/datafusion_sql.parser.json)

## CopyToSource

`enum` · `datafusion_sql::parser::CopyToSource`

```rust
enum CopyToSource
```

**Variants**: `Relation`, `Query`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## ParserInput

`enum` · `datafusion_sql::parser::ParserInput`

```rust
enum ParserInput<'a>
```

**Variants**: `Sql`, `Tokens`

**Implements**: `core::convert::From`

**via `core::convert::From`**

```rust
fn from(tokens: Vec<TokenWithSpan>) -> Self
fn from(sql: &'a str) -> Self
```

Describes a possible input for parser

---

## ResetStatement

`enum` · `datafusion_sql::parser::ResetStatement`

```rust
enum ResetStatement
```

**Variants**: `Variable`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

DataFusion extension for `RESET`

---

## Statement

`enum` · `datafusion_sql::parser::Statement`

```rust
enum Statement
```

**Variants**: `Statement`, `CreateExternalTable`, `CopyTo`, `Explain`, `Reset`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

DataFusion SQL Statement.

This can either be a [`Statement`] from [`sqlparser`] from a
standard SQL dialect, or a DataFusion extension such as `CREATE
EXTERNAL TABLE`. See [`DFParser`] for more information.

[`Statement`]: sqlparser::ast::Statement

---

## CopyToStatement

`struct` · `datafusion_sql::parser::CopyToStatement`

```rust
struct CopyToStatement
```

**Fields**: `source`, `target`, `partitioned_by`, `stored_as`, `options`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

DataFusion extension DDL for `COPY`

# Syntax:

```text
COPY <table_name | (<query>)>
TO
<destination_url>
(key_value_list)
```

# Examples

```sql
COPY lineitem  TO 'lineitem'
STORED AS PARQUET (
  partitions 16,
  row_group_limit_rows 100000,
  row_group_limit_bytes 200000
)

COPY (SELECT l_orderkey from lineitem) to 'lineitem.parquet';
```

---

## CreateExternalTable

`struct` · `datafusion_sql::parser::CreateExternalTable`

```rust
struct CreateExternalTable
```

**Fields**: `name`, `columns`, `file_type`, `locations`, `table_partition_cols`, `order_exprs`, `if_not_exists`, `or_replace`, `temporary`, `unbounded`, `options`, `constraints`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

DataFusion extension DDL for `CREATE EXTERNAL TABLE`

Syntax:

```text
CREATE
[ OR REPLACE ]
EXTERNAL TABLE
[ IF NOT EXISTS ]
<TABLE_NAME>[ (<column_definition>) ]
STORED AS <file_type>
[ PARTITIONED BY (<column_definition list> | <column list>) ]
[ WITH ORDER (<ordered column list>)
[ OPTIONS (<key_value_list>) ]
LOCATION <literal> | LOCATION (<literal>[, ...])

<column_definition> := (<column_name> <data_type>, ...)

<column_list> := (<column_name>, ...)

<ordered_column_list> := (<column_name> <sort_clause>, ...)

<key_value_list> := (<literal> <literal, <literal> <literal>, ...)
```

---

## DFParser

`struct` · `datafusion_sql::parser::DFParser`

```rust
struct DFParser<'a>
```

**Fields**: `parser`

**Methods** (17)

```rust
fn parse_copy(&mut self) -> Result<Statement, DataFusionError>
fn parse_create(&mut self) -> Result<Statement, DataFusionError>
fn parse_explain(&mut self) -> Result<Statement, DataFusionError>
fn parse_explain_format(&mut self) -> Result<Option<String>, DataFusionError>
fn parse_expr(&mut self) -> Result<ExprWithAlias, DataFusionError>
fn parse_into_expr(&mut self) -> Result<ExprWithAlias, DataFusionError>
fn parse_option_key(&mut self) -> Result<String, DataFusionError>
fn parse_option_value(&mut self) -> Result<Value, DataFusionError>
fn parse_order_by_expr(&mut self) -> Result<OrderByExpr, DataFusionError>
fn parse_order_by_exprs(&mut self) -> Result<Vec<OrderByExpr>, DataFusionError>
fn parse_reset(&mut self) -> Result<Statement, DataFusionError>
fn parse_sql(sql: &'a str) -> Result<VecDeque<Statement>, DataFusionError>
fn parse_sql_into_expr(sql: &str) -> Result<ExprWithAlias, DataFusionError>
fn parse_sql_into_expr_with_dialect(sql: &str, dialect: &dyn Dialect) -> Result<ExprWithAlias, DataFusionError>
fn parse_sql_with_dialect(sql: &str, dialect: &dyn Dialect) -> Result<VecDeque<Statement>, DataFusionError>
fn parse_statement(&mut self) -> Result<Statement, DataFusionError>
fn parse_statements(&mut self) -> Result<VecDeque<Statement>, DataFusionError>
```

DataFusion SQL Parser based on [`sqlparser`]

Parses DataFusion's SQL dialect, often delegating to [`sqlparser`]'s [`Parser`].

DataFusion mostly follows existing SQL dialects via
`sqlparser`. However, certain statements such as `COPY` and
`CREATE EXTERNAL TABLE` have special syntax in DataFusion. See
[`Statement`] for a list of this special syntax

---

## DFParserBuilder

`struct` · `datafusion_sql::parser::DFParserBuilder`

```rust
struct DFParserBuilder<'a, 'b>
```

**Methods** (4)

```rust
fn build(self) -> Result<DFParser<'b>, DataFusionError>
fn new(input: impl Into<ParserInput<'a>>) -> Self
fn with_dialect(self, dialect: &'b dyn Dialect) -> Self
fn with_recursion_limit(self, recursion_limit: usize) -> Self
```

Builder for [`DFParser`]

# Example: Create and Parse SQL statements
```
# use datafusion_sql::parser::DFParserBuilder;
# use datafusion_common::Result;
# fn test() -> Result<()> {
let mut parser = DFParserBuilder::new("SELECT * FROM foo; SELECT 1 + 2").build()?;
// parse the SQL into DFStatements
let statements = parser.parse_statements()?;
assert_eq!(statements.len(), 2);
# Ok(())
# }
```

# Example: Create and Parse expression with a different dialect
```
# use datafusion_sql::parser::DFParserBuilder;
# use datafusion_common::Result;
# use datafusion_sql::sqlparser::dialect::MySqlDialect;
# use datafusion_sql::sqlparser::ast::Expr;
# fn test() -> Result<()> {
let dialect = MySqlDialect {}; // Parse using MySQL dialect
let mut parser = DFParserBuilder::new("1 + 2")
    .with_dialect(&dialect)
    .build()?;
// parse 1+2 into an sqlparser::ast::Expr
let res = parser.parse_expr()?;
assert!(matches!(res.expr, Expr::BinaryOp { .. }));
# Ok(())
# }
```

---

## ExplainStatement

`struct` · `datafusion_sql::parser::ExplainStatement`

```rust
struct ExplainStatement
```

**Fields**: `options`, `statement`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

DataFusion specific `EXPLAIN`

Supports both the legacy keyword form and, on dialects whose
[`Dialect::supports_explain_with_utility_options`] returns `true`
(PostgreSQL, DuckDB, etc.), the Postgres-style parenthesized option list:

```sql
-- Legacy keyword form (any dialect)
EXPLAIN <ANALYZE> <VERBOSE> [FORMAT format] statement

-- Postgres-style option form (dialect-gated)
EXPLAIN (option [arg] [, ...]) statement
```

See [`ExplainStatementOptions`] for the list of supported options in the
parenthesized form.

---
