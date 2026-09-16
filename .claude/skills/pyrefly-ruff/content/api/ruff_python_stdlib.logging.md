# `ruff_python_stdlib::logging`

Crate `ruff_python_stdlib` · 1 public items · structured records in [`model/ruff_python_stdlib.logging.json`](../model/ruff_python_stdlib.logging.json)

## LoggingLevel

`enum` · `ruff_python_stdlib::logging::LoggingLevel`

```rust
enum LoggingLevel
```

**Variants**: `Debug`, `Critical`, `Error`, `Exception`, `Info`, `Warn`, `Warning`

**Derives**: Clone, Copy, Debug

**Methods** (1)

```rust
fn from_attribute(level: &str) -> Option<Self>
```

---
