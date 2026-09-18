# `arrow_ipc::reader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.reader.json).

<a id="op-ef6a9482b890a795127b75ea"></a>
## reader

`module` · `arrow_ipc::reader` · arrow-ipc 59.3.0

```rust
mod reader
```

Source: `src/reader.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Arrow IPC File and Stream Readers

# Notes

The [`FileReader`](../operations/arrow_ipc.reader.FileReader.md#op-637d6dc146e8bf36b6e304bf) and [`StreamReader`](../operations/arrow_ipc.reader.StreamReader.md#op-e303cfe08d702b62a9f2f221) have similar interfaces,
however the [`FileReader`](../operations/arrow_ipc.reader.FileReader.md#op-637d6dc146e8bf36b6e304bf) expects a reader that supports [`Seek`]ing

[`Seek`]: std::io::Seek

Unresolved upstream links (retained, not inferred): `std::io::Seek`.
