# `ruff_python_formatter::context`

Crate `ruff_python_formatter` · 1 public items · structured records in [`model/ruff_python_formatter.context.json`](../model/ruff_python_formatter.context.json)

## PyFormatContext

`struct` · `ruff_python_formatter::context::PyFormatContext`

Also reachable as `ruff_python_formatter::PyFormatContext`

```rust
struct PyFormatContext<'a>
```

**Implements**: `ruff_formatter::FormatContext`

**Derives**: Debug

**via `ruff_formatter::FormatContext`**

```rust
fn options(&self) -> &Self::Options
fn source_code(&self) -> SourceCode<'_>
```

---
