# `datafusion_execution::object_store::ObjectStoreUrl`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.object_store.ObjectStoreUrl.json).

<a id="op-80582a87395549e6a527145e"></a>
## ObjectStoreUrl

`struct` · `datafusion_execution::object_store::ObjectStoreUrl` · datafusion-execution 55.1.0

```rust
struct ObjectStoreUrl
```

Source: `src/object_store.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A parsed URL identifying a particular [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) instance

For example:
* `file://` for local file system
* `s3://bucket` for AWS S3 bucket
* `oss://bucket` for Aliyun OSS bucket

<a id="op-c52f1b3b93020b0db6f2e35a"></a>
## as_ref

`function` · `datafusion_execution::object_store::ObjectStoreUrl::as_ref` · datafusion-execution 55.1.0

```rust
fn as_ref(&self) -> &Url
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreUrl", "path": "ObjectStoreUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [102, 2], "filename": "src/object_store.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "url::Url", "path": "Url"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/object_store.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c77e939b7c85648df2684552"></a>
## as_ref

`function` · `datafusion_execution::object_store::ObjectStoreUrl::as_ref` · datafusion-execution 55.1.0

```rust
fn as_ref(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreUrl", "path": "ObjectStoreUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [96, 2], "filename": "src/object_store.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/object_store.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e92b32309a0e6c4fb95bdb14"></a>
## as_str

`function` · `datafusion_execution::object_store::ObjectStoreUrl::as_str` · datafusion-execution 55.1.0

```rust
fn as_str(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreUrl", "path": "ObjectStoreUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [90, 2], "filename": "src/object_store.rs"}, "trait": null, "trait_path": null}`

Source: `src/object_store.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns this [`ObjectStoreUrl`](../operations/datafusion_execution.object_store.ObjectStoreUrl.md#op-80582a87395549e6a527145e) as a string

<a id="op-6c2793a736dbf8cfac2945a2"></a>
## clone

`function` · `datafusion_execution::object_store::ObjectStoreUrl::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> ObjectStoreUrl
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreUrl", "path": "ObjectStoreUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 17], "end": [38, 22], "filename": "src/object_store.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/object_store.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ade4d9c483a0a1a0774a9ad"></a>
## cmp

`function` · `datafusion_execution::object_store::ObjectStoreUrl::cmp` · datafusion-execution 55.1.0

```rust
fn cmp(&self, other: &ObjectStoreUrl) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreUrl", "path": "ObjectStoreUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 51], "end": [38, 54], "filename": "src/object_store.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/object_store.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00f3112a54bc7e85f26f1f06"></a>
## eq

`function` · `datafusion_execution::object_store::ObjectStoreUrl::eq` · datafusion-execution 55.1.0

```rust
fn eq(&self, other: &ObjectStoreUrl) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreUrl", "path": "ObjectStoreUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 24], "end": [38, 33], "filename": "src/object_store.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/object_store.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b188ba99283794c9f4f0a9bc"></a>
## fmt

`function` · `datafusion_execution::object_store::ObjectStoreUrl::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreUrl", "path": "ObjectStoreUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/object_store.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/object_store.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed00ee1459c32a5d3586c442"></a>
## fmt

`function` · `datafusion_execution::object_store::ObjectStoreUrl::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreUrl", "path": "ObjectStoreUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [108, 2], "filename": "src/object_store.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/object_store.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85ce24c4f2639fb08d894466"></a>
## hash

`function` · `datafusion_execution::object_store::ObjectStoreUrl::hash` · datafusion-execution 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreUrl", "path": "ObjectStoreUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 56], "end": [38, 60], "filename": "src/object_store.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/object_store.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9471cf2a03db49f6e0f970a"></a>
## local_filesystem

`function` · `datafusion_execution::object_store::ObjectStoreUrl::local_filesystem` · datafusion-execution 55.1.0

```rust
fn local_filesystem() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreUrl", "path": "ObjectStoreUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [90, 2], "filename": "src/object_store.rs"}, "trait": null, "trait_path": null}`

Source: `src/object_store.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

An [`ObjectStoreUrl`](../operations/datafusion_execution.object_store.ObjectStoreUrl.md#op-80582a87395549e6a527145e) for the local filesystem (`file://`)

# Example
```
# use datafusion_execution::object_store::ObjectStoreUrl;
let local_fs = ObjectStoreUrl::parse("file://").unwrap();
assert_eq!(local_fs, ObjectStoreUrl::local_filesystem())
```

<a id="op-e793c6ae2c33cdc93d676d79"></a>
## parse

`function` · `datafusion_execution::object_store::ObjectStoreUrl::parse` · datafusion-execution 55.1.0

```rust
fn parse(s: impl AsRef<str>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreUrl", "path": "ObjectStoreUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [90, 2], "filename": "src/object_store.rs"}, "trait": null, "trait_path": null}`

Source: `src/object_store.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Parse an [`ObjectStoreUrl`](../operations/datafusion_execution.object_store.ObjectStoreUrl.md#op-80582a87395549e6a527145e) from a string

# Example
```
# use url::Url;
# use datafusion_execution::object_store::ObjectStoreUrl;
let object_store_url = ObjectStoreUrl::parse("s3://bucket").unwrap();
assert_eq!(object_store_url.as_str(), "s3://bucket/");
// can also access the underlying `Url`
let url: &Url = object_store_url.as_ref();
assert_eq!(url.scheme(), "s3");
assert_eq!(url.host_str(), Some("bucket"));
assert_eq!(url.path(), "/");
```

<a id="op-d190c8e3c7627d61acaf77c3"></a>
## partial_cmp

`function` · `datafusion_execution::object_store::ObjectStoreUrl::partial_cmp` · datafusion-execution 55.1.0

```rust
fn partial_cmp(&self, other: &ObjectStoreUrl) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreUrl", "path": "ObjectStoreUrl"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 39], "end": [38, 49], "filename": "src/object_store.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/object_store.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
