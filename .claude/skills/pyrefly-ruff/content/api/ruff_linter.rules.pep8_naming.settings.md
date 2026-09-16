# `ruff_linter::rules::pep8_naming::settings`

Crate `ruff_linter` · 3 public items · structured records in [`model/ruff_linter.rules.pep8_naming.settings.json`](../model/ruff_linter.rules.pep8_naming.settings.json)

## IgnoreNames

`enum` · `ruff_linter::rules::pep8_naming::settings::IgnoreNames`

```rust
enum IgnoreNames
```

**Variants**: `Default`, `UserProvided`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn from_options(ignore_names: Option<Vec<String>>, extend_ignore_names: Option<Vec<String>>) -> Result<Self, SettingsError>
fn from_patterns(patterns: impl IntoIterator<Item = String>) -> Result<Self, SettingsError>
fn matches(&self, name: &str) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, state: &mut CacheKeyHasher)
```

---

## SettingsError

`enum` · `ruff_linter::rules::pep8_naming::settings::SettingsError`

```rust
enum SettingsError
```

**Variants**: `InvalidIgnoreName`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Error returned by the [`TryFrom`] implementation of [`Settings`].

---

## Settings

`struct` · `ruff_linter::rules::pep8_naming::settings::Settings`

```rust
struct Settings
```

**Fields**: `ignore_names`, `classmethod_decorators`, `staticmethod_decorators`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---
