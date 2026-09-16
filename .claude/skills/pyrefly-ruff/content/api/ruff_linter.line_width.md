# `ruff_linter::line_width`

Crate `ruff_linter` · 5 public items · structured records in [`model/ruff_linter.line_width.json`](../model/ruff_linter.line_width.json)

## ParseLineWidthError

`enum` · `ruff_linter::line_width::ParseLineWidthError`

```rust
enum ParseLineWidthError
```

**Variants**: `ParseError`, `TryFromIntError`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Error type returned when parsing a [`LineLength`] from a string fails

---

## IndentWidth

`struct` · `ruff_linter::line_width::IndentWidth`

```rust
struct IndentWidth
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(tab_size: NonZeroU8) -> Self
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

The size of a tab.

---

## LineLength

`struct` · `ruff_linter::line_width::LineLength`

```rust
struct LineLength
```

**Implements**: `core::convert::TryFrom`, `core::fmt::Display`, `core::str::traits::FromStr`, `ruff_cache::cache_key::CacheKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn value(&self) -> u16
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: u16) -> Result<Self, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, state: &mut CacheKeyHasher)
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The length of a line of text that is considered too long.

The allowed range of values is 1..=65535

---

## LineLengthFromIntError

`struct` · `ruff_linter::line_width::LineLengthFromIntError`

```rust
struct LineLengthFromIntError
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Error type returned when converting a u16 to a [`LineLength`] fails

---

## LineWidthBuilder

`struct` · `ruff_linter::line_width::LineWidthBuilder`

```rust
struct LineWidthBuilder
```

**Derives**: Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd

A measure of the width of a line of text.

This is used to determine if a line is too long.
It should be compared to a [`LineLength`].

---
