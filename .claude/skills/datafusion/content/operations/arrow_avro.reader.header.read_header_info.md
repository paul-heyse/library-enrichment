# `arrow_avro::reader::header::read_header_info`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.reader.header.read_header_info.json).

<a id="op-14db7ace3d315265a357dd69"></a>
## read_header_info

`function` · `arrow_avro::reader::header::read_header_info` · arrow-avro 59.3.0

```rust
fn read_header_info<R: BufRead>(reader: R) -> Result<HeaderInfo, errors::AvroError>
```

Source: `src/reader/header.rs:149`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Reads the Avro file header (magic, metadata, sync marker) from `reader`.

On success, returns the parsed [`HeaderInfo`](../operations/arrow_avro.reader.header.HeaderInfo.md#op-cfe8c3dfb2978eb03d7adba6) containing the header and its length in bytes.
