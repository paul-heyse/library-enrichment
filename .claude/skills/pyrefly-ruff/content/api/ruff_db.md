# `ruff_db`

Crate `ruff_db` · 9 public items · structured records in [`model/ruff_db.json`](../model/ruff_db.json)

## STACK_SIZE

`constant` · `ruff_db::STACK_SIZE`

```rust
const STACK_SIZE: usize = _
```

---

## disable_lru

`function` · `ruff_db::disable_lru`

```rust
fn disable_lru(db: &mut dyn Db)
```

Disables LRU bookkeeping for all queries defined by this crate.

This is useful for short-lived database users that don't need to evict query results across
revisions.

---

## max_parallelism

`function` · `ruff_db::max_parallelism`

```rust
fn max_parallelism() -> std::num::NonZeroUsize
```

Returns the maximum number of tasks that ty is allowed
to process in parallel.

Returns [`std::thread::available_parallelism`], unless the environment
variable `TY_MAX_PARALLELISM` or `RAYON_NUM_THREADS` is set. `TY_MAX_PARALLELISM` takes
precedence over `RAYON_NUM_THREADS`.

Falls back to `1` if `available_parallelism` is not available.

Setting `TY_MAX_PARALLELISM` to `2` only restricts the number of threads that ty spawns
to process work in parallel. For example, to index a directory or checking the files of a project.
ty can still spawn more threads for other tasks, e.g. to wait for a Ctrl+C signal or
watching the files for changes.

---

## program_version

`function` · `ruff_db::program_version`

```rust
fn program_version() -> Option<&'static str>
```

Returns the version of the executing program if set.

---

## set_program_version

`function` · `ruff_db::set_program_version`

```rust
fn set_program_version(version: String) -> Result<(), String>
```

Sets the version of the executing program.

## Errors
If the version has already been initialized (can only be set once).

---

## PythonFile

`struct` · `ruff_db::PythonFile`

```rust
struct PythonFile<'db>
```

**Implements**: `get_size2::GetSize`, `salsa::id::AsId`, `salsa::id::FromId`, `salsa::interned::Configuration`, `salsa::salsa_struct::SalsaStructInDb`, `salsa::salsa_value::SalsaValue`, `salsa::zalsa::HasJar`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, Send, StructuralPartialEq, Sync

**Methods** (5)

```rust
fn default_debug_fmt(this: Self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result where for<'db> File: ::std::fmt::Debug, for<'db> PythonVersion: ::std::fmt::Debug
fn file<Db_>(self, db: &'db Db_) -> File where Db_: ?Sized + zalsa_::Database
fn ingredient(zalsa: &zalsa_::Zalsa) -> &zalsa_struct_::IngredientImpl<Self>
fn new<Db_, T0: zalsa_::Lookup<File> + ::std::hash::Hash, T1: zalsa_::Lookup<PythonVersion> + ::std::hash::Hash>(db: &'db Db_, file: T0, python_version: T1) -> Self where Db_: ?Sized + ::salsa::Database, File: zalsa_::HashEqLike<T0>, PythonVersion: zalsa_::HashEqLike<T1>
fn python_version<Db_>(self, db: &'db Db_) -> PythonVersion where Db_: ?Sized + zalsa_::Database
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
fn heap_size(value: &Self::Fields<'_>) -> Option<usize>
fn serialize<S: zalsa_::serde::Serializer>(fields: &Self::Fields<'_>, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
```

**via `salsa::salsa_struct::SalsaStructInDb`**

```rust
fn cast(id: zalsa_::Id, type_id: zalsa_::TypeId) -> zalsa_::Option<Self>
fn entries(zalsa: &zalsa_::Zalsa) -> impl Iterator<Item = zalsa_::DatabaseKeyIndex> + '_
fn lookup_ingredient_index(aux: &zalsa_::Zalsa) -> zalsa_::IngredientIndices
unsafe fn memo_table(zalsa: &zalsa_::Zalsa, id: zalsa_::Id, current_revision: zalsa_::Revision) -> zalsa_::MemoTableWithTypes<'_>
```

A file paired with the Python version used to parse its contents.

This is the key for [`parsed::parsed_module`]. Including the Python version allows the same
file to be parsed for different versions within a single Salsa revision without sharing an
incompatible AST or syntax diagnostics.

---

## Db

`trait` · `ruff_db::Db`

```rust
trait Db: salsa::Database
```

**Implementors** (1)

- `ruff_graph::db::ModuleDb`

**Methods** (3)

```rust
fn files(&self) -> &Files
fn system(&self) -> &dyn System
fn vendored(&self) -> &VendoredFileSystem
```

Most basic database that gives access to files, the host system, source code, and parsed AST.

---

## RustDoc

`trait` · `ruff_db::RustDoc`

```rust
trait RustDoc
```

**Methods** (1)

```rust
fn rust_doc() -> &'static str
```

Trait for types that can provide Rust documentation.

Use `derive(RustDoc)` to automatically implement this trait for types that have a static string documentation.

---

## FxDashMap

`type_alias` · `ruff_db::FxDashMap`

```rust
type FxDashMap<K, V> = dashmap::DashMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>
```

---
