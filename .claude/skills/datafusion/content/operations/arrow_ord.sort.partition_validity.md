# `arrow_ord::sort::partition_validity`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.sort.partition_validity.json).

<a id="op-05ddba895bf3930368b6e0cc"></a>
## partition_validity

`function` · `arrow_ord::sort::partition_validity` · arrow-ord 59.3.0

```rust
fn partition_validity(array: &dyn Array) -> (Vec<u32>, Vec<u32>)
```

Source: `src/sort.rs:194`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Partition indices of an Arrow array into two categories:
- `valid`: indices of non-null elements
- `nulls`: indices of null elements

Optimized for performance with fast-path for all-valid arrays
and bit-parallel scan for null-containing arrays.
