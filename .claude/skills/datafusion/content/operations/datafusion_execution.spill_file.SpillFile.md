# `datafusion_execution::spill_file::SpillFile`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.spill_file.SpillFile.json).

<a id="op-f9549091c567b61ec7cb7828"></a>
## SpillFile

`trait` · `datafusion_execution::spill_file::SpillFile` · datafusion-execution 55.1.0

```rust
trait SpillFile: Send + Sync
```

Source: `src/spill_file.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Abstraction over a spill file backend.
Implementations handle their own quota enforcement and blocking concerns.

<a id="op-dba9f651e8d3d48e3fea9e7b"></a>
## open_writer

`function` · `datafusion_execution::spill_file::SpillFile::open_writer` · datafusion-execution 55.1.0

```rust
fn open_writer(&self) -> Result<Box<dyn SpillWriter>>
```

Source: `src/spill_file.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Opens a writer for appending data to this file.

<a id="op-0aa65b8348161b5fbb486ed5"></a>
## path

`function` · `datafusion_execution::spill_file::SpillFile::path` · datafusion-execution 55.1.0

```rust
fn path(&self) -> Option<&Path>
```

Source: `src/spill_file.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns the OS path if this is a local file, None otherwise.

<a id="op-84dc45f069f07335c055f12e"></a>
## read_stream

`function` · `datafusion_execution::spill_file::SpillFile::read_stream` · datafusion-execution 55.1.0

```rust
fn read_stream(&self) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes>> + Send>>>
```

Source: `src/spill_file.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns file contents as an async stream of byte chunks.

<a id="op-796f16d4e441430f1b83276b"></a>
## size

`function` · `datafusion_execution::spill_file::SpillFile::size` · datafusion-execution 55.1.0

```rust
fn size(&self) -> Option<u64>
```

Source: `src/spill_file.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns current size in bytes if cheaply available.
