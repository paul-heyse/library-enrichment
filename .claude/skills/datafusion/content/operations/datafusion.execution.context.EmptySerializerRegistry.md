# `datafusion::execution::context::EmptySerializerRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.execution.context.EmptySerializerRegistry.json).

<a id="op-967a42f4be6abdde74acecac"></a>
## EmptySerializerRegistry

`struct` · `datafusion::execution::context::EmptySerializerRegistry` · datafusion 55.1.0

```rust
struct EmptySerializerRegistry
```

Source: `src/execution/context/mod.rs:2252`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Default implementation of [SerializerRegistry](../operations/datafusion_expr.registry.SerializerRegistry.md#op-815a1f89b969d5eb430d928d) that throws unimplemented error
for all requests.

<a id="op-0ff2b5ef95e8b7717a408acf"></a>
## deserialize_logical_plan

`function` · `datafusion::execution::context::EmptySerializerRegistry::deserialize_logical_plan` · datafusion 55.1.0

```rust
fn deserialize_logical_plan(&self, name: &str, _bytes: &[u8]) -> Result<Arc<dyn UserDefinedLogicalNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::EmptySerializerRegistry", "path": "EmptySerializerRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2254, 1], "end": [2274, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::SerializerRegistry", "path": "SerializerRegistry"}, "trait_path": "datafusion_expr::registry::SerializerRegistry"}`

Source: `src/execution/context/mod.rs:2265`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31014eb0dc328462ee1bcef9"></a>
## fmt

`function` · `datafusion::execution::context::EmptySerializerRegistry::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::EmptySerializerRegistry", "path": "EmptySerializerRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2251, 10], "end": [2251, 15], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution/context/mod.rs:2251`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0021c2a22eac01eab6bccbbe"></a>
## serialize_logical_plan

`function` · `datafusion::execution::context::EmptySerializerRegistry::serialize_logical_plan` · datafusion 55.1.0

```rust
fn serialize_logical_plan(&self, node: &dyn UserDefinedLogicalNode) -> Result<Vec<u8>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::EmptySerializerRegistry", "path": "EmptySerializerRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2254, 1], "end": [2274, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::SerializerRegistry", "path": "SerializerRegistry"}, "trait_path": "datafusion_expr::registry::SerializerRegistry"}`

Source: `src/execution/context/mod.rs:2255`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
