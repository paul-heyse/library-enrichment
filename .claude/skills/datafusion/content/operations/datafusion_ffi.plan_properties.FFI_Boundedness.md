# `datafusion_ffi::plan_properties::FFI_Boundedness`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.plan_properties.FFI_Boundedness.json).

<a id="op-632bd9d154154119531b7445"></a>
## FFI_Boundedness

`enum` · `datafusion_ffi::plan_properties::FFI_Boundedness` · datafusion-ffi 55.1.0

```rust
enum FFI_Boundedness
```

Source: `src/plan_properties.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI safe version of [`Boundedness`](../operations/datafusion_physical_plan.execution_plan.Boundedness.md#op-0bd02bea0eb0e3d05f074321).

<a id="op-cfa5093f3993a67de7434b76"></a>
## Bounded

`variant` · `datafusion_ffi::plan_properties::FFI_Boundedness::Bounded` · datafusion-ffi 55.1.0

```rust
Bounded
```

Source: `src/plan_properties.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e88ca3415cbeb230d21e1221"></a>
## Unbounded

`variant` · `datafusion_ffi::plan_properties::FFI_Boundedness::Unbounded` · datafusion-ffi 55.1.0

```rust
Unbounded
```

Source: `src/plan_properties.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56f65aa8d94431f7a817cb84"></a>
## clone

`function` · `datafusion_ffi::plan_properties::FFI_Boundedness::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> FFI_Boundedness
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::plan_properties::FFI_Boundedness", "path": "FFI_Boundedness"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 10], "end": [200, 15], "filename": "src/plan_properties.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/plan_properties.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-060a2c9b3571660af6e85c40"></a>
## from

`function` · `datafusion_ffi::plan_properties::FFI_Boundedness::from` · datafusion-ffi 55.1.0

```rust
fn from(value: Boundedness) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::plan_properties::FFI_Boundedness", "path": "FFI_Boundedness"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [217, 2], "filename": "src/plan_properties.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::Boundedness", "path": "Boundedness"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/plan_properties.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
