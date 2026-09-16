# `ruff_linter::rules::flake8_pytest_style::settings`

Crate `ruff_linter` · 4 public items · structured records in [`model/ruff_linter.rules.flake8_pytest_style.settings.json`](../model/ruff_linter.rules.flake8_pytest_style.settings.json)

## SettingsError

`enum` · `ruff_linter::rules::flake8_pytest_style::settings::SettingsError`

```rust
enum SettingsError
```

**Variants**: `InvalidRaisesRequireMatchFor`, `InvalidRaisesExtendRequireMatchFor`, `InvalidWarnsRequireMatchFor`, `InvalidWarnsExtendRequireMatchFor`

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

## default_broad_exceptions

`function` · `ruff_linter::rules::flake8_pytest_style::settings::default_broad_exceptions`

```rust
fn default_broad_exceptions() -> Vec<settings::types::IdentifierPattern>
```

---

## default_broad_warnings

`function` · `ruff_linter::rules::flake8_pytest_style::settings::default_broad_warnings`

```rust
fn default_broad_warnings() -> Vec<settings::types::IdentifierPattern>
```

---

## Settings

`struct` · `ruff_linter::rules::flake8_pytest_style::settings::Settings`

```rust
struct Settings
```

**Fields**: `fixture_parentheses`, `parametrize_names_type`, `parametrize_values_type`, `parametrize_values_row_type`, `raises_require_match_for`, `raises_extend_require_match_for`, `mark_parentheses`, `warns_require_match_for`, `warns_extend_require_match_for`

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
