# `datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.physical_optimizer.ForeignPhysicalOptimizerRule.json).

<a id="op-1a495f8328f2b0ec101da5b2"></a>
## ForeignPhysicalOptimizerRule

`struct` · `datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule` · datafusion-ffi 55.1.0

```rust
struct ForeignPhysicalOptimizerRule
```

Source: `src/physical_optimizer.rs:282`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_PhysicalOptimizerRule to interact with the foreign rule.

<a id="op-ea4e9ac06d46efdd6aff0f9b"></a>
## fmt

`function` · `datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule", "path": "ForeignPhysicalOptimizerRule"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 10], "end": [281, 15], "filename": "src/physical_optimizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/physical_optimizer.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2749685fb222cba84e4b9cf8"></a>
## name

`function` · `datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule::name` · datafusion-ffi 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule", "path": "ForeignPhysicalOptimizerRule"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [351, 2], "filename": "src/physical_optimizer.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/physical_optimizer.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-585dc96b479c842af657f550"></a>
## optimize

`function` · `datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule::optimize` · datafusion-ffi 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule", "path": "ForeignPhysicalOptimizerRule"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [351, 2], "filename": "src/physical_optimizer.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/physical_optimizer.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-596f52a6b7de903189c36028"></a>
## optimize_with_context

`function` · `datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule::optimize_with_context` · datafusion-ffi 55.1.0

```rust
fn optimize_with_context(&self, plan: Arc<dyn ExecutionPlan>, context: &dyn PhysicalOptimizerContext) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule", "path": "ForeignPhysicalOptimizerRule"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [351, 2], "filename": "src/physical_optimizer.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/physical_optimizer.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9950fb752ce43392dbd4d5a1"></a>
## schema_check

`function` · `datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule::schema_check` · datafusion-ffi 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule", "path": "ForeignPhysicalOptimizerRule"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 1], "end": [351, 2], "filename": "src/physical_optimizer.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/physical_optimizer.rs:348`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
