# `ruff_text_size::size`

Crate `ruff_text_size` · 1 public items · structured records in [`model/ruff_text_size.size.json`](../model/ruff_text_size.size.json)

## TextSize

`struct` · `ruff_text_size::size::TextSize`

Also reachable as `ruff_text_size::TextSize`

```rust
struct TextSize
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::iter::traits::accum::Sum`, `core::ops::arith::Add`, `core::ops::arith::AddAssign`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (8)

```rust
fn checked_add(self, rhs: TextSize) -> Option<TextSize>
fn checked_sub(self, rhs: TextSize) -> Option<TextSize>
const fn new(offset: u32) -> Self
fn of<T: TextLen>(text: T) -> TextSize
fn saturating_add(self, rhs: TextSize) -> TextSize
fn saturating_sub(self, rhs: TextSize) -> TextSize
const fn to_u32(&self) -> u32
const fn to_usize(&self) -> usize
```

**via `core::convert::From`**

```rust
fn from(raw: u32) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: usize) -> Result<Self, TryFromIntError>
```

**via `core::iter::traits::accum::Sum`**

```rust
fn sum<I: Iterator<Item = A>>(iter: I) -> TextSize
```

**via `core::ops::arith::Add`**

```rust
fn add(self, other: &TextSize) -> TextSize
fn add(self, other: TextSize) -> TextSize
```

**via `core::ops::arith::AddAssign`**

```rust
fn add_assign(&mut self, rhs: A)
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, other: &TextSize) -> TextSize
fn sub(self, other: TextSize) -> TextSize
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, rhs: S)
```

A measure of text length. Also, equivalently, an index into text.

This is a UTF-8 bytes offset stored as `u32`, but
most clients should treat it as an opaque measure.

For cases that need to escape `TextSize` and return to working directly
with primitive integers, `TextSize` can be converted losslessly to/from
`u32` via [`From`] conversions as well as losslessly be converted [`Into`]
`usize`. The `usize -> TextSize` direction can be done via [`TryFrom`].

These escape hatches are primarily required for unit testing and when
converting from UTF-8 size to another coordinate space, such as UTF-16.

---
