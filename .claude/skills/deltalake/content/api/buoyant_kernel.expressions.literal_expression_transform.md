# `buoyant_kernel::expressions::literal_expression_transform`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.expressions.literal_expression_transform.json`](../model/buoyant_kernel.expressions.literal_expression_transform.json)

## Error

`enum` · `buoyant_kernel::expressions::literal_expression_transform::Error`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.expressions.literal_expression_transform.Error.md)

Also reachable as `delta_kernel::expressions::literal_expression_transform::Error`

```rust
enum Error
```

**Variants**: `Schema`, `ExcessScalars`, `InsufficientScalars`, `EmptyStack`, `Unsupported`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Any error for [`LiteralExpressionTransform`]

---
