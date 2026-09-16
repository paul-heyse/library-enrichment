# `ruff_linter::registry::rule_set`

Crate `ruff_linter` · 2 public items · structured records in [`model/ruff_linter.registry.rule_set.json`](../model/ruff_linter.registry.rule_set.json)

## RuleSet

`struct` · `ruff_linter::registry::rule_set::RuleSet`

Also reachable as `ruff_linter::registry::RuleSet`

```rust
struct RuleSet
```

**Implements**: `core::fmt::Display`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (13)

```rust
const fn contains(&self, rule: Rule) -> bool
const fn empty() -> Self
const fn from_rule(rule: Rule) -> Self
const fn from_rules(rules: &[Rule]) -> Self
fn insert(&mut self, rule: Rule)
const fn intersects(&self, other: &Self) -> bool
const fn is_empty(&self) -> bool
fn iter(&self) -> RuleSetIterator
const fn len(&self) -> usize
fn remove(&mut self, rule: Rule)
fn set(&mut self, rule: Rule, enabled: bool)
const fn subtract(self, other: &Self) -> Self
const fn union(self, other: &Self) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: IntoIterator<Item = Rule>>(&mut self, iter: T)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = Rule>>(iter: T) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

A set of [`Rule`]s.

Uses a bitset where a bit of one signals that the Rule with that [u16] is in this set.

---

## RuleSetIterator

`struct` · `ruff_linter::registry::rule_set::RuleSetIterator`

Also reachable as `ruff_linter::registry::RuleSetIterator`

```rust
struct RuleSetIterator
```

**Implements**: `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

---
