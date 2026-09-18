# `datafusion_common::schema_reference`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.schema_reference.json`](../model/datafusion_common.schema_reference.json)

## SchemaReference

`enum` · `datafusion_common::schema_reference::SchemaReference`

Also reachable as `datafusion::common::SchemaReference`, `datafusion_common::SchemaReference`

```rust
enum SchemaReference
```

**Variants**: `Bare`, `Full`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn schema_name(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.schema_reference.SchemaReference.md).


---
