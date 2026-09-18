# `buoyant_kernel::engine::arrow_utils::fix_nested_null_masks`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_utils.fix_nested_null_masks.json).

<a id="op-38b7d85965bdd1f233427ad0"></a>
## fix_nested_null_masks

`function` · `buoyant_kernel::engine::arrow_utils::fix_nested_null_masks` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fix_nested_null_masks(batch: arrow::array::StructArray) -> arrow::array::StructArray
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L1099).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:1099`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Use this function to recursively compute properly unioned null masks for all nested
columns of a record batch, making it safe to project out and consume nested columns.

Arrow does not guarantee that the null masks associated with nested columns are accurate --
instead, the reader must consult the union of logical null masks the column and all
ancestors. The parquet reader stopped doing this automatically as of arrow-53.3, for example.
