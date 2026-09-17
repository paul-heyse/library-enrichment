# `cargo_metadata::libtest`

Crate `cargo_metadata` · 3 public items · structured records in [`model/cargo_metadata.libtest.json`](../model/cargo_metadata.libtest.json)

## SuiteEvent

`enum` · `cargo_metadata::libtest::SuiteEvent`

```rust
enum SuiteEvent
```

**Variants**: `Started`, `Ok`, `Failed`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Suite related event
Suite event

---

## TestEvent

`enum` · `cargo_metadata::libtest::TestEvent`

```rust
enum TestEvent
```

**Variants**: `Started`, `Ok`, `Failed`, `Ignored`, `Timeout`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn name(&self) -> &str
fn stdout(&self) -> Option<&str>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Test event

---

## TestMessage

`enum` · `cargo_metadata::libtest::TestMessage`

Also reachable as `cargo_metadata::TestMessage`

```rust
enum TestMessage
```

**Variants**: `Suite`, `Test`, `Bench`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents the output of `cargo test -- -Zunstable-options --report-time --show-output --format json`.

requires --report-time

# Stability

As this struct is for interfacing with the unstable libtest json output, this struct may change at any time, without semver guarantees.

---
