# `datafusion_execution::disk_manager`

Crate `datafusion-execution` · 8 public items · structured records in [`model/datafusion_execution.disk_manager.json`](../model/datafusion_execution.disk_manager.json)

## DEFAULT_MAX_SPILL_MERGE_FAN_IN

`constant` · `datafusion_execution::disk_manager::DEFAULT_MAX_SPILL_MERGE_FAN_IN`

```rust
const DEFAULT_MAX_SPILL_MERGE_FAN_IN: usize = 0
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.disk_manager.DEFAULT_MAX_SPILL_MERGE_FAN_IN.md).


---

## DEFAULT_MAX_TEMP_DIRECTORY_SIZE

`constant` · `datafusion_execution::disk_manager::DEFAULT_MAX_TEMP_DIRECTORY_SIZE`

```rust
const DEFAULT_MAX_TEMP_DIRECTORY_SIZE: u64 = _
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.disk_manager.DEFAULT_MAX_TEMP_DIRECTORY_SIZE.md).


---

## DiskManagerMode

`enum` · `datafusion_execution::disk_manager::DiskManagerMode`

```rust
enum DiskManagerMode
```

**Variants**: `OsTmpDirectory`, `Directories`, `Custom`, `Disabled`

**Derives**: Clone, Debug, Default

[Full member, field, variant and typed contracts](../operations/datafusion_execution.disk_manager.DiskManagerMode.md).


---

## DiskManager

`struct` · `datafusion_execution::disk_manager::DiskManager`

Also reachable as `datafusion::execution::DiskManager`, `datafusion_execution::DiskManager`

```rust
struct DiskManager
```

**Derives**: Debug

**Methods** (12)

```rust
fn builder() -> DiskManagerBuilder
fn create_tmp_file(&Arc<self>, request_description: &str) -> Result<Arc<dyn SpillFile>>
fn max_spill_merge_fan_in(&self) -> usize
fn max_temp_directory_size(&self) -> u64
fn set_arc_max_temp_directory_size(this: &Arc<Self>, max_temp_directory_size: u64) -> Result<()>
fn set_max_spill_merge_fan_in(&self, max_spill_merge_fan_in: usize)
fn set_max_temp_directory_size(&self, max_temp_directory_size: u64) -> Result<()>
fn spilling_progress(&self) -> SpillingProgress
fn temp_dir_paths(&self) -> Vec<PathBuf>
fn tmp_files_enabled(&self) -> bool
fn used_disk_space(&self) -> u64
fn with_max_temp_directory_size(self, max_temp_directory_size: u64) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.disk_manager.DiskManager.md).


Manages files generated during query execution, e.g. spill files generated
while processing dataset larger than available memory.

---

## DiskManagerBuilder

`struct` · `datafusion_execution::disk_manager::DiskManagerBuilder`

```rust
struct DiskManagerBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (9)

```rust
fn build(self) -> Result<DiskManager>
fn set_max_spill_merge_fan_in(&mut self, value: usize)
fn set_max_temp_directory_size(&mut self, value: u64)
fn set_mode(&mut self, mode: DiskManagerMode)
fn set_temp_file_factory(&mut self, temp_file_factory: Arc<dyn TempFileFactory>)
fn with_max_spill_merge_fan_in(self, value: usize) -> Self
fn with_max_temp_directory_size(self, value: u64) -> Self
fn with_mode(self, mode: DiskManagerMode) -> Self
fn with_temp_file_factory(self, temp_file_factory: Arc<dyn TempFileFactory>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.disk_manager.DiskManagerBuilder.md).


Builder pattern for the [DiskManager] structure

---

## FileSpillWriter

`struct` · `datafusion_execution::disk_manager::FileSpillWriter`

```rust
struct FileSpillWriter
```

**Implements**: `core::io::write::Write`, `datafusion_execution::spill_file::SpillWriter`

**via `core::io::write::Write`**

```rust
fn flush(&mut self) -> std::io::Result<()>
fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>
```

**via `datafusion_execution::spill_file::SpillWriter`**

```rust
fn finish(&mut self) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.disk_manager.FileSpillWriter.md).


---

## RefCountedTempFile

`struct` · `datafusion_execution::disk_manager::RefCountedTempFile`

```rust
struct RefCountedTempFile
```

**Implements**: `core::ops::drop::Drop`, `datafusion_execution::spill_file::SpillFile`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn inner(&self) -> &NamedTempFile
fn path(&self) -> &Path
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

**via `datafusion_execution::spill_file::SpillFile`**

```rust
fn open_writer(&self) -> Result<Box<dyn SpillWriter>>
fn path(&self) -> Option<&Path>
fn read_stream(&self) -> Result<std::pin::Pin<Box<dyn futures::Stream<Item = Result<Bytes>> + Send>>>
fn size(&self) -> Option<u64>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.disk_manager.RefCountedTempFile.md).


A wrapper around a [`NamedTempFile`] that also contains
a reference to its parent temporary directory.

This type is Clone-able, allowing multiple references to the same underlying file.
The file is deleted only when the last reference is dropped.

The parent temporary directory is also kept alive as long as any reference to
this file exists, preventing premature cleanup of the directory.

Once all references to this file are dropped, the file is deleted, and the
disk usage is subtracted from the disk manager's total.

---

## SpillingProgress

`struct` · `datafusion_execution::disk_manager::SpillingProgress`

```rust
struct SpillingProgress
```

**Fields**: `current_bytes`, `active_files_count`

**Derives**: Clone, Copy, Debug

[Full member, field, variant and typed contracts](../operations/datafusion_execution.disk_manager.SpillingProgress.md).


Information about the current disk usage for spilling

---
