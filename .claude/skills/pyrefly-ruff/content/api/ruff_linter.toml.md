# `ruff_linter::toml`

Crate `ruff_linter` · 3 public items · structured records in [`model/ruff_linter.toml.json`](../model/ruff_linter.toml.json)

## lint_fix_toml

`function` · `ruff_linter::toml::lint_fix_toml`

```rust
fn lint_fix_toml<'a>(path: &std::path::Path, source: &'a str, settings: &settings::LinterSettings, source_type: ruff_python_ast::TomlSourceType, unsafe_fixes: settings::types::UnsafeFixes) -> TomlFixerResult<'a>
```

Generate [`Diagnostic`]s for a TOML configuration file, iteratively fixing until stable.

---

## lint_toml

`function` · `ruff_linter::toml::lint_toml`

```rust
fn lint_toml(path: &std::path::Path, contents: &str, settings: &settings::LinterSettings, source_type: ruff_python_ast::TomlSourceType) -> Vec<ruff_db::diagnostic::Diagnostic>
```

---

## TomlFixerResult

`struct` · `ruff_linter::toml::TomlFixerResult`

```rust
struct TomlFixerResult<'a>
```

**Fields**: `diagnostics`, `transformed`, `fixed`

---
