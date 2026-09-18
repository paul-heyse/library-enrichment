# `sqlparser::ast::Statement::Fetch`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Fetch.json).

<a id="op-378f42a4e9f96160e60f5649"></a>
## direction

`struct_field` · `sqlparser::ast::Statement::Fetch::direction` · sqlparser 0.62.0

```rust
direction: FetchDirection
```

Source: `src/ast/mod.rs:4050`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The fetch direction (e.g., `FORWARD`, `BACKWARD`).

<a id="op-b760b8fc58d0fffa28c269de"></a>
## into

`struct_field` · `sqlparser::ast::Statement::Fetch::into` · sqlparser 0.62.0

```rust
into: Option<ObjectName>
```

Source: `src/ast/mod.rs:4054`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional target table to fetch rows into.

<a id="op-2305bd8c209fe14d7a672c17"></a>
## name

`struct_field` · `sqlparser::ast::Statement::Fetch::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:4048`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Cursor name

<a id="op-bc700959e3c36fff8610d891"></a>
## position

`struct_field` · `sqlparser::ast::Statement::Fetch::position` · sqlparser 0.62.0

```rust
position: FetchPosition
```

Source: `src/ast/mod.rs:4052`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The fetch position (e.g., `ALL`, `NEXT`, `ABSOLUTE`).
