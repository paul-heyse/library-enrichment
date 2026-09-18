# `datafusion_common::utils::longest_consecutive_prefix`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.longest_consecutive_prefix.json).

<a id="op-4c8193df0cb17a98415f46f5"></a>
## longest_consecutive_prefix

`function` · `datafusion_common::utils::longest_consecutive_prefix` · datafusion-common 55.1.0

```rust
fn longest_consecutive_prefix<T: Borrow<usize>>(sequence: impl IntoIterator<Item = T>) -> usize
```

Source: `src/utils/mod.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This function finds the longest prefix of the form 0, 1, 2, ... within the
collection `sequence`. Examples:
- For 0, 1, 2, 4, 5; we would produce 3, meaning 0, 1, 2 is the longest satisfying
  prefix.
- For 1, 2, 3, 4; we would produce 0, meaning there is no such prefix.
