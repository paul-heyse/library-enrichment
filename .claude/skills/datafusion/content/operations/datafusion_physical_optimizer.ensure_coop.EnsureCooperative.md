# `datafusion_physical_optimizer::ensure_coop::EnsureCooperative`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_coop.EnsureCooperative.json).

<a id="op-fe53237dbfc32d9e59d026ba"></a>
## EnsureCooperative

`struct` · `datafusion_physical_optimizer::ensure_coop::EnsureCooperative` · datafusion-physical-optimizer 55.1.0

```rust
struct EnsureCooperative
```

Source: `src/ensure_coop.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

`EnsureCooperative` is a [`PhysicalOptimizerRule`](../operations/datafusion_session.physical_optimizer.PhysicalOptimizerRule.md#op-266e99a7574020b03ac0e686) that inspects the physical plan for
sub plans that do not participate in cooperative scheduling. The plan is subdivided into sub
plans on eager evaluation boundaries. Leaf nodes and eager evaluation roots are checked
to see if they participate in cooperative scheduling. Those that do no are wrapped in
a [`CooperativeExec`](../operations/datafusion_physical_plan.coop.CooperativeExec.md#op-1efab241141bdbd717c01df7) parent.

<a id="op-7d891ff96577a945a9fd514c"></a>
## default

`function` · `datafusion_physical_optimizer::ensure_coop::EnsureCooperative::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_coop::EnsureCooperative", "path": "EnsureCooperative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [52, 2], "filename": "src/ensure_coop.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/ensure_coop.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ad16d177652a2cb073d8717"></a>
## fmt

`function` · `datafusion_physical_optimizer::ensure_coop::EnsureCooperative::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_coop::EnsureCooperative", "path": "EnsureCooperative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [58, 2], "filename": "src/ensure_coop.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ensure_coop.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5896af5b8a4c813ad64e4728"></a>
## name

`function` · `datafusion_physical_optimizer::ensure_coop::EnsureCooperative::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_coop::EnsureCooperative", "path": "EnsureCooperative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [128, 2], "filename": "src/ensure_coop.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/ensure_coop.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41415e8dd2f2c582793a1204"></a>
## new

`function` · `datafusion_physical_optimizer::ensure_coop::EnsureCooperative::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_coop::EnsureCooperative", "path": "EnsureCooperative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [46, 2], "filename": "src/ensure_coop.rs"}, "trait": null, "trait_path": null}`

Source: `src/ensure_coop.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11766a7714716ba4f44c24d6"></a>
## optimize

`function` · `datafusion_physical_optimizer::ensure_coop::EnsureCooperative::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_coop::EnsureCooperative", "path": "EnsureCooperative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [128, 2], "filename": "src/ensure_coop.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/ensure_coop.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca2a8c2afc7c1ca45140eed3"></a>
## schema_check

`function` · `datafusion_physical_optimizer::ensure_coop::EnsureCooperative::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_coop::EnsureCooperative", "path": "EnsureCooperative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [128, 2], "filename": "src/ensure_coop.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/ensure_coop.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
