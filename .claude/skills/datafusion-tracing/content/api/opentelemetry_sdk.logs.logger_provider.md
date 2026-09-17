# `opentelemetry_sdk::logs::logger_provider`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.logs.logger_provider.json`](../model/opentelemetry_sdk.logs.logger_provider.json)

## LoggerProviderBuilder

`struct` · `opentelemetry_sdk::logs::logger_provider::LoggerProviderBuilder`

Also reachable as `opentelemetry_sdk::logs::LoggerProviderBuilder`

```rust
struct LoggerProviderBuilder
```

**Derives**: Debug, Default

**Methods** (5)

```rust
fn build(self) -> SdkLoggerProvider
fn with_batch_exporter<T: LogExporter + 'static>(self, exporter: T) -> Self
fn with_log_processor<T: LogProcessor + 'static>(self, processor: T) -> Self
fn with_resource(self, resource: Resource) -> Self
fn with_simple_exporter<T: LogExporter + 'static>(self, exporter: T) -> Self
```

Builder for provider attributes.

---

## SdkLoggerProvider

`struct` · `opentelemetry_sdk::logs::logger_provider::SdkLoggerProvider`

Also reachable as `opentelemetry_sdk::logs::SdkLoggerProvider`

```rust
struct SdkLoggerProvider
```

**Implements**: `opentelemetry::logs::logger::LoggerProvider`

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn builder() -> LoggerProviderBuilder
fn force_flush(&self) -> OTelSdkResult
fn shutdown(&self) -> OTelSdkResult
fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult
```

**via `opentelemetry::logs::logger::LoggerProvider`**

```rust
fn logger(&self, name: impl Into<Cow<'static, str>>) -> Self::Logger
fn logger_with_scope(&self, scope: InstrumentationScope) -> Self::Logger
```

Handles the creation and coordination of [`Logger`]s.

All `Logger`s created by a `SdkLoggerProvider` will share the same
[`Resource`] and have their created log records processed by the
configured log processors. This is a clonable handle to the `SdkLoggerProvider`
itself, and cloning it will create a new reference, not a new instance of a
`SdkLoggerProvider`. Dropping the last reference will trigger the shutdown of
the provider, ensuring that all remaining logs are flushed and no further
logs are processed. Shutdown can also be triggered manually by calling
the [`shutdown`](SdkLoggerProvider::shutdown) method.

[`Logger`]: opentelemetry::logs::Logger
[`Resource`]: crate::Resource

---
