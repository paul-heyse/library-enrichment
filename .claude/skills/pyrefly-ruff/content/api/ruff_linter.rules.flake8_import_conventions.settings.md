# `ruff_linter::rules::flake8_import_conventions::settings`

Crate `ruff_linter` · 4 public items · structured records in [`model/ruff_linter.rules.flake8_import_conventions.settings.json`](../model/ruff_linter.rules.flake8_import_conventions.settings.json)

## default_aliases

`function` · `ruff_linter::rules::flake8_import_conventions::settings::default_aliases`

```rust
fn default_aliases(preview: settings::types::PreviewMode) -> rustc_hash::FxHashMap<String, String>
```

---

## default_banned_aliases

`function` · `ruff_linter::rules::flake8_import_conventions::settings::default_banned_aliases`

```rust
fn default_banned_aliases(preview: settings::types::PreviewMode) -> rustc_hash::FxHashMap<String, BannedAliases>
```

---

## BannedAliases

`struct` · `ruff_linter::rules::flake8_import_conventions::settings::BannedAliases`

```rust
struct BannedAliases
```

**Implements**: `core::fmt::Display`, `core::iter::traits::collect::FromIterator`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self
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

`struct` · `ruff_linter::rules::flake8_import_conventions::settings::Settings`

```rust
struct Settings
```

**Fields**: `aliases`, `banned_aliases`, `banned_from`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn new(preview: PreviewMode) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---
