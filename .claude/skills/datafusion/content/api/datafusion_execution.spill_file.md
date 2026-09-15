# `datafusion_execution::spill_file`

Crate `datafusion-execution` · 3 public items · structured records in [`model/datafusion_execution.spill_file.json`](../model/datafusion_execution.spill_file.json)

## SpillFile

`trait` · `datafusion_execution::spill_file::SpillFile`

Also reachable as `datafusion::execution::SpillFile`, `datafusion_execution::SpillFile`

```rust
trait SpillFile: Send + Sync
```

**Implementors** (1)

- `datafusion_execution::disk_manager::RefCountedTempFile`

**Methods** (4)

```rust
fn open_writer(&self) -> Result<Box<dyn SpillWriter>>
fn path(&self) -> Option<&Path>
fn read_stream(&self) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes>> + Send>>>
fn size(&self) -> Option<u64>
```

Abstraction over a spill file backend.
Implementations handle their own quota enforcement and blocking concerns.

---

## SpillWriter

`trait` · `datafusion_execution::spill_file::SpillWriter`

Also reachable as `datafusion::execution::SpillWriter`, `datafusion_execution::SpillWriter`

```rust
trait SpillWriter: std::io::Write + Send
```

**Implementors** (1)

- `datafusion_execution::disk_manager::FileSpillWriter`

**Methods** (1)

```rust
fn finish(&mut self) -> Result<()>
```

Writer for spill file backends.

---

## TempFileFactory

`trait` · `datafusion_execution::spill_file::TempFileFactory`

Also reachable as `datafusion::execution::TempFileFactory`, `datafusion_execution::TempFileFactory`

```rust
trait TempFileFactory: Send + Sync
```

**Methods** (1)

```rust
fn create_temp_file(&self, description: &str) -> Result<Arc<dyn SpillFile>>
```

Factory for creating spill files.

---
