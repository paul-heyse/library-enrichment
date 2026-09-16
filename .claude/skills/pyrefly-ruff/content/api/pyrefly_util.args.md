# `pyrefly_util::args`

Crate `pyrefly_util` · 3 public items · structured records in [`model/pyrefly_util.args.json`](../model/pyrefly_util.args.json)

## clap_env

`function` · `pyrefly_util::args::clap_env`

```rust
fn clap_env(suffix: &str) -> String
```

---

## get_args_expanded

`function` · `pyrefly_util::args::get_args_expanded`

```rust
fn get_args_expanded(args: impl Iterator<Item = std::ffi::OsString>) -> anyhow::Result<Vec<std::ffi::OsString>>
```

Do `@` file expansion

---

## ENV_VAR_OVERRIDE_PREFIX

`static` · `pyrefly_util::args::ENV_VAR_OVERRIDE_PREFIX`

```rust
static ENV_VAR_OVERRIDE_PREFIX: &str
```

---
