# `ra_ap_ide::folding_ranges`

Crate `ra_ap_ide` · 2 public items · structured records in [`model/ra_ap_ide.folding_ranges.json`](../model/ra_ap_ide.folding_ranges.json)

## FoldKind

`enum` · `ra_ap_ide::folding_ranges::FoldKind`

Also reachable as `ra_ap_ide::FoldKind`

```rust
enum FoldKind
```

**Variants**: `Comment`, `Imports`, `Region`, `Block`, `ArgList`, `Array`, `WhereClause`, `ReturnType`, `MatchArm`, `Function`, `Modules`, `Consts`, `Statics`, `TypeAliases`, `ExternCrates`, `Stmt`, `TailExpr`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## Fold

`struct` · `ra_ap_ide::folding_ranges::Fold`

Also reachable as `ra_ap_ide::Fold`

```rust
struct Fold
```

**Fields**: `range`, `kind`, `collapsed_text`

**Derives**: Debug

**Methods** (2)

```rust
fn new(range: TextRange, kind: FoldKind) -> Self
fn with_text(self, text: Option<String>) -> Self
```

---
