# `arrow_ipc::writer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.writer.json).

<a id="op-acf6a22ba3a5c39c22cad104"></a>
## writer

`module` · `arrow_ipc::writer` · arrow-ipc 59.3.0

```rust
mod writer
```

Source: `src/writer.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Arrow IPC File and Stream Writers

# Notes

[`FileWriter`](../operations/arrow_ipc.writer.FileWriter.md#op-de36feaee6f6c9a3aec5da12) and [`StreamWriter`](../operations/arrow_ipc.writer.StreamWriter.md#op-b808ea0aac7879e19d25b939) have similar interfaces,
however the [`FileWriter`](../operations/arrow_ipc.writer.FileWriter.md#op-de36feaee6f6c9a3aec5da12) expects a reader that supports [`Seek`]ing

[`Seek`]: std::io::Seek

Unresolved upstream links (retained, not inferred): `std::io::Seek`.
