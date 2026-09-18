# `datafusion_physical_plan::execution_plan::ChildrenPropertiesMode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.ChildrenPropertiesMode.json).

<a id="op-afe4b426967ff3c172825ad5"></a>
## ChildrenPropertiesMode

`enum` · `datafusion_physical_plan::execution_plan::ChildrenPropertiesMode` · datafusion-physical-plan 55.1.0

```rust
enum ChildrenPropertiesMode
```

Source: `src/execution_plan.rs:1054`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Indicates whether the plan properties of the new children must be recomputed.

Part of [`ReplaceChildrenOptions`](../operations/datafusion_physical_plan.execution_plan.ReplaceChildrenOptions.md#op-928d85a13cadaae91c04dc83).

<a id="op-e31e5a70039a4fa83b73b6e1"></a>
## Keep

`variant` · `datafusion_physical_plan::execution_plan::ChildrenPropertiesMode::Keep` · datafusion-physical-plan 55.1.0

```rust
Keep
```

Source: `src/execution_plan.rs:1057`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The plan properties of the new children are identical to the properties
of the existing children, so we can skip recomputation.

<a id="op-449a09e8483cda87bd8b69c9"></a>
## Recompute

`variant` · `datafusion_physical_plan::execution_plan::ChildrenPropertiesMode::Recompute` · datafusion-physical-plan 55.1.0

```rust
Recompute
```

Source: `src/execution_plan.rs:1060`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The plan properties of the new children are different from the properties
of the existing children, so we must recompute the properties from scratch.

<a id="op-fdcb248ad7a49d160828bd79"></a>
## clone

`function` · `datafusion_physical_plan::execution_plan::ChildrenPropertiesMode::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> ChildrenPropertiesMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::ChildrenPropertiesMode", "path": "ChildrenPropertiesMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1053, 17], "end": [1053, 22], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution_plan.rs:1053`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-483b53cf06e0151812c59427"></a>
## eq

`function` · `datafusion_physical_plan::execution_plan::ChildrenPropertiesMode::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &ChildrenPropertiesMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::ChildrenPropertiesMode", "path": "ChildrenPropertiesMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1053, 30], "end": [1053, 39], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/execution_plan.rs:1053`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fe88d581d4b17eeeb858f58"></a>
## fmt

`function` · `datafusion_physical_plan::execution_plan::ChildrenPropertiesMode::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::ChildrenPropertiesMode", "path": "ChildrenPropertiesMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1053, 10], "end": [1053, 15], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution_plan.rs:1053`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
