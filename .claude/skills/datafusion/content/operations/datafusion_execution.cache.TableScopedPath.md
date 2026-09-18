# `datafusion_execution::cache::TableScopedPath`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.TableScopedPath.json).

<a id="op-119bc212d2dd52ee1bef83f4"></a>
## TableScopedPath

`struct` · `datafusion_execution::cache::TableScopedPath` · datafusion-execution 55.1.0

```rust
struct TableScopedPath
```

Source: `src/cache/mod.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Each entry is scoped to its use within a specific table so that the cache
can differentiate between identical paths in different tables, and
table-level cache invalidation.

<a id="op-da8224bfde739718e7f64e52"></a>
## clone

`function` · `datafusion_execution::cache::TableScopedPath::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> TableScopedPath
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::TableScopedPath", "path": "TableScopedPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 31], "end": [149, 36], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/cache/mod.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b4037f3868a80257e69780a"></a>
## eq

`function` · `datafusion_execution::cache::TableScopedPath::eq` · datafusion-execution 55.1.0

```rust
fn eq(&self, other: &TableScopedPath) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::TableScopedPath", "path": "TableScopedPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 10], "end": [149, 19], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/cache/mod.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06211afda23a35095113f974"></a>
## fmt

`function` · `datafusion_execution::cache::TableScopedPath::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::TableScopedPath", "path": "TableScopedPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 38], "end": [149, 43], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/cache/mod.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18deb99ae1d2a50b0c11fe9a"></a>
## fmt

`function` · `datafusion_execution::cache::TableScopedPath::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::TableScopedPath", "path": "TableScopedPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [169, 2], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/cache/mod.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5efb043629686d237be450a"></a>
## hash

`function` · `datafusion_execution::cache::TableScopedPath::hash` · datafusion-execution 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::TableScopedPath", "path": "TableScopedPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 25], "end": [149, 29], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/cache/mod.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e36b0125e154f5d24265d39b"></a>
## heap_size

`function` · `datafusion_execution::cache::TableScopedPath::heap_size` · datafusion-execution 55.1.0

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::TableScopedPath", "path": "TableScopedPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [159, 2], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "datafusion_common::heap_size::DFHeapSize", "path": "DFHeapSize"}, "trait_path": "datafusion_common::heap_size::DFHeapSize"}`

Source: `src/cache/mod.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d6430c87ec10f981e6ea9f3"></a>
## path

`struct_field` · `datafusion_execution::cache::TableScopedPath::path` · datafusion-execution 55.1.0

```rust
path: object_store::path::Path
```

Source: `src/cache/mod.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18749d8b02ed6a08f8157546"></a>
## size

`function` · `datafusion_execution::cache::TableScopedPath::size` · datafusion-execution 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::TableScopedPath", "path": "TableScopedPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [144, 2], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}, "trait_path": "datafusion_execution::cache::CacheKey"}`

Source: `src/cache/mod.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de4cf96b20aaa15231a5b040"></a>
## table

`struct_field` · `datafusion_execution::cache::TableScopedPath::table` · datafusion-execution 55.1.0

```rust
table: Option<datafusion_common::TableReference>
```

Source: `src/cache/mod.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0b9f67accd0a250f199c7cc"></a>
## table_ref

`function` · `datafusion_execution::cache::TableScopedPath::table_ref` · datafusion-execution 55.1.0

```rust
fn table_ref(&self) -> Option<&TableReference>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::TableScopedPath", "path": "TableScopedPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [144, 2], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "datafusion_execution::cache::CacheKey", "path": "CacheKey"}, "trait_path": "datafusion_execution::cache::CacheKey"}`

Source: `src/cache/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
