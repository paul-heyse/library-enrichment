# `ruff_ranged_value`

Crate `ruff_ranged_value` · 3 public items · structured records in [`model/ruff_ranged_value.json`](../model/ruff_ranged_value.json)

## ValueSource

`enum` · `ruff_ranged_value::ValueSource`

```rust
enum ValueSource
```

**Variants**: `File`, `ScriptMetadata`, `Cli`, `Editor`, `UvMetadata`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn file(&self, db: &dyn Db) -> Option<File>
```

---

## RangedValue

`struct` · `ruff_ranged_value::RangedValue`

```rust
struct RangedValue<T>
```

**Implements**: `core::convert::AsRef`, `core::fmt::Display`, `core::iter::traits::collect::IntoIterator`, `core::ops::deref::Deref`, `core::ops::deref::DerefMut`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (7)

```rust
fn cli(value: T) -> Self
fn into_inner(self) -> T
fn map_value<R>(self, f: impl FnOnce(T) -> R) -> RangedValue<R>
fn new(value: T, source: ValueSource) -> Self
fn python_extension(value: T) -> Self
fn range(&self) -> Option<TextRange>
fn source(&self) -> &ValueSource
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &U
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut T
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A value that "remembers" where it comes from (source) and its range in source.

## Equality, Hash, and Ordering
The equality, hash, and ordering are solely based on the value. They disregard the value's range
or source.

This ensures that two resolved configurations are identical even if the position of a value has changed
or if the values were loaded from different sources.

---

## ValueSourceGuard

`struct` · `ruff_ranged_value::ValueSourceGuard`

```rust
struct ValueSourceGuard
```

**Implements**: `core::ops::drop::Drop`

**Methods** (3)

```rust
fn new(source: ValueSource, is_toml: bool) -> Self
fn with_source_map(source: ValueSource, source_map: ScriptSourceMap) -> Self
fn without_spans() -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

Guard to safely change the [`ValueSource`] for the current thread.

---
