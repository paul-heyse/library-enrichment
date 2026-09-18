# `sqlparser::ast::Statement::CreateSecret`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.CreateSecret.json).

<a id="op-114cef0a6107146482cd346a"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::Statement::CreateSecret::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/mod.rs:3727`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `IF NOT EXISTS` was present.

<a id="op-7cecb9d0a138f90b9817dbeb"></a>
## name

`struct_field` · `sqlparser::ast::Statement::CreateSecret::name` · sqlparser 0.62.0

```rust
name: Option<Ident>
```

Source: `src/ast/mod.rs:3729`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional secret name.

<a id="op-a36f4244b500dd668e658192"></a>
## options

`struct_field` · `sqlparser::ast::Statement::CreateSecret::options` · sqlparser 0.62.0

```rust
options: Vec<SecretOption>
```

Source: `src/ast/mod.rs:3735`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional secret options.

<a id="op-8665753b7c6aec419b516656"></a>
## or_replace

`struct_field` · `sqlparser::ast::Statement::CreateSecret::or_replace` · sqlparser 0.62.0

```rust
or_replace: bool
```

Source: `src/ast/mod.rs:3723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `OR REPLACE` was specified.

<a id="op-a770d206020c80519a871d45"></a>
## secret_type

`struct_field` · `sqlparser::ast::Statement::CreateSecret::secret_type` · sqlparser 0.62.0

```rust
secret_type: Ident
```

Source: `src/ast/mod.rs:3733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The secret type identifier.

<a id="op-7e691ad3aeed476d0bd6ca6d"></a>
## storage_specifier

`struct_field` · `sqlparser::ast::Statement::CreateSecret::storage_specifier` · sqlparser 0.62.0

```rust
storage_specifier: Option<Ident>
```

Source: `src/ast/mod.rs:3731`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional storage specifier identifier.

<a id="op-a2941bb452ff368e0b9eb094"></a>
## temporary

`struct_field` · `sqlparser::ast::Statement::CreateSecret::temporary` · sqlparser 0.62.0

```rust
temporary: Option<bool>
```

Source: `src/ast/mod.rs:3725`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `TEMPORARY` flag.
