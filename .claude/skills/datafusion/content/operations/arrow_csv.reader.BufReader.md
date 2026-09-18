# `arrow_csv::reader::BufReader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_csv.reader.BufReader.json).

<a id="op-4bee472bdef640cf895be341"></a>
## BufReader

`struct` · `arrow_csv::reader::BufReader` · arrow-csv 59.3.0

```rust
struct BufReader<R>
```

Source: `src/reader/mod.rs:504`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

CSV file reader implementation. See [`Reader`](../operations/arrow_csv.reader.Reader.md#op-80f5b4a82073800a3487a573) for usage

Despite having the same name as [`std::io::BufReader`, this structure does
not buffer reads itself

<a id="op-c198b010d0ac51589f940d58"></a>
## Item

`assoc_type` · `arrow_csv::reader::BufReader::Item` · arrow-csv 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_csv::reader::BufReader", "path": "BufReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [557, 1], "end": [563, 2], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/reader/mod.rs:558`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd745fb0836340fe8cb1f5a8"></a>
## fmt

`function` · `arrow_csv::reader::BufReader::fmt` · arrow-csv 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_csv::reader::BufReader", "path": "BufReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "generic_params": [], "type": {"generic": "R"}}}]}, "is_negative": false, "span": {"begin": [511, 1], "end": [520, 2], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader/mod.rs:515`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a9a4b240ed93ffadde4da29"></a>
## next

`function` · `arrow_csv::reader::BufReader::next` · arrow-csv 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_csv::reader::BufReader", "path": "BufReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [557, 1], "end": [563, 2], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/reader/mod.rs:560`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09872f211f4914ca195ed443"></a>
## schema

`function` · `arrow_csv::reader::BufReader::schema` · arrow-csv 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_csv::reader::BufReader", "path": "BufReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 1], "end": [569, 2], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchReader", "path": "RecordBatchReader"}, "trait_path": "arrow_array::record_batch::RecordBatchReader"}`

Source: `src/reader/mod.rs:566`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3c9384546f7202d6e676cb2"></a>
## schema

`function` · `arrow_csv::reader::BufReader::schema` · arrow-csv 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "alloc::io::buffered::bufreader::BufReader", "path": "BufReader"}}}], "constraints": []}}, "id": "arrow_csv::reader::BufReader", "path": "BufReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [535, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:525`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Returns the schema of the reader, useful for getting the schema without reading
record batches
