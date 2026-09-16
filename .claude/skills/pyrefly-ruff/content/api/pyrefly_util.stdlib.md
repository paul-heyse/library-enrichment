# `pyrefly_util::stdlib`

Crate `pyrefly_util` · 3 public items · structured records in [`model/pyrefly_util.stdlib.json`](../model/pyrefly_util.stdlib.json)

## get_stdlib_path_registry

`function` · `pyrefly_util::stdlib::get_stdlib_path_registry`

```rust
fn get_stdlib_path_registry() -> &'static std::sync::LazyLock<lock::RwLock<starlark_map::small_set::SmallSet<std::path::PathBuf>>>
```

Returns a reference to the stdlib path registry for reading.

---

## is_python_stdlib_file

`function` · `pyrefly_util::stdlib::is_python_stdlib_file`

```rust
fn is_python_stdlib_file(path: &std::path::Path) -> bool
```

Determines whether a file path belongs to the Python standard library.

This function checks if the given path is located within any of the configured
Python interpreter's standard library directories. It canonicalizes both the input
path and the stdlib paths for comparison to handle symlinks correctly.

---

## register_stdlib_paths

`function` · `pyrefly_util::stdlib::register_stdlib_paths`

```rust
fn register_stdlib_paths(paths: Vec<std::path::PathBuf>)
```

Registers stdlib paths discovered from a Python interpreter.

---
