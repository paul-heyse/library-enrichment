# `ast_grep_dynamic::custom_lang`

Crate `ast-grep-dynamic` · 2 public items · structured records in [`model/ast_grep_dynamic.custom_lang.json`](../model/ast_grep_dynamic.custom_lang.json)

## LibraryPath

`enum` · `ast_grep_dynamic::custom_lang::LibraryPath`

Also reachable as `ast_grep_dynamic::LibraryPath`

```rust
enum LibraryPath
```

**Variants**: `Single`, `Platform`

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

---

## CustomLang

`struct` · `ast_grep_dynamic::custom_lang::CustomLang`

Also reachable as `ast_grep_dynamic::CustomLang`

```rust
struct CustomLang
```

**Fields**: `library_path`, `language_symbol`, `meta_var_char`, `expando_char`, `extensions`, `outline_rules`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone

**Methods** (1)

```rust
fn register(base: &Path, langs: HashMap<String, CustomLang>) -> Result<(), DynamicLangError>
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
