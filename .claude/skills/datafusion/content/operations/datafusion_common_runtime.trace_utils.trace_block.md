# `datafusion_common_runtime::trace_utils::trace_block`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common_runtime.trace_utils.trace_block.json).

<a id="op-5b871fe4e9748b63a0d1e7b0"></a>
## trace_block

`function` · `datafusion_common_runtime::trace_utils::trace_block` · datafusion-common-runtime 55.1.0

```rust
fn trace_block<T, F>(f: F) -> Box<dyn FnOnce() -> T + Send> where F: FnOnce() -> T + Send + 'static, T: Send + 'static
```

Source: `src/trace_utils.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

Optionally instruments a blocking closure with custom tracing.

If a tracer has been injected via `set_tracer`, the closure is wrapped so that
its return value is boxed (erasing its type), passed to the tracer, and then the
result is downcast back to the original type. If no tracer is set, the closure is
returned unmodified (except for being boxed).

# Type Parameters
* `T` - The concrete return type of the closure.
* `F` - The closure type.

# Parameters
* `f` - The blocking closure to potentially instrument.
