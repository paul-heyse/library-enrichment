# `ra_ap_base_db::input`

Crate `ra_ap_base_db` · 25 public items · structured records in [`model/ra_ap_base_db.input.json`](../model/ra_ap_base_db.input.json)

## CrateOrigin

`enum` · `ra_ap_base_db::input::CrateOrigin`

Also reachable as `ra_ap_base_db::CrateOrigin`

```rust
enum CrateOrigin
```

**Variants**: `Rustc`, `Local`, `Library`, `Lang`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn is_lang(&self) -> bool
fn is_lib(&self) -> bool
fn is_local(&self) -> bool
```

Origin of the crates.

---

## LangCrateOrigin

`enum` · `ra_ap_base_db::input::LangCrateOrigin`

Also reachable as `ra_ap_base_db::LangCrateOrigin`

```rust
enum LangCrateOrigin
```

**Variants**: `Alloc`, `Core`, `ProcMacro`, `Std`, `Test`, `Dependency`, `Other`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(s: &str) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## ProcMacroLoadingError

`enum` · `ra_ap_base_db::input::ProcMacroLoadingError`

Also reachable as `ra_ap_base_db::ProcMacroLoadingError`

```rust
enum ProcMacroLoadingError
```

**Variants**: `Disabled`, `FailedToBuild`, `ExpectedProcMacroArtifact`, `MissingDylibPath`, `NotYetBuilt`, `NoProcMacros`, `ProcMacroSrvError`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_hard_error(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## ReleaseChannel

`enum` · `ra_ap_base_db::input::ReleaseChannel`

Also reachable as `ra_ap_base_db::ReleaseChannel`

```rust
enum ReleaseChannel
```

**Variants**: `Stable`, `Beta`, `Nightly`

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn as_str(self) -> &'static str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(str: &str) -> Result<Self, Self::Err>
```

---

## Crate

`struct` · `ra_ap_base_db::input::Crate`

Also reachable as `ra_ap_base_db::Crate`, `ra_ap_ide::Crate`

```rust
struct Crate
```

**Implements**: `core::convert::From`, `salsa::id::AsId`, `salsa::id::FromId`, `salsa::input::Configuration`, `salsa::input::HasBuilder`, `salsa::salsa_struct::SalsaStructInDb`, `salsa::salsa_value::SalsaValue`, `salsa::zalsa::HasJar`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (18)

```rust
fn builder(data: BuiltCrateData, extra_data: ExtraCrateData, workspace_data: Arc<CrateWorkspaceData>, cfg_options: CfgOptions, env: Env) -> <Self as zalsa_struct_::HasBuilder>::Builder
fn cfg_options<'db, Db_>(self, db: &'db Db_) -> &'db CfgOptions where Db_: ?Sized + zalsa_::Database
fn data<'db, Db_>(self, db: &'db Db_) -> &'db BuiltCrateData where Db_: ?Sized + zalsa_::Database
fn default_debug_fmt(this: Self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result where for<'__trivial_bounds> BuiltCrateData: ::std::fmt::Debug, for<'__trivial_bounds> ExtraCrateData: ::std::fmt::Debug, for<'__trivial_bounds> Arc<CrateWorkspaceData>: ::std::fmt::Debug, for<'__trivial_bounds> CfgOptions: ::std::fmt::Debug, for<'__trivial_bounds> Env: ::std::fmt::Debug
fn env<'db, Db_>(self, db: &'db Db_) -> &'db Env where Db_: ?Sized + zalsa_::Database
fn extra_data<'db, Db_>(self, db: &'db Db_) -> &'db ExtraCrateData where Db_: ?Sized + zalsa_::Database
fn ingredient(db: &dyn zalsa_::Database) -> &zalsa_struct_::IngredientImpl<Self>
fn ingredient_mut(zalsa_mut: &mut zalsa_::Zalsa) -> (&mut zalsa_struct_::IngredientImpl<Self>, &mut zalsa_::Runtime)
fn new<Db_>(db: &Db_, data: BuiltCrateData, extra_data: ExtraCrateData, workspace_data: Arc<CrateWorkspaceData>, cfg_options: CfgOptions, env: Env) -> Self where Db_: ?Sized + salsa::Database
fn root_file_id(self, db: &dyn salsa::Database) -> EditionedFileId
fn set_cfg_options<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = CfgOptions> where Db_: ?Sized + zalsa_::Database
fn set_data<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = BuiltCrateData> where Db_: ?Sized + zalsa_::Database
fn set_env<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = Env> where Db_: ?Sized + zalsa_::Database
fn set_extra_data<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = ExtraCrateData> where Db_: ?Sized + zalsa_::Database
fn set_workspace_data<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = Arc<CrateWorkspaceData>> where Db_: ?Sized + zalsa_::Database
fn transitive_deps(self, db: &dyn salsa::Database) -> Vec<Crate>
fn transitive_rev_deps(self, db: &dyn SourceDatabase) -> Box<[Crate]>
fn workspace_data<'db, Db_>(self, db: &'db Db_) -> &'db Arc<CrateWorkspaceData> where Db_: ?Sized + zalsa_::Database
```

