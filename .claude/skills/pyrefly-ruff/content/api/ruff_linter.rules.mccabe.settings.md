# `ruff_linter::rules::mccabe::settings`

Crate `ruff_linter` · 2 public items · structured records in [`model/ruff_linter.rules.mccabe.settings.json`](../model/ruff_linter.rules.mccabe.settings.json)

## DEFAULT_MAX_COMPLEXITY

`constant` · `ruff_linter::rules::mccabe::settings::DEFAULT_MAX_COMPLEXITY`

```rust
const DEFAULT_MAX_COMPLEXITY: usize = 10
```

---

## Settings

`struct` · `ruff_linter::rules::mccabe::settings::Settings`

```rust
struct Settings
```

**Fields**: `max_complexity`

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
