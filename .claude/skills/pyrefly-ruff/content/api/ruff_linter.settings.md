# `ruff_linter::settings`

Crate `ruff_linter` · 5 public items · structured records in [`model/ruff_linter.settings.json`](../model/ruff_linter.settings.json)

## DEFAULT_SELECTORS

`constant` · `ruff_linter::settings::DEFAULT_SELECTORS`

```rust
const DEFAULT_SELECTORS: &[RuleSelector] = _
```

---

## TASK_TAGS

`constant` · `ruff_linter::settings::TASK_TAGS`

```rust
const TASK_TAGS: &[&str] = _
```

---

## DUMMY_VARIABLE_RGX

`static` · `ruff_linter::settings::DUMMY_VARIABLE_RGX`

```rust
static DUMMY_VARIABLE_RGX: std::sync::LazyLock<regex::Regex>
```

---

## LinterSettings

`struct` · `ruff_linter::settings::LinterSettings`

```rust
struct LinterSettings
```

**Fields**: `exclude`, `extension`, `project_root`, `rules`, `per_file_ignores`, `fix_safety`, `unresolved_target_version`, `per_file_target_version`, `preview`, `explicit_preview_rules`, `allowed_confusables`, `builtins`, `dummy_variable_rgx`, `external`, `ignore_init_module_imports`, `logger_objects`, `namespace_packages`, `src`, `tab_size`, `line_length`, `task_tags`, `typing_modules`, `typing_extensions`, `future_annotations`, `flake8_annotations`, `flake8_bandit`, `flake8_boolean_trap`, `flake8_bugbear`, `flake8_builtins`, `flake8_comprehensions`, `flake8_copyright`, `flake8_errmsg`, `flake8_gettext`, `flake8_implicit_str_concat`, `flake8_import_conventions`, `flake8_pytest_style`, `flake8_quotes`, `flake8_self`, `flake8_tidy_imports`, `flake8_type_checking`, `flake8_unused_arguments`, `isort`, `mccabe`, `pep8_naming`, `pycodestyle`, `pydoclint`, `pydocstyle`, `pyflakes`, `pylint`, `pyupgrade`, `ruff`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Default

**Methods** (7)

```rust
fn for_rule(rule_code: Rule) -> Self
fn for_rules(rules: impl IntoIterator<Item = Rule>) -> Self
fn new(project_root: &Path) -> Self
fn resolve_target_version(&self, path: &Path) -> TargetVersion
fn with_external_rules(self, rules: &[&str]) -> Self
fn with_preview_mode(self) -> Self
fn with_target_version(self, target_version: PythonVersion) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---

## TargetVersion

`struct` · `ruff_linter::settings::TargetVersion`

```rust
struct TargetVersion
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn linter_version(&self) -> PythonVersion
fn parser_version(&self) -> PythonVersion
```

**via `core::convert::From`**

```rust
fn from(value: PythonVersion) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

A thin wrapper around `Option<PythonVersion>` to clarify the reason for different `unwrap`
calls in various places.

For example, we want to default to `PythonVersion::latest()` for parsing and detecting semantic
syntax errors because this will minimize version-related diagnostics when the Python version is
unset. In contrast, we want to default to `PythonVersion::default()` for lint rules. These
correspond to the [`TargetVersion::parser_version`] and [`TargetVersion::linter_version`]
methods, respectively.

---
