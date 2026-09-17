# `ast_grep_config::transform`

Crate `ast-grep-config` · 3 public items · structured records in [`model/ast_grep_config.transform.json`](../model/ast_grep_config.transform.json)

## TransformError

`enum` · `ast_grep_config::transform::TransformError`

```rust
enum TransformError
```

**Variants**: `Parse`, `Cyclic`, `AlreadyDefined`, `MalformedVar`

---

## Transformation

`enum` · `ast_grep_config::transform::Transformation`

Also reachable as `ast_grep_config::Transformation`

```rust
enum Transformation
```

**Variants**: `Simplied`, `Object`

**Implements**: `schemars::JsonSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone

**Methods** (1)

```rust
fn parse<L: Language>(&self, lang: &L) -> Result<Trans<MetaVariable>, TransformError>
```

**via `schemars::JsonSchema`**

```rust
fn inline_schema() -> bool
fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema
fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str>
fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Transform

`struct` · `ast_grep_config::transform::Transform`

```rust
struct Transform
```

---
