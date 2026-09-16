# `pyrefly_config::finder`

Crate `pyrefly_config` · 3 public items · structured records in [`model/pyrefly_config.finder.json`](../model/pyrefly_config.finder.json)

## debug_log

`function` · `pyrefly_config::finder::debug_log`

```rust
fn debug_log(errors: Vec<ConfigError>)
```

When debugging is enabled, log errors.

---

## ConfigError

`struct` · `pyrefly_config::finder::ConfigError`

```rust
struct ConfigError
```

**Methods** (6)

```rust
fn context(self, context: String) -> Self
fn error(msg: anyhow::Error) -> Self
fn get_message(&self) -> String
fn print(&self)
fn severity(&self) -> Severity
fn warn(msg: anyhow::Error) -> Self
```

---

## ConfigFinder

`struct` · `pyrefly_config::finder::ConfigFinder`

```rust
struct ConfigFinder
```

**Methods** (11)

```rust
fn add_errors(&self, errors: Vec<ConfigError>)
fn checkpoint<R, E>(&self, result: Result<R, E>) -> Result<R, E>
fn clear(&self)
fn directory(&self, dir: &Path) -> Option<ArcId<ConfigFile>>
fn errors(&self) -> Vec<ConfigError>
fn file(&self, file: &Path) -> Option<ArcId<ConfigFile>>
fn new(load: Box<dyn Fn(&std::path::Path) -> (pyrefly_util::arc_id::ArcId<config::ConfigFile>, Vec<ConfigError>) + Send + Sync>, fallback: Box<dyn Fn(pyrefly_python::module_name::ModuleNameWithKind, &pyrefly_python::module_path::ModulePath) -> pyrefly_util::arc_id::ArcId<config::ConfigFile> + Send + Sync>, clear_extra_caches: Box<dyn Fn() + Send + Sync>) -> Self
fn new_constant(constant: ArcId<ConfigFile>) -> Self
fn new_custom(before: Box<dyn Fn(pyrefly_python::module_name::ModuleName, &pyrefly_python::module_path::ModulePath) -> anyhow::Result<Option<pyrefly_util::arc_id::ArcId<config::ConfigFile>>> + Send + Sync>, load: Box<dyn Fn(&std::path::Path) -> (pyrefly_util::arc_id::ArcId<config::ConfigFile>, Vec<ConfigError>) + Send + Sync>, fallback: Box<dyn Fn(pyrefly_python::module_name::ModuleNameWithKind, &pyrefly_python::module_path::ModulePath) -> pyrefly_util::arc_id::ArcId<config::ConfigFile> + Send + Sync>, clear_extra_caches: Box<dyn Fn() + Send + Sync>) -> Self
fn print_errors(&self)
fn python_file(&self, name: ModuleNameWithKind, path: &ModulePath) -> ArcId<ConfigFile>
```

A way to find a config file given a directory or Python file.
Uses a lot of caching.

---
