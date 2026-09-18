# `datafusion_execution::cache::cache_manager::CachedFileList`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.cache_manager.CachedFileList.json).

<a id="op-b8a81d2ea198f9c7a9cb8287"></a>
## CachedFileList

`struct` · `datafusion_execution::cache::cache_manager::CachedFileList` · datafusion-execution 55.1.0

```rust
struct CachedFileList
```

Source: `src/cache/cache_manager.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Cached file listing.

TTL expiration is handled internally by the cache implementation.

<a id="op-1a59f78b5900da338e8abdc0"></a>
## Target

`assoc_type` · `datafusion_execution::cache::cache_manager::CachedFileList::Target` · datafusion-execution 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileList", "path": "CachedFileList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 1], "end": [233, 2], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/cache/cache_manager.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d354ad33a0572ec80b8a86d8"></a>
## clone

`function` · `datafusion_execution::cache::cache_manager::CachedFileList::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> CachedFileList
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileList", "path": "CachedFileList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 17], "end": [163, 22], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/cache/cache_manager.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a83d3b1e46f11f780b2e78b"></a>
## deref

`function` · `datafusion_execution::cache::cache_manager::CachedFileList::deref` · datafusion-execution 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileList", "path": "CachedFileList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 1], "end": [233, 2], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/cache/cache_manager.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f006003604af2dc393d89803"></a>
## eq

`function` · `datafusion_execution::cache::cache_manager::CachedFileList::eq` · datafusion-execution 55.1.0

```rust
fn eq(&self, other: &CachedFileList) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileList", "path": "CachedFileList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 24], "end": [163, 33], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/cache/cache_manager.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed6f242e33577a500af874ef"></a>
## files

`struct_field` · `datafusion_execution::cache::cache_manager::CachedFileList::files` · datafusion-execution 55.1.0

```rust
files: std::sync::Arc<Vec<object_store::ObjectMeta>>
```

Source: `src/cache/cache_manager.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

The cached file list.

<a id="op-343007d57f6ec89fd9bbacf9"></a>
## files_matching_prefix

`function` · `datafusion_execution::cache::cache_manager::CachedFileList::files_matching_prefix` · datafusion-execution 55.1.0

```rust
fn files_matching_prefix(&self, prefix: &Option<Path>) -> Arc<Vec<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileList", "path": "CachedFileList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 1], "end": [200, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns files matching the given prefix.

When prefix is `None`, returns a clone of the `Arc` (no data copy).
When filtering is needed, returns a new `Arc` with filtered results (clones each matching [`ObjectMeta`](../operations/object_store.ObjectMeta.md#op-84641755fb7ee613fe92d518)).

<a id="op-505a0e7ce7aa22ffeb7092a0"></a>
## fmt

`function` · `datafusion_execution::cache::cache_manager::CachedFileList::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileList", "path": "CachedFileList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 10], "end": [163, 15], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/cache/cache_manager.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5846ecd95ed3e2ebdbd5e097"></a>
## from

`function` · `datafusion_execution::cache::cache_manager::CachedFileList::from` · datafusion-execution 55.1.0

```rust
fn from(files: Vec<ObjectMeta>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileList", "path": "CachedFileList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [239, 2], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::ObjectMeta", "path": "ObjectMeta"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/cache/cache_manager.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32f9e00c70facd3565f1b50e"></a>
## new

`function` · `datafusion_execution::cache::cache_manager::CachedFileList::new` · datafusion-execution 55.1.0

```rust
fn new(files: Vec<ObjectMeta>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileList", "path": "CachedFileList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 1], "end": [200, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create a new cached file list.

<a id="op-f18f9385938c856395f2fe2d"></a>
## size

`function` · `datafusion_execution::cache::cache_manager::CachedFileList::size` · datafusion-execution 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileList", "path": "CachedFileList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [212, 2], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}, "trait_path": "datafusion_execution::cache::CacheValue"}`

Source: `src/cache/cache_manager.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
