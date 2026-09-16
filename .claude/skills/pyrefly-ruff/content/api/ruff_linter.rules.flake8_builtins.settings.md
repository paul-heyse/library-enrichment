# `ruff_linter::rules::flake8_builtins::settings`

Crate `ruff_linter` · 1 public items · structured records in [`model/ruff_linter.rules.flake8_builtins.settings.json`](../model/ruff_linter.rules.flake8_builtins.settings.json)

## Settings

`struct` · `ruff_linter::rules::flake8_builtins::settings::Settings`

```rust
struct Settings
```

**Fields**: `ignorelist`, `allowed_modules`, `strict_checking`

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
