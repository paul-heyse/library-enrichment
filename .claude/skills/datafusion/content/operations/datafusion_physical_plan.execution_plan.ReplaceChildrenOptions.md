# `datafusion_physical_plan::execution_plan::ReplaceChildrenOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.ReplaceChildrenOptions.json).

<a id="op-928d85a13cadaae91c04dc83"></a>
## ReplaceChildrenOptions

`struct` · `datafusion_physical_plan::execution_plan::ReplaceChildrenOptions` · datafusion-physical-plan 55.1.0

```rust
struct ReplaceChildrenOptions
```

Source: `src/execution_plan.rs:1035`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Options for [`ExecutionPlan::replace_children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4e162e6df45a86327a682f22)

<a id="op-826e318d960d4339df7e2375"></a>
## children_properties

`struct_field` · `datafusion_physical_plan::execution_plan::ReplaceChildrenOptions::children_properties` · datafusion-physical-plan 55.1.0

```rust
children_properties: ChildrenPropertiesMode
```

Source: `src/execution_plan.rs:1038`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Describes how plan properties should be handled for the replacement
children.

<a id="op-c91af4cc0aac5a7e64aba73e"></a>
## clone

`function` · `datafusion_physical_plan::execution_plan::ReplaceChildrenOptions::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> ReplaceChildrenOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::ReplaceChildrenOptions", "path": "ReplaceChildrenOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1034, 17], "end": [1034, 22], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution_plan.rs:1034`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80bbcef24b81b4d37bae7c38"></a>
## eq

`function` · `datafusion_physical_plan::execution_plan::ReplaceChildrenOptions::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &ReplaceChildrenOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::ReplaceChildrenOptions", "path": "ReplaceChildrenOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1034, 30], "end": [1034, 39], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/execution_plan.rs:1034`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3eb38875ba43a024fe63ec3a"></a>
## fmt

`function` · `datafusion_physical_plan::execution_plan::ReplaceChildrenOptions::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::ReplaceChildrenOptions", "path": "ReplaceChildrenOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1034, 10], "end": [1034, 15], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution_plan.rs:1034`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-998e4903f09741659a6da705"></a>
## new

`function` · `datafusion_physical_plan::execution_plan::ReplaceChildrenOptions::new` · datafusion-physical-plan 55.1.0

```rust
const fn new(children_properties: ChildrenPropertiesMode) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::ReplaceChildrenOptions", "path": "ReplaceChildrenOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1041, 1], "end": [1048, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1043`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create new options for [`ExecutionPlan::replace_children`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4e162e6df45a86327a682f22).
