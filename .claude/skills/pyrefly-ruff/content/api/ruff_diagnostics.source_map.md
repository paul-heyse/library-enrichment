# `ruff_diagnostics::source_map`

Crate `ruff_diagnostics` · 2 public items · structured records in [`model/ruff_diagnostics.source_map.json`](../model/ruff_diagnostics.source_map.json)

## SourceMap

`struct` · `ruff_diagnostics::source_map::SourceMap`

Also reachable as `ruff_diagnostics::SourceMap`

```rust
struct SourceMap
```

**Derives**: Default, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn markers(&self) -> &[SourceMarker]
fn push_end_marker(&mut self, edit: &Edit, output_length: TextSize)
fn push_marker(&mut self, offset: TextSize, output_length: TextSize)
fn push_start_marker(&mut self, edit: &Edit, output_length: TextSize)
```

A collection of [`SourceMarker`].

Sourcemaps are used to map positions in the original source to positions in
the transformed code. Here, only the boundaries of edits are tracked instead
of every single character.

This mapping maintains the invariant that markers are in source order.

---

## SourceMarker

`struct` · `ruff_diagnostics::source_map::SourceMarker`

Also reachable as `ruff_diagnostics::SourceMarker`

```rust
struct SourceMarker
```

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn dest(&self) -> TextSize
fn new(source: TextSize, dest: TextSize) -> Self
const fn source(&self) -> TextSize
```

Lightweight sourcemap marker representing the source and destination
position for an [`Edit`].

---
