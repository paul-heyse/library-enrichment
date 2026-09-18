# `parquet::column::reader::decoder::ColumnValueDecoder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.reader.decoder.ColumnValueDecoder.json).

<a id="op-1ae380992cd396d6f3c1d142"></a>
## ColumnValueDecoder

`trait` · `parquet::column::reader::decoder::ColumnValueDecoder` · parquet 59.3.0

```rust
trait ColumnValueDecoder
```

Source: `src/column/reader/decoder.rs:89`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Decodes value data

<a id="op-ab40603ba45b004f6c1f3177"></a>
## Buffer

`assoc_type` · `parquet::column::reader::decoder::ColumnValueDecoder::Buffer` · parquet 59.3.0

```rust
Buffer
```

Source: `src/column/reader/decoder.rs:90`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a10bd80b0f8e8849ec988093"></a>
## new

`function` · `parquet::column::reader::decoder::ColumnValueDecoder::new` · parquet 59.3.0

```rust
fn new(col: &ColumnDescPtr) -> Self
```

Source: `src/column/reader/decoder.rs:93`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ColumnValueDecoder`]

<a id="op-0bcc0eea6223f2982b3e943c"></a>
## read

`function` · `parquet::column::reader::decoder::ColumnValueDecoder::read` · parquet 59.3.0

```rust
fn read(&mut self, out: &mut Self::Buffer, num_values: usize) -> Result<usize>
```

Source: `src/column/reader/decoder.rs:129`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Read up to `num_values` values into `out`

# Panics

Implementations may panic if `range` overlaps with already written data


<a id="op-78926c14c62d7812afd087b0"></a>
## set_data

`function` · `parquet::column::reader::decoder::ColumnValueDecoder::set_data` · parquet 59.3.0

```rust
fn set_data(&mut self, encoding: Encoding, data: Bytes, num_levels: usize, num_values: Option<usize>) -> Result<()>
```

Source: `src/column/reader/decoder.rs:115`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the current data page

- `encoding` - the encoding of the page
- `data` - a point to the page's uncompressed value data
- `num_levels` - the number of levels contained within the page, i.e. values including nulls
- `num_values` - the number of non-null values contained within the page (V2 page only)

Note: data encoded with [`Encoding::RLE`] may not know its exact length, as the final
run may be zero-padded. As such if `num_values` is not provided (i.e. `None`),
subsequent calls to `ColumnValueDecoder::read` may yield more values than
non-null definition levels within the page

<a id="op-f28fba80b9618e82d78cb65d"></a>
## set_dict

`function` · `parquet::column::reader::decoder::ColumnValueDecoder::set_dict` · parquet 59.3.0

```rust
fn set_dict(&mut self, buf: Bytes, num_values: u32, encoding: Encoding, is_sorted: bool) -> Result<()>
```

Source: `src/column/reader/decoder.rs:96`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the current dictionary page

<a id="op-4e2adfaf51be3ac6490a4958"></a>
## skip_values

`function` · `parquet::column::reader::decoder::ColumnValueDecoder::skip_values` · parquet 59.3.0

```rust
fn skip_values(&mut self, num_values: usize) -> Result<usize>
```

Source: `src/column/reader/decoder.rs:134`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Skips over `num_values` values

Returns the number of values skipped
