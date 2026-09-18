# `arrow_csv::reader::Format`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_csv.reader.Format.json).

<a id="op-58a18762af38335f6661c4fd"></a>
## Format

`struct` · `arrow_csv::reader::Format` · arrow-csv 59.3.0

```rust
struct Format
```

Source: `src/reader/mod.rs:274`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

The format specification for the CSV file

<a id="op-1b1f06af6839be60373b29a7"></a>
## clone

`function` · `arrow_csv::reader::Format::clone` · arrow-csv 59.3.0

```rust
fn clone(&self) -> Format
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Format", "path": "Format"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 17], "end": [273, 22], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/reader/mod.rs:273`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e46d8064d3cf0cf638fb5a79"></a>
## default

`function` · `arrow_csv::reader::Format::default` · arrow-csv 59.3.0

```rust
fn default() -> Format
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Format", "path": "Format"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 24], "end": [273, 31], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/reader/mod.rs:273`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48d189d8b30f128e61b1a6ad"></a>
## fmt

`function` · `arrow_csv::reader::Format::fmt` · arrow-csv 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Format", "path": "Format"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 10], "end": [273, 15], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader/mod.rs:273`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab00716ad761917d6033efcd"></a>
## infer_schema

`function` · `arrow_csv::reader::Format::infer_schema` · arrow-csv 59.3.0

```rust
fn infer_schema<R: Read>(&self, reader: R, max_records: Option<usize>) -> Result<(Schema, usize), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Format", "path": "Format"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [453, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:360`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Infer schema of CSV records from the provided `reader`

If `max_records` is `None`, all records will be read, otherwise up to `max_records`
records are read to infer the schema

Returns inferred schema and number of records read

<a id="op-2297b0830a8ded0b50ceccca"></a>
## with_comment

`function` · `arrow_csv::reader::Format::with_comment` · arrow-csv 59.3.0

```rust
fn with_comment(self, comment: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Format", "path": "Format"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [453, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:332`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Specify a comment character, defaults to `None`

Lines starting with this character will be ignored

<a id="op-73332248115139cbcd50cffc"></a>
## with_delimiter

`function` · `arrow_csv::reader::Format::with_delimiter` · arrow-csv 59.3.0

```rust
fn with_delimiter(self, delimiter: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Format", "path": "Format"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [453, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:306`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Specify a custom delimiter character, defaults to comma `','`

<a id="op-32c040ee51f789f6193dbacf"></a>
## with_escape

`function` · `arrow_csv::reader::Format::with_escape` · arrow-csv 59.3.0

```rust
fn with_escape(self, escape: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Format", "path": "Format"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [453, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:312`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Specify an escape character, defaults to `None`

<a id="op-1c249e8202b8f6339a3e2180"></a>
## with_header

`function` · `arrow_csv::reader::Format::with_header` · arrow-csv 59.3.0

```rust
fn with_header(self, has_header: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Format", "path": "Format"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [453, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Specify whether the CSV file has a header, defaults to `false`

When `true`, the first row of the CSV file is treated as a header row

<a id="op-451b6aa02d4eaae5f53ce467"></a>
## with_header_validation

`function` · `arrow_csv::reader::Format::with_header_validation` · arrow-csv 59.3.0

```rust
fn with_header_validation(self, validate_header: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Format", "path": "Format"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [453, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:300`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Specify whether to validate the CSV header against the schema, defaults to `false`

When `true`, the first row gets validated against the schema before any data is read

Only applies when [`Self::with_header`](../operations/arrow_csv.reader.Format.md#op-1c249e8202b8f6339a3e2180) is set to `true`

<a id="op-8110bb739bc38f3c5e6c80e3"></a>
## with_null_regex

`function` · `arrow_csv::reader::Format::with_null_regex` · arrow-csv 59.3.0

```rust
fn with_null_regex(self, null_regex: Regex) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Format", "path": "Format"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [453, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:338`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Provide a regex to match null values, defaults to `^$`

<a id="op-392a350c0c8adffce970ea6e"></a>
## with_quote

`function` · `arrow_csv::reader::Format::with_quote` · arrow-csv 59.3.0

```rust
fn with_quote(self, quote: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Format", "path": "Format"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [453, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:318`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Specify a custom quote character, defaults to double quote `'"'`

<a id="op-8a38f362a0247d7d7f18c2a6"></a>
## with_terminator

`function` · `arrow_csv::reader::Format::with_terminator` · arrow-csv 59.3.0

```rust
fn with_terminator(self, terminator: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Format", "path": "Format"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [453, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:324`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Specify a custom terminator character, defaults to CRLF

<a id="op-9f4f5eef5fee2901f5b89d5e"></a>
## with_truncated_rows

`function` · `arrow_csv::reader::Format::with_truncated_rows` · arrow-csv 59.3.0

```rust
fn with_truncated_rows(self, allow: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Format", "path": "Format"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [453, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:349`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Whether to allow truncated rows when parsing.

By default this is set to `false` and will error if the CSV rows have different lengths.
When set to true then it will allow records with less than the expected number of columns
and fill the missing columns with nulls. If the record's schema is not nullable, then it
will still return an error.
