# `arrow_ipc::writer::write_message`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.writer.write_message.json).

<a id="op-3625269d74ae8dd5df992adf"></a>
## write_message

`function` · `arrow_ipc::writer::write_message` · arrow-ipc 59.3.0

```rust
fn write_message<W: Write>(writer: W, encoded: EncodedData, write_options: &IpcWriteOptions) -> Result<(usize, usize), ArrowError>
```

Source: `src/writer.rs:2205`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Write a message's IPC data and buffers, returning metadata and buffer data lengths written
