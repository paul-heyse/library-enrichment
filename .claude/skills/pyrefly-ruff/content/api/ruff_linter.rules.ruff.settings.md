# `ruff_linter::rules::ruff::settings`

Crate `ruff_linter` · 1 public items · structured records in [`model/ruff_linter.rules.ruff.settings.json`](../model/ruff_linter.rules.ruff.settings.json)

## Settings

`struct` · `ruff_linter::rules::ruff::settings::Settings`

```rust
struct Settings
```

**Fields**: `parenthesize_tuple_in_subscript`, `strictly_empty_init_modules`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---
