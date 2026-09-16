# `ruff_workspace::configuration`

Crate `ruff_workspace` · 6 public items · structured records in [`model/ruff_workspace.configuration.json`](../model/ruff_workspace.configuration.json)

## RuleSelectorKind

`enum` · `ruff_workspace::configuration::RuleSelectorKind`

```rust
enum RuleSelectorKind
```

**Variants**: `Enable`, `Disable`, `Modify`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn is_disable(&self) -> bool
const fn is_enable(&self) -> bool
const fn is_modify(&self) -> bool
```

---

## AnalyzeConfiguration

`struct` · `ruff_workspace::configuration::AnalyzeConfiguration`

```rust
struct AnalyzeConfiguration
```

**Fields**: `exclude`, `preview`, `direction`, `detect_string_imports`, `string_imports_min_dots`, `include_dependencies`, `type_checking_imports`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn from_options(options: AnalyzeOptions, project_root: &Path) -> Result<Self>
```

---

## Configuration

`struct` · `ruff_workspace::configuration::Configuration`

```rust
struct Configuration
```

**Fields**: `cache_dir`, `extend`, `fix`, `fix_only`, `unsafe_fixes`, `output_format`, `output_prefer_rule_codes`, `preview`, `required_version`, `extension`, `show_fixes`, `exclude`, `extend_exclude`, `extend_include`, `force_exclude`, `include`, `respect_gitignore`, `builtins`, `namespace_packages`, `src`, `target_version`, `per_file_target_version`, `line_length`, `indent_width`, `lint`, `format`, `analyze`

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn combine(self, config: Self) -> Self
fn from_options(options: Options, path: Option<&Path>, project_root: &Path) -> Result<Self>
fn into_settings(self, project_root: &Path) -> Result<Settings>
```

---

## FormatConfiguration

`struct` · `ruff_workspace::configuration::FormatConfiguration`

```rust
struct FormatConfiguration
```

**Fields**: `exclude`, `preview`, `extension`, `indent_style`, `quote_style`, `nested_string_quote_style`, `magic_trailing_comma`, `line_ending`, `docstring_code_format`, `docstring_code_line_width`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn from_options(options: FormatOptions, project_root: &Path) -> Result<Self>
```

---

## LintConfiguration

`struct` · `ruff_workspace::configuration::LintConfiguration`

```rust
struct LintConfiguration
```

**Fields**: `exclude`, `preview`, `extend_per_file_ignores`, `per_file_ignores`, `rule_selections`, `explicit_preview_rules`, `extend_unsafe_fixes`, `extend_safe_fixes`, `allowed_confusables`, `dummy_variable_rgx`, `external`, `ignore_init_module_imports`, `logger_objects`, `task_tags`, `typing_modules`, `typing_extensions`, `future_annotations`, `flake8_annotations`, `flake8_bandit`, `flake8_boolean_trap`, `flake8_bugbear`, `flake8_builtins`, `flake8_comprehensions`, `flake8_copyright`, `flake8_errmsg`, `flake8_gettext`, `flake8_implicit_str_concat`, `flake8_import_conventions`, `flake8_pytest_style`, `flake8_quotes`, `flake8_self`, `flake8_tidy_imports`, `flake8_type_checking`, `flake8_unused_arguments`, `isort`, `mccabe`, `pep8_naming`, `pycodestyle`, `pydoclint`, `pydocstyle`, `pyflakes`, `pylint`, `pyupgrade`, `ruff`

**Derives**: Clone, Debug, Default

---

## RuleSelection

`struct` · `ruff_workspace::configuration::RuleSelection`

```rust
struct RuleSelection
```

**Fields**: `select`, `ignore`, `extend_select`, `fixable`, `unfixable`, `extend_fixable`

**Derives**: Clone, Debug, Default

---
