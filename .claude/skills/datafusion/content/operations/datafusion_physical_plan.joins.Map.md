# `datafusion_physical_plan::joins::Map`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.Map.json).

<a id="op-019dac099651410387431f3a"></a>
## Map

`enum` · `datafusion_physical_plan::joins::Map` · datafusion-physical-plan 55.1.0

```rust
enum Map
```

Source: `src/joins/mod.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The build-side map of a hash join, indexing build rows by join key.

Under [`NullEquality::NullEqualsNothing`], build rows with a NULL in any
join key column can never match a probe row and are omitted from the map.
[`Map::is_empty`](../operations/datafusion_physical_plan.joins.Map.md#op-ab144bf75cae305f9af2adad) and [`Map::num_of_distinct_key`](../operations/datafusion_physical_plan.joins.Map.md#op-08e6ebbb63942fcfa263b53d) therefore reflect the
*matchable* build rows: the map can be empty even when the build side
contains rows.

[`NullEquality::NullEqualsNothing`]: datafusion_common::NullEquality::NullEqualsNothing

<a id="op-f61ac1472b545daf0e664a05"></a>
## ArrayMap

`variant` · `datafusion_physical_plan::joins::Map::ArrayMap` · datafusion-physical-plan 55.1.0

```rust
ArrayMap
```

Source: `src/joins/mod.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb69262acfabd432bb5d4900"></a>
## HashMap

`variant` · `datafusion_physical_plan::joins::Map::HashMap` · datafusion-physical-plan 55.1.0

```rust
HashMap
```

Source: `src/joins/mod.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab144bf75cae305f9af2adad"></a>
## is_empty

`function` · `datafusion_physical_plan::joins::Map::is_empty` · datafusion-physical-plan 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [82, 2], "filename": "src/joins/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/mod.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns `true` if the map contains no elements.

<a id="op-08e6ebbb63942fcfa263b53d"></a>
## num_of_distinct_key

`function` · `datafusion_physical_plan::joins::Map::num_of_distinct_key` · datafusion-physical-plan 55.1.0

```rust
fn num_of_distinct_key(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [82, 2], "filename": "src/joins/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/mod.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the number of elements in the map.
