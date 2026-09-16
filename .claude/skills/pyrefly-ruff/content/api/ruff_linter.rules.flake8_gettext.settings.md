# `ruff_linter::rules::flake8_gettext::settings`

Crate `ruff_linter` · 2 public items · structured records in [`model/ruff_linter.rules.flake8_gettext.settings.json`](../model/ruff_linter.rules.flake8_gettext.settings.json)

## default_func_names

`function` · `ruff_linter::rules::flake8_gettext::settings::default_func_names`

```rust
fn default_func_names() -> Vec<ruff_python_ast::name::Name>
```

---

## Settings

`struct` · `ruff_linter::rules::flake8_gettext::settings::Settings`

```rust
struct Settings
```

**Fields**: `function_names`

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
