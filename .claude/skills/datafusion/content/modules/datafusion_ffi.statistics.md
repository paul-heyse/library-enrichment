# `datafusion_ffi::statistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.statistics.json).

<a id="op-6870ad580bee1bb040fd5788"></a>
## statistics

`module` · `datafusion_ffi::statistics` · datafusion-ffi 55.1.0

```rust
mod statistics
```

Source: `src/statistics.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Helpers for moving [`Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6) across the FFI boundary as prost-encoded
`datafusion_proto_common::Statistics` bytes.

[`Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6) contains [`Precision<ScalarValue>`] for column min/max/sum,
and `ScalarValue` is a large enum that's impractical to mirror in
`#[repr(C)]`. The proto round-trip already exists in `datafusion-proto-common`
and is the same pattern used to ship filter expressions across the FFI
boundary, so we reuse it here.

[`Precision<ScalarValue>`]: datafusion_common::stats::Precision
