# `arrow_ipc::reader::read_footer_length`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.reader.read_footer_length.json).

<a id="op-efc80a5db0602257772de716"></a>
## read_footer_length

`function` · `arrow_ipc::reader::read_footer_length` · arrow-ipc 59.3.0

```rust
fn read_footer_length(buf: [u8; 10]) -> Result<usize, ArrowError>
```

Source: `src/reader.rs:942`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Read the footer length from the last 10 bytes of an Arrow IPC file

Expects a 4 byte footer length followed by `b"ARROW1"`
