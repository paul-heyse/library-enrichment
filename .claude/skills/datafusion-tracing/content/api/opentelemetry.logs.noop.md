# `opentelemetry::logs::noop`

Crate `opentelemetry` · 3 public items · structured records in [`model/opentelemetry.logs.noop.json`](../model/opentelemetry.logs.noop.json)

## NoopLogRecord

`struct` · `opentelemetry::logs::noop::NoopLogRecord`

```rust
struct NoopLogRecord
```

A no-operation log record that implements the LogRecord trait.

---

## NoopLogger

`struct` · `opentelemetry::logs::noop::NoopLogger`

```rust
struct NoopLogger
```

A no-op implementation of a [`Logger`]

---

## NoopLoggerProvider

`struct` · `opentelemetry::logs::noop::NoopLoggerProvider`

Also reachable as `opentelemetry::logs::NoopLoggerProvider`

```rust
struct NoopLoggerProvider
```

**Implements**: `opentelemetry::logs::logger::LoggerProvider`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `opentelemetry::logs::logger::LoggerProvider`**

```rust
fn logger_with_scope(&self, _scope: InstrumentationScope) -> Self::Logger
```

A no-op implementation of a [`LoggerProvider`].

---
