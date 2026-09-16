# `ruff_workspace::settings`

Crate `ruff_workspace` · 4 public items · structured records in [`model/ruff_workspace.settings.json`](../model/ruff_workspace.settings.json)

## LineEnding

`enum` · `ruff_workspace::settings::LineEnding`

```rust
enum LineEnding
```

**Variants**: `Auto`, `Lf`, `CrLf`, `Native`

---

## FileResolverSettings

`struct` · `ruff_workspace::settings::FileResolverSettings`

Also reachable as `ruff_workspace::FileResolverSettings`

```rust
struct FileResolverSettings
```

**Fields**: `exclude`, `extend_exclude`, `force_exclude`, `include`, `extend_include`, `respect_gitignore`, `project_root`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---

## FormatterSettings

`struct` · `ruff_workspace::settings::FormatterSettings`

Also reachable as `ruff_workspace::FormatterSettings`

```rust
struct FormatterSettings
```

**Fields**: `exclude`, `extension`, `preview`, `unresolved_target_version`, `per_file_target_version`, `line_width`, `indent_style`, `indent_width`, `quote_style`, `nested_string_quote_style`, `magic_trailing_comma`, `line_ending`, `docstring_code_format`, `docstring_code_line_width`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn resolve_target_version(&self, path: &Path) -> PythonVersion
fn to_format_options(&self, source_type: PySourceType, source: &str, path: Option<&Path>) -> PyFormatOptions
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---

## Settings

`struct` · `ruff_workspace::settings::Settings`

Also reachable as `ruff_workspace::Settings`

```rust
struct Settings
```

**Fields**: `cache_dir`, `fix`, `fix_only`, `unsafe_fixes`, `output_format`, `output_prefer_rule_codes`, `show_fixes`, `file_resolver`, `linter`, `formatter`, `analyze`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Debug, Default

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---
