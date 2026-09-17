# `ra_ap_span::hygiene`

Crate `ra_ap_span` · 2 public items · structured records in [`model/ra_ap_span.hygiene.json`](../model/ra_ap_span.hygiene.json)

## Transparency

`enum` · `ra_ap_span::hygiene::Transparency`

Also reachable as `ra_ap_span::Transparency`

```rust
enum Transparency
```

**Variants**: `Transparent`, `SemiOpaque`, `Opaque`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn is_opaque(&self) -> bool
```

A property of a macro expansion that determines how identifiers
produced by that expansion are resolved.

---

## SyntaxContext

`struct` · `ra_ap_span::hygiene::SyntaxContext`

Also reachable as `ra_ap_span::SyntaxContext`

```rust
struct SyntaxContext
```

**Implements**: `core::fmt::Display`, `salsa::id::AsId`, `salsa::id::FromId`, `salsa::interned::Configuration`, `salsa::salsa_struct::SalsaStructInDb`, `salsa::salsa_value::SalsaValue`, `salsa::zalsa::HasJar`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Send, StructuralPartialEq, Sync

**Methods** (20)

```rust
fn edition<Db>(self, db: &'db Db) -> Edition where Db: ?Sized + zalsa_::Database
const unsafe fn from_u32(u32: u32) -> Self
fn ingredient(zalsa: &zalsa_::Zalsa) -> &zalsa_struct_::IngredientImpl<Self>
const fn into_u32(self) -> u32
fn is_opaque(self, db: &'db dyn salsa::Database) -> bool
fn is_root(self) -> bool
fn marks(self, db: &'db dyn salsa::Database) -> impl Iterator<Item = (MacroCallId, Transparency)>
fn marks_rev(self, db: &'db dyn salsa::Database) -> impl Iterator<Item = (MacroCallId, Transparency)>
fn new<Db, T0: zalsa_::Lookup<Option<MacroCallId>> + std::hash::Hash, T1: zalsa_::Lookup<Transparency> + std::hash::Hash, T2: zalsa_::Lookup<Edition> + std::hash::Hash, T3: zalsa_::Lookup<SyntaxContext> + std::hash::Hash>(db: &'db Db, outer_expn: T0, outer_transparency: T1, edition: T2, parent: T3, opaque: impl FnOnce(SyntaxContext) -> SyntaxContext, opaque_and_semiopaque: impl FnOnce(SyntaxContext) -> SyntaxContext) -> Self where Db: ?Sized + salsa::Database, Option<MacroCallId>: zalsa_::HashEqLike<T0>, Transparency: zalsa_::HashEqLike<T1>, Edition: zalsa_::HashEqLike<T2>, SyntaxContext: zalsa_::HashEqLike<T3>
fn normalize_to_macro_rules(self, db: &'db dyn salsa::Database) -> SyntaxContext
fn normalize_to_macros_2_0(self, db: &'db dyn salsa::Database) -> SyntaxContext
fn opaque<Db>(self, db: &'db Db) -> SyntaxContext where Db: ?Sized + zalsa_::Database
fn opaque_and_semiopaque<Db>(self, db: &'db Db) -> SyntaxContext where Db: ?Sized + zalsa_::Database
fn outer_expn<Db>(self, db: &'db Db) -> Option<MacroCallId> where Db: ?Sized + zalsa_::Database
fn outer_mark(self, db: &'db dyn salsa::Database) -> (Option<MacroCallId>, Transparency)
fn outer_transparency<Db>(self, db: &'db Db) -> Transparency where Db: ?Sized + zalsa_::Database
fn parent<Db>(self, db: &'db Db) -> SyntaxContext where Db: ?Sized + zalsa_::Database
fn remove_mark(&mut self, db: &'db dyn salsa::Database) -> (Option<MacroCallId>, Transparency)
fn remove_root_edition(&mut self)
const fn root(edition: Edition) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `salsa::id::AsId`**

```rust
fn as_id(&self) -> salsa::Id
```

**via `salsa::id::FromId`**

```rust
fn from_id(id: salsa::Id) -> Self
```

**via `salsa::interned::Configuration`**

```rust
fn deserialize<'de, D>(_: D) -> Result<Self::Fields<'static>, D::Error> where D: zalsa_::serde::Deserializer<'de>
fn serialize<S>(_: &Self::Fields<'_>, _: S) -> Result<S::Ok, S::Error> where S: zalsa_::serde::Serializer
```

**via `salsa::salsa_struct::SalsaStructInDb`**

```rust
fn cast(id: salsa::Id, type_id: std::any::TypeId) -> Option<Self>
fn entries(zalsa: &zalsa_::Zalsa) -> impl Iterator<Item = zalsa_::DatabaseKeyIndex> + '_
fn lookup_ingredient_index(aux: &zalsa_::Zalsa) -> salsa::plumbing::IngredientIndices
unsafe fn memo_table(zalsa: &zalsa_::Zalsa, id: zalsa_::Id, current_revision: zalsa_::Revision) -> zalsa_::MemoTableWithTypes<'_>
```

A syntax context describes a hierarchy tracking order of macro definitions.

---
