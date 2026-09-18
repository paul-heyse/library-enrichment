# `datafusion_expr::utils::powerset`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.powerset.json).

<a id="op-6e97455534bc07bed313846f"></a>
## powerset

`function` · `datafusion_expr::utils::powerset` · datafusion-expr 55.1.0

```rust
fn powerset<T>(slice: &[T]) -> datafusion_common::Result<Vec<Vec<&T>>>
```

Source: `src/utils.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The [power set] (or powerset) of a set S is the set of all subsets of S, \
including the empty set and S itself.

Example:

If S is the set {x, y, z}, then all the subsets of S are \
 {} \
 {x} \
 {y} \
 {z} \
 {x, y} \
 {x, z} \
 {y, z} \
 {x, y, z} \
 and hence the power set of S is {{}, {x}, {y}, {z}, {x, y}, {x, z}, {y, z}, {x, y, z}}.

[power set]: https://en.wikipedia.org/wiki/Power_set
