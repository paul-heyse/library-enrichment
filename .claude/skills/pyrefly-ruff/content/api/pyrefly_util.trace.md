# `pyrefly_util::trace`

Crate `pyrefly_util` · 3 public items · structured records in [`model/pyrefly_util.trace.json`](../model/pyrefly_util.trace.json)

## init_tracing

`function` · `pyrefly_util::trace::init_tracing`

```rust
fn init_tracing(verbose: bool, testing: bool)
```

Set up tracing so it prints to stderr, and can be used for output.
Most things should use `info` and `debug` level for showing messages.

---

## tracing_layer

`function` · `pyrefly_util::trace::tracing_layer`

```rust
fn tracing_layer(verbose: bool, testing: bool) -> TracingLayer
```

Create a layer for user tracing.

---

## TracingLayer

`type_alias` · `pyrefly_util::trace::TracingLayer`

```rust
type TracingLayer = Box<dyn Layer<tracing_subscriber::Registry> + Send + Sync>
```

---
