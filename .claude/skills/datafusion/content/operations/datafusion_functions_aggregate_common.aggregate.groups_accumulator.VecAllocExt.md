# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::VecAllocExt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.VecAllocExt.json).

<a id="op-daa7a1c9504fe9c126535182"></a>
## VecAllocExt

`trait` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::VecAllocExt` · datafusion-functions-aggregate-common 55.1.0

```rust
trait VecAllocExt
```

Source: `src/aggregate/groups_accumulator.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Extension trait for [`Vec`] to account for allocations.

Unresolved upstream links (retained, not inferred): ``Vec``.

<a id="op-93e636b0ada113f3c16c88fb"></a>
## T

`assoc_type` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::VecAllocExt::T` · datafusion-functions-aggregate-common 55.1.0

```rust
T
```

Source: `src/aggregate/groups_accumulator.rs:449`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Item type.

<a id="op-a5c97349099227eb1531ee7e"></a>
## allocated_size

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::VecAllocExt::allocated_size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn allocated_size(&self) -> usize
```

Source: `src/aggregate/groups_accumulator.rs:453`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Return the amount of memory allocated by this Vec (not
recursively counting any heap allocations contained within the
structure). Does not include the size of `self`