**via `salsa::id::AsId`**

```rust
fn as_id(&self) -> salsa::Id
```

**via `salsa::id::FromId`**

```rust
fn from_id(id: salsa::Id) -> Self
```

**via `salsa::input::Configuration`**

```rust
fn deserialize<'de, D: zalsa_::serde::Deserializer<'de>>(deserializer: D) -> ::std::result::Result<Self::Fields, D::Error>
fn serialize<S: zalsa_::serde::Serializer>(fields: &Self::Fields, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
```

**via `salsa::salsa_struct::SalsaStructInDb`**

```rust
fn cast(id: zalsa_::Id, type_id: zalsa_::TypeId) -> zalsa_::Option<Self>
fn entries(zalsa: &zalsa_::Zalsa) -> impl Iterator<Item = zalsa_::DatabaseKeyIndex> + '_
fn lookup_ingredient_index(aux: &zalsa_::Zalsa) -> zalsa_::IngredientIndices
unsafe fn memo_table(zalsa: &zalsa_::Zalsa, id: zalsa_::Id, current_revision: zalsa_::Revision) -> zalsa_::MemoTableWithTypes<'_>
```

---

## CrateBuilder

`struct` · `ra_ap_base_db::input::CrateBuilder`

Also reachable as `ra_ap_base_db::CrateBuilder`

```rust
struct CrateBuilder
```

**Fields**: `basic`, `extra`, `cfg_options`, `env`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## CrateData

`struct` · `ra_ap_base_db::input::CrateData`

```rust
struct CrateData<Id>
```

**Fields**: `root_file_id`, `edition`, `dependencies`, `origin`, `crate_attrs`, `is_proc_macro`, `proc_macro_cwd`

---

## CrateDisplayName

`struct` · `ra_ap_base_db::input::CrateDisplayName`

Also reachable as `ra_ap_base_db::CrateDisplayName`

```rust
struct CrateDisplayName
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn canonical_name(&self) -> &Symbol
fn crate_name(&self) -> &CrateName
fn from_canonical_name(canonical_name: &str) -> CrateDisplayName
```

**via `core::convert::From`**

```rust
fn from(crate_name: CrateName) -> CrateDisplayName
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Symbol
```

---

## CrateGraphBuilder

`struct` · `ra_ap_base_db::input::CrateGraphBuilder`

Also reachable as `ra_ap_base_db::CrateGraphBuilder`, `ra_ap_ide::CrateGraphBuilder`

```rust
struct CrateGraphBuilder
```

**Implements**: `core::ops::index::Index`

**Derives**: Clone, Debug, Default

**Methods** (8)

