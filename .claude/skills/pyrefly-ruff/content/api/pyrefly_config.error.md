# `pyrefly_config::error`

Crate `pyrefly_config` · 2 public items · structured records in [`model/pyrefly_config.error.json`](../model/pyrefly_config.error.json)

## ErrorConfig

`struct` · `pyrefly_config::error::ErrorConfig`

```rust
struct ErrorConfig<'a>
```

**Fields**: `display_config`, `ignore_errors_in_generated_code`, `enabled_ignores`, `type_ignore_unknown_tag_behavior`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(display_config: Cow<'a, ErrorDisplayConfig>, ignore_errors_in_generated_code: bool, enabled_ignores: SmallSet<Tool>, type_ignore_unknown_tag_behavior: TypeIgnoreUnknownTagBehavior) -> Self
```

---

## ErrorDisplayConfig

`struct` · `pyrefly_config::error::ErrorDisplayConfig`

```rust
struct ErrorDisplayConfig
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn iter(&self) -> impl Iterator<Item = (ErrorKind, Severity)> + '_
fn merge_user_overrides(&mut self, user: &ErrorDisplayConfig)
fn new(config: HashMap<ErrorKind, Severity>) -> Self
fn set_default_severity(&mut self, kind: ErrorKind, severity: Severity)
fn set_error_severity(&mut self, kind: ErrorKind, severity: Severity)
fn severity(&self, kind: ErrorKind) -> Severity
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents overrides for errors to emit when collecting/printing errors.
Not all error kinds are required to be defined in this map. Any that are missing
will use the default severity associated with that error kind.

---
