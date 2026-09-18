# `parquet::column::writer::GenericColumnWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.writer.GenericColumnWriter.json).

<a id="op-e4f721b4650fb9a0c39a8a74"></a>
## GenericColumnWriter

`struct` · `parquet::column::writer::GenericColumnWriter` · parquet 59.3.0

```rust
struct GenericColumnWriter<'a, E: ColumnValueEncoder>
```

Source: `src/column/writer/mod.rs:442`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Generic column writer for a primitive Parquet column

<a id="op-22865d3e5922817c5524f95d"></a>
## close

`function` · `parquet::column::writer::GenericColumnWriter::close` · parquet 59.3.0

```rust
fn close(self) -> Result<ColumnCloseResult>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "E"}}], "constraints": []}}, "id": "parquet::column::writer::GenericColumnWriter", "path": "GenericColumnWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::writer::encoder::ColumnValueEncoder", "path": "ColumnValueEncoder"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 1], "end": [1694, 2], "filename": "src/column/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/writer/mod.rs:778`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Finalizes writes and closes the column writer.
Returns total bytes written, total rows written and column chunk metadata.

<a id="op-c51682910b6b6291edcc771a"></a>
## get_descriptor

`function` · `parquet::column::writer::GenericColumnWriter::get_descriptor` · parquet 59.3.0

```rust
fn get_descriptor(&self) -> &ColumnDescPtr
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "E"}}], "constraints": []}}, "id": "parquet::column::writer::GenericColumnWriter", "path": "GenericColumnWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::writer::encoder::ColumnValueEncoder", "path": "ColumnValueEncoder"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 1], "end": [1694, 2], "filename": "src/column/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/writer/mod.rs:772`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to a [`ColumnDescPtr`](../operations/parquet.schema.types.ColumnDescPtr.md#op-2935be8a415cad25a9c7574e)

<a id="op-7ccfd56e11b2f3a2f74f5625"></a>
## get_total_bytes_written

`function` · `parquet::column::writer::GenericColumnWriter::get_total_bytes_written` · parquet 59.3.0

```rust
fn get_total_bytes_written(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "E"}}], "constraints": []}}, "id": "parquet::column::writer::GenericColumnWriter", "path": "GenericColumnWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::writer::encoder::ColumnValueEncoder", "path": "ColumnValueEncoder"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 1], "end": [1694, 2], "filename": "src/column/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/writer/mod.rs:745`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns total number of bytes written by this column writer so far.
This value is also returned when column writer is closed.

Note: this value does not include any buffered data that has not
yet been flushed to a page.

<a id="op-eb3902f0d6d76ccccd80c05e"></a>
## get_total_rows_written

`function` · `parquet::column::writer::GenericColumnWriter::get_total_rows_written` · parquet 59.3.0

```rust
fn get_total_rows_written(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "E"}}], "constraints": []}}, "id": "parquet::column::writer::GenericColumnWriter", "path": "GenericColumnWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::writer::encoder::ColumnValueEncoder", "path": "ColumnValueEncoder"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 1], "end": [1694, 2], "filename": "src/column/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/writer/mod.rs:767`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns total number of rows written by this column writer so far.
This value is also returned when column writer is closed.

<a id="op-9fd044095102c2c705e968ca"></a>
## new

`function` · `parquet::column::writer::GenericColumnWriter::new` · parquet 59.3.0

```rust
fn new(descr: ColumnDescPtr, props: WriterPropertiesPtr, page_writer: Box<dyn PageWriter + 'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "E"}}], "constraints": []}}, "id": "parquet::column::writer::GenericColumnWriter", "path": "GenericColumnWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::writer::encoder::ColumnValueEncoder", "path": "ColumnValueEncoder"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 1], "end": [1694, 2], "filename": "src/column/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/writer/mod.rs:479`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a new instance of [`GenericColumnWriter`](../operations/parquet.column.writer.GenericColumnWriter.md#op-e4f721b4650fb9a0c39a8a74).

<a id="op-eeaafd8c8dd79b7551c4b38e"></a>
## write_batch

`function` · `parquet::column::writer::GenericColumnWriter::write_batch` · parquet 59.3.0

```rust
fn write_batch(&mut self, values: &E::Values, def_levels: Option<&[i16]>, rep_levels: Option<&[i16]>) -> Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "E"}}], "constraints": []}}, "id": "parquet::column::writer::GenericColumnWriter", "path": "GenericColumnWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::writer::encoder::ColumnValueEncoder", "path": "ColumnValueEncoder"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 1], "end": [1694, 2], "filename": "src/column/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/writer/mod.rs:676`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Writes batch of values, definition levels and repetition levels.
Returns number of values processed (written).

If definition and repetition levels are provided, we write fully those levels and
select how many values to write (this number will be returned), since number of
actual written values may be smaller than provided values.

If only values are provided, then all values are written and the length of
of the values buffer is returned.

Definition and/or repetition levels can be omitted, if values are
non-nullable and/or non-repeated.

<a id="op-0c57ca7a081a7e9f53dd88a4"></a>
## write_batch_with_statistics

`function` · `parquet::column::writer::GenericColumnWriter::write_batch_with_statistics` · parquet 59.3.0

```rust
fn write_batch_with_statistics(&mut self, values: &E::Values, def_levels: Option<&[i16]>, rep_levels: Option<&[i16]>, min: Option<&E::T>, max: Option<&E::T>, distinct_count: Option<u64>) -> Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "E"}}], "constraints": []}}, "id": "parquet::column::writer::GenericColumnWriter", "path": "GenericColumnWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::writer::encoder::ColumnValueEncoder", "path": "ColumnValueEncoder"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 1], "end": [1694, 2], "filename": "src/column/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/writer/mod.rs:700`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Writer may optionally provide pre-calculated statistics for use when computing
chunk-level statistics

NB: [`WriterProperties::statistics_enabled`](../operations/parquet.file.properties.WriterProperties.md#op-895c1c05fbde2ced95030c15) must be set to [`EnabledStatistics::Chunk`](../operations/parquet.file.properties.EnabledStatistics.md#op-de078fa8f540ac9d7d3949a2)
for these statistics to take effect. If [`EnabledStatistics::None`](../operations/parquet.file.properties.EnabledStatistics.md#op-a54aec7b98f8822fdcc64f83) they will be ignored,
and if [`EnabledStatistics::Page`](../operations/parquet.file.properties.EnabledStatistics.md#op-918e760bc6c733e101ef05af) the chunk statistics will instead be computed from the
computed page statistics
