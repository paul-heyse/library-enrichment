# `ra_ap_ide::hover`

Crate `ra_ap_ide` · 8 public items · structured records in [`model/ra_ap_ide.hover.json`](../model/ra_ap_ide.hover.json)

## HoverAction

`enum` · `ra_ap_ide::hover::HoverAction`

Also reachable as `ra_ap_ide::HoverAction`

```rust
enum HoverAction
```

**Variants**: `Runnable`, `Implementation`, `Reference`, `GoToType`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, __analysis: &::ide_db::ra_fixture::RaFixtureAnalysis, __virtual_file_id: ::ide_db::ra_fixture::FileId, __real_file_id: ::ide_db::ra_fixture::FileId) -> Result<Self, ()>
```

---

## HoverDocFormat

`enum` · `ra_ap_ide::hover::HoverDocFormat`

Also reachable as `ra_ap_ide::HoverDocFormat`

```rust
enum HoverDocFormat
```

**Variants**: `Markdown`, `PlainText`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## MemoryLayoutHoverRenderKind

`enum` · `ra_ap_ide::hover::MemoryLayoutHoverRenderKind`

Also reachable as `ra_ap_ide::MemoryLayoutHoverRenderKind`

```rust
enum MemoryLayoutHoverRenderKind
```

**Variants**: `Decimal`, `Hexadecimal`, `Both`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## SubstTyLen

`enum` · `ra_ap_ide::hover::SubstTyLen`

Also reachable as `ra_ap_ide::SubstTyLen`

```rust
enum SubstTyLen
```

**Variants**: `Unlimited`, `LimitTo`, `Hide`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## HoverConfig

`struct` · `ra_ap_ide::hover::HoverConfig`

Also reachable as `ra_ap_ide::HoverConfig`

```rust
struct HoverConfig<'a>
```

**Fields**: `links_in_hover`, `memory_layout`, `documentation`, `keywords`, `format`, `max_trait_assoc_items_count`, `max_fields_count`, `max_enum_variants_count`, `max_subst_ty_len`, `show_drop_glue`, `ra_fixture`

**Derives**: Clone, Debug

---

## HoverGotoTypeData

`struct` · `ra_ap_ide::hover::HoverGotoTypeData`

Also reachable as `ra_ap_ide::HoverGotoTypeData`

```rust
struct HoverGotoTypeData
```

**Fields**: `mod_path`, `nav`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, __analysis: &::ide_db::ra_fixture::RaFixtureAnalysis, __virtual_file_id: ::ide_db::ra_fixture::FileId, __real_file_id: ::ide_db::ra_fixture::FileId) -> Result<Self, ()>
```

---

## HoverResult

`struct` · `ra_ap_ide::hover::HoverResult`

Also reachable as `ra_ap_ide::HoverResult`

```rust
struct HoverResult
```

**Fields**: `markup`, `actions`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, __analysis: &::ide_db::ra_fixture::RaFixtureAnalysis, __virtual_file_id: ::ide_db::ra_fixture::FileId, __real_file_id: ::ide_db::ra_fixture::FileId) -> Result<Self, ()>
```

Contains the results when hovering over an item

---

## MemoryLayoutHoverConfig

`struct` · `ra_ap_ide::hover::MemoryLayoutHoverConfig`

Also reachable as `ra_ap_ide::MemoryLayoutHoverConfig`

```rust
struct MemoryLayoutHoverConfig
```

**Fields**: `size`, `offset`, `alignment`, `padding`, `niches`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---
