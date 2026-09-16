# `ruff_text_size::range`

Crate `ruff_text_size` · 1 public items · structured records in [`model/ruff_text_size.range.json`](../model/ruff_text_size.range.json)

## TextRange

`struct` · `ruff_text_size::range::TextRange`

Also reachable as `ruff_text_size::TextRange`

```rust
struct TextRange
```

**Implements**: `core::convert::From`, `core::ops::arith::Add`, `core::ops::arith::AddAssign`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `core::ops::range::RangeBounds`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::display::DisplayWith`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (24)

```rust
fn add_end(&self, amount: TextSize) -> TextRange
fn add_start(&self, amount: TextSize) -> TextRange
fn at(offset: TextSize, len: TextSize) -> TextRange
fn checked_add(self, offset: TextSize) -> Option<TextRange>
fn checked_sub(self, offset: TextSize) -> Option<TextRange>
fn contains(self, offset: TextSize) -> bool
fn contains_inclusive(self, offset: TextSize) -> bool
fn contains_range(self, other: TextRange) -> bool
fn cover(self, other: TextRange) -> TextRange
fn cover_offset(self, offset: TextSize) -> TextRange
fn empty(offset: TextSize) -> TextRange
const fn end(self) -> TextSize
fn intersect(self, other: TextRange) -> Option<TextRange>
const fn is_empty(self) -> bool
const fn len(self) -> TextSize
const fn new(start: TextSize, end: TextSize) -> TextRange
fn ordering(self, other: TextRange) -> Ordering
const fn start(self) -> TextSize
fn sub_end(&self, amount: TextSize) -> TextRange
fn sub_start(&self, amount: TextSize) -> TextRange
fn to_std_range(&self) -> Range<usize>
fn up_to(end: TextSize) -> TextRange
fn with_end(&self, offset: TextSize) -> TextRange
fn with_start(&self, offset: TextSize) -> TextRange
```

**via `core::convert::From`**

```rust
fn from(r: Range<TextSize>) -> Self
```

**via `core::ops::arith::Add`**

```rust
fn add(self, offset: TextSize) -> TextRange
fn add(self, other: &TextSize) -> TextRange
```

**via `core::ops::arith::AddAssign`**

```rust
fn add_assign(&mut self, rhs: A)
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, offset: TextSize) -> TextRange
fn sub(self, other: &TextSize) -> TextRange
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, rhs: S)
```

**via `core::ops::range::RangeBounds`**

```rust
fn end_bound(&self) -> Bound<&TextSize>
fn start_bound(&self) -> Bound<&TextSize>
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

A range in text, represented as a pair of [`TextSize`][struct@TextSize].

It is a logic error for `start` to be greater than `end`.

---
