# `ast_grep_config::label`

Crate `ast-grep-config` · 3 public items · structured records in [`model/ast_grep_config.label.json`](../model/ast_grep_config.label.json)

## LabelStyle

`enum` · `ast_grep_config::label::LabelStyle`

Also reachable as `ast_grep_config::LabelStyle`

```rust
enum LabelStyle
```

**Variants**: `Primary`, `Secondary`

**Implements**: `schemars::JsonSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

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

## Label

`struct` · `ast_grep_config::label::Label`

Also reachable as `ast_grep_config::Label`

```rust
struct Label<'r, 't, D: Doc>
```

**Fields**: `style`, `message`, `start_node`, `end_node`

**Methods** (1)

```rust
fn range(&self) -> Range<usize>
```

A label is a way to mark a specific part of the code with a styled message.
It is used to provide diagnostic information in LSP or CLI.
'r represents a lifetime for the message string from `rule`.
't represents a lifetime for the node from a ast `tree`.

---

## LabelConfig

`struct` · `ast_grep_config::label::LabelConfig`

```rust
struct LabelConfig
```

**Fields**: `style`, `message`

---
