# `arrow_ord::ord::DynComparator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.ord.DynComparator.json).

<a id="op-6345922e5d8f905e5601e421"></a>
## DynComparator

`type_alias` · `arrow_ord::ord::DynComparator` · arrow-ord 59.3.0

```rust
type DynComparator = Box<dyn Fn(usize, usize) -> std::cmp::Ordering + Send + Sync>
```

Source: `src/ord.rs:50`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Compare the values at two arbitrary indices in two arrays.
