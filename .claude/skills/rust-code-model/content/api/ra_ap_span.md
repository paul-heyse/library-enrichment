# `ra_ap_span`

Crate `ra_ap_span` · 5 public items · structured records in [`model/ra_ap_span.json`](../model/ra_ap_span.json)

## EditionedFileId

`struct` · `ra_ap_span::EditionedFileId`

```rust
struct EditionedFileId
```

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (7)

```rust
const fn as_u32(self) -> u32
const fn current_edition(file_id: FileId) -> Self
const fn edition(self) -> Edition
const fn file_id(self) -> FileId
fn from_raw(u32: u32) -> Self
const fn new(file_id: FileId, edition: Edition) -> Self
const fn unpack(self) -> (FileId, Edition)
```

A [`FileId`] and [`Edition`] bundled up together.
The MSB is reserved for `HirFileId` encoding, more upper bits are used to then encode the edition.

---

## HirFileId

`struct` · `ra_ap_span::HirFileId`

```rust
struct HirFileId
```

**Derives**: Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

Input to the analyzer is a set of files, where each file is identified by
`FileId` and contains source code. However, another source of source code in
Rust are macros: each macro can be thought of as producing a "temporary
file". To assign an id to such a file, we use the id of the macro call that
produced the file. So, a `HirFileId` is either a `FileId` (source code
written by user), or a `MacroCallId` (source code produced by macro).

What is a `MacroCallId`? Simplifying, it's a `HirFileId` of a file
containing the call plus the offset of the macro call in the file. Note that
this is a recursive definition! However, the size_of of `HirFileId` is
finite (because everything bottoms out at the real `FileId`) and small
(`MacroCallId` uses the location interning. You can check details here:
<https://en.wikipedia.org/wiki/String_interning>).

Internally this holds a `salsa::Id`, but we cannot use this definition here
as it references things from base-db and hir-expand.

---

## MacroCallId

`struct` · `ra_ap_span::MacroCallId`

```rust
struct MacroCallId
```

**Implements**: `ra_ap_hir::MacroCallIdExt`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

`MacroCallId` identifies a particular macro invocation, like
`println!("Hello, {}", world)`.

---

## Span

`struct` · `ra_ap_span::Span`

```rust
struct Span
```

**Fields**: `range`, `anchor`, `ctx`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn cover(self, other: Span) -> Span
fn eq_ignoring_ctx(self, other: Self) -> bool
fn join(self, other: Span, differing_anchor: impl FnOnce(Span, Span) -> Option<Span>) -> Option<Span>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Spans represent a region of code, used by the IDE to be able link macro inputs and outputs
together. Positions in spans are relative to some [`SpanAnchor`] to make them more incremental
friendly.

---

## SpanAnchor

`struct` · `ra_ap_span::SpanAnchor`

```rust
struct SpanAnchor
```

**Fields**: `file_id`, `ast_id`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---
