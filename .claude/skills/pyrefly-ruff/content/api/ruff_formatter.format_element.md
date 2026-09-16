# `ruff_formatter::format_element`

Crate `ruff_formatter` · 11 public items · structured records in [`model/ruff_formatter.format_element.json`](../model/ruff_formatter.format_element.json)

## BestFittingMode

`enum` · `ruff_formatter::format_element::BestFittingMode`

Also reachable as `ruff_formatter::prelude::BestFittingMode`

```rust
enum BestFittingMode
```

**Variants**: `FirstLine`, `AllLines`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

Mode used to determine if any variant (except the most expanded) fits for [`BestFittingVariants`].

---

## FormatElement

`enum` · `ruff_formatter::format_element::FormatElement`

Also reachable as `ruff_formatter::FormatElement`, `ruff_formatter::prelude::FormatElement`

```rust
enum FormatElement
```

**Variants**: `Space`, `Line`, `ExpandParent`, `SourcePosition`, `Token`, `Text`, `SourceCodeSlice`, `LineSuffixBoundary`, `Interned`, `BestFitting`, `Tag`

**Implements**: `ruff_formatter::format_element::FormatElements`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn is_start_tag(&self) -> bool
const fn is_tag(&self) -> bool
```

**via `ruff_formatter::format_element::FormatElements`**

```rust
fn end_tag(&self, kind: TagKind) -> Option<&Tag>
fn has_label(&self, label_id: LabelId) -> bool
fn start_tag(&self, _: TagKind) -> Option<&Tag>
fn will_break(&self) -> bool
```

Language agnostic IR for formatting source code.

Use the helper functions like [`crate::builders::space`], [`crate::builders::soft_line_break`] etc. defined in this file to create elements.

---

## LineMode

`enum` · `ruff_formatter::format_element::LineMode`

Also reachable as `ruff_formatter::prelude::LineMode`

```rust
enum LineMode
```

**Variants**: `SoftOrSpace`, `Soft`, `Hard`, `Empty`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn is_hard(&self) -> bool
```

---

## PrintMode

`enum` · `ruff_formatter::format_element::PrintMode`

Also reachable as `ruff_formatter::prelude::PrintMode`

```rust
enum PrintMode
```

**Variants**: `Flat`, `Expanded`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn is_expanded(&self) -> bool
const fn is_flat(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(value: GroupMode) -> Self
```

---

## TextWidth

`enum` · `ruff_formatter::format_element::TextWidth`

Also reachable as `ruff_formatter::prelude::TextWidth`

```rust
enum TextWidth
```

**Variants**: `Width`, `Multiline`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn from_text(text: &str, indent_width: IndentWidth) -> TextWidth
const fn width(self) -> Option<Width>
```

The pre-computed unicode width of a text if it is a single-line text or a marker
that it is a multiline text if it contains a line feed.

---

## normalize_newlines

`function` · `ruff_formatter::format_element::normalize_newlines`

Also reachable as `ruff_formatter::normalize_newlines`, `ruff_formatter::prelude::normalize_newlines`

```rust
fn normalize_newlines<const N: usize>(text: &str, terminators: [char; N]) -> std::borrow::Cow<'_, str>
```

Replace the line terminators matching the provided list with "\n"
since its the only line break type supported by the printer

---

## BestFittingVariants

`struct` · `ruff_formatter::format_element::BestFittingVariants`

Also reachable as `ruff_formatter::prelude::BestFittingVariants`

```rust
struct BestFittingVariants
```

**Implements**: `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn most_expanded(&self) -> &[FormatElement]
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

The different variants for this element.
The first element is the one that takes up the most space horizontally (the most flat),
The last element takes up the least space horizontally (but most horizontal space).

---

## BestFittingVariantsIter

`struct` · `ruff_formatter::format_element::BestFittingVariantsIter`

Also reachable as `ruff_formatter::prelude::BestFittingVariantsIter`

```rust
struct BestFittingVariantsIter<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn last(self) -> Option<Self::Item> where Self: Sized
fn next(&mut self) -> Option<Self::Item>
```

---

## Interned

`struct` · `ruff_formatter::format_element::Interned`

Also reachable as `ruff_formatter::prelude::Interned`

```rust
struct Interned
```

**Implements**: `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

---

## Width

`struct` · `ruff_formatter::format_element::Width`

Also reachable as `ruff_formatter::prelude::Width`

```rust
struct Width
```

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn value(self) -> u32
```

New-type wrapper for a single-line text unicode width.
Mainly to prevent access to the inner value.

## Representation

Represents the width by adding 1 to the actual width so that the width can be represented by a [`NonZeroU32`],
allowing [`TextWidth`] or [`Option<Width>`] fit in 4 bytes rather than 8.

This means that 2^32 cannot be precisely represented and instead has the same value as 2^32-1.
This imprecision shouldn't matter in practice because either text are longer than any configured line width
and thus, the text should break.

---

## FormatElements

`trait` · `ruff_formatter::format_element::FormatElements`

Also reachable as `ruff_formatter::prelude::FormatElements`

```rust
trait FormatElements
```

**Implementors** (1)

- `ruff_formatter::format_element::FormatElement`

**Methods** (4)

```rust
fn end_tag(&self, kind: TagKind) -> Option<&Tag>
fn has_label(&self, label: LabelId) -> bool
fn start_tag(&self, kind: TagKind) -> Option<&Tag>
fn will_break(&self) -> bool
```

---
