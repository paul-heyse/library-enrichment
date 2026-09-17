# `opentelemetry_sdk::logs::logger`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.logs.logger.json`](../model/opentelemetry_sdk.logs.logger.json)

## SdkLogger

`struct` · `opentelemetry_sdk::logs::logger::SdkLogger`

Also reachable as `opentelemetry_sdk::logs::SdkLogger`

```rust
struct SdkLogger
```

**Implements**: `opentelemetry::logs::logger::Logger`

**Derives**: Clone, Debug

**via `opentelemetry::logs::logger::Logger`**

```rust
fn create_log_record(&self) -> Self::LogRecord
fn emit(&self, record: Self::LogRecord)
fn event_enabled(&self, level: Severity, target: &str, name: Option<&str>) -> bool
```

The object for emitting [`LogRecord`]s.

[`LogRecord`]: opentelemetry::logs::LogRecord

---
