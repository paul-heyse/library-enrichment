# `pyrefly_config::environment::environment`

Crate `pyrefly_config` · 1 public items · structured records in [`model/pyrefly_config.environment.environment.json`](../model/pyrefly_config.environment.environment.json)

## PythonEnvironment

`struct` · `pyrefly_config::environment::environment::PythonEnvironment`

```rust
struct PythonEnvironment
```

**Fields**: `python_platform`, `python_version`, `site_package_path`, `interpreter_site_package_path`, `interpreter_stdlib_path`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn get_default_interpreter_env() -> PythonEnvironment
fn get_env_from_interpreter(interpreter: &Path) -> anyhow::Result<PythonEnvironment>
fn get_interpreter_env(interpreter: &Path) -> (PythonEnvironment, Option<anyhow::Error>)
fn override_empty(&mut self, other: Self)
fn set_empty_to_default(&mut self)
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

Values representing the environment of the Python interpreter.
These values are `None` by default, so we can tell if a config
overrode them, or if we should query a Python interpreter for
any missing values. We can't query a Python interpreter
on config parsing, since we also won't know if an executable
other than the first available on the path should be used (i.e.
should we always look at a venv/conda environment instead?)

---
