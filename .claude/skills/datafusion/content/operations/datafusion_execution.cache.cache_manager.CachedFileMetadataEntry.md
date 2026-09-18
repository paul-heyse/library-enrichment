# `datafusion_execution::cache::cache_manager::CachedFileMetadataEntry`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.cache_manager.CachedFileMetadataEntry.json).

<a id="op-5d7fdb873e28f330be5a251a"></a>
## CachedFileMetadataEntry

`struct` · `datafusion_execution::cache::cache_manager::CachedFileMetadataEntry` · datafusion-execution 55.1.0

```rust
struct CachedFileMetadataEntry
```

Source: `src/cache/cache_manager.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Cached file metadata entry with validation information.

<a id="op-eaa410ade2a3f312016d8eb3"></a>
## clone

`function` · `datafusion_execution::cache::cache_manager::CachedFileMetadataEntry::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> CachedFileMetadataEntry
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileMetadataEntry", "path": "CachedFileMetadataEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [260, 10], "end": [260, 15], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/cache/cache_manager.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9318ecdf2d05726c3f71727f"></a>
## file_metadata

`struct_field` · `datafusion_execution::cache::cache_manager::CachedFileMetadataEntry::file_metadata` · datafusion-execution 55.1.0

```rust
file_metadata: std::sync::Arc<dyn FileMetadata>
```

Source: `src/cache/cache_manager.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

The cached file metadata.

<a id="op-afdd3328829953fc9e50b21d"></a>
## fmt

`function` · `datafusion_execution::cache::cache_manager::CachedFileMetadataEntry::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileMetadataEntry", "path": "CachedFileMetadataEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 1], "end": [297, 2], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/cache/cache_manager.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9f683c948973c71d3dbd2e2"></a>
## is_valid_for

`function` · `datafusion_execution::cache::cache_manager::CachedFileMetadataEntry::is_valid_for` · datafusion-execution 55.1.0

```rust
fn is_valid_for(&self, current_meta: &ObjectMeta) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileMetadataEntry", "path": "CachedFileMetadataEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [288, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:284`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Check if this cached entry is still valid for the given metadata.

<a id="op-0086fb1b9b753830d2914822"></a>
## meta

`struct_field` · `datafusion_execution::cache::cache_manager::CachedFileMetadataEntry::meta` · datafusion-execution 55.1.0

```rust
meta: object_store::ObjectMeta
```

Source: `src/cache/cache_manager.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

File metadata used for cache validation (size, last_modified).

<a id="op-ac111f29d96cc136cd3f06e0"></a>
## new

`function` · `datafusion_execution::cache::cache_manager::CachedFileMetadataEntry::new` · datafusion-execution 55.1.0

```rust
fn new(meta: ObjectMeta, file_metadata: Arc<dyn FileMetadata>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileMetadataEntry", "path": "CachedFileMetadataEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [288, 2], "filename": "src/cache/cache_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/cache_manager.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create a new cached file metadata entry.

<a id="op-d09a7972d42f13e85aa2a9e6"></a>
## size

`function` · `datafusion_execution::cache::cache_manager::CachedFileMetadataEntry::size` · datafusion-execution 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::cache_manager::CachedFileMetadataEntry", "path": "CachedFileMetadataEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [268, 1], "end": [272, 2], "filename": "src/cache/cache_manager.rs"}, "trait": {"args": null, "id": "datafusion_execution::cache::CacheValue", "path": "CacheValue"}, "trait_path": "datafusion_execution::cache::CacheValue"}`

Source: `src/cache/cache_manager.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
