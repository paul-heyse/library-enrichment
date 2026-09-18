# `sqlparser::ast::query::TableFactor::Function`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.Function.json).

<a id="op-a3f45e2e877e90a0de269223"></a>
## alias

`struct_field` · `sqlparser::ast::query::TableFactor::Function::alias` · sqlparser 0.62.0

```rust
alias: Option<TableAlias>
```

Source: `src/ast/query.rs:1528`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the result of the function.

<a id="op-2daf35f03c9f2237b074c9fa"></a>
## args

`struct_field` · `sqlparser::ast::query::TableFactor::Function::args` · sqlparser 0.62.0

```rust
args: Vec<FunctionArg>
```

Source: `src/ast/query.rs:1524`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Arguments passed to the function.

<a id="op-ae3213b0ccd9d24407aacf3e"></a>
## lateral

`struct_field` · `sqlparser::ast::query::TableFactor::Function::lateral` · sqlparser 0.62.0

```rust
lateral: bool
```

Source: `src/ast/query.rs:1520`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the function is LATERAL.

<a id="op-5b749ec59b04fed69a21515c"></a>
## name

`struct_field` · `sqlparser::ast::query::TableFactor::Function::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/query.rs:1522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the table function.

<a id="op-fa4b463fa90e0ad3a4690ba4"></a>
## with_ordinality

`struct_field` · `sqlparser::ast::query::TableFactor::Function::with_ordinality` · sqlparser 0.62.0

```rust
with_ordinality: bool
```

Source: `src/ast/query.rs:1526`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `WITH ORDINALITY` was specified to include ordinality.
