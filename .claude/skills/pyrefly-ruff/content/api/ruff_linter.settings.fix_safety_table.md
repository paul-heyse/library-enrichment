# `ruff_linter::settings::fix_safety_table`

Crate `ruff_linter` · 1 public items · structured records in [`model/ruff_linter.settings.fix_safety_table.json`](../model/ruff_linter.settings.fix_safety_table.json)

## FixSafetyTable

`struct` · `ruff_linter::settings::fix_safety_table::FixSafetyTable`

```rust
struct FixSafetyTable
```

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn from_rule_selectors(extend_safe_fixes: &[UnresolvedRuleSelector], extend_unsafe_fixes: &[UnresolvedRuleSelector], preview_options: &PreviewOptions) -> Result<Self, RuleResolutionError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

A table to keep track of which rules fixes should have
their safety overridden.

---
