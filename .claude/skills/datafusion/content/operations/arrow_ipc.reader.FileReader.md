# `arrow_ipc::reader::FileReader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.reader.FileReader.json).

<a id="op-637d6dc146e8bf36b6e304bf"></a>
## FileReader

`struct` · `arrow_ipc::reader::FileReader` · arrow-ipc 59.3.0

```rust
struct FileReader<R>
```

Source: `src/reader.rs:1338`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Arrow File Reader

Reads Arrow [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es from bytes in the [IPC File Format],
providing random access to the record batches.

# See Also

* [`Self::set_index`](../operations/arrow_ipc.reader.FileReader.md#op-3cb1c25f5682b51adde1b7ab) for random access
* [`StreamReader`](../operations/arrow_ipc.reader.StreamReader.md#op-e303cfe08d702b62a9f2f221) for reading streaming data

# Example: Reading from a `File`
```
# use std::io::Cursor;
use arrow_array::record_batch;
# use arrow_ipc::reader::FileReader;
# use arrow_ipc::writer::FileWriter;
# let batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
# let mut file = vec![]; // mimic a stream for the example
# {
#  let mut writer = FileWriter::try_new(&mut file, &batch.schema()).unwrap();
#  writer.write(&batch).unwrap();
#  writer.write(&batch).unwrap();
#  writer.finish().unwrap();
# }
# let mut file = Cursor::new(&file);
let projection = None; // read all columns
let mut reader = FileReader::try_new(&mut file, projection).unwrap();
// Position the reader to the second batch
reader.set_index(1).unwrap();
// read batches from the reader using the Iterator trait
let mut num_rows = 0;
for batch in reader {
   let batch = batch.unwrap();
   num_rows += batch.num_rows();
}
assert_eq!(num_rows, 3);
```
# Example: Reading from `mmap`ed file

For an example creating Arrays without copying using  memory mapped (`mmap`)
files see the [`zero_copy_ipc`] example.

[IPC File Format]: https://arrow.apache.org/docs/format/Columnar.html#ipc-file-format
[`zero_copy_ipc`]: https://github.com/apache/arrow-rs/blob/main/arrow/examples/zero_copy_ipc.rs

<a id="op-e6cc156db6eb2f81fb5da060"></a>
## Item

`assoc_type` · `arrow_ipc::reader::FileReader::Item` · arrow-ipc 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::FileReader", "path": "FileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::seek::Seek", "path": "Seek"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1463, 1], "end": [1474, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/reader.rs:1464`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d153046fb36081cf82b06640"></a>
## custom_metadata

`function` · `arrow_ipc::reader::FileReader::custom_metadata` · arrow-ipc 59.3.0

```rust
fn custom_metadata(&self) -> &HashMap<String, String>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::FileReader", "path": "FileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::seek::Seek", "path": "Seek"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1380, 1], "end": [1461, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1400`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Return user defined customized metadata

<a id="op-ad8378df63ab4d8a44b6566f"></a>
## fmt

`function` · `arrow_ipc::reader::FileReader::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::FileReader", "path": "FileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1360, 1], "end": [1369, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader.rs:1361`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bee86230b7d0ac0450a33d28"></a>
## get_mut

`function` · `arrow_ipc::reader::FileReader::get_mut` · arrow-ipc 59.3.0

```rust
fn get_mut(&mut self) -> &mut R
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::FileReader", "path": "FileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::seek::Seek", "path": "Seek"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1380, 1], "end": [1461, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1448`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Gets a mutable reference to the underlying reader.

It is inadvisable to directly read from the underlying reader.

<a id="op-61f3733cdff2cdad273d277e"></a>
## get_ref

`function` · `arrow_ipc::reader::FileReader::get_ref` · arrow-ipc 59.3.0

```rust
fn get_ref(&self) -> &R
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::FileReader", "path": "FileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::seek::Seek", "path": "Seek"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1380, 1], "end": [1461, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1441`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Gets a reference to the underlying reader.

It is inadvisable to directly read from the underlying reader.

<a id="op-4d5aff1b2b2c0fe7de778817"></a>
## next

`function` · `arrow_ipc::reader::FileReader::next` · arrow-ipc 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::FileReader", "path": "FileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::seek::Seek", "path": "Seek"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1463, 1], "end": [1474, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/reader.rs:1466`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29b20f145323b4769e8fdc32"></a>
## num_batches

`function` · `arrow_ipc::reader::FileReader::num_batches` · arrow-ipc 59.3.0

```rust
fn num_batches(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::FileReader", "path": "FileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::seek::Seek", "path": "Seek"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1380, 1], "end": [1461, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1405`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Return the number of batches in the file

<a id="op-8d03a7c11900c061cf05b662"></a>
## schema

`function` · `arrow_ipc::reader::FileReader::schema` · arrow-ipc 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::FileReader", "path": "FileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::seek::Seek", "path": "Seek"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1380, 1], "end": [1461, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1410`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Return the schema of the file

<a id="op-a7e5cdf7b43e93e8a8188c8d"></a>
## schema

`function` · `arrow_ipc::reader::FileReader::schema` · arrow-ipc 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::FileReader", "path": "FileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::seek::Seek", "path": "Seek"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 1], "end": [1480, 2], "filename": "src/reader.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchReader", "path": "RecordBatchReader"}, "trait_path": "arrow_array::record_batch::RecordBatchReader"}`

Source: `src/reader.rs:1477`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cb1c25f5682b51adde1b7ab"></a>
## set_index

`function` · `arrow_ipc::reader::FileReader::set_index` · arrow-ipc 59.3.0

```rust
fn set_index(&mut self, index: usize) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::FileReader", "path": "FileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::seek::Seek", "path": "Seek"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1380, 1], "end": [1461, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1417`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

See to a specific [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

Sets the current block to the index, allowing random reads

<a id="op-83fb7ff70e1971cd86eb34b0"></a>
## try_new

`function` · `arrow_ipc::reader::FileReader::try_new` · arrow-ipc 59.3.0

```rust
fn try_new(reader: R, projection: Option<Vec<usize>>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::FileReader", "path": "FileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::seek::Seek", "path": "Seek"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1380, 1], "end": [1461, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1391`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create a new file reader.

There is no internal buffering. If buffered reads are needed you likely want to use
[`FileReader::try_new_buffered`](../operations/arrow_ipc.reader.FileReader.md#op-899dfb786db15a0d86a5b9c9) instead.

# Errors

An ['Err'](Result::Err) may be returned if:
- the file does not meet the Arrow Format footer requirements, or
- file endianness does not match the target endianness.

Unresolved upstream links (retained, not inferred): `Result::Err`.

<a id="op-899dfb786db15a0d86a5b9c9"></a>
## try_new_buffered

`function` · `arrow_ipc::reader::FileReader::try_new_buffered` · arrow-ipc 59.3.0

```rust
fn try_new_buffered(reader: R, projection: Option<Vec<usize>>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "alloc::io::buffered::bufreader::BufReader", "path": "std::io::BufReader"}}}], "constraints": []}}, "id": "arrow_ipc::reader::FileReader", "path": "FileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::seek::Seek", "path": "Seek"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1371, 1], "end": [1378, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1375`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to create a new file reader with the reader wrapped in a BufReader.

See [`FileReader::try_new`](../operations/arrow_ipc.reader.FileReader.md#op-83fb7ff70e1971cd86eb34b0) for an unbuffered version.

<a id="op-400075171d6ad5045b744a05"></a>
## with_skip_validation

`function` · `arrow_ipc::reader::FileReader::with_skip_validation` · arrow-ipc 59.3.0

```rust
unsafe fn with_skip_validation(self, skip_validation: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_ipc::reader::FileReader", "path": "FileReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::io::seek::Seek", "path": "Seek"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1380, 1], "end": [1461, 2], "filename": "src/reader.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader.rs:1457`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Specifies if validation should be skipped when reading data (defaults to `false`)

# Safety

See [`FileDecoder::with_skip_validation`](../operations/arrow_ipc.reader.FileDecoder.md#op-e750154fa82597609b22c72a)
