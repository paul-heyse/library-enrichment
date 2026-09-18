# `datafusion_substrait::serializer::deserialize_bytes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.serializer.deserialize_bytes.json).

<a id="op-89a736b0ffc708e34045936e"></a>
## deserialize_bytes

`function` · `datafusion_substrait::serializer::deserialize_bytes` · datafusion-substrait 55.1.0

```rust
fn deserialize_bytes(proto_bytes: &[u8]) -> datafusion::error::Result<Box<substrait::proto::Plan>>
```

Source: `src/serializer.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Deserializes a plan from the bytes.
