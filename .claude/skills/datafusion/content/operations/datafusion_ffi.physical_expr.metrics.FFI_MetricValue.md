# `datafusion_ffi::physical_expr::metrics::FFI_MetricValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.physical_expr.metrics.FFI_MetricValue.json).

<a id="op-ccf6dece0245a469ba1b26ac"></a>
## FFI_MetricValue

`enum` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue` · datafusion-ffi 55.1.0

```rust
enum FFI_MetricValue
```

Source: `src/physical_expr/metrics.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI-stable mirror of [`MetricValue`].

This is part of the stable ABI and must not be reordered. New variants must be
appended at the end.

<a id="op-b0b115a833e42afdc3cf2b1f"></a>
## Count

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::Count` · datafusion-ffi 55.1.0

```rust
Count
```

Source: `src/physical_expr/metrics.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01c0bad19fa2e7d6b3635369"></a>
## CurrentMemoryUsage

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::CurrentMemoryUsage` · datafusion-ffi 55.1.0

```rust
CurrentMemoryUsage
```

Source: `src/physical_expr/metrics.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f5bdae6ad4828240a59db29"></a>
## Custom

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::Custom` · datafusion-ffi 55.1.0

```rust
Custom
```

Source: `src/physical_expr/metrics.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Custom metrics are marshalled as their `Display` output plus the
`as_usize()` fallback. The underlying `dyn CustomMetricValue` type is
not preserved across the boundary, so `aggregate`/`as_any` downcasting
are lost; the reconstructed value uses [`FfiCustomMetricValue`].

<a id="op-1b77a6c3e18ccef61a273a13"></a>
## ElapsedComputeNs

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::ElapsedComputeNs` · datafusion-ffi 55.1.0

```rust
ElapsedComputeNs
```

Source: `src/physical_expr/metrics.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1939586a4ca09785f7115d23"></a>
## EndTimestampNsUTC

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::EndTimestampNsUTC` · datafusion-ffi 55.1.0

```rust
EndTimestampNsUTC
```

Source: `src/physical_expr/metrics.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6266d523c727742465e78540"></a>
## Gauge

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::Gauge` · datafusion-ffi 55.1.0

```rust
Gauge
```

Source: `src/physical_expr/metrics.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6c0c209021733aede5c8800"></a>
## OutputBatches

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::OutputBatches` · datafusion-ffi 55.1.0

```rust
OutputBatches
```

Source: `src/physical_expr/metrics.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d2436d42b88916a1fda6cfc"></a>
## OutputBytes

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::OutputBytes` · datafusion-ffi 55.1.0

```rust
OutputBytes
```

Source: `src/physical_expr/metrics.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18ef1816ec0452ab4f7800c2"></a>
## OutputRows

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::OutputRows` · datafusion-ffi 55.1.0

```rust
OutputRows
```

Source: `src/physical_expr/metrics.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f3a9fa575601751c92d08ef"></a>
## PeakMemoryUsage

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::PeakMemoryUsage` · datafusion-ffi 55.1.0

```rust
PeakMemoryUsage
```

Source: `src/physical_expr/metrics.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ea046bd6c4cd4707020e3ee"></a>
## PruningMetrics

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::PruningMetrics` · datafusion-ffi 55.1.0

```rust
PruningMetrics
```

Source: `src/physical_expr/metrics.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ce5782b81036fbeae91071a"></a>
## Ratio

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::Ratio` · datafusion-ffi 55.1.0

```rust
Ratio
```

Source: `src/physical_expr/metrics.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fbd013b71a69a2cc2542102"></a>
## SpillCount

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::SpillCount` · datafusion-ffi 55.1.0

```rust
SpillCount
```

Source: `src/physical_expr/metrics.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb2a78af36e7618366a5853e"></a>
## SpilledBytes

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::SpilledBytes` · datafusion-ffi 55.1.0

```rust
SpilledBytes
```

Source: `src/physical_expr/metrics.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b75dc39ee24fd000d6e95f13"></a>
## SpilledRows

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::SpilledRows` · datafusion-ffi 55.1.0

```rust
SpilledRows
```

Source: `src/physical_expr/metrics.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-180985b6ae3dcf244a4f4240"></a>
## StartTimestampNsUTC

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::StartTimestampNsUTC` · datafusion-ffi 55.1.0

```rust
StartTimestampNsUTC
```

Source: `src/physical_expr/metrics.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-946887e64f33febbdb39503a"></a>
## Time

`variant` · `datafusion_ffi::physical_expr::metrics::FFI_MetricValue::Time` · datafusion-ffi 55.1.0

```rust
Time
```

Source: `src/physical_expr/metrics.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
