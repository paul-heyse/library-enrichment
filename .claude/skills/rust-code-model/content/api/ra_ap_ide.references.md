# `ra_ap_ide::references`

Crate `ra_ap_ide` · 3 public items · structured records in [`model/ra_ap_ide.references.json`](../model/ra_ap_ide.references.json)

## Declaration

`struct` · `ra_ap_ide::references::Declaration`

```rust
struct Declaration
```

**Fields**: `nav`, `is_mut`

Information about the declaration site of a searched item.

---

## FindAllRefsConfig

`struct` · `ra_ap_ide::references::FindAllRefsConfig`

Also reachable as `ra_ap_ide::FindAllRefsConfig`

```rust
struct FindAllRefsConfig<'a>
```

**Fields**: `search_scope`, `ra_fixture`, `exclude_imports`, `exclude_tests`

**Derives**: Debug

---

## ReferenceSearchResult

`struct` · `ra_ap_ide::references::ReferenceSearchResult`

Also reachable as `ra_ap_ide::ReferenceSearchResult`

```rust
struct ReferenceSearchResult
```

**Fields**: `declaration`, `references`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Clone, Debug

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, __analysis: &::ide_db::ra_fixture::RaFixtureAnalysis, __virtual_file_id: ::ide_db::ra_fixture::FileId, __real_file_id: ::ide_db::ra_fixture::FileId) -> Result<Self, ()>
```

Result of a reference search operation.

---
