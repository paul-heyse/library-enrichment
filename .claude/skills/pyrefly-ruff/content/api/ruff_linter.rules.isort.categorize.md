# `ruff_linter::rules::isort::categorize`

Crate `ruff_linter` · 4 public items · structured records in [`model/ruff_linter.rules.isort.categorize.json`](../model/ruff_linter.rules.isort.categorize.json)

## ImportSection

`enum` · `ruff_linter::rules::isort::categorize::ImportSection`

Also reachable as `ruff_linter::rules::isort::ImportSection`

```rust
enum ImportSection
```

**Variants**: `Known`, `UserDefined`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

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

## ImportType

`enum` · `ruff_linter::rules::isort::categorize::ImportType`

Also reachable as `ruff_linter::rules::isort::ImportType`

```rust
enum ImportType
```

**Variants**: `Future`, `StandardLibrary`, `ThirdParty`, `FirstParty`, `LocalFolder`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `strum::IntoEnumIterator`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

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

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> ImportTypeIter
```

---

## ImportTypeIter

`struct` · `ruff_linter::rules::isort::categorize::ImportTypeIter`

```rust
struct ImportTypeIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [ImportType]

---

## KnownModules

`struct` · `ruff_linter::rules::isort::categorize::KnownModules`

```rust
struct KnownModules
```

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn new(first_party: Vec<IdentifierPattern>, third_party: Vec<IdentifierPattern>, local_folder: Vec<IdentifierPattern>, standard_library: Vec<IdentifierPattern>, user_defined: FxHashMap<String, Vec<IdentifierPattern>>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---
