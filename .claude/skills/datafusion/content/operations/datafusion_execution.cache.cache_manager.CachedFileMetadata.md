# `datafusion_execution::cache::cache_manager::CachedFileMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.cache_manager.CachedFileMetadata.json).

<a id="op-735ab08141f86091bef3968c"></a>
## CachedFileMetadata

`struct` · `datafusion_execution::cache::cache_manager::CachedFileMetadata` · datafusion-execution 55.1.0

```rust
struct CachedFileMetadata
```

Source: `src/cache/cache_manager.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Cached metadata for a file, including statistics and ordering.

This struct embeds the [`ObjectMeta`](../operations/object_store.ObjectMeta.md#op-84641755fb7ee613fe92d518) used for cache validation,
the `file_schema` fingerprint, cached statistics, and ordering information.

<a id="op-9558ebd7cf136b7c5bf362bd"></a>
## clone

`function` · `datafusion_execution::cache::cache_manager::CachedFileMetadata::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> CachedFileMetadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileMetadata", "path": "CachedFileMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 17], "end": [96, 22], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/cache/cache_manager.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-940b8a910e3f4dd8e2ce61a9"></a>
## eq

`function` · `datafusion_execution::cache::cache_manager::CachedFileMetadata::eq` · datafusion-execution 55.1.0

```rust
fn eq(&self, other: &CachedFileMetadata) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileMetadata", "path": "CachedFileMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 24], "end": [96, 33], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/cache/cache_manager.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27bd4aaf9252b72db556327f"></a>
## fmt

`function` · `datafusion_execution::cache::cache_manager::CachedFileMetadata::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileMetadata", "path": "CachedFileMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 10], "end": [96, 15], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/cache/cache_manager.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad99b2186612f8fb3251afed"></a>
## heap_size

`function` · `datafusion_execution::cache::cache_manager::CachedFileMetadata::heap_size` · datafusion-execution 55.1.0

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileMetadata", "path": "CachedFileMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [158, 2], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "datafusion_common::heap_size::DFHeapSize", "path": "DFHeapSize"}, "trait_path": "datafusion_common::heap_size::DFHeapSize"}`

Source: `src/cache/cache_manager.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5d4629dabac2f1689af996e"></a>
## is_valid_for

`function` · `datafusion_execution::cache::cache_manager::CachedFileMetadata::is_valid_for` · datafusion-execution 55.1.0

```rust
fn is_valid_for(&self, current_meta: &ObjectMeta, current_schema_fingerprint: &Arc<SchemaFingerprint>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileMetadata", "path": "CachedFileMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [138, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Check if this cached entry is still valid for the given metadata.

Returns true if the file size, last modified time, and schema match.

<a id="op-d7fcade71b4c6f49d87c00ad"></a>
## meta

`struct_field` · `datafusion_execution::cache::cache_manager::CachedFileMetadata::meta` · datafusion-execution 55.1.0

```rust
meta: object_store::ObjectMeta
```

Source: `src/cache/cache_manager.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

File metadata used for cache validation (size, last_modified).

<a id="op-68cf4a10f30f07663dd502d4"></a>
## new

`function` · `datafusion_execution::cache::cache_manager::CachedFileMetadata::new` · datafusion-execution 55.1.0

```rust
fn new(meta: ObjectMeta, schema_fingerprint: Arc<SchemaFingerprint>, statistics: Arc<Statistics>, ordering: Option<LexOrdering>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileMetadata", "path": "CachedFileMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [138, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create a new cached file metadata entry.

<a id="op-7674747e340d5d419bbd383b"></a>
## ordering

`struct_field` · `datafusion_execution::cache::cache_manager::CachedFileMetadata::ordering` · datafusion-execution 55.1.0

```rust
ordering: Option<datafusion_physical_expr_common::sort_expr::LexOrdering>
```

Source: `src/cache/cache_manager.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Cached ordering for the file.

<a id="op-e7b31782df2abfa807e93e07"></a>
## schema_fingerprint

`struct_field` · `datafusion_execution::cache::cache_manager::CachedFileMetadata::schema_fingerprint` · datafusion-execution 55.1.0

```rust
schema_fingerprint: std::sync::Arc<SchemaFingerprint>
```

Source: `src/cache/cache_manager.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Fingerprint of the `file_schema` used to compute `statistics`.

<a id="op-3b99e492a6af31046912f49a"></a>
## size

`function` · `datafusion_execution::cache::cache_manager::CachedFileMetadata::size` · datafusion-execution 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileMetadata", "path": "CachedFileMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [144, 2], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}, "trait_path": "datafusion_execution::cache::CacheValue"}`

Source: `src/cache/cache_manager.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f9d3f04b71d4ab108f893a7"></a>
## statistics

`struct_field` · `datafusion_execution::cache::cache_manager::CachedFileMetadata::statistics` · datafusion-execution 55.1.0

```rust
statistics: std::sync::Arc<datafusion_common::Statistics>
```

Source: `src/cache/cache_manager.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Cached statistics for the file, if available.
