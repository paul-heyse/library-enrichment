# `ruff_python_formatter`

Crate `ruff_python_formatter` · 5 public items · structured records in [`model/ruff_python_formatter.json`](../model/ruff_python_formatter.json)

## FormatModuleError

`enum` · `ruff_python_formatter::FormatModuleError`

```rust
enum FormatModuleError
```

**Variants**: `ParseError`, `FormatError`, `PrintError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn range(&self) -> Option<TextRange>
```

**via `core::convert::From`**

```rust
fn from(source: PrintError) -> Self
fn from(source: ParseError) -> Self
fn from(source: FormatError) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## format_module_ast

`function` · `ruff_python_formatter::format_module_ast`

```rust
fn format_module_ast<'a>(parsed: &'a ruff_python_parser::Parsed<ruff_python_ast::Mod>, trivia: &'a ruff_python_trivia::TriviaRanges, source: &'a str, options: PyFormatOptions) -> FormatResult<ruff_formatter::Formatted<PyFormatContext<'a>>>
```

---

## format_module_source

`function` · `ruff_python_formatter::format_module_source`

```rust
fn format_module_source(source: &str, options: PyFormatOptions) -> Result<ruff_formatter::Printed, FormatModuleError>
```

---

## formatted_file

`function` · `ruff_python_formatter::formatted_file`

```rust
fn formatted_file(db: &dyn Db, file: ruff_db::files::File) -> Result<Option<String>, FormatModuleError>
```

---

## pretty_comments

`function` · `ruff_python_formatter::pretty_comments`

```rust
fn pretty_comments(module: &ruff_python_ast::Mod, trivia: &ruff_python_trivia::TriviaRanges, source: &str) -> String
```

Public function for generating a printable string of the debug comments.

---
