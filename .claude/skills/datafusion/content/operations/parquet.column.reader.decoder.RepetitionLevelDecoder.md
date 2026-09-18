# `parquet::column::reader::decoder::RepetitionLevelDecoder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.reader.decoder.RepetitionLevelDecoder.json).

<a id="op-4aae82cbb83706fd7a12b5f1"></a>
## RepetitionLevelDecoder

`trait` · `parquet::column::reader::decoder::RepetitionLevelDecoder` · parquet 59.3.0

```rust
trait RepetitionLevelDecoder: ColumnLevelDecoder
```

Source: `src/column/reader/decoder.rs:38`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dff3f9670b5997303977dbda"></a>
## flush_partial

`function` · `parquet::column::reader::decoder::RepetitionLevelDecoder::flush_partial` · parquet 59.3.0

```rust
fn flush_partial(&mut self) -> bool
```

Source: `src/column/reader/decoder.rs:65`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Flush any partially read or skipped record

<a id="op-e491a7a09e1a11520c3736b1"></a>
## read_rep_levels

`function` · `parquet::column::reader::decoder::RepetitionLevelDecoder::read_rep_levels` · parquet 59.3.0

```rust
fn read_rep_levels(&mut self, out: &mut Self::Buffer, num_records: usize, num_levels: usize) -> Result<(usize, usize)>
```

Source: `src/column/reader/decoder.rs:48`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Read up to `max_records` of repetition level data into `out` returning the number
of complete records and levels read

A record only ends when the data contains a subsequent repetition level of 0,
it is therefore left to the caller to delimit the final record in a column

# Panics

Implementations may panic if `range` overlaps with already written data

<a id="op-ede06d63b428bf03c8492d78"></a>
## skip_rep_levels

`function` · `parquet::column::reader::decoder::RepetitionLevelDecoder::skip_rep_levels` · parquet 59.3.0

```rust
fn skip_rep_levels(&mut self, num_records: usize, num_levels: usize) -> Result<(usize, usize)>
```

Source: `src/column/reader/decoder.rs:62`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Skips over up to `num_levels` repetition levels corresponding to `num_records` records,
where a record is delimited by a repetition level of 0

Returns the number of records skipped, and the number of levels skipped

A record only ends when the data contains a subsequent repetition level of 0,
it is therefore left to the caller to delimit the final record in a column
