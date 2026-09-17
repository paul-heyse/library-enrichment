# `ra_ap_base_db::target`

Crate `ra_ap_base_db` · 4 public items · structured records in [`model/ra_ap_base_db.target.json`](../model/ra_ap_base_db.target.json)

## Arch

`enum` · `ra_ap_base_db::target::Arch`

```rust
enum Arch
```

**Variants**: `Wasm32`, `Wasm64`, `Other`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## TargetData

`struct` · `ra_ap_base_db::target::TargetData`

```rust
struct TargetData
```

**Fields**: `data_layout`, `arch`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## TargetLoadError

`struct` · `ra_ap_base_db::target::TargetLoadError`

```rust
struct TargetLoadError
```

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: &str) -> Self
fn from(value: String) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## TargetLoadResult

`type_alias` · `ra_ap_base_db::target::TargetLoadResult`

```rust
type TargetLoadResult = Result<TargetData, TargetLoadError>
```

---
