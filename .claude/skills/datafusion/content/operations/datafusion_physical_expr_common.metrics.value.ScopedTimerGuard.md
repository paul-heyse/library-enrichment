# `datafusion_physical_expr_common::metrics::value::ScopedTimerGuard`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.value.ScopedTimerGuard.json).

<a id="op-e38b1a43fa9521debd6019bb"></a>
## ScopedTimerGuard

`struct` · `datafusion_physical_expr_common::metrics::value::ScopedTimerGuard` · datafusion-physical-expr-common 55.1.0

```rust
struct ScopedTimerGuard<'a>
```

Source: `src/metrics/value.rs:321`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

RAAI structure that adds all time between its construction and
destruction to the CPU time or the first call to `stop` whichever
comes first

<a id="op-8eebb064ba940573d83219f0"></a>
## done

`function` · `datafusion_physical_expr_common::metrics::value::ScopedTimerGuard::done` · datafusion-physical-expr-common 55.1.0

```rust
fn done(self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::value::ScopedTimerGuard", "path": "ScopedTimerGuard"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [357, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Stop the timer, record the time taken and consume self

<a id="op-72dfa0b46d13d0672dea9b67"></a>
## done_with

`function` · `datafusion_physical_expr_common::metrics::value::ScopedTimerGuard::done_with` · datafusion-physical-expr-common 55.1.0

```rust
fn done_with(self, end_time: Instant)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::value::ScopedTimerGuard", "path": "ScopedTimerGuard"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [357, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Stop the timer, record the time taken since `end_time` endpoint, and
consume self.

<a id="op-81d6796bcc7499d2ad30d8bd"></a>
## drop

`function` · `datafusion_physical_expr_common::metrics::value::ScopedTimerGuard::drop` · datafusion-physical-expr-common 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::value::ScopedTimerGuard", "path": "ScopedTimerGuard"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [359, 1], "end": [363, 2], "filename": "src/metrics/value.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/metrics/value.rs:360`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c82a298cdf9b13cefea7d25d"></a>
## restart

`function` · `datafusion_physical_expr_common::metrics::value::ScopedTimerGuard::restart` · datafusion-physical-expr-common 55.1.0

```rust
fn restart(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::value::ScopedTimerGuard", "path": "ScopedTimerGuard"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [357, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Restarts the timer recording from the current time

<a id="op-f97da1d9e30d77c9dd266d40"></a>
## stop

`function` · `datafusion_physical_expr_common::metrics::value::ScopedTimerGuard::stop` · datafusion-physical-expr-common 55.1.0

```rust
fn stop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::value::ScopedTimerGuard", "path": "ScopedTimerGuard"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [357, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Stop the timer timing and record the time taken

<a id="op-a19d68d3f84c1a43fbc3d881"></a>
## stop_with

`function` · `datafusion_physical_expr_common::metrics::value::ScopedTimerGuard::stop_with` · datafusion-physical-expr-common 55.1.0

```rust
fn stop_with(&mut self, end_time: Instant)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::value::ScopedTimerGuard", "path": "ScopedTimerGuard"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [357, 2], "filename": "src/metrics/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/value.rs:345`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Stop the timer timing and record the time taken since the given endpoint.
