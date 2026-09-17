# `ra_ap_ide::file_structure`

Crate `ra_ap_ide` · 3 public items · structured records in [`model/ra_ap_ide.file_structure.json`](../model/ra_ap_ide.file_structure.json)

## StructureNodeKind

`enum` · `ra_ap_ide::file_structure::StructureNodeKind`

Also reachable as `ra_ap_ide::StructureNodeKind`

```rust
enum StructureNodeKind
```

**Variants**: `SymbolKind`, `ExternBlock`, `Region`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

---

## FileStructureConfig

`struct` · `ra_ap_ide::file_structure::FileStructureConfig`

Also reachable as `ra_ap_ide::FileStructureConfig`

```rust
struct FileStructureConfig
```

**Fields**: `exclude_locals`

**Derives**: Clone, Debug

---

## StructureNode

`struct` · `ra_ap_ide::file_structure::StructureNode`

Also reachable as `ra_ap_ide::StructureNode`

```rust
struct StructureNode
```

**Fields**: `parent`, `label`, `navigation_range`, `node_range`, `kind`, `detail`, `deprecated`

**Derives**: Clone, Debug

---
