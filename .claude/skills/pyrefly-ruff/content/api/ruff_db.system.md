# `ruff_db::system`

Crate `ruff_db` · 9 public items · structured records in [`model/ruff_db.system.json`](../model/ruff_db.system.json)

## FileType

`enum` · `ruff_db::system::FileType`

```rust
enum FileType
```

**Variants**: `File`, `Directory`, `Symlink`

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn is_directory(self) -> bool
const fn is_file(self) -> bool
const fn is_symlink(self) -> bool
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---

## WhichError

`enum` · `ruff_db::system::WhichError`

```rust
enum WhichError
```

**Variants**: `CannotFindBinaryPath`, `CannotGetCurrentDirAndPathListEmpty`, `CannotCanonicalize`, `NonUtf8Path`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## file_time_now

`function` · `ruff_db::system::file_time_now`

```rust
fn file_time_now() -> filetime::FileTime
```

---

## DirectoryEntry

`struct` · `ruff_db::system::DirectoryEntry`

```rust
struct DirectoryEntry
```

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn file_type(&self) -> FileType
fn into_path(self) -> SystemPathBuf
fn new(path: SystemPathBuf, file_type: FileType) -> Self
fn path(&self) -> &SystemPath
```

---

## Metadata

`struct` · `ruff_db::system::Metadata`

```rust
struct Metadata
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn file_type(&self) -> FileType
fn new(revision: FileRevision, permissions: Option<u32>, file_type: FileType) -> Self
fn permissions(&self) -> Option<u32>
fn revision(&self) -> FileRevision
```

---

## System

`trait` · `ruff_db::system::System`

```rust
trait System: Debug + Sync + Send
```

**Implementors** (2)

- `ruff_db::system::test::InMemorySystem`
- `ruff_db::system::test::TestSystem`

**Methods** (25)

```rust
fn as_any(&self) -> &dyn std::any::Any
fn as_any_mut(&mut self) -> &mut dyn std::any::Any
fn as_writable(&self) -> Option<&dyn WritableSystem>
fn cache_dir(&self) -> Option<SystemPathBuf>
fn canonicalize_path(&self, path: &SystemPath) -> Result<SystemPathBuf>
fn command_executor(&self) -> Option<&dyn CommandExecutor>
fn current_directory(&self) -> &SystemPath
fn dyn_clone(&self) -> Box<dyn System>
fn env_var(&self, name: &str) -> std::result::Result<String, std::env::VarError>
fn is_directory(&self, path: &SystemPath) -> bool
fn is_file(&self, path: &SystemPath) -> bool
fn is_same_file(&self, first: &SystemPath, second: &SystemPath) -> Result<bool>
fn path_exists(&self, path: &SystemPath) -> bool
fn path_metadata(&self, path: &SystemPath) -> Result<Metadata>
fn read_directory<'a>(&'a self, path: &SystemPath) -> Result<Box<dyn Iterator<Item = Result<DirectoryEntry>> + 'a>>
fn read_to_notebook(&self, path: &SystemPath) -> std::result::Result<Notebook, NotebookError>
fn read_to_string(&self, path: &SystemPath) -> Result<String>
fn read_virtual_path_to_notebook(&self, path: &SystemVirtualPath) -> std::result::Result<Notebook, NotebookError>
fn read_virtual_path_to_string(&self, path: &SystemVirtualPath) -> Result<String>
fn run_command(&self, command: Command) -> Result<Output>
fn source_type(&self, path: &SystemPath) -> Option<PySourceType>
fn user_config_directory(&self) -> Option<SystemPathBuf>
fn virtual_path_source_type(&self, path: &SystemVirtualPath) -> Option<PySourceType>
fn walk_directory(&self, path: &SystemPath) -> WalkDirectoryBuilder
fn which(&self, binary_name: &str) -> WhichResult
```

The system on which Ruff runs.

Ruff supports running on the CLI, in a language server, and in a browser (WASM). Each of these
host-systems differ in what system operations they support and how they interact with the file system:
* Language server:
   * Reading a file's content should take into account that it might have unsaved changes because it's open in the editor.
   * Use structured representations for notebooks, making deserializing a notebook from a string unnecessary.
   * Use their own file watching infrastructure.
* WASM (Browser):
   * There are ways to emulate a file system in WASM but a native memory-filesystem is more efficient.
   * Doesn't support a current working directory
   * File watching isn't supported.

Abstracting the system also enables tests to use a more efficient in-memory file system.

---

## WritableSystem

`trait` · `ruff_db::system::WritableSystem`

```rust
trait WritableSystem: System
```

**Implementors** (2)

- `ruff_db::system::test::InMemorySystem`
- `ruff_db::system::test::TestSystem`

**Methods** (6)

```rust
fn create_directory_all(&self, path: &SystemPath) -> Result<()>
fn create_new_file(&self, path: &SystemPath) -> Result<()>
fn dyn_clone(&self) -> Box<dyn WritableSystem>
fn get_or_cache(&self, path: &SystemPath, read_contents: &dyn Fn() -> Result<String>) -> Result<Option<SystemPathBuf>>
fn write_file(&self, path: &SystemPath, content: &str) -> Result<()>
fn write_file_bytes(&self, path: &SystemPath, content: &[u8]) -> Result<()>
```

System trait for non-readonly systems.

---

## Result

`type_alias` · `ruff_db::system::Result`

```rust
type Result<T> = std::io::Result<T>
```

---

## WhichResult

`type_alias` · `ruff_db::system::WhichResult`

```rust
type WhichResult = std::result::Result<SystemPathBuf, WhichError>
```

---
