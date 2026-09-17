# `ra_ap_project_model::toolchain_info::rustc_cfg`

Crate `ra_ap_project_model` · 1 public items · structured records in [`model/ra_ap_project_model.toolchain_info.rustc_cfg.json`](../model/ra_ap_project_model.toolchain_info.rustc_cfg.json)

## get

`function` · `ra_ap_project_model::toolchain_info::rustc_cfg::get`

```rust
fn get(config: toolchain_info::QueryConfig<'_>, target: Option<&str>, extra_env: &rustc_hash::FxHashMap<String, Option<String>>) -> Vec<cfg::CfgAtom>
```

Uses `rustc --print cfg` to fetch the builtin cfgs.

---
