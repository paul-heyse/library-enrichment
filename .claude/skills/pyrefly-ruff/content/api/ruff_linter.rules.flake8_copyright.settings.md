# `ruff_linter::rules::flake8_copyright::settings`

Crate `ruff_linter` · 2 public items · structured records in [`model/ruff_linter.rules.flake8_copyright.settings.json`](../model/ruff_linter.rules.flake8_copyright.settings.json)

## COPYRIGHT

`static` · `ruff_linter::rules::flake8_copyright::settings::COPYRIGHT`

```rust
static COPYRIGHT: std::sync::LazyLock<regex::Regex>
```

---

## Settings

`struct` · `ruff_linter::rules::flake8_copyright::settings::Settings`

```rust
struct Settings
```

**Fields**: `notice_rgx`, `author`, `min_file_size`

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
