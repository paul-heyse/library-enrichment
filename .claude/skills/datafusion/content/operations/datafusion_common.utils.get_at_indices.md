# `datafusion_common::utils::get_at_indices`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.get_at_indices.json).

<a id="op-a1f560a6d1e134dfaeff6686"></a>
## get_at_indices

`function` · `datafusion_common::utils::get_at_indices` · datafusion-common 55.1.0

```rust
fn get_at_indices<T: Clone, I: Borrow<usize>>(items: &[T], indices: impl IntoIterator<Item = I>) -> Result<Vec<T>>
```

Source: `src/utils/mod.rs:368`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This function "takes" the elements at `indices` from the slice `items`.
