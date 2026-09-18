# `object_store::prefix::PrefixStore`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.prefix.PrefixStore.json).

<a id="op-5d713d1af8bc1816cc1c9a19"></a>
## PrefixStore

`struct` · `object_store::prefix::PrefixStore` · object_store 0.13.2

```rust
struct PrefixStore<T>
```

Source: `src/prefix.rs:32`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Store wrapper that applies a constant prefix to all paths handled by the store.

<a id="op-fa0f44a84ffbf1b8d0473530"></a>
## abort_multipart

`function` · `object_store::prefix::PrefixStore::abort_multipart` · object_store 0.13.2

```rust
async fn abort_multipart(&self, path: &Path, id: &MultipartId) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 1], "end": [226, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/prefix.rs:222`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79a7c6b655542259c989ba66"></a>
## clone

`function` · `object_store::prefix::PrefixStore::clone` · object_store 0.13.2

```rust
fn clone(&self) -> PrefixStore<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 22], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/prefix.rs:31`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66b267e68001784d6f7d3059"></a>
## complete_multipart

`function` · `object_store::prefix::PrefixStore::complete_multipart` · object_store 0.13.2

```rust
async fn complete_multipart(&self, path: &Path, id: &MultipartId, parts: Vec<PartId>) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 1], "end": [226, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/prefix.rs:212`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9022a0a50010a9ed1e1a390"></a>
## copy_opts

`function` · `object_store::prefix::PrefixStore::copy_opts` · object_store 0.13.2

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [192, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/prefix.rs:181`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36fc6ce2059712de6090a0a1"></a>
## create_multipart

`function` · `object_store::prefix::PrefixStore::create_multipart` · object_store 0.13.2

```rust
async fn create_multipart(&self, path: &Path) -> Result<MultipartId>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 1], "end": [226, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/prefix.rs:196`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c564fe97e6d5f96a730e19b"></a>
## delete_stream

`function` · `object_store::prefix::PrefixStore::delete_stream` · object_store 0.13.2

```rust
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [192, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/prefix.rs:128`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02da1ade9231bcaa1c972e59"></a>
## fmt

`function` · `object_store::prefix::PrefixStore::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/prefix.rs:31`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-830a085b3ea875bfa5158ede"></a>
## fmt

`function` · `object_store::prefix::PrefixStore::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [41, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/prefix.rs:38`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-310cba49601451f0ade278fe"></a>
## get_opts

`function` · `object_store::prefix::PrefixStore::get_opts` · object_store 0.13.2

```rust
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [192, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/prefix.rs:118`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd6852d409a9a0e041c775b0"></a>
## get_ranges

`function` · `object_store::prefix::PrefixStore::get_ranges` · object_store 0.13.2

```rust
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> Result<Vec<Bytes>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [192, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/prefix.rs:123`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-433abf44728b70829d0a16c8"></a>
## list

`function` · `object_store::prefix::PrefixStore::list` · object_store 0.13.2

```rust
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [192, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/prefix.rs:143`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a8215c99ba90e9921d87dbe"></a>
## list_with_delimiter

`function` · `object_store::prefix::PrefixStore::list_with_delimiter` · object_store 0.13.2

```rust
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [192, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/prefix.rs:162`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bc759efcd66ff63cd210ba4"></a>
## list_with_offset

`function` · `object_store::prefix::PrefixStore::list_with_offset` · object_store 0.13.2

```rust
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [192, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/prefix.rs:150`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44187e1a4029b001c7fb9881"></a>
## new

`function` · `object_store::prefix::PrefixStore::new` · object_store 0.13.2

```rust
fn new(store: T, prefix: impl Into<Path>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [66, 2], "filename": "src/prefix.rs"}, "trait": null, "trait_path": null}`

Source: `src/prefix.rs:45`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new instance of [`PrefixStore`](../operations/object_store.prefix.PrefixStore.md#op-5d713d1af8bc1816cc1c9a19)

<a id="op-92e0bcb25f641027a2ff555d"></a>
## put_multipart_opts

`function` · `object_store::prefix::PrefixStore::put_multipart_opts` · object_store 0.13.2

```rust
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [192, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/prefix.rs:109`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bac08775ff94d4a69b35227d"></a>
## put_opts

`function` · `object_store::prefix::PrefixStore::put_opts` · object_store 0.13.2

```rust
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [192, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/prefix.rs:99`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7236e6d52cc4bf329ad64c12"></a>
## put_part

`function` · `object_store::prefix::PrefixStore::put_part` · object_store 0.13.2

```rust
async fn put_part(&self, path: &Path, id: &MultipartId, part_idx: usize, data: PutPayload) -> Result<PartId>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 1], "end": [226, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/prefix.rs:201`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-519c9ab4dea6cbc46b319507"></a>
## rename_opts

`function` · `object_store::prefix::PrefixStore::rename_opts` · object_store 0.13.2

```rust
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::prefix::PrefixStore", "path": "PrefixStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [192, 2], "filename": "src/prefix.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/prefix.rs:187`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
