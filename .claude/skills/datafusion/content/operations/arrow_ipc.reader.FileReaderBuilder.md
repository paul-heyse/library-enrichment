# `arrow_ipc::reader::FileReaderBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.reader.FileReaderBuilder.json).

<a id="op-de18798ea04e62f8cadce7ae"></a>
## FileReaderBuilder

`struct` · `arrow_ipc::reader::FileReaderBuilder` · arrow-ipc 59.3.0

```rust
struct FileReaderBuilder
```

Source: `src/reader.rs:1154`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Build an Arrow [`FileReader`](../operations/arrow_ipc.reader.FileReader.md#op-637d6dc146e8bf36b6e304bf) with custom options.

<a id="op-b250b54c6e8886386c7a7b7c"></a>
## build

`function` · `arrow_ipc::reader::FileReaderBuilder::build` · arrow-ipc 59.3.0

```rust
fn build<R: Read + Seek>(self, reader: R) -> Result<FileReader<R>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileReaderBuilder", "path": "FileReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1174, 1], "end": [1292, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1223`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Build [`FileReader`](../operations/arrow_ipc.reader.FileReader.md#op-637d6dc146e8bf36b6e304bf) with given reader.

<a id="op-e736ea8019b7cf81f4891ea2"></a>
## default

`function` · `arrow_ipc::reader::FileReaderBuilder::default` · arrow-ipc 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileReaderBuilder", "path": "FileReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1163, 1], "end": [1172, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/reader.rs:1164`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b0a81688c2199524827bcbd"></a>
## fmt

`function` · `arrow_ipc::reader::FileReaderBuilder::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileReaderBuilder", "path": "FileReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1153, 10], "end": [1153, 15], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader.rs:1153`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1336f9871aa09c6fef5e1615"></a>
## new

`function` · `arrow_ipc::reader::FileReaderBuilder::new` · arrow-ipc 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileReaderBuilder", "path": "FileReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1174, 1], "end": [1292, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1178`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Options for creating a new [`FileReader`](../operations/arrow_ipc.reader.FileReader.md#op-637d6dc146e8bf36b6e304bf).

To convert a builder into a reader, call [`FileReaderBuilder::build`](../operations/arrow_ipc.reader.FileReaderBuilder.md#op-b250b54c6e8886386c7a7b7c).

<a id="op-f9888fce5765d9a5ac62ccbe"></a>
## with_max_footer_fb_depth

`function` · `arrow_ipc::reader::FileReaderBuilder::with_max_footer_fb_depth` · arrow-ipc 59.3.0

```rust
fn with_max_footer_fb_depth(self, max_footer_fb_depth: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileReaderBuilder", "path": "FileReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1174, 1], "end": [1292, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1217`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Flatbuffers option for parsing the footer. Controls the max depth for schemas with
nested fields parsed from the footer.

By default this is set to `64` which roughly translates to a schema with
a field nested 60 levels down through other struct fields.

This default limit is enforced to protect against malicious files with a extremely
deep flatbuffer structure which could cause a denial of service attack.

If you need to ingest a trusted file with a deeply nested field and are facing the
error `"Unable to get root as footer: DepthLimitReached"` then increase this
parameter as necessary.

<a id="op-b8f06387d6747dd0e5637038"></a>
## with_max_footer_fb_tables

`function` · `arrow_ipc::reader::FileReaderBuilder::with_max_footer_fb_tables` · arrow-ipc 59.3.0

```rust
fn with_max_footer_fb_tables(self, max_footer_fb_tables: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileReaderBuilder", "path": "FileReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1174, 1], "end": [1292, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1200`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Flatbuffers option for parsing the footer. Controls the max number of fields and
metadata key-value pairs that can be parsed from the schema of the footer.

By default this is set to `1_000_000` which roughly translates to a schema with
no metadata key-value pairs but 499,999 fields.

This default limit is enforced to protect against malicious files with a massive
amount of flatbuffer tables which could cause a denial of service attack.

If you need to ingest a trusted file with a massive number of fields and/or
metadata key-value pairs and are facing the error `"Unable to get root as
footer: TooManyTables"` then increase this parameter as necessary.

<a id="op-7f825e0e9aad4c06ea0d8265"></a>
## with_projection

`function` · `arrow_ipc::reader::FileReaderBuilder::with_projection` · arrow-ipc 59.3.0

```rust
fn with_projection(self, projection: Vec<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::FileReaderBuilder", "path": "FileReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1174, 1], "end": [1292, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1183`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Optional projection for which columns to load (zero-based column indices).
