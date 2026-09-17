# `opentelemetry::context`

Crate `opentelemetry` · 2 public items · structured records in [`model/opentelemetry.context.json`](../model/opentelemetry.context.json)

## Context

`struct` · `opentelemetry::context::Context`

Also reachable as `opentelemetry::Context`

```rust
struct Context
```

**Implements**: `opentelemetry::baggage::BaggageExt`, `opentelemetry::trace::context::TraceContextExt`

**Derives**: Clone, Debug, Default

**Methods** (11)

```rust
fn attach(self) -> ContextGuard
fn current() -> Self
fn current_with_value<T: 'static + Send + Sync>(value: T) -> Self
fn enter_telemetry_suppressed_scope() -> ContextGuard
fn get<T: 'static>(&self) -> Option<&T>
fn is_current_telemetry_suppressed() -> bool
fn is_telemetry_suppressed(&self) -> bool
fn map_current<T>(f: impl FnOnce(&Context) -> T) -> T
fn new() -> Self
fn with_telemetry_suppressed(&self) -> Self
fn with_value<T: 'static + Send + Sync>(&self, value: T) -> Self
```

**via `opentelemetry::baggage::BaggageExt`**

```rust
fn baggage(&self) -> &Baggage
fn current_with_baggage<T: Into<Baggage>>(baggage: T) -> Self
fn with_baggage<T: Into<Baggage>>(&self, baggage: T) -> Self
fn with_cleared_baggage(&self) -> Self
```

**via `opentelemetry::trace::context::TraceContextExt`**

```rust
fn current_with_span<T: trace::Span + Send + Sync + 'static>(span: T) -> Self
fn has_active_span(&self) -> bool
fn span(&self) -> SpanRef<'_>
fn with_remote_span_context(&self, span_context: trace::SpanContext) -> Self
fn with_span<T: trace::Span + Send + Sync + 'static>(&self, span: T) -> Self
```

An execution-scoped collection of values.

A [`Context`] is a propagation mechanism which carries execution-scoped
values across API boundaries and between logically associated execution
units. Cross-cutting concerns access their data in-process using the same
shared context object.

[`Context`]s are immutable, and their write operations result in the creation
of a new context containing the original values and the new specified values.

## Context state

Concerns can create and retrieve their local state in the current execution
state represented by a context through the [`get`] and [`with_value`]
methods. It is recommended to use application-specific types when storing new
context values to avoid unintentionally overwriting existing state.

## Managing the current context

Contexts can be associated with the caller's current execution unit on a
given thread via the [`attach`] method, and previous contexts can be restored
by dropping the returned [`ContextGuard`]. Context can be nested, and will
restore their parent outer context when detached on drop. To access the
values of the context, a snapshot can be created via the [`Context::current`]
method.

[`Context::current`]: Context::current()
[`get`]: Context::get()
[`with_value`]: Context::with_value()
[`attach`]: Context::attach()

# Examples

```
use opentelemetry::Context;

// Application-specific `a` and `b` values
#[derive(Debug, PartialEq)]
struct ValueA(&'static str);
#[derive(Debug, PartialEq)]
struct ValueB(u64);

let _outer_guard = Context::new().with_value(ValueA("a")).attach();

// Only value a has been set
let current = Context::current();
assert_eq!(current.get::<ValueA>(), Some(&ValueA("a")));
assert_eq!(current.get::<ValueB>(), None);

{
    let _inner_guard = Context::current_with_value(ValueB(42)).attach();
    // Both values are set in inner context
    let current = Context::current();
    assert_eq!(current.get::<ValueA>(), Some(&ValueA("a")));
    assert_eq!(current.get::<ValueB>(), Some(&ValueB(42)));
}

// Resets to only the `a` value when inner guard is dropped
let current = Context::current();
assert_eq!(current.get::<ValueA>(), Some(&ValueA("a")));
assert_eq!(current.get::<ValueB>(), None);
```

---

## ContextGuard

`struct` · `opentelemetry::context::ContextGuard`

Also reachable as `opentelemetry::ContextGuard`

```rust
struct ContextGuard
```

**Implements**: `core::ops::drop::Drop`

**Derives**: Debug

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A guard that resets the current context to the prior context when dropped.

---
