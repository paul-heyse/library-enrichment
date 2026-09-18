# `datafusion_ffi::placement`

Crate `datafusion-ffi` · 1 public items · structured records in [`model/datafusion_ffi.placement.json`](../model/datafusion_ffi.placement.json)

## FFI_ExpressionPlacement

`enum` · `datafusion_ffi::placement::FFI_ExpressionPlacement`

```rust
enum FFI_ExpressionPlacement
```

**Variants**: `Literal`, `Column`, `MoveTowardsLeafNodes`, `KeepInPlace`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: ExpressionPlacement) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.placement.FFI_ExpressionPlacement.md).


---
