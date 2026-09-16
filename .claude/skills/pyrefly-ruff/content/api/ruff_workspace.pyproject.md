# `ruff_workspace::pyproject`

Crate `ruff_workspace` · 5 public items · structured records in [`model/ruff_workspace.pyproject.json`](../model/ruff_workspace.pyproject.json)

## find_fallback_target_version

`function` · `ruff_workspace::pyproject::find_fallback_target_version`

```rust
fn find_fallback_target_version<P: AsRef<std::path::Path>>(path: P) -> Option<ruff_linter::settings::types::PythonVersion>
```

Derive target version from `required-version` in `pyproject.toml`, if
such a file exists in an ancestor directory.

---

## find_settings_toml

`function` · `ruff_workspace::pyproject::find_settings_toml`

```rust
fn find_settings_toml<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Option<std::path::PathBuf>>
```

Find the path to the `pyproject.toml` or `ruff.toml` file, if such a file
exists.

---

## find_user_settings_toml

`function` · `ruff_workspace::pyproject::find_user_settings_toml`

```rust
fn find_user_settings_toml() -> Option<std::path::PathBuf>
```

Find the path to the user-specific `pyproject.toml` or `ruff.toml`, if it
exists.

---

## settings_toml

`function` · `ruff_workspace::pyproject::settings_toml`

```rust
fn settings_toml<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Option<std::path::PathBuf>>
```

Return the path to the `pyproject.toml` or `ruff.toml` file in a given
directory.

---

## Pyproject

`struct` · `ruff_workspace::pyproject::Pyproject`

```rust
struct Pyproject
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---
