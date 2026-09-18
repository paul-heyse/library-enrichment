# `arrow_csv::reader::ReaderBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_csv.reader.ReaderBuilder.json).

<a id="op-743461f30ad5174b593cb0bd"></a>
## ReaderBuilder

`struct` · `arrow_csv::reader::ReaderBuilder` · arrow-csv 59.3.0

```rust
struct ReaderBuilder
```

Source: `src/reader/mod.rs:1152`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Builder for CSV [`Reader`](../operations/arrow_csv.reader.Reader.md#op-80f5b4a82073800a3487a573)s

<a id="op-126eae42af3ef8aa44eb0e83"></a>
## build

`function` · `arrow_csv::reader::ReaderBuilder::build` · arrow-csv 59.3.0

```rust
fn build<R: Read>(self, reader: R) -> Result<Reader<R>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1290`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Create a new `Reader` from a non-buffered reader

If `R: BufRead` consider using [`Self::build_buffered`](../operations/arrow_csv.reader.ReaderBuilder.md#op-592d511107d001c922017993) to avoid unnecessary additional
buffering, as internally this method wraps `reader` in [`std::io::BufReader`]

Unresolved upstream links (retained, not inferred): ``std::io::BufReader``.

<a id="op-592d511107d001c922017993"></a>
## build_buffered

`function` · `arrow_csv::reader::ReaderBuilder::build_buffered` · arrow-csv 59.3.0

```rust
fn build_buffered<R: BufRead>(self, reader: R) -> Result<BufReader<R>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1295`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Create a new `BufReader` from a buffered reader

<a id="op-f44352dbecb6433d6c1c5b21"></a>
## build_decoder

`function` · `arrow_csv::reader::ReaderBuilder::build_decoder` · arrow-csv 59.3.0

```rust
fn build_decoder(self) -> Decoder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1303`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Builds a decoder that can be used to decode CSV from an arbitrary byte stream

<a id="op-014d1df2485c4702cecdc6ef"></a>
## fmt

`function` · `arrow_csv::reader::ReaderBuilder::fmt` · arrow-csv 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1151, 10], "end": [1151, 15], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader/mod.rs:1151`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e7b52ef8dc522178742f158"></a>
## new

`function` · `arrow_csv::reader::ReaderBuilder::new` · arrow-csv 59.3.0

```rust
fn new(schema: SchemaRef) -> ReaderBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1190`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Create a new builder for configuring [`Reader`](../operations/arrow_csv.reader.Reader.md#op-80f5b4a82073800a3487a573) CSV parsing options.

To convert a builder into a reader, call [`ReaderBuilder::build`](../operations/arrow_csv.reader.ReaderBuilder.md#op-126eae42af3ef8aa44eb0e83). See
the [module-level documentation](crate::reader) for more details and examples.

# Example

```
# use arrow_csv::{Reader, ReaderBuilder};
# use std::fs::File;
# use std::io::Seek;
# use std::sync::Arc;
# use arrow_csv::reader::Format;
#
let mut file = File::open("test/data/uk_cities_with_headers.csv").unwrap();
// Infer the schema with the first 100 records
let (schema, _) = Format::default().infer_schema(&mut file, Some(100)).unwrap();
file.rewind().unwrap();

// create a builder
ReaderBuilder::new(Arc::new(schema)).build(file).unwrap();
```

<a id="op-adf1d07dd43b8fe0617cfeb4"></a>
## with_batch_size

`function` · `arrow_csv::reader::ReaderBuilder::with_batch_size` · arrow-csv 59.3.0

```rust
fn with_batch_size(self, batch_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1257`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the batch size (number of records to load at one time)

<a id="op-c89f8bba41774c2c85d8d3d7"></a>
## with_bounds

`function` · `arrow_csv::reader::ReaderBuilder::with_bounds` · arrow-csv 59.3.0

```rust
fn with_bounds(self, start: usize, end: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1264`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the bounds over which to scan the reader.
`start` and `end` are line numbers.

<a id="op-a67ec2202924f64c6bd41401"></a>
## with_comment

`function` · `arrow_csv::reader::ReaderBuilder::with_comment` · arrow-csv 59.3.0

```rust
fn with_comment(self, comment: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1245`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Provide a comment character, lines starting with this character will be ignored

<a id="op-c3e781bfee649ca2a26dcb2e"></a>
## with_delimiter

`function` · `arrow_csv::reader::ReaderBuilder::with_delimiter` · arrow-csv 59.3.0

```rust
fn with_delimiter(self, delimiter: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1221`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the CSV file's column delimiter as a byte character

<a id="op-b7dae6d209dfe798f701daff"></a>
## with_escape

`function` · `arrow_csv::reader::ReaderBuilder::with_escape` · arrow-csv 59.3.0

```rust
fn with_escape(self, escape: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1227`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the given character as the CSV file's escape character

<a id="op-96dd00bab02bbd7b7be8d3ac"></a>
## with_format

`function` · `arrow_csv::reader::ReaderBuilder::with_format` · arrow-csv 59.3.0

```rust
fn with_format(self, format: Format) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1215`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Overrides the [Format](../operations/arrow_csv.reader.Format.md#op-58a18762af38335f6661c4fd) of this [ReaderBuilder](../operations/arrow_csv.reader.ReaderBuilder.md#op-743461f30ad5174b593cb0bd)

<a id="op-944c135636129951e6d81aff"></a>
## with_header

`function` · `arrow_csv::reader::ReaderBuilder::with_header` · arrow-csv 59.3.0

```rust
fn with_header(self, has_header: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1201`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set whether the CSV file has a header

<a id="op-e17071273577d18fdf4eb972"></a>
## with_header_validation

`function` · `arrow_csv::reader::ReaderBuilder::with_header_validation` · arrow-csv 59.3.0

```rust
fn with_header_validation(self, validate_header: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1209`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set whether to validate the CSV header against the schema

This option only applies when [`Self::with_header`](../operations/arrow_csv.reader.ReaderBuilder.md#op-944c135636129951e6d81aff) is set to `true`, and defaults to `false`

<a id="op-f061058ba6e45c3a1c130b93"></a>
## with_null_regex

`function` · `arrow_csv::reader::ReaderBuilder::with_null_regex` · arrow-csv 59.3.0

```rust
fn with_null_regex(self, null_regex: Regex) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1251`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Provide a regex to match null values, defaults to `^$`

<a id="op-3f4642fea16ce01d08d6b8f7"></a>
## with_projection

`function` · `arrow_csv::reader::ReaderBuilder::with_projection` · arrow-csv 59.3.0

```rust
fn with_projection(self, projection: Vec<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1270`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the reader's column projection

<a id="op-83478d704ab5d5c22b4dcb03"></a>
## with_quote

`function` · `arrow_csv::reader::ReaderBuilder::with_quote` · arrow-csv 59.3.0

```rust
fn with_quote(self, quote: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1233`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Set the given character as the CSV file's quote character, by default it is double quote

<a id="op-fe1f4377d70aca73a3b0ce3f"></a>
## with_terminator

`function` · `arrow_csv::reader::ReaderBuilder::with_terminator` · arrow-csv 59.3.0

```rust
fn with_terminator(self, terminator: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1239`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Provide a custom terminator character, defaults to CRLF

<a id="op-0d12f1c0fa7d307d64e01ca6"></a>
## with_truncated_rows

`function` · `arrow_csv::reader::ReaderBuilder::with_truncated_rows` · arrow-csv 59.3.0

```rust
fn with_truncated_rows(self, allow: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1330, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:1281`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Whether to allow truncated rows when parsing.

By default this is set to `false` and will error if the CSV rows have different lengths.
When set to true then it will allow records with less than the expected number of columns
and fill the missing columns with nulls. If the record's schema is not nullable, then it
will still return an error.
