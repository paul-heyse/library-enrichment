# `datafusion_common::spans`

Crate `datafusion-common` · 3 public items · structured records in [`model/datafusion_common.spans.json`](../model/datafusion_common.spans.json)

## Location

`struct` · `datafusion_common::spans::Location`

Also reachable as `datafusion::common::Location`, `datafusion_common::Location`

```rust
struct Location
```

**Fields**: `line`, `column`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: sqlparser::tokenizer::Location) -> Self
```

Represents a location, determined by a line and a column number, in the
original SQL query.

---

## Span

`struct` · `datafusion_common::spans::Span`

Also reachable as `datafusion::common::Span`, `datafusion_common::Span`

```rust
struct Span
```

**Fields**: `start`, `end`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (5)

```rust
fn new(start: Location, end: Location) -> Self
fn try_from_sqlparser_span(span: sqlparser::tokenizer::Span) -> Option<Span>
fn union(&self, other: &Span) -> Span
fn union_iter<I: IntoIterator<Item = Span>>(iter: I) -> Option<Span>
fn union_opt(&self, other: &Option<Span>) -> Span
```

Represents an interval of characters in the original SQL query.

---

## Spans

`struct` · `datafusion_common::spans::Spans`

Also reachable as `datafusion::common::Spans`, `datafusion_common::Spans`

```rust
struct Spans
```

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (5)

```rust
fn add_span(&mut self, span: Span)
fn first(&self) -> Option<Span>
fn get_spans(&self) -> &[Span]
fn iter(&self) -> impl Iterator<Item = &Span>
fn new() -> Self
```

A collection of [`Span`], meant to be used as a field of entities whose
location in the original SQL query is desired to be tracked. Sometimes an
entity can have multiple spans. e.g. if you want to track the position of
the column a that comes from SELECT 1 AS a UNION ALL SELECT 2 AS a you'll
need two spans.

---
