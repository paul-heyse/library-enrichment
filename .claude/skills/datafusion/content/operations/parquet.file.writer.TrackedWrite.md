# `parquet::file::writer::TrackedWrite`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.writer.TrackedWrite.json).

<a id="op-ea89b2859b3cf3ef198ee52c"></a>
## TrackedWrite

`struct` · `parquet::file::writer::TrackedWrite` · parquet 59.3.0

```rust
struct TrackedWrite<W: Write>
```

Source: `src/file/writer.rs:53`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A wrapper around a [`Write`] that keeps track of the number
of bytes that have been written. The given [`Write`] is wrapped
with a [`BufWriter`] to optimize writing performance.

Unresolved upstream links (retained, not inferred): ``BufWriter``, ``Write``.

<a id="op-b16e5f59dd6fdebcda5fdeed"></a>
## bytes_written

`function` · `parquet::file::writer::TrackedWrite::bytes_written` · parquet 59.3.0

```rust
fn bytes_written(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::TrackedWrite", "path": "TrackedWrite"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [92, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:69`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of bytes written to this instance

<a id="op-a47a58570340458f8746a061"></a>
## flush

`function` · `parquet::file::writer::TrackedWrite::flush` · parquet 59.3.0

```rust
fn flush(&mut self) -> std::io::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::TrackedWrite", "path": "TrackedWrite"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [117, 2], "filename": "src/file/writer.rs"}, "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}, "trait_path": "core::io::write::Write"}`

Source: `src/file/writer.rs:114`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-186ea9a17c4efb4826153344"></a>
## inner

`function` · `parquet::file::writer::TrackedWrite::inner` · parquet 59.3.0

```rust
fn inner(&self) -> &W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::TrackedWrite", "path": "TrackedWrite"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [92, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:74`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to the underlying writer.

<a id="op-d3f33d89f05439007af15d2a"></a>
## inner_mut

`function` · `parquet::file::writer::TrackedWrite::inner_mut` · parquet 59.3.0

```rust
fn inner_mut(&mut self) -> &mut W
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::TrackedWrite", "path": "TrackedWrite"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [92, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:82`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a mutable reference to the underlying writer.

It is inadvisable to directly write to the underlying writer, doing so
will likely result in data corruption

<a id="op-5923d2ebd97d0d2418f002b1"></a>
## into_inner

`function` · `parquet::file::writer::TrackedWrite::into_inner` · parquet 59.3.0

```rust
fn into_inner(self) -> Result<W>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::TrackedWrite", "path": "TrackedWrite"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [92, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:87`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the underlying writer.

<a id="op-af8d23c7c7b86738175c7bea"></a>
## new

`function` · `parquet::file::writer::TrackedWrite::new` · parquet 59.3.0

```rust
fn new(inner: W) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::TrackedWrite", "path": "TrackedWrite"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [92, 2], "filename": "src/file/writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/writer.rs:60`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`TrackedWrite`](../operations/parquet.file.writer.TrackedWrite.md#op-ea89b2859b3cf3ef198ee52c) from a [`Write`]

Unresolved upstream links (retained, not inferred): ``Write``.

<a id="op-9e696ad94257cbc17f9e6b34"></a>
## write

`function` · `parquet::file::writer::TrackedWrite::write` · parquet 59.3.0

```rust
fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::TrackedWrite", "path": "TrackedWrite"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [117, 2], "filename": "src/file/writer.rs"}, "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}, "trait_path": "core::io::write::Write"}`

Source: `src/file/writer.rs:95`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68455a82b77c7e7bd05d053f"></a>
## write_all

`function` · `parquet::file::writer::TrackedWrite::write_all` · parquet 59.3.0

```rust
fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::TrackedWrite", "path": "TrackedWrite"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [117, 2], "filename": "src/file/writer.rs"}, "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}, "trait_path": "core::io::write::Write"}`

Source: `src/file/writer.rs:107`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f55fb9b5f0aa64f7fddce907"></a>
## write_vectored

`function` · `parquet::file::writer::TrackedWrite::write_vectored` · parquet 59.3.0

```rust
fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "W"}}], "constraints": []}}, "id": "parquet::file::writer::TrackedWrite", "path": "TrackedWrite"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}}}], "default": null, "is_synthetic": false}}, "name": "W"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [117, 2], "filename": "src/file/writer.rs"}, "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}, "trait_path": "core::io::write::Write"}`

Source: `src/file/writer.rs:101`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
