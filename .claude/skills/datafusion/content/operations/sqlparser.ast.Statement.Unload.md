# `sqlparser::ast::Statement::Unload`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Unload.json).

<a id="op-ad93d328458539246db82a67"></a>
## auth

`struct_field` · `sqlparser::ast::Statement::Unload::auth` · sqlparser 0.62.0

```rust
auth: Option<IamRoleKind>
```

Source: `src/ast/mod.rs:4756`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional IAM role/auth information.

<a id="op-f2f3102127eddae8fd26d0d8"></a>
## options

`struct_field` · `sqlparser::ast::Statement::Unload::options` · sqlparser 0.62.0

```rust
options: Vec<CopyLegacyOption>
```

Source: `src/ast/mod.rs:4760`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Legacy copy-style options.

<a id="op-d82b7c310c153bdd26dccc4c"></a>
## query

`struct_field` · `sqlparser::ast::Statement::Unload::query` · sqlparser 0.62.0

```rust
query: Option<Box<Query>>
```

Source: `src/ast/mod.rs:4750`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional query AST to unload.

<a id="op-4677afbcc6a5d695b5cc218a"></a>
## query_text

`struct_field` · `sqlparser::ast::Statement::Unload::query_text` · sqlparser 0.62.0

```rust
query_text: Option<String>
```

Source: `src/ast/mod.rs:4752`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional original query text.

<a id="op-9ae90df66690fbf31dd55611"></a>
## to

`struct_field` · `sqlparser::ast::Statement::Unload::to` · sqlparser 0.62.0

```rust
to: Ident
```

Source: `src/ast/mod.rs:4754`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Destination identifier.

<a id="op-b63e44eab677b48f5ab49340"></a>
## with

`struct_field` · `sqlparser::ast::Statement::Unload::with` · sqlparser 0.62.0

```rust
with: Vec<SqlOption>
```

Source: `src/ast/mod.rs:4758`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional `WITH` options.
