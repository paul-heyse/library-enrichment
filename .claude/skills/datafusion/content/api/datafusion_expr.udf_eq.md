# `datafusion_expr::udf_eq`

Crate `datafusion-expr` · 1 public items · structured records in [`model/datafusion_expr.udf_eq.json`](../model/datafusion_expr.udf_eq.json)

## UdfEq

`struct` · `datafusion_expr::udf_eq::UdfEq`

```rust
struct UdfEq<Ptr: UdfPointer>
```

**Implements**: `core::convert::From`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**via `core::convert::From`**

```rust
fn from(ptr: Ptr) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

A wrapper around a pointer to UDF that implements `Eq` and `Hash` delegating to
corresponding methods on the UDF trait.

If you want to just compare pointers for equality, use [`super::ptr_eq::PtrEq`].

---
