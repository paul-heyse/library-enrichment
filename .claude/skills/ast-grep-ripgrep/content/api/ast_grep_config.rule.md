# `ast_grep_config::rule`

Crate `ast-grep-config` · 6 public items · structured records in [`model/ast_grep_config.rule.json`](../model/ast_grep_config.rule.json)

## PatternStyle

`enum` · `ast_grep_config::rule::PatternStyle`

```rust
enum PatternStyle
```

**Variants**: `Str`, `Contextual`

A String pattern will match one single AST node according to pattern syntax.
Or an object with field `context`, `selector` and optionally `strictness`.

---

## Rule

`enum` · `ast_grep_config::rule::Rule`

Also reachable as `ast_grep_config::Rule`

```rust
enum Rule
```

**Variants**: `Pattern`, `Kind`, `Regex`, `NthChild`, `Range`, `Inside`, `Has`, `Precedes`, `Follows`, `All`, `Any`, `Not`, `Matches`

**Implements**: `ast_grep_core::matcher::Matcher`

**Derives**: Default

**Methods** (2)

```rust
fn defined_vars(&self) -> HashSet<&str>
fn verify_util(&self) -> Result<(), RuleSerializeError>
```

**via `ast_grep_core::matcher::Matcher`**

```rust
fn match_node_with_env<'tree, D: Doc>(&self, node: Node<'tree, D>, env: &mut Cow<'_, MetaVarEnv<'tree, D>>) -> Option<Node<'tree, D>>
fn potential_kinds(&self) -> Option<BitSet>
```

---

## RuleSerializeError

`enum` · `ast_grep_config::rule::RuleSerializeError`

Also reachable as `ast_grep_config::RuleSerializeError`

```rust
enum RuleSerializeError
```

**Variants**: `MissPositiveMatcher`, `InvalidKind`, `InvalidPattern`, `NthChild`, `WrongRegex`, `MatchesReference`, `InvalidUtils`, `InvalidRange`, `FieldNotSupported`, `InvalidField`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: ParameterizedUtilError) -> Self
fn from(source: RegexMatcherError) -> Self
fn from(source: SelectorError) -> Self
fn from(source: PatternError) -> Self
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

## SerializableMatches

`enum` · `ast_grep_config::rule::SerializableMatches`

```rust
enum SerializableMatches
```

**Variants**: `Id`, `Call`

---

## Strictness

`enum` · `ast_grep_config::rule::Strictness`

```rust
enum Strictness
```

**Variants**: `Cst`, `Smart`, `Ast`, `Relaxed`, `Signature`, `Template`

---

## SerializableRule

`struct` · `ast_grep_config::rule::SerializableRule`

Also reachable as `ast_grep_config::SerializableRule`

```rust
struct SerializableRule
```

**Fields**: `pattern`, `kind`, `regex`, `nth_child`, `range`, `inside`, `has`, `precedes`, `follows`, `all`, `any`, `not`, `matches`

**Implements**: `schemars::JsonSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Default

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

A rule object to find matching AST nodes. We have three categories of rules in ast-grep.

* Atomic: the most basic rule to match AST. We have two variants: Pattern and Kind.

* Relational: filter matched target according to their position relative to other nodes.

* Composite: use logic operation all/any/not to compose the above rules to larger rules.

Every rule has it's unique name so we can combine several rules in one object.

---
