# `ra_ap_ide::markup`

Crate `ra_ap_ide` · 1 public items · structured records in [`model/ra_ap_ide.markup.json`](../model/ra_ap_ide.markup.json)

## Markup

`struct` · `ra_ap_ide::markup::Markup`

Also reachable as `ra_ap_ide::Markup`

```rust
struct Markup
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn as_str(&self) -> &str
fn fenced_block(contents: impl fmt::Display) -> Markup
fn fenced_block_text(contents: impl fmt::Display) -> Markup
```

**via `core::convert::From`**

```rust
fn from(text: String) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, _analysis: &ra_fixture::RaFixtureAnalysis, _virtual_file_id: ra_fixture::FileId, _real_file_id: ra_fixture::FileId) -> Result<Self, ()>
```

---
