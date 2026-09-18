# `object_store::memory::InMemory`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.memory.InMemory.json).

<a id="op-2b004086fc4faa5587daef80"></a>
## InMemory

`struct` · `object_store::memory::InMemory` · object_store 0.13.2

```rust
struct InMemory
```

Source: `src/memory.rs:82`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

In-memory storage suitable for testing or for opting out of using a cloud
storage provider.

<a id="op-943a37f281c11c97fce33441"></a>
## abort_multipart

`function` · `object_store::memory::InMemory::abort_multipart` · object_store 0.13.2

```rust
async fn abort_multipart(&self, _path: &Path, id: &MultipartId) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [473, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/memory.rs:469`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9bacb9ead4650406447455e"></a>
## clone

`function` · `object_store::memory::InMemory::clone` · object_store 0.13.2

```rust
fn clone(&self) -> InMemory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 26], "end": [81, 31], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/memory.rs:81`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a8bd30732d1dc8b0f725231"></a>
## complete_multipart

`function` · `object_store::memory::InMemory::complete_multipart` · object_store 0.13.2

```rust
async fn complete_multipart(&self, path: &Path, id: &MultipartId, _parts: Vec<PartId>) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [473, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/memory.rs:445`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-957cfca925431bd72b0f4f1a"></a>
## copy_opts

`function` · `object_store::memory::InMemory::copy_opts` · object_store 0.13.2

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [415, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/memory.rs:389`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ee5e71e109995a216d147fd"></a>
## create_multipart

`function` · `object_store::memory::InMemory::create_multipart` · object_store 0.13.2

```rust
async fn create_multipart(&self, _path: &Path) -> Result<MultipartId>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [473, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/memory.rs:419`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6292f7926637ecb5a397a9f8"></a>
## default

`function` · `object_store::memory::InMemory::default` · object_store 0.13.2

```rust
fn default() -> InMemory
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 17], "end": [81, 24], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/memory.rs:81`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc92b21a595b3eb22605a615"></a>
## delete_stream

`function` · `object_store::memory::InMemory::delete_stream` · object_store 0.13.2

```rust
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [415, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/memory.rs:297`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2612d8594003075ce8d50a77"></a>
## fmt

`function` · `object_store::memory::InMemory::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [193, 1], "end": [197, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/memory.rs:194`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1bce6a702f2083514dee7fa"></a>
## fmt

`function` · `object_store::memory::InMemory::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 10], "end": [81, 15], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory.rs:81`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c334aadaf969252afbe42f9"></a>
## fork

`function` · `object_store::memory::InMemory::fork` · object_store 0.13.2

```rust
fn fork(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [475, 1], "end": [501, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:483`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Creates a fork of the store, with the current content copied into the
new store.

<a id="op-ba24bd9a50226eaaef52c740"></a>
## get_opts

`function` · `object_store::memory::InMemory::get_opts` · object_store 0.13.2

```rust
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [415, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/memory.rs:237`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f5e69f8a8bdaf9724380826"></a>
## get_ranges

`function` · `object_store::memory::InMemory::get_ranges` · object_store 0.13.2

```rust
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> Result<Vec<Bytes>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [415, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/memory.rs:272`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-529131587d5d0fd6ba07a634"></a>
## list

`function` · `object_store::memory::InMemory::list` · object_store 0.13.2

```rust
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [415, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/memory.rs:311`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3f0ffc9d10acb726dab05aa"></a>
## list_with_delimiter

`function` · `object_store::memory::InMemory::list_with_delimiter` · object_store 0.13.2

```rust
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [415, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/memory.rs:343`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The memory implementation returns all results, as opposed to the cloud
versions which limit their results to 1k or more because of API
limitations.

<a id="op-f15a89c9f76e6c87d78f241f"></a>
## new

`function` · `object_store::memory::InMemory::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [475, 1], "end": [501, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:477`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create new in-memory storage.

<a id="op-41bbb473eb4dfdb7663caabb"></a>
## put_multipart_opts

`function` · `object_store::memory::InMemory::put_multipart_opts` · object_store 0.13.2

```rust
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [415, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/memory.rs:224`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5fdae237e243fbd68a64941"></a>
## put_opts

`function` · `object_store::memory::InMemory::put_opts` · object_store 0.13.2

```rust
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [415, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/memory.rs:201`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffa032041d0d27e530995dd3"></a>
## put_part

`function` · `object_store::memory::InMemory::put_part` · object_store 0.13.2

```rust
async fn put_part(&self, _path: &Path, id: &MultipartId, part_idx: usize, payload: PutPayload) -> Result<PartId>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::memory::InMemory", "path": "InMemory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [473, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/memory.rs:427`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
