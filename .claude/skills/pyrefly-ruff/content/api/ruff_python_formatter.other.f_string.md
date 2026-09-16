# `ruff_python_formatter::other::f_string`

Crate `ruff_python_formatter` · 1 public items · structured records in [`model/ruff_python_formatter.other.f_string.json`](../model/ruff_python_formatter.other.f_string.json)

## FormatFString

`struct` · `ruff_python_formatter::other::f_string::FormatFString`

```rust
struct FormatFString
```

Formats an f-string which is part of a larger f-string expression.

For example, this would be used to format the f-string part in `"foo" f"bar {x}"`
or the standalone f-string in `f"foo {x} bar"`.

---
