# `datafusion_common_runtime::trace_utils`

Crate `datafusion-common-runtime` · 5 public items · structured records in [`model/datafusion_common_runtime.trace_utils.json`](../model/datafusion_common_runtime.trace_utils.json)

## JoinSetTracerError

`enum` · `datafusion_common_runtime::trace_utils::JoinSetTracerError`

Also reachable as `datafusion::common::runtime::JoinSetTracerError`, `datafusion_common_runtime::JoinSetTracerError`

```rust
enum JoinSetTracerError
```

**Variants**: `AlreadySet`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
```

[Full member, field, variant and typed contracts](../operations/datafusion_common_runtime.trace_utils.JoinSetTracerError.md).


A custom error type for tracer injection failures.

---

## set_join_set_tracer

`function` · `datafusion_common_runtime::trace_utils::set_join_set_tracer`

Also reachable as `datafusion::common::runtime::set_join_set_tracer`, `datafusion_common_runtime::set_join_set_tracer`

```rust
fn set_join_set_tracer(tracer: &'static dyn JoinSetTracer) -> Result<(), JoinSetTracerError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common_runtime.trace_utils.set_join_set_tracer.md).


Set the custom tracer for both futures and blocking closures.

This should be called once at startup. If called more than once, an
`Err(JoinSetTracerError)` is returned. If not called at all, a no-op tracer that does nothing
is used.

---

## trace_block

`function` · `datafusion_common_runtime::trace_utils::trace_block`

Also reachable as `datafusion::common::runtime::trace_block`, `datafusion_common_runtime::trace_block`

```rust
fn trace_block<T, F>(f: F) -> Box<dyn FnOnce() -> T + Send> where F: FnOnce() -> T + Send + 'static, T: Send + 'static
```

[Full member, field, variant and typed contracts](../operations/datafusion_common_runtime.trace_utils.trace_block.md).


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

---

## trace_future

`function` · `datafusion_common_runtime::trace_utils::trace_future`

Also reachable as `datafusion::common::runtime::trace_future`, `datafusion_common_runtime::trace_future`

```rust
fn trace_future<T, F>(future: F) -> futures::future::BoxFuture<'static, T> where F: Future<Output = T> + Send + 'static, T: Send + 'static
```

[Full member, field, variant and typed contracts](../operations/datafusion_common_runtime.trace_utils.trace_future.md).


Optionally instruments a future with custom tracing.

If a tracer has been injected via `set_tracer`, the future's output is
boxed (erasing its type), passed to the tracer, and then downcast back
to the expected type. If no tracer is set, the original future is returned.

# Type Parameters
* `T` - The concrete output type of the future.
* `F` - The future type.

# Parameters
* `future` - The future to potentially instrument.

---

## JoinSetTracer

`trait` · `datafusion_common_runtime::trace_utils::JoinSetTracer`

Also reachable as `datafusion::common::runtime::JoinSetTracer`, `datafusion_common_runtime::JoinSetTracer`

```rust
trait JoinSetTracer: Send + Sync + 'static
```

**Methods** (2)

```rust
fn trace_block(&self, f: Box<dyn FnOnce() -> Box<dyn Any + Send> + Send>) -> Box<dyn FnOnce() -> Box<dyn Any + Send> + Send>
fn trace_future(&self, fut: BoxFuture<'static, Box<dyn Any + Send>>) -> BoxFuture<'static, Box<dyn Any + Send>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common_runtime.trace_utils.JoinSetTracer.md).


A trait for injecting instrumentation into either asynchronous futures or
blocking closures at runtime.

---
