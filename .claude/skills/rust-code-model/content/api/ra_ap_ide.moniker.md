# `ra_ap_ide::moniker`

Crate `ra_ap_ide` · 8 public items · structured records in [`model/ra_ap_ide.moniker.json`](../model/ra_ap_ide.moniker.json)

## MonikerDescriptorKind

`enum` · `ra_ap_ide::moniker::MonikerDescriptorKind`

Also reachable as `ra_ap_ide::MonikerDescriptorKind`

```rust
enum MonikerDescriptorKind
```

**Variants**: `Namespace`, `Type`, `Term`, `Method`, `TypeParameter`, `Parameter`, `Macro`, `Meta`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: SymbolInformationKind) -> Self
```

---

## MonikerKind

`enum` · `ra_ap_ide::moniker::MonikerKind`

Also reachable as `ra_ap_ide::MonikerKind`

```rust
enum MonikerKind
```

**Variants**: `Import`, `Export`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

---

## MonikerResult

`enum` · `ra_ap_ide::moniker::MonikerResult`

Also reachable as `ra_ap_ide::MonikerResult`

```rust
enum MonikerResult
```

**Variants**: `Moniker`, `Local`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn from_def(db: &RootDatabase, def: Definition<'_>, from_crate: Crate) -> Option<Self>
```

---

## SymbolInformationKind

`enum` · `ra_ap_ide::moniker::SymbolInformationKind`

Also reachable as `ra_ap_ide::SymbolInformationKind`

```rust
enum SymbolInformationKind
```

**Variants**: `AssociatedType`, `Attribute`, `Constant`, `Enum`, `EnumMember`, `Field`, `Function`, `Macro`, `Method`, `Module`, `Parameter`, `SelfParameter`, `StaticMethod`, `StaticVariable`, `Struct`, `Trait`, `TraitMethod`, `Type`, `TypeAlias`, `TypeParameter`, `Union`, `Variable`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

---

## Moniker

`struct` · `ra_ap_ide::moniker::Moniker`

Also reachable as `ra_ap_ide::Moniker`

```rust
struct Moniker
```

**Fields**: `identifier`, `kind`, `package_information`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

Information which uniquely identifies a definition which might be referenceable outside of the
source file. Visibility declarations do not affect presence.

---

## MonikerDescriptor

`struct` · `ra_ap_ide::moniker::MonikerDescriptor`

```rust
struct MonikerDescriptor
```

**Fields**: `name`, `desc`

---

## MonikerIdentifier

`struct` · `ra_ap_ide::moniker::MonikerIdentifier`

Also reachable as `ra_ap_ide::MonikerIdentifier`

```rust
struct MonikerIdentifier
```

**Fields**: `crate_name`, `description`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## PackageInformation

`struct` · `ra_ap_ide::moniker::PackageInformation`

Also reachable as `ra_ap_ide::PackageInformation`

```rust
struct PackageInformation
```

**Fields**: `name`, `repo`, `version`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---
