# `datafusion_common_runtime::trace_utils::trace_future`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common_runtime.trace_utils.trace_future.json).

<a id="op-76ebf04874a6b368a1fa16c6"></a>
## trace_future

`function` · `datafusion_common_runtime::trace_utils::trace_future` · datafusion-common-runtime 55.1.0

```rust
fn trace_future<T, F>(future: F) -> futures::future::BoxFuture<'static, T> where F: Future<Output = T> + Send + 'static, T: Send + 'static
```

Source: `src/trace_utils.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

Optionally instruments a future with custom tracing.

If a tracer has been injected via `set_tracer`, the future's output is
boxed (erasing its type), passed to the tracer, and then downcast back
to the expected type. If no tracer is set, the original future is returned.

# Type Parameters
* `T` - The concrete output type of the future.
* `F` - The future type.

# Parameters
* `future` - The future to potentially instrument.