```rust
fn add_crate_root(&mut self, root_file_id: FileId, edition: Edition, display_name: Option<CrateDisplayName>, version: Option<String>, cfg_options: CfgOptions, potential_cfg_options: Option<CfgOptions>, env: Env, origin: CrateOrigin, crate_attrs: Vec<String>, is_proc_macro: bool, proc_macro_cwd: Arc<AbsPathBuf>, ws_data: Arc<CrateWorkspaceData>) -> CrateBuilderId
fn add_dep(&mut self, from: CrateBuilderId, dep: DependencyBuilder) -> Result<(), CyclicDependenciesError>
fn extend(&mut self, other: CrateGraphBuilder, proc_macros: &mut ProcMacroPaths) -> FxHashMap<CrateBuilderId, CrateBuilderId>
fn iter(&self) -> impl Iterator<Item = CrateBuilderId> + '_
fn remove_crates_except(&mut self, to_keep: &[CrateBuilderId]) -> Vec<Option<CrateBuilderId>>
fn set_in_db(self, db: &mut dyn SourceDatabase) -> CratesIdMap
fn shrink_to_fit(&mut self)
fn transitive_deps(&self, of: CrateBuilderId) -> impl Iterator<Item = CrateBuilderId>
```

**via `core::ops::index::Index`**

```rust
fn index(&self, index: CrateBuilderId) -> &Self::Output
```

---

## CrateName

`struct` · `ra_ap_base_db::input::CrateName`

Also reachable as `ra_ap_base_db::CrateName`

```rust
struct CrateName
```

**Implements**: `core::fmt::Display`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn new(name: &str) -> Result<CrateName, &str>
fn normalize_dashes(name: &str) -> CrateName
fn symbol(&self) -> &Symbol
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Symbol
```

---

## CratesMap

`struct` · `ra_ap_base_db::input::CratesMap`

Also reachable as `ra_ap_base_db::CratesMap`

```rust
struct CratesMap
```

**Derives**: Debug, Default

The mapping from [`UniqueCrateData`] to their [`Crate`] input.

---

## CyclicDependenciesError

`struct` · `ra_ap_base_db::input::CyclicDependenciesError`

```rust
struct CyclicDependenciesError
```

The crate graph had a cycle. This is typically a bug, and
rust-analyzer logs a warning when it encounters a cycle. Generally
rust-analyzer will continue working OK in the presence of cycle,
but it's better to have an accurate crate graph.

## dev-dependencies

Note that it's actually legal for a Cargo package (i.e. a thing
with a `Cargo.toml`) to depend on itself in dev-dependencies. This
can enable additional features, and is typically used when a
project wants features to be enabled in tests. Dev-dependencies
are not propagated, so they aren't visible to package that depend
on this one.

<https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies>

However, rust-analyzer constructs its crate graph from Cargo
metadata, so it can end up producing a cyclic crate graph from a
well-formed package graph.

<https://github.com/rust-lang/rust-analyzer/issues/14167>

---

## Dependency

`struct` · `ra_ap_base_db::input::Dependency`

```rust
struct Dependency<Id>
```

**Fields**: `crate_id`, `name`

---

## Env

`struct` · `ra_ap_base_db::input::Env`

Also reachable as `ra_ap_base_db::Env`

```rust
struct Env
```

**Implements**: `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn contains_key(&self, arg: &str) -> bool
fn extend_from_other(&mut self, other: &Env)
fn get(&self, env: &str) -> Option<String>
fn insert(&mut self, k: impl Into<String>, v: impl Into<String>) -> Option<String>
fn is_empty(&self) -> bool
fn set(&mut self, env: &str, value: impl Into<String>)
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: IntoIterator<Item = (String, String)>>(&mut self, iter: T)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self
```

---

## ExtraCrateData

`struct` · `ra_ap_base_db::input::ExtraCrateData`

Also reachable as `ra_ap_base_db::ExtraCrateData`

```rust
struct ExtraCrateData
```

**Fields**: `version`, `display_name`, `potential_cfg_options`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

Crate data unrelated to analysis.

---

## SourceRoot

`struct` · `ra_ap_base_db::input::SourceRoot`

Also reachable as `ra_ap_base_db::SourceRoot`, `ra_ap_ide::SourceRoot`

```rust
struct SourceRoot
```

**Fields**: `is_library`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn file_for_path(&self, path: &VfsPath) -> Option<&FileId>
fn iter(&self) -> impl Iterator<Item = FileId> + '_
fn new_library(file_set: FileSet) -> SourceRoot
fn new_local(file_set: FileSet) -> SourceRoot
fn path_for_file(&self, file: &FileId) -> Option<&VfsPath>
fn resolve_path(&self, path: AnchoredPath<'_>) -> Option<FileId>
```

