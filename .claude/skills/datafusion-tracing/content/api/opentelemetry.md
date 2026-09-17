# `opentelemetry`

Crate `opentelemetry` · 4 public items · structured records in [`model/opentelemetry.json`](../model/opentelemetry.json)

## otel_debug

`macro` · `opentelemetry::otel_debug`

```rust
macro_rules! otel_debug
```

Macro for logging debug messages in OpenTelemetry.

# Fields:
- `name`: The operation or action being logged.
- Additional optional key-value pairs can be passed as attributes.

# Example:
```rust
use opentelemetry::otel_debug;
otel_debug!(name: "debug_operation", debug_level = "high", version = "1.0.0");
```

---

## otel_error

`macro` · `opentelemetry::otel_error`

```rust
macro_rules! otel_error
```

Macro for logging error messages in OpenTelemetry.

# Fields:
- `name`: The operation or action being logged.
- Additional optional key-value pairs can be passed as attributes.

# Example:
```rust
use opentelemetry::otel_error;
otel_error!(name: "export_failure", error_code = 500, version = "1.0.0");
```

---

## otel_info

`macro` · `opentelemetry::otel_info`

```rust
macro_rules! otel_info
```


**Note**: These macros (`otel_info!`, `otel_warn!`, `otel_debug!`, and `otel_error!`) are intended to be used
**internally within OpenTelemetry code** or for **custom exporters, processors and other plugins**. They are not designed
for general application logging and should not be used for that purpose.

When running tests with `--nocapture`, these macros will print their output to stdout. This is useful for debugging
test failures and understanding the flow of operations during testing.

Macro for logging informational messages in OpenTelemetry.

# Fields:
- `name`: The operation or action being logged.
- Additional optional key-value pairs can be passed as attributes.

# Example:
```rust
use opentelemetry::otel_info;
otel_info!(name: "sdk_start", version = "1.0.0", schema_url = "http://example.com");
```

---

## otel_warn

`macro` · `opentelemetry::otel_warn`

```rust
macro_rules! otel_warn
```

Macro for logging warning messages in OpenTelemetry.

# Fields:
- `name`: The operation or action being logged.
- Additional optional key-value pairs can be passed as attributes.

# Example:
```rust
use opentelemetry::otel_warn;
otel_warn!(name: "export_warning", error_code = 404, version = "1.0.0");
```

---
