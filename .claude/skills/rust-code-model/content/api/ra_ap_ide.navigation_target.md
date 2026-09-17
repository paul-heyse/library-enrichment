# `ra_ap_ide::navigation_target`

Crate `ra_ap_ide` · 3 public items · structured records in [`model/ra_ap_ide.navigation_target.json`](../model/ra_ap_ide.navigation_target.json)

## NavigationTarget

`struct` · `ra_ap_ide::navigation_target::NavigationTarget`

Also reachable as `ra_ap_ide::NavigationTarget`

```rust
struct NavigationTarget
```

**Fields**: `file_id`, `full_range`, `focus_range`, `name`, `kind`, `container_name`, `description`, `alias`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn focus_or_full_range(&self) -> TextRange
```

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, analysis: &ide_db::ra_fixture::RaFixtureAnalysis, _virtual_file_id: FileId, real_file_id: FileId) -> Result<Self, ()>
```

`NavigationTarget` represents an element in the editor's UI which you can
click on to navigate to a particular piece of code.

Typically, a `NavigationTarget` corresponds to some element in the source
code, like a function or a struct, but this is not strictly required.

---

## UpmappingResult

`struct` · `ra_ap_ide::navigation_target::UpmappingResult`

Also reachable as `ra_ap_ide::UpmappingResult`

```rust
struct UpmappingResult<T>
```

**Fields**: `call_site`, `def_site`

**Implements**: `core::iter::traits::collect::IntoIterator`

**Derives**: Debug

**Methods** (2)

```rust
fn call_site(self) -> T
fn collect<FI: FromIterator<T>>(self) -> FI
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

---

## TryToNav

`trait` · `ra_ap_ide::navigation_target::TryToNav`

Also reachable as `ra_ap_ide::TryToNav`

```rust
trait TryToNav
```

**Implementors** (18)

- `either::Either`
- `ra_ap_hir::Adt`
- `ra_ap_hir::AssocItem`
- `ra_ap_hir::BuiltinType`
- `ra_ap_hir::ConstParam`
- `ra_ap_hir::ExternCrateDecl`
- `ra_ap_hir::Field`
- `ra_ap_hir::GenericParam`
- `ra_ap_hir::Impl`
- `ra_ap_hir::InlineAsmOperand`
- `ra_ap_hir::Label`
- `ra_ap_hir::LifetimeParam`
- `ra_ap_hir::Macro`
- `ra_ap_hir::ModuleDef`
- `ra_ap_hir::TypeOrConstParam`
- `ra_ap_hir::TypeParam`
- `ra_ap_hir::symbols::FileSymbol`
- `ra_ap_ide_db::defs::Definition`

**Methods** (1)

```rust
fn try_to_nav(&self, sema: &Semantics<'_, RootDatabase>) -> Option<UpmappingResult<NavigationTarget>>
```

---
