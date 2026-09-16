# `ruff_linter::rules::flake8_bandit::settings`

Crate `ruff_linter` · 2 public items · structured records in [`model/ruff_linter.rules.flake8_bandit.settings.json`](../model/ruff_linter.rules.flake8_bandit.settings.json)

## default_tmp_dirs

`function` · `ruff_linter::rules::flake8_bandit::settings::default_tmp_dirs`

```rust
fn default_tmp_dirs() -> Vec<String>
```

---

## Settings

`struct` · `ruff_linter::rules::flake8_bandit::settings::Settings`

```rust
struct Settings
```

**Fields**: `hardcoded_tmp_directory`, `check_typed_exception`, `extend_markup_names`, `allowed_markup_calls`

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
