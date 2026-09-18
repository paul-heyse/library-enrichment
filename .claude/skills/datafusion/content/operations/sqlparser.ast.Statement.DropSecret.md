# `sqlparser::ast::Statement::DropSecret`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.DropSecret.json).

<a id="op-6603f4c5f180c3a7101f9bf6"></a>
## if_exists

`struct_field` · `sqlparser::ast::Statement::DropSecret::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/mod.rs:3966`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `IF EXISTS` was present.

<a id="op-3a1742a4abd45dab15f322e8"></a>
## name

`struct_field` · `sqlparser::ast::Statement::DropSecret::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:3970`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the secret to drop.

<a id="op-f27701a95fe43c05049063fc"></a>
## storage_specifier

`struct_field` · `sqlparser::ast::Statement::DropSecret::storage_specifier` · sqlparser 0.62.0

```rust
storage_specifier: Option<Ident>
```

Source: `src/ast/mod.rs:3972`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional storage specifier identifier.

<a id="op-c09daa0f15e679820a4eee5c"></a>
## temporary

`struct_field` · `sqlparser::ast::Statement::DropSecret::temporary` · sqlparser 0.62.0

```rust
temporary: Option<bool>
```

Source: `src/ast/mod.rs:3968`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `TEMPORARY` marker.
