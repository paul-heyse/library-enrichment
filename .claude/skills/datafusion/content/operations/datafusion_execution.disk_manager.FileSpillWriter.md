# `datafusion_execution::disk_manager::FileSpillWriter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.disk_manager.FileSpillWriter.json).

<a id="op-8a110f67567284edecd498d5"></a>
## FileSpillWriter

`struct` · `datafusion_execution::disk_manager::FileSpillWriter` · datafusion-execution 55.1.0

```rust
struct FileSpillWriter
```

Source: `src/disk_manager.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f88800b47dfa68d16baf6b7a"></a>
## finish

`function` · `datafusion_execution::disk_manager::FileSpillWriter::finish` · datafusion-execution 55.1.0

```rust
fn finish(&mut self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::FileSpillWriter", "path": "FileSpillWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [518, 1], "end": [523, 2], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "datafusion_execution::spill_file::SpillWriter", "path": "SpillWriter"}, "trait_path": "datafusion_execution::spill_file::SpillWriter"}`

Source: `src/disk_manager.rs:519`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf27a46cddca3e86db24ad3a"></a>
## flush

`function` · `datafusion_execution::disk_manager::FileSpillWriter::flush` · datafusion-execution 55.1.0

```rust
fn flush(&mut self) -> std::io::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::FileSpillWriter", "path": "FileSpillWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 1], "end": [516, 2], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}, "trait_path": "core::io::write::Write"}`

Source: `src/disk_manager.rs:513`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e36e604e976aeed1f213882d"></a>
## write

`function` · `datafusion_execution::disk_manager::FileSpillWriter::write` · datafusion-execution 55.1.0

```rust
fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::FileSpillWriter", "path": "FileSpillWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 1], "end": [516, 2], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}, "trait_path": "core::io::write::Write"}`

Source: `src/disk_manager.rs:479`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
