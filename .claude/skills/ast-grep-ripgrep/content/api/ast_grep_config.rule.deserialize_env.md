# `ast_grep_config::rule::deserialize_env`

Crate `ast-grep-config` · 2 public items · structured records in [`model/ast_grep_config.rule.deserialize_env.json`](../model/ast_grep_config.rule.deserialize_env.json)

## DeserializeEnv

`struct` · `ast_grep_config::rule::deserialize_env::DeserializeEnv`

Also reachable as `ast_grep_config::DeserializeEnv`

```rust
struct DeserializeEnv<L: Language>
```

**Derives**: Clone

**Methods** (5)

```rust
fn deserialize_rule(&self, serialized: SerializableRule) -> Result<Rule, RuleSerializeError>
fn new(lang: L) -> Self
fn parse_global_utils(utils: Vec<SerializableGlobalRule<L>>) -> Result<GlobalRules, RuleCoreError>
fn with_globals(self, globals: &GlobalRules) -> Self
fn with_utils(self, utils: &HashMap<String, SerializableRule>) -> Result<Self, RuleSerializeError>
```

A struct to store information to deserialize rules.

---

## SerializableGlobalRule

`struct` · `ast_grep_config::rule::deserialize_env::SerializableGlobalRule`

Also reachable as `ast_grep_config::SerializableGlobalRule`

```rust
struct SerializableGlobalRule<L: Language>
```

**Fields**: `core`, `id`, `arguments`, `language`

**Implements**: `schemars::JsonSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone

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
