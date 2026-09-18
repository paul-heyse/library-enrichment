# `deltalake_aws::storage::S3StorageBackend`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.storage.S3StorageBackend.json).

<a id="op-06dee9d109146210a1b9e486"></a>
## S3StorageBackend

`struct` · `deltalake_aws::storage::S3StorageBackend` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct S3StorageBackend
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L193).

Source: `crates/aws/src/storage.rs:193`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

An S3 implementation of the [ObjectStore] trait

Unresolved upstream links (retained, not inferred): `ObjectStore`.

<a id="op-23d076b3d24e7c78a3eacd6f"></a>
## copy_opts

`function` · `deltalake_aws::storage::S3StorageBackend::copy_opts` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> ObjectStoreResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L285).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageBackend", "path": "S3StorageBackend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [302, 2], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/aws/src/storage.rs:285`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5aca64ea337e61dcdad3eca"></a>
## delete_stream

`function` · `deltalake_aws::storage::S3StorageBackend::delete_stream` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn delete_stream(&self, locations: BoxStream<'static, ObjectStoreResult<Path>>) -> BoxStream<'static, ObjectStoreResult<Path>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L262).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageBackend", "path": "S3StorageBackend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [302, 2], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/aws/src/storage.rs:262`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-567753d3cafd598dcfc607b4"></a>
## fmt

`function` · `deltalake_aws::storage::S3StorageBackend::fmt` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L222).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageBackend", "path": "S3StorageBackend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [229, 2], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/aws/src/storage.rs:222`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0af6b3d3963fb8abda8c2fa"></a>
## fmt

`function` · `deltalake_aws::storage::S3StorageBackend::fmt` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L200).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageBackend", "path": "S3StorageBackend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [207, 2], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/aws/src/storage.rs:200`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d63f0a6a5d28e2b0091e6c48"></a>
## get_opts

`function` · `deltalake_aws::storage::S3StorageBackend::get_opts` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_opts(&self, location: &Path, options: GetOptions) -> ObjectStoreResult<GetResult>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L250).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageBackend", "path": "S3StorageBackend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [302, 2], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/aws/src/storage.rs:250`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a1a93f1203f26b493c2a1a0"></a>
## get_ranges

`function` · `deltalake_aws::storage::S3StorageBackend::get_ranges` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> ObjectStoreResult<Vec<Bytes>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L254).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageBackend", "path": "S3StorageBackend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [302, 2], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/aws/src/storage.rs:254`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b25ab7c523e01bde988a28a"></a>
## list

`function` · `deltalake_aws::storage::S3StorageBackend::list` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, ObjectStoreResult<ObjectMeta>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L269).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageBackend", "path": "S3StorageBackend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [302, 2], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/aws/src/storage.rs:269`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0e728ba7352c55153033751"></a>
## list_with_delimiter

`function` · `deltalake_aws::storage::S3StorageBackend::list_with_delimiter` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> ObjectStoreResult<ListResult>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L281).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageBackend", "path": "S3StorageBackend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [302, 2], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/aws/src/storage.rs:281`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c778564881cf2171e771efb4"></a>
## list_with_offset

`function` · `deltalake_aws::storage::S3StorageBackend::list_with_offset` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, ObjectStoreResult<ObjectMeta>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L273).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageBackend", "path": "S3StorageBackend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [302, 2], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/aws/src/storage.rs:273`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6278d79dfa395ec343069f02"></a>
## put_multipart_opts

`function` · `deltalake_aws::storage::S3StorageBackend::put_multipart_opts` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn put_multipart_opts(&self, location: &Path, options: PutMultipartOptions) -> ObjectStoreResult<Box<dyn MultipartUpload>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L242).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageBackend", "path": "S3StorageBackend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [302, 2], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/aws/src/storage.rs:242`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-636c6f5164e3f00f4c7499e1"></a>
## put_opts

`function` · `deltalake_aws::storage::S3StorageBackend::put_opts` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn put_opts(&self, location: &Path, bytes: PutPayload, options: PutOptions) -> ObjectStoreResult<PutResult>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L233).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageBackend", "path": "S3StorageBackend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [302, 2], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/aws/src/storage.rs:233`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d118a38b80936b1f4c1cc30e"></a>
## rename_opts

`function` · `deltalake_aws::storage::S3StorageBackend::rename_opts` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> ObjectStoreResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L294).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageBackend", "path": "S3StorageBackend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [302, 2], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/aws/src/storage.rs:294`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4056e6ef790188a948c7cb0f"></a>
## try_new

`function` · `deltalake_aws::storage::S3StorageBackend::try_new` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(storage: ObjectStoreRef, allow_unsafe_rename: bool) -> ObjectStoreResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L213).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageBackend", "path": "S3StorageBackend"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [219, 2], "filename": "crates/aws/src/storage.rs"}, "trait": null, "trait_path": null}`

Source: `crates/aws/src/storage.rs:213`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates a new S3StorageBackend.

Options are described in [constants](../modules/deltalake_aws.constants.md#op-1da55f2e5231c3610d5e2034).

<a id="op-16d679d7aa522aad75530562"></a>
## allow_unsafe_rename

`struct_field` · `deltalake_aws::storage::S3StorageBackend::allow_unsafe_rename` · deltalake-aws 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
allow_unsafe_rename: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L196).

Source: `crates/aws/src/storage.rs:196`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether allowed to performance rename_if_not_exist as rename

<a id="op-f2fe9d9fc1377f62aeaefaed"></a>
## inner

`struct_field` · `deltalake_aws::storage::S3StorageBackend::inner` · deltalake-aws 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: deltalake_core::logstore::ObjectStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L194).

Source: `crates/aws/src/storage.rs:194`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
