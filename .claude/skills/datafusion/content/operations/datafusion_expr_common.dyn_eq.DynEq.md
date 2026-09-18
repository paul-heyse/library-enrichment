# `datafusion_expr_common::dyn_eq::DynEq`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.dyn_eq.DynEq.json).

<a id="op-ee2f7a30a4c03e9d5680ea5a"></a>
## DynEq

`trait` · `datafusion_expr_common::dyn_eq::DynEq` · datafusion-expr-common 55.1.0

```rust
trait DynEq: private::EqSealed
```

Source: `src/dyn_eq.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A dyn-compatible version of [`Eq`] trait.
The implementation constraints for this trait are the same as for [`Eq`]:
the implementation must be reflexive, symmetric, and transitive.
Additionally, if two values can be compared with [`DynEq`](../operations/datafusion_expr_common.dyn_eq.DynEq.md#op-ee2f7a30a4c03e9d5680ea5a) and [`PartialEq`] then
they must be [`DynEq`](../operations/datafusion_expr_common.dyn_eq.DynEq.md#op-ee2f7a30a4c03e9d5680ea5a)-equal if and only if they are [`PartialEq`]-equal.
It is therefore strongly discouraged to implement this trait for types
that implement `PartialEq<Other>` or `Eq<Other>` for any type `Other` other than `Self`.

Note: This trait should not be implemented directly. Implement `Eq` and `Any` and use
the blanket implementation.

Unresolved upstream links (retained, not inferred): ``Eq``, ``PartialEq``.

<a id="op-a40f8fcc04ced1301c643ff5"></a>
## dyn_eq

`function` · `datafusion_expr_common::dyn_eq::DynEq::dyn_eq` · datafusion-expr-common 55.1.0

```rust
fn dyn_eq(&self, other: &dyn Any) -> bool
```

Source: `src/dyn_eq.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
