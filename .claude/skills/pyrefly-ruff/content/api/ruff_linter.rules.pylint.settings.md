# `ruff_linter::rules::pylint::settings`

Crate `ruff_linter` · 2 public items · structured records in [`model/ruff_linter.rules.pylint.settings.json`](../model/ruff_linter.rules.pylint.settings.json)

## ConstantType

`enum` · `ruff_linter::rules::pylint::settings::ConstantType`

```rust
enum ConstantType
```

**Variants**: `Bytes`, `Complex`, `Float`, `Int`, `Str`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Settings

`struct` · `ruff_linter::rules::pylint::settings::Settings`

```rust
struct Settings
```

**Fields**: `allow_magic_value_types`, `allow_dunder_method_names`, `max_args`, `max_positional_args`, `max_returns`, `max_bool_expr`, `max_branches`, `max_statements`, `max_statements_in_try`, `max_public_methods`, `max_locals`, `max_nested_blocks`

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
