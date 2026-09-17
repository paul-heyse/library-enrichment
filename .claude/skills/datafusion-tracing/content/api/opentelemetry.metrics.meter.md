# `opentelemetry::metrics::meter`

Crate `opentelemetry` · 2 public items · structured records in [`model/opentelemetry.metrics.meter.json`](../model/opentelemetry.metrics.meter.json)

## Meter

`struct` · `opentelemetry::metrics::meter::Meter`

Also reachable as `opentelemetry::metrics::Meter`

```rust
struct Meter
```

**Derives**: Clone, Debug

**Methods** (16)

```rust
fn f64_counter(&self, name: impl Into<Cow<'static, str>>) -> InstrumentBuilder<'_, Counter<f64>>
fn f64_gauge(&self, name: impl Into<Cow<'static, str>>) -> InstrumentBuilder<'_, Gauge<f64>>
fn f64_histogram(&self, name: impl Into<Cow<'static, str>>) -> HistogramBuilder<'_, Histogram<f64>>
fn f64_observable_counter(&self, name: impl Into<Cow<'static, str>>) -> AsyncInstrumentBuilder<'_, ObservableCounter<f64>, f64>
fn f64_observable_gauge(&self, name: impl Into<Cow<'static, str>>) -> AsyncInstrumentBuilder<'_, ObservableGauge<f64>, f64>
fn f64_observable_up_down_counter(&self, name: impl Into<Cow<'static, str>>) -> AsyncInstrumentBuilder<'_, ObservableUpDownCounter<f64>, f64>
fn f64_up_down_counter(&self, name: impl Into<Cow<'static, str>>) -> InstrumentBuilder<'_, UpDownCounter<f64>>
fn i64_gauge(&self, name: impl Into<Cow<'static, str>>) -> InstrumentBuilder<'_, Gauge<i64>>
fn i64_observable_gauge(&self, name: impl Into<Cow<'static, str>>) -> AsyncInstrumentBuilder<'_, ObservableGauge<i64>, i64>
fn i64_observable_up_down_counter(&self, name: impl Into<Cow<'static, str>>) -> AsyncInstrumentBuilder<'_, ObservableUpDownCounter<i64>, i64>
fn i64_up_down_counter(&self, name: impl Into<Cow<'static, str>>) -> InstrumentBuilder<'_, UpDownCounter<i64>>
fn u64_counter(&self, name: impl Into<Cow<'static, str>>) -> InstrumentBuilder<'_, Counter<u64>>
fn u64_gauge(&self, name: impl Into<Cow<'static, str>>) -> InstrumentBuilder<'_, Gauge<u64>>
fn u64_histogram(&self, name: impl Into<Cow<'static, str>>) -> HistogramBuilder<'_, Histogram<u64>>
fn u64_observable_counter(&self, name: impl Into<Cow<'static, str>>) -> AsyncInstrumentBuilder<'_, ObservableCounter<u64>, u64>
fn u64_observable_gauge(&self, name: impl Into<Cow<'static, str>>) -> AsyncInstrumentBuilder<'_, ObservableGauge<u64>, u64>
```

Provides the ability to create instruments for recording measurements or
accepting callbacks to report measurements.

# Instrument Types

Instruments are categorized as either synchronous or asynchronous:

- **Synchronous Instruments** (e.g., Counter): These are used inline with
  your application's processing logic. For example, you might use a Counter
  to record the number of HTTP requests received.

- **Asynchronous Instruments** (e.g., ObservableGauge): These allow you to
  register a callback function that is invoked during export. For instance,
  you could use an asynchronous gauge to monitor temperature from a sensor
  every time metrics are exported.

# Example Usage

