# `object_store::tags::TagSet`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.tags.TagSet.json).

<a id="op-de03e4ce862050b5b69144c8"></a>
## TagSet

`struct` · `object_store::tags::TagSet` · object_store 0.13.2

```rust
struct TagSet
```

Source: `src/tags.rs:25`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A collection of key value pairs used to annotate objects

<https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-tagging.html>
<https://learn.microsoft.com/en-us/rest/api/storageservices/set-blob-tags>

<a id="op-aa7d8b38d3760727dd9adc0a"></a>
## clone

`function` · `object_store::tags::TagSet::clone` · object_store 0.13.2

```rust
fn clone(&self) -> TagSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::tags::TagSet", "path": "TagSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 17], "end": [24, 22], "filename": "src/tags.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/tags.rs:24`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52e7d6bfe3b7519a7bcd0f02"></a>
## default

`function` · `object_store::tags::TagSet::default` · object_store 0.13.2

```rust
fn default() -> TagSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::tags::TagSet", "path": "TagSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 24], "end": [24, 31], "filename": "src/tags.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/tags.rs:24`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7de36292b61db545b6c62b9d"></a>
## encoded

`function` · `object_store::tags::TagSet::encoded` · object_store 0.13.2

```rust
fn encoded(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::tags::TagSet", "path": "TagSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [44, 2], "filename": "src/tags.rs"}, "trait": null, "trait_path": null}`

Source: `src/tags.rs:41`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Return this [`TagSet`](../operations/object_store.tags.TagSet.md#op-de03e4ce862050b5b69144c8) as a URL-encoded string

<a id="op-f92afaf3c2991681e37b7672"></a>
## eq

`function` · `object_store::tags::TagSet::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &TagSet) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::tags::TagSet", "path": "TagSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 37], "end": [24, 46], "filename": "src/tags.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tags.rs:24`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec82f89fda6971ab8d3c71d4"></a>
## fmt

`function` · `object_store::tags::TagSet::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::tags::TagSet", "path": "TagSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 10], "end": [24, 15], "filename": "src/tags.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tags.rs:24`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab4f9a7fb0c0dd3808420259"></a>
## push

`function` · `object_store::tags::TagSet::push` · object_store 0.13.2

```rust
fn push(&mut self, key: &str, value: &str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::tags::TagSet", "path": "TagSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [44, 2], "filename": "src/tags.rs"}, "trait": null, "trait_path": null}`

Source: `src/tags.rs:36`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Append a key value pair to this [`TagSet`](../operations/object_store.tags.TagSet.md#op-de03e4ce862050b5b69144c8)

Stores have different restrictions on what characters are permitted,
for portability it is recommended applications use no more than 10 tags,
and stick to alphanumeric characters, and `+ - = . _ : /`

<https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObjectTagging.html>
<https://learn.microsoft.com/en-us/rest/api/storageservices/set-blob-tags?tabs=azure-ad#request-body>
