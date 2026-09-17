# `ra_ap_project_model::toolchain_info::version`

Crate `ra_ap_project_model` · 1 public items · structured records in [`model/ra_ap_project_model.toolchain_info.version.json`](../model/ra_ap_project_model.toolchain_info.version.json)

## get

`function` · `ra_ap_project_model::toolchain_info::version::get`

```rust
fn get(config: toolchain_info::QueryConfig<'_>, extra_env: &rustc_hash::FxHashMap<String, Option<String>>) -> Result<Option<semver::Version>, anyhow::Error>
```

---
