# `datafusion_expr_common::dyn_eq`

Crate `datafusion-expr-common` · 2 public items · structured records in [`model/datafusion_expr_common.dyn_eq.json`](../model/datafusion_expr_common.dyn_eq.json)

## DynEq

`trait` · `datafusion_expr_common::dyn_eq::DynEq`

Also reachable as `datafusion_physical_expr_common::physical_expr::DynEq`

```rust
trait DynEq: private::EqSealed
```

**Methods** (1)

```rust
fn dyn_eq(&self, other: &dyn Any) -> bool
```

A dyn-compatible version of [`Eq`] trait.
The implementation constraints for this trait are the same as for [`Eq`]:
the implementation must be reflexive, symmetric, and transitive.
Additionally, if two values can be compared with [`DynEq`] and [`PartialEq`] then
they must be [`DynEq`]-equal if and only if they are [`PartialEq`]-equal.
It is therefore strongly discouraged to implement this trait for types
that implement `PartialEq<Other>` or `Eq<Other>` for any type `Other` other than `Self`.

Note: This trait should not be implemented directly. Implement `Eq` and `Any` and use
the blanket implementation.

---

## DynHash

`trait` · `datafusion_expr_common::dyn_eq::DynHash`

Also reachable as `datafusion_physical_expr_common::physical_expr::DynHash`

```rust
trait DynHash: private::HashSealed
```

**Methods** (1)

```rust
fn dyn_hash(&self, _state: &mut dyn Hasher)
```

A dyn-compatible version of [`Hash`] trait.
If two values are equal according to [`DynEq`], they must produce the same hash value.

Note: This trait should not be implemented directly. Implement `Hash` and `Any` and use
the blanket implementation.

---
