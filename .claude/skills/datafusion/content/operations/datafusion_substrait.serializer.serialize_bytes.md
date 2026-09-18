# `datafusion_substrait::serializer::serialize_bytes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.serializer.serialize_bytes.json).

<a id="op-b8e9bd5fab592b9bfb558819"></a>
## serialize_bytes

`function` · `datafusion_substrait::serializer::serialize_bytes` · datafusion-substrait 55.1.0

```rust
async fn serialize_bytes(sql: &str, ctx: &SessionContext) -> datafusion::error::Result<Vec<u8>>
```

Source: `src/serializer.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Plans a sql and serializes the generated logical plan to bytes.
