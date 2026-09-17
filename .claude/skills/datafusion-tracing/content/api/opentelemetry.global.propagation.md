# `opentelemetry::global::propagation`

Crate `opentelemetry` · 2 public items · structured records in [`model/opentelemetry.global.propagation.json`](../model/opentelemetry.global.propagation.json)

## get_text_map_propagator

`function` · `opentelemetry::global::propagation::get_text_map_propagator`

Also reachable as `opentelemetry::global::get_text_map_propagator`

```rust
fn get_text_map_propagator<T, F>(f: F) -> T where F: FnMut(&dyn TextMapPropagator) -> T
```

Executes a closure with a reference to the current global [`TextMapPropagator`] propagator.

---

## set_text_map_propagator

`function` · `opentelemetry::global::propagation::set_text_map_propagator`

Also reachable as `opentelemetry::global::set_text_map_propagator`

```rust
fn set_text_map_propagator<P: TextMapPropagator + Send + Sync + 'static>(propagator: P)
```

Sets the given [`TextMapPropagator`] propagator as the current global propagator.

---
