# `ruff`

Crate `ruff` · 3 public items · structured records in [`model/ruff.json`](../model/ruff.json)

## ExitStatus

`enum` · `ruff::ExitStatus`

```rust
enum ExitStatus
```

**Variants**: `Success`, `Failure`, `Error`

**Derives**: Clone, Copy

---

## check

`function` · `ruff::check`

```rust
fn check(args: args::CheckCommand, global_options: args::GlobalConfigArgs) -> anyhow::Result<ExitStatus>
```

---

## run

`function` · `ruff::run`

```rust
fn run(_: args::Args) -> anyhow::Result<ExitStatus>
```

---
