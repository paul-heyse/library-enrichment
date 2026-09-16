# `ruff_python_parser::parser::options`

Crate `ruff_python_parser` · 1 public items · structured records in [`model/ruff_python_parser.parser.options.json`](../model/ruff_python_parser.parser.options.json)

## ParseOptions

`struct` · `ruff_python_parser::parser::options::ParseOptions`

Also reachable as `ruff_python_parser::ParseOptions`

```rust
struct ParseOptions
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn target_version(&self) -> PythonVersion
fn with_target_version(self, target_version: PythonVersion) -> Self
```

**via `core::convert::From`**

```rust
fn from(mode: Mode) -> Self
fn from(source_type: PySourceType) -> Self
```

Options for controlling how a source file is parsed.

You can construct a [`ParseOptions`] directly from a [`Mode`]:

```
use ruff_python_parser::{Mode, ParseOptions};

let options = ParseOptions::from(Mode::Module);
```

or from a [`PySourceType`]

```
use ruff_python_ast::PySourceType;
use ruff_python_parser::ParseOptions;

let options = ParseOptions::from(PySourceType::Python);
```

---
