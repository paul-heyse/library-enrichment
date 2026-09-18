# `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.physical_optimizer.FFI_PhysicalOptimizerRule.json).

<a id="op-48387403b4fec24c9b82a3b8"></a>
## FFI_PhysicalOptimizerRule

`struct` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule` · datafusion-ffi 55.1.0

```rust
struct FFI_PhysicalOptimizerRule
```

Source: `src/physical_optimizer.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`PhysicalOptimizerRule`](../operations/datafusion_session.physical_optimizer.PhysicalOptimizerRule.md#op-266e99a7574020b03ac0e686) across FFI boundaries.

<a id="op-19a7ce8d73187c473b7b8851"></a>
## clone

`struct_field` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/physical_optimizer.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the rule. This should
only need to be called by the receiver of the plan.

<a id="op-63d115d32ab5ed1d4ab58dd1"></a>
## clone

`function` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule", "path": "FFI_PhysicalOptimizerRule"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [308, 2], "filename": "src/physical_optimizer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/physical_optimizer.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ab65158d6d9117ecc800248"></a>
## drop

`function` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule", "path": "FFI_PhysicalOptimizerRule"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [246, 2], "filename": "src/physical_optimizer.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/physical_optimizer.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-799a1dc548917989005685a0"></a>
## fmt

`function` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule", "path": "FFI_PhysicalOptimizerRule"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 10], "end": [114, 15], "filename": "src/physical_optimizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/physical_optimizer.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5f09a4303ef7afcaf6373bf"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/physical_optimizer.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface.

<a id="op-03e0029cc5b815bdd4c35156"></a>
## name

`struct_field` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule::name` · datafusion-ffi 55.1.0

```rust
name: unsafe fn(&Self) -> stabby::string::String
```

Source: `src/physical_optimizer.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdef11148ccffe1c09434fea"></a>
## new

`function` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule::new` · datafusion-ffi 55.1.0

```rust
fn new(rule: Arc<dyn PhysicalOptimizerRule + Send + Sync>, runtime: Option<Handle>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule", "path": "FFI_PhysicalOptimizerRule"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [275, 2], "filename": "src/physical_optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_optimizer.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Creates a new [`FFI_PhysicalOptimizerRule`](../operations/datafusion_ffi.physical_optimizer.FFI_PhysicalOptimizerRule.md#op-48387403b4fec24c9b82a3b8).

<a id="op-5cc20b71474dd1ef06847354"></a>
## optimize

`struct_field` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule::optimize` · datafusion-ffi 55.1.0

```rust
optimize: unsafe fn(&Self, &execution_plan::FFI_ExecutionPlan, config::FFI_ConfigOptions) -> util::FFI_Result<execution_plan::FFI_ExecutionPlan>
```

Source: `src/physical_optimizer.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6db011c0902fa10e4bd695f0"></a>
## optimize_with_context

`struct_field` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule::optimize_with_context` · datafusion-ffi 55.1.0

```rust
optimize_with_context: unsafe fn(&Self, &execution_plan::FFI_ExecutionPlan, &FFI_PhysicalOptimizerContext) -> util::FFI_Result<execution_plan::FFI_ExecutionPlan>
```

Source: `src/physical_optimizer.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e4458788b2cb63424c77fa1"></a>
## private_data

`struct_field` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/physical_optimizer.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the rule.
A [`ForeignPhysicalOptimizerRule`](../operations/datafusion_ffi.physical_optimizer.ForeignPhysicalOptimizerRule.md#op-1a495f8328f2b0ec101da5b2) should never attempt to access this data.

<a id="op-121e87e55631fe417b8984d8"></a>
## release

`struct_field` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/physical_optimizer.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-6c2b6731fae4ba7ecb44c514"></a>
## schema_check

`struct_field` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule::schema_check` · datafusion-ffi 55.1.0

```rust
schema_check: unsafe fn(&Self) -> bool
```

Source: `src/physical_optimizer.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92129f73c7a3eeef7f503999"></a>
## version

`struct_field` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerRule::version` · datafusion-ffi 55.1.0

```rust
version: unsafe fn() -> u64
```

Source: `src/physical_optimizer.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the major DataFusion version number of this rule.
