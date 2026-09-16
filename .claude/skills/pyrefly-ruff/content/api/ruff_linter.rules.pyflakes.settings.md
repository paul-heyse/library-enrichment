# `ruff_linter::rules::pyflakes::settings`

Crate `ruff_linter` · 1 public items · structured records in [`model/ruff_linter.rules.pyflakes.settings.json`](../model/ruff_linter.rules.pyflakes.settings.json)

## Settings

`struct` · `ruff_linter::rules::pyflakes::settings::Settings`

```rust
struct Settings
```

**Fields**: `extend_generics`, `allowed_unused_imports`

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
