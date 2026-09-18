# `arrow_avro::reader::async_reader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.reader.async_reader.json).

<a id="op-4901ddb78e53337623d7c64f"></a>
## async_reader

`module` · `arrow_avro::reader::async_reader` · arrow-avro 59.3.0

```rust
mod async_reader
```

Source: `src/reader/async_reader/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Asynchronous implementation of Avro file reader.

This module provides [`AsyncAvroFileReader`](../operations/arrow_avro.reader.async_reader.AsyncAvroFileReader.md#op-1556b0e38a5fc44a0cc6248c), which supports reading and decoding
the Avro OCF format from any source that implements [`AsyncFileReader`](../operations/arrow_avro.reader.async_reader.async_file_reader.AsyncFileReader.md#op-b37e46224b914587014a7456).
