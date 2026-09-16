# `pyrefly_build::module_resolver`

Crate `pyrefly_build` · 9 public items · structured records in [`model/pyrefly_build.module_resolver.json`](../model/pyrefly_build.module_resolver.json)

## FindResult

`enum` · `pyrefly_build::module_resolver::FindResult`

```rust
enum FindResult
```

**Variants**: `SingleFilePyiModule`, `SingleFilePyModule`, `RegularPackage`, `LegacyNamespacePackage`, `ImplicitNamespacePackage`, `CompiledModule`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn module_path(self) -> Option<ModulePath>
```

---

## StubSearchResult

`enum` · `pyrefly_build::module_resolver::StubSearchResult`

```rust
enum StubSearchResult
```

**Variants**: `Provides`, `Transparent`

**Derives**: Debug

**Methods** (1)

```rust
fn into_find_result(self) -> FindResult
```

---

## find_module_prefixes

`function` · `pyrefly_build::module_resolver::find_module_prefixes`

```rust
fn find_module_prefixes<'a>(prefix: pyrefly_python::module_name::ModuleName, include: impl Iterator<Item = &'a std::path::PathBuf>) -> Vec<pyrefly_python::module_name::ModuleName>
```

---

## find_module_results

`function` · `pyrefly_build::module_resolver::find_module_results`

```rust
fn find_module_results<'a, I>(module: pyrefly_python::module_name::ModuleName, include: I, style_filter: Option<pyrefly_python::module_path::ModuleStyle>, phantom_paths: &mut Option<&mut Vec<std::path::PathBuf>>, dir_cache: &DirEntryCache, observer: Option<&dyn ModuleResolutionObserver>) -> ModuleSearchResult where I: Iterator<Item = &'a std::path::PathBuf> + Clone
```

---

## package_has_py_typed

`function` · `pyrefly_build::module_resolver::package_has_py_typed`

```rust
fn package_has_py_typed(module: pyrefly_python::module_name::ModuleName, result: &FindResult, dir_cache: &DirEntryCache) -> bool
```

Whether the package containing `module` has a `py.typed` marker.

PEP 561 places the marker in the top-level package directory, but under a
PEP 420 namespace root the top level of the distribution is a sub-package of
the namespace rather than the namespace itself. Accept a marker in any package
directory between the resolved module and the search root.

---

## DirEntryCache

`struct` · `pyrefly_build::module_resolver::DirEntryCache`

```rust
struct DirEntryCache
```

**Derives**: Debug, Default

**Methods** (3)

```rust
fn dir_exists(&self, dir: &Path) -> bool
fn file_exists(&self, path: &Path) -> bool
fn new() -> Self
```

Cache of directory listings to avoid repeated stat() calls during module resolution.

Each directory is read at most once. Entries store
the file type alongside the name, which normally comes from dirent `d_type`;
symlinks are followed with `metadata()` to preserve `Path::is_dir()` behavior.
The cache is never invalidated, so callers should scope it to a stable
resolution transaction or replace it when file changes are observed.

---

## ModuleResolver

`struct` · `pyrefly_build::module_resolver::ModuleResolver`

```rust
struct ModuleResolver
```

**Derives**: Debug

**Methods** (2)

```rust
fn new(roots: impl IntoIterator<Item = PathBuf>) -> Self
fn resolve(&self, module: ModuleName, style: Option<ModuleStyle>) -> Option<ModulePath>
```

Filesystem-only module resolver for build-system integrations.

This intentionally exposes the shared package/stub/namespace lookup core
without checker-specific policy such as bundled third-party stubs, `py.typed`
diagnostics, import-ignore handling, or namespace fallback across separate
search phases.

---

## ModuleSearchResult

`struct` · `pyrefly_build::module_resolver::ModuleSearchResult`

```rust
struct ModuleSearchResult
```

**Fields**: `stub_result`, `normal_result`

**Derives**: Debug, Default

---

## ModuleResolutionObserver

`trait` · `pyrefly_build::module_resolver::ModuleResolutionObserver`

```rust
trait ModuleResolutionObserver
```

**Methods** (2)

```rust
fn observe_read(&self, elapsed_ns: u64)
fn observe_stat(&self, elapsed_ns: u64)
```

---
