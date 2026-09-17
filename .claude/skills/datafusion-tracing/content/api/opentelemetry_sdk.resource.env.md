# `opentelemetry_sdk::resource::env`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.resource.env.json`](../model/opentelemetry_sdk.resource.env.json)

## EnvResourceDetector

`struct` · `opentelemetry_sdk::resource::env::EnvResourceDetector`

Also reachable as `opentelemetry_sdk::resource::EnvResourceDetector`

```rust
struct EnvResourceDetector
```

**Implements**: `opentelemetry_sdk::resource::ResourceDetector`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `opentelemetry_sdk::resource::ResourceDetector`**

```rust
fn detect(&self) -> Resource
```

EnvResourceDetector extract resource from environment variable
`OTEL_RESOURCE_ATTRIBUTES`. See [OpenTelemetry Resource
Spec](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/resource/sdk.md#specifying-resource-information-via-an-environment-variable)
for details.

---

## SdkProvidedResourceDetector

`struct` · `opentelemetry_sdk::resource::env::SdkProvidedResourceDetector`

Also reachable as `opentelemetry_sdk::resource::SdkProvidedResourceDetector`

```rust
struct SdkProvidedResourceDetector
```

**Implements**: `opentelemetry_sdk::resource::ResourceDetector`

**Derives**: Debug

**via `opentelemetry_sdk::resource::ResourceDetector`**

```rust
fn detect(&self) -> Resource
```

There are attributes which MUST be provided by the SDK as specified in
[the Resource SDK specification]. This detector detects those attributes and
if the attribute cannot be detected, it uses the default value.

This detector will first try `OTEL_SERVICE_NAME` env. If it's not available,
then it will check the `OTEL_RESOURCE_ATTRIBUTES` env and see if it contains
`service.name` resource. If it's also not available, it will use `unknown_service`.

If users want to set an empty service name, they can provide
a resource with empty value and `service.name` key.

[the Resource SDK specification]:https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/resource/sdk.md#sdk-provided-resource-attributes

---
