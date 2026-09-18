# `datafusion_physical_expr::projection::ProjectionRef`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.projection.ProjectionRef.json).

<a id="op-570fc3381aff93c491892a2b"></a>
## ProjectionRef

`type_alias` · `datafusion_physical_expr::projection::ProjectionRef` · datafusion-physical-expr 55.1.0

```rust
type ProjectionRef = std::sync::Arc<[usize]>
```

Source: `src/projection.rs:987`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Describes an immutable reference counted projection.

This structure represents projecting a set of columns by index.
[`Arc`] is used to make it cheap to clone.

Unresolved upstream links (retained, not inferred): ``Arc``.
