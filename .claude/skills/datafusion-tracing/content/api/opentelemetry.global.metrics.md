# `opentelemetry::global::metrics`

Crate `opentelemetry` · 4 public items · structured records in [`model/opentelemetry.global.metrics.json`](../model/opentelemetry.global.metrics.json)

## meter

`function` · `opentelemetry::global::metrics::meter`

Also reachable as `opentelemetry::global::meter`

```rust
fn meter(name: &'static str) -> metrics::Meter
```

Creates a named [`Meter`] via the currently configured global [`MeterProvider`].

This is a more convenient way of expressing `global::meter_provider().meter(name)`.

**NOTE:** Calls to [`meter()`] return a [`Meter`] backed by the global [`MeterProvider`] configured during the method invocation.
If the global [`MeterProvider`] is changed after getting [`Meter`] instances from these calls, the [`Meter`] instances returned will not reflect the change.

---

## meter_provider

`function` · `opentelemetry::global::metrics::meter_provider`

Also reachable as `opentelemetry::global::meter_provider`

```rust
fn meter_provider() -> std::sync::Arc<dyn MeterProvider + Send + Sync>
```

Returns an instance of the currently configured global [`MeterProvider`].

---

## meter_with_scope

`function` · `opentelemetry::global::metrics::meter_with_scope`

Also reachable as `opentelemetry::global::meter_with_scope`

```rust
fn meter_with_scope(scope: InstrumentationScope) -> metrics::Meter
```

Creates a [`Meter`] with the given instrumentation scope.

This is a simpler alternative to `global::meter_provider().meter_with_scope(...)`

**NOTE:** Calls to [`meter_with_scope()`] return a [`Meter`] backed by the global [`MeterProvider`] configured during the method invocation.
If the global [`MeterProvider`] is changed after getting [`Meter`] instances from these calls, the [`Meter`] instances returned will not reflect the change.

# Example

```
use std::sync::Arc;
use opentelemetry::global::meter_with_scope;
use opentelemetry::InstrumentationScope;
use opentelemetry::KeyValue;

let scope = InstrumentationScope::builder("io.opentelemetry")
    .with_version("0.17")
    .with_schema_url("https://opentelemetry.io/schema/1.2.0")
    .with_attributes(vec![(KeyValue::new("key", "value"))])
    .build();

let meter = meter_with_scope(scope);
```

---

## set_meter_provider

`function` · `opentelemetry::global::metrics::set_meter_provider`

Also reachable as `opentelemetry::global::set_meter_provider`

```rust
fn set_meter_provider<P>(new_provider: P) where P: metrics::MeterProvider + Send + Sync + 'static
```

Sets the given [`MeterProvider`] instance as the current global meter
provider.
Libraries should NOT call this function. It is intended for applications/executables.

**NOTE:** This function should be called before getting [`Meter`] instances via [`meter()`] or [`meter_with_scope()`]. Otherwise, you could get no-op [`Meter`] instances.

---
