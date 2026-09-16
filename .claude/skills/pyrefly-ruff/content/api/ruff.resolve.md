# `ruff::resolve`

Crate `ruff` · 1 public items · structured records in [`model/ruff.resolve.json`](../model/ruff.resolve.json)

## resolve

`function` · `ruff::resolve::resolve`

```rust
fn resolve(config_arguments: &args::ConfigArguments, stdin_filename: Option<&std::path::Path>) -> anyhow::Result<ruff_workspace::resolver::PyprojectConfig>
```

Resolve the relevant settings strategy and defaults for the current
invocation.

---
