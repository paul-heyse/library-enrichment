# `datafusion_ffi::physical_expr::FFI_PhysicalExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.physical_expr.FFI_PhysicalExpr.json).

<a id="op-0acc9367876e5b03f6e08a37"></a>
## FFI_PhysicalExpr

`struct` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr` · datafusion-ffi 55.1.0

```rust
struct FFI_PhysicalExpr
```

Source: `src/physical_expr/mod.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e976dc8e24477df28fb331b2"></a>
## children

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::children` · datafusion-ffi 55.1.0

```rust
children: unsafe fn(&Self) -> stabby::vec::Vec<FFI_PhysicalExpr>
```

Source: `src/physical_expr/mod.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ccfa88529a84eb8df5cca27"></a>
## clone

`function` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::FFI_PhysicalExpr", "path": "FFI_PhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [535, 1], "end": [539, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/physical_expr/mod.rs:536`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b2da444d636c84e4bf4f054"></a>
## clone

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/physical_expr/mod.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the provider of the execution plan. This should
only need to be called by the receiver of the plan.

<a id="op-572a8035961a8eb9cbb46e10"></a>
## data_type

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::data_type` · datafusion-ffi 55.1.0

```rust
data_type: unsafe fn(&Self, arrow_wrappers::WrappedSchema) -> util::FFI_Result<arrow_wrappers::WrappedSchema>
```

Source: `src/physical_expr/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0181f1a1e884835c02afb9e2"></a>
## display

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::display` · datafusion-ffi 55.1.0

```rust
display: unsafe fn(&Self) -> stabby::string::String
```

Source: `src/physical_expr/mod.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2973f36e624b86f682f6da8"></a>
## drop

`function` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::FFI_PhysicalExpr", "path": "FFI_PhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [458, 1], "end": [462, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/physical_expr/mod.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-928aabbdbe1e808236fcbf67"></a>
## evaluate

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::evaluate` · datafusion-ffi 55.1.0

```rust
evaluate: unsafe fn(&Self, arrow_wrappers::WrappedArray) -> util::FFI_Result<expr::columnar_value::FFI_ColumnarValue>
```

Source: `src/physical_expr/mod.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5aa300c2cf0d3ba6d0566e9e"></a>
## evaluate_bounds

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::evaluate_bounds` · datafusion-ffi 55.1.0

```rust
evaluate_bounds: unsafe fn(&Self, stabby::vec::Vec<expr::interval::FFI_Interval>) -> util::FFI_Result<expr::interval::FFI_Interval>
```

Source: `src/physical_expr/mod.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0917df35d41ae1aa3d9bbd89"></a>
## evaluate_selection

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::evaluate_selection` · datafusion-ffi 55.1.0

```rust
evaluate_selection: unsafe fn(&Self, arrow_wrappers::WrappedArray, arrow_wrappers::WrappedArray) -> util::FFI_Result<expr::columnar_value::FFI_ColumnarValue>
```

Source: `src/physical_expr/mod.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a439c054aa005c8f1d609630"></a>
## evaluate_statistics

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::evaluate_statistics` · datafusion-ffi 55.1.0

```rust
evaluate_statistics: unsafe fn(&Self, stabby::vec::Vec<expr::distribution::FFI_Distribution>) -> util::FFI_Result<expr::distribution::FFI_Distribution>
```

Source: `src/physical_expr/mod.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-caed29e11b183959b89dbc56"></a>
## expression_id

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::expression_id` · datafusion-ffi 55.1.0

```rust
expression_id: unsafe fn(&Self) -> util::FFI_Option<u64>
```

Source: `src/physical_expr/mod.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5b33c404bf6e644d0430755"></a>
## fmt

`function` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::FFI_PhysicalExpr", "path": "FFI_PhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 10], "end": [55, 15], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/physical_expr/mod.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59d79f2620970435b501dc93"></a>
## fmt_sql

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::fmt_sql` · datafusion-ffi 55.1.0

```rust
fmt_sql: unsafe fn(&Self) -> util::FFI_Result<stabby::string::String>
```

Source: `src/physical_expr/mod.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6cf77e5a9e8debffd68baa4"></a>
## from

`function` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::from` · datafusion-ffi 55.1.0

```rust
fn from(expr: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::FFI_PhysicalExpr", "path": "FFI_PhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [464, 1], "end": [500, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/physical_expr/mod.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Creates a new [`FFI_PhysicalExpr`](../operations/datafusion_ffi.physical_expr.FFI_PhysicalExpr.md#op-0acc9367876e5b03f6e08a37).

<a id="op-6cd7cf889b287bdcc1d3b9fc"></a>
## get_properties

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::get_properties` · datafusion-ffi 55.1.0

```rust
get_properties: unsafe fn(&Self, stabby::vec::Vec<expr::expr_properties::FFI_ExprProperties>) -> util::FFI_Result<expr::expr_properties::FFI_ExprProperties>
```

Source: `src/physical_expr/mod.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abad6b5896bb3f8334c1e056"></a>
## hash

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::hash` · datafusion-ffi 55.1.0

```rust
hash: unsafe fn(&Self) -> u64
```

Source: `src/physical_expr/mod.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-169ab11132ff8cdc2954b360"></a>
## is_volatile_node

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::is_volatile_node` · datafusion-ffi 55.1.0

```rust
is_volatile_node: unsafe fn(&Self) -> bool
```

Source: `src/physical_expr/mod.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d0b446e5590bd1c487f4641"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/physical_expr/mod.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface.

<a id="op-944cd432707148ca3231552f"></a>
## new_with_children

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::new_with_children` · datafusion-ffi 55.1.0

```rust
new_with_children: unsafe fn(&Self, &stabby::vec::Vec<FFI_PhysicalExpr>) -> util::FFI_Result<Self>
```

Source: `src/physical_expr/mod.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6279dbfc245a6c64df82b5cc"></a>
## nullable

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::nullable` · datafusion-ffi 55.1.0

```rust
nullable: unsafe fn(&Self, arrow_wrappers::WrappedSchema) -> util::FFI_Result<bool>
```

Source: `src/physical_expr/mod.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a4c8e967c7d31eaf42f26fc"></a>
## private_data

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/physical_expr/mod.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the plan.
A [`ForeignPhysicalExpr`](../operations/datafusion_ffi.physical_expr.ForeignPhysicalExpr.md#op-2134bfaadf5b6df794872c38) should never attempt to access this data.

<a id="op-0c625072d84dd31c9a0e946e"></a>
## propagate_constraints

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::propagate_constraints` · datafusion-ffi 55.1.0

```rust
propagate_constraints: unsafe fn(&Self, expr::interval::FFI_Interval, stabby::vec::Vec<expr::interval::FFI_Interval>) -> util::FFI_Result<util::FFI_Option<stabby::vec::Vec<expr::interval::FFI_Interval>>>
```

Source: `src/physical_expr/mod.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf100d768f8fff8e811d504c"></a>
## propagate_statistics

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::propagate_statistics` · datafusion-ffi 55.1.0

```rust
propagate_statistics: unsafe fn(&Self, expr::distribution::FFI_Distribution, stabby::vec::Vec<expr::distribution::FFI_Distribution>) -> util::FFI_Result<util::FFI_Option<stabby::vec::Vec<expr::distribution::FFI_Distribution>>>
```

Source: `src/physical_expr/mod.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c844626e23072c1e4251f00"></a>
## release

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/physical_expr/mod.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-80f71e8a1584aa8ff9b6398c"></a>
## return_field

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::return_field` · datafusion-ffi 55.1.0

```rust
return_field: unsafe fn(&Self, arrow_wrappers::WrappedSchema) -> util::FFI_Result<arrow_wrappers::WrappedSchema>
```

Source: `src/physical_expr/mod.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-583fb9442872513be59b0fbe"></a>
## snapshot

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::snapshot` · datafusion-ffi 55.1.0

```rust
snapshot: unsafe fn(&Self) -> util::FFI_Result<util::FFI_Option<FFI_PhysicalExpr>>
```

Source: `src/physical_expr/mod.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fc30648126c9dceb34f850e"></a>
## snapshot_generation

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::snapshot_generation` · datafusion-ffi 55.1.0

```rust
snapshot_generation: unsafe fn(&Self) -> u64
```

Source: `src/physical_expr/mod.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-436594c04cd697734436e8a0"></a>
## version

`struct_field` · `datafusion_ffi::physical_expr::FFI_PhysicalExpr::version` · datafusion-ffi 55.1.0

```rust
version: unsafe fn() -> u64
```

Source: `src/physical_expr/mod.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the major DataFusion version number of this provider.
