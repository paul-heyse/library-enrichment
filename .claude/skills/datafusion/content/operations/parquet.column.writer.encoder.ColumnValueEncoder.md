# `parquet::column::writer::encoder::ColumnValueEncoder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.writer.encoder.ColumnValueEncoder.json).

<a id="op-0971980704756d798f7c1343"></a>
## ColumnValueEncoder

`trait` · `parquet::column::writer::encoder::ColumnValueEncoder` · parquet 59.3.0

```rust
trait ColumnValueEncoder
```

Source: `src/column/writer/encoder.rs:73`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A generic encoder of [`ColumnValues`] to data and dictionary pages used by
[super::GenericColumnWriter`]

<a id="op-2b2ad423da7cd3b85c5eb30a"></a>
## T

`assoc_type` · `parquet::column::writer::encoder::ColumnValueEncoder::T` · parquet 59.3.0

```rust
T
```

Source: `src/column/writer/encoder.rs:77`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The underlying value type of [`Self::Values`]

Note: this avoids needing to fully qualify `<Self::Values as ColumnValues>::T`

<a id="op-adac07cd2adc2c8719c2366f"></a>
## Values

`assoc_type` · `parquet::column::writer::encoder::ColumnValueEncoder::Values` · parquet 59.3.0

```rust
Values
```

Source: `src/column/writer/encoder.rs:80`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The values encoded by this encoder

<a id="op-14c737e9d5d11d985c55e708"></a>
## compresses_against_previous_value

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::compresses_against_previous_value` · parquet 59.3.0

```rust
fn compresses_against_previous_value(&self) -> bool
```

Source: `src/column/writer/encoder.rs:152`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns true if the encoder compresses each value against the value
immediately before it, within the current page.

For such encodings a page boundary is not free: flushing discards the
previous value, so the first value of the next page is stored in full.
[`GenericColumnWriter::should_add_data_page`] uses this to exempt a
page's mandatory first value from the data page byte limit.

Per encoding:
* `DELTA_BYTE_ARRAY`: true. Each value is stored as the length of the
  prefix it shares with its predecessor plus the remaining suffix.
* Everything else: false, the default. `PLAIN` and
  `DELTA_LENGTH_BYTE_ARRAY` store a value at the same cost wherever it
  lands, and a dictionary outlives the pages that index into it, so no
  page boundary makes a value more expensive.

[`GenericColumnWriter::should_add_data_page`]: crate::column::writer::GenericColumnWriter::should_add_data_page

<a id="op-8f466917df93e95fb07b5d16"></a>
## count_values_within_byte_budget

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::count_values_within_byte_budget` · parquet 59.3.0

```rust
fn count_values_within_byte_budget(_values: &Self::Values, _offset: usize, _len: usize, _byte_budget: usize) -> Option<usize>
```

Source: `src/column/writer/encoder.rs:109`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the largest `k` such that the first `k` values in
`values[offset..offset + len]` encode to at most `byte_budget`
bytes — i.e. how many values fit in a single page byte budget.

Returns `len` if every value fits. Returns at least 1 if a single
value alone exceeds the budget, matching parquet's "at least one
value per data page" rule.

`None` means "no cheap estimate available"; the caller stays on
the batched fast path and lets the post-write
`should_add_data_page` check handle bounding.

Implementations should short-circuit aggressively: the typical
case is "everything fits, return `len`", and the next-most-common
case is "one wide value, return 1." The variable-width walk only
needs to be precise when the chunk is genuinely near the budget.

<a id="op-58350beb3e737d7bb9185ef5"></a>
## count_values_within_byte_budget_gather

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::count_values_within_byte_budget_gather` · parquet 59.3.0

```rust
fn count_values_within_byte_budget_gather(_values: &Self::Values, _indices: &[usize], _byte_budget: usize) -> Option<usize>
```

Source: `src/column/writer/encoder.rs:121`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

As [`Self::count_values_within_byte_budget`] but using gather
`indices` rather than a contiguous range. Returns the number of
`indices` that fit, not the maximum index value.

<a id="op-777a9b95feefeb7554231df4"></a>
## estimated_data_page_size

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::estimated_data_page_size` · parquet 59.3.0

```rust
fn estimated_data_page_size(&self) -> usize
```

Source: `src/column/writer/encoder.rs:167`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns an estimate of the encoded data page size in bytes

This should include:
<already_written_encoded_byte_size> + <estimated_encoded_size_of_unflushed_bytes>

<a id="op-4b0099909b7206e051756648"></a>
## estimated_dict_page_size

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::estimated_dict_page_size` · parquet 59.3.0

```rust
fn estimated_dict_page_size(&self) -> Option<usize>
```

Source: `src/column/writer/encoder.rs:161`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns an estimate of the encoded size of dictionary page size in bytes, or `None` if no dictionary

<a id="op-49e951b3b6d7bc37aa4db858"></a>
## estimated_memory_size

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::estimated_memory_size` · parquet 59.3.0

```rust
fn estimated_memory_size(&self) -> usize
```

Source: `src/column/writer/encoder.rs:158`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the estimated total memory usage of the encoder


<a id="op-8b7044ee84ab27b4345cc44d"></a>
## flush_bloom_filter

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::flush_bloom_filter` · parquet 59.3.0

```rust
fn flush_bloom_filter(&mut self) -> Option<Sbbf>
```

Source: `src/column/writer/encoder.rs:182`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Flushes bloom filter if enabled and returns it, otherwise returns `None`. Subsequent writes
will *not* be tracked by the bloom filter as it is empty since. This should be called once
near the end of encoding.

<a id="op-e72b15b2ccf608bc47975ec6"></a>
## flush_data_page

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::flush_data_page` · parquet 59.3.0

```rust
fn flush_data_page(&mut self) -> Result<DataPageValues<Self::T>>
```

Source: `src/column/writer/encoder.rs:177`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Flush the next data page for this column chunk

<a id="op-4cf224d4206e757ba20c689e"></a>
## flush_dict_page

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::flush_dict_page` · parquet 59.3.0

```rust
fn flush_dict_page(&mut self) -> Result<Option<DictionaryPage>>
```

Source: `src/column/writer/encoder.rs:174`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Flush the dictionary page for this column chunk if any. Any subsequent calls to
[`Self::write`] will not be dictionary encoded

Note: [`Self::flush_data_page`] must be called first, as this will error if there
are any pending page values

<a id="op-bc9b80b7877a330bb1fc1198"></a>
## flush_geospatial_statistics

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::flush_geospatial_statistics` · parquet 59.3.0

```rust
fn flush_geospatial_statistics(&mut self) -> Option<Box<GeospatialStatistics>>
```

Source: `src/column/writer/encoder.rs:186`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Computes [`GeospatialStatistics`], if any, and resets internal state such that any internal
accumulator is prepared to accumulate statistics for the next column chunk.

<a id="op-c8344f5580b8876eb2c87d2b"></a>
## has_dictionary

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::has_dictionary` · parquet 59.3.0

```rust
fn has_dictionary(&self) -> bool
```

Source: `src/column/writer/encoder.rs:133`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns true if this encoder has a dictionary page

<a id="op-dde78124e8031341c363dfcb"></a>
## num_values

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::num_values` · parquet 59.3.0

```rust
fn num_values(&self) -> usize
```

Source: `src/column/writer/encoder.rs:130`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of buffered values

<a id="op-d0e19c33ed005b1fcab36a44"></a>
## try_new

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::try_new` · parquet 59.3.0

```rust
fn try_new(descr: &ColumnDescPtr, props: &WriterProperties) -> Result<Self> where Self: Sized
```

Source: `src/column/writer/encoder.rs:83`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ColumnValueEncoder`]

<a id="op-d02afa6b657076c2c65cc552"></a>
## write

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::write` · parquet 59.3.0

```rust
fn write(&mut self, values: &Self::Values, offset: usize, len: usize) -> Result<()>
```

Source: `src/column/writer/encoder.rs:88`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Write the corresponding values to this [`ColumnValueEncoder`]

<a id="op-397680ed43ac3c955829b77f"></a>
## write_gather

`function` · `parquet::column::writer::encoder::ColumnValueEncoder::write_gather` · parquet 59.3.0

```rust
fn write_gather(&mut self, values: &Self::Values, indices: &[usize]) -> Result<()>
```

Source: `src/column/writer/encoder.rs:91`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Write the values at the indexes in `indices` to this [`ColumnValueEncoder`]
