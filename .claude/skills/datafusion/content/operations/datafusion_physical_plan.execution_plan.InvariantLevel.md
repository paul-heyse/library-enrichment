# `datafusion_physical_plan::execution_plan::InvariantLevel`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.InvariantLevel.json).

<a id="op-3d8b1dde050c68e1f1355748"></a>
## InvariantLevel

`enum` · `datafusion_physical_plan::execution_plan::InvariantLevel` · datafusion-physical-plan 55.1.0

```rust
enum InvariantLevel
```

Source: `src/execution_plan.rs:1189`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

[`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) Invariant Level

What set of assertions ([Invariant]s)  holds for a particular `ExecutionPlan`

[Invariant]: https://en.wikipedia.org/wiki/Invariant_(mathematics)#Invariants_in_computer_science

<a id="op-ac377728389bcc00cac1214f"></a>
## Always

`variant` · `datafusion_physical_plan::execution_plan::InvariantLevel::Always` · datafusion-physical-plan 55.1.0

```rust
Always
```

Source: `src/execution_plan.rs:1192`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Invariants that are always true for the [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) node
such as the number of expected children.

<a id="op-f3d261cd1da890c2e843b8af"></a>
## Executable

`variant` · `datafusion_physical_plan::execution_plan::InvariantLevel::Executable` · datafusion-physical-plan 55.1.0

```rust
Executable
```

Source: `src/execution_plan.rs:1196`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Invariants that must hold true for the [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) node
to be "executable", such as ordering and/or distribution requirements
being fulfilled.

<a id="op-e5005ee0ac121b291a4d67fe"></a>
## clone

`function` · `datafusion_physical_plan::execution_plan::InvariantLevel::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> InvariantLevel
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::InvariantLevel", "path": "InvariantLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1188, 10], "end": [1188, 15], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution_plan.rs:1188`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
