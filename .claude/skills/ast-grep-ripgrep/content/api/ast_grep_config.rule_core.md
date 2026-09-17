# `ast_grep_config::rule_core`

Crate `ast-grep-config` · 3 public items · structured records in [`model/ast_grep_config.rule_core.json`](../model/ast_grep_config.rule_core.json)

## RuleCoreError

`enum` · `ast_grep_config::rule_core::RuleCoreError`

Also reachable as `ast_grep_config::RuleCoreError`

```rust
enum RuleCoreError
```

**Variants**: `Yaml`, `Utils`, `Rule`, `Constraints`, `Transform`, `UndefinedMetaVar`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: YamlError) -> Self
fn from(source: RuleSerializeError) -> Self
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

## RuleCore

`struct` · `ast_grep_config::rule_core::RuleCore`

Also reachable as `ast_grep_config::RuleCore`

```rust
struct RuleCore
```

**Implements**: `ast_grep_core::matcher::Matcher`, `core::ops::deref::Deref`

**Derives**: Default

**Methods** (6)

```rust
fn defined_vars(&self) -> HashSet<&str>
fn get_env<L: Language>(&self, lang: L) -> DeserializeEnv<L>
fn new(rule: Rule) -> Self
fn with_matchers(self, constraints: HashMap<String, Rule>) -> Self
fn with_registration(self, registration: RuleRegistration) -> Self
fn with_transform(self, transform: Option<Transform>) -> Self
```

**via `ast_grep_core::matcher::Matcher`**

```rust
fn match_node_with_env<'tree, D: Doc>(&self, node: Node<'tree, D>, env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
fn potential_kinds(&self) -> Option<BitSet>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

---

## SerializableRuleCore

`struct` · `ast_grep_config::rule_core::SerializableRuleCore`

Also reachable as `ast_grep_config::SerializableRuleCore`

```rust
struct SerializableRuleCore
```

**Fields**: `rule`, `constraints`, `utils`, `transform`

**Implements**: `schemars::JsonSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone

**Methods** (1)

```rust
fn get_matcher<L: Language>(&self, env: DeserializeEnv<L>) -> std::result::Result<RuleCore, RuleCoreError>
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

Used for global rules, rewriters, and pyo3/napi

---
