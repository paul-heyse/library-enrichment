# `datafusion_physical_expr_common::metrics::value::Time`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.value.Time.json).

<a id="op-0552155cccfca2434f2fc732"></a>
## Time

`struct` · `datafusion_physical_expr_common::metrics::value::Time` · datafusion-physical-expr-common 55.1.0

```rust
struct Time
```

Source: `src/metrics/value.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Measure a potentially non contiguous duration of time

<a id="op-fab024141a8fe1773928e458"></a>
## add

`function` · `datafusion_physical_expr_common::metrics::value::Time::add` · datafusion-physical-expr-common 55.1.0

```rust
fn add(&self, other: &Time)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Time", "path": "Time"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [234, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add the number of nanoseconds of other `Time` to self

<a id="op-cc942d629ff9bb05e8ea9250"></a>
## add_duration

`function` · `datafusion_physical_expr_common::metrics::value::Time::add_duration` · datafusion-physical-expr-common 55.1.0

```rust
fn add_duration(&self, duration: Duration)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Time", "path": "Time"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [234, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add duration of time to self

Note: this will always increment the recorded time by at least 1 nanosecond
to distinguish between the scenario of no values recorded, in which
case the value will be 0, and no measurable amount of time having passed,
in which case the value will be small but not 0.

This is based on the assumption that the timing logic in most cases is likely
to take at least a nanosecond, and so this is reasonable mechanism to avoid
ambiguity, especially on systems with low-resolution monotonic clocks

<a id="op-ae2ff35a621f0176f1795fa4"></a>
## add_elapsed

`function` · `datafusion_physical_expr_common::metrics::value::Time::add_elapsed` · datafusion-physical-expr-common 55.1.0

```rust
fn add_elapsed(&self, start: Instant)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Time", "path": "Time"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [234, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Add elapsed nanoseconds since `start`to self

<a id="op-d7f28a39980fe1f86cd91989"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::value::Time::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> Time
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Time", "path": "Time"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 17], "end": [153, 22], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/value.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b2abd397efd23c6bebc26f3"></a>
## default

`function` · `datafusion_physical_expr_common::metrics::value::Time::default` · datafusion-physical-expr-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Time", "path": "Time"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [159, 1], "end": [163, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/metrics/value.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba6ff794b0f11c092d719613"></a>
## eq

`function` · `datafusion_physical_expr_common::metrics::value::Time::eq` · datafusion-physical-expr-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Time", "path": "Time"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 1], "end": [169, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/metrics/value.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d048e83504e1bbf742dd9a8"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::Time::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Time", "path": "Time"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 10], "end": [153, 15], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/value.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba206e5f553928ef989a63ce"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::value::Time::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Time", "path": "Time"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [175, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/metrics/value.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a8cc88d1e421c836b2acbf8"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::value::Time::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Time", "path": "Time"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [234, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create a new [`Time`](../operations/datafusion_physical_expr_common.metrics.value.Time.md#op-0552155cccfca2434f2fc732) wrapper suitable for recording elapsed
times for operations.

<a id="op-51d5a8592fffa95304bce186"></a>
## timer

`function` · `datafusion_physical_expr_common::metrics::value::Time::timer` · datafusion-physical-expr-common 55.1.0

```rust
fn timer(&self) -> ScopedTimerGuard<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Time", "path": "Time"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [234, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

return a scoped guard that adds the amount of time elapsed
between its creation and its drop or call to `stop` to the
underlying metric.

<a id="op-53cce3bcc287c0bc97a3e9b1"></a>
## timer_with

`function` · `datafusion_physical_expr_common::metrics::value::Time::timer_with` · datafusion-physical-expr-common 55.1.0

```rust
fn timer_with(&self, now: Instant) -> ScopedTimerGuard<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Time", "path": "Time"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [234, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return a scoped guard that adds the amount of time elapsed between the
given instant and its drop (or the call to `stop`) to the underlying metric

<a id="op-519f815e76e14d96ad21cfeb"></a>
## value

`function` · `datafusion_physical_expr_common::metrics::value::Time::value` · datafusion-physical-expr-common 55.1.0

```rust
fn value(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::value::Time", "path": "Time"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [234, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Get the number of nanoseconds record by this Time metric
