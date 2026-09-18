# `datafusion_physical_plan::joins::join_hash_map::contain_hashes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.join_hash_map.contain_hashes.json).

<a id="op-7d8986c8d2cda86a5797917b"></a>
## contain_hashes

`function` · `datafusion_physical_plan::joins::join_hash_map::contain_hashes` · datafusion-physical-plan 55.1.0

```rust
fn contain_hashes<T>(map: &hashbrown::HashTable<(u64, T)>, hash_values: &[u64]) -> arrow::array::BooleanArray
```

Source: `src/joins/join_hash_map.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