Files are grouped into source roots. A source root is a directory on the
file systems which is watched for changes. Typically it corresponds to a
Rust crate. Source roots *might* be nested: in this case, a file belongs to
the nearest enclosing source root. Paths to files are always relative to a
source root, and the analyzer does not know the root path of the source root at
all. So, a file from one source root can't refer to a file in another source
root by path.

---

## SourceRootId

`struct` · `ra_ap_base_db::input::SourceRootId`

Also reachable as `ra_ap_base_db::SourceRootId`, `ra_ap_ide::SourceRootId`

```rust
struct SourceRootId
```

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

---

## UniqueCrateData

`struct` · `ra_ap_base_db::input::UniqueCrateData`

Also reachable as `ra_ap_base_db::UniqueCrateData`

```rust
struct UniqueCrateData
```

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

The crate data from which we derive the `Crate`.

We want this to contain as little data as possible, because if it contains dependencies and
something changes, this crate and all of its dependencies ids are invalidated, which causes
pretty much everything to be recomputed. If the crate id is not invalidated, only this crate's
information needs to be recomputed.

*Most* different crates have different root files (actually, pretty much all of them).
Still, it is possible to have crates distinguished by other factors (e.g. dependencies).
So we store only the root file - unless we find that this crate has the same root file as
another crate, in which case we store all data for one of them (if one is a dependency of
the other, we store for it, because it has more dependencies to be invalidated).

---

## BuiltCrateData

`type_alias` · `ra_ap_base_db::input::BuiltCrateData`

Also reachable as `ra_ap_base_db::BuiltCrateData`

```rust
type BuiltCrateData = CrateData<Crate>
```

---

## BuiltDependency

`type_alias` · `ra_ap_base_db::input::BuiltDependency`

Also reachable as `ra_ap_base_db::BuiltDependency`

```rust
type BuiltDependency = Dependency<Crate>
```

---

## CrateBuilderId

`type_alias` · `ra_ap_base_db::input::CrateBuilderId`

Also reachable as `ra_ap_base_db::CrateBuilderId`

```rust
type CrateBuilderId = la_arena::Idx<CrateBuilder>
```

---

## CrateDataBuilder

`type_alias` · `ra_ap_base_db::input::CrateDataBuilder`

Also reachable as `ra_ap_base_db::CrateDataBuilder`

```rust
type CrateDataBuilder = CrateData<CrateBuilderId>
```

---

## CratesIdMap

`type_alias` · `ra_ap_base_db::input::CratesIdMap`

Also reachable as `ra_ap_base_db::CratesIdMap`

```rust
type CratesIdMap = rustc_hash::FxHashMap<CrateBuilderId, Crate>
```

---

## DependencyBuilder

`type_alias` · `ra_ap_base_db::input::DependencyBuilder`

Also reachable as `ra_ap_base_db::DependencyBuilder`

```rust
type DependencyBuilder = Dependency<CrateBuilderId>
```

---

## ProcMacroPaths

`type_alias` · `ra_ap_base_db::input::ProcMacroPaths`

Also reachable as `ra_ap_base_db::ProcMacroPaths`

```rust
type ProcMacroPaths = rustc_hash::FxHashMap<CrateBuilderId, Result<(String, vfs::AbsPathBuf), ProcMacroLoadingError>>
```

---
