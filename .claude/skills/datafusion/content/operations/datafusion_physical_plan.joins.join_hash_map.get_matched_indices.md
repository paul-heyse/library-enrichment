# `datafusion_physical_plan::joins::join_hash_map::get_matched_indices`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.join_hash_map.get_matched_indices.json).

<a id="op-4aa0412840e1a5e274e40616"></a>
## get_matched_indices

`function` · `datafusion_physical_plan::joins::join_hash_map::get_matched_indices` · datafusion-physical-plan 55.1.0

```rust
fn get_matched_indices<'a, T>(map: &hashbrown::HashTable<(u64, T)>, next: &[T], iter: Box<dyn Iterator<Item = (usize, &'a u64)> + 'a>, deleted_offset: Option<usize>) -> (Vec<u32>, Vec<u64>) where T: Copy + TryFrom<usize> + PartialOrd + Into<u64> + Sub<Output = T>, <T as TryFrom>::Error: Debug
```

Source: `src/joins/join_hash_map.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
