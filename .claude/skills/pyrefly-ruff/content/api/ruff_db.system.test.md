# `ruff_db::system::test`

Crate `ruff_db` · 4 public items · structured records in [`model/ruff_db.system.test.json`](../model/ruff_db.system.test.json)

## InMemorySystem

`struct` · `ruff_db::system::test::InMemorySystem`

Also reachable as `ruff_db::system::InMemorySystem`

```rust
struct InMemorySystem
```

**Implements**: `ruff_db::system::System`, `ruff_db::system::WritableSystem`

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn from_memory_fs(memory_fs: MemoryFileSystem) -> Self
fn fs(&self) -> &MemoryFileSystem
fn set_user_configuration_directory(&self, directory: Option<SystemPathBuf>)
```

**via `ruff_db::system::System`**

```rust
fn as_any(&self) -> &dyn std::any::Any
fn as_any_mut(&mut self) -> &mut dyn std::any::Any
fn as_writable(&self) -> Option<&dyn WritableSystem>
fn cache_dir(&self) -> Option<SystemPathBuf>
fn canonicalize_path(&self, path: &SystemPath) -> Result<SystemPathBuf>
fn current_directory(&self) -> &SystemPath
fn dyn_clone(&self) -> Box<dyn System>
fn is_same_file(&self, first: &SystemPath, second: &SystemPath) -> Result<bool>
fn path_metadata(&self, path: &SystemPath) -> Result<Metadata>
fn read_directory<'a>(&'a self, path: &SystemPath) -> Result<Box<dyn Iterator<Item = Result<DirectoryEntry>> + 'a>>
fn read_to_notebook(&self, path: &SystemPath) -> std::result::Result<Notebook, NotebookError>
fn read_to_string(&self, path: &SystemPath) -> Result<String>
fn read_virtual_path_to_notebook(&self, path: &SystemVirtualPath) -> std::result::Result<Notebook, NotebookError>
fn read_virtual_path_to_string(&self, path: &SystemVirtualPath) -> Result<String>
fn user_config_directory(&self) -> Option<SystemPathBuf>
fn walk_directory(&self, path: &SystemPath) -> WalkDirectoryBuilder
fn which(&self, _name: &str) -> WhichResult
```

**via `ruff_db::system::WritableSystem`**

```rust
fn create_directory_all(&self, path: &SystemPath) -> Result<()>
fn create_new_file(&self, path: &SystemPath) -> Result<()>
fn dyn_clone(&self) -> Box<dyn WritableSystem>
fn write_file_bytes(&self, path: &SystemPath, content: &[u8]) -> Result<()>
```

---

## TestSystem

`struct` · `ruff_db::system::test::TestSystem`

Also reachable as `ruff_db::system::TestSystem`

```rust
struct TestSystem
```

**Implements**: `ruff_db::system::System`, `ruff_db::system::WritableSystem`, `ruff_db::system::command::CommandExecutor`

**Derives**: Clone, Debug, Default

**Methods** (7)

```rust
fn clear_env_vars(&self)
fn in_memory(&self) -> &InMemorySystem
fn memory_file_system(&self) -> &MemoryFileSystem
fn new(inner: impl WritableSystem + RefUnwindSafe + Send + Sync + 'static) -> Self
fn remove_env_var(&self, name: impl Into<String>)
fn set_env_var(&self, name: impl Into<String>, value: impl Into<String>)
fn set_env_vars<I, K, V>(&self, variables: I) where I: IntoIterator<Item = (K, V)>, K: Into<String>, V: Into<String>
```

**via `ruff_db::system::System`**

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
fn is_same_file(&self, first: &SystemPath, second: &SystemPath) -> Result<bool>
fn path_metadata(&self, path: &SystemPath) -> Result<Metadata>
fn read_directory<'a>(&'a self, path: &SystemPath) -> Result<Box<dyn Iterator<Item = Result<DirectoryEntry>> + 'a>>
fn read_to_notebook(&self, path: &SystemPath) -> std::result::Result<Notebook, NotebookError>
fn read_to_string(&self, path: &SystemPath) -> Result<String>
fn read_virtual_path_to_notebook(&self, path: &SystemVirtualPath) -> std::result::Result<Notebook, NotebookError>
fn read_virtual_path_to_string(&self, path: &SystemVirtualPath) -> Result<String>
fn run_command(&self, command: Command) -> Result<Output>
fn user_config_directory(&self) -> Option<SystemPathBuf>
fn walk_directory(&self, path: &SystemPath) -> WalkDirectoryBuilder
fn which(&self, _name: &str) -> WhichResult
```

**via `ruff_db::system::WritableSystem`**

```rust
fn create_directory_all(&self, path: &SystemPath) -> Result<()>
fn create_new_file(&self, path: &SystemPath) -> Result<()>
fn dyn_clone(&self) -> Box<dyn WritableSystem>
fn write_file_bytes(&self, path: &SystemPath, content: &[u8]) -> Result<()>
```

**via `ruff_db::system::command::CommandExecutor`**

```rust
fn dyn_clone(&self) -> Box<dyn CommandExecutor>
fn execute(&self, command: Command) -> Result<Output>
```

System implementation intended for testing.

It uses a memory-file system by default, but can be switched to the real file system for tests
verifying more advanced file system features.

## Warning
Don't use this system for production code. It's intended for testing only.

---

## DbWithTestSystem

`trait` · `ruff_db::system::test::DbWithTestSystem`

Also reachable as `ruff_db::system::DbWithTestSystem`

```rust
trait DbWithTestSystem: Db + Sized
```

**Methods** (5)

```rust
fn memory_file_system(&self) -> &MemoryFileSystem
fn test_system(&self) -> &TestSystem
fn test_system_mut(&mut self) -> &mut TestSystem
fn use_system<S>(&mut self, os: S) where S: WritableSystem + Send + Sync + RefUnwindSafe + 'static
fn write_virtual_file(&mut self, path: impl AsRef<SystemVirtualPath>, content: impl AsRef<[u8]>)
```

Extension trait for databases that use [`TestSystem`].

Provides various helper function that ease testing.

---

## DbWithWritableSystem

`trait` · `ruff_db::system::test::DbWithWritableSystem`

Also reachable as `ruff_db::system::DbWithWritableSystem`

```rust
trait DbWithWritableSystem: Db + Sized
```

**Methods** (4)

```rust
fn writable_system(&self) -> &Self::System
fn write_dedented(&mut self, path: &str, content: &str) -> Result<()>
fn write_file(&mut self, path: impl AsRef<SystemPath>, content: impl AsRef<str>) -> Result<()>
fn write_files<P, C, I>(&mut self, files: I) -> Result<()> where I: IntoIterator<Item = (P, C)>, P: AsRef<SystemPath>, C: AsRef<str>
```

Extension trait for databases that use a [`WritableSystem`].

Provides various helper function that ease testing.

---
