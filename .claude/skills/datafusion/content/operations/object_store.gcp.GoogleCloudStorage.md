# `object_store::gcp::GoogleCloudStorage`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.gcp.GoogleCloudStorage.json).

<a id="op-4b3bbb82170d36762c713ba6"></a>
## GoogleCloudStorage

`struct` · `object_store::gcp::GoogleCloudStorage` · object_store 0.13.2

```rust
struct GoogleCloudStorage
```

Source: `src/gcp/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Interface for [Google Cloud Storage](https://cloud.google.com/storage/).

<a id="op-ad51de072d96efcb6f7699c4"></a>
## abort_multipart

`function` · `object_store::gcp::GoogleCloudStorage::abort_multipart` · object_store 0.13.2

```rust
async fn abort_multipart(&self, path: &Path, id: &MultipartId) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [261, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/gcp/mod.rs:258`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc57769ada48c0b61a294541"></a>
## clone

`function` · `object_store::gcp::GoogleCloudStorage::clone` · object_store 0.13.2

```rust
fn clone(&self) -> GoogleCloudStorage
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 17], "end": [77, 22], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gcp/mod.rs:77`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9be5bc2bcd0dc522a0d9398f"></a>
## complete_multipart

`function` · `object_store::gcp::GoogleCloudStorage::complete_multipart` · object_store 0.13.2

```rust
async fn complete_multipart(&self, path: &Path, id: &MultipartId, parts: Vec<PartId>) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [261, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/gcp/mod.rs:249`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7dd964ab33f030099d278258"></a>
## copy_opts

`function` · `object_store::gcp::GoogleCloudStorage::copy_opts` · object_store 0.13.2

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [229, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/gcp/mod.rs:218`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66e450629de872955d31aaa1"></a>
## create_multipart

`function` · `object_store::gcp::GoogleCloudStorage::create_multipart` · object_store 0.13.2

```rust
async fn create_multipart(&self, path: &Path) -> Result<MultipartId>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [261, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/gcp/mod.rs:233`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95083787eb3daaee1c713881"></a>
## credentials

`function` · `object_store::gcp::GoogleCloudStorage::credentials` · object_store 0.13.2

```rust
fn credentials(&self) -> &GcpCredentialProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [102, 2], "filename": "src/gcp/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/mod.rs:94`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the [`GcpCredentialProvider`](../operations/object_store.gcp.GcpCredentialProvider.md#op-db800b581a1f03156ba6e5a6) used by [`GoogleCloudStorage`](../operations/object_store.gcp.GoogleCloudStorage.md#op-4b3bbb82170d36762c713ba6)

<a id="op-7768c431d71c4f337111fd21"></a>
## delete_stream

`function` · `object_store::gcp::GoogleCloudStorage::delete_stream` · object_store 0.13.2

```rust
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [229, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/gcp/mod.rs:184`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63a62c824d6f4bb9623d0f0b"></a>
## fmt

`function` · `object_store::gcp::GoogleCloudStorage::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 10], "end": [77, 15], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gcp/mod.rs:77`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-654159fe421d69de84f164bf"></a>
## fmt

`function` · `object_store::gcp::GoogleCloudStorage::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [90, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/gcp/mod.rs:83`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-991c00743e9294f2d878a4e9"></a>
## get_opts

`function` · `object_store::gcp::GoogleCloudStorage::get_opts` · object_store 0.13.2

```rust
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [229, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/gcp/mod.rs:180`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f01620e8e73b69892674ba98"></a>
## list

`function` · `object_store::gcp::GoogleCloudStorage::list` · object_store 0.13.2

```rust
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [229, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/gcp/mod.rs:202`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a76bfdf8e97e35e6a6c440e"></a>
## list_paginated

`function` · `object_store::gcp::GoogleCloudStorage::list_paginated` · object_store 0.13.2

```rust
async fn list_paginated(&self, prefix: Option<&str>, opts: PaginatedListOptions) -> Result<PaginatedListResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 1], "end": [300, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::list::PaginatedListStore", "path": "PaginatedListStore"}, "trait_path": "object_store::list::PaginatedListStore"}`

Source: `src/gcp/mod.rs:293`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d78190879383ccd876553d9"></a>
## list_with_delimiter

`function` · `object_store::gcp::GoogleCloudStorage::list_with_delimiter` · object_store 0.13.2

```rust
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [229, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/gcp/mod.rs:214`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-defee61521fd41a438023056"></a>
## list_with_offset

`function` · `object_store::gcp::GoogleCloudStorage::list_with_offset` · object_store 0.13.2

```rust
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [229, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/gcp/mod.rs:206`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55702deecdd1bfa97e5bc202"></a>
## put_multipart_opts

`function` · `object_store::gcp::GoogleCloudStorage::put_multipart_opts` · object_store 0.13.2

```rust
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [229, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/gcp/mod.rs:162`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1476a26ee55cef65bc30934f"></a>
## put_opts

`function` · `object_store::gcp::GoogleCloudStorage::put_opts` · object_store 0.13.2

```rust
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [229, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/gcp/mod.rs:153`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15630f2f9da56615585f1daf"></a>
## put_part

`function` · `object_store::gcp::GoogleCloudStorage::put_part` · object_store 0.13.2

```rust
async fn put_part(&self, path: &Path, id: &MultipartId, part_idx: usize, payload: PutPayload) -> Result<PartId>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [261, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/gcp/mod.rs:239`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7591b632ad5d6d4935ef6e9"></a>
## signed_url

`function` · `object_store::gcp::GoogleCloudStorage::signed_url` · object_store 0.13.2

```rust
async fn signed_url(&self, method: Method, path: &Path, expires_in: Duration) -> Result<Url>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [289, 2], "filename": "src/gcp/mod.rs"}, "trait": {"args": null, "id": "object_store::signer::Signer", "path": "Signer"}, "trait_path": "object_store::signer::Signer"}`

Source: `src/gcp/mod.rs:265`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5900fab97778aa39c9546e2a"></a>
## signing_credentials

`function` · `object_store::gcp::GoogleCloudStorage::signing_credentials` · object_store 0.13.2

```rust
fn signing_credentials(&self) -> &GcpSigningCredentialProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::GoogleCloudStorage", "path": "GoogleCloudStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [102, 2], "filename": "src/gcp/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/mod.rs:99`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the [`GcpSigningCredentialProvider`](../operations/object_store.gcp.GcpSigningCredentialProvider.md#op-ebbf5f6d923926254ad656ac) used by [`GoogleCloudStorage`](../operations/object_store.gcp.GoogleCloudStorage.md#op-4b3bbb82170d36762c713ba6)
