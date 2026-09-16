# `ruff_workspace::resolver`

Crate `ruff_workspace` · 16 public items · structured records in [`model/ruff_workspace.resolver.json`](../model/ruff_workspace.resolver.json)

## ConfigurationOrigin

`enum` · `ruff_workspace::resolver::ConfigurationOrigin`

```rust
enum ConfigurationOrigin
```

**Variants**: `Unknown`, `UserSpecified`, `UserSettings`, `Ancestor`

**Derives**: Clone, Copy, Debug

How the configuration is provided.

---

## ExclusionKind

`enum` · `ruff_workspace::resolver::ExclusionKind`

```rust
enum ExclusionKind
```

**Variants**: `Exclude`, `ExtendExclude`, `LintExclude`, `FormatExclude`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## InclusionKind

`enum` · `ruff_workspace::resolver::InclusionKind`

```rust
enum InclusionKind
```

**Variants**: `Include`, `ExtendInclude`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## PyprojectDiscoveryStrategy

`enum` · `ruff_workspace::resolver::PyprojectDiscoveryStrategy`

```rust
enum PyprojectDiscoveryStrategy
```

**Variants**: `Fixed`, `Hierarchical`

**Derives**: Clone, Copy, Debug

The strategy used to discover the relevant `pyproject.toml` file for each
Python file.

---

## Relativity

`enum` · `ruff_workspace::resolver::Relativity`

```rust
enum Relativity
```

**Variants**: `Cwd`, `Parent`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy

**via `core::convert::From`**

```rust
fn from(value: ConfigurationOrigin) -> Self
```

The strategy for resolving file paths in a `pyproject.toml`.

---

## ResolvedFile

`enum` · `ruff_workspace::resolver::ResolvedFile`

```rust
enum ResolvedFile
```

**Variants**: `Root`, `Nested`

**Derives**: Clone, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn file_name(&self) -> &OsStr
fn into_path(self) -> PathBuf
fn is_root(&self) -> bool
fn path(&self) -> &Path
```

---

## match_any_exclusion

`function` · `ruff_workspace::resolver::match_any_exclusion`

```rust
fn match_any_exclusion(path: &std::path::Path, resolver_settings: &FileResolverSettings, lint_exclude: Option<&globset::GlobSet>, format_exclude: Option<&globset::GlobSet>) -> Option<ExclusionKind>
```

Return the [`ExclusionKind`] for a given [`Path`], if the path or any of its ancestors match
any of the exclusion criteria.

---

## match_any_inclusion

`function` · `ruff_workspace::resolver::match_any_inclusion`

```rust
fn match_any_inclusion(path: &std::path::Path, resolver_settings: &FileResolverSettings) -> Option<InclusionKind>
```

Return the [`InclusionKind`] for a given [`Path`], if the path match any of the inclusion
criteria.

---

## match_exclusion

`function` · `ruff_workspace::resolver::match_exclusion`

```rust
fn match_exclusion<P: AsRef<std::path::Path>, R: AsRef<std::path::Path>>(file_path: P, file_basename: R, exclusion: &globset::GlobSet) -> bool
```

Return `true` if the given file should be ignored based on the exclusion
criteria.

---

## project_file_at_path

`function` · `ruff_workspace::resolver::project_file_at_path`

```rust
fn project_file_at_path(path: &std::path::Path, resolver: &mut Resolver<'_>, transformer: &dyn ConfigurationTransformer) -> anyhow::Result<bool>
```

Return `true` if the Python file at [`Path`] is _not_ excluded.

---

## project_files_in_path

`function` · `ruff_workspace::resolver::project_files_in_path`

```rust
fn project_files_in_path<'a>(paths: &[std::path::PathBuf], pyproject_config: &'a PyprojectConfig, transformer: &dyn ConfigurationTransformer + Sync) -> anyhow::Result<(Vec<anyhow::Result<ResolvedFile, ignore::Error>>, Resolver<'a>)>
```

Find all project files in a set of paths, following configured include/exclude settings.

---

## resolve_configuration

`function` · `ruff_workspace::resolver::resolve_configuration`

```rust
fn resolve_configuration(initial_config_path: &std::path::Path, transformer: &dyn ConfigurationTransformer, origin: ConfigurationOrigin) -> anyhow::Result<configuration::Configuration>
```

Recursively resolve a [`Configuration`] from a `pyproject.toml` file at the
specified [`Path`].

---

## resolve_root_settings

`function` · `ruff_workspace::resolver::resolve_root_settings`

```rust
fn resolve_root_settings(pyproject: &std::path::Path, transformer: &dyn ConfigurationTransformer, origin: ConfigurationOrigin) -> anyhow::Result<settings::Settings>
```

Extract the [`Settings`] from a given `pyproject.toml` and process the
configuration with the given [`ConfigurationTransformer`].

---

## PyprojectConfig

`struct` · `ruff_workspace::resolver::PyprojectConfig`

```rust
struct PyprojectConfig
```

**Fields**: `strategy`, `settings`, `path`

**Derives**: Debug

**Methods** (1)

```rust
fn new(strategy: PyprojectDiscoveryStrategy, settings: Settings, path: Option<PathBuf>) -> Self
```

The configuration information from a `pyproject.toml` file.

---

## Resolver

`struct` · `ruff_workspace::resolver::Resolver`

```rust
struct Resolver<'a>
```

**Derives**: Debug

**Methods** (7)

```rust
fn base_settings(&self) -> &Settings
fn force_exclude(&self) -> bool
fn new(pyproject_config: &'a PyprojectConfig) -> Self
fn package_roots(&'a self, files: &[&'a Path]) -> FxHashMap<&'a Path, Option<PackageRoot<'a>>>
fn resolve(&self, path: &Path) -> &Settings
fn resolve_with_path(&self, path: &Path) -> (&Settings, Option<&Path>)
fn settings(&self) -> impl Iterator<Item = &Settings>
```

---

## ConfigurationTransformer

`trait` · `ruff_workspace::resolver::ConfigurationTransformer`

```rust
trait ConfigurationTransformer
```

**Implementors** (1)

- `ruff::args::ConfigArguments`

**Methods** (1)

```rust
fn transform(&self, config: Configuration) -> Configuration
```

Applies a transformation to a [`Configuration`].

Used to override options with the values provided by the CLI.

---
