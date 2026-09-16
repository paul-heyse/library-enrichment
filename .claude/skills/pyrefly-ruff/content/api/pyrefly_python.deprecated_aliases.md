# `pyrefly_python::deprecated_aliases`

Crate `pyrefly_python` · 1 public items · structured records in [`model/pyrefly_python.deprecated_aliases.json`](../model/pyrefly_python.deprecated_aliases.json)

## is_deprecated_stdlib_alias

`function` · `pyrefly_python::deprecated_aliases::is_deprecated_stdlib_alias`

```rust
fn is_deprecated_stdlib_alias(python_version: sys_info::PythonVersion, module_name: &str, name: &str) -> bool
```

Whether `typing.<name>` is a deprecated alias for a builtin or `collections.abc` type
on the given Python version.

These aliases are not marked with `@deprecated` in typeshed, but the `typing`
documentation steers users away from them:
- The generic aliases (`typing.List`, `typing.Dict`, `typing.Iterable`, ...) are
  superseded by their builtin / `collections.abc` counterparts (`list`, `dict`,
  `collections.abc.Iterable`, ...) once PEP 585 made those subscriptable in 3.9.
- `typing.Optional` / `typing.Union` are superseded by the `X | None` / `X | Y`
  syntax from PEP 604 in 3.10.

We use this to prefer the non-deprecated spelling in auto-imports and quick fixes.

---
