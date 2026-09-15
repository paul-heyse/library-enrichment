# `arrow_schema::extension::canonical`

Crate `arrow-schema` · 1 public items · structured records in [`model/arrow_schema.extension.canonical.json`](../model/arrow_schema.extension.canonical.json)

## CanonicalExtensionType

`enum` · `arrow_schema::extension::canonical::CanonicalExtensionType`

```rust
enum CanonicalExtensionType
```

**Variants**: `FixedShapeTensor`, `VariableShapeTensor`, `Json`, `Uuid`, `Opaque`, `Bool8`, `TimestampWithOffset`

**Implements**: `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: Bool8) -> Self
fn from(value: Uuid) -> Self
fn from(value: VariableShapeTensor) -> Self
fn from(value: Opaque) -> Self
fn from(value: TimestampWithOffset) -> Self
fn from(value: Json) -> Self
fn from(value: FixedShapeTensor) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &Field) -> Result<Self, Self::Error>
```

Canonical extension types.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#format-canonical-extensions>

---
