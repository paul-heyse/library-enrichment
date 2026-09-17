# `ra_ap_base_db`

Crate `ra_ap_base_db` · 21 public items · structured records in [`model/ra_ap_base_db.json`](../model/ra_ap_base_db.json)

## DEFAULT_BORROWCK_LRU_CAP

`constant` · `ra_ap_base_db::DEFAULT_BORROWCK_LRU_CAP`

```rust
const DEFAULT_BORROWCK_LRU_CAP: u16 = 2024
```

---

## DEFAULT_FILE_TEXT_LRU_CAP

`constant` · `ra_ap_base_db::DEFAULT_FILE_TEXT_LRU_CAP`

```rust
const DEFAULT_FILE_TEXT_LRU_CAP: u16 = 16
```

---

## DEFAULT_PARSE_LRU_CAP

`constant` · `ra_ap_base_db::DEFAULT_PARSE_LRU_CAP`

```rust
const DEFAULT_PARSE_LRU_CAP: u16 = 128
```

---

## all_crates

`function` · `ra_ap_base_db::all_crates`

```rust
fn all_crates(db: &dyn salsa::Database) -> std::sync::Arc<[Crate]>
```

Returns the crates in topological order.

**Warning**: do not use this query in `hir-*` crates! It kills incrementality across crate metadata modifications.

---

## relevant_crates

`function` · `ra_ap_base_db::relevant_crates`

```rust
fn relevant_crates(db: &dyn SourceDatabase, file_id: FileId) -> &[Crate]
```

---

## set_all_crates_with_durability

`function` · `ra_ap_base_db::set_all_crates_with_durability`

```rust
fn set_all_crates_with_durability(db: &mut dyn salsa::Database, crates: impl IntoIterator<Item = Crate>, durability: salsa::Durability)
```

---

## source_root_crates

`function` · `ra_ap_base_db::source_root_crates`

```rust
fn source_root_crates(db: &dyn SourceDatabase, id: SourceRootId) -> &[Crate]
```

Crates whose root file is in `id`.

---

## toolchain_channel

`function` · `ra_ap_base_db::toolchain_channel`

```rust
fn toolchain_channel(db: &dyn salsa::Database, krate: Crate) -> Option<ReleaseChannel>
```

---

## impl_intern_key

`macro` · `ra_ap_base_db::impl_intern_key`

```rust
macro_rules! impl_intern_key
```

---

## CrateWorkspaceData

`struct` · `ra_ap_base_db::CrateWorkspaceData`

```rust
struct CrateWorkspaceData
```

**Fields**: `target`, `toolchain`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_atleast_187(&self) -> bool
```

Crate related data shared by the whole workspace.

---

## DbPanicContext

`struct` · `ra_ap_base_db::DbPanicContext`

```rust
struct DbPanicContext
```

**Implements**: `core::ops::drop::Drop`

**Methods** (1)

```rust
fn enter(frame: String) -> DbPanicContext
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

---

## FileSourceRootInput

`struct` · `ra_ap_base_db::FileSourceRootInput`

```rust
struct FileSourceRootInput
```

**Implements**: `salsa::id::AsId`, `salsa::id::FromId`, `salsa::input::Configuration`, `salsa::input::HasBuilder`, `salsa::salsa_struct::SalsaStructInDb`, `salsa::salsa_value::SalsaValue`, `salsa::zalsa::HasJar`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn builder(source_root_id: SourceRootId) -> <Self as zalsa_struct_::HasBuilder>::Builder
fn default_debug_fmt(this: Self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result where for<'__trivial_bounds> SourceRootId: ::std::fmt::Debug
fn ingredient(db: &dyn zalsa_::Database) -> &zalsa_struct_::IngredientImpl<Self>
fn ingredient_mut(zalsa_mut: &mut zalsa_::Zalsa) -> (&mut zalsa_struct_::IngredientImpl<Self>, &mut zalsa_::Runtime)
fn new<Db_>(db: &Db_, source_root_id: SourceRootId) -> Self where Db_: ?Sized + salsa::Database
fn set_source_root_id<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = SourceRootId> where Db_: ?Sized + zalsa_::Database
fn source_root_id<'db, Db_>(self, db: &'db Db_) -> SourceRootId where Db_: ?Sized + zalsa_::Database
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

## FileText

`struct` · `ra_ap_base_db::FileText`

```rust
struct FileText
```

