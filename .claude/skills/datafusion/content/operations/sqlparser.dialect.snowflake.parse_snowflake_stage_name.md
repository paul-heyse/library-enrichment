# `sqlparser::dialect::snowflake::parse_snowflake_stage_name`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.snowflake.parse_snowflake_stage_name.json).

<a id="op-d3310b544fb46969b011f373"></a>
## parse_snowflake_stage_name

`function` · `sqlparser::dialect::snowflake::parse_snowflake_stage_name` · sqlparser 0.62.0

```rust
fn parse_snowflake_stage_name(parser: &mut parser::Parser<'_>) -> Result<ast::ObjectName, parser::ParserError>
```

Source: `src/dialect/snowflake.rs:1287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parses a Snowflake stage name, which may start with `@` for internal stages.
Examples: `@mystage`, `@namespace.stage`, `schema.table`
