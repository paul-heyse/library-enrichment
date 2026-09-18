# `datafusion_ffi::plan_properties::FFI_PlanProperties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.plan_properties.FFI_PlanProperties.json).

<a id="op-f386a991ea104a20200e7994"></a>
## FFI_PlanProperties

`struct` · `datafusion_ffi::plan_properties::FFI_PlanProperties` · datafusion-ffi 55.1.0

```rust
struct FFI_PlanProperties
```

Source: `src/plan_properties.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`PlanProperties`](../operations/datafusion_physical_plan.execution_plan.PlanProperties.md#op-5e820cca2acc2c7b69a4a0b3) across FFI boundaries.

<a id="op-d30dfa85c4f7eca9ffc207ba"></a>
## boundedness

`struct_field` · `datafusion_ffi::plan_properties::FFI_PlanProperties::boundedness` · datafusion-ffi 55.1.0

```rust
boundedness: unsafe fn(&Self) -> FFI_Boundedness
```

Source: `src/plan_properties.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Indicate boundedness of the plan and its memory requirements.

<a id="op-fce8a4133109c3639d2078cb"></a>
## drop

`function` · `datafusion_ffi::plan_properties::FFI_PlanProperties::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::plan_properties::FFI_PlanProperties", "path": "FFI_PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [131, 2], "filename": "src/plan_properties.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/plan_properties.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d365d95c54e54faa3726cc2"></a>
## emission_type

`struct_field` · `datafusion_ffi::plan_properties::FFI_PlanProperties::emission_type` · datafusion-ffi 55.1.0

```rust
emission_type: unsafe fn(&Self) -> FFI_EmissionType
```

Source: `src/plan_properties.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the emission type of the plan.

<a id="op-f04e1dcdc0dc0bb37234a93c"></a>
## fmt

`function` · `datafusion_ffi::plan_properties::FFI_PlanProperties::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::plan_properties::FFI_PlanProperties", "path": "FFI_PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/plan_properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/plan_properties.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20e410a25b4a5d7e9640b0e6"></a>
## from

`function` · `datafusion_ffi::plan_properties::FFI_PlanProperties::from` · datafusion-ffi 55.1.0

```rust
fn from(props: &PlanProperties) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::plan_properties::FFI_PlanProperties", "path": "FFI_PlanProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [150, 2], "filename": "src/plan_properties.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::PlanProperties", "path": "PlanProperties"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/plan_properties.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eee9b6ccdadfe77864580f76"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::plan_properties::FFI_PlanProperties::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/plan_properties.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc) and
the crate's `README.md` for more information.

<a id="op-1ac4bc771bd9b3e8c81d9aa1"></a>
## output_ordering

`struct_field` · `datafusion_ffi::plan_properties::FFI_PlanProperties::output_ordering` · datafusion-ffi 55.1.0

```rust
output_ordering: unsafe fn(&Self) -> util::FFI_Option<stabby::vec::Vec<physical_expr::sort::FFI_PhysicalSortExpr>>
```

Source: `src/plan_properties.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

The output ordering of the plan.

<a id="op-01d11f179f85948ce3082ed0"></a>
## output_partitioning

`struct_field` · `datafusion_ffi::plan_properties::FFI_PlanProperties::output_partitioning` · datafusion-ffi 55.1.0

```rust
output_partitioning: unsafe fn(&Self) -> physical_expr::partitioning::FFI_Partitioning
```

Source: `src/plan_properties.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

The output partitioning of the plan.

<a id="op-20ff05f675ad2cac656de86b"></a>
## private_data

`struct_field` · `datafusion_ffi::plan_properties::FFI_PlanProperties::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/plan_properties.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the plan.
The foreign library should never attempt to access this data.

<a id="op-8f3cc7cc50c134d1b96b3eac"></a>
## release

`struct_field` · `datafusion_ffi::plan_properties::FFI_PlanProperties::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/plan_properties.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-c913ccf0832c814cf931a563"></a>
## schema

`struct_field` · `datafusion_ffi::plan_properties::FFI_PlanProperties::schema` · datafusion-ffi 55.1.0

```rust
schema: unsafe fn(&Self) -> arrow_wrappers::WrappedSchema
```

Source: `src/plan_properties.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the schema of the plan.
