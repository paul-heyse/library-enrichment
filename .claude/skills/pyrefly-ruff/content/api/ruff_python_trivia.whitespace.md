# `ruff_python_trivia::whitespace`

Crate `ruff_python_trivia` · 9 public items · structured records in [`model/ruff_python_trivia.whitespace.json`](../model/ruff_python_trivia.whitespace.json)

## expand_tabs

`function` · `ruff_python_trivia::whitespace::expand_tabs`

Also reachable as `ruff_python_trivia::expand_tabs`

```rust
fn expand_tabs(source: &str) -> std::borrow::Cow<'_, str>
```

Expands tabs to the next eight-column tab stop, matching Python's `str.expandtabs`.

---

## has_leading_content

`function` · `ruff_python_trivia::whitespace::has_leading_content`

Also reachable as `ruff_python_trivia::has_leading_content`

```rust
fn has_leading_content(offset: ruff_text_size::TextSize, source: &str) -> bool
```

Return `true` if the node starting the given [`TextSize`] has leading content.

---

## has_trailing_content

`function` · `ruff_python_trivia::whitespace::has_trailing_content`

Also reachable as `ruff_python_trivia::has_trailing_content`

```rust
fn has_trailing_content(offset: ruff_text_size::TextSize, source: &str) -> bool
```

Return `true` if the node ending at the given [`TextSize`] has trailing content.

---

## indentation_at_offset

`function` · `ruff_python_trivia::whitespace::indentation_at_offset`

Also reachable as `ruff_python_trivia::indentation_at_offset`

```rust
fn indentation_at_offset(offset: ruff_text_size::TextSize, source: &str) -> Option<&str>
```

Extract the leading indentation from a line.

---

## is_python_whitespace

`function` · `ruff_python_trivia::whitespace::is_python_whitespace`

Also reachable as `ruff_python_trivia::is_python_whitespace`

```rust
const fn is_python_whitespace(c: char) -> bool
```

Returns `true` for [whitespace](https://docs.python.org/3/reference/lexical_analysis.html#whitespace-between-tokens)
characters.

---

## leading_indentation

`function` · `ruff_python_trivia::whitespace::leading_indentation`

Also reachable as `ruff_python_trivia::leading_indentation`

```rust
fn leading_indentation(line: &str) -> &str
```

Extract the leading indentation from a line.

---

## tab_offset

`function` · `ruff_python_trivia::whitespace::tab_offset`

Also reachable as `ruff_python_trivia::tab_offset`

```rust
const fn tab_offset(column: usize, tab_size: usize) -> usize
```

Returns the number of columns from `column` to the next tab stop.

---

## tab_offset_u32

`function` · `ruff_python_trivia::whitespace::tab_offset_u32`

Also reachable as `ruff_python_trivia::tab_offset_u32`

```rust
const fn tab_offset_u32(column: u32, tab_size: u32) -> u32
```

Returns the number of columns from `column` to the next tab stop using `u32` values.

---

## PythonWhitespace

`trait` · `ruff_python_trivia::whitespace::PythonWhitespace`

Also reachable as `ruff_python_trivia::PythonWhitespace`

```rust
trait PythonWhitespace
```

**Methods** (3)

```rust
fn trim_whitespace(&self) -> &Self
fn trim_whitespace_end(&self) -> &Self
fn trim_whitespace_start(&self) -> &Self
```

---
