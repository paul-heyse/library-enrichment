# `datafusion_expr::ptr_eq::arc_ptr_hash`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.ptr_eq.arc_ptr_hash.json).

<a id="op-adc39c6a7074add260f264a1"></a>
## arc_ptr_hash

`function` · `datafusion_expr::ptr_eq::arc_ptr_hash` · datafusion-expr 55.1.0

```rust
fn arc_ptr_hash<T: ?Sized>(a: &std::sync::Arc<T>, hasher: &mut impl Hasher)
```

Source: `src/ptr_eq.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Hashes an `Arc` pointer based on its underlying pointer value.
The general contract for this function is that if [`arc_ptr_eq`](../operations/datafusion_expr.ptr_eq.arc_ptr_eq.md#op-49e6c8e3842c179fcfa55528) returns `true`
for two `Arc`s, then this function should return the same hash value for both.
