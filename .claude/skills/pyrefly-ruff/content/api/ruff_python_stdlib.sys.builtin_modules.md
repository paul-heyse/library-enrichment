# `ruff_python_stdlib::sys::builtin_modules`

Crate `ruff_python_stdlib` · 1 public items · structured records in [`model/ruff_python_stdlib.sys.builtin_modules.json`](../model/ruff_python_stdlib.sys.builtin_modules.json)

## is_builtin_module

`function` · `ruff_python_stdlib::sys::builtin_modules::is_builtin_module`

Also reachable as `ruff_python_stdlib::sys::is_builtin_module`

```rust
fn is_builtin_module(minor_version: u8, module: &str) -> bool
```

Return `true` if `module` is a [builtin module] on the given
Python 3 version.

"Builtin modules" are modules that are compiled directly into the
Python interpreter. These can never be shadowed by first-party
modules; the normal rules of module resolution do not apply to these
modules.

[builtin module]: https://docs.python.org/3/library/sys.html#sys.builtin_module_names

---
