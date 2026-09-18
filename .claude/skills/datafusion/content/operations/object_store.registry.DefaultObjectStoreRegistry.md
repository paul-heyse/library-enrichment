# `object_store::registry::DefaultObjectStoreRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.registry.DefaultObjectStoreRegistry.json).

<a id="op-2b1ce076eac40850c138b6cd"></a>
## DefaultObjectStoreRegistry

`struct` · `object_store::registry::DefaultObjectStoreRegistry` · object_store 0.13.2

```rust
struct DefaultObjectStoreRegistry
```

Source: `src/registry.rs:112`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An [`ObjectStoreRegistry`](../operations/object_store.registry.ObjectStoreRegistry.md#op-c074e1961f5f8e41ba850998) that uses [`parse_url_opts`](../operations/object_store.parse.parse_url_opts.md#op-196a22ab979ecdcbef0d1d32) to create stores based on the environment

<a id="op-dc3f4551ca43dfae39a7c0ef"></a>
## default

`function` · `object_store::registry::DefaultObjectStoreRegistry::default` · object_store 0.13.2

```rust
fn default() -> DefaultObjectStoreRegistry
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::registry::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 17], "end": [111, 24], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/registry.rs:111`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6532e7e9d9b8033914060720"></a>
## fmt

`function` · `object_store::registry::DefaultObjectStoreRegistry::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::registry::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 10], "end": [111, 15], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/registry.rs:111`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f058d73c7d341864fb2f5ce6"></a>
## new

`function` · `object_store::registry::DefaultObjectStoreRegistry::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::registry::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 1], "end": [174, 2], "filename": "src/registry.rs"}, "trait": null, "trait_path": null}`

Source: `src/registry.rs:171`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`DefaultObjectStoreRegistry`](../operations/object_store.registry.DefaultObjectStoreRegistry.md#op-2b1ce076eac40850c138b6cd)

<a id="op-3e5f2a3d3f54f5989ce20e3e"></a>
## register

`function` · `object_store::registry::DefaultObjectStoreRegistry::register` · object_store 0.13.2

```rust
fn register(&self, url: Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::registry::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [176, 1], "end": [218, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "object_store::registry::ObjectStoreRegistry", "path": "ObjectStoreRegistry"}, "trait_path": "object_store::registry::ObjectStoreRegistry"}`

Source: `src/registry.rs:177`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0710c9a83f27ed61d9ae87f"></a>
## resolve

`function` · `object_store::registry::DefaultObjectStoreRegistry::resolve` · object_store 0.13.2

```rust
fn resolve(&self, to_resolve: &Url) -> Result<(Arc<dyn ObjectStore>, Path)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::registry::DefaultObjectStoreRegistry", "path": "DefaultObjectStoreRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [176, 1], "end": [218, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "object_store::registry::ObjectStoreRegistry", "path": "ObjectStoreRegistry"}, "trait_path": "object_store::registry::ObjectStoreRegistry"}`

Source: `src/registry.rs:188`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
