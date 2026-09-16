# `ruff_python_stdlib::builtins`

Crate `ruff_python_stdlib` · 6 public items · structured records in [`model/ruff_python_stdlib.builtins.json`](../model/ruff_python_stdlib.builtins.json)

## is_exception

`function` · `ruff_python_stdlib::builtins::is_exception`

```rust
fn is_exception(name: &str, minor_version: u8) -> bool
```

Returns `true` if the given name is that of a builtin exception.

See: <https://docs.python.org/3/library/exceptions.html#exception-hierarchy>

---

## is_iterator

`function` · `ruff_python_stdlib::builtins::is_iterator`

```rust
fn is_iterator(name: &str) -> bool
```

Returns `true` if the given name is that of a Python builtin iterator.

---

## is_python_builtin

`function` · `ruff_python_stdlib::builtins::is_python_builtin`

```rust
fn is_python_builtin(name: &str, minor_version: u8, is_notebook: bool) -> bool
```

Returns `true` if the given name is that of a Python builtin.

Intended to be kept in sync with [`python_builtins`].

---

## python_builtins

`function` · `ruff_python_stdlib::builtins::python_builtins`

```rust
fn python_builtins(minor_version: u8, is_notebook: bool) -> impl Iterator<Item = &'static str>
```

Return the list of builtins for the given Python minor version.

Intended to be kept in sync with [`is_python_builtin`].

---

## python_magic_globals

`function` · `ruff_python_stdlib::builtins::python_magic_globals`

```rust
fn python_magic_globals(minor_version: u8) -> impl Iterator<Item = &'static str>
```

Return the list of magic globals for the given Python minor version.

---

## version_builtin_was_added

`function` · `ruff_python_stdlib::builtins::version_builtin_was_added`

```rust
fn version_builtin_was_added(name: &str) -> Option<u8>
```

Return `Some(version)`, where `version` corresponds to the Python minor version
in which the builtin was added

---
