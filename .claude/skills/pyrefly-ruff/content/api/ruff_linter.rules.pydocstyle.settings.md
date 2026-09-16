# `ruff_linter::rules::pydocstyle::settings`

Crate `ruff_linter` · 3 public items · structured records in [`model/ruff_linter.rules.pydocstyle.settings.json`](../model/ruff_linter.rules.pydocstyle.settings.json)

## Convention

`enum` · `ruff_linter::rules::pydocstyle::settings::Convention`

```rust
enum Convention
```

**Variants**: `Google`, `Numpy`, `Pep257`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn rules_to_be_ignored(self) -> &'static [Rule]
```

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

## DecoratorIterator

`struct` · `ruff_linter::rules::pydocstyle::settings::DecoratorIterator`

```rust
struct DecoratorIterator<'a>
```

**Implements**: `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<QualifiedName<'a>>
```

---

## Settings

`struct` · `ruff_linter::rules::pydocstyle::settings::Settings`

```rust
struct Settings
```

**Fields**: `convention`, `ignore_decorators`, `property_decorators`, `ignore_var_parameters`

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
