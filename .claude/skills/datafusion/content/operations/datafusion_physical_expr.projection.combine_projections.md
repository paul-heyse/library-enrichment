# `datafusion_physical_expr::projection::combine_projections`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.projection.combine_projections.json).

<a id="op-094fa677e54b6d8a7fa4489f"></a>
## combine_projections

`function` · `datafusion_physical_expr::projection::combine_projections` · datafusion-physical-expr 55.1.0

```rust
fn combine_projections(p1: Option<&ProjectionRef>, p2: Option<&ProjectionRef>) -> datafusion_common::Result<Option<ProjectionRef>>
```

Source: `src/projection.rs:1004`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Combine two projections.

If `p1` is [`None`] then there are no changes.
Otherwise, if passed `p2` is not [`None`] then it is remapped
according to the `p1`. Otherwise, there are no changes.

# Example

If stored projection is [0, 2] and we call `apply_projection([0, 2, 3])`,
then the resulting projection will be [0, 3].

# Error

Returns an internal error if `p1` contains index that is greater than `p2` len.


Unresolved upstream links (retained, not inferred): ``None``.
