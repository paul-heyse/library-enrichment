# `ra_ap_project_model::toolchain_info::target_data`

Crate `ra_ap_project_model` · 3 public items · structured records in [`model/ra_ap_project_model.toolchain_info.target_data.json`](../model/ra_ap_project_model.toolchain_info.target_data.json)

## Arch

`enum` · `ra_ap_project_model::toolchain_info::target_data::Arch`

```rust
enum Arch
```

**Variants**: `Wasm32`, `Wasm64`, `Other`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---

## get

`function` · `ra_ap_project_model::toolchain_info::target_data::get`

```rust
fn get(config: toolchain_info::QueryConfig<'_>, target: Option<&str>, extra_env: &rustc_hash::FxHashMap<String, Option<String>>, version: Option<&semver::Version>) -> anyhow::Result<target::TargetData>
```

Uses `rustc --print target-spec-json`.

---

## TargetSpec

`struct` · `ra_ap_project_model::toolchain_info::target_data::TargetSpec`

```rust
struct TargetSpec
```

**Fields**: `data_layout`, `arch`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---
