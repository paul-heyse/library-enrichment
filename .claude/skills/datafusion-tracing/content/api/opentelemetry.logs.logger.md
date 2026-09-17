# `opentelemetry::logs::logger`

Crate `opentelemetry` · 2 public items · structured records in [`model/opentelemetry.logs.logger.json`](../model/opentelemetry.logs.logger.json)

## Logger

`trait` · `opentelemetry::logs::logger::Logger`

Also reachable as `opentelemetry::logs::Logger`

```rust
trait Logger
```

**Implementors** (1)

- `opentelemetry_sdk::logs::logger::SdkLogger`

**Methods** (3)

```rust
fn create_log_record(&self) -> Self::LogRecord
fn emit(&self, record: Self::LogRecord)
fn event_enabled(&self, level: Severity, target: &str, name: Option<&str>) -> bool
```

The interface for emitting [`LogRecord`]s.

---

## LoggerProvider

`trait` · `opentelemetry::logs::logger::LoggerProvider`

Also reachable as `opentelemetry::logs::LoggerProvider`

```rust
trait LoggerProvider
```

**Implementors** (2)

- `opentelemetry::logs::noop::NoopLoggerProvider`
- `opentelemetry_sdk::logs::logger_provider::SdkLoggerProvider`

**Methods** (2)

```rust
fn logger(&self, name: impl Into<Cow<'static, str>>) -> Self::Logger
fn logger_with_scope(&self, scope: InstrumentationScope) -> Self::Logger
```

Interfaces that can create [`Logger`] instances.

---
