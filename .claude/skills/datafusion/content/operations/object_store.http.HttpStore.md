# `object_store::http::HttpStore`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.http.HttpStore.json).

<a id="op-547611b00a8e9d96e730371c"></a>
## HttpStore

`struct` · `object_store::http::HttpStore` · object_store 0.13.2

```rust
struct HttpStore
```

Source: `src/http/mod.rs:87`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) implementation for generic HTTP servers

See [`crate::http`](../modules/object_store.http.md#op-864ea065fc3b64d53597d706) for more information

<a id="op-4fddfea6010119d421147dc2"></a>
## copy_opts

`function` · `object_store::http::HttpStore::copy_opts` · object_store 0.13.2

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpStore", "path": "HttpStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [226, 2], "filename": "src/http/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/http/mod.rs:215`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-227ccdae6d0afa2471e60bd5"></a>
## delete_stream

`function` · `object_store::http::HttpStore::delete_stream` · object_store 0.13.2

```rust
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpStore", "path": "HttpStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [226, 2], "filename": "src/http/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/http/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b197464147ad74435243c868"></a>
## fmt

`function` · `object_store::http::HttpStore::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpStore", "path": "HttpStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [95, 2], "filename": "src/http/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/http/mod.rs:92`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5ea9dc48001f8a14209de7f"></a>
## fmt

`function` · `object_store::http::HttpStore::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpStore", "path": "HttpStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 10], "end": [86, 15], "filename": "src/http/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/http/mod.rs:86`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03cc6c24eb307e931b017000"></a>
## get_opts

`function` · `object_store::http::HttpStore::get_opts` · object_store 0.13.2

```rust
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpStore", "path": "HttpStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [226, 2], "filename": "src/http/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/http/mod.rs:137`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c64e1b818b2ff8a9d5f51fe"></a>
## list

`function` · `object_store::http::HttpStore::list` · object_store 0.13.2

```rust
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpStore", "path": "HttpStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [226, 2], "filename": "src/http/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/http/mod.rs:159`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4368f13974f0e2986b03d6a3"></a>
## list_with_delimiter

`function` · `object_store::http::HttpStore::list_with_delimiter` · object_store 0.13.2

```rust
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpStore", "path": "HttpStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [226, 2], "filename": "src/http/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/http/mod.rs:183`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79016d9e13e6590767de9ef7"></a>
## put_multipart_opts

`function` · `object_store::http::HttpStore::put_multipart_opts` · object_store 0.13.2

```rust
async fn put_multipart_opts(&self, _location: &Path, _opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpStore", "path": "HttpStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [226, 2], "filename": "src/http/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/http/mod.rs:126`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25674b8523998318558880f6"></a>
## put_opts

`function` · `object_store::http::HttpStore::put_opts` · object_store 0.13.2

```rust
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::http::HttpStore", "path": "HttpStore"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [226, 2], "filename": "src/http/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/http/mod.rs:99`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
