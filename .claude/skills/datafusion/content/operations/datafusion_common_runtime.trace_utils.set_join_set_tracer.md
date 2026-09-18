# `datafusion_common_runtime::trace_utils::set_join_set_tracer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common_runtime.trace_utils.set_join_set_tracer.json).

<a id="op-9fb358745619cc36e7601a21"></a>
## set_join_set_tracer

`function` · `datafusion_common_runtime::trace_utils::set_join_set_tracer` · datafusion-common-runtime 55.1.0

```rust
fn set_join_set_tracer(tracer: &'static dyn JoinSetTracer) -> Result<(), JoinSetTracerError>
```

Source: `src/trace_utils.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

Set the custom tracer for both futures and blocking closures.

This should be called once at startup. If called more than once, an
`Err(JoinSetTracerError)` is returned. If not called at all, a no-op tracer that does nothing
is used.
