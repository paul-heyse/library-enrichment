# `ruff_python_codegen`

Crate `ruff_python_codegen` · 1 public items · structured records in [`model/ruff_python_codegen.json`](../model/ruff_python_codegen.json)

## round_trip

`function` · `ruff_python_codegen::round_trip`

```rust
fn round_trip(code: &str) -> Result<String, ruff_python_parser::ParseError>
```

Run round-trip source code generation on a given Python code.

---
