# `deltalake_opendal::shim::ConditionalPutShim`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_opendal.shim.ConditionalPutShim.json).

<a id="op-4fff2c159f12b76cb21c0981"></a>
## ConditionalPutShim

`struct` · `deltalake_opendal::shim::ConditionalPutShim` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ConditionalPutShim
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L22).

Source: `crates/opendal/src/shim.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Wraps an inner store and emulates [`PutMode::Create`] with a HEAD-then-PUT.

OpenDAL backends that don't implement `write_with_if_not_exists` reject
`PutMode::Create` with `Unsupported`. This shim approximates the conditional
semantics: it is racy across concurrent writers, so it is only appropriate
for single-writer stores.

Unresolved upstream links (retained, not inferred): ``PutMode::Create``.

<a id="op-a4eb2fc4e5086e9167085442"></a>
## copy_opts

`function` · `deltalake_opendal::shim::ConditionalPutShim::copy_opts` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> ObjectStoreResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L105).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::shim::ConditionalPutShim", "path": "ConditionalPutShim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [130, 2], "filename": "crates/opendal/src/shim.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/opendal/src/shim.rs:105`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40b3f586598894c8df688b83"></a>
## delete_stream

`function` · `deltalake_opendal::shim::ConditionalPutShim::delete_stream` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn delete_stream(&self, locations: BoxStream<'static, ObjectStoreResult<Path>>) -> BoxStream<'static, ObjectStoreResult<Path>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L82).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::shim::ConditionalPutShim", "path": "ConditionalPutShim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [130, 2], "filename": "crates/opendal/src/shim.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/opendal/src/shim.rs:82`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c8938647acb43b3fa7ee840"></a>
## fmt

`function` · `deltalake_opendal::shim::ConditionalPutShim::fmt` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L39).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::shim::ConditionalPutShim", "path": "ConditionalPutShim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [42, 2], "filename": "crates/opendal/src/shim.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/opendal/src/shim.rs:39`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b160059de47c9ed345eb88a0"></a>
## fmt

`function` · `deltalake_opendal::shim::ConditionalPutShim::fmt` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L33).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::shim::ConditionalPutShim", "path": "ConditionalPutShim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [36, 2], "filename": "crates/opendal/src/shim.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/opendal/src/shim.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bceb6b1c7e12974f71c329b"></a>
## get_opts

`function` · `deltalake_opendal::shim::ConditionalPutShim::get_opts` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_opts(&self, location: &Path, options: GetOptions) -> ObjectStoreResult<GetResult>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L70).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::shim::ConditionalPutShim", "path": "ConditionalPutShim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [130, 2], "filename": "crates/opendal/src/shim.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/opendal/src/shim.rs:70`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ed0a7dacc2f38aa1340503e"></a>
## get_ranges

`function` · `deltalake_opendal::shim::ConditionalPutShim::get_ranges` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn get_ranges(&self, location: &Path, ranges: &[Range<u64>]) -> ObjectStoreResult<Vec<Bytes>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L74).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::shim::ConditionalPutShim", "path": "ConditionalPutShim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [130, 2], "filename": "crates/opendal/src/shim.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/opendal/src/shim.rs:74`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63811c40b3ee4ed0a7e23438"></a>
## list

`function` · `deltalake_opendal::shim::ConditionalPutShim::list` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, ObjectStoreResult<ObjectMeta>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L89).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::shim::ConditionalPutShim", "path": "ConditionalPutShim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [130, 2], "filename": "crates/opendal/src/shim.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/opendal/src/shim.rs:89`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ef92ea778aac8a9a27449f8"></a>
## list_with_delimiter

`function` · `deltalake_opendal::shim::ConditionalPutShim::list_with_delimiter` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> ObjectStoreResult<ListResult>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L101).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::shim::ConditionalPutShim", "path": "ConditionalPutShim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [130, 2], "filename": "crates/opendal/src/shim.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/opendal/src/shim.rs:101`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24aff781b2548a95aa43ce77"></a>
## list_with_offset

`function` · `deltalake_opendal::shim::ConditionalPutShim::list_with_offset` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, ObjectStoreResult<ObjectMeta>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L93).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::shim::ConditionalPutShim", "path": "ConditionalPutShim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [130, 2], "filename": "crates/opendal/src/shim.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/opendal/src/shim.rs:93`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aca90d8787df0e5026cb503c"></a>
## new

`function` · `deltalake_opendal::shim::ConditionalPutShim::new` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(inner: Arc<dyn ObjectStore>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L27).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::shim::ConditionalPutShim", "path": "ConditionalPutShim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [30, 2], "filename": "crates/opendal/src/shim.rs"}, "trait": null, "trait_path": null}`

Source: `crates/opendal/src/shim.rs:27`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3ee0351a734f2ed912081bc"></a>
## put_multipart_opts

`function` · `deltalake_opendal::shim::ConditionalPutShim::put_multipart_opts` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn put_multipart_opts(&self, location: &Path, options: PutMultipartOptions) -> ObjectStoreResult<Box<dyn MultipartUpload>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L123).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::shim::ConditionalPutShim", "path": "ConditionalPutShim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [130, 2], "filename": "crates/opendal/src/shim.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/opendal/src/shim.rs:123`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e585686a112a2fef8db61adf"></a>
## put_opts

`function` · `deltalake_opendal::shim::ConditionalPutShim::put_opts` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn put_opts(&self, location: &Path, bytes: PutPayload, options: PutOptions) -> ObjectStoreResult<PutResult>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L46).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::shim::ConditionalPutShim", "path": "ConditionalPutShim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [130, 2], "filename": "crates/opendal/src/shim.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/opendal/src/shim.rs:46`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b5521815b9ee2bb204a78af"></a>
## rename_opts

`function` · `deltalake_opendal::shim::ConditionalPutShim::rename_opts` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn rename_opts(&self, from: &Path, to: &Path, options: RenameOptions) -> ObjectStoreResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L114).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::shim::ConditionalPutShim", "path": "ConditionalPutShim"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [130, 2], "filename": "crates/opendal/src/shim.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `crates/opendal/src/shim.rs:114`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3573aba77889b96effa7184a"></a>
## inner

`struct_field` · `deltalake_opendal::shim::ConditionalPutShim::inner` · deltalake-opendal 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: std::sync::Arc<dyn ObjectStore>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/shim.rs#L23).

Source: `crates/opendal/src/shim.rs:23`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
