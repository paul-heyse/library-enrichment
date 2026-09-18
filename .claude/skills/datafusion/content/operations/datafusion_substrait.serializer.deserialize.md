# `datafusion_substrait::serializer::deserialize`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.serializer.deserialize.json).

<a id="op-a13ea19db24aee87064a894c"></a>
## deserialize

`function` · `datafusion_substrait::serializer::deserialize` · datafusion-substrait 55.1.0

```rust
async fn deserialize(path: impl AsRef<std::path::Path>) -> datafusion::error::Result<Box<substrait::proto::Plan>>
```

Source: `src/serializer.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Reads the file at `path` and deserializes a plan from the bytes.
