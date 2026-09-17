# `ra_ap_vfs::anchored_path`

Crate `ra_ap_vfs` · 2 public items · structured records in [`model/ra_ap_vfs.anchored_path.json`](../model/ra_ap_vfs.anchored_path.json)

## AnchoredPath

`struct` · `ra_ap_vfs::anchored_path::AnchoredPath`

Also reachable as `ra_ap_base_db::AnchoredPath`, `ra_ap_vfs::AnchoredPath`

```rust
struct AnchoredPath<'a>
```

**Fields**: `anchor`, `path`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Path relative to a file.

Borrowed version of [`AnchoredPathBuf`].

---

## AnchoredPathBuf

`struct` · `ra_ap_vfs::anchored_path::AnchoredPathBuf`

Also reachable as `ra_ap_base_db::AnchoredPathBuf`, `ra_ap_vfs::AnchoredPathBuf`

```rust
struct AnchoredPathBuf
```

**Fields**: `anchor`, `path`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

Path relative to a file.

Owned version of [`AnchoredPath`].

---
