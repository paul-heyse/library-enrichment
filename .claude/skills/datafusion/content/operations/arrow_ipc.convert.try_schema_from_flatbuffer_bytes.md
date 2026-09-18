# `arrow_ipc::convert::try_schema_from_flatbuffer_bytes`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.convert.try_schema_from_flatbuffer_bytes.json).

<a id="op-93f44f3e7cf7ff32ca1c8a94"></a>
## try_schema_from_flatbuffer_bytes

`function` · `arrow_ipc::convert::try_schema_from_flatbuffer_bytes` · arrow-ipc 59.3.0

```rust
fn try_schema_from_flatbuffer_bytes(bytes: &[u8]) -> Result<Schema, ArrowError>
```

Source: `src/convert.rs:229`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try deserialize flat buffer format bytes into a schema
