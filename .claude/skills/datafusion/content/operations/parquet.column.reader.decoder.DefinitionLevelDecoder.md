# `parquet::column::reader::decoder::DefinitionLevelDecoder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.reader.decoder.DefinitionLevelDecoder.json).

<a id="op-0df548266ca6472a15d6a2d2"></a>
## DefinitionLevelDecoder

`trait` · `parquet::column::reader::decoder::DefinitionLevelDecoder` · parquet 59.3.0

```rust
trait DefinitionLevelDecoder: ColumnLevelDecoder
```

Source: `src/column/reader/decoder.rs:68`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aafda37fd794c63b712e1a99"></a>
## read_def_levels

`function` · `parquet::column::reader::decoder::DefinitionLevelDecoder::read_def_levels` · parquet 59.3.0

```rust
fn read_def_levels(&mut self, out: &mut Self::Buffer, num_levels: usize) -> Result<(usize, usize)>
```

Source: `src/column/reader/decoder.rs:76`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Read up to `num_levels` definition levels into `out`.

Returns the number of values read, and the number of levels read.

# Panics

Implementations may panic if `range` overlaps with already written data

<a id="op-ad9c23696478f749fc80f608"></a>
## skip_def_levels

`function` · `parquet::column::reader::decoder::DefinitionLevelDecoder::skip_def_levels` · parquet 59.3.0

```rust
fn skip_def_levels(&mut self, num_levels: usize) -> Result<(usize, usize)>
```

Source: `src/column/reader/decoder.rs:85`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Skips over `num_levels` definition levels.

Returns the number of values skipped, and the number of levels skipped.
