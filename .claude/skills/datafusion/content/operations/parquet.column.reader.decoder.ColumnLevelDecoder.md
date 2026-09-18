# `parquet::column::reader::decoder::ColumnLevelDecoder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.reader.decoder.ColumnLevelDecoder.json).

<a id="op-e3659f95285ffa6d4c245a36"></a>
## ColumnLevelDecoder

`trait` · `parquet::column::reader::decoder::ColumnLevelDecoder` · parquet 59.3.0

```rust
trait ColumnLevelDecoder
```

Source: `src/column/reader/decoder.rs:31`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Decodes level data

<a id="op-1ff289ddbb1ee6ad5e6aa679"></a>
## Buffer

`assoc_type` · `parquet::column::reader::decoder::ColumnLevelDecoder::Buffer` · parquet 59.3.0

```rust
Buffer
```

Source: `src/column/reader/decoder.rs:32`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ea39646bc8e0e53c10971e3"></a>
## set_data

`function` · `parquet::column::reader::decoder::ColumnLevelDecoder::set_data` · parquet 59.3.0

```rust
fn set_data(&mut self, encoding: Encoding, data: Bytes) -> Result<()>
```

Source: `src/column/reader/decoder.rs:35`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set data for this [`ColumnLevelDecoder`]
