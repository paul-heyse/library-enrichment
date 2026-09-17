# `ast_grep_language`

Crate `ast-grep-language` · 31 public items · structured records in [`model/ast_grep_language.json`](../model/ast_grep_language.json)

## SupportLang

`enum` · `ast_grep_language::SupportLang`

```rust
enum SupportLang
```

**Variants**: `Bash`, `C`, `Cpp`, `CSharp`, `Css`, `Dart`, `Go`, `Elixir`, `Haskell`, `Hcl`, `Html`, `Java`, `JavaScript`, `Json`, `Kotlin`, `Lua`, `Markdown`, `Nix`, `Php`, `Python`, `Ruby`, `Rust`, `Scala`, `Solidity`, `Swift`, `Tsx`, `TypeScript`, `Yaml`

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn all_langs() -> &'static [SupportLang]
fn file_types(&self) -> Types
```

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn extract_meta_var(&self, source: &str) -> Option<MetaVariable>
fn field_to_id(&self, field: &str) -> Option<u16>
fn from_path<P: AsRef<Path>>(path: P) -> Option<Self>
fn kind_to_id(&self, kind: &str) -> u16
fn meta_var_char(&self) -> char
fn pre_process_pattern<'q>(&self, query: &'q str) -> Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn extract_injections<L: LanguageExt>(&self, root: Node<'_, StrDoc<L>>) -> Vec<(String, Vec<TSRange>)>
fn get_ts_language(&self) -> TSLanguage
fn injectable_languages(&self) -> Option<&'static [&'static str]>
```

**via `core::convert::From`**

```rust
fn from(_: Python) -> Self
fn from(_: Bash) -> Self
fn from(_: Css) -> Self
fn from(_: Yaml) -> Self
fn from(_: Ruby) -> Self
fn from(_: Elixir) -> Self
fn from(_: Tsx) -> Self
fn from(_: Markdown) -> Self
fn from(_: Rust) -> Self
fn from(_: Go) -> Self
fn from(_: JavaScript) -> Self
fn from(_: Swift) -> Self
fn from(_: Haskell) -> Self
fn from(_: Html) -> Self
fn from(_: Dart) -> Self
fn from(_: Solidity) -> Self
fn from(_: Hcl) -> Self
fn from(_: Lua) -> Self
fn from(_: Kotlin) -> Self
fn from(_: C) -> Self
fn from(_: Java) -> Self
fn from(_: Nix) -> Self
fn from(_: Cpp) -> Self
fn from(_: TypeScript) -> Self
fn from(_: Scala) -> Self
fn from(_: Php) -> Self
fn from(_: CSharp) -> Self
fn from(_: Json) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents all built-in languages.

---

## SupportLangErr

`enum` · `ast_grep_language::SupportLangErr`

```rust
enum SupportLangErr
```

**Variants**: `LanguageNotSupported`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>
```

---

## config_file_type

`function` · `ast_grep_language::config_file_type`

```rust
fn config_file_type() -> ignore::types::Types
```

---

## Bash

`struct` · `ast_grep_language::Bash`

```rust
struct Bash
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## C

`struct` · `ast_grep_language::C`

```rust
struct C
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## CSharp

`struct` · `ast_grep_language::CSharp`

```rust
struct CSharp
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Cpp

`struct` · `ast_grep_language::Cpp`

```rust
struct Cpp
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Css

`struct` · `ast_grep_language::Css`

```rust
struct Css
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Dart

`struct` · `ast_grep_language::Dart`

```rust
struct Dart
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Elixir

`struct` · `ast_grep_language::Elixir`

```rust
struct Elixir
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Go

`struct` · `ast_grep_language::Go`

```rust
struct Go
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Haskell

`struct` · `ast_grep_language::Haskell`

```rust
struct Haskell
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Hcl

`struct` · `ast_grep_language::Hcl`

```rust
struct Hcl
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Java

`struct` · `ast_grep_language::Java`

```rust
struct Java
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## JavaScript

`struct` · `ast_grep_language::JavaScript`

```rust
struct JavaScript
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Json

`struct` · `ast_grep_language::Json`

```rust
struct Json
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Kotlin

`struct` · `ast_grep_language::Kotlin`

```rust
struct Kotlin
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Lua

`struct` · `ast_grep_language::Lua`

```rust
struct Lua
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Markdown

`struct` · `ast_grep_language::Markdown`

```rust
struct Markdown
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Nix

`struct` · `ast_grep_language::Nix`

```rust
struct Nix
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Php

`struct` · `ast_grep_language::Php`

```rust
struct Php
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Python

`struct` · `ast_grep_language::Python`

```rust
struct Python
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Ruby

`struct` · `ast_grep_language::Ruby`

```rust
struct Ruby
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Rust

`struct` · `ast_grep_language::Rust`

```rust
struct Rust
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Scala

`struct` · `ast_grep_language::Scala`

```rust
struct Scala
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Solidity

`struct` · `ast_grep_language::Solidity`

```rust
struct Solidity
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Swift

`struct` · `ast_grep_language::Swift`

```rust
struct Swift
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
fn pre_process_pattern<'q>(&self, query: &'q str) -> std::borrow::Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Tsx

`struct` · `ast_grep_language::Tsx`

```rust
struct Tsx
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## TypeScript

`struct` · `ast_grep_language::TypeScript`

```rust
struct TypeScript
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Yaml

`struct` · `ast_grep_language::Yaml`

```rust
struct Yaml
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `ast_grep_language::Alias`, `core::fmt::Display`, `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn field_to_id(&self, field: &str) -> Option<u16>
fn kind_to_id(&self, kind: &str) -> u16
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

---

## Alias

`trait` · `ast_grep_language::Alias`

```rust
trait Alias: Display
```

**Implementors** (28)

- `ast_grep_language::Bash`
- `ast_grep_language::C`
- `ast_grep_language::CSharp`
- `ast_grep_language::Cpp`
- `ast_grep_language::Css`
- `ast_grep_language::Dart`
- `ast_grep_language::Elixir`
- `ast_grep_language::Go`
- `ast_grep_language::Haskell`
- `ast_grep_language::Hcl`
- `ast_grep_language::Java`
- `ast_grep_language::JavaScript`
- `ast_grep_language::Json`
- `ast_grep_language::Kotlin`
- `ast_grep_language::Lua`
- `ast_grep_language::Markdown`
- `ast_grep_language::Nix`
- `ast_grep_language::Php`
- `ast_grep_language::Python`
- `ast_grep_language::Ruby`
- `ast_grep_language::Rust`
- `ast_grep_language::Scala`
- `ast_grep_language::Solidity`
- `ast_grep_language::Swift`
- `ast_grep_language::Tsx`
- `ast_grep_language::TypeScript`
- `ast_grep_language::Yaml`
- `ast_grep_language::html::Html`

---
