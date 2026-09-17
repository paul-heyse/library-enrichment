# `ra_ap_hir::has_source`

Crate `ra_ap_hir` · 1 public items · structured records in [`model/ra_ap_hir.has_source.json`](../model/ra_ap_hir.has_source.json)

## HasSource

`trait` · `ra_ap_hir::has_source::HasSource`

Also reachable as `ra_ap_hir::HasSource`

```rust
trait HasSource: Sized
```

**Implementors** (22)

- `ra_ap_hir::Adt`
- `ra_ap_hir::Const`
- `ra_ap_hir::Enum`
- `ra_ap_hir::EnumVariant`
- `ra_ap_hir::ExternCrateDecl`
- `ra_ap_hir::Field`
- `ra_ap_hir::Function`
- `ra_ap_hir::Impl`
- `ra_ap_hir::InlineAsmOperand`
- `ra_ap_hir::Label`
- `ra_ap_hir::LifetimeParam`
- `ra_ap_hir::LocalSource`
- `ra_ap_hir::Macro`
- `ra_ap_hir::Param`
- `ra_ap_hir::SelfParam`
- `ra_ap_hir::Static`
- `ra_ap_hir::Struct`
- `ra_ap_hir::Trait`
- `ra_ap_hir::TypeAlias`
- `ra_ap_hir::TypeOrConstParam`
- `ra_ap_hir::Union`
- `ra_ap_hir::Variant`

**Methods** (2)

```rust
fn source(self, db: &dyn HirDatabase) -> Option<InFile<Self::Ast>>
fn source_with_range(self, db: &dyn HirDatabase) -> Option<InFile<(TextRange, Option<Self::Ast>)>>
```

---
