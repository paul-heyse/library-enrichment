# `pyrefly::playground`

Crate `pyrefly` · 8 public items · structured records in [`model/pyrefly.playground.json`](../model/pyrefly.playground.json)

## AutoCompletionItem

`struct` · `pyrefly::playground::AutoCompletionItem`

```rust
struct AutoCompletionItem
```

**Implements**: `serde_core::ser::Serialize`

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## CompletionTextEdit

`struct` · `pyrefly::playground::CompletionTextEdit`

```rust
struct CompletionTextEdit
```

**Implements**: `serde_core::ser::Serialize`

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A Monaco `ISingleEditOperation`.

---

## Diagnostic

`struct` · `pyrefly::playground::Diagnostic`

```rust
struct Diagnostic
```

**Fields**: `start_line`, `start_col`, `end_line`, `end_col`, `message_header`, `message_details`, `kind`, `severity`, `filename`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## InlayHint

`struct` · `pyrefly::playground::InlayHint`

```rust
struct InlayHint
```

**Implements**: `serde_core::ser::Serialize`

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## MonacoHover

`struct` · `pyrefly::playground::MonacoHover`

```rust
struct MonacoHover
```

**Implements**: `serde_core::ser::Serialize`

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Playground

`struct` · `pyrefly::playground::Playground`

```rust
struct Playground
```

**Methods** (11)

```rust
fn autocomplete(&self, pos: Position) -> Vec<AutoCompletionItem>
fn get_errors(&self) -> Vec<Diagnostic>
fn goto_definition(&mut self, pos: Position) -> Vec<Range>
fn hover(&self, pos: Position) -> Option<MonacoHover>
fn inlay_hint(&self, call_argument_names: bool) -> Vec<InlayHint>
fn new(python_version: Option<&str>) -> Result<Self, String>
fn semantic_tokens(&self, range: Option<Range>) -> Option<SemanticTokensResult>
fn semantic_tokens_legend(&self) -> SemanticTokensLegend
fn set_active_file(&mut self, filename: &str)
fn update_sandbox_files(&mut self, files: SmallMap<String, String>, force_update: bool) -> Option<String>
fn update_single_file(&mut self, filename: String, content: String)
```

---

## Position

`struct` · `pyrefly::playground::Position`

```rust
struct Position
```

**Fields**: `column`, `line`

**Implements**: `serde_core::ser::Serialize`

**Methods** (1)

```rust
fn new(line: i32, column: i32) -> Self
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Range

`struct` · `pyrefly::playground::Range`

```rust
struct Range
```

**Fields**: `start_line`, `start_col`, `end_line`, `end_col`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---
