# `ast_grep_dynamic`

Crate `ast-grep-dynamic` · 3 public items · structured records in [`model/ast_grep_dynamic.json`](../model/ast_grep_dynamic.json)

## DynamicLangError

`enum` · `ast_grep_dynamic::DynamicLangError`

```rust
enum DynamicLangError
```

**Variants**: `NotConfigured`, `OpenLib`, `ReadSymbol`, `IncompatibleVersion`, `GetLibPath`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: std::io::Error) -> Self
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

## DynamicLang

`struct` · `ast_grep_dynamic::DynamicLang`

```rust
struct DynamicLang
```

**Implements**: `ast_grep_core::language::Language`, `ast_grep_core::tree_sitter::LanguageExt`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn all_langs() -> Vec<Self>
fn file_types(&self) -> Types
fn name(&self) -> &str
fn register(regs: Vec<Registration>) -> Result<(), DynamicLangError>
```

**via `ast_grep_core::language::Language`**

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn field_to_id(&self, field: &str) -> Option<u16>
fn from_path<P: AsRef<Path>>(path: P) -> Option<Self>
fn kind_to_id(&self, kind: &str) -> u16
fn meta_var_char(&self) -> char
fn pre_process_pattern<'q>(&self, query: &'q str) -> Cow<'q, str>
```

**via `ast_grep_core::tree_sitter::LanguageExt`**

```rust
fn get_ts_language(&self) -> TSLanguage
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(name: &str) -> Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

Represents a tree-sitter language loaded as dynamic lib.

---

## Registration

`struct` · `ast_grep_dynamic::Registration`

```rust
struct Registration
```

**Fields**: `lang_name`, `lib_path`, `symbol`, `meta_var_char`, `expando_char`, `extensions`

**Derives**: Default

---
