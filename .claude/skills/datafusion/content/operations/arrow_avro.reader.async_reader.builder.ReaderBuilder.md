# `arrow_avro::reader::async_reader::builder::ReaderBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.reader.async_reader.builder.ReaderBuilder.json).

<a id="op-08602347b73f764e50afbd19"></a>
## ReaderBuilder

`struct` · `arrow_avro::reader::async_reader::builder::ReaderBuilder` · arrow-avro 59.3.0

```rust
struct ReaderBuilder<R>
```

Source: `src/reader/async_reader/builder.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Builder for an asynchronous Avro file reader.

<a id="op-fa371dd8951c5b59e24291d4"></a>
## build_with_header

`function` · `arrow_avro::reader::async_reader::builder::ReaderBuilder::build_with_header` · arrow-avro 59.3.0

```rust
fn build_with_header(self, header_info: HeaderInfo) -> Result<AsyncAvroFileReader<R>, AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::builder::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader", "path": "AsyncFileReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [296, 2], "filename": "src/reader/async_reader/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/builder.rs:209`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Build the asynchronous Avro reader with the provided header.

This allows initializing the reader with pre-parsed header information.
Note that this method is not async because it does not need to perform any I/O operations.

Note: Any `header_size_hint` set via [`Self::with_header_size_hint`](../operations/arrow_avro.reader.async_reader.builder.ReaderBuilder.md#op-9c74b6d400951f073b8e4c1e) is not used
when building with a pre-parsed header, since no header fetching occurs.

<a id="op-2d2d30e37d02f20bdc8e5fe5"></a>
## try_build

`function` · `arrow_avro::reader::async_reader::builder::ReaderBuilder::try_build` · arrow-avro 59.3.0

```rust
async fn try_build(self) -> Result<AsyncAvroFileReader<R>, AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::builder::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_avro::reader::async_reader::async_file_reader::AsyncFileReader", "path": "AsyncFileReader"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [296, 2], "filename": "src/reader/async_reader/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/builder.rs:189`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Build the asynchronous Avro reader with the provided parameters.
This reads the header first to initialize the reader state.

<a id="op-9c74b6d400951f073b8e4c1e"></a>
## with_header_size_hint

`function` · `arrow_avro::reader::async_reader::builder::ReaderBuilder::with_header_size_hint` · arrow-avro 59.3.0

```rust
fn with_header_size_hint(self, hint: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::builder::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [120, 2], "filename": "src/reader/async_reader/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/builder.rs:93`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Provide a hint for the expected size of the Avro header in bytes.
This can help optimize the initial read operation when fetching the header.

<a id="op-aace2b056535f439a7931281"></a>
## with_projection

`function` · `arrow_avro::reader::async_reader::builder::ReaderBuilder::with_projection` · arrow-avro 59.3.0

```rust
fn with_projection(self, projection: Vec<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::builder::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [120, 2], "filename": "src/reader/async_reader/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/builder.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Specify a projection of column indices to read from the Avro file.
This can help optimize reading by only fetching the necessary columns.

<a id="op-0df973bf519acc8792acf553"></a>
## with_range

`function` · `arrow_avro::reader::async_reader::builder::ReaderBuilder::with_range` · arrow-avro 59.3.0

```rust
fn with_range(self, range: Range<u64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::builder::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [120, 2], "filename": "src/reader/async_reader/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/builder.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Specify a byte range to read from the Avro file.
If this is provided, the reader will read all the blocks within the specified range,
if the range ends mid-block, it will attempt to fetch the remaining bytes to complete the block,
but no further blocks will be read.
If this is omitted, the full file will be read.

<a id="op-c4ed198fe179a4a6fe959ecc"></a>
## with_reader_schema

`function` · `arrow_avro::reader::async_reader::builder::ReaderBuilder::with_reader_schema` · arrow-avro 59.3.0

```rust
fn with_reader_schema(self, reader_schema: AvroSchema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::builder::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [120, 2], "filename": "src/reader/async_reader/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/builder.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Specify a reader schema to use when reading the Avro file.
This can be useful to project specific columns or handle schema evolution.
If this is not provided, the schema will be derived from the Arrow schema provided.

<a id="op-e62a27b14f4131a791c697f3"></a>
## with_strict_mode

`function` · `arrow_avro::reader::async_reader::builder::ReaderBuilder::with_strict_mode` · arrow-avro 59.3.0

```rust
fn with_strict_mode(self, strict_mode: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::builder::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [120, 2], "filename": "src/reader/async_reader/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/builder.rs:106`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Enable strict mode for schema validation and data reading.

<a id="op-456a63a1d870818f6202fa45"></a>
## with_tz

`function` · `arrow_avro::reader::async_reader::builder::ReaderBuilder::with_tz` · arrow-avro 59.3.0

```rust
fn with_tz(self, tz: Tz) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::builder::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [120, 2], "filename": "src/reader/async_reader/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/builder.rs:116`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Sets the timezone representation for Avro timestamp fields.

The default is `Tz::OffsetZero`, meaning the "+00:00" time zone ID.

<a id="op-f82359f27a9586198db96ce3"></a>
## with_utf8_view

`function` · `arrow_avro::reader::async_reader::builder::ReaderBuilder::with_utf8_view` · arrow-avro 59.3.0

```rust
fn with_utf8_view(self, utf8_view: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_avro::reader::async_reader::builder::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [120, 2], "filename": "src/reader/async_reader/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/async_reader/builder.rs:101`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Enable usage of Utf8View types when reading string data.
