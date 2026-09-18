# `datafusion_common::utils::split_vec_min_alloc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.split_vec_min_alloc.json).

<a id="op-57ef9340f75bbcfa735cca33"></a>
## split_vec_min_alloc

`function` · `datafusion_common::utils::split_vec_min_alloc` · datafusion-common 55.1.0

```rust
fn split_vec_min_alloc<T>(vec: &mut Vec<T>, n: usize) -> Vec<T>
```

Source: `src/utils/mod.rs:406`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Splits `vec` at index `n`, returning the first `n` elements and leaving the
remaining `vec.len() - n` elements in `vec`.

Allocates for whichever side is smaller, so the new allocation is
`min(n, vec.len() - n)` rather than always `n` (as `vec.drain(0..n).collect()`
would). This matters when the split emits a prefix under memory pressure,
where `n` can be close to `vec.len()`.
