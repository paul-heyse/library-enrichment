# `ruff_diagnostics::fix`

Crate `ruff_diagnostics` · 3 public items · structured records in [`model/ruff_diagnostics.fix.json`](../model/ruff_diagnostics.fix.json)

## Applicability

`enum` · `ruff_diagnostics::fix::Applicability`

Also reachable as `ruff_diagnostics::Applicability`

```rust
enum Applicability
```

**Variants**: `DisplayOnly`, `Unsafe`, `Safe`

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
const fn is_display_only(&self) -> bool
const fn is_safe(&self) -> bool
const fn is_unsafe(&self) -> bool
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

Indicates if a fix can be applied.

---

## IsolationLevel

`enum` · `ruff_diagnostics::fix::IsolationLevel`

Also reachable as `ruff_diagnostics::IsolationLevel`

```rust
enum IsolationLevel
```

**Variants**: `Group`, `NonOverlapping`

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

Indicates the level of isolation required to apply a fix.

---

## Fix

`struct` · `ruff_diagnostics::fix::Fix`

Also reachable as `ruff_diagnostics::Fix`

```rust
struct Fix
```

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (16)

```rust
fn applicability(&self) -> Applicability
fn applicable_edit(edit: Edit, applicability: Applicability) -> Self
fn applicable_edits(edit: Edit, rest: impl IntoIterator<Item = Edit>, applicability: Applicability) -> Self
fn applies(&self, applicability: Applicability) -> bool
fn display_only_edit(edit: Edit) -> Self
fn display_only_edits(edit: Edit, rest: impl IntoIterator<Item = Edit>) -> Self
fn edits(&self) -> &[Edit]
fn into_edits(self) -> Vec<Edit>
fn isolate(self, isolation: IsolationLevel) -> Self
fn isolation(&self) -> IsolationLevel
fn min_start(&self) -> Option<TextSize>
fn safe_edit(edit: Edit) -> Self
fn safe_edits(edit: Edit, rest: impl IntoIterator<Item = Edit>) -> Self
fn unsafe_edit(edit: Edit) -> Self
fn unsafe_edits(edit: Edit, rest: impl IntoIterator<Item = Edit>) -> Self
fn with_applicability(self, applicability: Applicability) -> Self
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

A collection of [`Edit`] elements to be applied to a source file.

---
