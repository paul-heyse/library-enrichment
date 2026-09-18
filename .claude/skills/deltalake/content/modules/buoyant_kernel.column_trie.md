# `buoyant_kernel::column_trie`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.column_trie.json).

<a id="op-e990e6dabb4c0484ced93591"></a>
## column_trie

`module` · `buoyant_kernel::column_trie` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod column_trie
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/column_trie.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/column_trie.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A trie (prefix tree) for efficient column path matching.

Used to quickly determine if a column path matches or is a descendant of any
user-specified column. This provides O(path_length) lookup instead of
O(num_specified_columns * path_length).