**Implements**: `salsa::id::AsId`, `salsa::id::FromId`, `salsa::input::Configuration`, `salsa::input::HasBuilder`, `salsa::salsa_struct::SalsaStructInDb`, `salsa::salsa_value::SalsaValue`, `salsa::zalsa::HasJar`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn builder(text: Arc<str>, file_id: vfs::FileId) -> <Self as zalsa_struct_::HasBuilder>::Builder
fn default_debug_fmt(this: Self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result where for<'__trivial_bounds> Arc<str>: ::std::fmt::Debug, for<'__trivial_bounds> vfs::FileId: ::std::fmt::Debug
fn file_id<'db, Db_>(self, db: &'db Db_) -> &'db vfs::FileId where Db_: ?Sized + zalsa_::Database
fn ingredient(db: &dyn zalsa_::Database) -> &zalsa_struct_::IngredientImpl<Self>
fn ingredient_mut(zalsa_mut: &mut zalsa_::Zalsa) -> (&mut zalsa_struct_::IngredientImpl<Self>, &mut zalsa_::Runtime)
fn new<Db_>(db: &Db_, text: Arc<str>, file_id: vfs::FileId) -> Self where Db_: ?Sized + salsa::Database
fn set_file_id<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = vfs::FileId> where Db_: ?Sized + zalsa_::Database
fn set_text<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = Arc<str>> where Db_: ?Sized + zalsa_::Database
fn text<'db, Db_>(self, db: &'db Db_) -> &'db Arc<str> where Db_: ?Sized + zalsa_::Database
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

## Files

`struct` · `ra_ap_base_db::Files`

```rust
struct Files
```

**Derives**: Debug, Default

**Methods** (7)

```rust
fn file_source_root(&self, db: &dyn SourceDatabase, id: vfs::FileId) -> FileSourceRootInput
fn file_text(&self, file_id: vfs::FileId) -> FileText
fn set_file_source_root_with_durability(&self, db: &mut dyn SourceDatabase, id: vfs::FileId, source_root_id: SourceRootId, durability: Durability)
fn set_file_text(&self, db: &mut dyn SourceDatabase, file_id: vfs::FileId, text: &str)
fn set_file_text_with_durability(&self, db: &mut dyn SourceDatabase, file_id: vfs::FileId, text: &str, durability: Durability)
fn set_source_root_with_durability(&self, db: &mut dyn SourceDatabase, source_root_id: SourceRootId, source_root: Arc<SourceRoot>, durability: Durability)
fn source_root(&self, source_root_id: SourceRootId) -> SourceRootInput
```

---

## LibraryRoots

`struct` · `ra_ap_base_db::LibraryRoots`

```rust
struct LibraryRoots
```

**Implements**: `salsa::id::AsId`, `salsa::id::FromId`, `salsa::input::Configuration`, `salsa::input::HasBuilder`, `salsa::salsa_struct::SalsaStructInDb`, `salsa::salsa_value::SalsaValue`, `salsa::zalsa::HasJar`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn builder(roots: FxHashSet<SourceRootId>) -> <Self as zalsa_struct_::HasBuilder>::Builder
fn default_debug_fmt(this: Self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result where for<'__trivial_bounds> FxHashSet<SourceRootId>: ::std::fmt::Debug
fn get<Db_>(db: &Db_) -> Self where Db_: ?Sized + salsa::Database
fn ingredient(db: &dyn zalsa_::Database) -> &zalsa_struct_::IngredientImpl<Self>
fn ingredient_mut(zalsa_mut: &mut zalsa_::Zalsa) -> (&mut zalsa_struct_::IngredientImpl<Self>, &mut zalsa_::Runtime)
fn new<Db_>(db: &Db_, roots: FxHashSet<SourceRootId>) -> Self where Db_: ?Sized + salsa::Database
fn roots<'db, Db_>(self, db: &'db Db_) -> &'db FxHashSet<SourceRootId> where Db_: ?Sized + zalsa_::Database
fn set_roots<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = FxHashSet<SourceRootId>> where Db_: ?Sized + zalsa_::Database
fn try_get<Db_>(db: &Db_) -> Option<Self> where Db_: ?Sized + salsa::Database
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

The set of roots for crates.io libraries.
Files in libraries are assumed to never change.

---

## LocalRoots

`struct` · `ra_ap_base_db::LocalRoots`

```rust
struct LocalRoots
```

