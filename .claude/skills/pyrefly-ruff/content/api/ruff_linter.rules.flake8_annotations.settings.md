# `ruff_linter::rules::flake8_annotations::settings`

Crate `ruff_linter` · 1 public items · structured records in [`model/ruff_linter.rules.flake8_annotations.settings.json`](../model/ruff_linter.rules.flake8_annotations.settings.json)

## Settings

`struct` · `ruff_linter::rules::flake8_annotations::settings::Settings`

```rust
struct Settings
```

**Fields**: `mypy_init_return`, `suppress_dummy_args`, `suppress_none_returning`, `allow_star_arg_any`, `ignore_fully_untyped`

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
