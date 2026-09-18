# `parquet::column::reader::GenericColumnReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.reader.GenericColumnReader.json).

<a id="op-363aa89c7f845d1a210399f7"></a>
## GenericColumnReader

`struct` · `parquet::column::reader::GenericColumnReader` · parquet 59.3.0

```rust
struct GenericColumnReader<R, D, V>
```

Source: `src/column/reader.rs:114`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Reads data for a given column chunk, using the provided decoders:

- R: `ColumnLevelDecoder` used to decode repetition levels
- D: `ColumnLevelDecoder` used to decode definition levels
- V: `ColumnValueDecoder` used to decode value data

<a id="op-26b1a3d17bb18870d3acb689"></a>
## new

`function` · `parquet::column::reader::GenericColumnReader::new` · parquet 59.3.0

```rust
fn new(descr: ColumnDescPtr, page_reader: Box<dyn PageReader>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::column::reader::decoder::RepetitionLevelDecoderImpl", "path": "crate::column::reader::decoder::RepetitionLevelDecoderImpl"}}}, {"type": {"resolved_path": {"args": null, "id": "parquet::column::reader::decoder::DefinitionLevelDecoderImpl", "path": "crate::column::reader::decoder::DefinitionLevelDecoderImpl"}}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "parquet::column::reader::GenericColumnReader", "path": "GenericColumnReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::reader::decoder::ColumnValueDecoder", "path": "ColumnValueDecoder"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [139, 1], "end": [161, 2], "filename": "src/column/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/reader.rs:144`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new column reader based on column descriptor and page reader.

<a id="op-62c308f4e6620e2058d86959"></a>
## read_records

`function` · `parquet::column::reader::GenericColumnReader::read_records` · parquet 59.3.0

```rust
fn read_records(&mut self, max_records: usize, def_levels: Option<&mut D::Buffer>, rep_levels: Option<&mut R::Buffer>, values: &mut V::Buffer) -> Result<(usize, usize, usize)>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "D"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "parquet::column::reader::GenericColumnReader", "path": "GenericColumnReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "D"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::reader::decoder::RepetitionLevelDecoder", "path": "RepetitionLevelDecoder"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::reader::decoder::DefinitionLevelDecoder", "path": "DefinitionLevelDecoder"}}}], "generic_params": [], "type": {"generic": "D"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::reader::decoder::ColumnValueDecoder", "path": "ColumnValueDecoder"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [163, 1], "end": [586, 2], "filename": "src/column/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/reader.rs:202`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Read up to `max_records` whole records, returning the number of complete
records, non-null values and levels decoded. All levels for a given record
will be read, i.e. the next repetition level, if any, will be 0

If the max definition level is 0, `def_levels` will be ignored and the number of records,
non-null values and levels decoded will all be equal, otherwise `def_levels` will be
populated with the number of levels read, with an error returned if it is `None`.

If the max repetition level is 0, `rep_levels` will be ignored and the number of records
and levels decoded will both be equal, otherwise `rep_levels` will be populated with
the number of levels read, with an error returned if it is `None`.

`values` will be contiguously populated with the non-null values. Note that if the column
is not required, this may be less than either `max_records` or the number of levels read

<a id="op-a4f5626a3e64e0bf7f25dd3e"></a>
## skip_records

`function` · `parquet::column::reader::GenericColumnReader::skip_records` · parquet 59.3.0

```rust
fn skip_records(&mut self, num_records: usize) -> Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}, {"type": {"generic": "D"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "parquet::column::reader::GenericColumnReader", "path": "GenericColumnReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "D"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::reader::decoder::RepetitionLevelDecoder", "path": "RepetitionLevelDecoder"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::reader::decoder::DefinitionLevelDecoder", "path": "DefinitionLevelDecoder"}}}], "generic_params": [], "type": {"generic": "D"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "parquet::column::reader::decoder::ColumnValueDecoder", "path": "ColumnValueDecoder"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [163, 1], "end": [586, 2], "filename": "src/column/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/reader.rs:312`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Skips over `num_records` records, where records are delimited by repetition levels of 0

# Returns

Returns the number of records skipped
