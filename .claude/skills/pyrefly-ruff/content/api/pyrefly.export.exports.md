# `pyrefly::export::exports`

Crate `pyrefly` · 5 public items · structured records in [`model/pyrefly.export.exports.json`](../model/pyrefly.export.exports.json)

## ExportLocation

`enum` · `pyrefly::export::exports::ExportLocation`

```rust
enum ExportLocation
```

**Variants**: `ThisModule`, `OtherModule`

**Derives**: Clone, Debug

Where is this export defined?

---

## Export

`struct` · `pyrefly::export::exports::Export`

```rust
struct Export
```

**Fields**: `location`, `symbol_kind`, `docstring_range`, `deprecation`, `is_final`, `special_export`

**Derives**: Clone, Debug

---

## ExportOrigin

`struct` · `pyrefly::export::exports::ExportOrigin`

```rust
struct ExportOrigin
```

**Fields**: `origin`, `is_final`

Result of checking whether an export is `Final`, including the defining module and name
found by following re-export chains.

---

## Exports

`struct` · `pyrefly::export::exports::Exports`

```rust
struct Exports
```

**Implements**: `core::fmt::Display`

**Derives**: Debug

**Methods** (14)

```rust
fn changed_exports(&self, other: &Exports, lookup: &dyn LookupExport, changed: &mut ModuleChanges)
fn docstring_range(&self) -> Option<TextRange>
fn dunder_all_name_at(&self, position: TextSize) -> Option<(TextRange, Name)>
fn explicit_dunder_all_names(&self) -> Option<&SmallSet<Name>>
fn exports(&self, lookup: &dyn LookupExport) -> Arc<SmallMap<Name, ExportLocation>>
fn get_partially_known_dunder_all(definitions: &Definitions) -> SmallSet<Name>
fn invalid_dunder_all_entries(&self, lookup: &dyn LookupExport) -> Vec<(TextRange, Name)>
fn is_explicit_reexport(&self, name: &Name) -> bool
fn is_implicit_reexport(&self, name: &Name) -> bool
fn is_submodule_imported_implicitly(&self, name: &Name) -> bool
fn new(x: &[Stmt], module_info: &pyrefly_python::module::Module, sys_info: SysInfo, build_symbols: bool) -> Self
fn symbols(&self) -> Option<&FlatSymbols>
fn unresolvable_dunder_all_range(&self) -> Option<TextRange>
fn wildcard(&self, lookup: &dyn LookupExport) -> Arc<SmallSet<Name>>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## LookupExport

`trait` · `pyrefly::export::exports::LookupExport`

```rust
trait LookupExport
```

**Methods** (11)

```rust
fn docstring_range(&self, module: ModuleName, name: &Name) -> Option<TextRange>
fn export_exists(&self, module: ModuleName, name: &Name) -> bool
fn export_origin(&self, module: ModuleName, name: &Name) -> ExportOrigin
fn get_deprecated(&self, module: ModuleName, name: &Name) -> Option<Deprecation>
fn get_every_export_untracked(&self, module: ModuleName) -> Option<SmallSet<Name>>
fn get_wildcard(&self, module: ModuleName) -> Option<Arc<SmallSet<Name>>>
fn is_implicit_reexport(&self, module: ModuleName, name: &Name) -> bool
fn is_special_export(&self, module: ModuleName, name: &Name) -> Option<SpecialExport>
fn is_submodule_imported_implicitly(&self, module: ModuleName, name: &Name) -> bool
fn module_exists(&self, module: ModuleName) -> FindingOrError<()>
fn reexport_source(&self, module: ModuleName, name: &Name) -> Option<ModuleName>
```

Find the exports of a given module. Beware: these APIs record dependencies between modules during lookups. Using the
wrong API can lead to invalidation bugs.

---
