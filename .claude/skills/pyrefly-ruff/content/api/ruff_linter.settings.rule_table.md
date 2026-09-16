# `ruff_linter::settings::rule_table`

Crate `ruff_linter` · 1 public items · structured records in [`model/ruff_linter.settings.rule_table.json`](../model/ruff_linter.settings.rule_table.json)

## RuleTable

`struct` · `ruff_linter::settings::rule_table::RuleTable`

```rust
struct RuleTable
```

**Implements**: `core::fmt::Display`, `core::iter::traits::collect::FromIterator`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**Methods** (6)

```rust
const fn any_enabled(&self, rules: &[Rule]) -> bool
fn disable(&mut self, rule: Rule)
const fn empty() -> Self
fn enable(&mut self, rule: Rule, should_fix: bool)
const fn enabled(&self, rule: Rule) -> bool
fn iter_enabled(&self) -> RuleSetIterator
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = Rule>>(iter: T) -> Self
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

A table to keep track of which rules are enabled and whether they should be fixed.

---
