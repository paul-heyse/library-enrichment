# `ast_grep_outline::extractor`

Crate `ast-grep-outline` · 10 public items · structured records in [`model/ast_grep_outline.extractor.json`](../model/ast_grep_outline.extractor.json)

## OutlineRuleError

`enum` · `ast_grep_outline::extractor::OutlineRuleError`

```rust
enum OutlineRuleError
```

**Variants**: `RuleConfig`, `Predicate`, `Template`, `UnknownParentRuleId`, `InvalidParentRuleRole`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: TemplateFixError) -> Self
fn from(source: RuleSerializeError) -> Self
fn from(source: RuleConfigError) -> Self
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

## SerializableOutlineRule

`enum` · `ast_grep_outline::extractor::SerializableOutlineRule`

```rust
enum SerializableOutlineRule<L>
```

**Variants**: `Item`, `Member`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone

**Methods** (1)

```rust
fn common(&self) -> &SerializableOutlineCommon<L>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Serializable outline extractor definition loaded from an outline rule YAML document.

The `role` field selects the concrete rule shape. Item rules create top-level
entries. Member rules create direct child entries that can attach to eligible
item rules through `parentRuleIds`.

---

## SerializablePredicate

`enum` · `ast_grep_outline::extractor::SerializablePredicate`

```rust
enum SerializablePredicate
```

**Variants**: `Literal`, `Rule`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Boolean derivation for outline flags.

A literal boolean sets the output flag directly. A rule object is evaluated
against the matched candidate node and sets the output flag from the match result.

---

## parse_outline_rules

`function` · `ast_grep_outline::extractor::parse_outline_rules`

```rust
fn parse_outline_rules<'a, L>(src: &'a str) -> Result<Vec<SerializableOutlineRule<L>>, serde_yaml::Error> where L: Deserialize<'a>
```

Parse a stream of YAML outline extractor documents.

---

## ExtractorCommon

`struct` · `ast_grep_outline::extractor::ExtractorCommon`

```rust
struct ExtractorCommon<L: Language>
```

**Fields**: `rule`, `symbol_type`, `name`, `signature`

**Methods** (1)

```rust
fn try_from(common: SerializableOutlineCommon<L>, globals: &GlobalRules, detail: OutlineEntryDetail) -> Result<Self, OutlineRuleError>
```

Shared parsed fields for every runnable outline extractor.

---

## ItemExtractor

`struct` · `ast_grep_outline::extractor::ItemExtractor`

```rust
struct ItemExtractor<L: Language>
```

**Fields**: `common`

**Methods** (3)

```rust
fn extract<'tree, D: Doc>(&self, node_match: &NodeMatch<'tree, D>, members: Vec<OutlineMember<'tree>>) -> OutlineItem<'tree>
fn match_node<'tree, D: Doc>(&self, node: &Node<'tree, D>) -> Option<NodeMatch<'tree, D>>
fn try_from(item: SerializableItemRule<L>, globals: &GlobalRules, detail: OutlineEntryDetail) -> Result<Self, OutlineRuleError>
```

Runnable item extractor for top-level file/module structure.

---

## MemberExtractor

`struct` · `ast_grep_outline::extractor::MemberExtractor`

```rust
struct MemberExtractor<L: Language>
```

**Fields**: `common`, `parent_rule_ids`

**Methods** (3)

```rust
fn extract<'tree, D: Doc>(&self, node_match: &NodeMatch<'tree, D>) -> OutlineMember<'tree>
fn match_node<'tree, D: Doc>(&self, node: &Node<'tree, D>) -> Option<NodeMatch<'tree, D>>
fn try_from(member: SerializableMemberRule<L>, globals: &GlobalRules, detail: OutlineEntryDetail) -> Result<Self, OutlineRuleError>
```

Runnable member extractor for direct child structure under an item.

---

## SerializableItemRule

`struct` · `ast_grep_outline::extractor::SerializableItemRule`

```rust
struct SerializableItemRule<L>
```

**Fields**: `common`, `is_import`, `is_exported`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Item extractor for top-level file/module structure.

---

## SerializableMemberRule

`struct` · `ast_grep_outline::extractor::SerializableMemberRule`

```rust
struct SerializableMemberRule<L>
```

**Fields**: `common`, `parent_rule_ids`, `is_public`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Member extractor for direct child structure under an item.

---

## SerializableOutlineCommon

`struct` · `ast_grep_outline::extractor::SerializableOutlineCommon`

```rust
struct SerializableOutlineCommon<L>
```

**Fields**: `id`, `language`, `symbol_type`, `matcher`, `rewriters`, `name`, `signature`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Shared serializable fields for every outline extractor.

---
