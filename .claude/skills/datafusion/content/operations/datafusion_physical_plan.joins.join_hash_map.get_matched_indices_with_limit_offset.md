# `datafusion_physical_plan::joins::join_hash_map::get_matched_indices_with_limit_offset`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.join_hash_map.get_matched_indices_with_limit_offset.json).

<a id="op-93e5f8d2583e12731f9bc432"></a>
## get_matched_indices_with_limit_offset

`function` · `datafusion_physical_plan::joins::join_hash_map::get_matched_indices_with_limit_offset` · datafusion-physical-plan 55.1.0

```rust
fn get_matched_indices_with_limit_offset<T>(map: &hashbrown::HashTable<(u64, T)>, next_chain: &[T], hash_values: &[u64], valid_keys: Option<&arrow::buffer::NullBuffer>, limit: usize, offset: (usize, Option<u64>), input_indices: &mut Vec<u32>, match_indices: &mut Vec<u64>) -> Option<(usize, Option<u64>)> where T: Copy + TryFrom<usize> + PartialOrd + Into<u64> + Sub<Output = T> + ArrowNativeType, <T as TryFrom>::Error: Debug
```

Source: `src/joins/join_hash_map.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
