# `datafusion_physical_optimizer::output_requirements::OutputRequirements`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.output_requirements.OutputRequirements.json).

<a id="op-d48d9c35fd3d8e9786869bdc"></a>
## OutputRequirements

`struct` · `datafusion_physical_optimizer::output_requirements::OutputRequirements` · datafusion-physical-optimizer 55.1.0

```rust
struct OutputRequirements
```

Source: `src/output_requirements.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This rule either adds or removes [`OutputRequirements`](../operations/datafusion_physical_optimizer.output_requirements.OutputRequirements.md#op-d48d9c35fd3d8e9786869bdc)s to/from the physical
plan according to its `mode` attribute, which is set by the constructors
`new_add_mode` and `new_remove_mode`. With this rule, we can keep track of
the global requirements (ordering and distribution) across rules.

The primary use case of this node and rule is to specify and preserve the desired output
ordering and distribution the entire plan. When sending to a single client, a single partition may
be desirable, but when sending to a multi-partitioned writer, keeping multiple partitions may be
better.

<a id="op-a52566f288f10ede6f79a6a5"></a>
## fmt

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirements::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirements", "path": "OutputRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 10], "end": [61, 15], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/output_requirements.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d0bb2415615e5652f27ed46"></a>
## name

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirements::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirements", "path": "OutputRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [381, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/output_requirements.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-593a14d9377476652ff0591b"></a>
## new_add_mode

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirements::new_add_mode` · datafusion-physical-optimizer 55.1.0

```rust
fn new_add_mode() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirements", "path": "OutputRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [90, 2], "filename": "src/output_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/output_requirements.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Create a new rule which works in `Add` mode; i.e. it simply adds a
top-level [`OutputRequirementExec`](../operations/datafusion_physical_optimizer.output_requirements.OutputRequirementExec.md#op-b5659f2738d710cccbf36f7a) into the physical plan to keep track
of global ordering and distribution requirements if there are any.
Note that this rule should run at the beginning. It is idempotent: when
invoked on a plan that already contains an `OutputRequirementExec` (at
the root or below it), it returns the plan unchanged.

<a id="op-6796d27467b1b240f888f4aa"></a>
## new_remove_mode

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirements::new_remove_mode` · datafusion-physical-optimizer 55.1.0

```rust
fn new_remove_mode() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirements", "path": "OutputRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [90, 2], "filename": "src/output_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/output_requirements.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Create a new rule which works in `Remove` mode; i.e. it simply removes
the top-level [`OutputRequirementExec`](../operations/datafusion_physical_optimizer.output_requirements.OutputRequirementExec.md#op-b5659f2738d710cccbf36f7a) from the physical plan if there is
any. We do this because a `OutputRequirementExec` is an ancillary,
non-executable operator whose sole purpose is to track global
requirements during optimization. Therefore, a
`OutputRequirementExec` should not appear in the final plan.

<a id="op-6cf1c51106945ba347b7c13b"></a>
## optimize

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirements::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirements", "path": "OutputRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [381, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/output_requirements.rs:355`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b82ce21ffe9237eff63d35a1"></a>
## schema_check

`function` · `datafusion_physical_optimizer::output_requirements::OutputRequirements::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::output_requirements::OutputRequirements", "path": "OutputRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [381, 2], "filename": "src/output_requirements.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/output_requirements.rs:378`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
