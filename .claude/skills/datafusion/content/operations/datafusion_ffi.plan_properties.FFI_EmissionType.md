# `datafusion_ffi::plan_properties::FFI_EmissionType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.plan_properties.FFI_EmissionType.json).

<a id="op-0fd47b0fb1b3b10d30314858"></a>
## FFI_EmissionType

`enum` · `datafusion_ffi::plan_properties::FFI_EmissionType` · datafusion-ffi 55.1.0

```rust
enum FFI_EmissionType
```

Source: `src/plan_properties.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI safe version of [`EmissionType`](../operations/datafusion_physical_plan.execution_plan.EmissionType.md#op-05c5e3937550b6fc12eae8da).

<a id="op-b929bd61aefe451124bb7d89"></a>
## Both

`variant` · `datafusion_ffi::plan_properties::FFI_EmissionType::Both` · datafusion-ffi 55.1.0

```rust
Both
```

Source: `src/plan_properties.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14e048c74e8488256fbbdbbb"></a>
## Final

`variant` · `datafusion_ffi::plan_properties::FFI_EmissionType::Final` · datafusion-ffi 55.1.0

```rust
Final
```

Source: `src/plan_properties.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f41dfe7c48e878340e9037bd"></a>
## Incremental

`variant` · `datafusion_ffi::plan_properties::FFI_EmissionType::Incremental` · datafusion-ffi 55.1.0

```rust
Incremental
```

Source: `src/plan_properties.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aae8a1f4392d8c4a77d3aa3e"></a>
## clone

`function` · `datafusion_ffi::plan_properties::FFI_EmissionType::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> FFI_EmissionType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::plan_properties::FFI_EmissionType", "path": "FFI_EmissionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 10], "end": [235, 15], "filename": "src/plan_properties.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/plan_properties.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d361faffafb3a27d1173937"></a>
## from

`function` · `datafusion_ffi::plan_properties::FFI_EmissionType::from` · datafusion-ffi 55.1.0

```rust
fn from(value: EmissionType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::plan_properties::FFI_EmissionType", "path": "FFI_EmissionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [250, 2], "filename": "src/plan_properties.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::EmissionType", "path": "EmissionType"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/plan_properties.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
