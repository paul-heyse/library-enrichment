# `ra_ap_ide::runnables`

Crate `ra_ap_ide` · 4 public items · structured records in [`model/ra_ap_ide.runnables.json`](../model/ra_ap_ide.runnables.json)

## RunnableKind

`enum` · `ra_ap_ide::runnables::RunnableKind`

Also reachable as `ra_ap_ide::RunnableKind`

```rust
enum RunnableKind
```

**Variants**: `TestMod`, `Test`, `Bench`, `DocTest`, `Bin`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, _analysis: &ra_fixture::RaFixtureAnalysis, _virtual_file_id: ra_fixture::FileId, _real_file_id: ra_fixture::FileId) -> Result<Self, ()>
```

---

## TestId

`enum` · `ra_ap_ide::runnables::TestId`

Also reachable as `ra_ap_ide::TestId`

```rust
enum TestId
```

**Variants**: `Name`, `Path`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## Runnable

`struct` · `ra_ap_ide::runnables::Runnable`

Also reachable as `ra_ap_ide::Runnable`

```rust
struct Runnable
```

**Fields**: `use_name_in_title`, `nav`, `kind`, `cfg`, `update_test`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn label(&self, target: Option<&str>) -> String
fn title(&self) -> String
```

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, __analysis: &::ide_db::ra_fixture::RaFixtureAnalysis, __virtual_file_id: ::ide_db::ra_fixture::FileId, __real_file_id: ::ide_db::ra_fixture::FileId) -> Result<Self, ()>
```

---

## UpdateTest

`struct` · `ra_ap_ide::runnables::UpdateTest`

Also reachable as `ra_ap_ide::UpdateTest`

```rust
struct UpdateTest
```

**Fields**: `expect_test`, `insta`, `snapbox`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn env(&self) -> ArrayVec<(&str, &str), 3>
fn label(&self) -> Option<SmolStr>
```

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, _analysis: &ra_fixture::RaFixtureAnalysis, _virtual_file_id: ra_fixture::FileId, _real_file_id: ra_fixture::FileId) -> Result<Self, ()>
```

---
