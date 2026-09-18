# `object_store::chunked::ChunkedStore`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.chunked.ChunkedStore.json).

<a id="op-b2eebc76c44478d71e96fcb7"></a>
## ChunkedStore

`struct` · `object_store::chunked::ChunkedStore` · object_store 0.13.2

```rust
struct ChunkedStore
```

Source: `src/chunked.rs:45`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Wraps a [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) and makes its get response return chunks
in a controllable manner.

A `ChunkedStore` makes the memory consumption and performance of
the wrapped [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) worse. It is intended for use within
tests, to control the chunks in the produced output streams. For
example, it is used to verify the delimiting logic in
newline_delimited_stream.

<a id="op-85c488a04ccbaada68f3fe5e"></a>
## copy_opts

`function` · `object_store::chunked::ChunkedStore::copy_opts` · object_store 0.13.2

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::chunked::ChunkedStore", "path": "ChunkedStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [172, 2], "filename": "src/chunked.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/chunked.rs:165`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf0820af3c5b8fde2fcf02e1"></a>
## delete_stream

`function` · `object_store::chunked::ChunkedStore::delete_stream` · object_store 0.13.2

```rust
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::chunked::ChunkedStore", "path": "ChunkedStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [172, 2], "filename": "src/chunked.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/chunked.rs:142`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d928ddc5576068b1fbca79f"></a>
## fmt

`function` · `object_store::chunked::ChunkedStore::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::chunked::ChunkedStore", "path": "ChunkedStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [61, 2], "filename": "src/chunked.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/chunked.rs:58`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea5aea3ff26dd2fe19b9ec81"></a>
## fmt

`function` · `object_store::chunked::ChunkedStore::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::chunked::ChunkedStore", "path": "ChunkedStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "src/chunked.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/chunked.rs:44`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bbb53faa9e4b46d0fbca583"></a>
## get_opts

`function` · `object_store::chunked::ChunkedStore::get_opts` · object_store 0.13.2

```rust
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::chunked::ChunkedStore", "path": "ChunkedStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [172, 2], "filename": "src/chunked.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/chunked.rs:83`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38baefc2521628e3e4307cd6"></a>
## get_ranges

`function` · `object_store::chunked::ChunkedStore::get_ranges` · object_store 0.13.2

```rust
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> Result<Vec<Bytes>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::chunked::ChunkedStore", "path": "ChunkedStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [172, 2], "filename": "src/chunked.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/chunked.rs:138`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35723082d50a917a4019f071"></a>
## list

`function` · `object_store::chunked::ChunkedStore::list` · object_store 0.13.2

```rust
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::chunked::ChunkedStore", "path": "ChunkedStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [172, 2], "filename": "src/chunked.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/chunked.rs:149`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-575f45eeb13bab5a28369c26"></a>
## list_with_delimiter

`function` · `object_store::chunked::ChunkedStore::list_with_delimiter` · object_store 0.13.2

```rust
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::chunked::ChunkedStore", "path": "ChunkedStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [172, 2], "filename": "src/chunked.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/chunked.rs:161`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e95bb3dcf08e38aee7a93964"></a>
## list_with_offset

`function` · `object_store::chunked::ChunkedStore::list_with_offset` · object_store 0.13.2

```rust
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::chunked::ChunkedStore", "path": "ChunkedStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [172, 2], "filename": "src/chunked.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/chunked.rs:153`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4600d33f2e0db6b1fcca2ea1"></a>
## new

`function` · `object_store::chunked::ChunkedStore::new` · object_store 0.13.2

```rust
fn new(inner: Arc<dyn ObjectStore>, chunk_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::chunked::ChunkedStore", "path": "ChunkedStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [55, 2], "filename": "src/chunked.rs"}, "trait": null, "trait_path": null}`

Source: `src/chunked.rs:52`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Creates a new [`ChunkedStore`](../operations/object_store.chunked.ChunkedStore.md#op-b2eebc76c44478d71e96fcb7) with the specified chunk_size

<a id="op-3310fd5939703a72e30addc0"></a>
## put_multipart_opts

`function` · `object_store::chunked::ChunkedStore::put_multipart_opts` · object_store 0.13.2

```rust
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::chunked::ChunkedStore", "path": "ChunkedStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [172, 2], "filename": "src/chunked.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/chunked.rs:75`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-381af43eb4f67900c8682d2e"></a>
## put_opts

`function` · `object_store::chunked::ChunkedStore::put_opts` · object_store 0.13.2

```rust
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::chunked::ChunkedStore", "path": "ChunkedStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [172, 2], "filename": "src/chunked.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/chunked.rs:66`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae5f43f304abf5579a11998b"></a>
## rename_opts

`function` · `object_store::chunked::ChunkedStore::rename_opts` · object_store 0.13.2

```rust
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::chunked::ChunkedStore", "path": "ChunkedStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [172, 2], "filename": "src/chunked.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/chunked.rs:169`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
