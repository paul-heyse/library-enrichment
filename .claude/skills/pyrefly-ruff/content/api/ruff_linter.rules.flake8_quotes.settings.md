# `ruff_linter::rules::flake8_quotes::settings`

Crate `ruff_linter` · 2 public items · structured records in [`model/ruff_linter.rules.flake8_quotes.settings.json`](../model/ruff_linter.rules.flake8_quotes.settings.json)

## Quote

`enum` · `ruff_linter::rules::flake8_quotes::settings::Quote`

```rust
enum Quote
```

**Variants**: `Double`, `Single`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: ruff_python_ast::str::Quote) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
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

## Settings

`struct` · `ruff_linter::rules::flake8_quotes::settings::Settings`

```rust
struct Settings
```

**Fields**: `inline_quotes`, `multiline_quotes`, `docstring_quotes`, `avoid_escape`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---
