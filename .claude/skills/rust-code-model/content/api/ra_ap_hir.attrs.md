# `ra_ap_hir::attrs`

Crate `ra_ap_hir` · 4 public items · structured records in [`model/ra_ap_hir.attrs.json`](../model/ra_ap_hir.attrs.json)

## AttrsOwner

`enum` · `ra_ap_hir::attrs::AttrsOwner`

```rust
enum AttrsOwner
```

**Variants**: `AttrDef`, `Field`, `LifetimeParam`, `TypeOrConstParam`, `Dummy`

---

## resolve_doc_path_on

`function` · `ra_ap_hir::attrs::resolve_doc_path_on`

Also reachable as `ra_ap_hir::resolve_doc_path_on`

```rust
fn resolve_doc_path_on(db: &dyn HirDatabase, def: impl HasAttrs + Copy, link: &str, ns: Option<hir_def::per_ns::Namespace>, is_inner_doc: hir_def::attrs::IsInnerDoc) -> Option<DocLinkDef>
```

Resolves the item `link` points to in the scope of `def`.

---

## AttrsWithOwner

`struct` · `ra_ap_hir::attrs::AttrsWithOwner`

Also reachable as `ra_ap_hir::AttrsWithOwner`

```rust
struct AttrsWithOwner
```

**Derives**: Clone, Debug

**Methods** (12)

```rust
fn cfgs<'db>(&self, db: &'db dyn HirDatabase) -> Option<&'db CfgExpr>
fn doc_aliases<'db>(&self, db: &'db dyn HirDatabase) -> &'db [Symbol]
fn hir_docs<'db>(&self, db: &'db dyn HirDatabase) -> Option<&'db Docs>
fn is_deprecated(&self) -> bool
fn is_doc_hidden(&self) -> bool
fn is_doc_notable_trait(&self) -> bool
fn is_macro_export(&self) -> bool
fn is_non_exhaustive(&self) -> bool
fn is_test(&self) -> bool
fn is_unstable(&self) -> bool
fn lang(&self, db: &dyn HirDatabase) -> Option<LangItem>
fn unstable_feature(&self, db: &dyn HirDatabase) -> Option<Symbol>
```

---

## HasAttrs

`trait` · `ra_ap_hir::attrs::HasAttrs`

Also reachable as `ra_ap_hir::HasAttrs`

```rust
trait HasAttrs: Sized
```

**Implementors** (22)

- `ra_ap_hir::Adt`
- `ra_ap_hir::AssocItem`
- `ra_ap_hir::Const`
- `ra_ap_hir::ConstParam`
- `ra_ap_hir::Crate`
- `ra_ap_hir::Enum`
- `ra_ap_hir::EnumVariant`
- `ra_ap_hir::ExternCrateDecl`
- `ra_ap_hir::Field`
- `ra_ap_hir::Function`
- `ra_ap_hir::GenericParam`
- `ra_ap_hir::Impl`
- `ra_ap_hir::LifetimeParam`
- `ra_ap_hir::Macro`
- `ra_ap_hir::Module`
- `ra_ap_hir::ModuleDef`
- `ra_ap_hir::Static`
- `ra_ap_hir::Struct`
- `ra_ap_hir::Trait`
- `ra_ap_hir::TypeAlias`
- `ra_ap_hir::TypeParam`
- `ra_ap_hir::Union`

**Methods** (2)

```rust
fn attrs(self, db: &dyn HirDatabase) -> AttrsWithOwner
fn hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>
```

---
