# `datafusion_tracing::exec_instrument_rule`

Crate `datafusion-tracing` · 7 public items · structured records in [`model/datafusion_tracing.exec_instrument_rule.json`](../model/datafusion_tracing.exec_instrument_rule.json)

## new_instrument_rule

`function` · `datafusion_tracing::exec_instrument_rule::new_instrument_rule`

```rust
fn new_instrument_rule(span_create_fn: std::sync::Arc<dyn Fn() -> tracing::Span + Send + Sync>, options: options::InstrumentationOptions) -> std::sync::Arc<dyn PhysicalOptimizerRule + Send + Sync>
```

---

## INIT

`static` · `datafusion_tracing::exec_instrument_rule::INIT`

```rust
static INIT: std::sync::Once
```

---

## InstrumentRule

`struct` · `datafusion_tracing::exec_instrument_rule::InstrumentRule`

```rust
struct InstrumentRule
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Debug

**via `datafusion_session::physical_optimizer::PhysicalOptimizerRule`**

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> datafusion::error::Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

---

## SpanTracer

`struct` · `datafusion_tracing::exec_instrument_rule::SpanTracer`

```rust
struct SpanTracer
```

**Implements**: `datafusion_common_runtime::trace_utils::JoinSetTracer`

**via `datafusion_common_runtime::trace_utils::JoinSetTracer`**

```rust
fn trace_block(&self, f: Box<dyn FnOnce() -> Box<dyn Any + Send> + Send>) -> Box<dyn FnOnce() -> Box<dyn Any + Send> + Send>
fn trace_future(&self, fut: futures::future::BoxFuture<'static, Box<dyn Any + Send>>) -> futures::future::BoxFuture<'static, Box<dyn Any + Send>>
```

A simple tracer that ensures any spawned task or blocking closure
inherits the current span via `in_current_span`.

---

## BoxedAny

`type_alias` · `datafusion_tracing::exec_instrument_rule::BoxedAny`

```rust
type BoxedAny = Box<dyn Any + Send>
```

---

## BoxedClosure

`type_alias` · `datafusion_tracing::exec_instrument_rule::BoxedClosure`

```rust
type BoxedClosure = Box<dyn FnOnce() -> Box<dyn Any + Send> + Send>
```

---

## BoxedFuture

`type_alias` · `datafusion_tracing::exec_instrument_rule::BoxedFuture`

```rust
type BoxedFuture = futures::future::BoxFuture<'static, Box<dyn Any + Send>>
```

---
