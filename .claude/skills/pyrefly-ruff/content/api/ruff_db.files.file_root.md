# `ruff_db::files::file_root`

Crate `ruff_db` · 2 public items · structured records in [`model/ruff_db.files.file_root.json`](../model/ruff_db.files.file_root.json)

## FileRootKind

`enum` · `ruff_db::files::file_root::FileRootKind`

Also reachable as `ruff_db::files::FileRootKind`

```rust
enum FileRootKind
```

**Variants**: `Project`, `SearchPath`

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---

## FileRoot

`struct` · `ruff_db::files::file_root::FileRoot`

Also reachable as `ruff_db::files::FileRoot`

```rust
struct FileRoot
```

**Implements**: `salsa::id::AsId`, `salsa::id::FromId`, `salsa::input::Configuration`, `salsa::input::HasBuilder`, `salsa::salsa_struct::SalsaStructInDb`, `salsa::salsa_value::SalsaValue`, `salsa::zalsa::HasJar`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn builder(path: Box<SystemPath>, kind_at_time_of_creation: FileRootKind) -> <Self as zalsa_struct_::HasBuilder>::Builder
fn default_debug_fmt(this: Self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result where for<'__trivial_bounds> Box<SystemPath>: ::std::fmt::Debug, for<'__trivial_bounds> FileRootKind: ::std::fmt::Debug
fn ingredient(db: &dyn zalsa_::Database) -> &zalsa_struct_::IngredientImpl<Self>
fn ingredient_mut(zalsa_mut: &mut zalsa_::Zalsa) -> (&mut zalsa_struct_::IngredientImpl<Self>, &mut zalsa_::Runtime)
fn kind_at_time_of_creation<'db, Db_>(self, db: &'db Db_) -> FileRootKind where Db_: ?Sized + zalsa_::Database
fn new<Db_>(db: &Db_, path: Box<SystemPath>, kind_at_time_of_creation: FileRootKind) -> Self where Db_: ?Sized + salsa::Database
fn path<'db, Db_>(self, db: &'db Db_) -> &'db <Box<SystemPath> as ::core::ops::Deref>::Target where Db_: ?Sized + zalsa_::Database
fn set_kind_at_time_of_creation<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = FileRootKind> where Db_: ?Sized + zalsa_::Database
fn set_path<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = Box<SystemPath>> where Db_: ?Sized + zalsa_::Database
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
fn heap_size(value: &Self::Fields) -> Option<usize>
fn serialize<S: zalsa_::serde::Serializer>(fields: &Self::Fields, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
```

**via `salsa::salsa_struct::SalsaStructInDb`**

```rust
fn cast(id: zalsa_::Id, type_id: zalsa_::TypeId) -> zalsa_::Option<Self>
fn entries(zalsa: &zalsa_::Zalsa) -> impl Iterator<Item = zalsa_::DatabaseKeyIndex> + '_
fn lookup_ingredient_index(aux: &zalsa_::Zalsa) -> zalsa_::IngredientIndices
unsafe fn memo_table(zalsa: &zalsa_::Zalsa, id: zalsa_::Id, current_revision: zalsa_::Revision) -> zalsa_::MemoTableWithTypes<'_>
```

A root path for files tracked by the database.

We currently create roots for:
* static module resolution paths
* the project root

File roots determine the durability of files and directories.

---
