# `ast_grep_config::rewriter`

Crate `ast-grep-config` · 4 public items · structured records in [`model/ast_grep_config.rewriter.json`](../model/ast_grep_config.rewriter.json)

## RewriterErrorReason

`enum` · `ast_grep_config::rewriter::RewriterErrorReason`

```rust
enum RewriterErrorReason
```

**Variants**: `Core`, `NoFixInRewriter`, `Fixer`

---

## Rewriter

`struct` · `ast_grep_config::rewriter::Rewriter`

```rust
struct Rewriter
```

**Fields**: `matcher`, `fixer`

---

## RewriterError

`struct` · `ast_grep_config::rewriter::RewriterError`

```rust
struct RewriterError
```

**Fields**: `id`, `reason`

---

## SerializableRewriter

`struct` · `ast_grep_config::rewriter::SerializableRewriter`

Also reachable as `ast_grep_config::SerializableRewriter`

```rust
struct SerializableRewriter
```

**Fields**: `id`, `fix`, `core`

**Implements**: `schemars::JsonSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone

**Methods** (1)

```rust
fn try_parse_rewriter<L: Language>(&self, upper_vars: &HashSet<&str>, env: &DeserializeEnv<L>) -> Result<Rewriter, RewriterError>
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
