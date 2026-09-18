# `arrow_ord::sort::partial_sort`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.sort.partial_sort.json).

<a id="op-2737bc1fa18cec7d1dc587ba"></a>
## partial_sort

`function` · `arrow_ord::sort::partial_sort` · arrow-ord 59.3.0

```rust
fn partial_sort<T, F>(v: &mut [T], limit: usize, is_less: F) where F: FnMut(&T, &T) -> std::cmp::Ordering
```

Source: `src/sort.rs:1114`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

It's unstable_sort, may not preserve the order of equal elements
