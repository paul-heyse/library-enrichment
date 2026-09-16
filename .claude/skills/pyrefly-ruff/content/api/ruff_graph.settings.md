# `ruff_graph::settings`

Crate `ruff_graph` · 3 public items · structured records in [`model/ruff_graph.settings.json`](../model/ruff_graph.settings.json)

## Direction

`enum` · `ruff_graph::settings::Direction`

Also reachable as `ruff_graph::Direction`

```rust
enum Direction
```

**Variants**: `Dependencies`, `Dependents`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---

## AnalyzeSettings

`struct` · `ruff_graph::settings::AnalyzeSettings`

Also reachable as `ruff_graph::AnalyzeSettings`

```rust
struct AnalyzeSettings
```

**Fields**: `exclude`, `preview`, `target_version`, `string_imports`, `include_dependencies`, `extension`, `type_checking_imports`

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

## StringImports

`struct` · `ruff_graph::settings::StringImports`

Also reachable as `ruff_graph::StringImports`

```rust
struct StringImports
```

**Fields**: `enabled`, `min_dots`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Copy, Debug, Default

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---
