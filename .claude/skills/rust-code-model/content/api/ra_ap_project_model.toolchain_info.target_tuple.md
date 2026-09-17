# `ra_ap_project_model::toolchain_info::target_tuple`

Crate `ra_ap_project_model` · 1 public items · structured records in [`model/ra_ap_project_model.toolchain_info.target_tuple.json`](../model/ra_ap_project_model.toolchain_info.target_tuple.json)

## get

`function` · `ra_ap_project_model::toolchain_info::target_tuple::get`

```rust
fn get(config: toolchain_info::QueryConfig<'_>, target: Option<&str>, extra_env: &rustc_hash::FxHashMap<String, Option<String>>) -> anyhow::Result<Vec<String>>
```

For cargo, runs `cargo -Zunstable-options config get build.target` to get the configured project target(s).
For rustc, runs `rustc --print -vV` to get the host target.

---
