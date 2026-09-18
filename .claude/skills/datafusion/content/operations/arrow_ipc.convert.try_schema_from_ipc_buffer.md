# `arrow_ipc::convert::try_schema_from_ipc_buffer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.convert.try_schema_from_ipc_buffer.json).

<a id="op-15cf69b8c3887175a16f3152"></a>
## try_schema_from_ipc_buffer

`function` · `arrow_ipc::convert::try_schema_from_ipc_buffer` · arrow-ipc 59.3.0

```rust
fn try_schema_from_ipc_buffer(buffer: &[u8]) -> Result<Schema, ArrowError>
```

Source: `src/convert.rs:246`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try deserialize the IPC format bytes into a schema
