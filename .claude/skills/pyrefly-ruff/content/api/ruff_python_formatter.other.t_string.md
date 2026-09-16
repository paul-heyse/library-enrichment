# `ruff_python_formatter::other::t_string`

Crate `ruff_python_formatter` · 1 public items · structured records in [`model/ruff_python_formatter.other.t_string.json`](../model/ruff_python_formatter.other.t_string.json)

## FormatTString

`struct` · `ruff_python_formatter::other::t_string::FormatTString`

```rust
struct FormatTString
```

Formats a t-string which is part of a larger t-string expression.

For example, this would be used to format the t-string part in `"foo" t"bar {x}"`
or the standalone t-string in `t"foo {x} bar"`.

---
