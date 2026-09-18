# `sqlparser::ast::FunctionArg::Named`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionArg.Named.json).

<a id="op-e1bfc64f27c1a0be5a06e4b1"></a>
## arg

`struct_field` · `sqlparser::ast::FunctionArg::Named::arg` · sqlparser 0.62.0

```rust
arg: FunctionArgExpr
```

Source: `src/ast/mod.rs:7925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The argument expression or wildcard form.

<a id="op-9b6cd674ba8c0b1a2c0ba48d"></a>
## name

`struct_field` · `sqlparser::ast::FunctionArg::Named::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:7923`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The identifier name of the argument.

<a id="op-4b95a1ca5d7ce929a808b1ae"></a>
## operator

`struct_field` · `sqlparser::ast::FunctionArg::Named::operator` · sqlparser 0.62.0

```rust
operator: FunctionArgOperator
```

Source: `src/ast/mod.rs:7927`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The operator separating name and value.
