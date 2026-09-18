# `sqlparser::ast::Statement::Copy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Copy.json).

<a id="op-78d8cd5c3fbd9bc344fda190"></a>
## legacy_options

`struct_field` · `sqlparser::ast::Statement::Copy::legacy_options` · sqlparser 0.62.0

```rust
legacy_options: Vec<CopyLegacyOption>
```

Source: `src/ast/mod.rs:3619`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

WITH options (before PostgreSQL version 9.0)

<a id="op-68d01d87b8f9dac59391a40c"></a>
## options

`struct_field` · `sqlparser::ast::Statement::Copy::options` · sqlparser 0.62.0

```rust
options: Vec<CopyOption>
```

Source: `src/ast/mod.rs:3617`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

WITH options (from PostgreSQL version 9.0)

<a id="op-95b8023c9f91dc545b1d3921"></a>
## source

`struct_field` · `sqlparser::ast::Statement::Copy::source` · sqlparser 0.62.0

```rust
source: CopySource
```

Source: `src/ast/mod.rs:3611`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The source of 'COPY TO', or the target of 'COPY FROM'

<a id="op-6453ee2a77c38621d1339871"></a>
## target

`struct_field` · `sqlparser::ast::Statement::Copy::target` · sqlparser 0.62.0

```rust
target: CopyTarget
```

Source: `src/ast/mod.rs:3615`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The target of 'COPY TO', or the source of 'COPY FROM'

<a id="op-d395b28c8b4c172481c75082"></a>
## to

`struct_field` · `sqlparser::ast::Statement::Copy::to` · sqlparser 0.62.0

```rust
to: bool
```

Source: `src/ast/mod.rs:3613`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If true, is a 'COPY TO' statement. If false is a 'COPY FROM'

<a id="op-20c108a59f081db59aad6638"></a>
## values

`struct_field` · `sqlparser::ast::Statement::Copy::values` · sqlparser 0.62.0

```rust
values: Vec<Option<String>>
```

Source: `src/ast/mod.rs:3621`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

VALUES a vector of values to be copied
