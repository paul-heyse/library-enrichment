# `parquet::schema::parser`

Crate `parquet` · 1 public items · structured records in [`model/parquet.schema.parser.json`](../model/parquet.schema.parser.json)

## parse_message_type

`function` · `parquet::schema::parser::parse_message_type`

```rust
fn parse_message_type(message_type: &str) -> errors::Result<schema::types::Type>
```

Parses message type as string into a Parquet [`Type`]
which, for example, could be used to extract individual columns. Returns Parquet
general error when parsing or validation fails.

---
