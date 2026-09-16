# `ruff_db::system::memory_fs`

Crate `ruff_db` · 2 public items · structured records in [`model/ruff_db.system.memory_fs.json`](../model/ruff_db.system.memory_fs.json)

## MemoryFileSystem

`struct` · `ruff_db::system::memory_fs::MemoryFileSystem`

Also reachable as `ruff_db::system::MemoryFileSystem`

```rust
struct MemoryFileSystem
```

**Derives**: Clone, Debug, Default

**Methods** (23)

```rust
fn canonicalize(&self, path: impl AsRef<SystemPath>) -> Result<SystemPathBuf>
fn create_directory_all(&self, path: impl AsRef<SystemPath>) -> Result<()>
fn current_directory(&self) -> &SystemPath
fn exists(&self, path: &SystemPath) -> bool
fn is_directory(&self, path: impl AsRef<SystemPath>) -> bool
fn is_executable(&self, path: &SystemPath) -> bool
fn is_file(&self, path: impl AsRef<SystemPath>) -> bool
fn metadata(&self, path: impl AsRef<SystemPath>) -> Result<Metadata>
fn new() -> Self
fn read_directory(&self, path: impl AsRef<SystemPath>) -> Result<ReadDirectory>
fn read_to_string(&self, path: impl AsRef<SystemPath>) -> Result<String>
fn remove_all(&self)
fn remove_directory(&self, path: impl AsRef<SystemPath>) -> Result<()>
fn remove_file(&self, path: impl AsRef<SystemPath>) -> Result<()>
fn remove_virtual_file(&self, path: impl AsRef<SystemVirtualPath>) -> Result<()>
fn touch(&self, path: impl AsRef<SystemPath>) -> Result<()>
fn virtual_path_exists(&self, path: &SystemVirtualPath) -> bool
fn walk_directory(&self, path: impl AsRef<SystemPath>) -> WalkDirectoryBuilder
fn with_current_directory(cwd: impl AsRef<SystemPath>) -> Self
fn write_file(&self, path: impl AsRef<SystemPath>, content: impl AsRef<[u8]>) -> Result<()>
fn write_file_all(&self, path: impl AsRef<SystemPath>, content: impl AsRef<[u8]>) -> Result<()>
fn write_files_all<P, C>(&self, files: impl IntoIterator<Item = (P, C)>) -> Result<()> where P: AsRef<SystemPath>, C: ToString
fn write_virtual_file(&self, path: impl AsRef<SystemVirtualPath>, content: impl AsRef<[u8]>)
```

File system that stores all content in memory.

The file system supports files and directories. Paths are case-sensitive.

The implementation doesn't aim at fully capturing the behavior of a real file system.
The implementation intentionally doesn't support:
* symlinks
* hardlinks
* permissions: All files and directories have the permission 0755.

Use a tempdir with the real file system to test these advanced file system features and behavior.

---

## ReadDirectory

`struct` · `ruff_db::system::memory_fs::ReadDirectory`

```rust
struct ReadDirectory
```

---
