# `ra_ap_base_db::change`

Crate `ra_ap_base_db` · 1 public items · structured records in [`model/ra_ap_base_db.change.json`](../model/ra_ap_base_db.change.json)

## FileChange

`struct` · `ra_ap_base_db::change::FileChange`

Also reachable as `ra_ap_base_db::FileChange`, `ra_ap_ide::FileChange`

```rust
struct FileChange
```

**Fields**: `roots`, `files_changed`, `crate_graph`

**Derives**: Debug, Default

**Methods** (4)

```rust
fn apply(self, db: &mut dyn SourceDatabase) -> Option<CratesIdMap>
fn change_file(&mut self, file_id: FileId, new_text: Option<String>)
fn set_crate_graph(&mut self, graph: CrateGraphBuilder)
fn set_roots(&mut self, roots: Vec<SourceRoot>)
```

Encapsulate a bunch of raw `.set` calls on the database.

---
