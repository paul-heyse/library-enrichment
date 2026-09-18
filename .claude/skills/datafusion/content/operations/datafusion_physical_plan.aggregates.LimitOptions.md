# `datafusion_physical_plan::aggregates::LimitOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.LimitOptions.json).

<a id="op-865c00886b5e3560e9476be2"></a>
## LimitOptions

`struct` · `datafusion_physical_plan::aggregates::LimitOptions` · datafusion-physical-plan 55.1.0

```rust
struct LimitOptions
```

Source: `src/aggregates/mod.rs:796`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Configuration for limit-based optimizations in aggregation

<a id="op-091212be7c53031868126467"></a>
## clone

`function` · `datafusion_physical_plan::aggregates::LimitOptions::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> LimitOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::LimitOptions", "path": "LimitOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [795, 17], "end": [795, 22], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aggregates/mod.rs:795`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36907ba1a064f94c4696326e"></a>
## descending

`struct_field` · `datafusion_physical_plan::aggregates::LimitOptions::descending` · datafusion-physical-plan 55.1.0

```rust
descending: Option<bool>
```

Source: `src/aggregates/mod.rs:801`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Optional ordering direction (true = descending, false = ascending)
This is used for TopK aggregation to maintain a priority queue with the correct ordering

<a id="op-c1adcdeb932100858b35a0f5"></a>
## descending

`function` · `datafusion_physical_plan::aggregates::LimitOptions::descending` · datafusion-physical-plan 55.1.0

```rust
fn descending(&self) -> Option<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::LimitOptions", "path": "LimitOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [804, 1], "end": [828, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:825`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed47592c8ceb087260ac6c7a"></a>
## eq

`function` · `datafusion_physical_plan::aggregates::LimitOptions::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &LimitOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::LimitOptions", "path": "LimitOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [795, 30], "end": [795, 39], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/aggregates/mod.rs:795`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf3ef323aac7451999937214"></a>
## fmt

`function` · `datafusion_physical_plan::aggregates::LimitOptions::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::LimitOptions", "path": "LimitOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [795, 10], "end": [795, 15], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregates/mod.rs:795`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae53924722c82a7f6e36672c"></a>
## limit

`function` · `datafusion_physical_plan::aggregates::LimitOptions::limit` · datafusion-physical-plan 55.1.0

```rust
fn limit(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::LimitOptions", "path": "LimitOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [804, 1], "end": [828, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:821`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f708321b273da0a05753967c"></a>
## limit

`struct_field` · `datafusion_physical_plan::aggregates::LimitOptions::limit` · datafusion-physical-plan 55.1.0

```rust
limit: usize
```

Source: `src/aggregates/mod.rs:798`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The maximum number of rows to return

<a id="op-a0e0d6718b01e9c26e12a788"></a>
## new

`function` · `datafusion_physical_plan::aggregates::LimitOptions::new` · datafusion-physical-plan 55.1.0

```rust
fn new(limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::LimitOptions", "path": "LimitOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [804, 1], "end": [828, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:806`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new LimitOptions with a limit and no specific ordering

<a id="op-2c98a039a71fbbcb27729f60"></a>
## new_with_order

`function` · `datafusion_physical_plan::aggregates::LimitOptions::new_with_order` · datafusion-physical-plan 55.1.0

```rust
fn new_with_order(limit: usize, descending: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::LimitOptions", "path": "LimitOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [804, 1], "end": [828, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:814`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new LimitOptions with a limit and ordering direction
