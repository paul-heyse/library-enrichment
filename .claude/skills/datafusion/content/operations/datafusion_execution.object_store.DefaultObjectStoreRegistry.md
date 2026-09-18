# `datafusion_execution::object_store::DefaultObjectStoreRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.object_store.DefaultObjectStoreRegistry.json).

<a id="op-6034f4a3b561921519b755ec"></a>
## DefaultObjectStoreRegistry

`struct` · `datafusion_execution::object_store::DefaultObjectStoreRegistry` · datafusion-execution 55.1.0

```rust
struct DefaultObjectStoreRegistry
```

Source: `src/object_store.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

The default [`ObjectStoreRegistry`](../operations/datafusion_execution.object_store.ObjectStoreRegistry.md#op-90df51667c296b43700e395a)

<a id="op-500faea797a1aa7086821736"></a>
## default

`function` · `datafusion_execution::object_store::DefaultObjectStoreRegistry::default` · datafusion-execution 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [205, 2], "filename": "src/object_store.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/object_store.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e142552edcc801834084d7ca"></a>
## deregister_store

`function` · `datafusion_execution::object_store::DefaultObjectStoreRegistry::deregister_store` · datafusion-execution 55.1.0

```rust
fn deregister_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [264, 2], "filename": "src/object_store.rs"}, "trait": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreRegistry", "path": "ObjectStoreRegistry"}, "trait_path": "datafusion_execution::object_store::ObjectStoreRegistry"}`

Source: `src/object_store.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c1f001bfce1836e7de5c7b2"></a>
## fmt

`function` · `datafusion_execution::object_store::DefaultObjectStoreRegistry::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [199, 2], "filename": "src/object_store.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/object_store.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b806c2386547cc970166437d"></a>
## get_store

`function` · `datafusion_execution::object_store::DefaultObjectStoreRegistry::get_store` · datafusion-execution 55.1.0

```rust
fn get_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [264, 2], "filename": "src/object_store.rs"}, "trait": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreRegistry", "path": "ObjectStoreRegistry"}, "trait_path": "datafusion_execution::object_store::ObjectStoreRegistry"}`

Source: `src/object_store.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee80eb8ed7857b2852292b2a"></a>
## new

`function` · `datafusion_execution::object_store::DefaultObjectStoreRegistry::new` · datafusion-execution 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 1], "end": [222, 2], "filename": "src/object_store.rs"}, "trait": null, "trait_path": null}`

Source: `src/object_store.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

This will register [`LocalFileSystem`](../operations/object_store.local.LocalFileSystem.md#op-972e6ecc4b3db74073f32c0b) to handle `file://` paths

<a id="op-b2dae9e9f9a60cf08fa8c4fd"></a>
## register_store

`function` · `datafusion_execution::object_store::DefaultObjectStoreRegistry::register_store` · datafusion-execution 55.1.0

```rust
fn register_store(&self, url: &Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::object_store::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [264, 2], "filename": "src/object_store.rs"}, "trait": {"args": null, "id": "datafusion_execution::object_store::ObjectStoreRegistry", "path": "ObjectStoreRegistry"}, "trait_path": "datafusion_execution::object_store::ObjectStoreRegistry"}`

Source: `src/object_store.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
