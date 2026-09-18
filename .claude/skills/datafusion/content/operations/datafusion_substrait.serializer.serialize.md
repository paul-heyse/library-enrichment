# `datafusion_substrait::serializer::serialize`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.serializer.serialize.json).

<a id="op-dbc26d207bdf8cebe0f71ba4"></a>
## serialize

`function` · `datafusion_substrait::serializer::serialize` · datafusion-substrait 55.1.0

```rust
async fn serialize(sql: &str, ctx: &SessionContext, path: impl AsRef<std::path::Path>) -> datafusion::error::Result<()>
```

Source: `src/serializer.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Plans a sql and serializes the generated logical plan to bytes.
The bytes are then written into a file at `path`.

Returns an error if the file already exists.
