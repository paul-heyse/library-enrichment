# `opentelemetry::common`

Crate `opentelemetry` · 7 public items · structured records in [`model/opentelemetry.common.json`](../model/opentelemetry.common.json)

## Array

`enum` · `opentelemetry::common::Array`

Also reachable as `opentelemetry::Array`

```rust
enum Array
```

**Variants**: `Bool`, `I64`, `F64`, `String`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(t: Vec<i64>) -> Self
fn from(t: Vec<bool>) -> Self
fn from(t: Vec<StringValue>) -> Self
fn from(t: Vec<f64>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result
```

A [Value::Array] containing homogeneous values.

---

## Value

`enum` · `opentelemetry::common::Value`

Also reachable as `opentelemetry::Value`

```rust
enum Value
```

**Variants**: `Bool`, `I64`, `F64`, `String`, `Array`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn as_str(&self) -> Cow<'_, str>
```

**via `core::convert::From`**

```rust
fn from(s: &'static str) -> Self
fn from(t: bool) -> Self
fn from(s: Cow<'static, str>) -> Self
fn from(t: StringValue) -> Self
fn from(s: Arc<str>) -> Self
fn from(t: f64) -> Self
fn from(s: String) -> Self
fn from(t: i64) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result
```

The value part of attribute [KeyValue] pairs.

---

## InstrumentationScope

`struct` · `opentelemetry::common::InstrumentationScope`

Also reachable as `opentelemetry::InstrumentationScope`

```rust
struct InstrumentationScope
```

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn attributes(&self) -> impl Iterator<Item = &KeyValue>
fn builder<T: Into<Cow<'static, str>>>(name: T) -> InstrumentationScopeBuilder
fn name(&self) -> &str
fn schema_url(&self) -> Option<&str>
fn version(&self) -> Option<&str>
```

Information about a library or crate providing instrumentation.

An instrumentation scope should be named to follow any naming conventions
of the instrumented library (e.g. 'middleware' for a web framework).

See the [instrumentation libraries] spec for more information.

[instrumentation libraries]: https://github.com/open-telemetry/opentelemetry-specification/blob/v1.9.0/specification/overview.md#instrumentation-libraries

---

## InstrumentationScopeBuilder

`struct` · `opentelemetry::common::InstrumentationScopeBuilder`

Also reachable as `opentelemetry::InstrumentationScopeBuilder`

```rust
struct InstrumentationScopeBuilder
```

**Derives**: Debug

**Methods** (4)

```rust
fn build(self) -> InstrumentationScope
fn with_attributes<I>(self, attributes: I) -> Self where I: IntoIterator<Item = KeyValue>
fn with_schema_url(self, schema_url: impl Into<Cow<'static, str>>) -> Self
fn with_version(self, version: impl Into<Cow<'static, str>>) -> Self
```

Configuration options for [InstrumentationScope].

An instrumentation scope is a library or crate providing instrumentation.
It should be named to follow any naming conventions of the instrumented
library (e.g. 'middleware' for a web framework).

Apart from the name, all other fields are optional.

See the [instrumentation libraries] spec for more information.

[instrumentation libraries]: https://github.com/open-telemetry/opentelemetry-specification/blob/v1.9.0/specification/overview.md#instrumentation-libraries

---

## Key

`struct` · `opentelemetry::common::Key`

Also reachable as `opentelemetry::Key`

```rust
struct Key
```

**Implements**: `core::borrow::Borrow`, `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn as_str(&self) -> &str
const fn from_static_str(value: &'static str) -> Self
fn new(value: impl Into<Key>) -> Self
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &str
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::convert::From`**

```rust
fn from(string: Arc<str>) -> Self
fn from(string: String) -> Self
fn from(key_str: &'static str) -> Self
fn from(string: Cow<'static, str>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result
```

The key part of attribute [KeyValue] pairs.

See the [attribute naming] spec for guidelines.

[attribute naming]: https://github.com/open-telemetry/semantic-conventions/blob/main/docs/general/attribute-naming.md

---

## KeyValue

`struct` · `opentelemetry::common::KeyValue`

Also reachable as `opentelemetry::KeyValue`

```rust
struct KeyValue
```

**Fields**: `key`, `value`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new<K, V>(key: K, value: V) -> Self where K: Into<Key>, V: Into<Value>
```

A key-value pair describing an attribute.

---

## StringValue

`struct` · `opentelemetry::common::StringValue`

Also reachable as `opentelemetry::StringValue`

```rust
struct StringValue
```

**Implements**: `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn as_str(&self) -> &str
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::convert::From`**

```rust
fn from(s: Cow<'static, str>) -> Self
fn from(s: Arc<str>) -> Self
fn from(s: Value) -> Self
fn from(s: String) -> Self
fn from(s: &'static str) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Wrapper for string-like values

---
