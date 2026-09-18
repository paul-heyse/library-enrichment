# `datafusion_physical_plan::joins::join_hash_map`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.join_hash_map.json).

<a id="op-b9d042664cdad13e778f0c4f"></a>
## join_hash_map

`module` · `datafusion_physical_plan::joins::join_hash_map` · datafusion-physical-plan 55.1.0

```rust
mod join_hash_map
```

Source: `src/joins/join_hash_map.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Hash map implementations for join operations.

Note: This module is public for internal testing purposes only
and is not guaranteed to be stable across versions.
This file contains the implementation of the `JoinHashMap` struct, which
is used to store the mapping between hash values based on the build side
["on" values] to a list of indices with this key's value.
