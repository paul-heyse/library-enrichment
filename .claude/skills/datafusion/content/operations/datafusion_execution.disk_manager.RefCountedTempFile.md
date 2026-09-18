# `datafusion_execution::disk_manager::RefCountedTempFile`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.disk_manager.RefCountedTempFile.json).

<a id="op-2ea0e5a71045567d23f762a7"></a>
## RefCountedTempFile

`struct` · `datafusion_execution::disk_manager::RefCountedTempFile` · datafusion-execution 55.1.0

```rust
struct RefCountedTempFile
```

Source: `src/disk_manager.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A wrapper around a [`NamedTempFile`] that also contains
a reference to its parent temporary directory.

This type is Clone-able, allowing multiple references to the same underlying file.
The file is deleted only when the last reference is dropped.

The parent temporary directory is also kept alive as long as any reference to
this file exists, preventing premature cleanup of the directory.

Once all references to this file are dropped, the file is deleted, and the
disk usage is subtracted from the disk manager's total.

Unresolved upstream links (retained, not inferred): ``NamedTempFile``.

<a id="op-6120faa86b0eab3e9132e9b5"></a>
## clone

`function` · `datafusion_execution::disk_manager::RefCountedTempFile::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::RefCountedTempFile", "path": "RefCountedTempFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [412, 1], "end": [421, 2], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/disk_manager.rs:413`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81e3f3d09aca8651c137d617"></a>
## drop

`function` · `datafusion_execution::disk_manager::RefCountedTempFile::drop` · datafusion-execution 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::RefCountedTempFile", "path": "RefCountedTempFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [438, 1], "end": [453, 2], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/disk_manager.rs:439`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ff7d7cb0aef71abaf5b939e"></a>
## fmt

`function` · `datafusion_execution::disk_manager::RefCountedTempFile::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::RefCountedTempFile", "path": "RefCountedTempFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [396, 10], "end": [396, 15], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/disk_manager.rs:396`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5c0c21c2425fe1149e94f6b"></a>
## inner

`function` · `datafusion_execution::disk_manager::RefCountedTempFile::inner` · datafusion-execution 55.1.0

```rust
fn inner(&self) -> &NamedTempFile
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::RefCountedTempFile", "path": "RefCountedTempFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [423, 1], "end": [435, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:428`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcd0d541a567a715f31ad082"></a>
## open_writer

`function` · `datafusion_execution::disk_manager::RefCountedTempFile::open_writer` · datafusion-execution 55.1.0

```rust
fn open_writer(&self) -> Result<Box<dyn SpillWriter>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::RefCountedTempFile", "path": "RefCountedTempFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [525, 1], "end": [587, 2], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "datafusion_execution::spill_file::SpillFile", "path": "SpillFile"}, "trait_path": "datafusion_execution::spill_file::SpillFile"}`

Source: `src/disk_manager.rs:575`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24173a5ba0e2d35b235d6f13"></a>
## path

`function` · `datafusion_execution::disk_manager::RefCountedTempFile::path` · datafusion-execution 55.1.0

```rust
fn path(&self) -> Option<&Path>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::RefCountedTempFile", "path": "RefCountedTempFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [525, 1], "end": [587, 2], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "datafusion_execution::spill_file::SpillFile", "path": "SpillFile"}, "trait_path": "datafusion_execution::spill_file::SpillFile"}`

Source: `src/disk_manager.rs:526`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdce1a5fbe603a95082b4c3e"></a>
## path

`function` · `datafusion_execution::disk_manager::RefCountedTempFile::path` · datafusion-execution 55.1.0

```rust
fn path(&self) -> &Path
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::RefCountedTempFile", "path": "RefCountedTempFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [423, 1], "end": [435, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:424`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f999723a270b5aaedf5a282b"></a>
## read_stream

`function` · `datafusion_execution::disk_manager::RefCountedTempFile::read_stream` · datafusion-execution 55.1.0

```rust
fn read_stream(&self) -> Result<std::pin::Pin<Box<dyn futures::Stream<Item = Result<Bytes>> + Send>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::RefCountedTempFile", "path": "RefCountedTempFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [525, 1], "end": [587, 2], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "datafusion_execution::spill_file::SpillFile", "path": "SpillFile"}, "trait_path": "datafusion_execution::spill_file::SpillFile"}`

Source: `src/disk_manager.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8712dec67dab6898523e715e"></a>
## size

`function` · `datafusion_execution::disk_manager::RefCountedTempFile::size` · datafusion-execution 55.1.0

```rust
fn size(&self) -> Option<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::RefCountedTempFile", "path": "RefCountedTempFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [525, 1], "end": [587, 2], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "datafusion_execution::spill_file::SpillFile", "path": "SpillFile"}, "trait_path": "datafusion_execution::spill_file::SpillFile"}`

Source: `src/disk_manager.rs:530`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
