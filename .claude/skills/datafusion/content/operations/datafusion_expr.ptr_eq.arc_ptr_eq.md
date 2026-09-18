# `datafusion_expr::ptr_eq::arc_ptr_eq`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.ptr_eq.arc_ptr_eq.json).

<a id="op-49e6c8e3842c179fcfa55528"></a>
## arc_ptr_eq

`function` · `datafusion_expr::ptr_eq::arc_ptr_eq` · datafusion-expr 55.1.0

```rust
fn arc_ptr_eq<T: ?Sized>(a: &std::sync::Arc<T>, b: &std::sync::Arc<T>) -> bool
```

Source: `src/ptr_eq.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Compares two `Arc` pointers for equality based on their underlying pointers values.
This is not equivalent to [`Arc::ptr_eq`] for fat pointers, see that method
for more information.

Unresolved upstream links (retained, not inferred): ``Arc::ptr_eq``.
