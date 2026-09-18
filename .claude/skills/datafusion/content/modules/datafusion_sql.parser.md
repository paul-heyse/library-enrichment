# `datafusion_sql::parser`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.parser.json).

<a id="op-6fb518c9479e67bcdfc4fc73"></a>
## parser

`module` · `datafusion_sql::parser` · datafusion-sql 55.1.0

```rust
mod parser
```

Source: `src/parser.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

[`DFParser`](../operations/datafusion_sql.parser.DFParser.md#op-c4bfa9a3c20364cfab8a7ca0): DataFusion SQL Parser based on [`sqlparser`](../modules/sqlparser.md#op-209408c3e0722e98219d2290)

This parser implements DataFusion specific statements such as
`CREATE EXTERNAL TABLE`
