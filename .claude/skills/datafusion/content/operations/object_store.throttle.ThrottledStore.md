# `object_store::throttle::ThrottledStore`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.throttle.ThrottledStore.json).

<a id="op-73d91dbb4d63914bc01deccf"></a>
## ThrottledStore

`struct` · `object_store::throttle::ThrottledStore` · object_store 0.13.2

```rust
struct ThrottledStore<T>
```

Source: `src/throttle.rs:117`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Store wrapper that wraps an inner store with some `sleep` calls.

This can be used for performance testing.

**Note that the behavior of the wrapper is deterministic and might not reflect real-world
conditions!**

<a id="op-5547ded4daf22f29d4a8acf9"></a>
## abort_multipart

`function` · `object_store::throttle::ThrottledStore::abort_multipart` · object_store 0.13.2

```rust
async fn abort_multipart(&self, path: &Path, id: &MultipartId) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [336, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/throttle.rs:333`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fdbe95bddc809b6d2553eec"></a>
## complete_multipart

`function` · `object_store::throttle::ThrottledStore::complete_multipart` · object_store 0.13.2

```rust
async fn complete_multipart(&self, path: &Path, id: &MultipartId, parts: Vec<PartId>) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [336, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/throttle.rs:324`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ecdf7993a24c1622f0652ce"></a>
## config

`function` · `object_store::throttle::ThrottledStore::config` · object_store 0.13.2

```rust
fn config(&self) -> ThrottleConfig
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [144, 2], "filename": "src/throttle.rs"}, "trait": null, "trait_path": null}`

Source: `src/throttle.rs:141`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Return copy of current config.

<a id="op-8860d3539c74cfda13c5e66c"></a>
## config_mut

`function` · `object_store::throttle::ThrottledStore::config_mut` · object_store 0.13.2

```rust
fn config_mut<F>(&self, f: F) where F: Fn(&mut ThrottleConfig)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [144, 2], "filename": "src/throttle.rs"}, "trait": null, "trait_path": null}`

Source: `src/throttle.rs:132`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Mutate config.

<a id="op-36993698c6ffcc3b912f66a7"></a>
## copy_opts

`function` · `object_store::throttle::ThrottledStore::copy_opts` · object_store 0.13.2

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [266, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/throttle.rs:255`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-124daa15166c8f5b7a6f521b"></a>
## create_multipart

`function` · `object_store::throttle::ThrottledStore::create_multipart` · object_store 0.13.2

```rust
async fn create_multipart(&self, path: &Path) -> Result<MultipartId>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [336, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/throttle.rs:309`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fde148a2c1028f63e5675c37"></a>
## delete_stream

`function` · `object_store::throttle::ThrottledStore::delete_stream` · object_store 0.13.2

```rust
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [266, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/throttle.rs:199`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10770e44e0761fb22d041c7f"></a>
## fmt

`function` · `object_store::throttle::ThrottledStore::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 10], "end": [116, 15], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/throttle.rs:116`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8d0878636c0a2fd67ddb8c6"></a>
## fmt

`function` · `object_store::throttle::ThrottledStore::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [150, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/throttle.rs:147`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e69360d5477a1b64ac6ce0cd"></a>
## get_opts

`function` · `object_store::throttle::ThrottledStore::get_opts` · object_store 0.13.2

```rust
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [266, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/throttle.rs:177`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce1d23d461f486e053dd4765"></a>
## get_ranges

`function` · `object_store::throttle::ThrottledStore::get_ranges` · object_store 0.13.2

```rust
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> Result<Vec<Bytes>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [266, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/throttle.rs:187`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc6926fb80c8fdd27ee0d7fe"></a>
## list

`function` · `object_store::throttle::ThrottledStore::list` · object_store 0.13.2

```rust
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [266, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/throttle.rs:212`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32d4ee0fe4b892fac19e6c17"></a>
## list_with_delimiter

`function` · `object_store::throttle::ThrottledStore::list_with_delimiter` · object_store 0.13.2

```rust
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [266, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/throttle.rs:242`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf83f80b99a91616a9c8d309"></a>
## list_with_offset

`function` · `object_store::throttle::ThrottledStore::list_with_offset` · object_store 0.13.2

```rust
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [266, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/throttle.rs:225`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a056eca9a2b29223e4fd6468"></a>
## new

`function` · `object_store::throttle::ThrottledStore::new` · object_store 0.13.2

```rust
fn new(inner: T, config: ThrottleConfig) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [144, 2], "filename": "src/throttle.rs"}, "trait": null, "trait_path": null}`

Source: `src/throttle.rs:124`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create new wrapper with zero waiting times.

<a id="op-aef000c540d999650e2bd02e"></a>
## put_multipart_opts

`function` · `object_store::throttle::ThrottledStore::put_multipart_opts` · object_store 0.13.2

```rust
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [266, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/throttle.rs:165`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-197f002062c820c1df53269e"></a>
## put_opts

`function` · `object_store::throttle::ThrottledStore::put_opts` · object_store 0.13.2

```rust
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [266, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/throttle.rs:155`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e47a520e7a8b6102523759b4"></a>
## put_part

`function` · `object_store::throttle::ThrottledStore::put_part` · object_store 0.13.2

```rust
async fn put_part(&self, path: &Path, id: &MultipartId, part_idx: usize, data: PutPayload) -> Result<PartId>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [336, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/throttle.rs:313`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b23dd4b77b1db609208aeeee"></a>
## rename_opts

`function` · `object_store::throttle::ThrottledStore::rename_opts` · object_store 0.13.2

```rust
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "object_store::throttle::ThrottledStore", "path": "ThrottledStore"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [266, 2], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/throttle.rs:261`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
