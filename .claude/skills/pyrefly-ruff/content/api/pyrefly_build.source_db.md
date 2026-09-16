# `pyrefly_build::source_db`

Crate `pyrefly_build` · 6 public items · structured records in [`model/pyrefly_build.source_db.json`](../model/pyrefly_build.source_db.json)

## ConfigName

`struct` · `pyrefly_build::source_db::ConfigName`

```rust
struct ConfigName
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`, `serde_core::de::Deserialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>) -> derive_more::core::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
```

The name of an entry in the top-level `configs` map of a build system
response. Targets refer to their config by name so that a config shared by
many targets is sent, stored, and parsed only once.

---

## ModulePathCache

`struct` · `pyrefly_build::source_db::ModulePathCache`

```rust
struct ModulePathCache
```

**Derives**: Debug

**Methods** (2)

```rust
fn get(&self, path: &Path) -> ModulePath
fn new() -> Self
```

A `ModulePath` is optimised so that equal paths compare equal with `Arc::ptr_eq`,
that only works if we reuse the `ModulePath` and don't create new ones each time.
Use this cache to ensure we are reusing them.

---

## Target

`struct` · `pyrefly_build::source_db::Target`

```rust
struct Target
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn from_string(x: String) -> Self
fn to_os_str(&self) -> &OsStr
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
```

---

## LiveSourceDatabase

`trait` · `pyrefly_build::source_db::LiveSourceDatabase`

```rust
trait LiveSourceDatabase: SourceDatabase
```

**Implementors** (1)

- `pyrefly_build::source_db::query_source_db::QuerySourceDatabase`

**Methods** (7)

```rust
fn get_config(&self, name: &ConfigName) -> Option<serde_json::Value>
fn get_generated_files(&self) -> SmallSet<InternedPath>
fn get_paths_to_watch(&self) -> SmallSet<WatchPattern>
fn get_target(&self, origin: Option<&Path>) -> Option<Target>
fn get_target_config_name(&self, origin: Option<&Path>) -> Option<ConfigName>
fn get_target_root(&self, origin: Option<&Path>) -> Option<PathBuf>
fn query_source_db(&self, files: SmallSet<InternedPath>, force: bool) -> (anyhow::Result<bool>, TelemetrySourceDbRebuildInstanceStats)
```

Source database functionality for build-system-backed databases that can be
queried again as the project changes.

---

## ModuleEnumerator

`trait` · `pyrefly_build::source_db::ModuleEnumerator`

```rust
trait ModuleEnumerator: SourceDatabase
```

**Implementors** (3)

- `pyrefly::playground::PlaygroundSourceDatabase`
- `pyrefly_build::source_db::buck_check::BuckCheckSourceDatabase`
- `pyrefly_build::source_db::map_db::MapDatabase`

**Methods** (1)

```rust
fn modules_to_check(&self) -> Vec<Handle>
```

A source database that can enumerate the module handles that should seed a
checking run.

---

## SourceDatabase

`trait` · `pyrefly_build::source_db::SourceDatabase`

```rust
trait SourceDatabase: Send + Sync + fmt::Debug
```

**Implementors** (4)

- `pyrefly::playground::PlaygroundSourceDatabase`
- `pyrefly_build::source_db::buck_check::BuckCheckSourceDatabase`
- `pyrefly_build::source_db::map_db::MapDatabase`
- `pyrefly_build::source_db::query_source_db::QuerySourceDatabase`

**Methods** (4)

```rust
fn as_live_source_database(&self) -> Option<&dyn LiveSourceDatabase>
fn handle_from_module_path(&self, module_path: &ModulePath) -> Option<Handle>
fn lookup(&self, module: ModuleName, origin: Option<&Path>, style_filter: Option<ModuleStyle>) -> Option<ModulePath>
fn may_contain_module(&self, _module: ModuleName) -> bool
```

Represents a virtual filesystem provided by a build system. A build system
should understand the relationship between targets and importable qualified
paths to the files contained in the build system.

---
