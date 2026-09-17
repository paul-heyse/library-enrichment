# `ra_ap_span::map`

Crate `ra_ap_span` · 2 public items · structured records in [`model/ra_ap_span.map.json`](../model/ra_ap_span.map.json)

## RealSpanMap

`struct` · `ra_ap_span::map::RealSpanMap`

Also reachable as `ra_ap_span::RealSpanMap`

```rust
struct RealSpanMap
```

**Implements**: `core::fmt::Display`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn absolute(file_id: EditionedFileId) -> Self
fn from_file(file_id: EditionedFileId, pairs: Box<[(TextSize, ErasedFileAstId)]>, end: TextSize) -> Self
fn span_for_range(&self, range: TextRange) -> Span
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## SpanMap

`struct` · `ra_ap_span::map::SpanMap`

Also reachable as `ra_ap_span::SpanMap`

```rust
struct SpanMap
```

**Fields**: `matched_arm`

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn empty() -> Self
fn finish(&mut self)
fn iter(&self) -> impl Iterator<Item = (TextSize, Span)> + '_
fn merge(&mut self, other_range: TextRange, other_size: TextSize, other: &SpanMap)
fn push(&mut self, offset: TextSize, span: Span)
fn ranges_with_span(&self, span: Span) -> impl Iterator<Item = (TextRange, SyntaxContext)> + '_
fn ranges_with_span_exact(&self, span: Span) -> impl Iterator<Item = (TextRange, SyntaxContext)> + '_
fn span_at(&self, offset: TextSize) -> Span
fn spans_for_range(&self, range: TextRange) -> impl Iterator<Item = Span> + '_
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

Maps absolute text ranges for the corresponding file to the relevant span data.

---
