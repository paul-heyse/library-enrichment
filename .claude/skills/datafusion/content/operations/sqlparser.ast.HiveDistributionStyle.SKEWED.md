# `sqlparser::ast::HiveDistributionStyle::SKEWED`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.HiveDistributionStyle.SKEWED.json).

<a id="op-e879c05160b4a6c6d2c7d217"></a>
## columns

`struct_field` · `sqlparser::ast::HiveDistributionStyle::SKEWED::columns` · sqlparser 0.62.0

```rust
columns: Vec<ColumnDef>
```

Source: `src/ast/mod.rs:8548`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Columns participating in the skew definition.

<a id="op-d2ba97371f0de1b3885c058c"></a>
## on

`struct_field` · `sqlparser::ast::HiveDistributionStyle::SKEWED::on` · sqlparser 0.62.0

```rust
on: Vec<ColumnDef>
```

Source: `src/ast/mod.rs:8550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Columns listed in the `ON` clause for skewing.

<a id="op-0ea038d2dddb35713896ffaf"></a>
## stored_as_directories

`struct_field` · `sqlparser::ast::HiveDistributionStyle::SKEWED::stored_as_directories` · sqlparser 0.62.0

```rust
stored_as_directories: bool
```

Source: `src/ast/mod.rs:8552`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether skewed data is stored as directories.
