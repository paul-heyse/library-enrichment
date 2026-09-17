# `ast_grep_config::rule_config`

Crate `ast-grep-config` · 6 public items · structured records in [`model/ast_grep_config.rule_config.json`](../model/ast_grep_config.rule_config.json)

## RuleConfigError

`enum` · `ast_grep_config::rule_config::RuleConfigError`

Also reachable as `ast_grep_config::RuleConfigError`

```rust
enum RuleConfigError
```

**Variants**: `Yaml`, `Core`, `Fixer`, `Rewriter`, `UndefinedRewriter`, `LabelVariable`, `MissingPotentialKinds`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: RuleCoreError) -> Self
fn from(source: YamlError) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## RuleFileGlob

`enum` · `ast_grep_config::rule_config::RuleFileGlob`

```rust
enum RuleFileGlob
```

**Variants**: `Glob`, `Config`

---

## Severity

`enum` · `ast_grep_config::rule_config::Severity`

Also reachable as `ast_grep_config::Severity`

```rust
enum Severity
```

**Variants**: `Off`, `Hint`, `Info`, `Warning`, `Error`

**Implements**: `core::str::traits::FromStr`, `schemars::JsonSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
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

## Metadata

`struct` · `ast_grep_config::rule_config::Metadata`

Also reachable as `ast_grep_config::Metadata`

```rust
struct Metadata
```

**Implements**: `schemars::JsonSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone

**via `schemars::JsonSchema`**

```rust
fn json_schema(_gen: &mut SchemaGenerator) -> Schema
fn schema_id() -> Cow<'static, str>
fn schema_name() -> Cow<'static, str>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A trivial wrapper around a HashMap to work around
the limitation of `serde_yaml::Value` not implementing `JsonSchema`.

---

## RuleConfig

`struct` · `ast_grep_config::rule_config::RuleConfig`

Also reachable as `ast_grep_config::RuleConfig`

```rust
struct RuleConfig<L: Language>
```

**Fields**: `matcher`, `fixer`

**Implements**: `core::ops::deref::Deref`, `core::ops::deref::DerefMut`

**Methods** (5)

```rust
fn deserialize<'de>(deserializer: Deserializer<'de>, globals: &GlobalRules) -> Result<Self, RuleConfigError> where L: Deserialize<'de>
fn get_fixer(&self) -> Result<Vec<Fixer>, RuleConfigError>
fn get_labels<'t, D: Doc>(&self, node: &NodeMatch<'t, D>) -> Vec<Label<'_, 't, D>>
fn get_message<D>(&self, node: &NodeMatch<'_, D>) -> String where D: Doc
fn try_from(inner: SerializableRuleConfig<L>, globals: &GlobalRules) -> Result<Self, RuleConfigError>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

---

## SerializableRuleConfig

`struct` · `ast_grep_config::rule_config::SerializableRuleConfig`

Also reachable as `ast_grep_config::SerializableRuleConfig`

```rust
struct SerializableRuleConfig<L: Language>
```

**Fields**: `core`, `fix`, `rewriters`, `id`, `language`, `message`, `note`, `severity`, `labels`, `files`, `ignores`, `url`, `metadata`

**Implements**: `core::ops::deref::Deref`, `core::ops::deref::DerefMut`, `schemars::JsonSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone

**Methods** (1)

```rust
fn get_matcher(&self, globals: &GlobalRules) -> Result<RuleCore, RuleConfigError>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut Self::Target
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
