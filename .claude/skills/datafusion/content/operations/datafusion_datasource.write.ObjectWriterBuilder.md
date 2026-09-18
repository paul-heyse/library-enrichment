# `datafusion_datasource::write::ObjectWriterBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.write.ObjectWriterBuilder.json).

<a id="op-878d5a3d77e5c935a7f28cb7"></a>
## ObjectWriterBuilder

`struct` · `datafusion_datasource::write::ObjectWriterBuilder` · datafusion-datasource 55.1.0

```rust
struct ObjectWriterBuilder
```

Source: `src/write/mod.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A builder for an [`AsyncWrite`] that writes to an object store location.

This can be used to specify file compression on the writer. The writer
will have a default buffer size unless altered. The specific default size
is chosen by [`BufWriter::new`].

We drop the `AbortableWrite` struct and the writer will not try to cleanup on failure.
Users can configure automatic cleanup with their cloud provider.

Unresolved upstream links (retained, not inferred): ``AsyncWrite``, ``BufWriter::new``.

<a id="op-ff97c4e3b91f9250f0f9e5ce"></a>
## build

`function` · `datafusion_datasource::write::ObjectWriterBuilder::build` · datafusion-datasource 55.1.0

```rust
fn build(self) -> Result<Box<dyn AsyncWrite + Send + Unpin>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::write::ObjectWriterBuilder", "path": "ObjectWriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [232, 2], "filename": "src/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/write/mod.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return a writer object that writes to the object store location.

If a buffer size has not been set, the default buffer buffer size will
be used.

# Errors
If there is an error applying the compression type.

<a id="op-a8f2772a1bf68662893188d2"></a>
## fmt

`function` · `datafusion_datasource::write::ObjectWriterBuilder::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::write::ObjectWriterBuilder", "path": "ObjectWriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 10], "end": [108, 15], "filename": "src/write/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/write/mod.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bf1c083bb285c3012966aba"></a>
## get_buffer_size

`function` · `datafusion_datasource::write::ObjectWriterBuilder::get_buffer_size` · datafusion-datasource 55.1.0

```rust
fn get_buffer_size(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::write::ObjectWriterBuilder", "path": "ObjectWriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [232, 2], "filename": "src/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/write/mod.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Currently specified buffer size in bytes.

<a id="op-4a28ddf787944c2b1cf1f50f"></a>
## get_compression_level

`function` · `datafusion_datasource::write::ObjectWriterBuilder::get_compression_level` · datafusion-datasource 55.1.0

```rust
fn get_compression_level(&self) -> Option<u32>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::write::ObjectWriterBuilder", "path": "ObjectWriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [232, 2], "filename": "src/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/write/mod.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Currently specified compression level.

<a id="op-716a77aa49a1b9ca90b1acd9"></a>
## new

`function` · `datafusion_datasource::write::ObjectWriterBuilder::new` · datafusion-datasource 55.1.0

```rust
fn new(file_compression_type: FileCompressionType, location: &Path, object_store: Arc<dyn ObjectStore>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::write::ObjectWriterBuilder", "path": "ObjectWriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [232, 2], "filename": "src/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/write/mod.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a new [`ObjectWriterBuilder`](../operations/datafusion_datasource.write.ObjectWriterBuilder.md#op-878d5a3d77e5c935a7f28cb7) for the specified path and compression type.

<a id="op-a9dbba156d2ef5fbe6b13932"></a>
## set_buffer_size

`function` · `datafusion_datasource::write::ObjectWriterBuilder::set_buffer_size` · datafusion-datasource 55.1.0

```rust
fn set_buffer_size(&mut self, buffer_size: Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::write::ObjectWriterBuilder", "path": "ObjectWriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [232, 2], "filename": "src/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/write/mod.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set buffer size in bytes for object writer.

# Example
```
# use datafusion_datasource::file_compression_type::FileCompressionType;
# use datafusion_datasource::write::ObjectWriterBuilder;
# use object_store::memory::InMemory;
# use object_store::path::Path;
# use std::sync::Arc;
# let compression_type = FileCompressionType::UNCOMPRESSED;
# let location = Path::from("/foo/bar");
# let object_store = Arc::new(InMemory::new());
let mut builder = ObjectWriterBuilder::new(compression_type, &location, object_store);
builder.set_buffer_size(Some(20 * 1024 * 1024)); //20 MiB
assert_eq!(
    builder.get_buffer_size(),
    Some(20 * 1024 * 1024),
    "Internal error: Builder buffer size doesn't match"
);
```

<a id="op-6c2cda1b8e89dfe32a909b4e"></a>
## set_compression_level

`function` · `datafusion_datasource::write::ObjectWriterBuilder::set_compression_level` · datafusion-datasource 55.1.0

```rust
fn set_compression_level(&mut self, compression_level: Option<u32>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::write::ObjectWriterBuilder", "path": "ObjectWriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [232, 2], "filename": "src/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/write/mod.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set compression level for object writer.

<a id="op-d07b2a8a3eb21991f0cf4775"></a>
## with_buffer_size

`function` · `datafusion_datasource::write::ObjectWriterBuilder::with_buffer_size` · datafusion-datasource 55.1.0

```rust
fn with_buffer_size(self, buffer_size: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::write::ObjectWriterBuilder", "path": "ObjectWriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [232, 2], "filename": "src/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/write/mod.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set buffer size in bytes for object writer, returning the builder.

# Example
```
# use datafusion_datasource::file_compression_type::FileCompressionType;
# use datafusion_datasource::write::ObjectWriterBuilder;
# use object_store::memory::InMemory;
# use object_store::path::Path;
# use std::sync::Arc;
# let compression_type = FileCompressionType::UNCOMPRESSED;
# let location = Path::from("/foo/bar");
# let object_store = Arc::new(InMemory::new());
let builder = ObjectWriterBuilder::new(compression_type, &location, object_store)
    .with_buffer_size(Some(20 * 1024 * 1024)); //20 MiB
assert_eq!(
    builder.get_buffer_size(),
    Some(20 * 1024 * 1024),
    "Internal error: Builder buffer size doesn't match"
);
```

<a id="op-12ade8cb4b5d49ba96a3d7a6"></a>
## with_compression_level

`function` · `datafusion_datasource::write::ObjectWriterBuilder::with_compression_level` · datafusion-datasource 55.1.0

```rust
fn with_compression_level(self, compression_level: Option<u32>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::write::ObjectWriterBuilder", "path": "ObjectWriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [232, 2], "filename": "src/write/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/write/mod.rs:198`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set compression level for object writer, returning the builder.