```rust
use opentelemetry::{global, KeyValue};

let meter = global::meter("my-meter");

// Synchronous Instruments

// u64 Counter
let u64_counter = meter.u64_counter("my_u64_counter").build();
u64_counter.add(
    10,
    &[
        KeyValue::new("mykey1", "myvalue1"),
        KeyValue::new("mykey2", "myvalue2"),
    ],
);

// f64 Counter
let f64_counter = meter.f64_counter("my_f64_counter").build();
f64_counter.add(
    3.15,
    &[
        KeyValue::new("mykey1", "myvalue1"),
        KeyValue::new("mykey2", "myvalue2"),
    ],
);


// u64 Observable Counter
let _observable_u64_counter = meter
    .u64_observable_counter("my_observable_u64_counter")
    .with_description("My observable counter example")
    .with_unit("myunit")
    .with_callback(|observer| {
        observer.observe(
            100,
            &[
                KeyValue::new("mykey1", "myvalue1"),
                KeyValue::new("mykey2", "myvalue2"),
            ],
        )
    })
    .build();

// f64 Observable Counter
let _observable_f64_counter = meter
    .f64_observable_counter("my_observable_f64_counter")
    .with_description("My observable counter example")
    .with_unit("myunit")
    .with_callback(|observer| {
        observer.observe(
            100.0,
            &[
                KeyValue::new("mykey1", "myvalue1"),
                KeyValue::new("mykey2", "myvalue2"),
            ],
        )
    })
    .build();

// i64 UpDownCounter
let updown_i64_counter = meter.i64_up_down_counter("my_updown_i64_counter").build();
updown_i64_counter.add(
    -10,
    &[
        KeyValue::new("mykey1", "myvalue1"),
        KeyValue::new("mykey2", "myvalue2"),
    ],
);

// f64 UpDownCounter
let updown_f64_counter = meter.f64_up_down_counter("my_updown_f64_counter").build();
updown_f64_counter.add(
    -10.67,
    &[
        KeyValue::new("mykey1", "myvalue1"),
        KeyValue::new("mykey2", "myvalue2"),
    ],
);

// i64 Observable UpDownCounter
let _observable_updown_i64_counter = meter
    .i64_observable_up_down_counter("my_observable_i64_updown_counter")
    .with_description("My observable updown counter example")
    .with_unit("myunit")
    .with_callback(|observer| {
        observer.observe(
            100,
            &[
                KeyValue::new("mykey1", "myvalue1"),
                KeyValue::new("mykey2", "myvalue2"),
            ],
        )
    })
    .build();

// f64 Observable UpDownCounter
let _observable_updown_f64_counter = meter
    .f64_observable_up_down_counter("my_observable_f64_updown_counter")
    .with_description("My observable updown counter example")
    .with_unit("myunit")
    .with_callback(|observer| {
        observer.observe(
            100.0,
            &[
                KeyValue::new("mykey1", "myvalue1"),
                KeyValue::new("mykey2", "myvalue2"),
            ],
        )
    })
    .build();

// i64 Gauge
let gauge = meter.i64_gauge("my_gauge").build();
gauge.record(
-10,
&[
    KeyValue::new("mykey1", "myvalue1"),
    KeyValue::new("mykey2", "myvalue2"),
],
);

// u64 Gauge
let gauge = meter.u64_gauge("my_gauge").build();
gauge.record(
101,
&[
    KeyValue::new("mykey1", "myvalue1"),
    KeyValue::new("mykey2", "myvalue2"),
],
);

// f64 Gauge
let gauge = meter.f64_gauge("my_gauge").build();
gauge.record(
12.5,
&[
    KeyValue::new("mykey1", "myvalue1"),
    KeyValue::new("mykey2", "myvalue2"),
],
);

// u64 Observable Gauge
let _observable_u64_gauge = meter
    .u64_observable_gauge("my_u64_gauge")
    .with_description("An observable gauge set to 1")
    .with_unit("myunit")
    .with_callback(|observer| {
        observer.observe(
            1,
            &[
                KeyValue::new("mykey1", "myvalue1"),
                KeyValue::new("mykey2", "myvalue2"),
            ],
        )
    })
    .build();

// f64 Observable Gauge
let _observable_f64_gauge = meter
    .f64_observable_gauge("my_f64_gauge")
    .with_description("An observable gauge set to 1.0")
    .with_unit("myunit")
    .with_callback(|observer| {
        observer.observe(
            1.0,
            &[
                KeyValue::new("mykey1", "myvalue1"),
                KeyValue::new("mykey2", "myvalue2"),
            ],
        )
    })
    .build();

// i64 Observable Gauge
let _observable_i64_gauge = meter
    .i64_observable_gauge("my_i64_gauge")
    .with_description("An observable gauge set to 1")
    .with_unit("myunit")
    .with_callback(|observer| {
        observer.observe(
            1,
            &[
                KeyValue::new("mykey1", "myvalue1"),
                KeyValue::new("mykey2", "myvalue2"),
            ],
        )
    })
    .build();

// f64 Histogram
let f64_histogram = meter.f64_histogram("my_f64_histogram").build();
f64_histogram.record(
    10.5,
    &[
        KeyValue::new("mykey1", "myvalue1"),
        KeyValue::new("mykey2", "myvalue2"),
    ],
);

// u64 Histogram
let u64_histogram = meter.u64_histogram("my_u64_histogram").build();
u64_histogram.record(
    12,
    &[
        KeyValue::new("mykey1", "myvalue1"),
        KeyValue::new("mykey2", "myvalue2"),
    ],
);
```

---

## MeterProvider

`trait` · `opentelemetry::metrics::meter::MeterProvider`

Also reachable as `opentelemetry::metrics::MeterProvider`

```rust
trait MeterProvider
```

**Implementors** (1)

- `opentelemetry_sdk::metrics::meter_provider::SdkMeterProvider`

**Methods** (2)

```rust
fn meter(&self, name: &'static str) -> Meter
fn meter_with_scope(&self, scope: InstrumentationScope) -> Meter
```

Provides access to named [Meter] instances, for instrumenting an application
or crate.

---
