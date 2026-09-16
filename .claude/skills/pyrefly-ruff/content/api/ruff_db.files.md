# `ruff_db::files`

Crate `ruff_db` · 7 public items · structured records in [`model/ruff_db.files.json`](../model/ruff_db.files.json)

## FileError

`enum` · `ruff_db::files::FileError`

```rust
enum FileError
```

**Variants**: `IsADirectory`, `NotFound`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result
```

---

## system_path_to_file

`function` · `ruff_db::files::system_path_to_file`

```rust
fn system_path_to_file(db: &dyn Db, path: impl AsRef<system::SystemPath>) -> Result<File, FileError>
```

Interns a file system path and returns a salsa `File` ingredient.

Returns `Err` if the path doesn't exist, isn't accessible, or if the path points to a directory.

---

## vendored_path_to_file

`function` · `ruff_db::files::vendored_path_to_file`

```rust
fn vendored_path_to_file(db: &dyn Db, path: impl AsRef<vendored::VendoredPath>) -> Result<File, FileError>
```

Interns a vendored file path. Returns `Some` if the vendored file for `path` exists and `None` otherwise.

---

## File

`struct` · `ruff_db::files::File`

```rust
struct File
```

**Implements**: `get_size2::GetSize`, `salsa::id::AsId`, `salsa::id::FromId`, `salsa::input::Configuration`, `salsa::input::HasBuilder`, `salsa::salsa_struct::SalsaStructInDb`, `salsa::salsa_value::SalsaValue`, `salsa::zalsa::HasJar`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (23)

```rust
fn builder(path: FilePath) -> <Self as zalsa_struct_::HasBuilder>::Builder
fn default_debug_fmt(this: Self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result where for<'__trivial_bounds> FilePath: ::std::fmt::Debug, for<'__trivial_bounds> Option<u32>: ::std::fmt::Debug, for<'__trivial_bounds> FileRevision: ::std::fmt::Debug, for<'__trivial_bounds> FileStatus: ::std::fmt::Debug, for<'__trivial_bounds> Option<SourceText>: ::std::fmt::Debug
fn exists(self, db: &dyn Db) -> bool
fn ingredient(db: &dyn zalsa_::Database) -> &zalsa_struct_::IngredientImpl<Self>
fn ingredient_mut(zalsa_mut: &mut zalsa_::Zalsa) -> (&mut zalsa_struct_::IngredientImpl<Self>, &mut zalsa_::Runtime)
fn is_package(self, db: &dyn Db) -> bool
fn is_stub(self, db: &dyn Db) -> bool
fn new<Db_>(db: &Db_, path: FilePath) -> Self where Db_: ?Sized + salsa::Database
fn path<'db, Db_>(self, db: &'db Db_) -> &'db FilePath where Db_: ?Sized + zalsa_::Database
fn permissions<'db, Db_>(self, db: &'db Db_) -> Option<u32> where Db_: ?Sized + zalsa_::Database
fn revision<'db, Db_>(self, db: &'db Db_) -> FileRevision where Db_: ?Sized + zalsa_::Database
fn set_path<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = FilePath> where Db_: ?Sized + zalsa_::Database
fn set_permissions<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = Option<u32>> where Db_: ?Sized + zalsa_::Database
fn set_revision<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = FileRevision> where Db_: ?Sized + zalsa_::Database
fn set_source_text_override<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = Option<SourceText>> where Db_: ?Sized + zalsa_::Database
fn set_status<'db, Db_>(self, db: &'db mut Db_) -> impl salsa::Setter<FieldTy = FileStatus> where Db_: ?Sized + zalsa_::Database
fn source_text_override<'db, Db_>(self, db: &'db Db_) -> &'db Option<SourceText> where Db_: ?Sized + zalsa_::Database
fn source_type(self, db: &dyn Db) -> PySourceType
fn status<'db, Db_>(self, db: &'db Db_) -> FileStatus where Db_: ?Sized + zalsa_::Database
fn sync(self, db: &mut dyn Db)
fn sync_path(db: &mut dyn Db, path: &SystemPath)
fn sync_path_only(db: &mut dyn Db, path: &SystemPath)
fn sync_virtual_path(db: &mut dyn Db, path: &SystemVirtualPath)
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

A file-system path that's either stored on the host system's file system or in the vendored file system.

# Ordering
Ordering is based on the file's salsa-assigned id and not on its values.
The id may change between runs.

---

## FileRange

`struct` · `ruff_db::files::FileRange`

```rust
struct FileRange
```

**Implements**: `core::convert::TryFrom`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn cover_range(self, range: TextRange) -> Self
const fn file(&self) -> File
const fn new(file: File, range: TextRange) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &Span) -> Result<Self, Self::Error>
fn try_from(value: Span) -> Result<Self, Self::Error>
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

Range with its corresponding file.

---

## Files

`struct` · `ruff_db::files::Files`

```rust
struct Files
```

**Implements**: `core::panic::unwind_safe::RefUnwindSafe`

**Derives**: Clone, Debug, Default

**Methods** (8)

```rust
fn freeze(&self)
fn root(&self, db: &dyn Db, path: &SystemPath) -> Option<FileRoot>
fn sync_all(db: &mut dyn Db)
fn sync_all_recursive<P, I>(db: &mut dyn Db, paths: I) where P: AsRef<SystemPath>, I: IntoIterator<Item = P>
fn try_add_root(&self, db: &dyn Db, path: &SystemPath, kind: FileRootKind) -> FileRoot
fn try_system(&self, db: &dyn Db, path: &SystemPath) -> Option<File>
fn try_virtual_file(&self, path: &SystemVirtualPath) -> Option<VirtualFile>
fn virtual_file(&self, db: &dyn Db, path: &SystemVirtualPath) -> VirtualFile
```

Lookup table that maps [file paths](`FilePath`) to salsa interned [`File`] instances.

---

## VirtualFile

`struct` · `ruff_db::files::VirtualFile`

```rust
struct VirtualFile
```

**Derives**: Clone, Copy, Debug

**Methods** (3)

```rust
fn close(&self, db: &mut dyn Db)
fn file(&self) -> File
fn sync(&self, db: &mut dyn Db)
```

A virtual file that doesn't exist on the file system.

This is a wrapper around a [`File`] that provides additional methods to interact with a virtual
file.

---
