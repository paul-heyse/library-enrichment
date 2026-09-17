# `ra_ap_ide::static_index`

Crate `ra_ap_ide` · 7 public items · structured records in [`model/ra_ap_ide.static_index.json`](../model/ra_ap_ide.static_index.json)

## VendoredLibrariesConfig

`enum` · `ra_ap_ide::static_index::VendoredLibrariesConfig`

Also reachable as `ra_ap_ide::VendoredLibrariesConfig`

```rust
enum VendoredLibrariesConfig<'a>
```

**Variants**: `Included`, `Excluded`

---

## ReferenceData

`struct` · `ra_ap_ide::static_index::ReferenceData`

```rust
struct ReferenceData
```

**Fields**: `range`, `is_definition`

---

## StaticIndex

`struct` · `ra_ap_ide::static_index::StaticIndex`

Also reachable as `ra_ap_ide::StaticIndex`

```rust
struct StaticIndex<'a>
```

**Fields**: `files`, `tokens`

**Derives**: Debug

**Methods** (1)

```rust
fn compute(analysis: &'a Analysis, vendored_libs_config: VendoredLibrariesConfig<'_>) -> StaticIndex<'a>
```

A static representation of fully analyzed source code.

The intended use-case is powering read-only code browsers and emitting LSIF/SCIP.

---

## StaticIndexedFile

`struct` · `ra_ap_ide::static_index::StaticIndexedFile`

Also reachable as `ra_ap_ide::StaticIndexedFile`

```rust
struct StaticIndexedFile
```

**Fields**: `file_id`, `folds`, `tokens`

**Derives**: Debug

---

## TokenId

`struct` · `ra_ap_ide::static_index::TokenId`

Also reachable as `ra_ap_ide::TokenId`

```rust
struct TokenId
```

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn raw(self) -> usize
```

---

## TokenStaticData

`struct` · `ra_ap_ide::static_index::TokenStaticData`

Also reachable as `ra_ap_ide::TokenStaticData`

```rust
struct TokenStaticData
```

**Fields**: `documentation`, `hover`, `definition`, `definition_body`, `references`, `moniker`, `display_name`, `signature`, `kind`

**Derives**: Debug

---

## TokenStore

`struct` · `ra_ap_ide::static_index::TokenStore`

```rust
struct TokenStore
```

---
