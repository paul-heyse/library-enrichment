# `pyrefly_config::config`

Crate `pyrefly_config` · 17 public items · structured records in [`model/pyrefly_config.config.json`](../model/pyrefly_config.config.json)

## BaselineFormat

`enum` · `pyrefly_config::config::BaselineFormat`

```rust
enum BaselineFormat
```

**Variants**: `Full`, `Minimal`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Amount of diagnostic information written to a baseline file.

---

## BaselineMatchingMode

`enum` · `pyrefly_config::config::BaselineMatchingMode`

```rust
enum BaselineMatchingMode
```

**Variants**: `Column`, `ConciseDescription`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Fields used to match diagnostics against baseline entries.

---

## ConfigScope

`enum` · `pyrefly_config::config::ConfigScope`

```rust
enum ConfigScope
```

**Variants**: `Default`, `Coverage`

**Derives**: Clone, Copy, Debug

Which scope of the config a command reads its settings from.
Currently only affects file-glob selection.

---

## ConfigSource

`enum` · `pyrefly_config::config::ConfigSource`

```rust
enum ConfigSource
```

**Variants**: `File`, `PythonToolMarker`, `Marker`, `FailedParse`, `Synthetic`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn root_from_file(&self) -> Option<&Path>
```

Where did this config come from?

---

## FallbackSearchPath

`enum` · `pyrefly_config::config::FallbackSearchPath`

```rust
enum FallbackSearchPath
```

**Variants**: `Explicit`, `DirectoryRelative`, `Empty`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn for_directory(&self, directory: Option<&Path>) -> Arc<Vec<PathBuf>>
fn is_empty(&self) -> bool
fn repr_for_directory(&self, directory: Option<&Path>) -> String
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A struct for getting, storing, and evaluating fallback search paths. A fallback
search path is a search path consisting of ancestor paths from a start path
(usually some Python file) up to and including an end directory, which is usually
the filesystem root (`/`), but can also be the config.

---

## ImportLookupPathPart

`enum` · `pyrefly_config::config::ImportLookupPathPart`

```rust
enum ImportLookupPathPart<'a>
```

**Variants**: `SearchPathFromArgs`, `SearchPathFromFile`, `ImportRoot`, `FallbackSearchPath`, `SitePackagePath`, `InterpreterSitePackagePath`, `BuildSystem`

**Implements**: `core::fmt::Display`

**Methods** (1)

```rust
fn is_empty(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## OutputFormat

`enum` · `pyrefly_config::config::OutputFormat`

```rust
enum OutputFormat
```

**Variants**: `MinText`, `FullText`, `FullTextWithGithub`, `Json`, `Github`, `JunitXml`, `CodeClimate`, `Sarif`, `OmitErrors`

**Implements**: `clap_builder::derive::ValueEnum`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `clap_builder::derive::ValueEnum`**

```rust
fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>
fn value_variants<'a>() -> &'a [Self]
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## ProjectLayout

`enum` · `pyrefly_config::config::ProjectLayout`

```rust
enum ProjectLayout
```

**Variants**: `Flat`, `Src`, `Parent`

**Derives**: Default

**Methods** (1)

```rust
fn new(project_root: &Path) -> Self
```

Where the importable Python code in a project lives. There are two common Python project layouts, src and flat.
See: https://packaging.python.org/en/latest/discussions/src-layout-vs-flat-layout/#src-layout-vs-flat-layout

---

## SynthesizedPresetReason

`enum` · `pyrefly_config::config::SynthesizedPresetReason`

```rust
enum SynthesizedPresetReason
```

**Variants**: `NoNearbyConfig`, `Migrated`, `UserOverride`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

Why a `ConfigFile` was synthesized rather than loaded from a real config
on disk. Set by `resolve_unconfigured_config` and read by the LSP status
bar and the CLI upsell to explain to the user how Pyrefly chose its
behavior in the absence of a `pyrefly.toml` / `[tool.pyrefly]` section.

---

## toml_error_span

`function` · `pyrefly_config::config::toml_error_span`

```rust
fn toml_error_span<T: DeserializeOwned>(config_str: &str, err: &anyhow::Error) -> Option<std::ops::Range<usize>>
```

The source span of the value responsible for a failed TOML parse of `T`, if
we can determine it.

`toml_edit` attaches a span to syntax errors and to type errors on plain
top-level fields, but drops it for values nested inside `#[serde(flatten)]`
structs -- which is where nearly every pyrefly setting lives (see
[`ConfigBase`]). Rather than hand-write a check for each setting, we recover
the span generically by treating the real parser as the source of truth:
re-run it with one leaf value removed at a time, and blame the value whose
removal makes parsing succeed. This stays correct as settings are added or
renamed, and only runs once parsing has already failed on a (tiny) config,
so the repeated re-parses are negligible. If probing can't pin down a value
(e.g. a syntax error means the document doesn't even parse), we fall back to
whatever span the parser reported directly.

---

## validate_path

`function` · `pyrefly_config::config::validate_path`

```rust
fn validate_path(path: &std::path::Path) -> anyhow::Result<()>
```

Returns an error if the path is definitely invalid.

---

## GENERATED_FILE_CONFIG_OVERRIDE

`static` · `pyrefly_config::config::GENERATED_FILE_CONFIG_OVERRIDE`

```rust
static GENERATED_FILE_CONFIG_OVERRIDE: std::sync::LazyLock<pyrefly_util::lock::RwLock<starlark_map::small_map::SmallMap<pyrefly_util::interned_path::InternedPath, pyrefly_util::arc_id::ArcId<ConfigFile>>>>
```

---

## ConfigFile

`struct` · `pyrefly_config::config::ConfigFile`

```rust
struct ConfigFile
```

**Fields**: `source`, `required_version`, `project_includes`, `project_excludes`, `disable_project_excludes_heuristics`, `search_path_from_args`, `search_path_from_file`, `import_root`, `fallback_search_path`, `disable_search_path_heuristics`, `enable_fallback_search_path`, `typeshed_path`, `baseline`, `baseline_error_level`, `baseline_matching_mode`, `baseline_format`, `output_format`, `interpreters`, `python_environment`, `preset`, `root`, `sub_configs`, `coverage`, `use_ignore_files`, `build_system`, `source_db`, `target_configs`, `min_severity`, `skip_lsp_config_indexing`, `extra_file_extensions`, `synthesized_preset_reason`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq

**Methods** (43)

```rust
fn check_unannotated_defs(&self, path: &Path) -> bool
fn configure(&mut self) -> Vec<ConfigError>
fn configure_at(&mut self, project_root: Option<&Path>) -> Vec<ConfigError>
fn default_project_includes() -> Globs
fn default_true() -> bool
fn disable_type_errors_in_ide(&self, path: &Path) -> bool
fn enabled_ignores(&self, path: &Path) -> Cow<'_, SmallSet<Tool>>
fn errors(&self, path: &Path) -> Cow<'_, ErrorDisplayConfig>
fn explicit_search_path(&self) -> impl Iterator<Item = &PathBuf> + Clone
fn from_file(config_path: &Path) -> (ConfigFile, Vec<ConfigError>)
fn from_real_config_file(&self) -> bool
fn get_error_config(&self, path: &Path) -> ErrorConfig<'_>
fn get_filtered_globs(&self, custom_excludes: Option<Globs>, scope: ConfigScope) -> FilteredGlobs
fn get_paths_to_watch(configs: &SmallSet<ArcId<ConfigFile>>) -> SmallSet<WatchPattern>
fn get_sys_info(&self) -> SysInfo
fn handle_from_module_path(&self, module_path: ModulePath) -> Handle
fn has_extra_file_extensions(&self) -> bool
fn heuristic_search_path(&self) -> impl Iterator<Item = &PathBuf> + Clone
fn ignore_missing_imports(&self, path: Option<&Path>, module: ModuleName) -> bool
fn includes(&self, scope: ConfigScope) -> &Globs
fn infer_return_types(&self, path: &Path) -> InferReturnTypes
fn infer_with_first_use(&self, path: &Path) -> bool
fn init_at_root(root: &Path, layout: &ProjectLayout, fallback: bool) -> Self
fn legacy_overload_expansion(&self, path: &Path) -> bool
fn parse_config(config_str: &str) -> anyhow::Result<ConfigFile>
fn python_platform(&self) -> &PythonPlatform
fn python_version(&self) -> PythonVersion
fn query_source_db(configs_to_files: &SmallMap<ArcId<ConfigFile>, SmallSet<ModulePath>>, force: bool, telemetry: Option<SubTaskTelemetry<'_>>) -> SourceDbQueryOutcome
fn recursion_limit_config(&self) -> Option<RecursionLimitConfig>
fn replace_imports_with_any(&self, path: Option<&Path>, module: ModuleName) -> bool
fn replace_untyped_imports_with_any(&self, path: Option<&Path>, module: ModuleName) -> bool
fn required_project_excludes() -> Globs
fn rewrite_with_path_to_config(&mut self, config_root: &Path)
fn search_path(&self) -> impl Iterator<Item = &PathBuf> + Clone
fn site_package_path(&self) -> impl Iterator<Item = &PathBuf> + Clone
fn spec_compliant_overloads(&self, path: &Path) -> bool
fn strict_callable_subtyping(&self, path: &Path) -> bool
fn strict_partial_subtyping(&self, path: &Path) -> bool
fn structured_import_lookup_path<'a>(&'a self, origin: Option<&'a Path>) -> Vec<ImportLookupPathPart<'a>>
fn treat_all_caps_as_final(&self, path: &Path) -> bool
fn type_ignore_unknown_tag_behavior(&self, path: &Path) -> TypeIgnoreUnknownTagBehavior
fn typeshed_stdlib_path(&self) -> Option<PathBuf>
fn write_to_toml_in_directory(&self, directory: &Path) -> Result<()>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## CoverageConfig

`struct` · `pyrefly_config::config::CoverageConfig`

```rust
struct CoverageConfig
```

**Fields**: `includes`, `excludes`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Config overrides for the `pyrefly coverage` commands.

---

## DirectoryRelativeFallbackSearchPathCache

`struct` · `pyrefly_config::config::DirectoryRelativeFallbackSearchPathCache`

```rust
struct DirectoryRelativeFallbackSearchPathCache
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn clear(&self)
fn get_ancestors(&self, path: &Path) -> Arc<Vec<PathBuf>>
fn new(up_to: Option<PathBuf>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A cache for managing and producing a fallback search path from
some directory up to and including a root (`up_to`, which is usually a
config directory or filesystem root if none is provided).
The fallback search path consists of a given directory and its ancestors
up to `up_to` or `/`.

---

## SourceDbQueryOutcome

`struct` · `pyrefly_config::config::SourceDbQueryOutcome`

```rust
struct SourceDbQueryOutcome
```

**Fields**: `reloaded`, `stats`, `error`

The result of requerying every live source database for a set of configs.
See [`ConfigFile::query_source_db`].

---

## SubConfig

`struct` · `pyrefly_config::config::SubConfig`

```rust
struct SubConfig
```

**Fields**: `matches`, `settings`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---
