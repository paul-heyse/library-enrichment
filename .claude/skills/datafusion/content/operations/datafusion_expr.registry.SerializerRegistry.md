# `datafusion_expr::registry::SerializerRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.registry.SerializerRegistry.json).

<a id="op-815a1f89b969d5eb430d928d"></a>
## SerializerRegistry

`trait` · `datafusion_expr::registry::SerializerRegistry` · datafusion-expr 55.1.0

```rust
trait SerializerRegistry: Debug + Send + Sync
```

Source: `src/registry.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Serializer and deserializer registry for extensions like [UserDefinedLogicalNode](../operations/datafusion_expr.logical_plan.extension.UserDefinedLogicalNode.md#op-2111ab49e0384ae550130432).

<a id="op-9d8a5b8a49b0ed5c1edaed38"></a>
## deserialize_logical_plan

`function` · `datafusion_expr::registry::SerializerRegistry::deserialize_logical_plan` · datafusion-expr 55.1.0

```rust
fn deserialize_logical_plan(&self, name: &str, bytes: &[u8]) -> Result<Arc<dyn UserDefinedLogicalNode>>
```

Source: `src/registry.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Deserialize user defined logical plan node ([UserDefinedLogicalNode](../operations/datafusion_expr.logical_plan.extension.UserDefinedLogicalNode.md#op-2111ab49e0384ae550130432)) from
bytes.

<a id="op-bf4f83e22e55ecb6e13a6a4f"></a>
## serialize_logical_plan

`function` · `datafusion_expr::registry::SerializerRegistry::serialize_logical_plan` · datafusion-expr 55.1.0

```rust
fn serialize_logical_plan(&self, node: &dyn UserDefinedLogicalNode) -> Result<Vec<u8>>
```

Source: `src/registry.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Serialize this node to a byte array. This serialization should not include
input plans.
