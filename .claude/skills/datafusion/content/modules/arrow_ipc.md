# `arrow_ipc`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.json).

<a id="op-318e8c625105f63dd319fd7b"></a>
## arrow_ipc

`module` · `arrow_ipc` · arrow-ipc 59.3.0

```rust
mod arrow_ipc
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Support for the [Arrow IPC Format]

The Arrow IPC format defines how to read and write [`RecordBatch`](../operations/arrow_ipc.gen.Message.RecordBatch.md#op-6110063effc9fadb5bd35059)es to/from
a file or stream of bytes. This format can be used to serialize and deserialize
data to files and over the network.

There are two variants of the IPC format:
1. [IPC Streaming Format]: Supports streaming data sources, implemented by
   [StreamReader] and [StreamWriter]

2. [IPC File Format]: Supports random access, implemented by [FileReader] and
   [FileWriter].

See the [`reader`](../modules/arrow_ipc.reader.md#op-ef6a9482b890a795127b75ea) and [`writer`](../modules/arrow_ipc.writer.md#op-acf6a22ba3a5c39c22cad104) modules for more information.

[Arrow IPC Format]: https://arrow.apache.org/docs/format/Columnar.html#serialization-and-interprocess-communication-ipc
[IPC Streaming Format]: https://arrow.apache.org/docs/format/Columnar.html#ipc-streaming-format
[StreamReader]: reader::StreamReader
[StreamWriter]: writer::StreamWriter
[IPC File Format]: https://arrow.apache.org/docs/format/Columnar.html#ipc-file-format
[FileReader]: reader::FileReader
[FileWriter]: writer::FileWriter
