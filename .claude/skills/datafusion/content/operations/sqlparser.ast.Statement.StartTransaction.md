# `sqlparser::ast::Statement::StartTransaction`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.StartTransaction.json).

<a id="op-163be98f6522cfaa8f5c7fe5"></a>
## begin

`struct_field` · `sqlparser::ast::Statement::StartTransaction::begin` · sqlparser 0.62.0

```rust
begin: bool
```

Source: `src/ast/mod.rs:4259`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when this was parsed as `BEGIN` instead of `START`.

<a id="op-ff35a0d7acca3378facd53bb"></a>
## exception

`struct_field` · `sqlparser::ast::Statement::StartTransaction::exception` · sqlparser 0.62.0

```rust
exception: Option<Vec<ExceptionWhen>>
```

Source: `src/ast/mod.rs:4286`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Exception handling with exception clauses.
Example:
```sql
EXCEPTION
    WHEN EXCEPTION_1 THEN
        SELECT 2;
    WHEN EXCEPTION_2 OR EXCEPTION_3 THEN
        SELECT 3;
    WHEN OTHER THEN
        SELECT 4;
```
<https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#beginexceptionend>
<https://docs.snowflake.com/en/sql-reference/snowflake-scripting/exception>

<a id="op-4c2364ea0999e4767b430985"></a>
## has_end_keyword

`struct_field` · `sqlparser::ast::Statement::StartTransaction::has_end_keyword` · sqlparser 0.62.0

```rust
has_end_keyword: bool
```

Source: `src/ast/mod.rs:4288`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

TRUE if the statement has an `END` keyword.

<a id="op-37e8eb80e7a5f55ae0917f35"></a>
## modes

`struct_field` · `sqlparser::ast::Statement::StartTransaction::modes` · sqlparser 0.62.0

```rust
modes: Vec<TransactionMode>
```

Source: `src/ast/mod.rs:4257`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Transaction modes such as `ISOLATION LEVEL` or `READ WRITE`.

<a id="op-0ebf0f509a1af9ad6c1b41d9"></a>
## modifier

`struct_field` · `sqlparser::ast::Statement::StartTransaction::modifier` · sqlparser 0.62.0

```rust
modifier: Option<TransactionModifier>
```

Source: `src/ast/mod.rs:4263`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional transaction modifier (e.g., `AND NO CHAIN`).

<a id="op-8c87c514c559ffc8de848a8e"></a>
## statements

`struct_field` · `sqlparser::ast::Statement::StartTransaction::statements` · sqlparser 0.62.0

```rust
statements: Vec<Statement>
```

Source: `src/ast/mod.rs:4272`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of statements belonging to the `BEGIN` block.
Example:
```sql
BEGIN
    SELECT 1;
    SELECT 2;
END;
```

<a id="op-c4e1e19ca56cb6f2fc4a97a0"></a>
## transaction

`struct_field` · `sqlparser::ast::Statement::StartTransaction::transaction` · sqlparser 0.62.0

```rust
transaction: Option<BeginTransactionKind>
```

Source: `src/ast/mod.rs:4261`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional specific keyword used: `TRANSACTION` or `WORK`.
