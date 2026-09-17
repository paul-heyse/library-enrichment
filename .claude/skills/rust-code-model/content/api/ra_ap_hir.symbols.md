# `ra_ap_hir::symbols`

Crate `ra_ap_hir` · 3 public items · structured records in [`model/ra_ap_hir.symbols.json`](../model/ra_ap_hir.symbols.json)

## DeclarationLocation

`struct` · `ra_ap_hir::symbols::DeclarationLocation`

```rust
struct DeclarationLocation
```

**Fields**: `hir_file_id`, `ptr`, `name_ptr`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn syntax<DB: HirDatabase>(&self, sema: &Semantics<'_, DB>) -> SyntaxNode
```

---

## FileSymbol

`struct` · `ra_ap_hir::symbols::FileSymbol`

```rust
struct FileSymbol<'db>
```

**Fields**: `name`, `def`, `loc`, `container_name`, `is_alias`, `is_assoc`, `is_import`, `do_not_complete`

**Implements**: `ra_ap_ide::navigation_target::TryToNav`, `salsa::salsa_value::SalsaValue`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

The actual data that is stored in the index. It should be as compact as
possible.

---

## SymbolCollector

`struct` · `ra_ap_hir::symbols::SymbolCollector`

```rust
struct SymbolCollector<'db>
```

**Methods** (5)

```rust
fn collect(&mut self, module: Module)
fn finish(self) -> Box<[FileSymbol<'a>]>
fn new(db: &'a dyn HirDatabase, collect_pub_only: bool) -> Self
fn new_module(db: &'a dyn HirDatabase, module: Module, collect_pub_only: bool) -> Box<[FileSymbol<'a>]>
fn push_crate_root(&mut self, krate: Crate)
```

---
