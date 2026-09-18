# `datafusion_common_runtime::trace_utils::JoinSetTracer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common_runtime.trace_utils.JoinSetTracer.json).

<a id="op-f6a986111ea9f435bed839ff"></a>
## JoinSetTracer

`trait` · `datafusion_common_runtime::trace_utils::JoinSetTracer` · datafusion-common-runtime 55.1.0

```rust
trait JoinSetTracer: Send + Sync + 'static
```

Source: `src/trace_utils.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

A trait for injecting instrumentation into either asynchronous futures or
blocking closures at runtime.

<a id="op-6c76bd362bbffffc646c5b1d"></a>
## trace_block

`function` · `datafusion_common_runtime::trace_utils::JoinSetTracer::trace_block` · datafusion-common-runtime 55.1.0

```rust
fn trace_block(&self, f: Box<dyn FnOnce() -> Box<dyn Any + Send> + Send>) -> Box<dyn FnOnce() -> Box<dyn Any + Send> + Send>
```

Source: `src/trace_utils.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

Function pointer type for tracing a blocking closure.

This function takes a boxed closure (with its return type erased)
and returns a boxed closure (with its return type still erased). The
tracer must apply instrumentation without changing the return value.

<a id="op-c142a0bbba358d02e777a35e"></a>
## trace_future

`function` · `datafusion_common_runtime::trace_utils::JoinSetTracer::trace_future` · datafusion-common-runtime 55.1.0

```rust
fn trace_future(&self, fut: BoxFuture<'static, Box<dyn Any + Send>>) -> BoxFuture<'static, Box<dyn Any + Send>>
```

Source: `src/trace_utils.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

Function pointer type for tracing a future.

This function takes a boxed future (with its output type erased)
and returns a boxed future (with its output still erased). The
tracer must apply instrumentation without altering the output.
