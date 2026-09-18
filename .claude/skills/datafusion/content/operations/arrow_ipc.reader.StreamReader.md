# `arrow_ipc::reader::StreamReader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.reader.StreamReader.json).

<a id="op-e303cfe08d702b62a9f2f221"></a>
## StreamReader

`struct` · `arrow_ipc::reader::StreamReader` · arrow-ipc 59.3.0

```rust
struct StreamReader<R>
```

Source: `src/reader.rs:1515`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Arrow Stream Reader

Reads Arrow [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es from bytes in the [IPC Streaming Format].

# See Also

* [`FileReader`](../operations/arrow_ipc.reader.FileReader.md#op-637d6dc146e8bf36b6e304bf) for random access.

# Example
```
# use arrow_array::record_batch;
# use arrow_ipc::reader::StreamReader;
# use arrow_ipc::writer::StreamWriter;
# let batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
# let mut stream = vec![]; // mimic a stream for the example
# {
#  let mut writer = StreamWriter::try_new(&mut stream, &batch.schema()).unwrap();
#  writer.write(&batch).unwrap();
#  writer.finish().unwrap();
# }
# let stream = stream.as_slice();
let projection = None; // read all columns
let mut reader = StreamReader::try_new(stream, projection).unwrap();
// read batches from the reader using the Iterator trait
let mut num_rows = 0;
for batch in reader {
   let batch = batch.unwrap();
   num_rows += batch.num_rows();
}
assert_eq!(num_rows, 3);
```

[IPC Streaming Format]: https://arrow.apache.org/docs/format/Columnar.html#ipc-streaming-format

<a id="op-d9594d5db25ac30e179d51a1"></a>
## Item

`assoc_type` · `arrow_ipc::reader::StreamReader::Item` · arrow-ipc 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::StreamReader", "path": "StreamReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1776, 1], "end": [1782, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/reader.rs:1777`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8738deb4fa10e1fb0d916c38"></a>
## fmt

`function` · `arrow_ipc::reader::StreamReader::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::StreamReader", "path": "StreamReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1541, 1], "end": [1551, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader.rs:1542`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ab65c32fa7b8283a7b6bcf2"></a>
## get_mut

`function` · `arrow_ipc::reader::StreamReader::get_mut` · arrow-ipc 59.3.0

```rust
fn get_mut(&mut self) -> &mut R
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::StreamReader", "path": "StreamReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1562, 1], "end": [1774, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1761`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Gets a mutable reference to the underlying reader.

It is inadvisable to directly read from the underlying reader.

<a id="op-db8de7fd6b93f985728a0a4e"></a>
## get_ref

`function` · `arrow_ipc::reader::StreamReader::get_ref` · arrow-ipc 59.3.0

```rust
fn get_ref(&self) -> &R
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::StreamReader", "path": "StreamReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1562, 1], "end": [1774, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1754`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Gets a reference to the underlying reader.

It is inadvisable to directly read from the underlying reader.

<a id="op-22a4f11790fcfee16c96bf21"></a>
## is_finished

`function` · `arrow_ipc::reader::StreamReader::is_finished` · arrow-ipc 59.3.0

```rust
fn is_finished(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::StreamReader", "path": "StreamReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1562, 1], "end": [1774, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1634`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Check if the stream is finished

<a id="op-9840d48373fbcb8f120c5400"></a>
## next

`function` · `arrow_ipc::reader::StreamReader::next` · arrow-ipc 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::StreamReader", "path": "StreamReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1776, 1], "end": [1782, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/reader.rs:1779`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63ef4244b9858bc37202bd9a"></a>
## schema

`function` · `arrow_ipc::reader::StreamReader::schema` · arrow-ipc 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::StreamReader", "path": "StreamReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1562, 1], "end": [1774, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1629`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Return the schema of the stream

<a id="op-f86ab7d52bf73943d9b3061c"></a>
## schema

`function` · `arrow_ipc::reader::StreamReader::schema` · arrow-ipc 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::StreamReader", "path": "StreamReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1784, 1], "end": [1788, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchReader", "path": "RecordBatchReader"}, "trait_path": "arrow_array::record_batch::RecordBatchReader"}`

Source: `src/reader.rs:1785`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c383f8c04a850ef65d3bc81f"></a>
## try_new

`function` · `arrow_ipc::reader::StreamReader::try_new` · arrow-ipc 59.3.0

```rust
fn try_new(reader: R, projection: Option<Vec<usize>>) -> Result<StreamReader<R>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::StreamReader", "path": "StreamReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1562, 1], "end": [1774, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1574`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create a new stream reader.

To check if the reader is done, use [`is_finished(self)`](StreamReader::is_finished).

There is no internal buffering. If buffered reads are needed you likely want to use
[`StreamReader::try_new_buffered`](../operations/arrow_ipc.reader.StreamReader.md#op-1f1fc3e2d4a7d0ccac5ce2cc) instead.

# Errors

An ['Err'](Result::Err) may be returned if the reader does not encounter a schema
as the first message in the stream.

Unresolved upstream links (retained, not inferred): `Result::Err`.

<a id="op-1f1fc3e2d4a7d0ccac5ce2cc"></a>
## try_new_buffered

`function` · `arrow_ipc::reader::StreamReader::try_new_buffered` · arrow-ipc 59.3.0

```rust
fn try_new_buffered(reader: R, projection: Option<Vec<usize>>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "alloc::io::buffered::bufreader::BufReader", "path": "std::io::BufReader"}}}], "constraints": []}}, "id": "arrow_ipc::reader::StreamReader", "path": "StreamReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1553, 1], "end": [1560, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1557`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create a new stream reader with the reader wrapped in a BufReader.

See [`StreamReader::try_new`](../operations/arrow_ipc.reader.StreamReader.md#op-c383f8c04a850ef65d3bc81f) for an unbuffered version.

<a id="op-d198892ef6bc38acdd044391"></a>
## try_new_unbuffered

`function` · `arrow_ipc::reader::StreamReader::try_new_unbuffered` · arrow-ipc 59.3.0

```rust
fn try_new_unbuffered(reader: R, projection: Option<Vec<usize>>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::StreamReader", "path": "StreamReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1562, 1], "end": [1774, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1621`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Deprecated, use [`StreamReader::try_new`](../operations/arrow_ipc.reader.StreamReader.md#op-c383f8c04a850ef65d3bc81f) instead.

<a id="op-74932c8353727b9b106ce1dd"></a>
## with_skip_validation

`function` · `arrow_ipc::reader::StreamReader::with_skip_validation` · arrow-ipc 59.3.0

```rust
unsafe fn with_skip_validation(self, skip_validation: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::StreamReader", "path": "StreamReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1562, 1], "end": [1774, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1770`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Specifies if validation should be skipped when reading data (defaults to `false`)

# Safety

See [`FileDecoder::with_skip_validation`](../operations/arrow_ipc.reader.FileDecoder.md#op-e750154fa82597609b22c72a)
