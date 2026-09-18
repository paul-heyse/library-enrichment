# `datafusion_physical_optimizer::ensure_requirements::EnsureRequirements`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.EnsureRequirements.json).

<a id="op-70914b7c2b51ad60b1d7a82d"></a>
## EnsureRequirements

`struct` · `datafusion_physical_optimizer::ensure_requirements::EnsureRequirements` · datafusion-physical-optimizer 55.1.0

```rust
struct EnsureRequirements
```

Source: `src/ensure_requirements/mod.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Optimizer rule that enforces both distribution and sorting requirements.

This rule combines the functionality of `EnforceDistribution` and
`EnforceSorting` into a coordinated sequence where distribution is
always settled before sorting for each operator, preventing the
non-idempotent interactions between the two separate rules.

See [module level documentation](self) for more details.

<a id="op-3ca58c76d32bd278540df604"></a>
## default

`function` · `datafusion_physical_optimizer::ensure_requirements::EnsureRequirements::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> EnsureRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_requirements::EnsureRequirements", "path": "EnsureRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 10], "end": [165, 17], "filename": "src/ensure_requirements/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/ensure_requirements/mod.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6e6ff2faf6a979e72fa08e0"></a>
## fmt

`function` · `datafusion_physical_optimizer::ensure_requirements::EnsureRequirements::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_requirements::EnsureRequirements", "path": "EnsureRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 19], "end": [165, 24], "filename": "src/ensure_requirements/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ensure_requirements/mod.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0dcb27b8ffd7df5365af563e"></a>
## name

`function` · `datafusion_physical_optimizer::ensure_requirements::EnsureRequirements::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_requirements::EnsureRequirements", "path": "EnsureRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [257, 2], "filename": "src/ensure_requirements/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/ensure_requirements/mod.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21d1602cd6482c7168f6fe4a"></a>
## new

`function` · `datafusion_physical_optimizer::ensure_requirements::EnsureRequirements::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_requirements::EnsureRequirements", "path": "EnsureRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [173, 2], "filename": "src/ensure_requirements/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ensure_requirements/mod.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Create a new `EnsureRequirements` optimizer rule.

<a id="op-93ac6bddf4e1f08d2165ac82"></a>
## optimize

`function` · `datafusion_physical_optimizer::ensure_requirements::EnsureRequirements::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_requirements::EnsureRequirements", "path": "EnsureRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [257, 2], "filename": "src/ensure_requirements/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/ensure_requirements/mod.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8112adf65b56aff68bbbfaaa"></a>
## schema_check

`function` · `datafusion_physical_optimizer::ensure_requirements::EnsureRequirements::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_requirements::EnsureRequirements", "path": "EnsureRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [257, 2], "filename": "src/ensure_requirements/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/ensure_requirements/mod.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
