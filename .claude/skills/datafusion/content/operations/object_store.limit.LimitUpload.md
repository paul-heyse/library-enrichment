# `object_store::limit::LimitUpload`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.limit.LimitUpload.json).

<a id="op-abb9f86b3b3c56d3f2de738e"></a>
## LimitUpload

`struct` · `object_store::limit::LimitUpload` · object_store 0.13.2

```rust
struct LimitUpload
```

Source: `src/limit.rs:205`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An [`MultipartUpload`](../operations/object_store.upload.MultipartUpload.md#op-d1bf78ac32fe2ad950a9d5d0) wrapper that limits the maximum number of concurrent requests

<a id="op-c55f4488e4e10494969208af"></a>
## abort

`function` · `object_store::limit::LimitUpload::abort` · object_store 0.13.2

```rust
async fn abort(&mut self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::limit::LimitUpload", "path": "LimitUpload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [240, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "object_store::upload::MultipartUpload", "path": "MultipartUpload"}, "trait_path": "object_store::upload::MultipartUpload"}`

Source: `src/limit.rs:236`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e73ba0c3381fe16bb93eb6d"></a>
## complete

`function` · `object_store::limit::LimitUpload::complete` · object_store 0.13.2

```rust
async fn complete(&mut self) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::limit::LimitUpload", "path": "LimitUpload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [240, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "object_store::upload::MultipartUpload", "path": "MultipartUpload"}, "trait_path": "object_store::upload::MultipartUpload"}`

Source: `src/limit.rs:231`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e48a27e25495f19b85ae7d2"></a>
## fmt

`function` · `object_store::limit::LimitUpload::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::limit::LimitUpload", "path": "LimitUpload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 10], "end": [204, 15], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/limit.rs:204`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-deb30dbd802909a2af4b0e1a"></a>
## new

`function` · `object_store::limit::LimitUpload::new` · object_store 0.13.2

```rust
fn new(upload: Box<dyn MultipartUpload>, max_concurrency: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::limit::LimitUpload", "path": "LimitUpload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [218, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:212`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`LimitUpload`](../operations/object_store.limit.LimitUpload.md#op-abb9f86b3b3c56d3f2de738e) limiting `upload` to `max_concurrency` concurrent requests

<a id="op-743a1f25dcdc927bf20abafc"></a>
## put_part

`function` · `object_store::limit::LimitUpload::put_part` · object_store 0.13.2

```rust
fn put_part(&mut self, data: PutPayload) -> UploadPart
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::limit::LimitUpload", "path": "LimitUpload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [240, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "object_store::upload::MultipartUpload", "path": "MultipartUpload"}, "trait_path": "object_store::upload::MultipartUpload"}`

Source: `src/limit.rs:222`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
