# `datafusion_execution::cache::SchemaFingerprint`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.SchemaFingerprint.json).

<a id="op-47b1c027f6d4d56204136d41"></a>
## SchemaFingerprint

`struct` · `datafusion_execution::cache::SchemaFingerprint` · datafusion-execution 55.1.0

```rust
struct SchemaFingerprint
```

Source: `src/cache/mod.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A fingerprint of the `file_schema` used to compute a file's statistics.

Captures exactly the attributes that determine the layout and meaning of
`Statistics::column_statistics`: each column's name, data type and
nullability, in order. It deliberately excludes field/schema metadata, which
cannot affect statistics — including it would needlessly fragment the cache.

<a id="op-4969ad00374afd753f0520e3"></a>
## clone

`function` · `datafusion_execution::cache::SchemaFingerprint::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> SchemaFingerprint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::SchemaFingerprint", "path": "SchemaFingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 10], "end": [177, 15], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/cache/mod.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bdcc5ed80eac4a140ce4fed"></a>
## eq

`function` · `datafusion_execution::cache::SchemaFingerprint::eq` · datafusion-execution 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::SchemaFingerprint", "path": "SchemaFingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 1], "end": [212, 2], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/cache/mod.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afeeb1c219731efdc95f9b77"></a>
## fmt

`function` · `datafusion_execution::cache::SchemaFingerprint::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::SchemaFingerprint", "path": "SchemaFingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 17], "end": [177, 22], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/cache/mod.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d4c262088594e4fed8e57b0"></a>
## from_schema

`function` · `datafusion_execution::cache::SchemaFingerprint::from_schema` · datafusion-execution 55.1.0

```rust
fn from_schema(file_schema: &Schema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::SchemaFingerprint", "path": "SchemaFingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [205, 2], "filename": "src/cache/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/mod.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Builds a fingerprint from the `file_schema` used to compute statistics
(the schema of the columns physically read, not the full table schema —
partition columns and their statistics are handled separately).

<a id="op-56ae31e35322679b916002fe"></a>
## hash

`function` · `datafusion_execution::cache::SchemaFingerprint::hash` · datafusion-execution 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::SchemaFingerprint", "path": "SchemaFingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 1], "end": [220, 2], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/cache/mod.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a2dcfb389cb6c3100b70f55"></a>
## heap_size

`function` · `datafusion_execution::cache::SchemaFingerprint::heap_size` · datafusion-execution 55.1.0

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::SchemaFingerprint", "path": "SchemaFingerprint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [226, 2], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "datafusion_common::heap_size::DFHeapSize", "path": "DFHeapSize"}, "trait_path": "datafusion_common::heap_size::DFHeapSize"}`

Source: `src/cache/mod.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
