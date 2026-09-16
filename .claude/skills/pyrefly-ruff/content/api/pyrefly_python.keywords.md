# `pyrefly_python::keywords`

Crate `pyrefly_python` · 5 public items · structured records in [`model/pyrefly_python.keywords.json`](../model/pyrefly_python.keywords.json)

## get_expression_keywords

`function` · `pyrefly_python::keywords::get_expression_keywords`

```rust
fn get_expression_keywords(version: sys_info::PythonVersion) -> Vec<&'static str>
```

Returns the Python keywords that are valid in an expression position for the
specified Python version. This excludes statement-only keywords (e.g. `while`,
`try`, `def`) that would be invalid inside a nested expression such as a call
argument or the right-hand side of an assignment. `await` is the only
expression keyword added after the base set (Python 3.5+).

---

## get_keywords

`function` · `pyrefly_python::keywords::get_keywords`

```rust
fn get_keywords(version: sys_info::PythonVersion) -> Vec<&'static str>
```

Returns a Vec containing all Python keywords for the specified Python version.

---

## is_keyword_escaped_dir

`function` · `pyrefly_python::keywords::is_keyword_escaped_dir`

```rust
fn is_keyword_escaped_dir(name: &str) -> bool
```

Returns true if the given name is a Python keyword that may appear as an
escaped directory name in configerator repos (e.g. `if` → `if_`). This is a
deliberately narrow subset; use [`is_reserved_keyword`] to ask whether a name
is a keyword at all.

---

## is_reserved_keyword

`function` · `pyrefly_python::keywords::is_reserved_keyword`

```rust
fn is_reserved_keyword(name: &str) -> bool
```

Whether `name` is a reserved keyword, i.e. `keyword.iskeyword(name)`. This is
spelled out rather than derived from the lists above: those are grouped by
where a keyword may appear, and include the soft keywords (`match`, `case`,
`type`), which are contextual and remain usable as ordinary names.

Version-independent, unlike [`get_keywords`]: a name reserved in any supported
version is reserved here, so callers asking "may this be written as an
identifier?" get the conservative answer.

---

## is_valid_identifier

`function` · `pyrefly_python::keywords::is_valid_identifier`

```rust
fn is_valid_identifier(name: &str) -> bool
```

Whether `name` can be written in source as an identifier: a keyword argument
name, an attribute, or a class field. Equivalent to Python's
`name.isidentifier() and not keyword.iskeyword(name)`.

---
