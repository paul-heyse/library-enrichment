# `datafusion_common::utils::set_difference`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.set_difference.json).

<a id="op-9fa5cdb5404624a47e2d0a9d"></a>
## set_difference

`function` · `datafusion_common::utils::set_difference` · datafusion-common 55.1.0

```rust
fn set_difference<T: Borrow<usize>, S: Borrow<usize>>(first: impl IntoIterator<Item = T>, second: impl IntoIterator<Item = S>) -> Vec<usize>
```

Source: `src/utils/mod.rs:1041`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculates the set difference between sequences `first` and `second`,
returning the result as a [`Vec`]. Preserves the ordering of `first`.

Unresolved upstream links (retained, not inferred): ``Vec``.
