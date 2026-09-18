# `arrow_avro::reader::async_reader::builder::read_header_info`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.reader.async_reader.builder.read_header_info.json).

<a id="op-89ce7b16ee6258b7a60337df"></a>
## read_header_info

`function` · `arrow_avro::reader::async_reader::builder::read_header_info` · arrow-avro 59.3.0

```rust
async fn read_header_info<R>(reader: &mut R, file_size: u64, header_size_hint: Option<u64>) -> Result<reader::header::HeaderInfo, errors::AvroError> where R: AsyncFileReader
```

Source: `src/reader/async_reader/builder.rs:125`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Reads the Avro file header (magic, metadata, sync marker) asynchronously from `reader`.

On success, returns the parsed [`HeaderInfo`](../operations/arrow_avro.reader.header.HeaderInfo.md#op-cfe8c3dfb2978eb03d7adba6) containing the header and its length in bytes.
