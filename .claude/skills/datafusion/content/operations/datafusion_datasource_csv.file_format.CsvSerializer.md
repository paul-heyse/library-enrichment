# `datafusion_datasource_csv::file_format::CsvSerializer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_csv.file_format.CsvSerializer.json).

<a id="op-6d1f46befe33092e28dfc4de"></a>
## CsvSerializer

`struct` · `datafusion_datasource_csv::file_format::CsvSerializer` · datafusion-datasource-csv 55.1.0

```rust
struct CsvSerializer
```

Source: `src/file_format.rs:697`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Define a struct for serializing CSV records to a stream

<a id="op-5713b51a29a86261496b1a84"></a>
## default

`function` · `datafusion_datasource_csv::file_format::CsvSerializer::default` · datafusion-datasource-csv 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSerializer", "path": "CsvSerializer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [690, 1], "end": [694, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_format.rs:691`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27fa7605255043dbe5f542d7"></a>
## fmt

`function` · `datafusion_datasource_csv::file_format::CsvSerializer::fmt` · datafusion-datasource-csv 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSerializer", "path": "CsvSerializer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [349, 1], "end": [355, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:350`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3092d7b486204e65592175a8"></a>
## new

`function` · `datafusion_datasource_csv::file_format::CsvSerializer::new` · datafusion-datasource-csv 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSerializer", "path": "CsvSerializer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [704, 1], "end": [724, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:706`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Constructor for the CsvSerializer object

<a id="op-f705fda62564d761f14ee753"></a>
## serialize

`function` · `datafusion_datasource_csv::file_format::CsvSerializer::serialize` · datafusion-datasource-csv 55.1.0

```rust
fn serialize(&self, batch: RecordBatch, initial: bool) -> Result<Bytes>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSerializer", "path": "CsvSerializer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [726, 1], "end": [736, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::write::BatchSerializer", "path": "BatchSerializer"}, "trait_path": "datafusion_datasource::write::BatchSerializer"}`

Source: `src/file_format.rs:727`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe7c0975077a89b1a282487e"></a>
## with_builder

`function` · `datafusion_datasource_csv::file_format::CsvSerializer::with_builder` · datafusion-datasource-csv 55.1.0

```rust
fn with_builder(self, builder: WriterBuilder) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSerializer", "path": "CsvSerializer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [704, 1], "end": [724, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:714`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Method for setting the CSV writer builder

<a id="op-0ae54791156716276ed2de99"></a>
## with_header

`function` · `datafusion_datasource_csv::file_format::CsvSerializer::with_header` · datafusion-datasource-csv 55.1.0

```rust
fn with_header(self, header: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvSerializer", "path": "CsvSerializer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [704, 1], "end": [724, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:720`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

Method for setting the CSV writer header status
