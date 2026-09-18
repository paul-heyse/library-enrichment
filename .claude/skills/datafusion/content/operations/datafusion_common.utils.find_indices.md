# `datafusion_common::utils::find_indices`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.find_indices.json).

<a id="op-d38f8f5b054aa5638b5f6230"></a>
## find_indices

`function` · `datafusion_common::utils::find_indices` · datafusion-common 55.1.0

```rust
fn find_indices<T: PartialEq, S: Borrow<T>>(items: &[T], targets: impl IntoIterator<Item = S>) -> Result<Vec<usize>>
```

Source: `src/utils/mod.rs:1055`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find indices of each element in `targets` inside `items`. If one of the
elements is absent in `items`, returns an error.
