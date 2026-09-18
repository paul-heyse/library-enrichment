# `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.storage.runtime.DeltaIOStorageBackend.json).

<a id="op-6d0947cffcb6eb0c6aca2e05"></a>
## DeltaIOStorageBackend

`struct` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaIOStorageBackend<T: ObjectStore + Clone>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L111).

Source: `crates/core/src/logstore/storage/runtime.rs:111`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Wraps any object store and runs IO in it's own runtime [EXPERIMENTAL]

<a id="op-4a7219a11cf9f4330fe83067"></a>
## clone

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeltaIOStorageBackend<T>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L110).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 10], "end": [110, 15], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/logstore/storage/runtime.rs:110`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2a6cb1a4b0d913ce811faf1"></a>
## copy_opts

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::copy_opts` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> ObjectStoreResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L272).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 1], "end": [314, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/core/src/logstore/storage/runtime.rs:272`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6483cf8c0d98fe574bbf3186"></a>
## delete_stream

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::delete_stream` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn delete_stream(&self, locations: BoxStream<'static, ObjectStoreResult<Path>>) -> BoxStream<'static, ObjectStoreResult<Path>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L232).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 1], "end": [314, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/core/src/logstore/storage/runtime.rs:232`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d07282f09b928b238be8c412"></a>
## fmt

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L185).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [184, 1], "end": [188, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/core/src/logstore/storage/runtime.rs:185`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f07cbd904585b2444cb681f2"></a>
## fmt

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L179).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [182, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/logstore/storage/runtime.rs:179`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c5106fed4e01c928982700d"></a>
## get_opts

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::get_opts` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_opts(&self, location: &Path, options: GetOptions) -> ObjectStoreResult<GetResult>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L206).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 1], "end": [314, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/core/src/logstore/storage/runtime.rs:206`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eab114c8a87986c83d358f81"></a>
## get_ranges

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::get_ranges` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> ObjectStoreResult<Vec<Bytes>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L215).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 1], "end": [314, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/core/src/logstore/storage/runtime.rs:215`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15e774984575a4a01c0002f6"></a>
## inner

`struct_field` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::inner` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
inner: T
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L113).

Source: `crates/core/src/logstore/storage/runtime.rs:113`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The wrapped object store that performs the actual IO.

<a id="op-de89d576e5d474074191a5aa"></a>
## list

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::list` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, ObjectStoreResult<ObjectMeta>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L256).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 1], "end": [314, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/core/src/logstore/storage/runtime.rs:256`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acd4b7376eb85a9d56b96850"></a>
## list_with_delimiter

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::list_with_delimiter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> ObjectStoreResult<ListResult>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L268).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 1], "end": [314, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/core/src/logstore/storage/runtime.rs:268`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e144b55e51f4f55d3915f5b"></a>
## list_with_offset

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::list_with_offset` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, ObjectStoreResult<ObjectMeta>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L260).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 1], "end": [314, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/core/src/logstore/storage/runtime.rs:260`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b00f40f10f45649802f8e4b"></a>
## new

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(store: T, rt: IORuntime) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L122).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [117, 1], "end": [125, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/storage/runtime.rs:122`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Wrap `store` so that its IO operations are executed on the dedicated runtime `rt`.

<a id="op-aef9ae11c4d79ef47887d00e"></a>
## put_multipart_opts

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::put_multipart_opts` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn put_multipart_opts(&self, location: &Path, options: PutMultipartOptions) -> ObjectStoreResult<Box<dyn MultipartUpload>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L302).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 1], "end": [314, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/core/src/logstore/storage/runtime.rs:302`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3812501e5d5a5dd17be8d05d"></a>
## put_opts

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::put_opts` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn put_opts(&self, location: &Path, bytes: PutPayload, options: PutOptions) -> ObjectStoreResult<PutResult>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L192).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 1], "end": [314, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/core/src/logstore/storage/runtime.rs:192`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-705020a6c9c314ec44aa5c2b"></a>
## rename_opts

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::rename_opts` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> ObjectStoreResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L287).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 1], "end": [314, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/core/src/logstore/storage/runtime.rs:287`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb87ca8bc8cb1c2c52d4083e"></a>
## spawn_io_rt

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::spawn_io_rt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn spawn_io_rt<F, O>(&self, f: F, store: &T, path: Path) -> BoxFuture<'_, ObjectStoreResult<O>> where F: for<'a> FnOnce(&'a T, &'a Path) -> BoxFuture<'a, ObjectStoreResult<O>> + Send + 'static, O: Send + 'static
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L129).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [176, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/storage/runtime.rs:129`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

spawn tasks on IO runtime

<a id="op-3df6c459c3b224e186919c16"></a>
## spawn_io_rt_from_to

`function` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::spawn_io_rt_from_to` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn spawn_io_rt_from_to<F, O>(&self, f: F, store: &T, from: Path, to: Path) -> BoxFuture<'_, ObjectStoreResult<O>> where F: for<'a> FnOnce(&'a T, &'a Path, &'a Path) -> BoxFuture<'a, ObjectStoreResult<O>> + Send + 'static, O: Send + 'static
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L152).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend", "path": "DeltaIOStorageBackend"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [176, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/storage/runtime.rs:152`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

spawn tasks on IO runtime

<a id="op-f8adc71fa4f711d2cfb8c364"></a>
## rt

`struct_field` · `deltalake_core::logstore::storage::runtime::DeltaIOStorageBackend::rt` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
rt: IORuntime
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L114).

Source: `crates/core/src/logstore/storage/runtime.rs:114`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
