# `arrow_avro::writer::EncodedRows`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.writer.EncodedRows.json).

<a id="op-a24bf6646de2b0d806b87c41"></a>
## EncodedRows

`struct` · `arrow_avro::writer::EncodedRows` · arrow-avro 59.3.0

```rust
struct EncodedRows
```

Source: `src/writer/mod.rs:176`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

A contiguous set of Avro encoded rows.

`EncodedRows` stores:
- a single backing byte buffer (`bytes::Bytes`)
- a `Vec<usize>` of row boundary offsets (length = `rows + 1`)

This lets callers get per-row payloads as zero-copy `Bytes` slices.

For compatibility with APIs that require owned `Vec<u8>`, use:
`let vecs: Vec<Vec<u8>> = rows.iter().map(|b| b.to_vec()).collect();`

<a id="op-180d518f1e01fc51fc212cb2"></a>
## bytes

`function` · `arrow_avro::writer::EncodedRows::bytes` · arrow-avro 59.3.0

```rust
fn bytes(&self) -> &Bytes
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::EncodedRows", "path": "EncodedRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [314, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:211`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns a reference to the single contiguous backing buffer.

This buffer contains the payloads of all rows concatenated together.

# Note

To access individual row payloads, prefer using [`Self::row`](../operations/arrow_avro.writer.EncodedRows.md#op-f788d1582cc8889d0b4fe889) or [`Self::iter`](../operations/arrow_avro.writer.EncodedRows.md#op-d5a5340d24ac33d0a647b30a)
rather than slicing this buffer manually.

<a id="op-6abcfef9f031d904803d9474"></a>
## clone

`function` · `arrow_avro::writer::EncodedRows::clone` · arrow-avro 59.3.0

```rust
fn clone(&self) -> EncodedRows
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::EncodedRows", "path": "EncodedRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 17], "end": [175, 22], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/writer/mod.rs:175`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c935346d82e5ee813ff577a"></a>
## fmt

`function` · `arrow_avro::writer::EncodedRows::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::EncodedRows", "path": "EncodedRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 10], "end": [175, 15], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer/mod.rs:175`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdd65f1a956202791c785cdc"></a>
## is_empty

`function` · `arrow_avro::writer::EncodedRows::is_empty` · arrow-avro 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::EncodedRows", "path": "EncodedRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [314, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:198`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns `true` if this container holds no encoded rows.

<a id="op-d5a5340d24ac33d0a647b30a"></a>
## iter

`function` · `arrow_avro::writer::EncodedRows::iter` · arrow-avro 59.3.0

```rust
fn iter(&self) -> impl ExactSizeIterator<Item = Bytes> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::EncodedRows", "path": "EncodedRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [314, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:311`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Iterate over rows as zero-copy `Bytes` slices.

This iterator is infallible and is intended for the common case where
`EncodedRows` is produced by [`Encoder::flush`](../operations/arrow_avro.writer.Encoder.md#op-c7a2b62f16f52a249150b213), which guarantees valid offsets.

# Examples

```
use std::sync::Arc;
use arrow_array::{ArrayRef, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use arrow_avro::writer::WriterBuilder;
use arrow_avro::writer::format::AvroSoeFormat;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let schema = Schema::new(vec![Field::new("x", DataType::Int32, false)]);
let batch = RecordBatch::try_new(
    Arc::new(schema.clone()),
    vec![Arc::new(Int32Array::from(vec![10, 20])) as ArrayRef],
)?;

let mut encoder = WriterBuilder::new(schema).build_encoder::<AvroSoeFormat>()?;
encoder.encode(&batch)?;
let rows = encoder.flush();

assert_eq!(rows.iter().count(), 2);
# Ok(())
# }
```

<a id="op-b6b7d3e511ef0bc3f85a414d"></a>
## len

`function` · `arrow_avro::writer::EncodedRows::len` · arrow-avro 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::EncodedRows", "path": "EncodedRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [314, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:192`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the number of encoded rows stored in this container.

<a id="op-0e8b79f0d049c4d919679f2e"></a>
## new

`function` · `arrow_avro::writer::EncodedRows::new` · arrow-avro 59.3.0

```rust
fn new(data: Bytes, offsets: Vec<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::EncodedRows", "path": "EncodedRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [314, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Create a new `EncodedRows` from a backing buffer and row boundary offsets.

`offsets` must have length `rows + 1`, and be monotonically non-decreasing.
The last offset should equal `data.len()`.

<a id="op-5fd7384fe36f3899f47c1d43"></a>
## offsets

`function` · `arrow_avro::writer::EncodedRows::offsets` · arrow-avro 59.3.0

```rust
fn offsets(&self) -> &[usize]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::EncodedRows", "path": "EncodedRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [314, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:220`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the row boundary offsets.

The returned slice always has the length `self.len() + 1`. The `n`th row payload
corresponds to `bytes[offsets[n] ... offsets[n+1]]`.

<a id="op-f788d1582cc8889d0b4fe889"></a>
## row

`function` · `arrow_avro::writer::EncodedRows::row` · arrow-avro 59.3.0

```rust
fn row(&self, n: usize) -> Result<Bytes, AvroError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::writer::EncodedRows", "path": "EncodedRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [314, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:255`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Return the `n`th row as a zero-copy `Bytes` slice.

# Errors

Returns an error if `n` is out of bounds or if the internal offsets are invalid
(e.g., offsets are not within the backing buffer).

# Examples

```
use std::sync::Arc;
use arrow_array::{ArrayRef, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use arrow_avro::writer::WriterBuilder;
use arrow_avro::writer::format::AvroSoeFormat;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let schema = Schema::new(vec![Field::new("x", DataType::Int32, false)]);
let batch = RecordBatch::try_new(
    Arc::new(schema.clone()),
    vec![Arc::new(Int32Array::from(vec![1, 2])) as ArrayRef],
)?;

let mut encoder = WriterBuilder::new(schema).build_encoder::<AvroSoeFormat>()?;
encoder.encode(&batch)?;
let rows = encoder.flush();

assert_eq!(rows.iter().count(), 2);
# Ok(())
# }
```
