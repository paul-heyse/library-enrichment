# `ruff_python_stdlib::identifiers`

Crate `ruff_python_stdlib` · 4 public items · structured records in [`model/ruff_python_stdlib.identifiers.json`](../model/ruff_python_stdlib.identifiers.json)

## is_identifier

`function` · `ruff_python_stdlib::identifiers::is_identifier`

```rust
fn is_identifier(name: &str) -> bool
```

Returns `true` if a string is a valid Python identifier (e.g., variable
name).

---

## is_mangled_private

`function` · `ruff_python_stdlib::identifiers::is_mangled_private`

```rust
fn is_mangled_private(id: &str) -> bool
```

Returns `true` if a string is a private identifier, such that, when the
identifier is defined in a class definition, it will be mangled prior to
code generation.

See: <https://docs.python.org/3/reference/expressions.html#private-name-mangling>.

---

## is_migration_name

`function` · `ruff_python_stdlib::identifiers::is_migration_name`

```rust
fn is_migration_name(name: &str) -> bool
```

Returns `true` if a string appears to be a valid migration file name (e.g., `0001_initial.py`).

---

## is_module_name

`function` · `ruff_python_stdlib::identifiers::is_module_name`

```rust
fn is_module_name(name: &str) -> bool
```

Returns `true` if a string is a PEP 8-compliant module name (i.e., consists of lowercase
letters, numbers, underscores, and is not a keyword).

---
