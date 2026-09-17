# `opentelemetry_sdk::resource`

Crate `opentelemetry_sdk` · 4 public items · structured records in [`model/opentelemetry_sdk.resource.json`](../model/opentelemetry_sdk.resource.json)

## Iter

`struct` · `opentelemetry_sdk::resource::Iter`

```rust
struct Iter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

An iterator over the entries of a `Resource`.

---

## Resource

`struct` · `opentelemetry_sdk::resource::Resource`

Also reachable as `opentelemetry_sdk::Resource`

```rust
struct Resource
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn builder() -> ResourceBuilder
fn builder_empty() -> ResourceBuilder
fn get(&self, key: &Key) -> Option<Value>
fn is_empty(&self) -> bool
fn iter(&self) -> Iter<'_>
fn len(&self) -> usize
fn schema_url(&self) -> Option<&str>
```

An immutable representation of the entity producing telemetry as attributes.
Utilizes `Arc` for efficient sharing and cloning.

---

## ResourceBuilder

`struct` · `opentelemetry_sdk::resource::ResourceBuilder`

```rust
struct ResourceBuilder
```

**Derives**: Debug

**Methods** (7)

```rust
fn build(self) -> Resource
fn with_attribute(self, kv: KeyValue) -> Self
fn with_attributes<T: IntoIterator<Item = KeyValue>>(self, kvs: T) -> Self
fn with_detector(self, detector: Box<dyn ResourceDetector>) -> Self
fn with_detectors(self, detectors: &[Box<dyn ResourceDetector>]) -> Self
fn with_schema_url<KV, S>(self, attributes: KV, schema_url: S) -> Self where KV: IntoIterator<Item = KeyValue>, S: Into<Cow<'static, str>>
fn with_service_name(self, name: impl Into<Value>) -> Self
```

Builder for [Resource]

---

## ResourceDetector

`trait` · `opentelemetry_sdk::resource::ResourceDetector`

```rust
trait ResourceDetector
```

**Implementors** (3)

- `opentelemetry_sdk::resource::env::EnvResourceDetector`
- `opentelemetry_sdk::resource::env::SdkProvidedResourceDetector`
- `opentelemetry_sdk::resource::telemetry::TelemetryResourceDetector`

**Methods** (1)

```rust
fn detect(&self) -> Resource
```

ResourceDetector detects OpenTelemetry resource information

Implementations of this trait can be passed to
the [`ResourceBuilder::with_detectors`] function to generate a Resource from the merged information.

---
