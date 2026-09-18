# `sqlparser::ast::Statement::Execute`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Execute.json).

<a id="op-f3eef5f2f46c603d3099ae92"></a>
## default

`struct_field` · `sqlparser::ast::Statement::Execute::default` · sqlparser 0.62.0

```rust
default: bool
```

Source: `src/ast/mod.rs:4553`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to invoke the procedure with the default parameter values
MSSQL: <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/execute-transact-sql?view=sql-server-ver17#default>

<a id="op-43703ddac36d02af115a82e0"></a>
## has_parentheses

`struct_field` · `sqlparser::ast::Statement::Execute::has_parentheses` · sqlparser 0.62.0

```rust
has_parentheses: bool
```

Source: `src/ast/mod.rs:4541`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether parentheses were present around `parameters`.

<a id="op-c4388feb68a2350021ae3996"></a>
## immediate

`struct_field` · `sqlparser::ast::Statement::Execute::immediate` · sqlparser 0.62.0

```rust
immediate: bool
```

Source: `src/ast/mod.rs:4543`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Is this an `EXECUTE IMMEDIATE`.

<a id="op-76e4c56071f3ed571938dbc1"></a>
## into

`struct_field` · `sqlparser::ast::Statement::Execute::into` · sqlparser 0.62.0

```rust
into: Vec<Ident>
```

Source: `src/ast/mod.rs:4545`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Identifiers to capture results into.

<a id="op-c4da88577f2b9c32a02b36ad"></a>
## name

`struct_field` · `sqlparser::ast::Statement::Execute::name` · sqlparser 0.62.0

```rust
name: Option<ObjectName>
```

Source: `src/ast/mod.rs:4537`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional function/procedure name.

<a id="op-873e962ac6698422032c9309"></a>
## output

`struct_field` · `sqlparser::ast::Statement::Execute::output` · sqlparser 0.62.0

```rust
output: bool
```

Source: `src/ast/mod.rs:4550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the last parameter is the return value of the procedure
MSSQL: <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/execute-transact-sql?view=sql-server-ver17#output>

<a id="op-ba1e11b5e97c382e22e76d16"></a>
## parameters

`struct_field` · `sqlparser::ast::Statement::Execute::parameters` · sqlparser 0.62.0

```rust
parameters: Vec<Expr>
```

Source: `src/ast/mod.rs:4539`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parameter expressions passed to execute.

<a id="op-29a73f3bc297dca2e6179f3f"></a>
## using

`struct_field` · `sqlparser::ast::Statement::Execute::using` · sqlparser 0.62.0

```rust
using: Vec<ExprWithAlias>
```

Source: `src/ast/mod.rs:4547`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`USING` expressions with optional aliases.
