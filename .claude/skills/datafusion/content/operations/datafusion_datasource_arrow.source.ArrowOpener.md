# `datafusion_datasource_arrow::source::ArrowOpener`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_arrow.source.ArrowOpener.json).

<a id="op-8ba81f3533ff069b4d18df8e"></a>
## ArrowOpener

`struct` · `datafusion_datasource_arrow::source::ArrowOpener` · datafusion-datasource-arrow 55.1.0

```rust
struct ArrowOpener
```

Source: `src/source.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

`FileOpener` wrapper for both Arrow IPC file and stream formats

<a id="op-42174a6274d4ee6786b4c740"></a>
## inner

`struct_field` · `datafusion_datasource_arrow::source::ArrowOpener::inner` · datafusion-datasource-arrow 55.1.0

```rust
inner: std::sync::Arc<dyn FileOpener>
```

Source: `src/source.rs:467`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8759a1675a8b26e3496c86f0"></a>
## new

`function` · `datafusion_datasource_arrow::source::ArrowOpener::new` · datafusion-datasource-arrow 55.1.0

```rust
fn new(inner: Arc<dyn FileOpener>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowOpener", "path": "ArrowOpener"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [476, 1], "end": [505, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:478`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

Creates a new [`ArrowOpener`](../operations/datafusion_datasource_arrow.source.ArrowOpener.md#op-8ba81f3533ff069b4d18df8e)

<a id="op-60305a8b190d8d12d6ab507e"></a>
## new_file_opener

`function` · `datafusion_datasource_arrow::source::ArrowOpener::new_file_opener` · datafusion-datasource-arrow 55.1.0

```rust
fn new_file_opener(object_store: Arc<dyn ObjectStore>, projection: Option<Vec<usize>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowOpener", "path": "ArrowOpener"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [476, 1], "end": [505, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbba35714a564ede11539214"></a>
## new_stream_file_opener

`function` · `datafusion_datasource_arrow::source::ArrowOpener::new_stream_file_opener` · datafusion-datasource-arrow 55.1.0

```rust
fn new_stream_file_opener(object_store: Arc<dyn ObjectStore>, projection: Option<Vec<usize>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowOpener", "path": "ArrowOpener"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [476, 1], "end": [505, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:494`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e61928b07746786c633a8a03"></a>
## open

`function` · `datafusion_datasource_arrow::source::ArrowOpener::open` · datafusion-datasource-arrow 55.1.0

```rust
fn open(&self, partitioned_file: PartitionedFile) -> Result<FileOpenFuture>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_arrow::source::ArrowOpener", "path": "ArrowOpener"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [470, 1], "end": [474, 2], "filename": "src/source.rs"}, "trait": {"args": null, "id": "datafusion_datasource::file_stream::FileOpener", "path": "FileOpener"}, "trait_path": "datafusion_datasource::file_stream::FileOpener"}`

Source: `src/source.rs:471`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-arrow/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
