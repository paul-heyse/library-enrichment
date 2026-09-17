# `opentelemetry_sdk::resource::telemetry`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.resource.telemetry.json`](../model/opentelemetry_sdk.resource.telemetry.json)

## TelemetryResourceDetector

`struct` · `opentelemetry_sdk::resource::telemetry::TelemetryResourceDetector`

Also reachable as `opentelemetry_sdk::resource::TelemetryResourceDetector`

```rust
struct TelemetryResourceDetector
```

**Implements**: `opentelemetry_sdk::resource::ResourceDetector`

**Derives**: Debug

**via `opentelemetry_sdk::resource::ResourceDetector`**

```rust
fn detect(&self) -> Resource
```

Detect the telemetry SDK information used to capture data recorded by the instrumentation libraries.

It provides:
- The name of the telemetry SDK(`telemetry.sdk.name`). It will be `opentelemetry` for SDK provided by opentelemetry project.
- The language of the telemetry SDK(`telemetry.sdk.language`). It will be `rust` for this SDK.
- The version of the telemetry SDK(`telemetry.sdk.version`). It will be current `opentelemetry_sdk` crate version.


See [semantic conventions](https://github.com/open-telemetry/semantic-conventions/blob/main/docs/resource/README.md#telemetry-sdk) for details.

---
