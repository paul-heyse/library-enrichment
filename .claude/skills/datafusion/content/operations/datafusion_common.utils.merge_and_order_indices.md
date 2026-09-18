# `datafusion_common::utils::merge_and_order_indices`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.merge_and_order_indices.json).

<a id="op-29bfc558cb76270369714fcf"></a>
## merge_and_order_indices

`function` · `datafusion_common::utils::merge_and_order_indices` · datafusion-common 55.1.0

```rust
fn merge_and_order_indices<T: Borrow<usize>, S: Borrow<usize>>(first: impl IntoIterator<Item = T>, second: impl IntoIterator<Item = S>) -> Vec<usize>
```

Source: `src/utils/mod.rs:1024`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Merges collections `first` and `second`, removes duplicates and sorts the
result, returning it as a [`Vec`].

Unresolved upstream links (retained, not inferred): ``Vec``.
