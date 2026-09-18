# `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.physical_optimizer.FFI_PhysicalOptimizerContext.json).

<a id="op-b4441fb1c4c68398dd08296f"></a>
## FFI_PhysicalOptimizerContext

`struct` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerContext` · datafusion-ffi 55.1.0

```rust
struct FFI_PhysicalOptimizerContext
```

Source: `src/physical_optimizer.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`PhysicalOptimizerContext`](../operations/datafusion_session.physical_optimizer.PhysicalOptimizerContext.md#op-3296df92ae1d4db86475371d) across FFI boundaries.

This provides access to configuration options for optimizer rules that need
extended context beyond the plan itself.

<a id="op-d53dc37ecf3b77a7915ab7fd"></a>
## config_options

`struct_field` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerContext::config_options` · datafusion-ffi 55.1.0

```rust
config_options: unsafe fn(&FFI_PhysicalOptimizerContext) -> config::FFI_ConfigOptions
```

Source: `src/physical_optimizer.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e373fc3318dab13765e752b"></a>
## drop

`function` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerContext::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerContext", "path": "FFI_PhysicalOptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [81, 2], "filename": "src/physical_optimizer.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/physical_optimizer.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-425131906c99e8ab88421a98"></a>
## fmt

`function` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerContext::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerContext", "path": "FFI_PhysicalOptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/physical_optimizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/physical_optimizer.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ac6e2cbbe049c2be13bc369"></a>
## new

`function` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerContext::new` · datafusion-ffi 55.1.0

```rust
fn new(context: &dyn PhysicalOptimizerContext) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerContext", "path": "FFI_PhysicalOptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [75, 2], "filename": "src/physical_optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_optimizer.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23bb887fa9e999428ef29b97"></a>
## private_data

`struct_field` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerContext::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *const std::ffi::c_void
```

Source: `src/physical_optimizer.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. Only accessed by the provider.

<a id="op-b7049ce30dfbd14befee6ae0"></a>
## release

`struct_field` · `datafusion_ffi::physical_optimizer::FFI_PhysicalOptimizerContext::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut FFI_PhysicalOptimizerContext)
```

Source: `src/physical_optimizer.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data.
