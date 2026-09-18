# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::SeenValues`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.accumulate.SeenValues.json).

<a id="op-af67b77128edf7c2013c53d8"></a>
## SeenValues

`enum` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::SeenValues` · datafusion-functions-aggregate-common 55.1.0

```rust
enum SeenValues
```

Source: `src/aggregate/groups_accumulator/accumulate.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

If the input has nulls, then the accumulator must potentially
handle each input null value specially (e.g. for `SUM` to mark the
corresponding sum as null)

If there are filters present, `NullState` tracks if it has seen
*any* value for that group (as some values may be filtered
out). Without a filter, the accumulator is only passed groups that
had at least one value to accumulate so they do not need to track
if they have seen values for a particular group.

<a id="op-49246b5aa70ade2763768f7e"></a>
## All

`variant` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::SeenValues::All` · datafusion-functions-aggregate-common 55.1.0

```rust
All
```

Source: `src/aggregate/groups_accumulator/accumulate.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

All groups seen so far have seen at least one non-null value

<a id="op-19b10c04a62c9df36605dd32"></a>
## Some

`variant` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::SeenValues::Some` · datafusion-functions-aggregate-common 55.1.0

```rust
Some
```

Source: `src/aggregate/groups_accumulator/accumulate.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8950dc2b1bd89a6307cd1d3d"></a>
## default

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::SeenValues::default` · datafusion-functions-aggregate-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::SeenValues", "path": "SeenValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [54, 2], "filename": "src/aggregate/groups_accumulator/accumulate.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aggregate/groups_accumulator/accumulate.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57e0de6e3828eacd55993a1b"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::SeenValues::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::SeenValues", "path": "SeenValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/aggregate/groups_accumulator/accumulate.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate/groups_accumulator/accumulate.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
