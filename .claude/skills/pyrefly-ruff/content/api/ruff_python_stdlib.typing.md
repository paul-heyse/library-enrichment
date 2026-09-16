# `ruff_python_stdlib::typing`

Crate `ruff_python_stdlib` · 14 public items · structured records in [`model/ruff_python_stdlib.typing.json`](../model/ruff_python_stdlib.typing.json)

## as_pep_585_generic

`function` · `ruff_python_stdlib::typing::as_pep_585_generic`

```rust
fn as_pep_585_generic(module: &str, member: &str) -> Option<(&'static str, &'static str)>
```

Given a typing member, returns the module and member name for a generic from the Python standard
library (e.g., `list` for `typing.List`), if such a generic was introduced by [PEP 585].

[PEP 585]: https://peps.python.org/pep-0585/

---

## is_immutable_generic_type

`function` · `ruff_python_stdlib::typing::is_immutable_generic_type`

```rust
fn is_immutable_generic_type(qualified_name: &[&str]) -> bool
```

Returns `true` if a call path represents that of an immutable, generic type from the Python
standard library (e.g. `tuple`).

---

## is_immutable_non_generic_type

`function` · `ruff_python_stdlib::typing::is_immutable_non_generic_type`

```rust
fn is_immutable_non_generic_type(qualified_name: &[&str]) -> bool
```

Returns `true` if a call path represents that of an immutable, non-generic type from the Python
standard library (e.g. `int` or `str`).

---

## is_immutable_return_type

`function` · `ruff_python_stdlib::typing::is_immutable_return_type`

```rust
fn is_immutable_return_type(qualified_name: &[&str]) -> bool
```

Returns `true` if a call path represents a function from the Python standard library that
returns a immutable value (e.g., `bool`).

---

## is_literal_member

`function` · `ruff_python_stdlib::typing::is_literal_member`

```rust
fn is_literal_member(member: &str) -> bool
```

Returns `true` if a name matches that of the `Literal` generic.

---

## is_mutable_return_type

`function` · `ruff_python_stdlib::typing::is_mutable_return_type`

```rust
fn is_mutable_return_type(qualified_name: &[&str]) -> bool
```

Returns `true` if a call path represents a function from the Python standard library that
returns a mutable value (e.g., `dict`).

---

## is_pep_593_generic_member

`function` · `ruff_python_stdlib::typing::is_pep_593_generic_member`

```rust
fn is_pep_593_generic_member(member: &str) -> bool
```

Returns `true` if a name matches that of a generic from [PEP 593] (e.g. `Annotated`).

See: <https://docs.python.org/3/library/typing.html>

[PEP 593]: https://peps.python.org/pep-0593/

---

## is_pep_593_generic_type

`function` · `ruff_python_stdlib::typing::is_pep_593_generic_type`

```rust
fn is_pep_593_generic_type(qualified_name: &[&str]) -> bool
```

Returns `true` if a call path is a [PEP 593] generic (e.g. `Annotated`).

See: <https://docs.python.org/3/library/typing.html>

[PEP 593]: https://peps.python.org/pep-0593/

---

## is_standard_library_generic

`function` · `ruff_python_stdlib::typing::is_standard_library_generic`

```rust
fn is_standard_library_generic(qualified_name: &[&str]) -> bool
```

Returns `true` if a call path is a generic from the Python standard library (e.g. `list`, which
can be used as `list[int]`).

See: <https://docs.python.org/3/library/typing.html>

---

## is_standard_library_generic_member

`function` · `ruff_python_stdlib::typing::is_standard_library_generic_member`

```rust
fn is_standard_library_generic_member(member: &str) -> bool
```

Returns `true` if a name matches that of a generic from the Python standard library (e.g.
`list` or `Set`).

See: <https://docs.python.org/3/library/typing.html>

---

## is_standard_library_literal

`function` · `ruff_python_stdlib::typing::is_standard_library_literal`

```rust
fn is_standard_library_literal(qualified_name: &[&str]) -> bool
```

Returns `true` if a call path is `Literal`.

---

## is_typed_dict

`function` · `ruff_python_stdlib::typing::is_typed_dict`

```rust
fn is_typed_dict(qualified_name: &[&str]) -> bool
```

---

## is_typed_dict_member

`function` · `ruff_python_stdlib::typing::is_typed_dict_member`

```rust
fn is_typed_dict_member(member: &str) -> bool
```

Returns `true` if a name matches that of `TypedDict`.

See: <https://docs.python.org/3/library/typing.html>

---

## simple_magic_return_type

`function` · `ruff_python_stdlib::typing::simple_magic_return_type`

```rust
fn simple_magic_return_type(method: &str) -> Option<&'static str>
```

Returns the expected return type for a magic method.

See: <https://github.com/JelleZijlstra/autotyping/blob/0adba5ba0eee33c1de4ad9d0c79acfd737321dd9/autotyping/autotyping.py#L69-L91>

---
