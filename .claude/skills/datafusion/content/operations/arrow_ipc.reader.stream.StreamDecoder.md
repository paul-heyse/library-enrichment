# `arrow_ipc::reader::stream::StreamDecoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.reader.stream.StreamDecoder.json).

<a id="op-945a70edefc042f10763dbee"></a>
## StreamDecoder

`struct` · `arrow_ipc::reader::stream::StreamDecoder` · arrow-ipc 59.3.0

```rust
struct StreamDecoder
```

Source: `src/reader/stream.rs:35`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

A low-level interface for reading [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) data from a stream of bytes

See [StreamReader](crate::reader::StreamReader) for a higher-level interface

<a id="op-5c8e95bc24fe46c31db8e91b"></a>
## decode

`function` · `arrow_ipc::reader::stream::StreamDecoder::decode` · arrow-ipc 59.3.0

```rust
fn decode(&mut self, buffer: &mut Buffer) -> Result<Option<RecordBatch>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::stream::StreamDecoder", "path": "StreamDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [302, 2], "filename": "src/reader/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/stream.rs:159`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Try to read the next [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) from the provided [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)

[`Buffer::advance`] will be called on `buffer` for any consumed bytes.

The push-based interface facilitates integration with sources that yield arbitrarily
delimited bytes ranges, such as a chunked byte stream received from object storage

```
# use arrow_array::RecordBatch;
# use arrow_buffer::Buffer;
# use arrow_ipc::reader::StreamDecoder;
# use arrow_schema::ArrowError;
#
fn print_stream<I>(src: impl Iterator<Item = Buffer>) -> Result<(), ArrowError> {
    let mut decoder = StreamDecoder::new();
    for mut x in src {
        while !x.is_empty() {
            if let Some(x) = decoder.decode(&mut x)? {
                println!("{x:?}");
            }
            if let Some(schema) = decoder.schema() {
                println!("Schema: {schema:?}");
            }
        }
    }
    decoder.finish().unwrap();
    Ok(())
}
```

Unresolved upstream links (retained, not inferred): ``Buffer::advance``.

<a id="op-24f38be20e661242a2a22025"></a>
## default

`function` · `arrow_ipc::reader::stream::StreamDecoder::default` · arrow-ipc 59.3.0

```rust
fn default() -> StreamDecoder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::stream::StreamDecoder", "path": "StreamDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 24], "filename": "src/reader/stream.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/reader/stream.rs:34`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6adee2df65f58c6aa4d6c72f"></a>
## finish

`function` · `arrow_ipc::reader::stream::StreamDecoder::finish` · arrow-ipc 59.3.0

```rust
fn finish(&mut self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::stream::StreamDecoder", "path": "StreamDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [302, 2], "filename": "src/reader/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/stream.rs:291`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Signal the end of stream

Returns an error if any partial data remains in the stream

<a id="op-00500ff1672a7e83800ca538"></a>
## fmt

`function` · `arrow_ipc::reader::stream::StreamDecoder::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::stream::StreamDecoder", "path": "StreamDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/reader/stream.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader/stream.rs:34`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6275fec945ec8d2cc23eb20"></a>
## new

`function` · `arrow_ipc::reader::stream::StreamDecoder::new` · arrow-ipc 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::stream::StreamDecoder", "path": "StreamDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [302, 2], "filename": "src/reader/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/stream.rs:90`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Create a new [`StreamDecoder`](../operations/arrow_ipc.reader.stream.StreamDecoder.md#op-945a70edefc042f10763dbee)

<a id="op-70ec6fe0a8316944a05369ed"></a>
## schema

`function` · `arrow_ipc::reader::stream::StreamDecoder::schema` · arrow-ipc 59.3.0

```rust
fn schema(&self) -> Option<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::stream::StreamDecoder", "path": "StreamDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [302, 2], "filename": "src/reader/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/stream.rs:112`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Return the schema if decoded, else None.

<a id="op-c97eecbb2e28fd1d7697693f"></a>
## with_require_alignment

`function` · `arrow_ipc::reader::stream::StreamDecoder::with_require_alignment` · arrow-ipc 59.3.0

```rust
fn with_require_alignment(self, require_alignment: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::stream::StreamDecoder", "path": "StreamDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [302, 2], "filename": "src/reader/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/stream.rs:106`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Specifies whether or not array data in input buffers is required to be properly aligned.

If `require_alignment` is true, this decoder will return an error if any array data in the
input `buf` is not properly aligned.
Under the hood it will use [`arrow_data::ArrayDataBuilder::build`] to construct
[`arrow_data::ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78).

If `require_alignment` is false (the default), this decoder will automatically allocate a
new aligned buffer and copy over the data if any array data in the input `buf` is not
properly aligned. (Properly aligned array data will remain zero-copy.)
Under the hood it will use [`arrow_data::ArrayDataBuilder::align_buffers`] to construct
[`arrow_data::ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78).

Unresolved upstream links (retained, not inferred): ``arrow_data::ArrayDataBuilder::build``, ``arrow_data::ArrayDataBuilder::align_buffers``.

<a id="op-a59ef0c6983413e7cd6e28c4"></a>
## with_skip_validation

`function` · `arrow_ipc::reader::stream::StreamDecoder::with_skip_validation` · arrow-ipc 59.3.0

```rust
unsafe fn with_skip_validation(self, skip_validation: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::reader::stream::StreamDecoder", "path": "StreamDecoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [302, 2], "filename": "src/reader/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/stream.rs:125`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Specifies if validation should be skipped when reading data (defaults to `false`)

# Safety

This flag must only be set to `true` when you trust the input data and are
sure the data you are reading is valid Arrow IPC stream data, otherwise
undefined behavior may result.

For example, DataFusion uses this when reading spill files it wrote itself.
