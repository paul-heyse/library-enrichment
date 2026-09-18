# `sqlparser::ast::Statement::LoadData`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.LoadData.json).

<a id="op-f34f2bbedac83a1401ce83dc"></a>
## inpath

`struct_field` · `sqlparser::ast::Statement::LoadData::inpath` · sqlparser 0.62.0

```rust
inpath: String
```

Source: `src/ast/mod.rs:4841`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Input path for files to load.

<a id="op-e90008ebb6f6fb52dae783c1"></a>
## local

`struct_field` · `sqlparser::ast::Statement::LoadData::local` · sqlparser 0.62.0

```rust
local: bool
```

Source: `src/ast/mod.rs:4839`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `LOCAL` is present.

<a id="op-0a0880c6a8103c8d8ff82376"></a>
## overwrite

`struct_field` · `sqlparser::ast::Statement::LoadData::overwrite` · sqlparser 0.62.0

```rust
overwrite: bool
```

Source: `src/ast/mod.rs:4843`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `OVERWRITE` was specified.

<a id="op-8e16b7b3f64af50a58a5ec91"></a>
## partitioned

`struct_field` · `sqlparser::ast::Statement::LoadData::partitioned` · sqlparser 0.62.0

```rust
partitioned: Option<Vec<Expr>>
```

Source: `src/ast/mod.rs:4847`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional partition specification.

<a id="op-30decd67ff5a451c24c5374c"></a>
## table_format

`struct_field` · `sqlparser::ast::Statement::LoadData::table_format` · sqlparser 0.62.0

```rust
table_format: Option<HiveLoadDataFormat>
```

Source: `src/ast/mod.rs:4849`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional table format information.

<a id="op-afe7d146f5339a18d1e47f5f"></a>
## table_name

`struct_field` · `sqlparser::ast::Statement::LoadData::table_name` · sqlparser 0.62.0

```rust
table_name: ObjectName
```

Source: `src/ast/mod.rs:4845`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Target table name to load into.
