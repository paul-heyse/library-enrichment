# `datafusion_common::types::logical`

Crate `datafusion-common` · 4 public items · structured records in [`model/datafusion_common.types.logical.json`](../model/datafusion_common.types.logical.json)

## TypeParameter

`enum` · `datafusion_common::types::logical::TypeParameter`

```rust
enum TypeParameter<'a>
```

**Variants**: `Type`, `Number`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

---

## TypeSignature

`enum` · `datafusion_common::types::logical::TypeSignature`

```rust
enum TypeSignature<'a>
```

**Variants**: `Native`, `Extension`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

Signature that uniquely identifies a type among other types.

---

## LogicalType

`trait` · `datafusion_common::types::logical::LogicalType`

```rust
trait LogicalType: Sync + Send
```

**Implementors** (1)

- `datafusion_common::types::native::NativeType`

**Methods** (3)

```rust
fn default_cast_for(&self, origin: &DataType) -> Result<DataType>
fn native(&self) -> &NativeType
fn signature(&self) -> TypeSignature<'_>
```

Representation of a logical type with its signature and its native backing
type.

The logical type is meant to be used during the DataFusion logical planning
phase in order to reason about logical types without worrying about their
underlying physical implementation.

### Extension types

[`LogicalType`] is a trait in order to allow the possibility of declaring
extension types:

```
use datafusion_common::types::{LogicalType, NativeType, TypeSignature};

struct JSON {}

impl LogicalType for JSON {
    fn native(&self) -> &NativeType {
        &NativeType::String
    }

    fn signature(&self) -> TypeSignature<'_> {
        TypeSignature::Extension {
            name: "JSON",
            parameters: &[],
        }
    }
}
```

---

## LogicalTypeRef

`type_alias` · `datafusion_common::types::logical::LogicalTypeRef`

```rust
type LogicalTypeRef = std::sync::Arc<dyn LogicalType>
```

A reference counted [`LogicalType`].

---
