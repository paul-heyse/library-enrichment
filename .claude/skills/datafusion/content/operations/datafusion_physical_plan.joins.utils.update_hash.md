# `datafusion_physical_plan::joins::utils::update_hash`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.utils.update_hash.json).

<a id="op-dd9b9b50b8b83b974b1c0e69"></a>
## update_hash

`function` · `datafusion_physical_plan::joins::utils::update_hash` · datafusion-physical-plan 55.1.0

```rust
fn update_hash(on: &[datafusion_physical_expr::PhysicalExprRef], batch: &arrow::array::RecordBatch, hash_map: &mut dyn JoinHashMapType, offset: usize, random_state: &datafusion_common::hash_utils::RandomState, hashes_buffer: &mut [u64], deleted_offset: usize, fifo_hashmap: bool, null_equality: datafusion_common::NullEquality) -> datafusion_common::Result<()>
```

Source: `src/joins/utils.rs:2124`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Updates `hash_map` with new entries from `batch` evaluated against the expressions `on`
using `offset` as a start value for `batch` row indices.

`fifo_hashmap` sets the order of iteration over `batch` rows while updating hashmap,
which allows to keep either first (if set to true) or last (if set to false) row index
as a chain head for rows with equal hash values.

Under [`NullEquality::NullEqualsNothing`](../operations/datafusion_common.null_equality.NullEquality.md#op-c116c80db5727dfee9e298d7), rows with a NULL in any key
column can never match a probe row, so they are not inserted into the map.
