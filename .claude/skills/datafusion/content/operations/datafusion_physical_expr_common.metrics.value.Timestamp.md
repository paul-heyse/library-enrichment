# `datafusion_physical_expr_common::metrics::value::Timestamp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.value.Timestamp.json).

<a id="op-6a83426f212eb627298de025"></a>
## Timestamp

`struct` · `datafusion_physical_expr_common::metrics::value::Timestamp` · datafusion-physical-expr-common 55.1.0

```rust
struct Timestamp
```

Source: `src/metrics/value.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Stores a single timestamp, stored as the number of nanoseconds
elapsed from Jan 1, 1970 UTC

<a id="op-fdd04fa6dc8e8cb1b595af77"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::value::Timestamp::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> Timestamp
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Timestamp", "path": "Timestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 17], "end": [238, 22], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/value.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c679c7c59581a9df3a987ec"></a>
## default

`function` · `datafusion_physical_expr_common::metrics::value::Timestamp::default` · datafusion-physical-expr-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Timestamp", "path": "Timestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [248, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/metrics/value.rs:245`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d85a0735f3d1cce82e285d4c"></a>
## eq

`function` · `datafusion_physical_expr_common::metrics::value::Timestamp::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Timestamp", "path": "Timestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [301, 1], "end": [305, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/metrics/value.rs:302`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51b0a1b4b6e99919d8c276f9"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::Timestamp::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Timestamp", "path": "Timestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 10], "end": [238, 15], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/value.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a39d503279b335ac1f2d0268"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::Timestamp::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Timestamp", "path": "Timestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [307, 1], "end": [316, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/metrics/value.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0776f9f3b39ce5802772c71b"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::value::Timestamp::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Timestamp", "path": "Timestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [299, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new timestamp and sets its value to 0

<a id="op-cd0411654192a22b571b9783"></a>
## record

`function` · `datafusion_physical_expr_common::metrics::value::Timestamp::record` · datafusion-physical-expr-common 55.1.0

```rust
fn record(&self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Timestamp", "path": "Timestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [299, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Sets the timestamps value to the current time

<a id="op-1b75008f124f14d6a7f7b066"></a>
## set

`function` · `datafusion_physical_expr_common::metrics::value::Timestamp::set` · datafusion-physical-expr-common 55.1.0

```rust
fn set(&self, now: DateTime<Utc>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Timestamp", "path": "Timestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [299, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Sets the timestamps value to a specified time

<a id="op-b640670eb22d148210c430ed"></a>
## update_to_max

`function` · `datafusion_physical_expr_common::metrics::value::Timestamp::update_to_max` · datafusion-physical-expr-common 55.1.0

```rust
fn update_to_max(&self, other: &Timestamp)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Timestamp", "path": "Timestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [299, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

sets the value of this timestamp to the maximum of this and other

<a id="op-8c9cd6f59ef8676bf307daa3"></a>
## update_to_min

`function` · `datafusion_physical_expr_common::metrics::value::Timestamp::update_to_min` · datafusion-physical-expr-common 55.1.0

```rust
fn update_to_min(&self, other: &Timestamp)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Timestamp", "path": "Timestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [299, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

sets the value of this timestamp to the minimum of this and other

<a id="op-89342f9a6e956bbfd4886f62"></a>
## value

`function` · `datafusion_physical_expr_common::metrics::value::Timestamp::value` · datafusion-physical-expr-common 55.1.0

```rust
fn value(&self) -> Option<DateTime<Utc>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Timestamp", "path": "Timestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [299, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:272`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

return the timestamps value at the last time `record()` was
called.

Returns `None` if `record()` has not been called
