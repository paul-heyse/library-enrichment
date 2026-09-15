# `datafusion_proto_models::generated::datafusion::merge_into_clause_node`

Crate `datafusion-proto-models` · 1 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.merge_into_clause_node.json`](../model/datafusion_proto_models.generated.datafusion.merge_into_clause_node.json)

## Kind

`enum` · `datafusion_proto_models::generated::datafusion::merge_into_clause_node::Kind`

```rust
enum Kind
```

**Variants**: `Matched`, `NotMatched`, `NotMatchedByTarget`, `NotMatchedBySource`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<Kind>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<Kind, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---
