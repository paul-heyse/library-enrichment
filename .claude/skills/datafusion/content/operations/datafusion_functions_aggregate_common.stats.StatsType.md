# `datafusion_functions_aggregate_common::stats::StatsType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.stats.StatsType.json).

<a id="op-46e10a83bfd19ec2e4d1f9f1"></a>
## StatsType

`enum` · `datafusion_functions_aggregate_common::stats::StatsType` · datafusion-functions-aggregate-common 55.1.0

```rust
enum StatsType
```

Source: `src/stats.rs:21`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

TODO: Move this to functions-aggregate module
Enum used for differentiating population and sample for statistical functions

<a id="op-fa365c266681fcf3d1f90ca9"></a>
## Population

`variant` · `datafusion_functions_aggregate_common::stats::StatsType::Population` · datafusion-functions-aggregate-common 55.1.0

```rust
Population
```

Source: `src/stats.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Population

<a id="op-10d1b3a73b9673dacfecfcdd"></a>
## Sample

`variant` · `datafusion_functions_aggregate_common::stats::StatsType::Sample` · datafusion-functions-aggregate-common 55.1.0

```rust
Sample
```

Source: `src/stats.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Sample

<a id="op-0dde55028c66921036b9ccc8"></a>
## clone

`function` · `datafusion_functions_aggregate_common::stats::StatsType::clone` · datafusion-functions-aggregate-common 55.1.0

```rust
fn clone(&self) -> StatsType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::stats::StatsType", "path": "StatsType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 32], "end": [20, 37], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/stats.rs:20`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94a09a9f7b7cebc628930f46"></a>
## eq

`function` · `datafusion_functions_aggregate_common::stats::StatsType::eq` · datafusion-functions-aggregate-common 55.1.0

```rust
fn eq(&self, other: &StatsType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::stats::StatsType", "path": "StatsType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 10], "end": [20, 19], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/stats.rs:20`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-837cb12732c3d13cab6e2e76"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::stats::StatsType::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::stats::StatsType", "path": "StatsType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 25], "end": [20, 30], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stats.rs:20`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f74efe631e5aa2f249271baf"></a>
## hash

`function` · `datafusion_functions_aggregate_common::stats::StatsType::hash` · datafusion-functions-aggregate-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::stats::StatsType", "path": "StatsType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 45], "end": [20, 49], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/stats.rs:20`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
