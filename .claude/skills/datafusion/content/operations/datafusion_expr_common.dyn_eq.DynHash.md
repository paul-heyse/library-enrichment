# `datafusion_expr_common::dyn_eq::DynHash`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.dyn_eq.DynHash.json).

<a id="op-b1cd12827f1210014f9301ab"></a>
## DynHash

`trait` · `datafusion_expr_common::dyn_eq::DynHash` · datafusion-expr-common 55.1.0

```rust
trait DynHash: private::HashSealed
```

Source: `src/dyn_eq.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A dyn-compatible version of [`Hash`] trait.
If two values are equal according to [`DynEq`](../operations/datafusion_expr_common.dyn_eq.DynEq.md#op-ee2f7a30a4c03e9d5680ea5a), they must produce the same hash value.

Note: This trait should not be implemented directly. Implement `Hash` and `Any` and use
the blanket implementation.

Unresolved upstream links (retained, not inferred): ``Hash``.

<a id="op-6b744efedc7e6cc0e2277a06"></a>
## dyn_hash

`function` · `datafusion_expr_common::dyn_eq::DynHash::dyn_hash` · datafusion-expr-common 55.1.0

```rust
fn dyn_hash(&self, _state: &mut dyn Hasher)
```

Source: `src/dyn_eq.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
