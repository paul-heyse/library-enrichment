# `parquet::file::writer::SerializedPageWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.writer.SerializedPageWriter.json).

<a id="op-808a2c37692c038a9ffbbd6b"></a>
## SerializedPageWriter

`struct` · `parquet::file::writer::SerializedPageWriter` · parquet 59.3.0

```rust
struct SerializedPageWriter<'a, W: Write>
```

Source: `src/file/writer.rs:1011`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A serialized implementation for Parquet [`PageWriter`](../operations/parquet.column.page.PageWriter.md#op-1532a369b470f0e4d5dfd714).
Writes and serializes pages and metadata into output stream.

`SerializedPageWriter` should not be used after calling `close()`.

<a id="op-1cd93eafa76d730aa6e1f521"></a>
## close

`function` · `parquet::file::writer::SerializedPageWriter::close` · parquet 59.3.0

```rust
fn close(&mut self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedPageWriter", "path": "SerializedPageWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1077, 1], "end": [1112, 2], "filename": "src/file/writer.rs"}, "trait": {"args": null, "id": "parquet::column::page::PageWriter", "path": "PageWriter"}, "trait_path": "parquet::column::page::PageWriter"}`

Source: `src/file/writer.rs:1108`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcc9235c67b71cc587ec1165"></a>
## new

`function` · `parquet::file::writer::SerializedPageWriter::new` · parquet 59.3.0

```rust
fn new(sink: &'a mut TrackedWrite<W>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedPageWriter", "path": "SerializedPageWriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1017, 1], "end": [1043, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:1019`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new page writer.

<a id="op-09fb62bec1bf387bb69c69ad"></a>
## write_page

`function` · `parquet::file::writer::SerializedPageWriter::write_page` · parquet 59.3.0

```rust
fn write_page(&mut self, page: CompressedPage) -> Result<PageWriteSpec>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::SerializedPageWriter", "path": "SerializedPageWriter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1077, 1], "end": [1112, 2], "filename": "src/file/writer.rs"}, "trait": {"args": null, "id": "parquet::column::page::PageWriter", "path": "PageWriter"}, "trait_path": "parquet::column::page::PageWriter"}`

Source: `src/file/writer.rs:1078`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
