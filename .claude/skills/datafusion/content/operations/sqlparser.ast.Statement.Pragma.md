# `sqlparser::ast::Statement::Pragma`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Pragma.json).

<a id="op-506e4c08add300a4251f9fd1"></a>
## is_eq

`struct_field` · `sqlparser::ast::Statement::Pragma::is_eq` · sqlparser 0.62.0

```rust
is_eq: bool
```

Source: `src/ast/mod.rs:4716`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the pragma used `=`.

<a id="op-6a911d851b662028190aff8b"></a>
## name

`struct_field` · `sqlparser::ast::Statement::Pragma::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:4712`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Pragma name (possibly qualified).

<a id="op-ca6f64d3d3a234d565d44b09"></a>
## value

`struct_field` · `sqlparser::ast::Statement::Pragma::value` · sqlparser 0.62.0

```rust
value: Option<ValueWithSpan>
```

Source: `src/ast/mod.rs:4714`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional pragma value.