**Implements**: `salsa::id::AsId`, `salsa::id::FromId`, `salsa::input::Configuration`, `salsa::input::HasBuilder`, `salsa::salsa_struct::SalsaStructInDb`, `salsa::salsa_value::SalsaValue`, `salsa::zalsa::HasJar`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn builder(roots: FxHashSet<SourceRootId>) -> <Self as zalsa_struct_::HasBuilder>::Builder
fn default_debug_fmt(this: Self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result where for<'__trivial_bounds> FxHashSet<SourceRootId>: ::std::fmt::Debug
fn get<Db_>(db: &Db_) -> Self where Db_: ?Sized + salsa::Database
fn ingredient(db: &dyn zalsa_::Database) -> &zalsa_struct_::IngredientImpl<Self>
fn ingredient_mut(zalsa_mut: &mut zalsa_::Zalsa) -> (&mut zalsa_struct_::IngredientImpl<Self>, &mut zalsa_::Runtime)
fn new<Db_>(db: &Db_, roots: FxHashSet<SourceRootId>) -> Self where Db_: ?Sized + salsa::Database
fn roots<'db, Db_>(self, db: &'db Db_) -> &'db FxHashSet<SourceRootId> where Db_: ?Sized + zalsa_::Database
fn set_roots<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = FxHashSet<SourceRootId>> where Db_: ?Sized + zalsa_::Database
fn try_get<Db_>(db: &Db_) -> Option<Self> where Db_: ?Sized + salsa::Database
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

The set of "local" (that is, from the current workspace) roots.
Files in local roots are assumed to change frequently.

---

## Nonce

`struct` · `ra_ap_base_db::Nonce`

```rust
struct Nonce
```

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn invalid() -> Nonce
fn new() -> Nonce
```

---

## SourceRootInput

`struct` · `ra_ap_base_db::SourceRootInput`

```rust
struct SourceRootInput
```

**Implements**: `salsa::id::AsId`, `salsa::id::FromId`, `salsa::input::Configuration`, `salsa::input::HasBuilder`, `salsa::salsa_struct::SalsaStructInDb`, `salsa::salsa_value::SalsaValue`, `salsa::zalsa::HasJar`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn builder(source_root: Arc<SourceRoot>) -> <Self as zalsa_struct_::HasBuilder>::Builder
fn default_debug_fmt(this: Self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result where for<'__trivial_bounds> Arc<SourceRoot>: ::std::fmt::Debug
fn ingredient(db: &dyn zalsa_::Database) -> &zalsa_struct_::IngredientImpl<Self>
fn ingredient_mut(zalsa_mut: &mut zalsa_::Zalsa) -> (&mut zalsa_struct_::IngredientImpl<Self>, &mut zalsa_::Runtime)
fn new<Db_>(db: &Db_, source_root: Arc<SourceRoot>) -> Self where Db_: ?Sized + salsa::Database
fn set_source_root<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = Arc<SourceRoot>> where Db_: ?Sized + zalsa_::Database
fn source_root<'db, Db_>(self, db: &'db Db_) -> Arc<SourceRoot> where Db_: ?Sized + zalsa_::Database
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

## SourceDatabase

`trait` · `ra_ap_base_db::SourceDatabase`

```rust
trait SourceDatabase: salsa::Database + std::fmt::Debug
```

**Methods** (10)

```rust
fn file_source_root(&self, id: vfs::FileId) -> FileSourceRootInput
fn file_text(&self, file_id: vfs::FileId) -> FileText
fn line_column(&self, file: FileId, offset: TextSize) -> Result<(u32, u32), ()>
fn nonce_and_revision(&self) -> (Nonce, salsa::Revision)
fn resolve_path(&self, path: AnchoredPath<'_>) -> Option<FileId>
fn set_file_source_root_with_durability(&mut self, id: vfs::FileId, source_root_id: SourceRootId, durability: Durability)
fn set_file_text(&mut self, file_id: vfs::FileId, text: &str)
fn set_file_text_with_durability(&mut self, file_id: vfs::FileId, text: &str, durability: Durability)
fn set_source_root_with_durability(&mut self, source_root_id: SourceRootId, source_root: Arc<SourceRoot>, durability: Durability)
fn source_root(&self, id: SourceRootId) -> SourceRootInput
```

---

## FxIndexMap

`type_alias` · `ra_ap_base_db::FxIndexMap`

```rust
type FxIndexMap<K, V> = indexmap::IndexMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>
```

---

## FxIndexSet

`type_alias` · `ra_ap_base_db::FxIndexSet`

```rust
type FxIndexSet<T> = indexmap::IndexSet<T, rustc_hash::FxBuildHasher>
```

---
