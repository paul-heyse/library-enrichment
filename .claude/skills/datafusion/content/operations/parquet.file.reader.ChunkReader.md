# `parquet::file::reader::ChunkReader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.reader.ChunkReader.json).

<a id="op-8eae425b3b94361396eb9d94"></a>
## ChunkReader

`trait` · `parquet::file::reader::ChunkReader` · parquet 59.3.0

```rust
trait ChunkReader: Length + Send + Sync
```

Source: `src/file/reader.rs:64`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Generates [`Read`]ers to read chunks of a Parquet data source.

The Parquet reader uses [`ChunkReader`](../operations/parquet.file.reader.ChunkReader.md#op-8eae425b3b94361396eb9d94) to access Parquet data, allowing
multiple decoders to read concurrently from different locations in the same
file.

The trait functions both as a reader and a factory for readers.
* random access via [`Self::get_bytes`](../operations/parquet.file.reader.ChunkReader.md#op-22e04ef7e0e3a7dbd2b8e23e)
* sequential access via the reader returned via factory method [`Self::get_read`](../operations/parquet.file.reader.ChunkReader.md#op-220c35b6bd86a16f1bb46993)

# Provided Implementations
* [`File`] for reading from local file system
* [`Bytes`] for reading from an in-memory buffer

User provided implementations can implement more sophisticated behaviors
such as on-demand buffering or scan sharing.

Unresolved upstream links (retained, not inferred): ``Read``, ``File``, ``Bytes``.

<a id="op-04edcfa1be0448e42e5a9190"></a>
## T

`assoc_type` · `parquet::file::reader::ChunkReader::T` · parquet 59.3.0

```rust
T
```

Source: `src/file/reader.rs:66`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The concrete type of reader returned by this trait

<a id="op-22e04ef7e0e3a7dbd2b8e23e"></a>
## get_bytes

`function` · `parquet::file::reader::ChunkReader::get_bytes` · parquet 59.3.0

```rust
fn get_bytes(&self, start: u64, length: usize) -> Result<Bytes>
```

Source: `src/file/reader.rs:81`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get a range of data in memory as [`Bytes`]

Similarly to [`Self::get_read`](../operations/parquet.file.reader.ChunkReader.md#op-220c35b6bd86a16f1bb46993), this method may have side-effects on
previously returned readers.

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-220c35b6bd86a16f1bb46993"></a>
## get_read

`function` · `parquet::file::reader::ChunkReader::get_read` · parquet 59.3.0

```rust
fn get_read(&self, start: u64) -> Result<Self::T>
```

Source: `src/file/reader.rs:75`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get a [`Read`] instance starting at the provided file offset

Returned readers follow the model of [`File::try_clone`] where mutations
of one reader affect all readers. Thus subsequent or concurrent calls to
[`Self::get_read`](../operations/parquet.file.reader.ChunkReader.md#op-220c35b6bd86a16f1bb46993) or [`Self::get_bytes`](../operations/parquet.file.reader.ChunkReader.md#op-22e04ef7e0e3a7dbd2b8e23e) may cause side-effects on
previously returned readers. Callers of `get_read` should take care
to avoid race conditions.

Unresolved upstream links (retained, not inferred): ``Read``, ``File::try_clone``.
