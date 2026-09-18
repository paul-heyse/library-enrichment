# `parquet::schema::parser::parse_message_type`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.parser.parse_message_type.json).

<a id="op-07e1c86fb42f88cd9d546550"></a>
## parse_message_type

`function` · `parquet::schema::parser::parse_message_type` · parquet 59.3.0

```rust
fn parse_message_type(message_type: &str) -> errors::Result<schema::types::Type>
```

Source: `src/schema/parser.rs:54`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parses message type as string into a Parquet [`Type`](../operations/parquet.schema.types.Type.md#op-e61bef0051911d1f490c45fc)
which, for example, could be used to extract individual columns. Returns Parquet
general error when parsing or validation fails.
