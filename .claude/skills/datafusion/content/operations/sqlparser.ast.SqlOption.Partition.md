# `sqlparser::ast::SqlOption::Partition`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SqlOption.Partition.json).

<a id="op-d48ec8d072bf0bdaee11286b"></a>
## column_name

`struct_field` · `sqlparser::ast::SqlOption::Partition::column_name` · sqlparser 0.62.0

```rust
column_name: Ident
```

Source: `src/ast/mod.rs:8822`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The partition column name.

<a id="op-47bb6a220f44768a0f40a4a9"></a>
## for_values

`struct_field` · `sqlparser::ast::SqlOption::Partition::for_values` · sqlparser 0.62.0

```rust
for_values: Vec<Expr>
```

Source: `src/ast/mod.rs:8826`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Values that define the partition boundaries.

<a id="op-15b5788ea279be2b83db02cf"></a>
## range_direction

`struct_field` · `sqlparser::ast::SqlOption::Partition::range_direction` · sqlparser 0.62.0

```rust
range_direction: Option<PartitionRangeDirection>
```

Source: `src/ast/mod.rs:8824`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional direction for the partition range (LEFT/RIGHT).
