# `ruff_linter::rules::flake8_errmsg::settings`

Crate `ruff_linter` · 1 public items · structured records in [`model/ruff_linter.rules.flake8_errmsg.settings.json`](../model/ruff_linter.rules.flake8_errmsg.settings.json)

## Settings

`struct` · `ruff_linter::rules::flake8_errmsg::settings::Settings`

```rust
struct Settings
```

**Fields**: `max_string_length`

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
