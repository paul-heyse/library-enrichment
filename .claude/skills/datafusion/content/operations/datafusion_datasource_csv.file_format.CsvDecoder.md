# `datafusion_datasource_csv::file_format::CsvDecoder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_csv.file_format.CsvDecoder.json).

<a id="op-348f6f8c3ad2ac913d6c470b"></a>
## CsvDecoder

`struct` · `datafusion_datasource_csv::file_format::CsvDecoder` · datafusion-datasource-csv 55.1.0

```rust
struct CsvDecoder
```

Source: `src/file_format.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30f347e9258cbdc9a3bd2c30"></a>
## can_flush_early

`function` · `datafusion_datasource_csv::file_format::CsvDecoder::can_flush_early` · datafusion-datasource-csv 55.1.0

```rust
fn can_flush_early(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvDecoder", "path": "CsvDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [335, 1], "end": [347, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::decoder::Decoder", "path": "Decoder"}, "trait_path": "datafusion_datasource::decoder::Decoder"}`

Source: `src/file_format.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b703ace92bab444881c8acb"></a>
## decode

`function` · `datafusion_datasource_csv::file_format::CsvDecoder::decode` · datafusion-datasource-csv 55.1.0

```rust
fn decode(&mut self, buf: &[u8]) -> Result<usize, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvDecoder", "path": "CsvDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [335, 1], "end": [347, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::decoder::Decoder", "path": "Decoder"}, "trait_path": "datafusion_datasource::decoder::Decoder"}`

Source: `src/file_format.rs:336`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c01b24ff959ea724d1dc342"></a>
## flush

`function` · `datafusion_datasource_csv::file_format::CsvDecoder::flush` · datafusion-datasource-csv 55.1.0

```rust
fn flush(&mut self) -> Result<Option<RecordBatch>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvDecoder", "path": "CsvDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [335, 1], "end": [347, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "datafusion_datasource::decoder::Decoder", "path": "Decoder"}, "trait_path": "datafusion_datasource::decoder::Decoder"}`

Source: `src/file_format.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bcc23d86fad2ab7bef21382"></a>
## fmt

`function` · `datafusion_datasource_csv::file_format::CsvDecoder::fmt` · datafusion-datasource-csv 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvDecoder", "path": "CsvDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 10], "end": [324, 15], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_format.rs:324`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0168ddc839bdd454a5b9c5c0"></a>
## new

`function` · `datafusion_datasource_csv::file_format::CsvDecoder::new` · datafusion-datasource-csv 55.1.0

```rust
fn new(decoder: arrow::csv::reader::Decoder) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_csv::file_format::CsvDecoder", "path": "CsvDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [329, 1], "end": [333, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-csv/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
