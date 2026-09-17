# `ra_ap_base_db::editioned_file_id`

Crate `ra_ap_base_db` · 1 public items · structured records in [`model/ra_ap_base_db.editioned_file_id.json`](../model/ra_ap_base_db.editioned_file_id.json)

## EditionedFileId

`struct` · `ra_ap_base_db::editioned_file_id::EditionedFileId`

Also reachable as `ra_ap_base_db::EditionedFileId`, `ra_ap_hir::EditionedFileId`

```rust
struct EditionedFileId
```

**Implements**: `salsa::id::AsId`, `salsa::id::FromId`, `salsa::interned::Configuration`, `salsa::salsa_struct::SalsaStructInDb`, `salsa::salsa_value::SalsaValue`, `salsa::zalsa::HasJar`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Send, StructuralPartialEq, Sync

**Methods** (11)

```rust
fn current_edition(db: &dyn Database, file_id: FileId) -> Self
fn default_debug_fmt(this: Self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result where for<'db> span::EditionedFileId: ::std::fmt::Debug
fn edition(self, db: &dyn Database) -> Edition
fn file_id(self, db: &dyn Database) -> vfs::FileId
fn from_span_file_id<Db_, T0: zalsa_::Lookup<span::EditionedFileId> + ::std::hash::Hash>(db: &'db Db_, field: T0) -> Self where Db_: ?Sized + ::salsa::Database, span::EditionedFileId: zalsa_::HashEqLike<T0>
fn ingredient(zalsa: &zalsa_::Zalsa) -> &zalsa_struct_::IngredientImpl<Self>
fn new(db: &dyn Database, file_id: FileId, edition: Edition) -> Self
fn parse(self, db: &dyn SourceDatabase) -> syntax::Parse<ast::SourceFile>
fn parse_errors<'db>(self, db: &'db dyn SourceDatabase) -> <Option<Box<[SyntaxError]>> as ::salsa::SalsaAsDeref>::AsDeref<'db>
fn span_file_id(self, db: &dyn Database) -> span::EditionedFileId
fn unpack(self, db: &dyn Database) -> (vfs::FileId, span::Edition)
```

**via `salsa::id::AsId`**

```rust
fn as_id(&self) -> ::salsa::Id
```

**via `salsa::id::FromId`**

```rust
fn from_id(id: ::salsa::Id) -> Self
```

**via `salsa::interned::Configuration`**

```rust
fn deserialize<'de, D: zalsa_::serde::Deserializer<'de>>(deserializer: D) -> ::std::result::Result<Self::Fields<'static>, D::Error>
fn serialize<S: zalsa_::serde::Serializer>(fields: &Self::Fields<'_>, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
```

**via `salsa::salsa_struct::SalsaStructInDb`**

```rust
fn cast(id: zalsa_::Id, type_id: zalsa_::TypeId) -> zalsa_::Option<Self>
fn entries(zalsa: &zalsa_::Zalsa) -> impl Iterator<Item = zalsa_::DatabaseKeyIndex> + '_
fn lookup_ingredient_index(aux: &zalsa_::Zalsa) -> zalsa_::IngredientIndices
unsafe fn memo_table(zalsa: &zalsa_::Zalsa, id: zalsa_::Id, current_revision: zalsa_::Revision) -> zalsa_::MemoTableWithTypes<'_>
```

---
