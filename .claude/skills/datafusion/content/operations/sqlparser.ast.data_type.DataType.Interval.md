# `sqlparser::ast::data_type::DataType::Interval`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.data_type.DataType.Interval.json).

<a id="op-7a4fcc3883ca8aa485bd103b"></a>
## fields

`struct_field` · `sqlparser::ast::data_type::DataType::Interval::fields` · sqlparser 0.62.0

```rust
fields: Option<IntervalFields>
```

Source: `src/ast/data_type.rs:386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[PostgreSQL] fields specification like `INTERVAL YEAR TO MONTH`.

[PostgreSQL]: https://www.postgresql.org/docs/17/datatype-datetime.html

<a id="op-3236fbd148c63475d2c8518b"></a>
## precision

`struct_field` · `sqlparser::ast::data_type::DataType::Interval::precision` · sqlparser 0.62.0

```rust
precision: Option<u64>
```

Source: `src/ast/data_type.rs:390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[PostgreSQL] subsecond precision like `INTERVAL HOUR TO SECOND(3)`

[PostgreSQL]: https://www.postgresql.org/docs/17/datatype-datetime.html
